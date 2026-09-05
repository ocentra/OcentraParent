#![cfg(test)]

use std::time::{Duration, Instant};

use ocentra_protected_capability_custody_protocol::account_issuer_contract::{
    ACCOUNT_ISSUER_PROTOCOL_VERSION, ACCOUNT_ISSUER_SERVICE, ACCOUNT_ISSUER_TRANSPORT_DOMAIN,
};
use ocentra_protected_capability_custody_protocol::account_issuer_session::AuthenticatedAccountIssuerRequest;
use ocentra_protected_capability_custody_protocol::constants::{
    BROKER_ACCEPT_DEADLINE_MILLIS, CORRELATION_BYTES, NONCE_BYTES, SESSION_HANDLE_BYTES,
};
use ocentra_protected_capability_custody_protocol::decode_account_issuer_request;
use ocentra_protected_capability_custody_protocol::handshake::{
    BrokerSessionWireValues, UntrustedBrokerHello, UntrustedClientHello,
};
use ocentra_protected_capability_custody_protocol::types::{
    CorrelationId, Nonce, ProtocolError, SessionHandle,
};

use super::account_issuer_request::AccountIssuerRequestBinding;
use super::BrokerRuntimeError;

const NOW_MILLIS: u64 = 1_000_000;
const REQUEST_TAG: u8 = 1;
const ISSUE_CURRENT_AUTHORITY_KIND: u8 = 6;
const AUTHJS_PROVIDER: u8 = 1;

fn append_field(wire: &mut Vec<u8>, value: &str) -> Result<(), ProtocolError> {
    let length = u32::try_from(value.len()).map_err(|_error| ProtocolError::FieldTooLarge)?;
    wire.extend_from_slice(&length.to_be_bytes());
    wire.extend_from_slice(value.as_bytes());
    Ok(())
}

fn issue_request_wire(
    correlation_id: &str,
    idempotency_key: &str,
) -> Result<Vec<u8>, ProtocolError> {
    let mut wire = ACCOUNT_ISSUER_TRANSPORT_DOMAIN.to_vec();
    wire.extend_from_slice(&ACCOUNT_ISSUER_PROTOCOL_VERSION.to_be_bytes());
    wire.push(REQUEST_TAG);
    wire.push(ISSUE_CURRENT_AUTHORITY_KIND);
    append_field(&mut wire, ACCOUNT_ISSUER_SERVICE)?;
    append_field(&mut wire, correlation_id)?;
    append_field(&mut wire, idempotency_key)?;
    append_field(&mut wire, &format!("sha256:ecdsa-p256:{}", "0".repeat(64)))?;
    wire.push(AUTHJS_PROVIDER);
    append_field(&mut wire, "authjs-subject-1")?;
    Ok(wire)
}

fn authenticated_request(
    correlation_id: &str,
    idempotency_key: &str,
) -> Result<AuthenticatedAccountIssuerRequest, ProtocolError> {
    let client = UntrustedClientHello::try_new(
        Nonce::try_from_bytes(&[0x11; NONCE_BYTES])?,
        CorrelationId::try_from_bytes(&[0x22; CORRELATION_BYTES])?,
        41,
        7,
        3,
    )?;
    let hello = UntrustedBrokerHello::authenticate_wire(
        &client,
        BrokerSessionWireValues {
            broker_nonce: Nonce::try_from_bytes(&[0x33; NONCE_BYTES])?,
            broker_process_id: 99,
            broker_session_id: 2,
            broker_epoch: 10,
            broker_key_epoch: 11,
            writer_lease_epoch: 12,
            watermark: 13,
            session_handle: SessionHandle::try_from_untrusted_bytes(&[0x44; SESSION_HANDLE_BYTES])?,
            session_expires_at_unix_millis: NOW_MILLIS + 4_000,
        },
        NOW_MILLIS,
    )?;
    let request =
        decode_account_issuer_request(&issue_request_wire(correlation_id, idempotency_key)?)?;
    let authenticator = hello.clone_authenticator();
    AuthenticatedAccountIssuerRequest::authenticate(
        &hello,
        request,
        1,
        NOW_MILLIS + 1_000,
        &authenticator,
    )
}

#[test]
fn account_issuer_request_binding_rejects_correlation_and_idempotency_substitution(
) -> Result<(), ProtocolError> {
    let authorized = authenticated_request("correlation-1", "idempotency-1")?;
    let authorized_at = Instant::now();
    let binding = AccountIssuerRequestBinding::new(&authorized, authorized_at);

    assert!(binding
        .verify_at(&authorized, authorized_at + Duration::from_millis(1))
        .is_ok());

    for substituted in [
        authenticated_request("correlation-2", "idempotency-1")?,
        authenticated_request("correlation-1", "idempotency-2")?,
    ] {
        assert!(matches!(
            binding.verify_at(&substituted, authorized_at + Duration::from_millis(1)),
            Err(BrokerRuntimeError::InvalidRequest)
        ));
    }
    Ok(())
}

#[test]
fn account_issuer_request_binding_expires_with_the_broker_admission_window(
) -> Result<(), ProtocolError> {
    let request = authenticated_request("correlation-1", "idempotency-1")?;
    let authorized_at = Instant::now();
    let binding = AccountIssuerRequestBinding::new(&request, authorized_at);
    let expired_at =
        authorized_at + Duration::from_millis(BROKER_ACCEPT_DEADLINE_MILLIS.saturating_add(1));

    assert!(matches!(
        binding.verify_at(&request, expired_at),
        Err(BrokerRuntimeError::InvalidRequest)
    ));
    assert!(matches!(
        binding.verify_at(
            &request,
            authorized_at
                .checked_sub(Duration::from_millis(1))
                .ok_or(ProtocolError::InvalidExpiry)?,
        ),
        Err(BrokerRuntimeError::InvalidRequest)
    ));
    Ok(())
}
