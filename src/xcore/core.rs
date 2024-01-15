/******************************************
 *        Copyright (c) xTekC.            *
 *        Licensed under MPL-2.0.         *
 *        See LICENSE for details.        *
 * https://www.mozilla.org/en-US/MPL/2.0/ *
 ******************************************/

use pnet::datalink;
use std::net::IpAddr;

pub fn interface_name() {
    for iface in datalink::interfaces() {
        let name = iface.name;
        println!("{}", name);
    }
}

pub fn status_of_interface() {
    for iface in datalink::interfaces() {
        let status = iface.is_up();
        println!("Up: {}", status);
    }
}

pub fn mac_address() {
    for iface in datalink::interfaces() {
        let maca = match iface.mac {
            Some(mac) => mac.to_string(),
            None => String::from("None"),
        };
        println!("MAC: {}", maca);
    }
}

pub fn ip_address() {
    let ifaces = datalink::interfaces();
    for iface in ifaces {
        for ip in iface.ips {
            let ip_version = match ip.ip() {
                IpAddr::V4(_) => "IPv4",
                IpAddr::V6(_) => "IPv6",
            };
            let ip_string = ip.ip().to_string();
            println!("{} {}", ip_version, ip_string);
        }
    }
}
