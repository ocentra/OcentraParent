use crate::codec::frame::{
    append_field, append_header, append_u64, decode_frame, encode_frame, Cursor,
};
use crate::constants::{
    ATTESTATION_DIGEST_BYTES, CORRELATION_BYTES, MESSAGE_REQUEST, NONCE_BYTES, SESSION_HANDLE_BYTES,
};
use crate::handshake::{AttestationDigest, SessionHandle};
use crate::request::{Request, RequestKind, UntrustedRequestWireValues};
use crate::target::{Action, TargetDescriptor, TargetKind};
use crate::types::{CorrelationId, Nonce, ProtocolError};

pub(super) fn encode(request: &Request) -> Result<Vec<u8>, ProtocolError> {
    let mut payload = Vec::with_capacity(352);
    append_header(&mut payload, MESSAGE_REQUEST, request.version());
    payload.extend_from_slice(request.nonce().as_bytes());
    payload.extend_from_slice(request.broker_nonce().as_bytes());
    payload.extend_from_slice(request.correlation().as_bytes());
    append_u64(&mut payload, request.client_process_epoch());
    append_u64(&mut payload, request.broker_epoch());
    append_u64(&mut payload, request.broker_key_epoch());
    append_u64(&mut payload, request.writer_lease_epoch());
    append_u64(&mut payload, request.watermark());
    append_u64(&mut payload, request.expected_authority_generation());
    append_u64(&mut payload, request.expected_target_generation());
    append_u64(&mut payload, request.expected_key_generation());
    append_u64(&mut payload, request.expected_writer_generation());
    payload.extend_from_slice(request.session_handle().as_bytes());
    payload.extend_from_slice(request.attestation_digest().as_bytes());
    payload.push(request.kind() as u8);
    append_field(&mut payload, request.operation())?;
    payload.push(request.action() as u8);
    encode_target(&mut payload, request.target())?;
    append_field(&mut payload, request.opaque_token())?;
    encode_frame(&payload)
}

pub(super) fn decode(frame: &[u8]) -> Result<Request, ProtocolError> {
    let payload = decode_frame(frame)?;
    let mut cursor = Cursor::new(payload);
    let version = cursor.take_header(MESSAGE_REQUEST)?;
    let nonce = Nonce::try_from_bytes(cursor.take_exact(NONCE_BYTES)?)?;
    let broker_nonce = Nonce::try_from_bytes(cursor.take_exact(NONCE_BYTES)?)?;
    let correlation = CorrelationId::try_from_bytes(cursor.take_exact(CORRELATION_BYTES)?)?;
    let client_process_epoch = cursor.take_u64()?;
    let broker_epoch = cursor.take_u64()?;
    let broker_key_epoch = cursor.take_u64()?;
    let writer_lease_epoch = cursor.take_u64()?;
    let watermark = cursor.take_u64()?;
    let expected_authority_generation = cursor.take_u64()?;
    let expected_target_generation = cursor.take_u64()?;
    let expected_key_generation = cursor.take_u64()?;
    let expected_writer_generation = cursor.take_u64()?;
    let session_handle = SessionHandle::try_from_bytes(cursor.take_exact(SESSION_HANDLE_BYTES)?)?;
    let attestation_digest =
        AttestationDigest::try_from_bytes(cursor.take_exact(ATTESTATION_DIGEST_BYTES)?)?;
    let kind = RequestKind::decode(cursor.take_u8()?)?;
    let operation = cursor.take_field()?;
    let action = Action::decode(cursor.take_u8()?)?;
    let target = decode_target(&mut cursor)?;
    let opaque_token = cursor.take_field()?;
    cursor.finish()?;
    Request::try_from_untrusted_wire_values(UntrustedRequestWireValues {
        nonce,
        broker_nonce,
        correlation,
        client_process_epoch,
        broker_epoch,
        broker_key_epoch,
        writer_lease_epoch,
        watermark,
        expected_authority_generation,
        expected_target_generation,
        expected_key_generation,
        expected_writer_generation,
        session_handle,
        attestation_digest,
        kind,
        operation,
        action,
        target,
        opaque_token,
    })
    .map(|mut request| {
        request.version = version;
        request
    })
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
