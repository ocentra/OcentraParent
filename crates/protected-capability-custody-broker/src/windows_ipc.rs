use std::io;

use interprocess::os::windows::named_pipe::{pipe_mode, PipeListenerOptions};
use interprocess::os::windows::security_descriptor::SecurityDescriptor;
use ocentra_protected_capability_custody_protocol::bootstrap::BootstrapPacket;
use ocentra_protected_capability_custody_protocol::constants::{
    BROKER_PIPE_SDDL, INITIAL_SESSION_SEQUENCE,
};
use ocentra_protected_capability_custody_protocol::handshake::{
    UntrustedBrokerHello, UntrustedClientHello,
};
use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;
use ocentra_protected_capability_custody_protocol::types::BootstrapAuthenticator;
use widestring::U16CString;
use zeroize::Zeroizing;

use crate::authority::{
    current_process_identity, process_identity, unix_now_millis, BrokerSessionAuthority,
};
use crate::{custody, map_transport_error, BrokerError};

pub(crate) fn run(pipe_name: &BrokerPipeName) -> Result<(), BrokerError> {
    let bootstrap = read_bootstrap()?;
    if &BrokerPipeName::from_nonce(bootstrap.identity().pipe_nonce()) != pipe_name {
        return Err(BrokerError::InvalidLaunch);
    }
    let listener = create_listener(pipe_name)?;
    let mut stream = listener.accept().map_err(map_transport_error)?;
    authenticate_pipe_client(&stream, &bootstrap)?;
    let client_frame = ocentra_protected_capability_custody_protocol::read_frame(&mut stream)?;
    let client_hello =
        ocentra_protected_capability_custody_protocol::decode_client_hello(&client_frame)?;
    authenticate_client_hello(&stream, &bootstrap, &client_hello)?;
    let custody = custody::BrokerCustodyService::open();
    let broker_identity = current_process_identity()?;
    let session = BrokerSessionAuthority::generate(
        broker_identity,
        unix_now_millis()?,
        custody.platform_session_state(),
    )?;
    let broker_hello = UntrustedBrokerHello::authenticate_wire(
        &client_hello,
        session.wire_values(),
        unix_now_millis()?,
        bootstrap.authenticator(),
    )?;
    write_encoded(
        &mut stream,
        &ocentra_protected_capability_custody_protocol::encode_broker_hello(&broker_hello)?,
    )?;
    serve_one_authenticated_request(
        &mut stream,
        &broker_hello,
        bootstrap.authenticator(),
        &custody,
    )
}

fn read_bootstrap() -> Result<BootstrapPacket, BrokerError> {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let frame = Zeroizing::new(ocentra_protected_capability_custody_protocol::read_frame(
        &mut input,
    )?);
    ocentra_protected_capability_custody_protocol::decode_bootstrap(frame.as_ref())
        .map_err(BrokerError::from)
}

fn create_listener(
    pipe_name: &BrokerPipeName,
) -> Result<
    interprocess::os::windows::named_pipe::PipeListener<pipe_mode::Bytes, pipe_mode::Bytes>,
    BrokerError,
> {
    let sddl = U16CString::from_str(BROKER_PIPE_SDDL).map_err(map_sddl_error)?;
    let descriptor = SecurityDescriptor::deserialize(&sddl).map_err(map_transport_error)?;
    PipeListenerOptions::new()
        .path(pipe_name.as_path())
        .accept_remote(false)
        .inheritable(false)
        .security_descriptor(Some(descriptor))
        .create_duplex::<pipe_mode::Bytes>()
        .map_err(map_transport_error)
}

fn authenticate_pipe_client(
    stream: &interprocess::os::windows::named_pipe::DuplexPipeStream<pipe_mode::Bytes>,
    bootstrap: &BootstrapPacket,
) -> Result<(), BrokerError> {
    let identity = bootstrap.identity();
    let peer_process_id = stream.client_process_id().map_err(map_transport_error)?;
    let peer_session_id = stream.client_session_id().map_err(map_transport_error)?;
    let observed = process_identity(peer_process_id)?;
    if peer_process_id != identity.client_process_id()
        || peer_session_id != identity.client_session_id()
        || observed.process_epoch != identity.client_process_epoch()
        || observed.session_id != identity.client_session_id()
    {
        return Err(BrokerError::PeerAuthentication);
    }
    Ok(())
}

fn authenticate_client_hello(
    stream: &interprocess::os::windows::named_pipe::DuplexPipeStream<pipe_mode::Bytes>,
    bootstrap: &BootstrapPacket,
    hello: &UntrustedClientHello,
) -> Result<(), BrokerError> {
    let identity = bootstrap.identity();
    if hello.client_process_id() != stream.client_process_id().map_err(map_transport_error)?
        || hello.client_session_id() != stream.client_session_id().map_err(map_transport_error)?
        || hello.client_process_id() != identity.client_process_id()
        || hello.client_process_epoch() != identity.client_process_epoch()
        || hello.client_session_id() != identity.client_session_id()
    {
        return Err(BrokerError::PeerAuthentication);
    }
    Ok(())
}

fn serve_one_authenticated_request(
    stream: &mut interprocess::os::windows::named_pipe::DuplexPipeStream<pipe_mode::Bytes>,
    broker_hello: &UntrustedBrokerHello,
    authenticator: &BootstrapAuthenticator,
    custody: &custody::BrokerCustodyService,
) -> Result<(), BrokerError> {
    let request_frame = Zeroizing::new(ocentra_protected_capability_custody_protocol::read_frame(
        stream,
    )?);
    let request =
        ocentra_protected_capability_custody_protocol::decode_request(request_frame.as_ref())?;
    let request = request.into_authenticated_session(
        broker_hello,
        unix_now_millis()?,
        INITIAL_SESSION_SEQUENCE,
        authenticator,
    )?;
    let response = custody.execute(&request, authenticator)?;
    let encoded_response =
        Zeroizing::new(ocentra_protected_capability_custody_protocol::encode_response(&response)?);
    write_encoded(stream, encoded_response.as_ref())
}

fn write_encoded(
    stream: &mut interprocess::os::windows::named_pipe::DuplexPipeStream<pipe_mode::Bytes>,
    frame: &[u8],
) -> Result<(), BrokerError> {
    ocentra_protected_capability_custody_protocol::write_frame(stream, frame)
        .map_err(BrokerError::from)
}

fn map_sddl_error(_error: widestring::error::ContainsNul<u16>) -> BrokerError {
    BrokerError::InvalidLaunch
}
