mod icmp;
mod rawsock;

use std::net::ToSocketAddrs;
use std::process;

const SIZE_OF_IP_V4_PACK: usize = 20;

fn main() {
    let id = process::id();
    let sock = rawsock::RawSocket::new_icmp(64, 200).unwrap();
    let icmp_pck = icmp::Packet::<2>::new_echo_req(id as u16, 0x0, &[0xFFu8; 2]);

    let dst_addr = "google.com:8080".to_socket_addrs().unwrap().nth(0).unwrap();

    sock.sendto(icmp_pck.raw(), dst_addr).unwrap();

    let mut buff = [0u8; SIZE_OF_IP_V4_PACK + 8 + 2];
    sock.recvfrom(&mut buff).unwrap();

    println!("buff = {:X?}", &buff[SIZE_OF_IP_V4_PACK..]);
    let rcv_icmp_pkt = icmp::Packet::<2>::from_raw(&buff[SIZE_OF_IP_V4_PACK..]).unwrap();
    println!("pck  = {:X?}", rcv_icmp_pkt.raw());
}
