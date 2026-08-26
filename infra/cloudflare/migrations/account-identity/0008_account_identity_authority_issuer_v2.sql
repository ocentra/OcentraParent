-- Account-owned issuer v2 verification custody.
--
-- Cloudflare/D1 is deliberately limited to public verifier material,
-- currentness/CAS state, and inbound receipt idempotency. It never stores a
-- private key, signed authority payload, or an authority outbox. The Account
-- owner remains the only authority minting/signing boundary.

CREATE TABLE ocentra_account_identity_issuer_v2_currentness (
  service_binding_id TEXT NOT NULL PRIMARY KEY CHECK (
    length(service_binding_id) BETWEEN 1 AND 1024
  ),
  account_id TEXT NOT NULL CHECK (length(account_id) BETWEEN 1 AND 1024),
  household_id TEXT NOT NULL CHECK (length(household_id) BETWEEN 1 AND 1024),
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  provider_subject TEXT NOT NULL CHECK (length(provider_subject) BETWEEN 1 AND 1024),
  service TEXT NOT NULL CHECK (
    service = 'ocentra.account-authority-producer.cloudflare.v2'
  ),
  key_id TEXT NOT NULL UNIQUE CHECK (
    length(key_id) = 82 AND
    substr(key_id, 1, 18) = 'sha256:ecdsa-p256:' AND
    substr(key_id, 19) NOT GLOB '*[^0-9a-f]*'
  ),
  key_generation INTEGER NOT NULL CHECK (
    key_generation > 0 AND key_generation <= 9007199254740991
  ),
  authority_generation INTEGER NOT NULL CHECK (
    authority_generation > 0 AND authority_generation <= 9007199254740991
  ),
  session_generation INTEGER NOT NULL CHECK (
    session_generation > 0 AND session_generation <= 9007199254740991
  ),
  public_key BLOB NOT NULL CHECK (
    length(public_key) = 65 AND substr(public_key, 1, 1) = X'04'
  ),
  status TEXT NOT NULL CHECK (status IN ('active', 'revoked')),
  created_at TEXT NOT NULL CHECK (julianday(created_at) IS NOT NULL),
  updated_at TEXT NOT NULL CHECK (julianday(updated_at) IS NOT NULL),
  UNIQUE (account_id, household_id, service, key_generation)
) STRICT;

CREATE INDEX idx_ocentra_account_identity_issuer_v2_currentness_binding
  ON ocentra_account_identity_issuer_v2_currentness (service, service_binding_id, status);

CREATE TABLE ocentra_account_identity_issuer_v2_inbound_receipts (
  receipt_id TEXT NOT NULL CHECK (
    length(receipt_id) = 79 AND
    substr(receipt_id, 1, 15) = 'sha256:receipt:' AND
    substr(receipt_id, 16) NOT GLOB '*[^0-9a-f]*'
  ),
  operation TEXT NOT NULL CHECK (
    operation IN ('IssueCurrentAuthority', 'AcknowledgeReceipt')
  ),
  account_id TEXT NOT NULL CHECK (length(account_id) BETWEEN 1 AND 1024),
  household_id TEXT NOT NULL CHECK (length(household_id) BETWEEN 1 AND 1024),
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  provider_subject TEXT NOT NULL CHECK (length(provider_subject) BETWEEN 1 AND 1024),
  service TEXT NOT NULL CHECK (
    service = 'ocentra.account-authority-producer.cloudflare.v2'
  ),
  service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) BETWEEN 1 AND 1024),
  correlation_id TEXT NOT NULL CHECK (length(correlation_id) BETWEEN 1 AND 1024),
  idempotency_key TEXT NOT NULL CHECK (length(idempotency_key) BETWEEN 1 AND 1024),
  payload_digest TEXT NOT NULL CHECK (
    length(payload_digest) = 71 AND
    substr(payload_digest, 1, 7) = 'sha256:' AND
    substr(payload_digest, 8) NOT GLOB '*[^0-9a-f]*'
  ),
  authority_payload_digest TEXT NOT NULL CHECK (
    length(authority_payload_digest) = 71 AND
    substr(authority_payload_digest, 1, 7) = 'sha256:' AND
    substr(authority_payload_digest, 8) NOT GLOB '*[^0-9a-f]*'
  ),
  key_id TEXT NOT NULL CHECK (
    length(key_id) = 82 AND
    substr(key_id, 1, 18) = 'sha256:ecdsa-p256:' AND
    substr(key_id, 19) NOT GLOB '*[^0-9a-f]*'
  ),
  key_generation INTEGER NOT NULL CHECK (
    key_generation > 0 AND key_generation <= 9007199254740991
  ),
  authority_generation INTEGER NOT NULL CHECK (
    authority_generation > 0 AND authority_generation <= 9007199254740991
  ),
  session_generation INTEGER NOT NULL CHECK (
    session_generation > 0 AND session_generation <= 9007199254740991
  ),
  issued_at TEXT NOT NULL CHECK (
    length(issued_at) = 24 AND julianday(issued_at) IS NOT NULL
  ),
  expires_at TEXT NOT NULL CHECK (
    length(expires_at) = 24 AND julianday(expires_at) IS NOT NULL
  ),
  wire_digest TEXT NOT NULL CHECK (
    length(wire_digest) = 71 AND
    substr(wire_digest, 1, 7) = 'sha256:' AND
    substr(wire_digest, 8) NOT GLOB '*[^0-9a-f]*'
  ),
  receipt_state TEXT NOT NULL CHECK (receipt_state = 'accepted'),
  recorded_at TEXT NOT NULL CHECK (julianday(recorded_at) IS NOT NULL),
  CHECK (julianday(issued_at) < julianday(expires_at)),
  PRIMARY KEY (service, service_binding_id, operation, receipt_id),
  UNIQUE (service, service_binding_id, operation, idempotency_key),
  FOREIGN KEY (service_binding_id)
    REFERENCES ocentra_account_identity_issuer_v2_currentness(service_binding_id)
    ON DELETE RESTRICT
) STRICT;

CREATE INDEX idx_ocentra_account_identity_issuer_v2_receipts_binding
  ON ocentra_account_identity_issuer_v2_inbound_receipts
  (service, service_binding_id, issued_at);

CREATE TABLE IF NOT EXISTS ocentra_account_identity_issuer_v2_schema (
  schema_name TEXT NOT NULL PRIMARY KEY CHECK (
    schema_name = 'account_identity_issuer_v2'
  ),
  schema_version INTEGER NOT NULL CHECK (schema_version = 8),
  applied_at TEXT NOT NULL CHECK (julianday(applied_at) IS NOT NULL)
) STRICT;

INSERT INTO ocentra_account_identity_issuer_v2_schema
  (schema_name, schema_version, applied_at)
VALUES
  ('account_identity_issuer_v2', 8, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
ON CONFLICT(schema_name) DO UPDATE SET
  schema_version = excluded.schema_version,
  applied_at = excluded.applied_at;
