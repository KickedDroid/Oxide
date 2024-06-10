use std::str::FromStr;

use anyhow::Context;
use iroh_net::key::SecretKey;
use tokio::io::{self, AsyncRead, AsyncWrite};
use tokio_util::sync::CancellationToken;

/// Copy from a reader to a quinn stream.
///
/// Will send a reset to the other side if the operation is cancelled, and fail
/// with an error.
///
/// Returns the number of bytes copied in case of success.
pub async fn copy_to_quinn(
    mut from: impl AsyncRead + Unpin,
    mut send: iroh_quinn::SendStream,
    token: CancellationToken,
) -> io::Result<u64> {
    tracing::trace!("copying to quinn");
    tokio::select! {
        res = tokio::io::copy(&mut from, &mut send) => {
            let size = res?;
            send.finish().await?;
            Ok(size)
        }
        _ = token.cancelled() => {
            // send a reset to the other side immediately
            send.reset(0u8.into()).ok();
            Err(io::Error::new(io::ErrorKind::Other, "cancelled"))
        }
    }
}

/// Copy from a quinn stream to a writer.
///
/// Will send stop to the other side if the operation is cancelled, and fail
/// with an error.
///
/// Returns the number of bytes copied in case of success.
pub async fn copy_from_quinn(
    mut recv: iroh_quinn::RecvStream,
    mut to: impl AsyncWrite + Unpin,
    token: CancellationToken,
) -> io::Result<u64> {
    tokio::select! {
        res = tokio::io::copy(&mut recv, &mut to) => {
            Ok(res?)
        },
        _ = token.cancelled() => {
            recv.stop(0u8.into()).ok();
            Err(io::Error::new(io::ErrorKind::Other, "cancelled"))
        }
    }
}

/// Get the secret key or generate a new one.
///
/// Print the secret key to stderr if it was generated, so the user can save it.
pub fn get_or_create_secret() -> anyhow::Result<SecretKey> {
    match std::env::var("OXIDE_SECRET") {
        Ok(secret) => SecretKey::from_str(&secret).context("invalid secret"),
        Err(_) => {
            let key = SecretKey::generate();
            Ok(key)
        }
    }
}

pub fn cancel_token<T>(token: CancellationToken) -> impl Fn(T) -> T {
    move |x| {
        token.cancel();
        x
    }
}
