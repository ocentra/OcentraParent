# Notification Feature Expectations

Notification features should reduce parent anxiety, not create noise. They also
cross provider boundaries, so notification content must be minimal by default
and must not turn WhatsApp, push, email, SMS, or Ocentra-hosted routing into a
child-activity data store.

## Parent Outcome

A parent receives a small number of timely, explainable alerts when attention is needed and can tune frequency, quiet hours, channel preferences, and escalation behavior without losing the audit trail.

## Child-Device Outcome

The child-device agent or trusted backend creates notification intents from typed
evidence references, policy decisions, health state, or sync/cloud status. Raw
observation noise does not become a parent notification until it passes through
an explicit alert rule and reason-code contract.

## Platform Scope

- The notification domain should support provider adapters for push, email, SMS, WhatsApp, or in-app delivery without hardcoding one provider into policy logic.
- Web and mobile parent surfaces may display notification history and preferences.
- Child-device agents record notification intents and audit references but do not embed third-party provider details in core policy decisions.
- Ocentra-hosted notification routing may carry delivery metadata and minimal
  alert bodies only. Sensitive detail belongs behind authenticated parent app,
  local/LAN, or parent-owned storage access.

## Data Scope

Notification payloads carry alert id, family/device scope, severity, reason
code, evidence reference, policy reference, delivery channel, delivery status,
retry state, and parent action link or intent reference. Provider-facing bodies
should minimize child activity details and avoid sensitive URLs, titles, message
text, filenames, raw evidence, screenshots, or generated reports unless a later
explicit policy allows it.

## Trust Boundary

Notification delivery crosses third-party provider boundaries. Provider adapters
receive only the minimal formatted content needed for that channel. Parent
authentication is required before viewing sensitive detail behind an alert.
Provider webhooks, tokens, templates, and delivery receipts are
security-sensitive. Ocentra-hosted routing should retain only delivery state and
short operational logs, not child evidence.

## Contract Boundary

Expected contracts include alert rule, notification intent, alert reason code, provider channel, delivery attempt, delivery result, retry policy, quiet-hours policy, escalation policy, parent preference, and audit event. Notification contracts reference evidence and policy ids; they do not duplicate raw evidence payloads.

Current contract proof:
`scripts/test/v0-8-notification-provider-status-boundary.mjs` validates a V3
notification rule/provider retry read model covering alert rules, reason codes,
provider channels, delivery attempt/result states, retry policy states,
quiet-hours decisions, escalation decisions, parent preference states, audit
refs, and evidence refs. This is a contract proof only: it does not claim a
provider adapter, real send/retry execution, observed provider receipts, raw
evidence in provider payloads, provider child-evidence storage, parent
preference UI, or notification history UI.

`scripts/test/notification-audit-history-contract-proof.mjs` validates a
logging-domain notification audit/history read model covering provider status,
retry lifecycle, receipt/manual-required refs, quiet-hours/escalation refs,
redaction-safe payload fields, and child-data non-custody. This is a logging
contract proof only: it does not claim provider adapters, send/retry execution,
webhook receipt ingestion, notification history UI, credentials, raw child data,
or Ocentra-hosted child evidence custody.

`scripts/test/notification-local-outbox-adapter-proof.mjs` validates a
parent-domain local outbox adapter-boundary proof that writes and rereads a
deterministic parent-owned JSONL outbox artifact. It covers minimal alert
envelopes, provider-channel abstraction, quiet-hours defer, retry,
dead-letter, receipt-required, manual-required, audit/evidence/policy refs, and
sensitive-detail minimization. This is a local outbox proof only: it does not
claim provider delivery, webhook receipt ingestion, provider credentials, cloud
routing, parent notification UI, raw child evidence, raw URLs/titles/message
text, screenshots, reports, or sensitive provider metadata storage.

`scripts/test/notification-local-outbox-scheduler-proof.mjs` validates a
parent-domain local outbox scheduler proof that writes and rereads a
deterministic parent-owned JSONL scheduler artifact. It covers due, held
quiet-hours, retry-window scheduled, dead-letter review, receipt-required, and
manual-required scheduler states, deterministic `nextAttemptAt` and retry
window rows, provider-channel abstraction, parent-owned artifact refs, and
sensitive-detail minimization. This is a scheduler proof only: it does not
claim provider delivery, retry worker execution, quiet-hours timer execution,
webhook receipt ingestion, provider credentials, cloud routing, parent
notification UI, durable production outbox storage, raw child evidence, raw
URLs/titles/message text, screenshots, reports, or sensitive provider metadata
storage.

`scripts/test/app-game-notification-local-outbox-bridge-proof.mjs` validates a
parent-domain app/game local outbox bridge proof that writes and rereads
deterministic parent-owned JSONL records from validated app/game notification
intents. It covers eligible local-outbox-only intents, minimal alert envelopes,
app/game evidence and policy refs, manual-required and unavailable rows that do
not queue delivery, and explicit no-provider/no-scheduler/no-UI/no-child/
no-adapter claims. This is a bridge proof only: it does not claim durable
production outbox storage, provider delivery, receipt ingestion, quiet-hours or
retry worker execution, cloud routing, parent notification UI, child delivery,
broad blocking, or platform support.

`scripts/test/app-game-notification-scheduler-bridge-proof.mjs` validates a
parent-domain app/game scheduler bridge proof that writes and rereads
deterministic scheduler JSONL rows from linked app/game local outbox records. It
covers due-local scheduler rows, source local-outbox refs, app/game evidence and
policy refs, manual-required and unavailable rows that remain unscheduled, and
explicit no-runtime/no-provider/no-UI/no-child/no-adapter claims. This is a
bridge proof only: it does not claim production retry workers, quiet-hours timer
execution, durable production outbox storage, provider delivery, receipt
ingestion, cloud routing, parent notification UI, child delivery, broad
blocking, or platform support.

## Failure Behavior

- Provider failure is visible, retryable when safe, and auditable.
- Quiet hours suppress or defer non-critical alerts according to parent preference.
- Duplicate observations collapse into a noise-controlled notification window where appropriate.
- If sensitive detail cannot be delivered safely, send a minimal alert that asks the parent to open the authenticated portal.
- Notification failure does not block local child-device safety decisions.

## Expected Deliverables

- Notification contract.
- Alert reason codes.
- Provider adapter boundary.
- Delivery status.
- Retry/failure handling.
- Quiet hours.
- Parent preference controls.
- Notification audit history.
- Noise control and deduplication policy.
- Sensitive-detail minimization templates.

## Acceptance

- Notifications reference evidence and policy reason.
- Provider failure is visible and retryable.
- Parents can tune frequency.
- Sensitive details are minimized in push, WhatsApp, email, or SMS bodies.
- Ocentra-hosted notification routing does not retain child activity detail by
  default.
- Notification history is auditable.
- Raw unclassified activity does not produce alerts by itself.
- Alert rules can distinguish policy violation, parent request, suspicious unknown, device offline, sync failure, and provider failure.
- Parent preferences and quiet hours affect delivery without deleting the underlying audit event.
- Provider adapters can be replaced or disabled without changing core policy logic.

## Validation Gates

- Contract tests for alert rules, reason codes, delivery status, retry state, quiet hours, and preferences.
- V3 notification rule/provider retry contract proof for reason codes, provider
  channels, delivery result states, retry policies, quiet-hours decisions,
  escalation decisions, parent preferences, audit refs, and evidence refs.
- Logging-domain notification audit/history contract proof for provider status,
  retry lifecycle, receipt/manual-required refs, quiet-hours/escalation refs,
  redaction-safe payload fields, and child-data non-custody.
- Parent-domain notification local outbox adapter-boundary proof for
  deterministic local outbox artifact writing/parsing, minimal alert envelopes,
  quiet-hours defer, retry, dead-letter, receipt-required, manual-required, and
  sensitive-detail minimization without provider delivery claims.
- Parent-domain notification local outbox scheduler proof for deterministic
  due/held quiet-hours/retry-window/dead-letter/receipt/manual scheduler states,
  parent-owned scheduler artifact writing/parsing, deterministic next-at/retry
  window behavior, and sensitive-detail minimization without provider delivery
  claims.
- Parent-domain app/game notification local outbox bridge proof for validated
  app/game notification intents becoming deterministic parent-owned JSONL
  records only when local-outbox eligible, with manual/unavailable rows kept
  unqueued and no provider/scheduler/UI/child/adapter claims.
- Parent-domain app/game notification scheduler bridge proof for linked
  app/game local outbox records becoming deterministic scheduler JSONL rows,
  with manual/unavailable rows kept unscheduled and no production runtime,
  provider/UI/child/adapter claims.
- Adapter boundary tests for success, retryable failure, permanent failure, webhook receipt, and disabled provider.
- Integration tests proving notification intents reference stored evidence or policy decisions.
- Parent-surface coverage for notification history, preference changes, quiet hours, and sensitive-detail drill-in behind authentication.
- Secret scan and provider credential review before any real provider is enabled.

## Non-Goals

- Do not send alerts from raw unclassified noise.
- Do not hardcode one provider into core policy logic.
- Do not expose sensitive evidence in third-party notification previews unless explicitly approved.
- Do not store generated child reports or raw evidence in notification provider
  metadata.

## Done Signal

The system can create, deliver, and audit one notification type through a provider boundary with clear parent controls and safe failure behavior.
