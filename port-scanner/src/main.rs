extern crate env_logger;

use std::net::Ipv4Addr;
use std::{collections::HashMap, env, fs, process};

use log::error;

use pnet::packet::tcp::TcpFlags;

// const TCP_SIZE: usize = 20;

fn main() {
    println!("Hello, port-scanner!");

    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        error!("Bad nunber of arguments. [ipaddr] [scantype]");
        process::exit(1);
    }

    let packet_info = {
        let contents = fs::read_to_string(".env").expect("Failed to read env file");
        let lines: Vec<_> = contents.split('\n').collect();
        let mut map = HashMap::new();
        for line in lines {
            let elm: Vec<_> = line.split('=').map(str::trim).collect();
            if elm.len() == 2 {
                map.insert(elm[0], elm[1]);
            }
        }
        PacketInfo {
            my_ipaddr: map["MY_IPADDR"].parse().expect("invalid ipaddr"),
            target_ipaddr: args[1].parse().expect("invalid target ipaddr"),
            my_port: map["MY_PORT"].parse().expect("invalid port number"),
            maximum_port: map["MAXIMUM_PORT_NUM"]
                .parse()
                .expect("invalid maximum port num"),
            scan_type: match args[2].as_str() {
                "sS" => ScanType::Syn,
                "sF" => ScanType::Fin,
                "sX" => ScanType::Xmas,
                "sN" => ScanType::Null,
                _ => {
                    error!("Undefined scan method, only accept [sS|sF|sN|sX].");
                    process::exit(1);
                }
            },
        }
    };
}

#[derive(Copy, Clone)]
enum ScanType {
    Syn = TcpFlags::SYN as isize,
    Fin = TcpFlags::FIN as isize,
    Xmas = (TcpFlags::FIN | TcpFlags::URG | TcpFlags::PSH) as isize,
    Null = 0,
}

struct PacketInfo {
    my_ipaddr: Ipv4Addr,
    target_ipaddr: Ipv4Addr,
    my_port: u16,
    maximum_port: u16,
    scan_type: ScanType,
}

impl PacketInfo {
    fn new(map: HashMap<String, String>, args: String) -> Self {
        return Self {
            my_ipaddr: map["MY_IPADDR"].parse().expect("invalid ipaddr"),
            target_ipaddr: args[1].parse().expect("invalid target ipaddr"),
            my_port: map["MY_PORT"].parse().expect("invalid port number"),
            maximum_port: map["MAXIMUM_PORT_NUM"]
                .parse()
                .expect("invalid maximum port num"),
            scan_type: match args[2].as_str() {
                "sS" => ScanType::Syn,
                "sF" => ScanType::Fin,
                "sX" => ScanType::Xmas,
                "sN" => ScanType::Null,
                _ => {
                    error!("Undefined scan method, only accept [sS|sF|sN|sX].");
                    process::exit(1);
                }
            },
        };
    }
}
