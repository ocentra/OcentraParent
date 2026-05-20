# Notification Feature Expectations

Notification features should reduce parent anxiety, not create noise.

## Parent Outcome

A parent receives a small number of timely, explainable alerts when attention is needed and can tune frequency, quiet hours, channel preferences, and escalation behavior without losing the audit trail.

## Child-Device Outcome

The child-device agent or trusted backend creates notification intents from typed evidence, policy decisions, health state, or sync/cloud status. Raw observation noise does not become a parent notification until it passes through an explicit alert rule and reason-code contract.

## Platform Scope

- The notification domain should support provider adapters for push, email, SMS, WhatsApp, or in-app delivery without hardcoding one provider into policy logic.
- Web and mobile parent surfaces may display notification history and preferences.
- Child-device agents record notification intents and audit references but do not embed third-party provider details in core policy decisions.

## Data Scope

Notification payloads carry alert id, family/device scope, severity, reason code, evidence reference, policy reference, delivery channel, delivery status, retry state, and parent action link or intent reference. Provider-facing bodies should minimize child activity details and avoid sensitive URLs, titles, message text, filenames, or raw evidence unless a later explicit policy allows it.

## Trust Boundary

Notification delivery crosses third-party provider boundaries. Provider adapters receive only the minimal formatted content needed for that channel. Parent authentication is required before viewing sensitive detail behind an alert. Provider webhooks, tokens, templates, and delivery receipts are security-sensitive.

## Contract Boundary

Expected contracts include alert rule, notification intent, alert reason code, provider channel, delivery attempt, delivery result, retry policy, quiet-hours policy, escalation policy, parent preference, and audit event. Notification contracts reference evidence and policy ids; they do not duplicate raw evidence payloads.

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
- Notification history is auditable.
- Raw unclassified activity does not produce alerts by itself.
- Alert rules can distinguish policy violation, ask-parent request, suspicious unknown, device offline, sync failure, and provider failure.
- Parent preferences and quiet hours affect delivery without deleting the underlying audit event.
- Provider adapters can be replaced or disabled without changing core policy logic.

## Validation Gates

- Contract tests for alert rules, reason codes, delivery status, retry state, quiet hours, and preferences.
- Adapter boundary tests for success, retryable failure, permanent failure, webhook receipt, and disabled provider.
- Integration tests proving notification intents reference stored evidence or policy decisions.
- Parent-surface coverage for notification history, preference changes, quiet hours, and sensitive-detail drill-in behind authentication.
- Secret scan and provider credential review before any real provider is enabled.

## Non-Goals

- Do not send alerts from raw unclassified noise.
- Do not hardcode one provider into core policy logic.
- Do not expose sensitive evidence in third-party notification previews unless explicitly approved.

## Done Signal

The system can create, deliver, and audit one notification type through a provider boundary with clear parent controls and safe failure behavior.
