use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream};
use ocentra_protected_capability_custody_core::broker_admission::BrokerExecutableGuard;
use ocentra_protected_capability_custody_protocol::bootstrap::BootstrapPacket;
use ocentra_protected_capability_custody_protocol::constants::{
    INITIAL_SESSION_SEQUENCE, MAX_REQUEST_TTL_MILLIS,
};
use ocentra_protected_capability_custody_protocol::handshake::{
    UntrustedBrokerHello, UntrustedClientHello,
};
use ocentra_protected_capability_custody_protocol::request::{
    RequestSessionEnvelope, UntrustedRequest, UntrustedRequestValues,
};
use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;
use ocentra_protected_capability_custody_protocol::types::{
    BootstrapAuthenticator, CorrelationId, Nonce, SessionTranscriptDigest,
};
use sysinfo::{Pid, System};
use zeroize::Zeroizing;

use crate::admission::{AuthenticatedBrokerSession, AuthenticatedResponse, ClientRequest};
use crate::{map_transport_error, ClientError};

mod broker_process;
mod connect;

use broker_process::{fixed_broker_path, process_executable_matches, spawn_broker, BrokerChild};

const CONNECT_DEADLINE_MILLIS: u64 = 5_000;
const CONNECT_RETRY_MILLIS: u64 = 10;

pub(crate) struct WindowsBrokerSession {
    stream: DuplexPipeStream<pipe_mode::Bytes>,
    child: BrokerChild,
    broker_hello: UntrustedBrokerHello,
    transcript_digest: SessionTranscriptDigest,
    authenticator: BootstrapAuthenticator,
    _broker_executable: BrokerExecutableGuard,
}

impl WindowsBrokerSession {
    pub(crate) fn execute(
        mut self,
        request: ClientRequest,
    ) -> Result<AuthenticatedResponse, ClientError> {
        let now = unix_now_millis()?;
        let request_expiry = now
            .checked_add(MAX_REQUEST_TTL_MILLIS)
            .ok_or(ClientError::PeerAuthentication)?
            .min(self.broker_hello.session_expires_at_unix_millis());
        let session = RequestSessionEnvelope::from_authenticated_hello(
            &self.broker_hello,
            self.transcript_digest,
            INITIAL_SESSION_SEQUENCE,
            request_expiry,
        )?;
        let wire_request = UntrustedRequest::authenticate_wire(
            UntrustedRequestValues {
                session,
                expected_generations: request.expected_generations,
                kind: request.kind,
                operation: request.operation,
                action: request.action,
                target: request.target,
                opaque_token: request.opaque_token,
            },
            &self.authenticator,
        )?;
        let encoded_request = Zeroizing::new(
            ocentra_protected_capability_custody_protocol::encode_request(&wire_request)?,
        );
        write_encoded(&mut self.stream, encoded_request.as_ref())?;
        let response_frame = Zeroizing::new(
            ocentra_protected_capability_custody_protocol::read_frame(&mut self.stream)?,
        );
        let response = ocentra_protected_capability_custody_protocol::decode_response(
            response_frame.as_ref(),
        )?;
        response.verify_authenticated_session(
            &wire_request,
            unix_now_millis()?,
            &self.authenticator,
        )?;
        let status = self.child.wait().map_err(map_transport_error)?;
        if !status.success() {
            return Err(ClientError::Transport);
        }
        Ok(AuthenticatedResponse::from_verified(response))
    }
}

pub(crate) fn connect() -> Result<AuthenticatedBrokerSession, ClientError> {
    connect::retry(connect_once)
}

fn connect_once() -> Result<AuthenticatedBrokerSession, ClientError> {
    let client_identity = current_process_identity()?;
    let bootstrap = BootstrapPacket::generate(
        client_identity.process_id,
        client_identity.process_epoch,
        client_identity.session_id,
    )?;
    let pipe_name = BrokerPipeName::from_nonce(bootstrap.identity().pipe_nonce());
    let broker_path = fixed_broker_path()?;
    let bootstrap_frame = Zeroizing::new(
        ocentra_protected_capability_custody_protocol::encode_bootstrap(&bootstrap)?,
    );
    let mut child = spawn_broker(&broker_path, &pipe_name)?;
    write_bootstrap(&mut child, bootstrap_frame.as_ref())?;
    let mut stream = connect_pipe(&pipe_name)?;
    authenticate_pipe_server(&stream, &child, &broker_path, client_identity.session_id)?;
    let client_hello = UntrustedClientHello::try_new(
        Nonce::generate()?,
        CorrelationId::generate()?,
        client_identity.process_id,
        client_identity.process_epoch,
        client_identity.session_id,
    )?;
    write_encoded(
        &mut stream,
        &ocentra_protected_capability_custody_protocol::encode_client_hello(&client_hello)?,
    )?;
    let broker_frame = ocentra_protected_capability_custody_protocol::read_frame(&mut stream)?;
    let broker_hello =
        ocentra_protected_capability_custody_protocol::decode_broker_hello(&broker_frame)?;
    let (_bootstrap_identity, authenticator) = bootstrap.into_parts();
    authenticate_broker_hello(&stream, &broker_path, &child, &client_hello, &broker_hello)?;
    let transcript_digest = broker_hello.verify_authenticated_provenance(
        &client_hello,
        unix_now_millis()?,
        &authenticator,
    )?;
    Ok(AuthenticatedBrokerSession {
        inner: WindowsBrokerSession {
            stream,
            child,
            broker_hello,
            transcript_digest,
            authenticator,
            _broker_executable: broker_path,
        },
    })
}

#[derive(Clone, Copy)]
struct ProcessIdentity {
    process_id: u32,
    process_epoch: u64,
    session_id: u32,
}

fn current_process_identity() -> Result<ProcessIdentity, ClientError> {
    process_identity(std::process::id())
}

fn process_identity(process_id: u32) -> Result<ProcessIdentity, ClientError> {
    if process_id == 0 {
        return Err(ClientError::PeerAuthentication);
    }
    let system = System::new_all();
    let process = system
        .process(Pid::from_u32(process_id))
        .ok_or(ClientError::PeerAuthentication)?;
    let process_epoch = process.start_time();
    let session_id = process
        .session_id()
        .map(Pid::as_u32)
        .ok_or(ClientError::PeerAuthentication)?;
    if process_epoch == 0 || session_id == 0 {
        return Err(ClientError::PeerAuthentication);
    }
    Ok(ProcessIdentity {
        process_id,
        process_epoch,
        session_id,
    })
}

fn write_bootstrap(child: &mut BrokerChild, encoded: &[u8]) -> Result<(), ClientError> {
    let mut input = child.take_stdin()?;
    ocentra_protected_capability_custody_protocol::write_frame(&mut input, encoded)?;
    drop(input);
    Ok(())
}

fn connect_pipe(
    pipe_name: &BrokerPipeName,
) -> Result<DuplexPipeStream<pipe_mode::Bytes>, ClientError> {
    let deadline = Instant::now()
        .checked_add(Duration::from_millis(CONNECT_DEADLINE_MILLIS))
        .ok_or(ClientError::Transport)?;
    loop {
        match DuplexPipeStream::<pipe_mode::Bytes>::connect_by_path(pipe_name.as_path()) {
            Ok(stream) => return Ok(stream),
            Err(_error) if Instant::now() < deadline => {
                thread::sleep(Duration::from_millis(CONNECT_RETRY_MILLIS));
            }
            Err(error) => return Err(map_transport_error(error)),
        }
    }
}

fn authenticate_pipe_server(
    stream: &DuplexPipeStream<pipe_mode::Bytes>,
    child: &BrokerChild,
    broker_path: &BrokerExecutableGuard,
    expected_session_id: u32,
) -> Result<(), ClientError> {
    let server_process_id = stream.server_process_id().map_err(map_transport_error)?;
    let server_session_id = stream.server_session_id().map_err(map_transport_error)?;
    let observed = process_identity(server_process_id)?;
    if server_process_id != child.id()
        || server_session_id != expected_session_id
        || observed.session_id != expected_session_id
        || !process_executable_matches(server_process_id, broker_path)?
    {
        return Err(ClientError::PeerAuthentication);
    }
    Ok(())
}

fn authenticate_broker_hello(
    stream: &DuplexPipeStream<pipe_mode::Bytes>,
    broker_path: &BrokerExecutableGuard,
    child: &BrokerChild,
    client_hello: &UntrustedClientHello,
    broker_hello: &UntrustedBrokerHello,
) -> Result<(), ClientError> {
    let server_process_id = stream.server_process_id().map_err(map_transport_error)?;
    let server_session_id = stream.server_session_id().map_err(map_transport_error)?;
    let observed = process_identity(server_process_id)?;
    if !broker_hello.matches_client(client_hello)
        || broker_hello.broker_process_id() != server_process_id
        || broker_hello.broker_process_id() != child.id()
        || broker_hello.broker_session_id() != server_session_id
        || broker_hello.broker_epoch() != observed.process_epoch
        || broker_hello.broker_session_id() != observed.session_id
        || !process_executable_matches(server_process_id, broker_path)?
    {
        return Err(ClientError::PeerAuthentication);
    }
    Ok(())
}

fn write_encoded(
    stream: &mut DuplexPipeStream<pipe_mode::Bytes>,
    frame: &[u8],
) -> Result<(), ClientError> {
    ocentra_protected_capability_custody_protocol::write_frame(stream, frame)
        .map_err(ClientError::from)
}

fn unix_now_millis() -> Result<u64, ClientError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(map_clock_error)?;
    u64::try_from(duration.as_millis()).map_err(map_clock_overflow)
}

fn map_clock_error(_error: std::time::SystemTimeError) -> ClientError {
    ClientError::PeerAuthentication
}

fn map_clock_overflow(_error: std::num::TryFromIntError) -> ClientError {
    ClientError::PeerAuthentication
}
