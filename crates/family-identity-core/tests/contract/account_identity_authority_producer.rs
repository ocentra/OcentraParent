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

struct SignedWire {
    wire: Vec<u8>,
    public_key: [u8; 65],
    key_id: String,
    receipt_id: String,
}

fn now() -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(NOW)
        .expect("test time")
        .with_timezone(&Utc)
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
    let receipt_id = "sha256:receipt:contract-test".to_owned();
    let signing_bytes = signing_bytes(
        operation,
        &receipt_id,
        &key_id,
        issued_at,
        expires_at,
        &payload,
    );
    let signature = key_pair
        .sign(&rng, &signing_bytes)
        .expect("sign test transport");
    let signature: [u8; 64] = signature
        .as_ref()
        .try_into()
        .expect("P-256 signature length");
    let mut wire = signing_bytes;
    wire.extend_from_slice(&signature);
    SignedWire {
        wire,
        public_key,
        key_id,
        receipt_id,
    }
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

#[test]
fn signed_issue_transport_verifies_with_typed_canonical_claims() {
    let claims = claims();
    let payload = serde_json::to_vec(&claims).expect("canonical claims JSON");
    let fixture = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        payload,
        ISSUED_AT,
        EXPIRES_AT,
    );

    let verified = verify(&fixture.wire, &fixture.public_key, now()).expect("verify issue wire");

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
}

#[test]
fn domain_key_and_signature_changes_fail_closed() {
    let payload = serde_json::to_vec(&claims()).expect("canonical claims JSON");
    let fixture = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        payload.clone(),
        ISSUED_AT,
        EXPIRES_AT,
    );

    let mut wrong_domain = fixture.wire.clone();
    wrong_domain[0] ^= 1;
    assert!(matches!(
        verify(&wrong_domain, &fixture.public_key, now()),
        Err(AccountIdentityAuthorityProducerV2Error::InvalidWire)
    ));

    let other_key = signed_wire(
        AccountIdentityAuthorityProducerV2Operation::IssueCurrentAuthority,
        payload,
        ISSUED_AT,
        EXPIRES_AT,
    );
    assert!(matches!(
        verify(&fixture.wire, &other_key.public_key, now()),
        Err(AccountIdentityAuthorityProducerV2Error::InvalidKeyId)
    ));

    let mut bad_signature = fixture.wire.clone();
    let last = bad_signature.len() - 1;
    bad_signature[last] ^= 1;
    assert!(matches!(
        verify(&bad_signature, &fixture.public_key, now()),
        Err(AccountIdentityAuthorityProducerV2Error::InvalidSignature
            | AccountIdentityAuthorityProducerV2Error::SignatureInvalid)
    ));
}
