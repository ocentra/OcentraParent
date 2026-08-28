use chrono::Utc;
use ocentra_family_identity_core::account_identity_authority_issuer_client::{
    AccountIdentityAuthorityIssuerClient, AccountIdentityAuthorityIssuerClientError,
};
use ocentra_family_identity_core::account_identity_authority_producer_v2::{
    expected_key_id, verify, AccountIdentityAuthorityProducerV2Error,
};

fn p256_generator() -> [u8; 65] {
    [
        0x04, 0x6b, 0x17, 0xd1, 0xf2, 0xe1, 0x2c, 0x42, 0x47, 0xf8, 0xbc, 0xe6, 0xe5, 0x63, 0xa4,
        0x40, 0xf2, 0x77, 0x03, 0x7d, 0x81, 0x2d, 0xeb, 0x33, 0xa0, 0xf4, 0xa1, 0x39, 0x45, 0xd8,
        0x98, 0xc2, 0x96, 0x4f, 0xe3, 0x42, 0xe2, 0xfe, 0x1a, 0x7f, 0x9b, 0x8e, 0xe7, 0xeb, 0x4a,
        0x7c, 0x0f, 0x9e, 0x16, 0x2b, 0xce, 0x33, 0x57, 0x6b, 0x31, 0x5e, 0xce, 0xcb, 0xb6, 0x40,
        0x68, 0x37, 0xbf, 0x51, 0xf5,
    ]
}

#[test]
fn account_owner_mount_does_not_guess_an_account_store_path() {
    assert!(matches!(
        AccountIdentityAuthorityIssuerClient::mount_account_owned(),
        Err(AccountIdentityAuthorityIssuerClientError::Unavailable)
    ));
}

#[test]
fn v2_transport_verifier_requires_a_canonical_key_and_wire() {
    let public_key = p256_generator();
    let key_id = expected_key_id(&public_key);
    assert!(key_id.starts_with("sha256:ecdsa-p256:"));
    assert_eq!(key_id.len(), "sha256:ecdsa-p256:".len() + 64);
    assert_eq!(expected_key_id(&public_key), key_id);

    assert!(matches!(
        verify(&[], &public_key, Utc::now()),
        Err(AccountIdentityAuthorityProducerV2Error::InvalidWire)
    ));
    assert!(matches!(
        verify(&[], &[0; 65], Utc::now()),
        Err(AccountIdentityAuthorityProducerV2Error::InvalidPublicKey)
    ));
}
