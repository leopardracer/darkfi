/* This file is part of DarkFi (https://dark.fi)
 *
 * Copyright (C) 2020-2023 Dyne.org foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{collections::HashSet, sync::Arc};

use darkfi::{
    async_daemonize, cli_desc,
    rpc::{
        jsonrpc::{ErrorCode, JsonError, JsonRequest, JsonResult},
        server::{listen_and_serve, RequestHandler},
    },
    system::{StoppableTask, StoppableTaskPtr},
    Error, Result,
};
use darkfi_serial::async_trait;
use log::{debug, error, info};
use serde::Deserialize;
use smol::{
    lock::{Mutex, MutexGuard},
    stream::StreamExt,
    Executor,
};
use structopt::StructOpt;
use structopt_toml::StructOptToml;
use url::Url;

mod monero;
mod stratum;

const CONFIG_FILE: &str = "darkfi_mmproxy.toml";
const CONFIG_FILE_CONTENTS: &str = include_str!("../darkfi_mmproxy.toml");

#[derive(Clone, Debug, Deserialize, StructOpt, StructOptToml)]
#[serde(default)]
#[structopt(name = "darkfi-mmproxy", about = cli_desc!())]
struct Args {
    #[structopt(short, parse(from_occurrences))]
    /// Increase verbosity (-vvv supported)
    verbose: u8,

    #[structopt(short, long)]
    /// Configuration file to use
    config: Option<String>,

    #[structopt(long, default_value = "tcp://127.0.0.1:3333")]
    /// JSON-RPC server listen URL
    rpc_listen: Url,

    #[structopt(long)]
    /// Set log file output
    log: Option<String>,
}

struct MiningProxy {
    /// JSON-RPC connection tracker
    rpc_connections: Mutex<HashSet<StoppableTaskPtr>>,
}

impl MiningProxy {
    fn new() -> Self {
        Self { rpc_connections: Mutex::new(HashSet::new()) }
    }
}

#[async_trait]
#[rustfmt::skip]
impl RequestHandler for MiningProxy {
    async fn handle_request(&self, req: JsonRequest) -> JsonResult {
        error!(target: "mmproxy::rpc", "--> {}", req.stringify().unwrap());

        match req.method.as_str() {
            "ping" => self.pong(req.id, req.params).await,

            // Stratum methods
            "login" => self.stratum_login(req.id, req.params).await,
            "job" => self.stratum_job(req.id, req.params).await,
            "submit" => self.stratum_submit(req.id, req.params).await,
            "keepalived" => self.stratum_keepalived(req.id, req.params).await,

            // Monero daemon methods
            "get_block_count" => self.monero_get_block_count(req.id, req.params).await,
            "on_get_block_hash" => self.monero_on_get_block_hash(req.id, req.params).await,
            "get_block_template" => self.monero_get_block_template(req.id, req.params).await,
            "submit_block" => self.monero_submit_block(req.id, req.params).await,
            "generateblocks" => self.monero_generateblocks(req.id, req.params).await,
            "get_last_block_header" => self.monero_get_last_block_header(req.id, req.params).await,
            "get_block_header_by_hash" => self.monero_get_block_header_by_hash(req.id, req.params).await,
            "get_block_header_by_height" => self.monero_get_block_header_by_height(req.id, req.params).await,
            "get_block_headers_range" => self.monero_get_block_headers_range(req.id, req.params).await,
            "get_block" => self.monero_get_block(req.id, req.params).await,
            "get_connections" => self.monero_get_connections(req.id, req.params).await,
            "get_info" => self.monero_get_info(req.id, req.params).await,
            "hard_fork_info" => self.monero_hard_fork_info(req.id, req.params).await,
            "set_bans" => self.monero_set_bans(req.id, req.params).await,
            "get_bans" => self.monero_get_bans(req.id, req.params).await,
            "banned" => self.monero_banned(req.id, req.params).await,
            "flush_txpool" => self.monero_flush_txpool(req.id, req.params).await,
            "get_output_histogram" => self.monero_get_output_histogram(req.id, req.params).await,
            "get_version" => self.monero_get_version(req.id, req.params).await,
            "get_coinbase_tx_sum" => self.monero_get_coinbase_tx_sum(req.id, req.params).await,
            "get_fee_estimate" => self.monero_get_fee_estimate(req.id, req.params).await,
            "get_alternate_chains" => self.monero_get_alternate_chains(req.id, req.params).await,
            "relay_tx" => self.monero_relay_tx(req.id, req.params).await,
            "sync_info" => self.monero_sync_info(req.id, req.params).await,
            "get_txpool_backlog" => self.monero_get_txpool_backlog(req.id, req.params).await,
            "get_output_distribution" => self.monero_get_output_distribution(req.id, req.params).await,
            "get_miner_data" => self.monero_get_miner_data(req.id, req.params).await,
            "prune_blockchain" => self.monero_prune_blockchain(req.id, req.params).await,
            "calc_pow" => self.monero_calc_pow(req.id, req.params).await,
            "flush_cache" => self.monero_flush_cache(req.id, req.params).await,
            "add_aux_pow" => self.monero_add_aux_pow(req.id, req.params).await,

            _ => JsonError::new(ErrorCode::MethodNotFound, None, req.id).into(),
        }
    }

    async fn connections_mut(&self) -> MutexGuard<'_, HashSet<StoppableTaskPtr>> {
        self.rpc_connections.lock().await
    }
}

async_daemonize!(realmain);
async fn realmain(args: Args, ex: Arc<Executor<'static>>) -> Result<()> {
    info!("Starting JSON-RPC server");
    let mmproxy = Arc::new(MiningProxy::new());
    let mmproxy_ = Arc::clone(&mmproxy);
    let rpc_task = StoppableTask::new();
    rpc_task.clone().start(
        listen_and_serve(args.rpc_listen, mmproxy.clone(), None, ex.clone()),
        |res| async move {
            match res {
                Ok(()) | Err(Error::RpcServerStopped) => mmproxy_.stop_connections().await,
                Err(e) => error!("Failed stopping JSON-RPC server: {}", e),
            }
        },
        Error::RpcServerStopped,
        ex.clone(),
    );

    // Signal handling for graceful termination.
    let (signals_handler, signals_task) = SignalHandler::new(ex)?;
    signals_handler.wait_termination(signals_task).await?;
    info!("Caught termination signal, cleaning up and exiting...");

    Ok(())
}