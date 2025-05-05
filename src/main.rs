use clap::*;

mod client_m;
use client_m::*;

mod server_m;
use server_m::*;



#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Args {
    #[clap(long, action)]
    server: bool
}

pub fn main() {
    if Args::parse().server { server(); } else { client(); }
    // if !Args::parse().server { client(); } else { server(); }
}