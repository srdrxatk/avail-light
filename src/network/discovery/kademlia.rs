// Substrate-lite
// Copyright (C) 2019-2020  Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// TODO: work in progress

use prost::Message as _;

mod dht_proto {
    // File generated by the build script.
    include!(concat!(env!("OUT_DIR"), "/dht.pb.rs"));
}

/// Data structure containing the k-buckets and the state of the current Kademlia queries.
pub struct Kademlia {}

impl Kademlia {
    /// Initializes a new empty data structure with empty k-buckets.
    pub fn new() -> Self {
        Kademlia {}
    }
}

/// Builds a wire message to send on the Kademlia request-response protocol to ask the target to
/// return the nodes closest to the parameter.
// TODO: parameter type?
pub fn build_find_node_request(peer_id: &[u8]) -> Vec<u8> {
    let protobuf = dht_proto::Message {
        r#type: dht_proto::message::MessageType::FindNode as i32,
        key: peer_id.to_vec(),
        ..Default::default()
    };

    let mut buf = Vec::with_capacity(protobuf.encoded_len());
    protobuf.encode(&mut buf).unwrap();
    buf
}