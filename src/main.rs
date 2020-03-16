#[macro_use]
extern crate bit_serialize_derive;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate serde_derive;

use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};

use ifaces::interface::{Interface, Kind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod protocol;
use protocol::parser::parse_ptp_message;

use docopt::Docopt;

const USAGE: &'static str = "
Rust PTP stack

Usage:
  ptp -i <iface> | --interface=<iface>
  ptp (-h | --help)

Options:
  -h --help                       Show this screen.
  -i <iface> --interface=<iface>  Choose network interface
";

#[derive(Debug, Deserialize)]
pub struct Args {
    flag_interface: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let iface_addr = if let std::net::IpAddr::V4(iface_addr) = Interface::get_all()
        .expect("Couldn't get interfaces")
        .iter()
        .filter(|iface| iface.name == args.flag_interface && iface.kind == Kind::Ipv4)
        .next()
        .expect(&format!("Couldn't find iface: {}", args.flag_interface))
        .addr
        .expect(&format!(
            "Could not get address of iface: {}",
            args.flag_interface
        ))
        .ip()
    {
        iface_addr
    } else {
        panic!("Address invalid!")
    };

    let sigint = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&sigint)).unwrap();

    let multicast_addr = "224.0.1.129".parse().unwrap();
    let bind_addr_event = "0.0.0.0:319".parse().unwrap();
    let bind_addr_general = "0.0.0.0:320".parse().unwrap();
    let mut socket_general = UdpSocket::bind(bind_addr_general).unwrap();
    let mut socket_event = UdpSocket::bind(bind_addr_event).unwrap();
    socket_general
        .join_multicast_v4(&multicast_addr, &iface_addr)
        .unwrap();
    socket_event
        .join_multicast_v4(&multicast_addr, &iface_addr)
        .unwrap();
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(128);

    poll.registry()
        .register(&mut socket_event, Token(0), Interest::READABLE)
        .unwrap();
    poll.registry()
        .register(&mut socket_general, Token(1), Interest::READABLE)
        .unwrap();

    println!(
        "Listening on: General: {}, Event: {}, Multicast: {}",
        bind_addr_general, bind_addr_event, multicast_addr
    );
    while !sigint.load(Ordering::Relaxed) {
        if let Err(err) = poll.poll(&mut events, None) {
            if err.kind() == std::io::ErrorKind::Interrupted {
                eprintln!("Poll interrupted");
            } else {
                panic!("Poll error: {}", err);
            }
        }

        for event in &events {
            if event.is_readable() {
                let mut buf = [0u8; 64];
                let (_number_of_bytes, src_addr) = if event.token() == Token(0) {
                    socket_event
                        .recv_from(&mut buf)
                        .expect("Didn't receive data")
                } else if event.token() == Token(1) {
                    socket_general
                        .recv_from(&mut buf)
                        .expect("Didn't receive data")
                } else {
                    panic!("Unknown token: {:?}", event.token())
                };
                let msg = parse_ptp_message(&buf).unwrap().1;
                println!("Received ptp message from: {}\n{:#?} ", src_addr, msg);
            }
        }
    }
}
