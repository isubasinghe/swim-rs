use rand::rngs::ThreadRng;
use rand::Rng;
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
    rng: ThreadRng,
    period: u32,
    failure_group_sz: u32,
}

impl SwimFailureDetector {
    pub fn new() {
        let mut rng = rand::thread_rng();
    }

    pub fn run_round(&mut self) {
        if self.nodes.len() < 1 {
            return;
        }

        let index = self.rng.gen_range(0..self.nodes.len());

        for _ in 0..self.failure_group_sz {
            let index = self.rng.gen_range(0..self.nodes.len());
        }
    }

    pub fn add_node(&self, node_id: Vec<u8>, addr: Addr) {}
}
