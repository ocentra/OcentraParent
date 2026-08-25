use crate::codec::frame::reader::Cursor;
use crate::codec::frame::{append_field, append_header, append_u64, decode_frame, encode_frame};
use crate::constants::{
    AUTHENTICATION_TAG_BYTES, CORRELATION_BYTES, MESSAGE_REQUEST, NONCE_BYTES,
    SESSION_HANDLE_BYTES, TRANSCRIPT_DIGEST_BYTES,
};
use crate::request::{
    authenticated::AuthenticatedRequest, ExpectedGenerations, RequestKind, RequestSessionEnvelope,
    UntrustedRequest, UntrustedRequestValues,
};
use crate::target::{Action, TargetDescriptor, TargetKind};
use crate::types::{
    AuthenticationTag, CorrelationId, Nonce, OpaquePreparedToken, ProtocolError,
    ProtocolGeneration, SessionHandle, SessionTranscriptDigest,
};

pub(super) fn encode(request: &AuthenticatedRequest) -> Result<Vec<u8>, ProtocolError> {
    let request = request.as_untrusted();
    let mut payload = Vec::with_capacity(512);
    append_header(&mut payload, MESSAGE_REQUEST, request.version());
    append_u64(&mut payload, request.protocol_generation().value());
    payload.extend_from_slice(request.nonce().as_bytes());
    payload.extend_from_slice(request.broker_nonce().as_bytes());
    payload.extend_from_slice(request.correlation().as_bytes());
    payload.extend_from_slice(&request.client_process_id().to_be_bytes());
    append_u64(&mut payload, request.client_process_epoch());
    payload.extend_from_slice(&request.client_session_id().to_be_bytes());
    payload.extend_from_slice(&request.broker_process_id().to_be_bytes());
    payload.extend_from_slice(&request.broker_session_id().to_be_bytes());
    append_u64(&mut payload, request.broker_epoch());
    append_u64(&mut payload, request.broker_key_epoch());
    append_u64(&mut payload, request.writer_lease_epoch());
    append_u64(&mut payload, request.watermark());
    payload.extend_from_slice(request.session_handle().as_bytes());
    payload.extend_from_slice(request.transcript_digest().as_bytes());
    append_u64(&mut payload, request.sequence());
    append_u64(&mut payload, request.expires_at_unix_millis());
    let generations = request.expected_generations();
    append_u64(&mut payload, generations.authority());
    append_u64(&mut payload, generations.target());
    append_u64(&mut payload, generations.key());
    append_u64(&mut payload, generations.writer());
    payload.push(request.kind() as u8);
    append_field(&mut payload, request.operation())?;
    payload.push(request.action() as u8);
    encode_target(&mut payload, request.target())?;
    let opaque_token = request
        .opaque_token()
        .map(OpaquePreparedToken::as_bytes)
        .map(<[u8; crate::constants::OPAQUE_TOKEN_BYTES]>::as_slice)
        .unwrap_or(&[]);
    append_field(&mut payload, opaque_token)?;
    payload.extend_from_slice(request.authentication_tag().as_bytes());
    encode_frame(&payload)
}

pub(super) fn decode(frame: &[u8]) -> Result<UntrustedRequest, ProtocolError> {
    let payload = decode_frame(frame)?;
    let mut cursor = Cursor::new(payload);
    let version = cursor.take_header(MESSAGE_REQUEST)?;
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
    let expected_generations = ExpectedGenerations::try_new(
        cursor.take_u64()?,
        cursor.take_u64()?,
        cursor.take_u64()?,
        cursor.take_u64()?,
    )?;
    let kind = RequestKind::decode(cursor.take_u8()?)?;
    let operation = cursor.take_field()?;
    let action = Action::decode(cursor.take_u8()?)?;
    let target = decode_target(&mut cursor)?;
    let opaque_token = decode_opaque_token(cursor.take_field()?)?;
    let authentication_tag =
        AuthenticationTag::try_from_untrusted_bytes(cursor.take_exact(AUTHENTICATION_TAG_BYTES)?)?;
    cursor.finish()?;
    UntrustedRequest::from_decoded(
        UntrustedRequestValues {
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
            expected_generations,
            kind,
            operation,
            action,
            target,
            opaque_token,
        },
        authentication_tag,
    )
}

fn encode_target(payload: &mut Vec<u8>, target: &TargetDescriptor) -> Result<(), ProtocolError> {
    payload.push(target.kind() as u8);
    append_field(payload, target.household())?;
    append_field(payload, target.device())?;
    append_field(payload, target.target())
}

fn decode_target(cursor: &mut Cursor<'_>) -> Result<TargetDescriptor, ProtocolError> {
    TargetDescriptor::try_new(
        TargetKind::decode(cursor.take_u8()?)?,
        cursor.take_field()?,
        cursor.take_field()?,
        cursor.take_field()?,
    )
}

fn decode_opaque_token(bytes: Vec<u8>) -> Result<Option<OpaquePreparedToken>, ProtocolError> {
    if bytes.is_empty() {
        Ok(None)
    } else {
        OpaquePreparedToken::from_untrusted_wire_bytes(bytes).map(Some)
    }
}
