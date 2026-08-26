//! One authenticated fixed-pipe session and request transcript.

use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_protected_capability_custody_core::broker_admission::{
    BrokerRuntimeError, ClientAnchor,
};
use ocentra_protected_capability_custody_protocol::account_issuer::AccountIssuerRequest;
use ocentra_protected_capability_custody_protocol::account_issuer_session::AuthenticatedAccountIssuerRequest;
use ocentra_protected_capability_custody_protocol::bootstrap::BootstrapPacket;
use ocentra_protected_capability_custody_protocol::constants::INITIAL_SESSION_SEQUENCE;
use ocentra_protected_capability_custody_protocol::handshake::{
    UntrustedBrokerHello, UntrustedClientHello,
};
use ocentra_protected_capability_custody_protocol::request::authenticated::AuthenticatedRequest;
use ocentra_protected_capability_custody_protocol::request::{
    RequestSessionEnvelope, UntrustedRequest, UntrustedRequestValues,
};
use ocentra_protected_capability_custody_protocol::types::BootstrapAuthenticator;
use zeroize::Zeroizing;

use super::peer::{self as windows_ipc_peer, PipePeerIdentity, PipeStream};
use crate::admission::{AuthenticatedBrokerSession, ClientRequest};
use crate::ClientError;
use ocentra_protected_capability_custody_protocol::response::UntrustedResponse;

pub(crate) struct ClientSession {
    stream: PipeStream,
    anchor: ClientAnchor,
    peer: PipePeerIdentity,
    client_hello: UntrustedClientHello,
    broker_hello: UntrustedBrokerHello,
    authenticator: BootstrapAuthenticator,
}

pub(crate) fn establish(mut stream: PipeStream) -> Result<AuthenticatedBrokerSession, ClientError> {
    let peer = windows_ipc_peer::observe_server(&stream)?;
    let anchor =
        ClientAnchor::open(peer.process_id(), peer.session_id()).map_err(map_anchor_error)?;
    anchor.revalidate().map_err(map_anchor_error)?;
    let identity = anchor.client_identity().map_err(map_anchor_error)?;
    let bootstrap = BootstrapPacket::generate(
        identity.process_id(),
        identity.process_epoch(),
        identity.session_id(),
    )
    .map_err(ClientError::Protocol)?;
    let correlation =
        ocentra_protected_capability_custody_protocol::types::CorrelationId::generate()
            .map_err(ClientError::Protocol)?;
    let client_hello = UntrustedClientHello::try_new(
        bootstrap.identity().pipe_nonce(),
        correlation,
        identity.process_id(),
        identity.process_epoch(),
        identity.session_id(),
    )
    .map_err(ClientError::Protocol)?;
    let deadline = connection_deadline();
    let encoded_bootstrap = Zeroizing::new(
        ocentra_protected_capability_custody_protocol::encode_bootstrap(&bootstrap)
            .map_err(ClientError::Protocol)?,
    );
    super::io::write_frame(&mut stream, encoded_bootstrap.as_ref(), deadline)?;
    let encoded_client_hello = Zeroizing::new(
        ocentra_protected_capability_custody_protocol::encode_client_hello(&client_hello)
            .map_err(ClientError::Protocol)?,
    );
    super::io::write_frame(&mut stream, encoded_client_hello.as_ref(), deadline)?;
    windows_ipc_peer::reobserve_server(&stream, peer)?;
    let encoded_broker_hello = Zeroizing::new(super::io::read_frame(&mut stream, deadline)?);
    let broker_hello = ocentra_protected_capability_custody_protocol::decode_broker_hello(
        encoded_broker_hello.as_ref(),
    )
    .map_err(ClientError::Protocol)?;
    broker_hello
        .verify_authenticated_provenance(&client_hello, unix_now_millis()?)
        .map_err(ClientError::Protocol)?;
    anchor
        .authorize_broker_hello(&broker_hello, peer.process_id(), peer.session_id())
        .map_err(map_anchor_error)?;
    windows_ipc_peer::reobserve_server(&stream, peer)?;
    let authenticator = broker_hello.clone_authenticator();
    Ok(AuthenticatedBrokerSession::from_session(ClientSession {
        stream,
        anchor,
        peer,
        client_hello,
        broker_hello,
        authenticator,
    }))
}

impl ClientSession {
    pub(crate) fn execute_account_issuer(
        mut self,
        request: AccountIssuerRequest,
    ) -> Result<
        ocentra_protected_capability_custody_protocol::account_issuer::AccountIssuerReceipt,
        ClientError,
    > {
        self.anchor.revalidate().map_err(map_anchor_error)?;
        windows_ipc_peer::reobserve_server(&self.stream, self.peer)?;
        let now = unix_now_millis()?;
        self.broker_hello
            .verify_authenticated_provenance(&self.client_hello, now)
            .map_err(ClientError::Protocol)?;
        let expires_at = now
            .checked_add(
                ocentra_protected_capability_custody_protocol::constants::MAX_REQUEST_TTL_MILLIS,
            )
            .ok_or(ClientError::Protocol(
                ocentra_protected_capability_custody_protocol::types::ProtocolError::InvalidExpiry,
            ))?
            .min(self.broker_hello.session_expires_at_unix_millis());
        let authenticated = AuthenticatedAccountIssuerRequest::authenticate(
            &self.broker_hello,
            request,
            INITIAL_SESSION_SEQUENCE,
            expires_at,
            &self.authenticator,
        )
        .map_err(ClientError::Protocol)?;
        let frame = Zeroizing::new(
            ocentra_protected_capability_custody_protocol::account_issuer_session::encode_request(
                &authenticated,
            )
            .map_err(ClientError::Protocol)?,
        );
        super::io::write_frame(&mut self.stream, frame.as_ref(), connection_deadline())?;
        let response_frame = Zeroizing::new(super::io::read_frame(
            &mut self.stream,
            connection_deadline(),
        )?);
        let response =
            ocentra_protected_capability_custody_protocol::account_issuer_session::decode_receipt(
                response_frame.as_ref(),
            )
            .map_err(ClientError::Protocol)?;
        let receipt = response
            .into_verified_receipt(
                &authenticated,
                &self.broker_hello,
                unix_now_millis()?,
                &self.authenticator,
            )
            .map_err(ClientError::Protocol)?;
        self.anchor.revalidate().map_err(map_anchor_error)?;
        Ok(receipt)
    }

    pub(crate) fn execute(
        mut self,
        request: ClientRequest,
    ) -> Result<UntrustedResponse, ClientError> {
        self.anchor.revalidate().map_err(map_anchor_error)?;
        windows_ipc_peer::reobserve_server(&self.stream, self.peer)?;
        let now = unix_now_millis()?;
        self.broker_hello
            .verify_authenticated_provenance(&self.client_hello, now)
            .map_err(ClientError::Protocol)?;
        let expires_at = now
            .checked_add(
                ocentra_protected_capability_custody_protocol::constants::MAX_REQUEST_TTL_MILLIS,
            )
            .ok_or(ClientError::Protocol(
                ocentra_protected_capability_custody_protocol::types::ProtocolError::InvalidExpiry,
            ))?
            .min(self.broker_hello.session_expires_at_unix_millis());
        let session = RequestSessionEnvelope::from_authenticated_hello(
            &self.broker_hello,
            self.broker_hello.transcript_digest(),
            INITIAL_SESSION_SEQUENCE,
            expires_at,
        )
        .map_err(ClientError::Protocol)?;
        let values = UntrustedRequestValues {
            session,
            expected_generations: request.expected_generations,
            kind: request.kind,
            operation: request.operation,
            action: request.action,
            target: request.target,
            opaque_token: request.opaque_token,
        };
        let authenticated = UntrustedRequest::authenticate_wire(values, &self.authenticator)
            .map_err(ClientError::Protocol)?;
        self.write_request(&authenticated, connection_deadline())?;
        let response_frame = Zeroizing::new(super::io::read_frame(
            &mut self.stream,
            connection_deadline(),
        )?);
        let response =
            ocentra_protected_capability_custody_protocol::decode_response(response_frame.as_ref())
                .map_err(ClientError::Protocol)?;
        response
            .verify_authenticated_session(&authenticated, unix_now_millis()?, &self.authenticator)
            .map_err(ClientError::Protocol)?;
        self.anchor.revalidate().map_err(map_anchor_error)?;
        Ok(response)
    }

    fn write_request(
        &mut self,
        request: &AuthenticatedRequest,
        deadline: std::time::Instant,
    ) -> Result<(), ClientError> {
        let frame = Zeroizing::new(
            ocentra_protected_capability_custody_protocol::encode_request(request)
                .map_err(ClientError::Protocol)?,
        );
        super::io::write_frame(&mut self.stream, frame.as_ref(), deadline)
    }
}

fn connection_deadline() -> std::time::Instant {
    std::time::Instant::now()
        + std::time::Duration::from_millis(
            ocentra_protected_capability_custody_protocol::constants::BROKER_ACCEPT_DEADLINE_MILLIS,
        )
}

fn unix_now_millis() -> Result<u64, ClientError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_error| ClientError::PeerAuthentication)?;
    u64::try_from(duration.as_millis()).map_err(|_error| ClientError::PeerAuthentication)
}

fn map_anchor_error(error: BrokerRuntimeError) -> ClientError {
    match error {
        BrokerRuntimeError::DeploymentRequired | BrokerRuntimeError::Unavailable => {
            ClientError::DeploymentRequired
        }
        BrokerRuntimeError::InvalidBrokerProcess
        | BrokerRuntimeError::InvalidRequest
        | BrokerRuntimeError::Binding(_)
        | BrokerRuntimeError::Custody(_) => ClientError::PeerAuthentication,
    }
}
