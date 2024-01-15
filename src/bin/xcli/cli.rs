/******************************************
 *        Copyright (c) xTekC.            *
 *        Licensed under MPL-2.0.         *
 *        See LICENSE for details.        *
 * https://www.mozilla.org/en-US/MPL/2.0/ *
 ******************************************/

#![allow(unused)]
use clap::Parser;
use ndi::xcore::core::*;

/// Get Network Device Information
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// ipaddress
    #[arg(short, long)]
    ip: String,
}

pub async fn cli_main() {
    let args = Args::parse();
}
