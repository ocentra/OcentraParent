use crate::codec::frame::{append_header, append_u64, decode_frame, encode_frame, Cursor};
use crate::constants::{
    ATTESTATION_DIGEST_BYTES, CORRELATION_BYTES, MESSAGE_BROKER_HELLO, MESSAGE_CLIENT_HELLO,
    NONCE_BYTES, SESSION_HANDLE_BYTES,
};
use crate::handshake::{AttestationDigest, BrokerHello, ClientHello, SessionHandle};
use crate::types::{CorrelationId, Nonce, ProtocolError};

pub(super) fn encode_client(hello: &ClientHello) -> Result<Vec<u8>, ProtocolError> {
    let mut payload = Vec::with_capacity(96);
    append_header(&mut payload, MESSAGE_CLIENT_HELLO, hello.version());
    payload.extend_from_slice(hello.nonce().as_bytes());
    payload.extend_from_slice(hello.correlation().as_bytes());
    append_u64(&mut payload, hello.client_process_epoch());
    encode_frame(payload)
}

pub(super) fn decode_client(frame: &[u8]) -> Result<ClientHello, ProtocolError> {
    let payload = decode_frame(frame)?;
    let mut cursor = Cursor::new(payload);
    let version = cursor.take_header(MESSAGE_CLIENT_HELLO)?;
    let nonce = Nonce::try_from_bytes(cursor.take_exact(NONCE_BYTES)?)?;
    let correlation = CorrelationId::try_from_bytes(cursor.take_exact(CORRELATION_BYTES)?)?;
    let client_process_epoch = cursor.take_u64()?;
    cursor.finish()?;
    ClientHello::try_new(nonce, correlation, client_process_epoch).map(|mut hello| {
        hello.version = version;
        hello
    })
}

pub(super) fn encode_broker(hello: &BrokerHello) -> Result<Vec<u8>, ProtocolError> {
    let mut payload = Vec::with_capacity(224);
    append_header(&mut payload, MESSAGE_BROKER_HELLO, hello.version());
    payload.extend_from_slice(hello.client_nonce().as_bytes());
    payload.extend_from_slice(hello.broker_nonce().as_bytes());
    payload.extend_from_slice(hello.correlation().as_bytes());
    append_u64(&mut payload, hello.client_process_epoch());
    append_u64(&mut payload, hello.broker_epoch());
    append_u64(&mut payload, hello.broker_key_epoch());
    append_u64(&mut payload, hello.writer_lease_epoch());
    append_u64(&mut payload, hello.watermark());
    append_u64(&mut payload, hello.authority_generation());
    append_u64(&mut payload, hello.target_generation());
    append_u64(&mut payload, hello.key_generation());
    append_u64(&mut payload, hello.writer_generation());
    payload.extend_from_slice(hello.session_handle().as_bytes());
    payload.extend_from_slice(hello.attestation_digest().as_bytes());
    encode_frame(payload)
}

pub(super) fn decode_broker(frame: &[u8]) -> Result<BrokerHello, ProtocolError> {
    let payload = decode_frame(frame)?;
    let mut cursor = Cursor::new(payload);
    let version = cursor.take_header(MESSAGE_BROKER_HELLO)?;
    let client_nonce = Nonce::try_from_bytes(cursor.take_exact(NONCE_BYTES)?)?;
    let broker_nonce = Nonce::try_from_bytes(cursor.take_exact(NONCE_BYTES)?)?;
    let correlation = CorrelationId::try_from_bytes(cursor.take_exact(CORRELATION_BYTES)?)?;
    let client_process_epoch = cursor.take_u64()?;
    let broker_epoch = cursor.take_u64()?;
    let broker_key_epoch = cursor.take_u64()?;
    let writer_lease_epoch = cursor.take_u64()?;
    let watermark = cursor.take_u64()?;
    let authority_generation = cursor.take_u64()?;
    let target_generation = cursor.take_u64()?;
    let key_generation = cursor.take_u64()?;
    let writer_generation = cursor.take_u64()?;
    let session_handle = SessionHandle::try_from_bytes(cursor.take_exact(SESSION_HANDLE_BYTES)?)?;
    let attestation_digest =
        AttestationDigest::try_from_bytes(cursor.take_exact(ATTESTATION_DIGEST_BYTES)?)?;
    cursor.finish()?;
    let client = ClientHello::try_new(client_nonce, correlation, client_process_epoch)?;
    BrokerHello::from_parts(
        &client,
        broker_nonce,
        broker_epoch,
        broker_key_epoch,
        writer_lease_epoch,
        watermark,
        authority_generation,
        target_generation,
        key_generation,
        writer_generation,
        session_handle,
        attestation_digest,
    )
    .map(|mut hello| {
        hello.version = version;
        hello
    })
}
