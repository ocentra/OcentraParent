use std::ffi::OsStr;

use ocentra_protected_capability_custody_protocol::bootstrap::BootstrapIdentity;
use ocentra_protected_capability_custody_protocol::constants::{
    BROKER_PIPE_NAME, CORRELATION_BYTES, NONCE_BYTES,
};
use ocentra_protected_capability_custody_protocol::handshake::UntrustedClientHello;
use ocentra_protected_capability_custody_protocol::request::ExpectedGenerations;
use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;
use ocentra_protected_capability_custody_protocol::types::{CorrelationId, Nonce, ProtocolError};

#[test]
fn client_selects_the_fixed_broker_pipe_without_a_caller_nonce() -> Result<(), ProtocolError> {
    let fixed = BrokerPipeName::fixed();
    let nonce = Nonce::try_from_bytes(&[0x55; NONCE_BYTES])?;
    let nonce_pipe = BrokerPipeName::from_nonce(nonce);

    assert_eq!(fixed.as_os_str(), OsStr::new(BROKER_PIPE_NAME));
    assert_ne!(fixed, nonce_pipe);
    Ok(())
}

#[test]
fn client_admission_rejects_zero_process_identity() -> Result<(), ProtocolError> {
    let nonce = Nonce::try_from_bytes(&[0x66; NONCE_BYTES])?;
    let correlation = CorrelationId::try_from_bytes(&[0x77; CORRELATION_BYTES])?;

    assert!(matches!(
        BootstrapIdentity::try_new(0, 7, 3, nonce),
        Err(ProtocolError::InvalidProcessId)
    ));
    assert!(matches!(
        UntrustedClientHello::try_new(nonce, correlation, 41, 0, 3),
        Err(ProtocolError::InvalidEpoch)
    ));
    assert!(matches!(
        UntrustedClientHello::try_new(nonce, correlation, 41, 7, 0),
        Err(ProtocolError::InvalidProcessId)
    ));
    Ok(())
}

#[test]
fn client_request_admission_rejects_zero_generation() {
    assert!(matches!(
        ExpectedGenerations::try_new(0, 2, 3, 4),
        Err(ProtocolError::InvalidEpoch)
    ));
    assert_eq!(ExpectedGenerations::initial_binding().authority(), 1);
    assert_eq!(ExpectedGenerations::initial_binding().target(), 1);
    assert_eq!(ExpectedGenerations::initial_binding().key(), 1);
    assert_eq!(ExpectedGenerations::initial_binding().writer(), 1);
}

#[cfg(not(windows))]
#[test]
fn client_connection_is_typed_unavailable_outside_windows() {
    assert!(matches!(
        ocentra_protected_capability_custody_client::connect(),
        Err(ocentra_protected_capability_custody_client::ClientError::UnsupportedPlatform)
    ));
}
