//! Kernel-reported identity for the server side of the fixed pipe.

use std::io;

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream};

use crate::ClientError;

pub(crate) type PipeStream = DuplexPipeStream<pipe_mode::Bytes>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipePeerIdentity {
    process_id: u32,
    session_id: u32,
}

impl PipePeerIdentity {
    pub(crate) fn process_id(self) -> u32 {
        self.process_id
    }

    pub(crate) fn session_id(self) -> u32 {
        self.session_id
    }
}

pub(crate) fn observe_server(stream: &PipeStream) -> Result<PipePeerIdentity, ClientError> {
    let process_id = stream.server_process_id().map_err(map_transport_error)?;
    let session_id = stream.server_session_id().map_err(map_transport_error)?;
    if process_id == 0 {
        return Err(ClientError::PeerAuthentication);
    }
    Ok(PipePeerIdentity {
        process_id,
        session_id,
    })
}

pub(crate) fn reobserve_server(
    stream: &PipeStream,
    expected: PipePeerIdentity,
) -> Result<(), ClientError> {
    let observed = observe_server(stream)?;
    if observed != expected {
        return Err(ClientError::PeerAuthentication);
    }
    Ok(())
}

fn map_transport_error(_error: io::Error) -> ClientError {
    ClientError::Transport
}
