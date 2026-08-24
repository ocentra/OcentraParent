use crate::bootstrap::{BootstrapIdentity, BootstrapPacket};
use crate::codec::frame::reader::Cursor;
use crate::codec::frame::{append_header, append_u64, decode_frame, encode_frame};
use crate::constants::{MESSAGE_BOOTSTRAP, NONCE_BYTES};
use crate::types::{Nonce, ProtocolError};

pub(super) fn encode(packet: &BootstrapPacket) -> Result<Vec<u8>, ProtocolError> {
    let identity = packet.identity();
    let mut payload = Vec::with_capacity(96);
    append_header(
        &mut payload,
        MESSAGE_BOOTSTRAP,
        crate::types::ProtocolVersion::CURRENT,
    );
    payload.extend_from_slice(&identity.client_process_id().to_be_bytes());
    append_u64(&mut payload, identity.client_process_epoch());
    payload.extend_from_slice(&identity.client_session_id().to_be_bytes());
    payload.extend_from_slice(identity.pipe_nonce().as_bytes());
    encode_frame(&payload)
}

pub(super) fn decode(frame: &[u8]) -> Result<BootstrapPacket, ProtocolError> {
    let payload = decode_frame(frame)?;
    let mut cursor = Cursor::new(payload);
    cursor.take_header(MESSAGE_BOOTSTRAP)?;
    let client_process_id = cursor.take_u32()?;
    let client_process_epoch = cursor.take_u64()?;
    let client_session_id = cursor.take_u32()?;
    let pipe_nonce = Nonce::try_from_bytes(cursor.take_exact(NONCE_BYTES)?)?;
    cursor.finish()?;
    Ok(BootstrapPacket::from_decoded(BootstrapIdentity::try_new(
        client_process_id,
        client_process_epoch,
        client_session_id,
        pipe_nonce,
    )?))
}
