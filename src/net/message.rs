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

use darkfi_serial::{
    async_trait, serialize_async, AsyncDecodable, AsyncEncodable, SerialDecodable, SerialEncodable,
};
use log::trace;
use smol::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use url::Url;

use super::economy::{Resource, ResourceLimit};
use crate::{Error, Result};

const MAGIC_BYTES: [u8; 4] = [0xd9, 0xef, 0xb6, 0x7d];

/// Generic message template.
pub trait Message: 'static + Send + Sync + AsyncDecodable + AsyncEncodable + ResourceLimit {
    const NAME: &'static str;
}

#[macro_export]
macro_rules! impl_p2p_message {
    ($st:ty, $nm:expr) => {
        impl Message for $st {
            const NAME: &'static str = $nm;
        }
    };
}

/// Outbound keepalive message.
#[derive(Debug, Copy, Clone, SerialEncodable, SerialDecodable)]
pub struct PingMessage {
    pub nonce: u16,
}
impl_p2p_message!(PingMessage, "ping");

impl ResourceLimit for PingMessage {
    fn limit(&self) -> Vec<(Resource, u32)> {
        vec![]
    }
}

/// Inbound keepalive message.
#[derive(Debug, Copy, Clone, SerialEncodable, SerialDecodable)]
pub struct PongMessage {
    pub nonce: u16,
}
impl_p2p_message!(PongMessage, "pong");

impl ResourceLimit for PongMessage {
    fn limit(&self) -> Vec<(Resource, u32)> {
        vec![]
    }
}

/// Requests address of outbound connecction.
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct GetAddrsMessage {
    /// Maximum number of addresses with preferred
    /// transports to receive. Response vector will
    /// also containg addresses without the preferred
    /// transports, so its size will be 2 * max.
    pub max: u32,
    /// Preferred addresses transports
    pub transports: Vec<String>,
}
impl_p2p_message!(GetAddrsMessage, "getaddr");

impl ResourceLimit for GetAddrsMessage {
    fn limit(&self) -> Vec<(Resource, u32)> {
        vec![]
    }
}

/// Sends address information to inbound connection.
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct AddrsMessage {
    pub addrs: Vec<(Url, u64)>,
}

impl_p2p_message!(AddrsMessage, "addr");

impl ResourceLimit for AddrsMessage {
    fn limit(&self) -> Vec<(Resource, u32)> {
        vec![]
    }
}

/// Requests version information of outbound connection.
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct VersionMessage {
    /// Only used for debugging. Compromises privacy when set.
    pub node_id: String,
    /// Identifies protocol version being used by the node
    pub version: semver::Version,
    /// UNIX timestamp of when the VersionMessage was created.
    pub timestamp: u64,
    /// Network address of the node receiving this message (before
    /// resolving).
    pub connect_recv_addr: Url,
    /// Network address of the node receiving this message (after
    /// resolving). Optional because only used by outbound connections.
    pub resolve_recv_addr: Option<Url>,
    /// External address of the sender node, if it exists (empty
    /// otherwise).
    pub ext_send_addr: Vec<Url>,
    /// List of features consisting of a tuple of (services, version)
    /// to be enabled for this connection
    pub features: Vec<(String, u32)>,
}
impl_p2p_message!(VersionMessage, "version");

impl ResourceLimit for VersionMessage {
    fn limit(&self) -> Vec<(Resource, u32)> {
        vec![]
    }
}

/// Sends version information to inbound connection.
/// Response to `VersionMessage`.
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct VerackMessage {
    /// App version
    pub app_version: semver::Version,
}
impl_p2p_message!(VerackMessage, "verack");

impl ResourceLimit for VerackMessage {
    fn limit(&self) -> Vec<(Resource, u32)> {
        vec![]
    }
}

/// Packets are the base type read from the network.
/// Converted to messages and passed to event loop.
#[derive(Debug, SerialEncodable, SerialDecodable)]
pub struct Packet {
    pub command: String,
    pub payload: Vec<u8>,
}

/// TODO: FIXME (doc)
///
/// Reads and decodes an inbound payload from the given async stream.
/// Returns decoded [`Packet`].
/// We start by extracting the packet length from the stream, then allocate
/// the precise buffer for this length using stream.take(). This provides
/// a basic DDOS protection.
pub async fn read_command<R: AsyncRead + Unpin + Send + Sized>(stream: &mut R) -> Result<String> {
    // Packets should have a 4 byte header of magic digits.
    // This is used for network debugging.
    let mut magic = [0u8; 4];
    trace!(target: "net::message", "Reading magic...");
    stream.read_exact(&mut magic).await?;

    trace!(target: "net::message", "Read magic {:?}", magic);
    if magic != MAGIC_BYTES {
        trace!(target: "net::message", "Error: Magic bytes mismatch");
        return Err(Error::MalformedPacket)
    }

    let command = String::decode_async(stream).await?;

    // TODO: reimplement bounds checking
    // First deserialize the command, i.e. the type of the message.
    //let cmd_len = VarInt::decode_async(stream).await?.0;
    //if cmd_len > PACKET_LIMIT_LEN {
    //    return Err(Error::PacketOutOfBounds)
    //}

    //let mut cmd_stream = stream.take(cmd_len);
    //let mut cmd_str: Vec<u8> = Vec::new();
    //cmd_str.try_reserve(cmd_len as usize)?;

    //for _ in 0..cmd_len {
    //    cmd_str.push(AsyncDecodable::decode_async(&mut cmd_stream).await?);
    //}
    //let command = String::from_utf8(cmd_str)?;

    Ok(command)
}

/// TODO: FIXME (doc)
///
/// Sends an outbound packet by writing data to the given async stream.
/// Returns the total written bytes.
pub async fn send_packet2<W: AsyncWrite + Unpin + Send + Sized, M: Message>(
    stream: &mut W,
    message: &M,
) -> Result<usize> {
    // TODO: reimplement assert
    //assert!(!packet.command.is_empty());
    assert!(std::mem::size_of::<usize>() <= std::mem::size_of::<u64>());

    let mut written: usize = 0;

    trace!(target: "net::message", "Sending magic...");
    written += MAGIC_BYTES.encode_async(stream).await?;
    trace!(target: "net::message", "Sent magic");

    written += M::NAME.to_string().encode_async(stream).await?;
    trace!(target: "net::message", "Sent command: {}", M::NAME.to_string());

    written += message.encode_async(stream).await?;
    trace!(target: "net::message", "Sent payload {} bytes", serialize_async(message).await.len() as u64);

    stream.flush().await?;

    Ok(written)
}

// TODO: modify .send_packet() so it calls .encode_async() on Message directly
/// Sends an outbound packet by writing data to the given async stream.
/// Returns the total written bytes.
pub async fn send_packet<W: AsyncWrite + Unpin + Send + Sized>(
    stream: &mut W,
    packet: Packet,
) -> Result<usize> {
    assert!(!packet.command.is_empty());
    assert!(std::mem::size_of::<usize>() <= std::mem::size_of::<u64>());

    let mut written: usize = 0;

    trace!(target: "net::message", "Sending magic...");
    written += MAGIC_BYTES.encode_async(stream).await?;
    trace!(target: "net::message", "Sent magic");

    written += packet.command.encode_async(stream).await?;
    trace!(target: "net::message", "Sent command: {}", packet.command);

    written += packet.payload.encode_async(stream).await?;
    trace!(target: "net::message", "Sent payload {} bytes", packet.payload.len() as u64);

    stream.flush().await?;

    Ok(written)
}
