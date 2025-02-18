use crate::codec::BitcoinCodec;
use crate::config;
use crate::errors::ConnectionError;
use std::net::SocketAddr;

use bitcoin::p2p::message::{NetworkMessage, RawNetworkMessage};
use bitcoin::p2p::message_network::VersionMessage;
use bitcoin::p2p::{Address, ServiceFlags};
use bitcoin::Network;
use futures::{SinkExt, StreamExt, TryFutureExt};
use rand::Rng;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_util::codec::Framed;

pub async fn connect(
    remote_address: &SocketAddr,
) -> Result<Framed<TcpStream, BitcoinCodec>, ConnectionError> {
    let config = config::build();
    let connection = TcpStream::connect(remote_address).map_err(ConnectionError::ConnectionFailed);

    // Most nodes will quickly respond. If they don't, we'll probably want to talk to other nodes instead.
    let stream = timeout(
        Duration::from_millis(config.connection_timeout_millisec),
        connection,
    )
    .map_err(ConnectionError::ConnectionTimedOut)
    .await??;

    let framed = Framed::new(stream, BitcoinCodec {});
    Ok(framed)
}

/// Perform a Bitcoin handshake as per [this protocol documentation](https://en.bitcoin.it/wiki/Protocol_documentation)
pub async fn perform_handshake(
    stream: &mut Framed<TcpStream, BitcoinCodec>,
    peer_address: &SocketAddr,
    local_address: &SocketAddr,
) -> Result<(), ConnectionError> {
    let version_message = RawNetworkMessage::new(
        Network::Bitcoin.magic(),
        NetworkMessage::Version(build_version_message(peer_address, local_address)),
    );

    stream
        .send(version_message)
        .await
        .map_err(ConnectionError::SendingFailed)?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => match message.payload() {
                NetworkMessage::Version(remote_version) => {
                    tracing::info!("Version message: {:?}", remote_version);

                    stream
                        .send(RawNetworkMessage::new(
                            Network::Bitcoin.magic(),
                            NetworkMessage::Verack,
                        ))
                        .await
                        .map_err(ConnectionError::SendingFailed)?;

                    return Ok(());
                }
                other_message => {
                    // We're only interested in the version message right now. Keep the loop running.
                    tracing::debug!("Unsupported message: {:?}", other_message);
                }
            },
            Err(err) => {
                tracing::error!("Decoding error: {}", err);
            }
        }
    }

    Err(ConnectionError::ConnectionLost)
}

pub fn build_version_message(
    receiver_address: &SocketAddr,
    sender_address: &SocketAddr,
) -> VersionMessage {
    let config = config::build();

    const SERVICES: ServiceFlags = ServiceFlags::NONE;

    let sender = Address::new(sender_address, SERVICES);
    let timestamp = chrono::Utc::now().timestamp();
    let receiver = Address::new(receiver_address, SERVICES);
    let nonce = rand::thread_rng().gen();

    VersionMessage::new(
        SERVICES,
        timestamp,
        receiver,
        sender,
        nonce,
        config.user_agent,
        // The height of the block that the node is currently at.
        // We are always at the genesis block. because our implementation is not a real node.
        config.start_height,
    )
}
