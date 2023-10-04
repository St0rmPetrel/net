mod rawsock;

fn main() {
    let _rs = rawsock::RawSocket::new_icmp(10, 2).unwrap();
}
