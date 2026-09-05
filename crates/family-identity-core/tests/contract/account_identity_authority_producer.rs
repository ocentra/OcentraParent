use std::convert::TryInto;

use chrono::{DateTime, Utc};
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

const ISSUED_AT: &str = "2026-08-28T19:00:00.000Z";
const EXPIRES_AT: &str = "2026-08-28T19:05:00.000Z";
const NOW: &str = "2026-08-28T19:01:00.000Z";

type TestResult<T> = Result<T, String>;

fn with_context<T, E: std::fmt::Debug>(result: Result<T, E>, context: &str) -> TestResult<T> {
    result.map_err(|error| format!("{context}: {error:?}"))
}

struct SignedWire {
    wire: Vec<u8>,
    public_key: [u8; 65],
    key_id: String,
    receipt_id: String,
}

fn now() -> TestResult<DateTime<Utc>> {
    Ok(with_context(DateTime::parse_from_rfc3339(NOW), "test time")?.with_timezone(&Utc))
}

fn claims() -> AccountIdentityAuthorityProducerV2Claims {
    AccountIdentityAuthorityProducerV2Claims {
        account_id: "account-1".to_owned(),
        household_id: "household-1".to_owned(),
        provider: "firebase".to_owned(),
        provider_subject: "provider-subject-1".to_owned(),
        member_id: "member-1".to_owned(),
        device_id: "device-1".to_owned(),
        session_id: "session-1".to_owned(),
    }
}

fn signed_wire(
    operation: AccountIdentityAuthorityProducerV2Operation,
    payload: &[u8],
    issued_at: &str,
    expires_at: &str,
) -> TestResult<SignedWire> {
    let rng = SystemRandom::new();
    let pkcs8 = with_context(
        EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng),
        "generate test signing key",
    )?;
    let key_pair = with_context(
        EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref(), &rng),
        "parse test signing key",
    )?;
    let public_key: [u8; 65] = with_context(
        key_pair.public_key().as_ref().try_into(),
        "P-256 public key length",
    )?;
    let key_id = expected_key_id(&public_key);
    let receipt_id = "sha256:receipt:contract-test".to_owned();
    let signing_bytes = signing_bytes(
        operation,
        &receipt_id,
        &key_id,
        issued_at,
        expires_at,
        payload,
    );
    let signature = with_context(key_pair.sign(&rng, &signing_bytes), "sign test transport")?;
    let mut signature: [u8; 64] =
        with_context(signature.as_ref().try_into(), "P-256 signature length")?;
    normalize_signature_to_low_s(&mut signature);
    let mut wire = signing_bytes;
    wire.extend_from_slice(&signature);
    Ok(SignedWire {
        wire,
        public_key,
        key_id,
        receipt_id,
    })
}

fn signing_bytes(
    operation: AccountIdentityAuthorityProducerV2Operation,
    receipt_id: &str,
    key_id: &str,
    issued_at: &str,
    expires_at: &str,
    payload: &[u8],
) -> Vec<u8> {
    let service_binding_id = "sha256:binding:contract-test";
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
        receipt_id.as_bytes(),
        key_id.as_bytes(),
        service_binding_id.as_bytes(),
        &key_generation,
        &enrollment_generation,
        &authority_generation,
        &session_generation,
        b"correlation-contract-test",
        b"idempotency-contract-test",
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

fn normalize_signature_to_low_s(signature: &mut [u8; 64]) {
    const P256_HALF_ORDER: [u8; 32] = [
        0x7f, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xde, 0x73, 0x7d, 0x56, 0xd3, 0x8b, 0xcf, 0x42, 0x79, 0xdc, 0xe5, 0x61, 0x7e, 0x31,
        0x92, 0xa8,
    ];
    const P256_ORDER: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xbc, 0xe6, 0xfa, 0xad, 0xa7, 0x17, 0x9e, 0x84, 0xf3, 0xb9, 0xca, 0xc2, 0xfc, 0x63,
        0x25, 0x51,
    ];

    if signature[32..].cmp(P256_HALF_ORDER.as_slice()) != std::cmp::Ordering::Greater {
        return;
    }

    let mut low_s = [0u8; 32];
    let mut borrow = 0_i16;
    for index in (0..32).rev() {
        let difference = P256_ORDER[index] as i16 - signature[32 + index] as i16 - borrow;
        if difference < 0 {
            low_s[index] = (difference + 256) as u8;
            borrow = 1;
        } else {
            low_s[index] = difference as u8;
            borrow = 0;
        }
    }
    signature[32..].copy_from_slice(&low_s);
}

#[test]
fn signed_issue_transport_verifies_with_typed_canonical_claims() -> TestResult<()> {
    let claims = claims();
    let payload = with_context(serde_json::to_vec(&claims), "canonical claims JSON")?;
    let fixture = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        &payload,
        ISSUED_AT,
        EXPIRES_AT,
    )?;

    let verified = with_context(
        verify(&fixture.wire, &fixture.public_key, now()?),
        "verify issue wire",
    )?;

    assert_eq!(
        verified.operation(),
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority
    );
    assert_eq!(verified.key_id(), fixture.key_id);
    assert_eq!(verified.receipt_id(), fixture.receipt_id);
    assert_eq!(
        verified.service_binding_id(),
        "sha256:binding:contract-test"
    );
    assert_eq!(verified.key_generation(), 1);
    assert_eq!(verified.enrollment_generation(), 2);
    assert_eq!(verified.authority_generation(), 3);
    assert_eq!(verified.session_generation(), 4);
    assert_eq!(verified.correlation_id(), "correlation-contract-test");
    assert_eq!(verified.idempotency_key(), "idempotency-contract-test");
    assert_eq!(verified.issued_at(), ISSUED_AT);
    assert_eq!(verified.expires_at(), EXPIRES_AT);
    assert_eq!(verified.claims(), &claims);
    Ok(())
}

#[test]
fn signed_issue_transport_rejects_noncanonical_rfc3339_timestamps() -> TestResult<()> {
    let payload = with_context(serde_json::to_vec(&claims()), "canonical claims JSON")?;
    for (issued_at, expires_at) in [
        ("2026-08-28T19:00:00Z", EXPIRES_AT),
        ("2026-08-28T19:00:00.000+00:00", EXPIRES_AT),
        (ISSUED_AT, "2026-08-28T19:05:00Z"),
    ] {
        let fixture = signed_wire(
            AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
            &payload,
            issued_at,
            expires_at,
        )?;

        assert!(matches!(
            verify(&fixture.wire, &fixture.public_key, now()?),
            Err(AccountIdentityAuthorityProducerV2Error::InvalidWire)
        ));
    }
    Ok(())
}

#[test]
fn domain_key_and_signature_changes_fail_closed() -> TestResult<()> {
    let payload = with_context(serde_json::to_vec(&claims()), "canonical claims JSON")?;
    let fixture = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        &payload,
        ISSUED_AT,
        EXPIRES_AT,
    )?;

    let mut wrong_domain = fixture.wire.clone();
    wrong_domain[0] ^= 1;
    assert!(matches!(
        verify(&wrong_domain, &fixture.public_key, now()?),
        Err(AccountIdentityAuthorityProducerV2Error::InvalidWire)
    ));

    let other_key = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        &payload,
        ISSUED_AT,
        EXPIRES_AT,
    )?;
    assert!(matches!(
        verify(&fixture.wire, &other_key.public_key, now()?),
        Err(AccountIdentityAuthorityProducerV2Error::InvalidKeyId)
    ));

    let mut bad_signature = fixture.wire.clone();
    let last = bad_signature.len() - 1;
    bad_signature[last] ^= 1;
    assert!(matches!(
        verify(&bad_signature, &fixture.public_key, now()?),
        Err(AccountIdentityAuthorityProducerV2Error::InvalidSignature
            | AccountIdentityAuthorityProducerV2Error::SignatureInvalid)
    ));
    Ok(())
}
