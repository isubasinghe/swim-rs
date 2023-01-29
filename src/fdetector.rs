use futures::SinkExt;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;

use bincode;
use bytes::{Buf, BytesMut};
use futures::{future::join_all, AsyncWriteExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::UdpSocket;
use tokio_util::{codec::Decoder, codec::Encoder, udp::UdpFramed};
use tracing::{error, info, warn};

const DATAGRAM_MAX_SZ: usize = 65_507;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Inquire {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Packet {
    Ping(),
    Ack(),
    AckRequire(),
    InformJoin(),
}

struct BinCodeCodec;

impl Decoder for BinCodeCodec {
    type Item = Packet;
    type Error = std::io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
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
}

impl Encoder<Packet> for BinCodeCodec {
    type Error = Error;
    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let data = match bincode::serialize(&item) {
            Ok(data) => data,
            Err(e) => {
                error!("error: {}", e);
                return Err(Error::new(ErrorKind::Other, e));
            }
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
}

pub type Id = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub peers: Vec<Peer>,
    pub period: u64,
    pub failure_group_sz: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Peer {
    id: Id,
    addr: Addr,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum PeerState {
    Alive,
    Suspect,
    Dead,
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
    pub fn new(
        id: Id,
        peers: Vec<Peer>,
        period: u64,
        failure_group_sz: u64,
    ) -> SwimFailureDetector {
        let rng = rand::thread_rng();
        SwimFailureDetector {
            id,
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

    pub fn add_node(&self, node_id: Vec<u8>, addr: Addr) {}

    pub async fn connect(&self, peer: &Peer) -> PeerState {
        let sock_addr = (peer.addr.host.to_owned(), peer.addr.port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();
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
            None => return Ok(()),
        };
        let sock_addr = host.to_socket_addrs().unwrap().next().unwrap();
        let ssock = UdpSocket::bind(sock_addr).await?;

        tokio::spawn(async move {
            let mut bcodec = BinCodeCodec {};
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

    pub async fn run(&mut self) {
        let mut futures = vec![];

        self.bind();
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
