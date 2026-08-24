use std::thread;

use ocentra_protected_capability_custody_core::broker_admission::BrokerExecutableGuard;
use ocentra_protected_capability_custody_protocol::bootstrap::BootstrapPacket;
use ocentra_protected_capability_custody_protocol::constants::BROKER_RESTART_ATTEMPTS;
use ocentra_protected_capability_custody_protocol::handshake::UntrustedClientHello;
use ocentra_protected_capability_custody_protocol::types::CorrelationId;
use zeroize::Zeroizing;

use super::{connect_pipe, io, peer, WindowsBrokerSession};
use crate::admission::AuthenticatedBrokerSession;
use crate::ClientError;

pub(super) fn connect() -> Result<AuthenticatedBrokerSession, ClientError> {
    let mut last_error = ClientError::BrokerUnavailable;
    for attempt in 0..BROKER_RESTART_ATTEMPTS {
        match connect_once() {
            Ok(session) => return Ok(session),
            Err(error) if retryable(&error) && attempt + 1 < BROKER_RESTART_ATTEMPTS => {
                last_error = error;
                thread::sleep(io::POLL_INTERVAL);
            }
            Err(error) => return Err(error),
        }
    }
    Err(last_error)
}

fn connect_once() -> Result<AuthenticatedBrokerSession, ClientError> {
    let client_identity = peer::current_process_identity()?;
    let broker_executable =
        BrokerExecutableGuard::open_client_sibling().map_err(map_broker_guard_error)?;
    let bootstrap = BootstrapPacket::generate(
        client_identity.process_id,
        client_identity.process_epoch,
        client_identity.session_id,
    )?;
    let bootstrap_frame = Zeroizing::new(
        ocentra_protected_capability_custody_protocol::encode_bootstrap(&bootstrap)?,
    );
    let mut stream = connect_pipe::connect_pipe(
        &ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName::fixed(),
    )?;
    stream
        .set_nonblocking(true)
        .map_err(io::map_transport_error)?;
    peer::authenticate_pipe_server(&stream, &broker_executable)?;
    io::write_frame(
        &mut stream,
        bootstrap_frame.as_ref(),
        io::connection_deadline()?,
    )?;
    let client_hello = UntrustedClientHello::try_new(
        // The OS-created pipe connection owns the bootstrap nonce. Reusing it
        // in the first hello binds the wire transcript to this exact pipe
        // instance instead of accepting an independently replayable hello.
        bootstrap.identity().pipe_nonce(),
        CorrelationId::generate()?,
        client_identity.process_id,
        client_identity.process_epoch,
        client_identity.session_id,
    )?;
    let encoded_client_hello = Zeroizing::new(
        ocentra_protected_capability_custody_protocol::encode_client_hello(&client_hello)?,
    );
    io::write_frame(
        &mut stream,
        encoded_client_hello.as_ref(),
        io::connection_deadline()?,
    )?;
    let broker_frame = Zeroizing::new(io::read_frame(&mut stream, io::connection_deadline()?)?);
    let broker_hello =
        ocentra_protected_capability_custody_protocol::decode_broker_hello(broker_frame.as_ref())?;
    peer::authenticate_broker_hello(&stream, &broker_executable, &client_hello, &broker_hello)?;
    let transcript_digest =
        broker_hello.verify_authenticated_provenance(&client_hello, io::unix_now_millis()?)?;
    let authenticator = broker_hello.clone_authenticator();
    Ok(AuthenticatedBrokerSession {
        inner: WindowsBrokerSession {
            stream,
            broker_hello,
            transcript_digest,
            authenticator,
            _broker_executable: broker_executable,
        },
    })
}

fn retryable(error: &ClientError) -> bool {
    matches!(
        error,
        ClientError::BrokerUnavailable | ClientError::Transport | ClientError::PeerAuthentication
    )
}

fn map_broker_guard_error(
    _error: ocentra_protected_capability_custody_core::broker_admission::BrokerRuntimeError,
) -> ClientError {
    ClientError::BrokerUnavailable
}
