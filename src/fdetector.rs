use std::collections::HashMap;

use protocol::Protocol;

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Inquire {}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub enum Packet {
    Ping(),
    Ack(),
    AckRequire(),
    InformJoin(),
}

pub struct Addr {}

pub struct SwimFailureDetector {
    id: Vec<u8>,
    nodes: Vec<Vec<u8>>,
    addrs: Vec<Addr>,
}

impl SwimFailureDetector {
    pub fn run_round(&self) {}

    pub fn add_node(&self, node_id: Vec<u8>, addr: Addr) {}
}
