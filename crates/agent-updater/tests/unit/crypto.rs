use ocentra_parent_agent_maintenance::constants::ED25519_ALGORITHM;
use ocentra_parent_agent_maintenance::crypto::{generate_key_pair, sign_bytes, verify_bytes};

#[test]
fn generated_key_pair_signs_and_verifies_payload() -> Result<(), Box<dyn std::error::Error>> {
    let keys = generate_key_pair();
    let payload = b"ocentra-parent-update-payload";
    let (signature, key_id) = sign_bytes(payload, &keys.private_key_base64)?;

    let verification = verify_bytes(
        payload,
        &signature,
        &keys.public_key_base64,
        &key_id,
        ED25519_ALGORITHM,
    );
    assert!(
        verification.is_ok(),
        "payload verifies failed: {verification:?}"
    );
    Ok(())
}

#[test]
fn verification_rejects_tampered_payload() -> Result<(), Box<dyn std::error::Error>> {
    let keys = generate_key_pair();
    let (signature, key_id) = sign_bytes(b"trusted", &keys.private_key_base64)?;

    let result = verify_bytes(
        b"tampered",
        &signature,
        &keys.public_key_base64,
        &key_id,
        ED25519_ALGORITHM,
    );

    assert!(matches!(result, Err(error) if error.to_string().contains("verification")));
    Ok(())
}
