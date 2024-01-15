/******************************************
 *        Copyright (c) xTekC.            *
 *        Licensed under MPL-2.0.         *
 *        See LICENSE for details.        *
 * https://www.mozilla.org/en-US/MPL/2.0/ *
 ******************************************/

use owo_colors::{self, OwoColorize};
use pnet::datalink;
use std::net::IpAddr;

pub fn active_interface_name() {
    for iface in datalink::interfaces() {
        if iface.ips.iter().any(|ip| matches!(ip.ip(), IpAddr::V4(ipv4_addr) if ipv4_addr.to_string().starts_with("192.168"))) {
             println!("{}", iface.name.yellow());
         }
    }
}

pub fn active_ipv4_address() {
    let ifaces = datalink::interfaces();
    for iface in ifaces {
        for ip in iface.ips {
            if let IpAddr::V4(ipv4_addr) = ip.ip() {
                if ipv4_addr.to_string().starts_with("192.168") {
                    println!("IPv4: {}", ipv4_addr.magenta());
                }
            }
        }
    }
}

pub fn active_ipv6_address() {
    for iface in datalink::interfaces() {
        let has_ipv4_192 = iface.ips.iter().any(|ip| matches!(ip.ip(), IpAddr::V4(ipv4_addr) if ipv4_addr.to_string().starts_with("192.168")));

        if has_ipv4_192 {
            for ip in iface.ips {
                if let IpAddr::V6(ipv6_addr) = ip.ip() {
                    println!("IPv6: {}", ipv6_addr.to_string().magenta());
                }
            }
        }
    }
}

pub fn all_network_info() {
    println!();
    for iface in datalink::interfaces() {
        let mac_address = match iface.mac {
            Some(mac) => mac.to_string(),
            None => String::from("None"),
        };

        println!(
            "{}\nUp: {:?}\nMAC: {}",
            iface.name.yellow(),
            iface.is_up().green(),
            mac_address.red()
        );

        for ip in iface.ips {
            let ip_version = match ip.ip() {
                IpAddr::V4(_) => "IPv4",
                IpAddr::V6(_) => "IPv6",
            };

            let ip_string = ip.ip().to_string();
            println!("{}: {}", ip_version, ip_string.magenta());
        }
        println!();
    }
}
