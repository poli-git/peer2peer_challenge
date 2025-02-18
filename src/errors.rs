use std::io;
use thiserror::Error;
use tokio::time::error::Elapsed;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Connection failed: {0:?}")]
    ConnectionFailed(io::Error),

    #[error("Connection timed out")]
    ConnectionTimedOut(Elapsed),

    #[error("Connection lost")]
    ConnectionLost,

    #[error("Sending failed")]
    SendingFailed(io::Error),
}
