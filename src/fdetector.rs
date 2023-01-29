use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use futures::future::join_all;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::UdpSocket;
use tokio_util::udp::UdpFramed;
use bincode;



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Inquire {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
impl Iterator for Addr {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<Self::Item> {
       None 
    }
}

impl ToSocketAddrs for Addr {
    type Iter = Addr; 
    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
       unimplemented!() 
    }
}

impl Clone for Addr {
    fn clone(&self) -> Self {
        Self { port: self.port, host: self.host.to_owned() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PeerState {
    Alive, 
    Suspect, 
    Dead
}

pub struct SwimFailureDetector {
    id: Id,
    peers: Vec<Peer>,
    rng: ThreadRng,
    period: u64,
    failure_group_sz: u64,
    peer_state: HashMap<Peer, PeerState>,
}

impl SwimFailureDetector {
    pub fn new(id: Id, peers: Vec<Peer>, period: u64, failure_group_sz: u64) -> SwimFailureDetector {
        let rng = rand::thread_rng();
        SwimFailureDetector{id, peers, rng, period, failure_group_sz, peer_state: HashMap::new()} 
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

    
    pub async fn connect(&self, peer: &Peer) -> PeerState {
        let sock_addr = (peer.addr.host.to_owned() , peer.addr.port).to_socket_addrs().unwrap().next().unwrap();
        todo!();
    }

    pub async fn bind(&self) -> std::io::Result<()> {
        let mut host: Option<Addr> = None;
        for peer in &self.peers {
            if peer.id == self.id {
                host = Some(peer.addr.clone());
                break;
            }
        }
        let host = match host {
            Some(host) => host, 
            None => return Ok(())
        };
        let sock_addr = host.to_socket_addrs().unwrap().next().unwrap();
        let ssock = UdpSocket::bind(sock_addr).await?;
        let mut buf = [0; 4096*12];
        loop {
          let (usize, addr) = match ssock.recv_from(&mut buf).await {
            Ok(d) => d, 
            Err(e) => {
                continue;
            }
          };
        }
    }

    pub async fn run(&mut self) {
        let mut futures = vec![];

        self.bind().await;
        for peer in &self.peers {
            if peer.id == self.id {
                continue;
            }

            let future = self.connect(peer);
            futures.push(future);
        }

        let states = join_all(futures).await;

    }
}
