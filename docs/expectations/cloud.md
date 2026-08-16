<!-- agent-capsule -->

> Agent Capsule
> Doc: Cloud Feature Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Cloud Feature Expectations

Cloud features support parent-away-from-home use cases without turning Ocentra
into the custody layer for child activity data.

Ocentra-hosted cloud is a control plane, distribution surface, subscription/auth
boundary, notification route, and optional stateless report compiler. It is not
the default storage system for raw evidence, journals, screenshots, browser
history, generated reports, or parent rules.

## Parent Outcome

A parent away from home can receive minimal alerts, authenticate, check account
or subscription state, download/install Ocentra Parent, and optionally compile or
view reports from parent-owned storage. The parent should always know whether a
view came from local cache, LAN/live child agent, parent-owned cloud storage, or
an Ocentra-hosted stateless compile request.

## Child-Device Outcome

The child-device agent remains local-first. It owns capture, evidence storage,
local AI/policy evaluation, timers, and enforcement. Cloud routing may deliver
authenticated typed intents or status envelopes, but the child device validates
and executes requests locally. Cloud unavailability must not stop already
configured local safety behavior.

## Platform Scope

- `family.ocentra.ca` is the public/download/account/subscription surface first.
- The production parent portal should be a local packaged app or mobile app
  surface for household control. Tauri is the preferred desktop-shell candidate
  unless a later architecture decision replaces it.
- A browser-hosted Ocentra page may support login, download, billing, docs,
  device/account status, notification drill-in, and parent-authorized stateless
  report compilation, but it must not become the source-of-truth child portal.
- Cloud routing is a control-plane and relay boundary, not a replacement for the
  local agent or parent-owned storage.
- Windows is the first child-agent target expected to prove remote health and
  scoped routing.
- Mobile parent apps may consume the same cloud and storage contracts when those
  app surfaces exist.

## Data Scope

Ocentra-hosted cloud may carry:

- Parent identity, account, subscription, entitlement, license, and billing
  references.
- Device registration metadata, pairing route metadata, heartbeat status,
  capability/status summaries, and relay delivery status.
- Minimal notification metadata: alert id, severity, reason code, delivery
  channel, delivery state, and parent action link.
- Parent-authorized connector metadata for external storage, such as provider
  name, connected account hint, folder id/reference, sync cursor, and last
  success/failure status.
- Short-lived report-compile request ids, job status, and operational telemetry
  that avoids child activity detail.

Ocentra-hosted cloud must not store by default:

- Raw encrypted journal segments.
- SQLite evidence databases.
- Screenshots, screen-analysis images, or raw visual evidence.
- Browser URL history, page titles, page contents, chat contents, keystrokes, or
  decrypted network payloads.
- Long-term child activity reports or generated summaries.
- Parent rules and approval history as the source of truth.
- Parent-owned storage contents or long-lived provider tokens unless a future
  explicit encrypted token-vault feature is designed, reviewed, and linked here.

## Trust Boundary

Cloud access requires authenticated parent identity and authenticated device or
storage-source identity. Every remote request must be scoped to a family, child
device or storage source, route, intent type, and request id. The relay must not
accept anonymous device commands, development-only bypass tokens, stale parent
sessions, or wrong-family route ids.

Ocentra-hosted report compilation, if implemented, must be stateless by default:

- Parent grants access to a parent-owned source such as Google Drive, OneDrive,
  iCloud Drive, Dropbox, NAS gateway, or a local upload.
- Compiler reads only the requested data class and time window.
- Compiler returns a report to the parent session or writes it back to the
  parent-owned destination.
- Compiler deletes temporary input/output and records only minimal operational
  status with a short TTL.

## Contract Boundary

Cloud contracts reuse or extend shared domain packages. Expected contract
families include parent account identity, entitlement snapshot, device
registration, device heartbeat, cloud route envelope, remote visibility query,
remote rule update, remote approval decision, relay delivery status,
parent-owned storage connector, report compile request/result, sync cursor,
conflict outcome, and audit event.

Worker/cloud runtime code must consume those contracts instead of inventing
parallel JSON payloads.

Current endpoint proof: `sync-export-endpoint-contract-proof` defines
endpoint-domain route ids, API paths, headers, query params, and
contract-version labels for parent-owned sync/export and remote connector
status boundaries. The proof keeps cloud as a status/control-plane boundary
only; it does not implement connector OAuth, upload/download, Ocentra-hosted
family data custody, account/subscription backend, or portal UI.

Current account endpoint proof: `billing-account-endpoint-contract-proof`
defines endpoint-domain route ids, API paths, headers, query params, and
contract-version labels for account status, plan/entitlement snapshot,
subscription status, device-limit decision, and account download/update/status
surfaces. The proof keeps cloud as an account/distribution status boundary only;
it does not implement Stripe SDK code, billing provider logic, an account
backend, portal UI, updater runtime, or child-activity custody.

Current public release/status proof: `production-release-public-status-proof`
defines parent-domain readiness rows for `family.ocentra.ca` public download,
release status, update status, account status, subscription status, and support
status surfaces. The proof keeps the public website runtime, account backend,
billing provider runtime, production publishing, signing/store proof, updater
execution, support backend upload, and child-activity custody unimplemented or
manual-required.

Current public runtime handoff proof:
`production-release-public-runtime-handoff-proof` defines parent-domain
route/status and backend adapter handoff rows for public download, release
status, update status, account status, subscription status, and support status.
The proof keeps cloud as an account/distribution/status handoff boundary only:
public runtime, account backend, billing provider runtime, production
publishing, signing/store proof, updater execution, support backend upload,
real device/store proof, and child-activity custody remain unimplemented or
manual-required.

Current stateless report compiler proof:
`stateless-report-compiler-status-proof` defines parent-domain request/status/
result contracts for parent-authorized report compilation from parent-owned
storage. It keeps cloud as a short-lived status/control-plane boundary with
source connector/cursor refs, requested data classes/time window, parent-owned
output destination, temp input/output TTL and deletion confirmation,
redaction/minimization flags, audit refs, and non-mutating failure behavior. It
does not implement a cloud worker, report compiler runtime, connector OAuth/
provider API, upload/download runtime, portal UI, retained temporary child
evidence, child-device mutation, or Ocentra-hosted family-data custody.

## Failure Behavior

- Local observation, local policy, local enforcement, local portal operation, and
  local parent cache continue when cloud is unavailable.
- Cloud outages show explicit stale/offline/queued status to the parent.
- Remote rule updates and approvals are idempotent and auditable; retries cannot
  apply stale state silently.
- A device receiving an expired, revoked, malformed, wrong-family, or
  wrong-device command rejects it and records a safe audit event.
- Parent-owned storage connector failures show provider/folder/status errors
  without deleting local evidence.
- Report compiler failure does not mutate source evidence, parent-owned storage,
  or local child-device data.

## Expected Deliverables

- Cloudflare control-plane boundary.
- Authenticated parent identity.
- Authenticated device identity.
- Device heartbeat and route status.
- Rule/query/approval event relay.
- Parent-owned storage connector contracts.
- Stateless report compiler contracts where remote compilation exists.
- Retry/backoff behavior.
- Conflict handling.
- Local-first fallback.
- Family/device authorization model.
- Auditable relay and compiler status.
- Sensitive-detail minimization policy for cloud logs.

## Acceptance

- Local operation works when cloud is unavailable.
- Remote rule updates, queries, approvals, and device events are authenticated
  and auditable.
- Device state cannot be overwritten silently by stale cloud state.
- Ocentra-hosted databases do not store child activity evidence or reports by
  default.
- Parent-owned storage connectors are explicit and parent-visible.
- Cloud logs do not leak sensitive child activity beyond minimal operational
  metadata.
- Cloud behavior reuses shared contracts instead of inventing parallel payloads.
- Remote parent actions are represented as typed intents and executed only by the
  child-device agent.
- Heartbeat and stale-device states are visible to the parent.
- Conflict outcomes are explicit: accepted, rejected as stale, queued,
  superseded, or needs parent review.
- Cloud relay does not require remote/API AI availability for child-device safety
  behavior.

## Validation Gates

- Contract tests for identity, route, heartbeat, relay, connector, compiler,
  conflict, and audit payloads.
- Endpoint-domain contract tests and `sync-export-endpoint-contract-proof` for
  parent-owned sync/export and remote connector status route boundaries.
- Endpoint-domain contract tests and `billing-account-endpoint-contract-proof`
  for account, entitlement, subscription, device-limit, download, update, and
  release-status route boundaries.
- Parent-domain contract tests and `production-release-public-status-proof` for
  public download, release/update status, account/subscription status, and
  support status readiness rows before public runtime or backend code exists.
- Parent-domain contract tests and
  `production-release-public-runtime-handoff-proof` for public route/status and
  backend adapter handoff rows before public website runtime, account backend,
  billing provider runtime, updater execution, support upload, or production
  publishing exists.
- Parent-domain contract tests and `stateless-report-compiler-status-proof`
  for parent-authorized compiler request scope, status/result states,
  temporary TTL/deletion confirmation, redaction/minimization, audit refs, and
  non-mutating failure behavior.
- Cloud runtime tests using real route handlers and auth validation boundaries,
  not unauthenticated happy-path fixtures.
- Child-agent integration tests for accepted remote intent, rejected stale
  intent, rejected wrong-device intent, queued retry, and local-first fallback.
- Parent-owned storage connector tests for least-privilege scope, expired grant,
  revoked grant, wrong folder, malformed export, and retry behavior.
- Portal/app coverage for remote health, queued or stale state, connector status,
  report compile status, and explicit command result.
- Secret scan, dependency policy, and security review for auth, tokens, provider
  configuration, billing, and logs.

## Non-Goals

- Do not replace local evidence storage with Ocentra cloud storage.
- Do not use Cloudflare KV, D1, R2, queues, analytics, or logs as the default
  family-data warehouse.
- Do not store raw child activity evidence, generated reports, screenshots, or
  parent rules in Ocentra-hosted infrastructure by default.
- Do not add paid provider requirements to local development.
- Do not route production family data through unauthenticated dev endpoints.
- Do not retain parent-owned storage tokens or source data beyond the explicit
  connector/compile contract.

## Done Signal

A parent can remotely authenticate, receive minimal status or alerts, use
parent-owned storage or a reachable child agent for family data, and send scoped
rule/query/approval intents while the child-device agent remains local-first and
Ocentra-hosted infrastructure does not become the child-activity store.
