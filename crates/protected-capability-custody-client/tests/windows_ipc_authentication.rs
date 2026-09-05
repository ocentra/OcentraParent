use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_protected_capability_custody_protocol::constants::{
    CORRELATION_BYTES, NONCE_BYTES, SESSION_HANDLE_BYTES, SESSION_TTL_MILLIS,
};
use ocentra_protected_capability_custody_protocol::handshake::{
    BrokerSessionWireValues, UntrustedBrokerHello, UntrustedClientHello,
};
use ocentra_protected_capability_custody_protocol::request::RequestSessionEnvelope;
use ocentra_protected_capability_custody_protocol::types::{
    CorrelationId, Nonce, ProtocolError, SessionHandle,
};

fn client_hello() -> Result<UntrustedClientHello, ProtocolError> {
    UntrustedClientHello::try_new(
        Nonce::try_from_bytes(&[0x11; NONCE_BYTES])?,
        CorrelationId::try_from_bytes(&[0x22; CORRELATION_BYTES])?,
        41,
        7,
        3,
    )
}

fn broker_session(now_unix_millis: u64) -> Result<BrokerSessionWireValues, ProtocolError> {
    Ok(BrokerSessionWireValues {
        broker_nonce: Nonce::try_from_bytes(&[0x33; NONCE_BYTES])?,
        broker_process_id: 99,
        broker_session_id: 0,
        broker_epoch: 10,
        broker_key_epoch: 11,
        writer_lease_epoch: 12,
        watermark: 13,
        session_handle: SessionHandle::try_from_untrusted_bytes(&[0x44; SESSION_HANDLE_BYTES])?,
        session_expires_at_unix_millis: now_unix_millis + SESSION_TTL_MILLIS,
    })
}

fn broker_hello(now_unix_millis: u64) -> Result<UntrustedBrokerHello, ProtocolError> {
    let client = client_hello()?;
    UntrustedBrokerHello::authenticate_wire(
        &client,
        broker_session(now_unix_millis)?,
        now_unix_millis,
    )
}

fn current_unix_millis() -> Result<u64, ProtocolError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_error| ProtocolError::InvalidExpiry)?;
    u64::try_from(duration.as_millis()).map_err(|_error| ProtocolError::InvalidExpiry)
}

#[test]
fn broker_provenance_rejects_process_or_session_drift() -> Result<(), ProtocolError> {
    let now = 1_000_000;
    let client = client_hello()?;
    let hello = broker_hello(now)?;

    let process_drift = UntrustedClientHello::try_new(
        client.nonce(),
        client.correlation(),
        client.client_process_id() + 1,
        client.client_process_epoch(),
        client.client_session_id(),
    )?;
    assert!(matches!(
        hello.verify_authenticated_provenance(&process_drift, now + 1),
        Err(ProtocolError::AuthenticationFailed)
    ));

    let session_drift = UntrustedClientHello::try_new(
        client.nonce(),
        client.correlation(),
        client.client_process_id(),
        client.client_process_epoch(),
        client.client_session_id() + 1,
    )?;
    assert!(matches!(
        hello.verify_authenticated_provenance(&session_drift, now + 1),
        Err(ProtocolError::AuthenticationFailed)
    ));
    Ok(())
}

#[test]
fn request_session_cannot_outlive_authenticated_broker_session() -> Result<(), ProtocolError> {
    let now = 1_000_000;
    let hello = broker_hello(now)?;
    let transcript = hello.transcript_digest();

    assert!(matches!(
        RequestSessionEnvelope::from_authenticated_hello(&hello, transcript, 0, now + 1),
        Err(ProtocolError::InvalidSequence)
    ));
    assert!(matches!(
        RequestSessionEnvelope::from_authenticated_hello(
            &hello,
            transcript,
            1,
            hello.session_expires_at_unix_millis() + 1,
        ),
        Err(ProtocolError::InvalidExpiry)
    ));
    assert!(matches!(
        RequestSessionEnvelope::from_authenticated_hello(&hello, transcript, 1, 0),
        Err(ProtocolError::InvalidExpiry)
    ));
    Ok(())
}

#[test]
fn authenticated_broker_hello_codec_preserves_the_os_identity_fields() -> Result<(), ProtocolError>
{
    let now = current_unix_millis()?;
    let client = client_hello()?;
    let hello = UntrustedBrokerHello::authenticate_wire(&client, broker_session(now)?, now)?;
    let frame = ocentra_protected_capability_custody_protocol::encode_broker_hello(&hello)?;
    let decoded = ocentra_protected_capability_custody_protocol::decode_broker_hello(&frame)?;

    assert_eq!(decoded.broker_process_id(), hello.broker_process_id());
    assert_eq!(decoded.broker_session_id(), hello.broker_session_id());
    assert_eq!(decoded.broker_epoch(), hello.broker_epoch());
    decoded.verify_authenticated_provenance(&client, now + 1)?;
    Ok(())
}
