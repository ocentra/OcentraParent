use std::time::{SystemTime, UNIX_EPOCH};

use crate::codec::frame::reader::Cursor;
use crate::codec::frame::{append_header, append_u64, decode_frame, encode_frame};
use crate::constants::{
    ATTESTATION_DIGEST_BYTES, CORRELATION_BYTES, MESSAGE_BROKER_HELLO, MESSAGE_CLIENT_HELLO,
    NONCE_BYTES, SESSION_HANDLE_BYTES,
};
use crate::handshake::{BrokerSessionWireValues, UntrustedBrokerHello, UntrustedClientHello};
use crate::types::{
    AttestationDigest, CorrelationId, Nonce, ProtocolError, ProtocolGeneration, SessionHandle,
};

pub(super) fn encode_client(hello: &UntrustedClientHello) -> Result<Vec<u8>, ProtocolError> {
    let mut payload = Vec::with_capacity(112);
    append_header(&mut payload, MESSAGE_CLIENT_HELLO, hello.version());
    append_u64(&mut payload, hello.protocol_generation().value());
    payload.extend_from_slice(hello.nonce().as_bytes());
    payload.extend_from_slice(hello.correlation().as_bytes());
    payload.extend_from_slice(&hello.client_process_id().to_be_bytes());
    append_u64(&mut payload, hello.client_process_epoch());
    payload.extend_from_slice(&hello.client_session_id().to_be_bytes());
    encode_frame(&payload)
}

pub(super) fn decode_client(frame: &[u8]) -> Result<UntrustedClientHello, ProtocolError> {
    let payload = decode_frame(frame)?;
    let mut cursor = Cursor::new(payload);
    let version = cursor.take_header(MESSAGE_CLIENT_HELLO)?;
    let protocol_generation = ProtocolGeneration::decode(cursor.take_u64()?)?;
    let nonce = Nonce::try_from_bytes(cursor.take_exact(NONCE_BYTES)?)?;
    let correlation = CorrelationId::try_from_bytes(cursor.take_exact(CORRELATION_BYTES)?)?;
    let client_process_id = cursor.take_u32()?;
    let client_process_epoch = cursor.take_u64()?;
    let client_session_id = cursor.take_u32()?;
    cursor.finish()?;
    UntrustedClientHello::try_new(
        nonce,
        correlation,
        client_process_id,
        client_process_epoch,
        client_session_id,
    )
    .map(|mut hello| {
        hello.version = version;
        hello.protocol_generation = protocol_generation;
        hello
    })
}

pub(super) fn encode_broker(hello: &UntrustedBrokerHello) -> Result<Vec<u8>, ProtocolError> {
    let mut payload = Vec::with_capacity(240);
    append_header(&mut payload, MESSAGE_BROKER_HELLO, hello.version());
    append_u64(&mut payload, hello.protocol_generation().value());
    payload.extend_from_slice(hello.client_nonce().as_bytes());
    payload.extend_from_slice(hello.broker_nonce().as_bytes());
    payload.extend_from_slice(hello.correlation().as_bytes());
    payload.extend_from_slice(&hello.client_process_id().to_be_bytes());
    append_u64(&mut payload, hello.client_process_epoch());
    payload.extend_from_slice(&hello.client_session_id().to_be_bytes());
    payload.extend_from_slice(&hello.broker_process_id().to_be_bytes());
    payload.extend_from_slice(&hello.broker_session_id().to_be_bytes());
    append_u64(&mut payload, hello.broker_epoch());
    append_u64(&mut payload, hello.broker_key_epoch());
    append_u64(&mut payload, hello.writer_lease_epoch());
    append_u64(&mut payload, hello.watermark());
    payload.extend_from_slice(hello.session_handle().as_bytes());
    payload.extend_from_slice(hello.attestation_digest().as_bytes());
    append_u64(&mut payload, hello.session_expires_at_unix_millis());
    encode_frame(&payload)
}

pub(super) fn decode_broker(frame: &[u8]) -> Result<UntrustedBrokerHello, ProtocolError> {
    let payload = decode_frame(frame)?;
    let mut cursor = Cursor::new(payload);
    let version = cursor.take_header(MESSAGE_BROKER_HELLO)?;
    let protocol_generation = ProtocolGeneration::decode(cursor.take_u64()?)?;
    let client_nonce = Nonce::try_from_bytes(cursor.take_exact(NONCE_BYTES)?)?;
    let broker_nonce = Nonce::try_from_bytes(cursor.take_exact(NONCE_BYTES)?)?;
    let correlation = CorrelationId::try_from_bytes(cursor.take_exact(CORRELATION_BYTES)?)?;
    let client_process_id = cursor.take_u32()?;
    let client_process_epoch = cursor.take_u64()?;
    let client_session_id = cursor.take_u32()?;
    let broker_process_id = cursor.take_u32()?;
    let broker_session_id = cursor.take_u32()?;
    let broker_epoch = cursor.take_u64()?;
    let broker_key_epoch = cursor.take_u64()?;
    let writer_lease_epoch = cursor.take_u64()?;
    let watermark = cursor.take_u64()?;
    let session_handle =
        SessionHandle::try_from_untrusted_bytes(cursor.take_exact(SESSION_HANDLE_BYTES)?)?;
    let attestation_digest =
        AttestationDigest::try_from_untrusted_bytes(cursor.take_exact(ATTESTATION_DIGEST_BYTES)?)?;
    let session_expires_at_unix_millis = cursor.take_u64()?;
    cursor.finish()?;
    let client = UntrustedClientHello::try_new(
        client_nonce,
        correlation,
        client_process_id,
        client_process_epoch,
        client_session_id,
    )?;
    UntrustedBrokerHello::from_untrusted_wire(
        &client,
        BrokerSessionWireValues {
            broker_nonce,
            broker_process_id,
            broker_session_id,
            broker_epoch,
            broker_key_epoch,
            writer_lease_epoch,
            watermark,
            session_handle,
            session_expires_at_unix_millis,
        },
        attestation_digest,
        unix_now_millis()?,
    )
    .map(|mut hello| {
        hello.version = version;
        hello.protocol_generation = protocol_generation;
        hello
    })
}

fn unix_now_millis() -> Result<u64, ProtocolError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(map_clock_error)?;
    u64::try_from(duration.as_millis()).map_err(map_clock_overflow)
}

fn map_clock_error(_error: std::time::SystemTimeError) -> ProtocolError {
    ProtocolError::InvalidExpiry
}

fn map_clock_overflow(_error: std::num::TryFromIntError) -> ProtocolError {
    ProtocolError::InvalidExpiry
}
