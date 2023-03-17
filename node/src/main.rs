//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

use std::env;

fn main() -> sc_cli::Result<()> {
    let path = env::args().nth(0).unwrap();
    let name = path.split('/').last().unwrap();
    if name == "polkadot" {
        polkadot_cli::run().unwrap();
        Ok(())
    } else {
        command::run()
    }
}
