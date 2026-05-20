# Cloud Feature Expectations

Cloud features support parent-away-from-home use cases.

## Parent Outcome

A parent away from home can see child-device health, send scoped rule/query/approval intents, and receive audited results without needing to expose the child device directly to the public internet.

## Child-Device Outcome

The child-device agent remains local-first. It owns capture, evidence storage, local AI/policy evaluation, and enforcement. Cloud routing delivers authenticated intents and status, but the child device validates and executes the request locally.

## Platform Scope

- The remote parent portal may live at `family.ocentra.ca`.
- Cloud routing is a control-plane and relay boundary, not a replacement for the local agent.
- Windows is the first child-agent target expected to prove remote health and scoped routing.
- Mobile parent apps may consume the same cloud contracts when those app surfaces exist; mobile child agents must not claim desktop-level behavior until platform capability is proven.

## Data Scope

Cloud may carry device registration, parent identity, child-device identity, heartbeat, capability/status summaries, sync cursors, scoped visibility query requests, rule updates, approval decisions, delivery status, and audit references. Raw evidence, decrypted journals, SQLite files, screen contents, browser contents, and packet payloads stay local unless a later privacy-reviewed contract explicitly permits a narrow export or summary.

## Trust Boundary

Cloud access requires authenticated parent identity and authenticated device identity. Every remote request must be scoped to a family, child device, route, intent type, and request id. The relay must not accept anonymous device commands, development-only bypass tokens, or stale parent sessions. Cloud logs should minimize child activity detail and prefer ids, status, reason codes, and evidence references.

## Contract Boundary

Cloud contracts reuse or extend shared domain packages. Expected contract families include parent account identity, family membership, device registration, device heartbeat, cloud route envelope, remote visibility query, remote rule update, remote approval decision, relay delivery status, sync cursor, conflict outcome, and audit event. Worker/cloud runtime code must consume those contracts instead of inventing parallel JSON payloads.

## Failure Behavior

- Local observation, local policy, local enforcement, and local portal operation continue when cloud is unavailable.
- Cloud outages show explicit stale/offline/queued status to the parent.
- Remote rule updates and approvals are idempotent and auditable; retries cannot apply stale state silently.
- A device receiving an expired, revoked, malformed, wrong-family, or wrong-device command rejects it and records a safe audit event.
- Cloud relay failure does not delete or mutate local evidence.

## Expected Deliverables

- Cloudflare control-plane boundary.
- Authenticated parent identity.
- Authenticated device identity.
- Device heartbeat.
- Rule/query/approval event relay.
- Sync queue.
- Retry/backoff behavior.
- Conflict handling.
- Local-first fallback.
- Family/device authorization model.
- Auditable relay delivery status.
- Sensitive-detail minimization policy for cloud logs.

## Acceptance

- Local operation works when cloud is unavailable.
- Remote rule updates, queries, approvals, and device events are authenticated and auditable.
- Device state cannot be overwritten silently by stale cloud state.
- Cloud logs do not leak sensitive child activity beyond intended product data.
- Cloud behavior reuses shared contracts instead of inventing parallel payloads.
- Remote parent actions are represented as typed intents and executed only by the child-device agent.
- Heartbeat and stale-device states are visible to the parent.
- Conflict outcomes are explicit: accepted, rejected as stale, queued, superseded, or needs parent review.
- Cloud relay does not require API AI availability for child-device safety behavior.

## Validation Gates

- Contract tests for identity, route, heartbeat, relay, conflict, and audit payloads.
- Cloud runtime tests using real route handlers and auth validation boundaries, not unauthenticated happy-path fixtures.
- Child-agent integration tests for accepted remote intent, rejected stale intent, rejected wrong-device intent, queued retry, and local-first fallback.
- Portal coverage for remote health, queued or stale state, and explicit command result.
- Secret scan, dependency policy, and security review for auth, tokens, provider configuration, and logs.

## Non-Goals

- Do not replace local evidence storage with cloud-only storage.
- Do not add paid provider requirements to local development.
- Do not route production family data through unauthenticated dev endpoints.

## Done Signal

A parent can remotely see device health or send a scoped rule, query, or approval intent through authenticated cloud routing while the child-device agent remains local-first and owns execution.
