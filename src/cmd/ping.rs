use crate::net::icmp::{self, Packet};
use crate::net::rawsock::RawSocket;

use std::fmt;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::process;
use std::time::Duration;
use std::time::Instant;

use anyhow::{anyhow, Result};
use crossbeam_channel::{bounded, select, tick, Receiver};
use ctrlc;

pub struct Ping {
    sock: RawSocket,
    id: u32,
    seq: u16,
    host: SocketAddr,
    host_name: String,
    wait: Duration,
    size: usize,

    buff: Vec<u8>,
    round_trip: Duration,
}

impl fmt::Display for Ping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} bytes from {}: icmp_seq={} time={} ms",
            self.size + icmp::HDR_BYTE_SIZE,
            self.host.ip(),
            self.seq,
            self.round_trip.as_millis(),
        )
    }
}

impl Ping {
    pub fn new_default(host_name: &str) -> Result<Self> {
        let mut host = host_name.to_string();
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
            id: process::id(),
            host: addr,
            host_name: host_name.to_string(),
            wait: Duration::from_secs(2),
            seq: 0,
            size,
            buff: vec![0; SIZE_OF_IP_V4_PACK + icmp::HDR_BYTE_SIZE + size],
            round_trip: Duration::from_millis(0),
        })
    }
}

const SIZE_OF_IP_V4_PACK: usize = 20;

impl Ping {
    pub fn ping(&mut self) -> Result<()> {
        println!(
            "PING {} ({}): {} data bytes",
            self.host_name,
            self.host.ip(),
            self.size
        );
        let ctrl_c_events = ctrl_channel()?;
        let ticks = tick(self.wait);

        loop {
            select! {
                recv(ticks) -> _ => {
                    // TODO: correctly handle the error
                    self.echo()?;
                    // TODO: gather statistics
                }
                recv(ctrl_c_events) -> _ => {
                    println!();
                    println!("--- {} ping statistics ---", self.host_name);
                    // TODO: print statistics
                    break;
                }
            }
        }

        Ok(())
    }

    fn echo(&mut self) -> Result<Duration> {
        let now = Instant::now();
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
        self.check_echo_reply(&rcv_icmp_pkt)?;
        self.round_trip = now.elapsed();

        println!("{}", self);

        self.seq += 1;
        Ok(self.round_trip)
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

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}
