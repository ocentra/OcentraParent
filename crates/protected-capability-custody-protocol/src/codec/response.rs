use crate::codec::frame::reader::Cursor;
use crate::codec::frame::{append_field, append_header, append_u64, decode_frame, encode_frame};
use crate::constants::{
    AUTHENTICATION_TAG_BYTES, CORRELATION_BYTES, MESSAGE_RESPONSE, NONCE_BYTES,
    REQUEST_DIGEST_BYTES, SESSION_HANDLE_BYTES, TRANSCRIPT_DIGEST_BYTES,
};
use crate::request::{RequestKind, RequestSessionEnvelope};
use crate::response::{
    DecodedResponseValues, ObservedGenerations, ResponseStatus, UntrustedResponse,
};
use crate::types::{
    AuthenticationTag, CorrelationId, Nonce, OpaquePreparedToken, ProtocolError,
    ProtocolGeneration, SessionHandle, SessionTranscriptDigest,
};

pub(super) fn encode(response: &UntrustedResponse) -> Result<Vec<u8>, ProtocolError> {
    let mut payload = Vec::with_capacity(448);
    append_header(&mut payload, MESSAGE_RESPONSE, response.version());
    append_u64(&mut payload, response.protocol_generation().value());
    payload.extend_from_slice(response.nonce().as_bytes());
    payload.extend_from_slice(response.broker_nonce().as_bytes());
    payload.extend_from_slice(response.correlation().as_bytes());
    payload.extend_from_slice(&response.client_process_id().to_be_bytes());
    append_u64(&mut payload, response.client_process_epoch());
    payload.extend_from_slice(&response.client_session_id().to_be_bytes());
    payload.extend_from_slice(&response.broker_process_id().to_be_bytes());
    payload.extend_from_slice(&response.broker_session_id().to_be_bytes());
    append_u64(&mut payload, response.broker_epoch());
    append_u64(&mut payload, response.broker_key_epoch());
    append_u64(&mut payload, response.writer_lease_epoch());
    append_u64(&mut payload, response.watermark());
    payload.extend_from_slice(response.session_handle().as_bytes());
    payload.extend_from_slice(response.transcript_digest().as_bytes());
    append_u64(&mut payload, response.sequence());
    append_u64(&mut payload, response.expires_at_unix_millis());
    payload.push(response.request_kind() as u8);
    payload.extend_from_slice(&response.request_digest());
    payload.push(response.status() as u8);
    encode_observed_generations(&mut payload, response.observed_generations());
    let opaque_token = response
        .opaque_token()
        .map(OpaquePreparedToken::as_bytes)
        .map(<[u8; crate::constants::OPAQUE_TOKEN_BYTES]>::as_slice)
        .unwrap_or(&[]);
    append_field(&mut payload, opaque_token)?;
    payload.extend_from_slice(response.authentication_tag().as_bytes());
    encode_frame(&payload)
}

pub(super) fn decode(frame: &[u8]) -> Result<UntrustedResponse, ProtocolError> {
    let payload = decode_frame(frame)?;
    let mut cursor = Cursor::new(payload);
    let version = cursor.take_header(MESSAGE_RESPONSE)?;
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
    let transcript_digest = SessionTranscriptDigest::try_from_untrusted_bytes(
        cursor.take_exact(TRANSCRIPT_DIGEST_BYTES)?,
    )?;
    let sequence = cursor.take_u64()?;
    let expires_at_unix_millis = cursor.take_u64()?;
    let request_kind = RequestKind::decode(cursor.take_u8()?)?;
    let request_digest = take_request_digest(&mut cursor)?;
    let status = ResponseStatus::decode(cursor.take_u8()?)?;
    let observed_generations = decode_observed_generations(&mut cursor)?;
    let opaque_token = decode_opaque_token(cursor.take_field()?)?;
    let authentication_tag =
        AuthenticationTag::try_from_untrusted_bytes(cursor.take_exact(AUTHENTICATION_TAG_BYTES)?)?;
    cursor.finish()?;
    UntrustedResponse::from_decoded(
        DecodedResponseValues {
            session: RequestSessionEnvelope {
                version,
                protocol_generation,
                client_nonce,
                broker_nonce,
                correlation,
                client_process_id,
                client_process_epoch,
                client_session_id,
                broker_process_id,
                broker_session_id,
                broker_epoch,
                broker_key_epoch,
                writer_lease_epoch,
                watermark,
                session_handle,
                transcript_digest,
                sequence,
                expires_at_unix_millis,
            },
            request_kind,
            request_digest,
            status,
            observed_generations,
            opaque_token,
        },
        authentication_tag,
    )
}

fn encode_observed_generations(payload: &mut Vec<u8>, generations: Option<ObservedGenerations>) {
    if let Some(generations) = generations {
        payload.push(1);
        append_u64(payload, generations.authority());
        append_u64(payload, generations.target());
        append_u64(payload, generations.key());
        append_u64(payload, generations.writer());
    } else {
        payload.push(0);
    }
}

fn decode_observed_generations(
    cursor: &mut Cursor<'_>,
) -> Result<Option<ObservedGenerations>, ProtocolError> {
    match cursor.take_u8()? {
        0 => Ok(None),
        1 => ObservedGenerations::try_new(
            cursor.take_u64()?,
            cursor.take_u64()?,
            cursor.take_u64()?,
            cursor.take_u64()?,
        )
        .map(Some),
        other => Err(ProtocolError::InvalidDiscriminant(other)),
    }
}

fn decode_opaque_token(bytes: Vec<u8>) -> Result<Option<OpaquePreparedToken>, ProtocolError> {
    if bytes.is_empty() {
        Ok(None)
    } else {
        OpaquePreparedToken::from_untrusted_wire_bytes(bytes).map(Some)
    }
}

fn take_request_digest(
    cursor: &mut Cursor<'_>,
) -> Result<[u8; REQUEST_DIGEST_BYTES], ProtocolError> {
    match cursor.take_exact(REQUEST_DIGEST_BYTES)?.try_into() {
        Ok(digest) => Ok(digest),
        Err(_error) => Err(ProtocolError::Truncated),
    }
}
