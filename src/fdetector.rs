use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;

use protocol::Protocol;
use serde::{Deserialize, Serialize};

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Inquire {}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub enum Packet {
    Ping(),
    Ack(),
    AckRequire(),
    InformJoin(),
}

pub type Id=u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub peers: Vec<Peer>,
    pub period: u64, 
    pub failure_group_sz: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Peer {
    id: Id,
    addr: Addr,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Addr {
    pub port: u16, 
    pub host: String
}

pub struct SwimFailureDetector {
    id: Id,
    peers: Vec<Peer>,
    rng: ThreadRng,
    period: u64,
    failure_group_sz: u64,
}

impl SwimFailureDetector {
    pub fn new(id: Id, peers: Vec<Peer>, period: u64, failure_group_sz: u64) -> SwimFailureDetector {
        let rng = rand::thread_rng();
        SwimFailureDetector{id, peers, rng, period, failure_group_sz} 
    }

    pub fn run_round(&mut self) {
        if self.peers.len() < 1 {
            return;
        }

        let index = self.rng.gen_range(0..self.peers.len());

        for _ in 0..self.failure_group_sz {
            let index = self.rng.gen_range(0..self.peers.len());
        }
    }

    pub fn add_node(&self, node_id: Vec<u8>, addr: Addr) {}


    pub fn run(&mut self) {

    }
}
