#[path = "endpoint_accept.rs"]
mod accept_loop;

use std::ffi::OsString;

use interprocess::os::windows::named_pipe::PipeListenerOptions;
use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;

use super::{PipeListenerType, PipeStream, TransportError};

const MAXIMUM_PIPE_NAME_BYTES: usize = 256;

/// A validated local named-pipe endpoint.
///
/// BRAND-INVARIANT: the value has the reviewed local-pipe prefix, a bounded
/// non-empty suffix, and only alphanumeric, dash, underscore, or dot suffix
/// characters.
#[derive(Debug)]
pub(crate) struct PipeName(String);

impl PipeName {
    pub(crate) fn parse(value: &OsString) -> Result<Self, TransportError> {
        let text = value.to_str().ok_or(TransportError::InvalidFrame)?;
        let prefix = crate::protocol::text::TextId::PipePrefix.text();
        if text.len() > MAXIMUM_PIPE_NAME_BYTES || !text.starts_with(&prefix) {
            return Err(TransportError::InvalidFrame);
        }
        let suffix = &text[prefix.len()..];
        if suffix.is_empty()
            || !suffix
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        {
            return Err(TransportError::InvalidFrame);
        }
        Ok(Self(text.to_owned()))
    }

    pub(crate) fn text(&self) -> &str {
        &self.0
    }
}

pub(super) fn bind(name: &PipeName) -> Result<PipeListenerType, TransportError> {
    PipeListenerOptions::new()
        .path(name.text())
        .nonblocking(false)
        .accept_remote(false)
        .inheritable(false)
        .create_duplex::<interprocess::os::windows::named_pipe::pipe_mode::Bytes>()
        .map_err(|_error| TransportError::Io)
}

pub(super) fn accept(
    listener: &PipeListenerType,
    parent: &ParentProcessObservation,
) -> Result<PipeStream, TransportError> {
    accept_loop::accept(listener, parent)
}

pub(super) fn verify_client(
    parent: &ParentProcessObservation,
    stream: &PipeStream,
) -> Result<(), TransportError> {
    accept_loop::verify_client(parent, stream)
}
