use ocentra_protected_capability_custody_protocol::bootstrap::{
    BootstrapIdentity, BootstrapPacket,
};
use ocentra_protected_capability_custody_protocol::constants::NONCE_BYTES;
use ocentra_protected_capability_custody_protocol::types::{Nonce, ProtocolError};
use ocentra_protected_capability_custody_protocol::{decode_bootstrap, encode_bootstrap};

#[test]
fn bootstrap_packet_round_trips_through_the_public_codec() -> Result<(), ProtocolError> {
    let packet = BootstrapPacket::generate(42, 9, 7)?;
    let expected_identity = packet.identity();
    let frame = encode_bootstrap(&packet)?;
    let decoded = decode_bootstrap(&frame)?;

    assert_eq!(decoded.into_identity(), expected_identity);
    Ok(())
}

#[test]
fn bootstrap_identity_and_frame_reject_invalid_inputs() -> Result<(), ProtocolError> {
    let nonce = Nonce::try_from_bytes(&[7_u8; NONCE_BYTES])?;
    assert!(matches!(
        BootstrapIdentity::try_new(0, 1, 7, nonce),
        Err(ProtocolError::InvalidProcessId)
    ));
    assert!(matches!(
        BootstrapIdentity::try_new(42, 0, 7, nonce),
        Err(ProtocolError::InvalidEpoch)
    ));
    assert!(matches!(
        BootstrapIdentity::try_new(42, 1, 0, nonce),
        Err(ProtocolError::InvalidProcessId)
    ));
    assert!(matches!(
        decode_bootstrap(&[0_u8; 4]),
        Err(ProtocolError::InvalidFrameLength)
    ));
    Ok(())
}
