/* This file is part of DarkFi (https://dark.fi)
 *
 * Copyright (C) 2020-2024 Dyne.org foundation
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

use crate::{Error, Result};
use async_trait::async_trait;
use smol::lock::Mutex;
use std::{collections::HashMap, sync::Arc};

// TODO: Expand or modify these resources needed.
#[derive(Eq, Hash, PartialEq)]
pub enum Resource {
    Memory,
    Cpu,
    Harddisk,
    Bandwidth,
}

pub type ResourceLimitPtr = Arc<dyn ResourceLimit + Send + Sync>;

// Each Message, Protocol etc defines a limit. This basically couples
// a Resource with a u32, i.e:
//
// pub trait ProtocolBase: ResourceLimit { ... }
// pub trait Message: 'static + Send + Sync + Encodable + Decodable + ResourceLimit { ... }
#[async_trait]
pub trait ResourceLimit {
    fn limit(&self) -> Vec<(Resource, u32)>;
}

// `ResourceMonitor` monitors activity and increments the limit,
// checking whether `usage_tally > dyn resource_limit`
//
// Owned by Channel.
//
// Protocols increment this via the ChannelPtr they hold, while message
// resources are incremented inside Channel `send_message()` and
// `recv_message()`.
//
// Returning an error from `increment()` will trigger actions in Channel
// such as `ban()`.
pub(in crate::net) struct ResourceMonitor {
    tally: Mutex<HashMap<Resource, u32>>,
}

impl ResourceMonitor {
    pub(in crate::net) fn new() -> Self {
        Self { tally: Mutex::new(HashMap::new()) }
    }

    async fn increment(&self, resource: Resource, score: u32, limit: u32) -> Result<()> {
        let mut tally = self.tally.lock().await;
        let entry = tally.get_mut(&resource).unwrap();
        *entry += score;
        if *entry > limit {
            // TODO: Error- Limit breached!
        }
        Ok(())
    }
}
