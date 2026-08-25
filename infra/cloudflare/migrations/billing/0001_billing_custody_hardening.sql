CREATE TABLE IF NOT EXISTS billing_provider_event_receipts (
  provider TEXT NOT NULL,
  event_id TEXT NOT NULL,
  event_fingerprint TEXT NOT NULL CHECK (
    length(event_fingerprint) = 86
    AND event_fingerprint GLOB 'provider-event:sha256:*'
    AND substr(event_fingerprint, 23) NOT GLOB '*[^0-9a-f]*'
  ),
  event_type TEXT NOT NULL,
  provider_occurred_at TEXT,
  provider_sequence INTEGER CHECK (
    provider_sequence IS NULL OR provider_sequence BETWEEN 0 AND 4294967295
  ),
  state_version INTEGER NOT NULL DEFAULT 0 CHECK (state_version BETWEEN 0 AND 4294967295),
  account_id TEXT,
  provider_customer_id TEXT,
  provider_subscription_id TEXT,
  provider_invoice_id TEXT,
  billing_subject TEXT,
  parent_account_ref TEXT,
  family_ref TEXT,
  billing_invoice_id TEXT,
  processing_state TEXT NOT NULL CHECK (
    processing_state IN ('received', 'ignored', 'queued', 'applied', 'manual-required', 'dead-letter')
  ),
  queue_state TEXT NOT NULL CHECK (
    queue_state IN ('pending', 'queued', 'delivered', 'manual-required', 'dead-letter')
  ),
  queue_attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (queue_attempt_count BETWEEN 0 AND 4294967295),
  last_queue_attempt_at TEXT,
  last_error TEXT CHECK (
    last_error IS NULL
    OR last_error IN (
      'billing-control-do-binding-missing',
      'billing-queue-operation-failed',
      'billing-read-model-unavailable',
      'provider-event-authority-revoked-or-changed',
      'provider-event-authority-unresolved',
      'provider-event-legacy-error-quarantined',
      'provider-event-order-metadata-missing',
      'provider-event-queue-delivery-failed'
    )
  ),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  PRIMARY KEY (provider, event_id)
);

CREATE TABLE IF NOT EXISTS billing_custody_migrations (
  migration_id TEXT PRIMARY KEY,
  applied_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS billing_provider_event_quarantine (
  provider TEXT NOT NULL,
  event_id TEXT NOT NULL,
  reason TEXT NOT NULL CHECK (reason = 'legacy-fingerprint-invalid'),
  quarantined_at TEXT NOT NULL,
  PRIMARY KEY (provider, event_id)
);

CREATE TABLE IF NOT EXISTS billing_mutation_outbox (
  request_key TEXT PRIMARY KEY,
  authority_subject TEXT,
  authority_version INTEGER CHECK (authority_version IS NULL OR authority_version >= 1),
  authority_token TEXT,
  mutation_kind TEXT NOT NULL,
  mutation_json TEXT NOT NULL,
  audit_state TEXT NOT NULL CHECK (audit_state IN ('pending', 'delivered')),
  audit_event_json TEXT NOT NULL,
  attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
  last_attempt_at TEXT,
  last_error TEXT CHECK (
    last_error IS NULL
    OR last_error IN (
      'billing-audit-event-invalid',
      'billing-audit-owner-unavailable',
      'billing-mutation-outbox-delivery-failed',
      'billing-mutation-outbox-legacy-error-quarantined',
      'billing-read-model-unavailable'
    )
  ),
  lease_token TEXT,
  lease_expires_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

INSERT OR IGNORE INTO billing_provider_event_quarantine (
  provider,
  event_id,
  reason,
  quarantined_at
)
SELECT
  provider,
  event_id,
  'legacy-fingerprint-invalid',
  updated_at
FROM billing_provider_event_receipts
WHERE
  length(event_fingerprint) <> 86
  OR event_fingerprint NOT GLOB 'provider-event:sha256:*'
  OR substr(event_fingerprint, 23) GLOB '*[^0-9a-f]*';

DELETE FROM billing_provider_event_receipts
WHERE
  length(event_fingerprint) <> 86
  OR event_fingerprint NOT GLOB 'provider-event:sha256:*'
  OR substr(event_fingerprint, 23) GLOB '*[^0-9a-f]*';

UPDATE billing_provider_event_receipts
SET last_error = 'provider-event-legacy-error-quarantined'
WHERE
  last_error IS NOT NULL
  AND last_error NOT IN (
    'billing-control-do-binding-missing',
    'billing-queue-operation-failed',
    'billing-read-model-unavailable',
    'provider-event-authority-revoked-or-changed',
    'provider-event-authority-unresolved',
    'provider-event-legacy-error-quarantined',
    'provider-event-order-metadata-missing',
    'provider-event-queue-delivery-failed'
  );

UPDATE billing_mutation_outbox
SET last_error = 'billing-mutation-outbox-legacy-error-quarantined'
WHERE
  last_error IS NOT NULL
  AND last_error NOT IN (
    'billing-audit-event-invalid',
    'billing-audit-owner-unavailable',
    'billing-mutation-outbox-delivery-failed',
    'billing-mutation-outbox-legacy-error-quarantined',
    'billing-read-model-unavailable'
  );

CREATE TRIGGER IF NOT EXISTS billing_provider_receipt_custody_insert_guard
BEFORE INSERT ON billing_provider_event_receipts
BEGIN
  SELECT RAISE(ABORT, 'billing-provider-event-custody-invalid')
  WHERE
    length(NEW.event_fingerprint) <> 86
    OR NEW.event_fingerprint NOT GLOB 'provider-event:sha256:*'
    OR substr(NEW.event_fingerprint, 23) GLOB '*[^0-9a-f]*'
    OR (
      NEW.last_error IS NOT NULL
      AND NEW.last_error NOT IN (
        'billing-control-do-binding-missing',
        'billing-queue-operation-failed',
        'billing-read-model-unavailable',
        'provider-event-authority-revoked-or-changed',
        'provider-event-authority-unresolved',
        'provider-event-legacy-error-quarantined',
        'provider-event-order-metadata-missing',
        'provider-event-queue-delivery-failed'
      )
    );
END;

CREATE TRIGGER IF NOT EXISTS billing_provider_receipt_custody_update_guard
BEFORE UPDATE OF event_fingerprint, last_error ON billing_provider_event_receipts
BEGIN
  SELECT RAISE(ABORT, 'billing-provider-event-custody-invalid')
  WHERE
    length(NEW.event_fingerprint) <> 86
    OR NEW.event_fingerprint NOT GLOB 'provider-event:sha256:*'
    OR substr(NEW.event_fingerprint, 23) GLOB '*[^0-9a-f]*'
    OR (
      NEW.last_error IS NOT NULL
      AND NEW.last_error NOT IN (
        'billing-control-do-binding-missing',
        'billing-queue-operation-failed',
        'billing-read-model-unavailable',
        'provider-event-authority-revoked-or-changed',
        'provider-event-authority-unresolved',
        'provider-event-legacy-error-quarantined',
        'provider-event-order-metadata-missing',
        'provider-event-queue-delivery-failed'
      )
    );
END;

CREATE TRIGGER IF NOT EXISTS billing_mutation_outbox_custody_insert_guard
BEFORE INSERT ON billing_mutation_outbox
BEGIN
  SELECT RAISE(ABORT, 'billing-mutation-outbox-error-code-invalid')
  WHERE
    NEW.last_error IS NOT NULL
    AND NEW.last_error NOT IN (
      'billing-audit-event-invalid',
      'billing-audit-owner-unavailable',
      'billing-mutation-outbox-delivery-failed',
      'billing-mutation-outbox-legacy-error-quarantined',
      'billing-read-model-unavailable'
    );
END;

CREATE TRIGGER IF NOT EXISTS billing_mutation_outbox_custody_update_guard
BEFORE UPDATE OF last_error ON billing_mutation_outbox
BEGIN
  SELECT RAISE(ABORT, 'billing-mutation-outbox-error-code-invalid')
  WHERE
    NEW.last_error IS NOT NULL
    AND NEW.last_error NOT IN (
      'billing-audit-event-invalid',
      'billing-audit-owner-unavailable',
      'billing-mutation-outbox-delivery-failed',
      'billing-mutation-outbox-legacy-error-quarantined',
      'billing-read-model-unavailable'
    );
END;

INSERT OR IGNORE INTO billing_custody_migrations (migration_id, applied_at)
VALUES ('provider-event-custody-v1', strftime('%Y-%m-%dT%H:%M:%fZ', 'now'));

INSERT OR IGNORE INTO billing_custody_migrations (migration_id, applied_at)
VALUES ('mutation-outbox-error-codes-v1', strftime('%Y-%m-%dT%H:%M:%fZ', 'now'));
