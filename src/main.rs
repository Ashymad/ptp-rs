#[macro_use]
extern crate nom;

use mio::{Events, Poll, Interest, Token};
use mio::net::UdpSocket;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

mod parser;
use parser::parse_ptp_header;

fn main() {
    let sigint = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&sigint)).unwrap();

    let iface_addr = "192.168.0.10".parse().unwrap();
    let multicast_addr = "224.0.1.129".parse().unwrap();
    let bind_addr_event = "0.0.0.0:319".parse().unwrap();
    let bind_addr_general = "0.0.0.0:320".parse().unwrap();
    let mut socket_general = UdpSocket::bind(bind_addr_general).unwrap();
    let mut socket_event = UdpSocket::bind(bind_addr_event).unwrap();
    socket_general.join_multicast_v4(&multicast_addr, &iface_addr).unwrap();
    socket_event.join_multicast_v4(&multicast_addr, &iface_addr).unwrap();
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(128);

    poll.registry().register(&mut socket_event, Token(0), Interest::READABLE).unwrap();
    poll.registry().register(&mut socket_general, Token(1), Interest::READABLE).unwrap();

    println!("Listening on: General: {}, Event: {}, Multicast: {}", bind_addr_general, bind_addr_event, multicast_addr);
    while !sigint.load(Ordering::Relaxed) {
        if let Err(err) = poll.poll(&mut events, None) {
            if err.kind() == std::io::ErrorKind::Interrupted {
                println!("Poll interrupted");
            } else {
                panic!("Poll error: {}", err);
            }
        }

        for event in &events {
            if event.is_readable() {
                let mut buf = [0u8; 34];
                let (number_of_bytes, src_addr) = if event.token() == Token(0) {
                    socket_event.recv_from(&mut buf).expect("Didn't receive data")
                } else if event.token() == Token(1) {
                    socket_general.recv_from(&mut buf).expect("Didn't receive data")
                } else {
                    panic!("Unknown token: {:?}", event.token())
                };
                println!("Received ptp message from: {}\n{:?} ", src_addr, parse_ptp_header(&buf).unwrap());
            }
        }
    }
}
