use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;

use bincode;
use bytes::{Buf, BytesMut};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::UdpSocket;
use tracing::{error, info, warn};
use std::time::Duration;
use tokio::time;

const DATAGRAM_MAX_SZ: usize = 65_507;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Inquire {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Packet {
    Ping(Addr),
    Ack(Addr),
    AckRequire(Addr),
    InformJoin(Addr),
}


fn decode(src: &mut BytesMut) -> Result<Option<Packet>, std::io::Error> {
    if src.len() < 4 {
        return Ok(None);
    }

    let mut length_bytes = [0u8; 4];
    length_bytes.copy_from_slice(&src[..4]);
    let length = u32::from_le_bytes(length_bytes) as usize;
    if src.len() < 4 + length {
        src.reserve(4 + length - src.len());
        return Ok(None);
    }
    let data = src[4..4 + length].to_vec();
    src.advance(4 + length);

    let packet: Packet = match bincode::deserialize(&data) {
        Ok(packet) => packet,
        Err(e) => {
            return Err(Error::new(ErrorKind::Other, e));
        }
    };
    Ok(Some(packet))
}


fn encode(item: Packet, dst: &mut BytesMut) -> Result<(), std::io::Error> {
    let data = match bincode::serialize(&item) {
        Ok(data) => data,
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    };

    if data.len() > DATAGRAM_MAX_SZ {
        return Err(Error::new(
            ErrorKind::Other,
            "PACKET EXCEEDS UDP MAX DATAGRAM SIZE",
        ));
    }
    let len_slice = u32::to_le_bytes(data.len() as u32);
    dst.reserve(4 + data.len());

    dst.extend_from_slice(&len_slice);
    dst.extend_from_slice(&data);
    Ok(())
}

pub type Id = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub peers: Vec<Peer>,
    pub period: u64,
    pub failure_group_sz: u64,
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub struct Peer {
    id: Id,
    addr: Addr,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
pub struct Addr {
    pub port: u16,
    pub host: String,
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
        Self {
            port: self.port,
            host: self.host.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PeerState {
    Alive,
    Suspect,
    Dead,
}

pub struct SwimFailureDetector {
    id: Id,
    addr: Addr, 
    peers: Vec<Peer>,
    rng: ThreadRng,
    period: u64,
    failure_group_sz: u64,
    peer_state: HashMap<Peer, (SocketAddr, PeerState)>,
}

impl SwimFailureDetector {
    pub fn new(
        id: Id,
        addr: Addr, 
        peers: Vec<Peer>,
        period: u64,
        failure_group_sz: u64,
    ) -> SwimFailureDetector {
        let rng = rand::thread_rng();
        SwimFailureDetector {
            id,
            addr,
            peers,
            rng,
            period,
            failure_group_sz,
            peer_state: HashMap::new(),
        }
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


    pub async fn connect(&self, peer: &Peer) -> (SocketAddr, PeerState) {
        let sock_addr = (peer.addr.host.to_owned(), peer.addr.port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();
        let sock = match UdpSocket::bind("0.0.0.0:0").await {
            Ok(sock) => sock, 
            Err(e) => {
                warn!("was unable to bind due to {}", e);
                return (sock_addr, PeerState::Dead);
            }
        };

        let p = Packet::Ping(self.addr.clone());

        let mut bytes = BytesMut::new();
        if let Err(e) = encode(p, &mut bytes) {
            error!("was not able to encode due to {}", e);
            return (sock_addr, PeerState::Dead);
        }

        if let Err(e) = sock.send_to(&bytes, sock_addr).await {
            error!("was not able to send due to {}", e);
            return (sock_addr, PeerState::Dead);
        }
        (sock_addr, PeerState::Alive)
    }

    pub async fn bind(&self) -> std::io::Result<()> {
        let sock_addr = self.addr.to_socket_addrs().unwrap().next().unwrap();
        info!("Starting server on {}", sock_addr);
        let ssock = UdpSocket::bind(sock_addr).await?;

        tokio::spawn(async move {
            let mut buf = [0; DATAGRAM_MAX_SZ];
            loop {
                let (usize, addr) = match ssock.recv_from(&mut buf).await {
                    Ok(d) => d,
                    Err(e) => {
                        // TODO: mark addr as failed
                        warn!("was not able to recv due to {}", e);
                        continue;
                    }
                };
            }
        });
        Ok(())
    }
    
    pub async fn run_server(&mut self) {
        let mut interval = time::interval(Duration::from_millis(self.period));
        loop {
            interval.tick().await;
            
            for _ in (0..self.failure_group_sz) {
                let i = self.rng.gen_range(0..self.peers.len());
                let p = &self.peers[i];
                let (addr, state) = match self.peer_state.get(p) {
                    Some(d) => d, 
                    None => continue
                };

                let mut bytes = BytesMut::new();
                
            }
        }
    }


    pub async fn run(&mut self) {
        info!("starting swim server");
        let mut futures = vec![];

        if let Err(e) = self.bind().await {
            error!("Was not able to join due to {}", e); 
            return;
        }

        for peer in &self.peers {
            if peer.id == self.id {
                continue;
            }
            let future = self.connect(peer);
            futures.push(future);
        }

        let states = join_all(futures).await;

        for (i, peer) in self.peers.iter().enumerate() {
           self.peer_state.insert(peer.clone(), states[i]); 
        }

        self.run_server().await;
    }
}
