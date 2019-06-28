extern crate env_logger;
extern crate log;

mod tcp_client;
mod tcp_server;

use std::env;

use log::error;

fn main() {
    println!("Hello, This is server.");
    env::set_var("RUST_LOG", "debug");

    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];
    // let (protocol, role, address) = parse_args();

    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                tcp_client::connect(address).unwrap_or_else(|e| error!("{}", e));
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                unimplemented!();
            }
            "client" => {}
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}

// fn parse_args() -> (&'static str, &'static str, &'static str) {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 4 {
//         error!("Please specify [tcp|udp] [server|client] [addr:port].");
//         std::process::exit(1);
//     }
//     let protocol: &str = &args[1];
//     let role: &str = &args[2];
//     let address: &str = &args[3];
//     return (protocol, role, address);
// }

fn missing_role() {
    error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}
