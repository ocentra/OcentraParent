use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_protected_capability_custody_protocol::bootstrap::BootstrapPacket;
use ocentra_protected_capability_custody_protocol::constants::{
    CORRELATION_BYTES, MAX_FRAME_BYTES, NONCE_BYTES, SESSION_HANDLE_BYTES,
};
use ocentra_protected_capability_custody_protocol::handshake::{
    BrokerSessionWireValues, UntrustedBrokerHello, UntrustedClientHello,
};
use ocentra_protected_capability_custody_protocol::request::{
    ExpectedGenerations, RequestKind, RequestSessionEnvelope, UntrustedRequest,
    UntrustedRequestValues,
};
use ocentra_protected_capability_custody_protocol::response::{
    ObservedGenerations, ResponseStatus, UntrustedResponse,
};
use ocentra_protected_capability_custody_protocol::target::{Action, TargetDescriptor, TargetKind};
use ocentra_protected_capability_custody_protocol::types::{
    CorrelationId, Nonce, OpaquePreparedToken, ProtocolError, SessionHandle,
};
use ocentra_protected_capability_custody_protocol::{
    decode_bootstrap, decode_broker_hello, decode_client_hello, decode_request, decode_response,
    encode_bootstrap, encode_broker_hello, encode_client_hello, encode_request, encode_response,
};

fn now_unix_millis() -> Result<u64, ProtocolError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_error| ProtocolError::InvalidExpiry)?;
    u64::try_from(duration.as_millis()).map_err(|_error| ProtocolError::InvalidExpiry)
}

fn client_hello() -> Result<UntrustedClientHello, ProtocolError> {
    UntrustedClientHello::try_new(
        Nonce::try_from_bytes(&[0x11; NONCE_BYTES])?,
        CorrelationId::try_from_bytes(&[0x22; CORRELATION_BYTES])?,
        41,
        7,
        3,
    )
}

fn broker_hello(now: u64) -> Result<UntrustedBrokerHello, ProtocolError> {
    let client = client_hello()?;
    let expires = now.checked_add(4_000).ok_or(ProtocolError::InvalidExpiry)?;
    UntrustedBrokerHello::authenticate_wire(
        &client,
        BrokerSessionWireValues {
            broker_nonce: Nonce::try_from_bytes(&[0x33; NONCE_BYTES])?,
            broker_process_id: 99,
            broker_session_id: 0,
            broker_epoch: 10,
            broker_key_epoch: 11,
            writer_lease_epoch: 12,
            watermark: 13,
            session_handle: SessionHandle::try_from_untrusted_bytes(&[0x44; SESSION_HANDLE_BYTES])?,
            session_expires_at_unix_millis: expires,
        },
        now,
    )
}

fn request_values(
    hello: &UntrustedBrokerHello,
    now: u64,
    kind: RequestKind,
    opaque_token: Option<OpaquePreparedToken>,
) -> Result<UntrustedRequestValues, ProtocolError> {
    let expires = now.checked_add(1_000).ok_or(ProtocolError::InvalidExpiry)?;
    Ok(UntrustedRequestValues {
        session: RequestSessionEnvelope::from_authenticated_hello(
            hello,
            hello.transcript_digest(),
            1,
            expires,
        )?,
        expected_generations: ExpectedGenerations::try_new(2, 3, 4, 5)?,
        kind,
        operation: vec![0x55, 0x66],
        action: Action::Seal,
        target: TargetDescriptor::try_new(TargetKind::Capability, vec![1], vec![2], vec![3])?,
        opaque_token,
    })
}

fn with_trailing_payload_byte(mut frame: Vec<u8>) -> Vec<u8> {
    let declared = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]);
    frame[0..4].copy_from_slice(&declared.saturating_add(1).to_be_bytes());
    frame.push(0);
    frame
}

#[test]
fn bootstrap_round_trips_with_identity_intact() -> Result<(), ProtocolError> {
    let bootstrap = BootstrapPacket::generate(41, 7, 3)?;
    let bootstrap_frame = encode_bootstrap(&bootstrap)?;
    assert_eq!(
        decode_bootstrap(&bootstrap_frame)?.identity(),
        bootstrap.identity()
    );
    Ok(())
}

#[test]
fn handshake_round_trips_and_rejects_client_identity_drift() -> Result<(), ProtocolError> {
    let now = now_unix_millis()?;
    let client = client_hello()?;
    let client_frame = encode_client_hello(&client)?;
    let decoded_client = decode_client_hello(&client_frame)?;
    assert_eq!(decoded_client, client);

    let hello = broker_hello(now)?;
    let hello_frame = encode_broker_hello(&hello)?;
    let decoded_hello = decode_broker_hello(&hello_frame)?;
    assert!(decoded_hello.matches_client(&decoded_client));
    assert_eq!(
        decoded_hello.verify_authenticated_provenance(&decoded_client, now + 1)?,
        hello.transcript_digest()
    );
    let drifted_client = UntrustedClientHello::try_new(
        decoded_client.nonce(),
        decoded_client.correlation(),
        decoded_client.client_process_id() + 1,
        decoded_client.client_process_epoch(),
        decoded_client.client_session_id(),
    )?;
    assert!(matches!(
        decoded_hello.verify_authenticated_provenance(&drifted_client, now + 1),
        Err(ProtocolError::AuthenticationFailed)
    ));
    Ok(())
}

#[test]
fn request_round_trip_requires_the_expected_sequence() -> Result<(), ProtocolError> {
    let now = now_unix_millis()?;
    let hello = broker_hello(now)?;
    let decoded_hello = decode_broker_hello(&encode_broker_hello(&hello)?)?;
    let authenticator = hello.clone_authenticator();
    let request = UntrustedRequest::authenticate_wire(
        request_values(&hello, now, RequestKind::Prepare, None)?,
        &authenticator,
    )?;
    let request_frame = encode_request(&request)?;
    let wrong_sequence_request = decode_request(&request_frame)?;
    assert!(matches!(
        wrong_sequence_request.into_authenticated_session(
            &decoded_hello,
            now + 2,
            2,
            &authenticator,
        ),
        Err(ProtocolError::InvalidSequence)
    ));
    let decoded_request = decode_request(&request_frame)?;
    let authenticated_decoded =
        decoded_request.into_authenticated_session(&decoded_hello, now + 2, 1, &authenticator)?;
    assert_eq!(
        authenticated_decoded.as_untrusted().operation(),
        &[0x55, 0x66]
    );
    assert_eq!(
        authenticated_decoded.as_untrusted().kind(),
        RequestKind::Prepare
    );
    Ok(())
}

#[test]
fn response_round_trip_authenticates_observed_generations() -> Result<(), ProtocolError> {
    let now = now_unix_millis()?;
    let hello = broker_hello(now)?;
    let authenticator = hello.clone_authenticator();
    let request = UntrustedRequest::authenticate_wire(
        request_values(&hello, now, RequestKind::Prepare, None)?,
        &authenticator,
    )?;
    let authenticated_decoded = decode_request(&encode_request(&request)?)?
        .into_authenticated_session(&hello, now + 2, 1, &authenticator)?;
    let token = OpaquePreparedToken::from_untrusted_wire_bytes(vec![0x77; 96])?;
    let response = UntrustedResponse::authenticate_wire(
        &authenticated_decoded,
        ResponseStatus::Prepared,
        Some(ObservedGenerations::try_new(6, 7, 8, 9)?),
        Some(token),
        &authenticator,
    )?;
    let response_frame = encode_response(&response)?;
    let decoded_response = decode_response(&response_frame)?;
    decoded_response.verify_authenticated_session(
        &authenticated_decoded,
        now + 3,
        &authenticator,
    )?;
    assert_eq!(decoded_response.status(), ResponseStatus::Prepared);
    assert_eq!(
        decoded_response
            .observed_generations()
            .map(|value| value.writer()),
        Some(9)
    );
    Ok(())
}

#[test]
fn wire_decoders_reject_wrong_domain_trailing_and_oversized_frames() -> Result<(), ProtocolError> {
    let packet = BootstrapPacket::generate(41, 7, 3)?;
    let frame = encode_bootstrap(&packet)?;

    assert!(matches!(
        decode_bootstrap(&[]),
        Err(ProtocolError::EmptyFrame)
    ));
    assert!(matches!(
        decode_bootstrap(&[0_u8; 4]),
        Err(ProtocolError::InvalidFrameLength)
    ));

    let mut wrong_domain = frame.clone();
    wrong_domain[4] ^= 1;
    assert!(matches!(
        decode_bootstrap(&wrong_domain),
        Err(ProtocolError::InvalidDomain)
    ));
    assert!(matches!(
        decode_bootstrap(&with_trailing_payload_byte(frame)),
        Err(ProtocolError::TrailingBytes)
    ));
    assert!(matches!(
        decode_bootstrap(&vec![0_u8; MAX_FRAME_BYTES + 1]),
        Err(ProtocolError::FrameTooLarge)
    ));
    Ok(())
}

#[test]
fn request_and_response_construction_rejects_missing_authority_fields() -> Result<(), ProtocolError>
{
    let now = now_unix_millis()?;
    let hello = broker_hello(now)?;
    assert!(matches!(
        ExpectedGenerations::try_new(0, 1, 1, 1),
        Err(ProtocolError::InvalidEpoch)
    ));
    assert!(matches!(
        TargetDescriptor::try_new(TargetKind::Device, vec![1], vec![2], vec![0; 1025]),
        Err(ProtocolError::FieldTooLarge)
    ));

    assert!(matches!(
        UntrustedRequest::authenticate_wire(
            request_values(&hello, now, RequestKind::Commit, None)?,
            &hello.clone_authenticator(),
        ),
        Err(ProtocolError::InvalidOpaqueToken)
    ));
    assert!(matches!(
        OpaquePreparedToken::from_untrusted_wire_bytes(vec![0_u8; 96]),
        Err(ProtocolError::InvalidOpaqueToken)
    ));
    Ok(())
}
