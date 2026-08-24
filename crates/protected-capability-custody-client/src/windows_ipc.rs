mod connect;
mod connect_pipe;
mod io;
mod peer;
mod session;

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream};
use ocentra_protected_capability_custody_core::broker_admission::BrokerExecutableGuard;
use ocentra_protected_capability_custody_protocol::handshake::UntrustedBrokerHello;
use ocentra_protected_capability_custody_protocol::types::{
    BootstrapAuthenticator, SessionTranscriptDigest,
};

use crate::admission::AuthenticatedBrokerSession;
use crate::ClientError;

type PipeStream = DuplexPipeStream<pipe_mode::Bytes>;

pub(crate) struct WindowsBrokerSession {
    pub(super) stream: PipeStream,
    pub(super) broker_hello: UntrustedBrokerHello,
    pub(super) transcript_digest: SessionTranscriptDigest,
    pub(super) authenticator: BootstrapAuthenticator,
    pub(super) _broker_executable: BrokerExecutableGuard,
}

pub(crate) fn connect() -> Result<AuthenticatedBrokerSession, ClientError> {
    connect::connect()
}
