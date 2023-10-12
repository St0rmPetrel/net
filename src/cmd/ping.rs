use crate::net::icmp::Packet;
use crate::net::rawsock::RawSocket;

use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::process;
use std::time::Duration;

use anyhow::{anyhow, Result};

pub struct Ping {
    sock: RawSocket,
    hops: u8,
    timeout: Duration,
    id: u32,
    host: SocketAddr,
    wait: Duration,
    size: usize,
}

impl Ping {
    pub fn new_default(host: &str) -> Result<Self> {
        let mut host = host.to_string();
        host.push_str(":8080");

        let addr = match host.to_socket_addrs()?.next() {
            Some(addr) => addr,
            None => return Err(anyhow!("fail to parse host = {host}")),
        };

        let hops = 56;
        let timeout = Duration::from_secs(1);
        let sock = RawSocket::new_icmp(hops, timeout.as_secs() as u32)?;

        Ok(Ping {
            sock,
            hops,
            timeout,
            id: process::id(),
            host: addr,
            wait: Duration::from_secs(2),
            size: 64,
        })
    }
}

const SIZE_OF_IP_V4_PACK: usize = 20;

impl Ping {
    pub fn echo(&self, seq: u16) -> Result<Duration> {
        let icmp_pck = Packet::<2>::new_echo_req(self.id as u16, seq, &[0xFFu8; 2]);

        self.sock.sendto(icmp_pck.raw(), self.host).unwrap();

        let mut buff = [0u8; SIZE_OF_IP_V4_PACK + 8 + 2];
        self.sock.recvfrom(&mut buff).unwrap();

        println!("buff = {:X?}", &buff[SIZE_OF_IP_V4_PACK..]);
        let rcv_icmp_pkt = Packet::<2>::from_raw(&buff[SIZE_OF_IP_V4_PACK..]).unwrap();
        println!("pck  = {:X?}", rcv_icmp_pkt.raw());
        Ok(Duration::from_millis(5_000))
    }
}
