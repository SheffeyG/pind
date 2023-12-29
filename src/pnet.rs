use pnet::datalink::{self, NetworkInterface};
use pnet::ipnetwork::IpNetwork;
use std::net::IpAddr;

fn main() {
    let interfaces = datalink::interfaces();

    for interface in interfaces {
        if interface.is_loopback() || !interface.is_up() {
            continue;
        }

        for ip_network in &interface.ips {
            if let IpNetwork::V4(ipv4_network) = ip_network {
                let ip_addr = ipv4_network.ip();
                println!("IP addr: {}", ip_addr);
            }
        }
    }
}
