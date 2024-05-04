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

use darkfi::{impl_p2p_message, net::Message};
use darkfi_serial::{
    async_trait, AsyncDecodable, AsyncEncodable, SerialDecodable, SerialEncodable, VarInt,
};
use smol::io::{AsyncReadExt, Cursor};

#[derive(Debug, PartialEq, Clone, SerialEncodable, SerialDecodable)]
pub struct TestMsg {
    pub payload: Vec<String>,
}
impl_p2p_message!(TestMsg, "TestMsg");

#[test]
fn message_encode() {
    smol::block_on(async {
        println!("==================================================");
        println!("Conducting first test...");

        let mut buffer = Vec::<u8>::new();
        let mut written: usize = 0;

        let testmsg = TestMsg { payload: vec![] };
        let name = TestMsg::NAME;

        written += name.encode_async(&mut buffer).await.unwrap();
        written += testmsg.encode_async(&mut buffer).await.unwrap();

        println!("Wrote bytes: {}", written);

        let mut cursor = Cursor::new(buffer);

        println!("Decoding String...");
        println!("==================================================");
        let command = String::decode_async(&mut cursor).await.unwrap();
        println!("==================================================");
        println!("Decoding String complete!");
        println!("==================================================");
        println!("Decoding Message...");
        println!("==================================================");
        let payload = TestMsg::decode_async(&mut cursor).await.unwrap();
        println!("==================================================");
        println!("Decoding Message complete!");
        println!("==================================================");

        assert!(command == name);
        assert!(payload == testmsg);

        println!("First test past!");
        println!("Conducting second test...");

        let mut buffer = Vec::<u8>::new();
        let mut written: usize = 0;

        let testmsg = TestMsg { payload: vec![] };

        written += testmsg.encode_async(&mut buffer).await.unwrap();
        println!("Wrote bytes: {}", written);

        let mut cursor = Cursor::new(buffer);

        println!("Getting length from VarInt decoding...");
        println!("==================================================");
        let len = VarInt::decode_async(&mut cursor).await.unwrap().length();
        println!("==================================================");
        println!("Got length: {}", len);
        println!("==================================================");
        let mut take = cursor.take(len as u64);
        println!("Decoding Message...");
        println!("==================================================");
        let payload = TestMsg::decode_async(&mut take).await.unwrap();
    });
}
