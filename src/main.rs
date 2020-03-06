extern crate signal_hook;

use mio::{Events, Poll, Interest, Token};
use mio::net::UdpSocket;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use std::net::{SocketAddr};

fn main() {
    let sigint = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&sigint)).unwrap();

    let addr: SocketAddr = "0.0.0.0:319".parse().unwrap();
    let mut socket = UdpSocket::bind(addr).unwrap();

    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(128);

    poll.registry().register(&mut socket, Token(0), Interest::READABLE).unwrap();

    println!("Listening on: {}", addr);
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
}
