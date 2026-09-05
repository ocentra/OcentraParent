use std::ffi::OsStr;

use ocentra_protected_capability_custody_protocol::constants::{
    BROKER_PIPE_NAME, CORRELATION_BYTES, NONCE_BYTES, SESSION_HANDLE_BYTES, SESSION_TTL_MILLIS,
};
use ocentra_protected_capability_custody_protocol::handshake::{
    BrokerSessionWireValues, UntrustedClientHello,
};
use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;
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

#[test]
fn broker_uses_the_single_fixed_pipe_endpoint() {
    let fixed = BrokerPipeName::fixed();
    assert_eq!(fixed.as_os_str(), OsStr::new(BROKER_PIPE_NAME));
}

#[test]
fn broker_session_state_rejects_missing_currentness_epochs() -> Result<(), ProtocolError> {
    let now = 1_000_000;
    let mut session = broker_session(now)?;
    session.broker_key_epoch = 0;

    assert!(matches!(
        session.try_new(now),
        Err(ProtocolError::InvalidEpoch)
    ));
    Ok(())
}

#[test]
fn broker_session_expiry_must_be_current_and_bounded() -> Result<(), ProtocolError> {
    let now = 1_000_000;
    let session = broker_session(now)?;
    let _current_session = session.try_new(now)?;

    let mut expired = session;
    expired.session_expires_at_unix_millis = now;
    assert!(matches!(
        expired.try_new(now),
        Err(ProtocolError::InvalidExpiry)
    ));

    let mut over_bound = session;
    over_bound.session_expires_at_unix_millis = now + SESSION_TTL_MILLIS + 1;
    assert!(matches!(
        over_bound.try_new(now),
        Err(ProtocolError::InvalidExpiry)
    ));
    Ok(())
}

#[test]
fn broker_hello_binds_client_identity_and_session_lifetime() -> Result<(), ProtocolError> {
    let now = 1_000_000;
    let client = client_hello()?;
    let hello = ocentra_protected_capability_custody_protocol::handshake::
        UntrustedBrokerHello::authenticate_wire(&client, broker_session(now)?, now)?;

    hello.verify_authenticated_provenance(&client, now + 1)?;

    let drifted_client = UntrustedClientHello::try_new(
        client.nonce(),
        client.correlation(),
        client.client_process_id() + 1,
        client.client_process_epoch(),
        client.client_session_id(),
    )?;
    assert!(matches!(
        hello.verify_authenticated_provenance(&drifted_client, now + 1),
        Err(ProtocolError::AuthenticationFailed)
    ));
    assert!(matches!(
        hello.verify_authenticated_provenance(&client, now + SESSION_TTL_MILLIS),
        Err(ProtocolError::AuthenticationFailed)
    ));
    Ok(())
}

#[cfg(not(windows))]
#[test]
fn broker_service_is_typed_unavailable_outside_windows() {
    assert!(matches!(
        ocentra_protected_capability_custody_broker::run_service(),
        Err(ocentra_protected_capability_custody_broker::BrokerError::UnsupportedPlatform)
    ));
}
