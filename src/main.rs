use clap::*;
use t5f::{ client, server };

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Args {
    #[clap(long, action)]
    server: bool
}

pub fn main() {
    if Args::parse().server { server(); } else { client(); }
}