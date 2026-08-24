use std::io;
use std::time::Instant;

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream};
use ocentra_protected_capability_custody_core::broker_admission::{
    BrokerCustodyRuntime, BrokerRuntimeError,
};
use ocentra_protected_capability_custody_protocol::bootstrap::BootstrapPacket;
use ocentra_protected_capability_custody_protocol::constants::INITIAL_SESSION_SEQUENCE;
use ocentra_protected_capability_custody_protocol::handshake::{
    UntrustedBrokerHello, UntrustedClientHello,
};
use zeroize::Zeroizing;

use crate::authority::{
    current_process_identity, peer_process_observation, unix_now_millis, BrokerSessionAuthority,
};
use crate::custody::BrokerCustodyService;
use crate::BrokerError;

type PipeStream = DuplexPipeStream<pipe_mode::Bytes>;

pub(super) fn serve(
    stream: &mut PipeStream,
    deadline: Instant,
    custody: &BrokerCustodyService,
) -> Result<(), BrokerError> {
    // Admit the OS peer before reading any caller-controlled bootstrap or
    // generating a broker session key. The current build fails closed here:
    // the required pinned process/token adapter is not available under the
    // safe dependency boundary. Once supplied, it must keep the process
    // handle alive through digest/admission and capture the token under this
    // RAII impersonation scope before protected registry custody is opened.
    let peer_process_id = stream.client_process_id().map_err(map_transport_error)?;
    let observation = peer_process_observation(peer_process_id)?;
    let token = {
        let _impersonation = stream.impersonate_client().map_err(map_transport_error)?;
        BrokerCustodyRuntime::observe_impersonated_client().map_err(map_runtime_error)?
    };
    custody.authorize_client_peer(&observation, token)?;

    let bootstrap_frame = Zeroizing::new(super::io::read_frame(stream, deadline)?);
    let bootstrap =
        ocentra_protected_capability_custody_protocol::decode_bootstrap(bootstrap_frame.as_ref())?;
    authenticate_pipe_client(stream, &bootstrap, &observation)?;

    let client_frame = Zeroizing::new(super::io::read_frame(stream, deadline)?);
    let client_hello =
        ocentra_protected_capability_custody_protocol::decode_client_hello(client_frame.as_ref())?;
    authenticate_client_hello(stream, &bootstrap, &client_hello)?;

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
    )?;
    let encoded_hello = Zeroizing::new(
        ocentra_protected_capability_custody_protocol::encode_broker_hello(&broker_hello)?,
    );
    super::io::write_frame(stream, encoded_hello.as_ref(), deadline)?;

    let request_frame = Zeroizing::new(super::io::read_frame(stream, deadline)?);
    let request =
        ocentra_protected_capability_custody_protocol::decode_request(request_frame.as_ref())?;
    let request = request.into_authenticated_session(
        &broker_hello,
        unix_now_millis()?,
        INITIAL_SESSION_SEQUENCE,
        broker_hello.authenticator(),
    )?;
    let response = custody.execute(&request, broker_hello.authenticator())?;
    let encoded_response =
        Zeroizing::new(ocentra_protected_capability_custody_protocol::encode_response(&response)?);
    super::io::write_frame(stream, encoded_response.as_ref(), deadline)
}

fn authenticate_pipe_client(
    stream: &PipeStream,
    bootstrap: &BootstrapPacket,
    observation: &crate::authority::PeerProcessObservation,
) -> Result<(), BrokerError> {
    let identity = bootstrap.identity();
    let peer_process_id = stream.client_process_id().map_err(map_transport_error)?;
    let peer_session_id = stream.client_session_id().map_err(map_transport_error)?;
    if peer_process_id != identity.client_process_id()
        || peer_session_id != identity.client_session_id()
        || observation.identity.process_id != identity.client_process_id()
        || observation.identity.process_epoch != identity.client_process_epoch()
        || observation.identity.session_id != identity.client_session_id()
    {
        return Err(BrokerError::PeerAuthentication);
    }
    Ok(())
}

fn authenticate_client_hello(
    stream: &PipeStream,
    bootstrap: &BootstrapPacket,
    hello: &UntrustedClientHello,
) -> Result<(), BrokerError> {
    let identity = bootstrap.identity();
    if hello.client_process_id() != stream.client_process_id().map_err(map_transport_error)?
        || hello.client_session_id() != stream.client_session_id().map_err(map_transport_error)?
        || hello.client_process_id() != identity.client_process_id()
        || hello.client_process_epoch() != identity.client_process_epoch()
        || hello.client_session_id() != identity.client_session_id()
        || hello.nonce() != identity.pipe_nonce()
    {
        return Err(BrokerError::PeerAuthentication);
    }
    Ok(())
}

fn map_transport_error(_error: io::Error) -> BrokerError {
    BrokerError::Transport
}

fn map_runtime_error(error: BrokerRuntimeError) -> BrokerError {
    match error {
        BrokerRuntimeError::DeploymentRequired => BrokerError::DeploymentRequired,
        _ => BrokerError::PeerAuthentication,
    }
}
