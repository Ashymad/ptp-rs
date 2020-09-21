#[macro_use]
extern crate bit_serialize_derive;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate nix;

use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};

use nix::errno::Errno::EAGAIN;
use nix::sys::socket::sockopt::ReceiveTimestamp;
use nix::sys::socket::{recvmsg, setsockopt, MsgFlags};
use nix::sys::time::TimeVal;
use nix::sys::uio::IoVec;
use nix::Error::Sys;

use std::os::unix::io::AsRawFd;

use ifaces::interface::{Interface, Kind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod protocol;
use protocol::parser::parse_ptp_message;

use docopt::Docopt;

const EVENT_MSG: Token = Token(0);
const GENERAL_MSG: Token = Token(0);

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
    assert!(setsockopt(socket_event.as_raw_fd(), ReceiveTimestamp, &true).is_ok());
    socket_general
        .join_multicast_v4(&multicast_addr, &iface_addr)
        .unwrap();
    socket_event
        .join_multicast_v4(&multicast_addr, &iface_addr)
        .unwrap();
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(128);

    poll.registry()
        .register(&mut socket_event, EVENT_MSG, Interest::READABLE)
        .unwrap();
    poll.registry()
        .register(&mut socket_general, GENERAL_MSG, Interest::READABLE)
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
                if event.token() == EVENT_MSG {
                    let mut cmsg_buf = cmsg_space!(TimeVal);
                    let iov_buf = IoVec::from_mut_slice(&mut buf[..]);
                    let msg = match recvmsg(
                        socket_event.as_raw_fd(),
                        &[iov_buf],
                        Some(&mut cmsg_buf),
                        MsgFlags::MSG_WAITALL,
                    ) {
                        Ok(data) => data,
                        Err(Sys(EAGAIN)) => continue,
                        Err(err) => panic!("Couldn't read data from socket: {:?}", err),
                    };
                    println!(
                        "Received ptp message from: {}",
                        msg.address.unwrap().to_str()
                    );
                    for cmsg in msg.cmsgs() {
                        println!("Ancillary data: {:?}", cmsg);
                    }
                } else if event.token() == GENERAL_MSG {
                    let (_number_of_bytes, src_addr) = socket_general
                        .recv_from(&mut buf)
                        .expect("Didn't receive data");
                    println!("Received ptp message from: {}", src_addr);
                } else {
                    panic!("Unknown token: {:?}", event.token())
                };
                let msg = parse_ptp_message(&buf).unwrap();
                println!("{:#?}", msg.1);
            }
        }
    }
}
