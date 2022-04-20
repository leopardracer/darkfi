use std::sync::Arc;

use async_executor::Executor;
use clap::Parser;
use easy_parallel::Parallel;
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use url::Url;
use rand::{rngs::OsRng, Rng, RngCore};

use darkfi::net2 as net;
use darkfi::{
    cli_desc,
    util::{cli::log_config, sleep},
    Result,
};

pub(crate) mod proto;

use crate::proto::debugmsg::{Debugmsg, ProtocolDebugmsg, SeenDebugmsgIds};

#[derive(Parser)]
#[clap(name = "p2pdebugging", about = cli_desc!(), version)]
struct Args {
    /// Verbosity level
    #[clap(short, parse(from_occurrences))]
    verbose: u8,
    /// node number:  
    /// 0-2 is for seed nodes
    /// 3-20 is for inbound connections nodes
    /// 21- is for outbound connections nodes
    #[clap(short, long, default_value = "0")]
    node: u8,
    /// broadcast messages
    #[clap(short, long)]
    broadcast: bool,
}

#[derive(Debug, Clone)]
enum State {
    Seed,
    Inbound,
    Outbound,
}

struct MockP2p {
    node_number: u8,
    state: State,
    broadcast: bool,
    address: Option<Url>,
}

impl MockP2p {
    async fn new(node_number: u8, _broadcast: bool) -> Result<(net::P2pPtr<impl net::Transport>, Self)> {
        let seed_addrs: Vec<Url> = vec![
            Url::parse("tcp://127.0.0.1:11001")?,
            Url::parse("tcp://127.0.0.1:11002")?,
            Url::parse("tcp://127.0.0.1:11003")?,
        ];

        let state: State;
        let address: Option<Url>;

        let mut broadcast = _broadcast;

        let p2p = match node_number {
            0..=2 => {
                address = Some(seed_addrs[node_number as usize].clone());

                let net_settings = net::Settings { inbound: address.clone(), ..Default::default() };
                let p2p = net::P2p::<net::TcpTransport>::new(net_settings).await;

                broadcast = false;
                state = State::Seed;

                p2p
            }
            3..=20 => {
                let random_port: u32 = rand::thread_rng().gen_range(11007..49151);
                address = Some(format!("tcp://127.0.0.1:{}", random_port).parse()?);

                let net_settings = net::Settings {
                    inbound: address.clone(),
                    external_addr: address.clone(),
                    seeds: seed_addrs,
                    ..Default::default()
                };

                let p2p = net::P2p::<net::TcpTransport>::new(net_settings).await;

                state = State::Inbound;

                p2p
            }
            _ => {
                address = None;

                let net_settings = net::Settings {
                    outbound_connections: 3,
                    seeds: seed_addrs,
                    ..Default::default()
                };

                let p2p = net::P2p::<net::TcpTransport>::new(net_settings).await;
                state = State::Outbound;

                p2p
            }
        };

        println!("start {:?} node #{} address {:?}", state, node_number, address);

        Ok((p2p, Self { node_number, state, broadcast, address }))
    }

    async fn run(&self, p2p: net::P2pPtr<impl net::Transport>, executor: Arc<Executor<'_>>) -> Result<()> {
        let state = self.state.clone();
        let node_number = self.node_number;
        let address = self.address.clone();

        let (sender, receiver) = async_channel::unbounded();
        let sender_clone = sender.clone();

        let seen_debugmsg_ids = SeenDebugmsgIds::new();
        let seen_debugmsg_ids_clone = seen_debugmsg_ids.clone();

        let registry = p2p.protocol_registry();
        registry
            .register(!net::SESSION_SEED, move |channel, p2p| {
                let sender = sender_clone.clone();
                let seen_debugmsg_ids = seen_debugmsg_ids_clone.clone();
                async move { ProtocolDebugmsg::init(channel, sender, seen_debugmsg_ids, p2p).await }
            })
            .await;

        let address_cloned = address.clone();
        if self.broadcast {
            println!("start broadcast {:?} node #{} address {:?}", state, node_number, address);
            let sleep_time = 10;
            let p2p_clone = p2p.clone();
            let executor_clone = executor.clone();
            executor_clone
                .spawn(async move {
                    loop {
                        sleep(sleep_time).await;

                        println!(
                            "broadcast sleep for {} {:?} node #{} address {:?}",
                            sleep_time, state, node_number, address
                        );

                        let random_id = OsRng.next_u32();

                        let msg = Debugmsg { id: random_id, message: "hello".to_string() };

                        println!(
                            "send {:?} {:?} node #{} address {:?}",
                            msg, state, node_number, address
                        );

                        p2p_clone.broadcast(msg).await.unwrap();
                    }
                })
                .detach();
        }

        let state = self.state.clone();
        let seen_debugmsg_ids_clone = seen_debugmsg_ids.clone();
        executor
            .spawn(async move {
                loop {
                    let msg = receiver.recv().await.unwrap();
                    println!(
                        "receive {:?} {:?} node #{} address {:?}",
                        msg, state, node_number, address_cloned 
                    );
                    seen_debugmsg_ids_clone.add_seen(msg.id).await;
                }
            })
            .detach();

        p2p.clone().start(executor.clone()).await?;
        p2p.run(executor).await
    }
}

async fn start(executor: Arc<Executor<'_>>, args: Args) -> Result<()> {
    let (p2p, mock_p2p) = MockP2p::new(args.node, args.broadcast).await?;
    mock_p2p.run(p2p.clone(), executor).await
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (lvl, conf) = log_config(args.verbose.into())?;
    TermLogger::init(lvl, conf, TerminalMode::Mixed, ColorChoice::Auto)?;

    let ex = Arc::new(Executor::new());
    let ex_clone = ex.clone();
    let (signal, shutdown) = async_channel::unbounded::<()>();
    let (_, result) = Parallel::new()
        .each(0..4, |_| smol::future::block_on(ex.run(shutdown.recv())))
        // Run the main future on the current thread.
        .finish(|| {
            smol::future::block_on(async move {
                start(ex_clone.clone(), args).await?;
                drop(signal);
                Ok::<(), darkfi::Error>(())
            })
        });

    result
}
