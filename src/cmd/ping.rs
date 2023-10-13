use crate::net::icmp::{self, Packet};
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
    seq: u16,
    host: SocketAddr,
    wait: Duration,
    size: usize,

    buff: Vec<u8>,
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
        let size = 64;

        Ok(Ping {
            sock,
            hops,
            timeout,
            id: process::id(),
            host: addr,
            wait: Duration::from_secs(2),
            seq: 0,
            size,
            buff: vec![0; SIZE_OF_IP_V4_PACK + icmp::HDR_BYTE_SIZE + size],
        })
    }
}

const SIZE_OF_IP_V4_PACK: usize = 20;

impl Ping {
    pub fn echo(&mut self) -> Result<Duration> {
        let icmp_pck = Packet::new_echo_req(self.id as u16, self.seq, vec![0xFFu8; self.size]);

        let send = self.sock.sendto(&icmp_pck.raw(), self.host)?;
        if send != icmp_pck.get_byte_size() {
            return Err(anyhow!(
                "send bytes size = {} must be equal size of a icmp packet = {}",
                send,
                icmp_pck.get_byte_size(),
            ));
        }

        let (recv, from) = self.sock.recvfrom(&mut self.buff)?;
        if recv - SIZE_OF_IP_V4_PACK != icmp_pck.get_byte_size() {
            return Err(anyhow!(
                "recv bytes size = {} must be equal size of a icmp packet = {}",
                recv,
                icmp_pck.get_byte_size(),
            ));
        }
        let from = from.unwrap();
        if from.ip() != self.host.ip() {
            return Err(anyhow!(
                "destenation host {} of send packet and souce host {} of recive packet shoud be equal",
                from.ip(),
                self.host.ip(),
            ));
        }

        let rcv_icmp_pkt = Packet::from_raw(&self.buff[SIZE_OF_IP_V4_PACK..]).unwrap();
        println!("pck  = {:X?}", rcv_icmp_pkt.raw());
        self.check_echo_reply(&rcv_icmp_pkt)?;

        println!("pck.data  = {:X?}", rcv_icmp_pkt.get_data());

        self.seq += 1;
        Ok(Duration::from_millis(5_000))
    }

    fn check_echo_reply(&self, pck: &Packet) -> Result<()> {
        if !pck.is_echo_reply() {
            return Err(anyhow!("expected echo reply icmp packet"));
        }

        if pck.get_echo_id()? != (self.id as u16) {
            return Err(anyhow!("packet id doesn't match"));
        }
        if pck.get_echo_seq()? != self.seq {
            return Err(anyhow!("packet sequence doesn't match"));
        }
        Ok(())
    }
}
