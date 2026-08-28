use std::convert::TryInto;

use chrono::{DateTime, Utc};
use ocentra_family_identity_core::account_identity_authority_issuer_client::AccountIdentityAuthorityIssuerClient;
use ocentra_family_identity_core::account_identity_authority_producer_v2::{
    expected_key_id, verify, AccountIdentityAuthorityProducerV2Error,
};
use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Claims, AccountIdentityAuthorityProducerV2Operation,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ENVIRONMENT,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_ALGORITHM,
};
use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};

const NOW: &str = "2026-08-28T19:01:00.000Z";

struct SignedWire {
    wire: Vec<u8>,
    public_key: [u8; 65],
}

fn now() -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(NOW)
        .expect("test time")
        .with_timezone(&Utc)
}

fn canonical_claims() -> Vec<u8> {
    serde_json::to_vec(&AccountIdentityAuthorityProducerV2Claims {
        account_id: "account-1".to_owned(),
        household_id: "household-1".to_owned(),
        provider: "firebase".to_owned(),
        provider_subject: "provider-subject-1".to_owned(),
        member_id: "member-1".to_owned(),
        device_id: "device-1".to_owned(),
        session_id: "session-1".to_owned(),
    })
    .expect("canonical claims JSON")
}

fn signed_wire(
    operation: AccountIdentityAuthorityProducerV2Operation,
    payload: Vec<u8>,
    issued_at: &str,
    expires_at: &str,
) -> SignedWire {
    let rng = SystemRandom::new();
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
        .expect("generate test signing key");
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref(), &rng)
        .expect("parse test signing key");
    let public_key: [u8; 65] = key_pair
        .public_key()
        .as_ref()
        .try_into()
        .expect("P-256 public key length");
    let key_id = expected_key_id(&public_key);
    let signing_bytes = signing_bytes(operation, &key_id, issued_at, expires_at, &payload);
    let signature = key_pair
        .sign(&rng, &signing_bytes)
        .expect("sign test transport");
    let signature: [u8; 64] = signature
        .as_ref()
        .try_into()
        .expect("P-256 signature length");
    let mut wire = signing_bytes;
    wire.extend_from_slice(&signature);
    SignedWire { wire, public_key }
}

fn signing_bytes(
    operation: AccountIdentityAuthorityProducerV2Operation,
    key_id: &str,
    issued_at: &str,
    expires_at: &str,
    payload: &[u8],
) -> Vec<u8> {
    let key_generation = 1_u64.to_be_bytes();
    let enrollment_generation = 2_u64.to_be_bytes();
    let authority_generation = 3_u64.to_be_bytes();
    let session_generation = 4_u64.to_be_bytes();
    let fields: [&[u8]; 16] = [
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION.as_bytes(),
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE.as_bytes(),
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ENVIRONMENT.as_bytes(),
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_ALGORITHM.as_bytes(),
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE.as_bytes(),
        b"sha256:receipt:unit-test",
        key_id.as_bytes(),
        b"sha256:binding:unit-test",
        &key_generation,
        &enrollment_generation,
        &authority_generation,
        &session_generation,
        b"correlation-unit-test",
        b"idempotency-unit-test",
        issued_at.as_bytes(),
        expires_at.as_bytes(),
    ];
    let mut bytes = Vec::new();
    bytes.extend_from_slice(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN);
    bytes.push(operation.message_kind());
    for field in fields {
        append_field(&mut bytes, field);
    }
    append_field(&mut bytes, payload);
    bytes
}

fn append_field(target: &mut Vec<u8>, field: &[u8]) {
    target.extend_from_slice(&(field.len() as u32).to_be_bytes());
    target.extend_from_slice(field);
}

#[test]
fn canonical_millisecond_utc_timestamps_are_accepted() {
    let fixture = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        canonical_claims(),
        NOW,
        "2026-08-28T19:06:00.000Z",
    );

    let verified = verify(&fixture.wire, &fixture.public_key, now())
        .expect("canonical UTC-millisecond timestamps verify");
    assert_eq!(verified.issued_at(), NOW);
    assert_eq!(verified.expires_at(), "2026-08-28T19:06:00.000Z");
}

#[test]
fn noncanonical_rfc3339_timestamp_forms_are_rejected() {
    for (issued_at, expires_at) in [
        ("2026-08-28T19:00:00Z", "2026-08-28T19:06:00.000Z"),
        ("2026-08-28T19:00:00.000+00:00", "2026-08-28T19:06:00.000Z"),
        (NOW, "2026-08-28T19:06:00Z"),
    ] {
        let fixture = signed_wire(
            AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
            canonical_claims(),
            issued_at,
            expires_at,
        );

        assert!(matches!(
            verify(&fixture.wire, &fixture.public_key, now()),
            Err(AccountIdentityAuthorityProducerV2Error::InvalidWire)
        ));
    }
}

#[test]
fn malformed_wire_is_rejected_before_any_authority_result() {
    let fixture = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        canonical_claims(),
        NOW,
        "2026-08-28T19:06:00.000Z",
    );
    let malformed = &fixture.wire[..64];

    assert!(matches!(
        verify(malformed, &fixture.public_key, now()),
        Err(AccountIdentityAuthorityProducerV2Error::InvalidWire)
    ));
}

#[test]
fn canonical_claim_bytes_are_required_even_with_a_valid_signature() {
    let noncanonical = br#"{"sessionId":"session-1","deviceId":"device-1","memberId":"member-1","providerSubject":"provider-subject-1","provider":"firebase","householdId":"household-1","accountId":"account-1"}"#.to_vec();
    let fixture = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        noncanonical,
        NOW,
        "2026-08-28T19:06:00.000Z",
    );

    assert!(matches!(
        verify(&fixture.wire, &fixture.public_key, now()),
        Err(AccountIdentityAuthorityProducerV2Error::InvalidWire)
    ));
}

#[test]
fn expired_and_future_issue_times_are_rejected() {
    let expired = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        canonical_claims(),
        "2026-08-28T18:00:00.000Z",
        "2026-08-28T18:05:00.000Z",
    );
    assert!(matches!(
        verify(&expired.wire, &expired.public_key, now()),
        Err(AccountIdentityAuthorityProducerV2Error::AuthorityExpired)
    ));

    let too_far_ahead = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        canonical_claims(),
        "2026-08-28T19:01:31.000Z",
        "2026-08-28T19:06:31.000Z",
    );
    assert!(matches!(
        verify(&too_far_ahead.wire, &too_far_ahead.public_key, now()),
        Err(AccountIdentityAuthorityProducerV2Error::AuthorityExpired)
    ));
}

#[test]
fn unsupported_acknowledgement_operation_cannot_be_verified_as_authority() {
    let fixture = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::AcknowledgeReceipt,
        b"acknowledgement-payload".to_vec(),
        NOW,
        "2026-08-28T19:06:00.000Z",
    );

    assert!(matches!(
        verify(&fixture.wire, &fixture.public_key, now()),
        Err(AccountIdentityAuthorityProducerV2Error::UnsupportedOperation)
    ));
}

#[test]
fn account_owned_issuer_mount_remains_typed_unavailable_without_custody() {
    assert!(matches!(
        AccountIdentityAuthorityIssuerClient::mount_account_owned(),
        Err(ocentra_family_identity_core::account_identity_authority_issuer_client::
            AccountIdentityAuthorityIssuerClientError::Unavailable)
    ));
}
