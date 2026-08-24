use ocentra_protected_capability_custody_protocol::constants::{
    INITIAL_SESSION_SEQUENCE, MAX_REQUEST_TTL_MILLIS,
};
use ocentra_protected_capability_custody_protocol::request::{
    RequestSessionEnvelope, UntrustedRequest, UntrustedRequestValues,
};
use zeroize::Zeroizing;

use super::{io, WindowsBrokerSession};
use crate::admission::{AuthenticatedResponse, ClientRequest};
use crate::ClientError;

impl WindowsBrokerSession {
    pub(crate) fn execute(
        mut self,
        request: ClientRequest,
    ) -> Result<AuthenticatedResponse, ClientError> {
        let now = io::unix_now_millis()?;
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
        io::write_frame(
            &mut self.stream,
            encoded_request.as_ref(),
            io::connection_deadline()?,
        )?;
        let response_frame = Zeroizing::new(io::read_frame(
            &mut self.stream,
            io::connection_deadline()?,
        )?);
        let response = ocentra_protected_capability_custody_protocol::decode_response(
            response_frame.as_ref(),
        )?;
        response.verify_authenticated_session(
            &wire_request,
            io::unix_now_millis()?,
            &self.authenticator,
        )?;
        Ok(AuthenticatedResponse::from_verified(response))
    }
}
