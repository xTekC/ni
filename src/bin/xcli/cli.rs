/******************************************
 *        Copyright (c) xTekC.            *
 *        Licensed under MPL-2.0.         *
 *        See LICENSE for details.        *
 * https://www.mozilla.org/en-US/MPL/2.0/ *
 ******************************************/

#![allow(unused)]
use clap::Parser;
use ni::xcore::core::{
    active_interface_name, active_ipv4_address, active_ipv6_address, all_network_info,
};
use owo_colors::OwoColorize;

/// Get Network Information
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// All Network Information
    #[arg(short, long)]
    all: bool,
}

pub async fn cli_main() {
    let cli = Args::parse();

    if cli.all {
        all_network_info();
    } else {
        println!();
        active_interface_name();
        active_ipv4_address();
        active_ipv6_address();
        println!();
    }
}
