use crate::codec::frame::{
    append_field, append_header, append_u64, decode_frame, encode_frame, Cursor,
};
use crate::constants::{
    ATTESTATION_DIGEST_BYTES, CORRELATION_BYTES, MESSAGE_RESPONSE, NONCE_BYTES,
    SESSION_HANDLE_BYTES,
};
use crate::handshake::{AttestationDigest, SessionHandle};
use crate::response::{Response, ResponseStatus};
use crate::types::{CorrelationId, Nonce, ProtocolError};

pub(super) fn encode(response: &Response) -> Result<Vec<u8>, ProtocolError> {
    let mut payload = Vec::with_capacity(256);
    append_header(&mut payload, MESSAGE_RESPONSE, response.version());
    payload.extend_from_slice(response.nonce().as_bytes());
    payload.extend_from_slice(response.correlation().as_bytes());
    append_u64(&mut payload, response.client_process_epoch());
    payload.extend_from_slice(response.session_handle().as_bytes());
    payload.extend_from_slice(response.attestation_digest().as_bytes());
    payload.push(response.status() as u8);
    append_u64(&mut payload, response.broker_epoch());
    append_u64(&mut payload, response.broker_key_epoch());
    append_u64(&mut payload, response.writer_lease_epoch());
    append_u64(&mut payload, response.watermark());
    append_u64(&mut payload, response.authority_generation());
    append_u64(&mut payload, response.target_generation());
    append_u64(&mut payload, response.key_generation());
    append_u64(&mut payload, response.writer_generation());
    append_field(&mut payload, response.opaque_token())?;
    encode_frame(payload)
}

pub(super) fn decode(frame: &[u8]) -> Result<Response, ProtocolError> {
    let payload = decode_frame(frame)?;
    let mut cursor = Cursor::new(payload);
    let version = cursor.take_header(MESSAGE_RESPONSE)?;
    let nonce = Nonce::try_from_bytes(cursor.take_exact(NONCE_BYTES)?)?;
    let correlation = CorrelationId::try_from_bytes(cursor.take_exact(CORRELATION_BYTES)?)?;
    let client_process_epoch = cursor.take_u64()?;
    let session_handle = SessionHandle::try_from_bytes(cursor.take_exact(SESSION_HANDLE_BYTES)?)?;
    let attestation_digest =
        AttestationDigest::try_from_bytes(cursor.take_exact(ATTESTATION_DIGEST_BYTES)?)?;
    let status = ResponseStatus::decode(cursor.take_u8()?)?;
    let broker_epoch = cursor.take_u64()?;
    let broker_key_epoch = cursor.take_u64()?;
    let writer_lease_epoch = cursor.take_u64()?;
    let watermark = cursor.take_u64()?;
    let authority_generation = cursor.take_u64()?;
    let target_generation = cursor.take_u64()?;
    let key_generation = cursor.take_u64()?;
    let writer_generation = cursor.take_u64()?;
    let opaque_token = cursor.take_field()?;
    cursor.finish()?;
    Response::from_parts(
        nonce,
        correlation,
        client_process_epoch,
        session_handle,
        attestation_digest,
        status,
        broker_epoch,
        broker_key_epoch,
        writer_lease_epoch,
        watermark,
        authority_generation,
        target_generation,
        key_generation,
        writer_generation,
        opaque_token,
    )
    .map(|mut response| {
        response.version = version;
        response
    })
}
