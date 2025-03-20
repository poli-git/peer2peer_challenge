mod codec;
mod config;
mod errors;
mod network;
mod trace_log;

use anyhow::Context;
use clap::Parser;
use codec::BitcoinCodec;
use network::{connect, perform_handshake};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use trace_log::init_tracing;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The address of the node to reach to.
    /// `dig seed.bitcoin.sipa.be +short` may provide a fresh list of nodes.
    #[arg(short, long, default_value = "103.246.186.138:8333")]
    remote_address: String,

    /// The address of this local node.
    /// This address doesn't matter much as it will be ignored by the bitcoind node
    #[arg(short, long, default_value = "0.0.0.0:0")]
    local_address: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let args = Args::parse();

    let remote_address = args
        .remote_address
        .parse::<SocketAddr>()
        .context("Invalid remote address")?;

    let local_address = args
        .local_address
        .parse::<SocketAddr>()
        .context("Invalid local address")?;

    let mut stream: Framed<TcpStream, BitcoinCodec> = connect(&remote_address).await?;

    Ok(perform_handshake(&mut stream, &remote_address, &local_address).await?)
}
