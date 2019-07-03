extern crate env_logger;
extern crate failure;
extern crate log;

extern crate pnet;

use std::env;

use log::{error, info};

use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};

use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;

use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

mod packets;
use packets::GettableEndPoints;

const WIDTH: usize = 20;

fn main() {
    println!("Hello, packet-capture!");

    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let interface_name = parse_arg();

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interface");

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e),
    };

    loop {
        match rx.next() {
            Ok(frame) => {
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        ipv4_handler(&frame);
                    }
                    EtherTypes::Ipv6 => {
                        // ipv6_handler(&frame);
                        unimplemented!();
                    }
                    _ => {
                        info!("Not an IPv4 or IPv6");
                    }
                }
            }
            Err(e) => {
                error!("Failed to read: {}", e);
            }
        }
    }
}

fn parse_arg() -> String {
    match env::args().nth(1) {
        Some(interface_name) => {
            return interface_name;
        }
        None => {
            error!("Please specify target interface name");
            std::process::exit(1);
        }
    };
}

fn ipv4_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
        match packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHeaderProtocols::Udp => {
                // udp_handler(&packet);
                unimplemented!();
            }
            _ => {
                info!("Not a TCP or UDP packet");
            }
        }
    }
}

fn tcp_handler(packet: &dyn GettableEndPoints) {
    let tcp = TcpPacket::new(packet.get_payload());
    if let Some(tcp) = tcp {
        print_packet_info(packet, &tcp, "TCP");
    }
}

fn print_packet_info(l3: &dyn GettableEndPoints, l4: &dyn GettableEndPoints, proto: &str) {
    println!(
        "Captured a {} packet from {}|{} to {}|{}\n",
        proto,
        l3.get_source(),
        l4.get_source(),
        l3.get_destination(),
        l4.get_destination()
    );
    let payload = l4.get_payload();
    let len = payload.len();

    // ペイロードの表示
    // 指定した定数幅で表示を行う
    for i in 0..len {
        print!("{:<02X} ", payload[i]);
        if i % WIDTH == WIDTH - 1 || i == len - 1 {
            for _j in 0..WIDTH - 1 - (i % (WIDTH)) {
                print!("   ");
            }
            print!("| ");
            for j in i - i % WIDTH..=i {
                if payload[j].is_ascii_alphabetic() {
                    print!("{}", payload[j] as char);
                } else {
                    // 非ascii文字は.で表示
                    print!(".");
                }
            }
            println!();
        }
    }
    println!("{}", "=".repeat(WIDTH * 3));
    println!();
}
