use pnet::datalink::{self};
use pnet::ipnetwork::IpNetwork;

fn main() {
    // 获取所有网络接口
    let interfaces = datalink::interfaces();

    // 遍历每个网络接口
    for interface in interfaces {
        // 排除回环接口和无效接口
        if interface.is_loopback() || !interface.is_up() {
            continue;
        }

        // 获取接口的 IP 地址
        for ip_network in &interface.ips {
            if let IpNetwork::V4(ipv4_network) = ip_network {
                let ip_addr = ipv4_network.ip();
                println!("IP 地址: {}", ip_addr);
            }
        }
    }
}
