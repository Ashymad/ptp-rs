use mio::{Events, Poll, Interest, Token};
use mio::net::UdpSocket;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use std::net::{SocketAddr, Ipv4Addr};

fn main() {
    let sigint = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&sigint)).unwrap();

    let addr_str = "192.168.0.10";
    let multicast_addr_str = "224.0.1.129";
    let port_str = "319";
    let soc_addr: SocketAddr = format!("{}:{}", addr_str, port_str).parse().unwrap();
    let addr: Ipv4Addr = addr_str.parse().unwrap();
    let multicast_addr: Ipv4Addr = multicast_addr_str.parse().unwrap();
    let mut socket = UdpSocket::bind(soc_addr).unwrap();
    socket.join_multicast_v4(&multicast_addr, &addr).unwrap();

    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(128);

    poll.registry().register(&mut socket, Token(0), Interest::READABLE).unwrap();

    println!("Listening on: {}, multicast group: {}", soc_addr, multicast_addr);
    while !sigint.load(Ordering::Relaxed) {
        if let Err(err) = poll.poll(&mut events, None) {
            if err.kind() == std::io::ErrorKind::Interrupted {
                println!("Poll interrupted");
            } else {
                panic!("Poll error: {}", err);
            }
        }

        for event in &events {
            if event.token() == Token(0) && event.is_readable() {
                let mut buf = [0u8; 34];
                let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
                                    .expect("Didn't receive data");
                println!("Received ptp message from: {}, of type: {}, length {} bytes", src_addr, buf[0] & 0x0F, number_of_bytes);
            }
        }
    }

    socket.leave_multicast_v4(&multicast_addr, &addr).unwrap();
}
