mod config;
mod errors;
mod network;
mod trace_log;

use anyhow::Context;
use clap::{command, Parser};
use codec::BitcoinCodec;
use network::{connect, perform_handshake};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use trace_log::init_tracing;

const CLI_COMMAND_HANDSHAKE: &str = "handshake";
const CLI_COMMAND_PEERS: &str = "get-peers";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command to  be executed | [handshake: Handshake / get-peers: GetPeers]
    #[arg(short, long, default_value = "handshake")]
    command: String,

    /// The address of the node to reach to.
    /// `dig seed.bitcoin.sipa.be +short` may provide a fresh list of nodes.
    #[arg(short, long, default_value = "86.89.77.44:8333")]
    remote_address: String,

    /// The address of this local node.
    /// This address doesn't matter much as it will be ignored by the bitcoind node
    #[arg(short, long, default_value = "0.0.0.0:0")]
    local_address: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    init_tracing();

    // Read arguments
    let args = Args::parse();

    // Get command
    let command = args.command.as_str();

    match command {
        CLI_COMMAND_HANDSHAKE => {
            let remote_address = args
                .remote_address
                .parse::<SocketAddr>()
                .context("Invalid remote address")?;

            let local_address = args
                .local_address
                .parse::<SocketAddr>()
                .context("Invalid local address")?;

            let mut stream: Framed<TcpStream, BitcoinCodec> = connect(&remote_address).await?;

            perform_handshake(&mut stream, &remote_address, &local_address).await?;
        }
        CLI_COMMAND_PEERS => {
            // let peer_id = args.peer_id;
            // let _ = get_peers(peer_id).await;
        }

        _ => {
            println!(
                "Error: Invalid Command:{} - For more information, try --help",
                command
            );
        }
    }
    Ok(())
}
