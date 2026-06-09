# Tracking And Eventing Real Test Matrix

This matrix applies to WP34-WP39 and any future tracking runtime work. It is a
test plan, not proof by itself.

The rule is:

```text
CODE -> TESTS -> RUN -> FIX -> PROOF -> DOCS
```

Tests prove real source behavior. Proof artifacts are receipts after real code
and tests pass.

## Non-Negotiable Test Rules

Do not count any of these as implementation tests:

- fake services;
- fake event bus;
- fake portal state;
- fake child-agent state;
- fake AI policy authority;
- fake provider delivery success;
- fake platform capability;
- proof-only tests that only assert generated proof JSON or non-claim rows.

Allowed test substitutes:

- temporary SQLite database;
- temporary NDJSON journal file;
- real local Rust service started by the test;
- real Tokio event handlers;
- real Playwright browser;
- real Android emulator for emulator-tier proof only;
- real Android physical device for manual/local physical proof;
- real iOS simulator on macOS CI for simulator-tier proof only;
- real iOS physical device for manual/local physical proof;
- provider dry-run only when labelled dry-run/manual-required, not delivery
  success.

## Required Test Categories

Every event-driven tracking workpack must say which categories apply, which
commands ran, and which higher-tier/manual tests remain.

1. Contract tests.
2. Unit tests.
3. Integration tests.
4. End-to-end tests.
5. Playwright UI tests.
6. Property/invariant tests.
7. Differential tests.
8. Mutation tests for high-risk gates.
9. Fuzzing tests for schema/API/replay.
10. Security tests.
11. AuthN/AuthZ matrix tests.
12. Replay/idempotency/order tests.
13. Load/spike/soak/resource tests.
14. Migration/rollback/version-skew tests.
15. Chaos/fault-injection tests.
16. Clock/DST/expiry tests.
17. AI boundary contract tests.
18. Logging/metrics/tracing/alert tests.
19. Human misuse/rate-limit tests.
20. Platform tests for Windows, Linux/WSL, Docker, Android, and iOS as
    applicable.

## Contract Tests

Targets:

```text
crates/agent-protocol
packages/activity-domain
packages/parent-domain
```

Required coverage:

- event type uniqueness;
- no raw runtime string invention;
- namespace ownership;
- schema/version binding;
- Effect Schema and Rust serde roundtrip;
- unknown event type rejection;
- duplicate event type registry failure;
- aggregate-key policy;
- journal policy.

Required event families:

```text
tracking.*
location.*
geofence.*
expected_place.*
nearby_place.*
notification.*
escalation.*
audit.*
portal.*
policy.*
ai.*
child_agent.*
parent_controller.*
```

Payload contract tests must reject:

- AI event with policy, enforcement, notification, live-mode, or escalation
  command fields;
- notification event without policy decision ref;
- live tracking event without TTL;
- nearby-place event without ambiguity or uncertainty;
- critical alert from one weak sample;
- precise location inferred from LAN/IP/pairing.

## Event Envelope Tests

Targets:

```text
crates/ocentra-eventing
crates/agent-protocol
```

Required coverage:

- event id is required;
- correlation id is required;
- causation id is required for derived events;
- aggregate key is required for ordered events;
- idempotency key is required for command events;
- source, custody, priority, and deadline are present where required;
- live dispatch rejects `serde_json::Value` payload shortcuts;
- stored envelope and live envelope boundaries are explicit.

## Rust Crate And Test Organization

- `crates/agent-protocol/tests`: public Rust-crossing tracking constants,
  payload structs, serde/roundtrip, unknown/duplicate event rejection, and
  TypeScript/Rust protocol parity where applicable.
- `crates/agent-core/tests`: public tracking runtime APIs, local durable state,
  ActivityStore projections, journal/replay/idempotency, retention/custody, and
  policy/evidence boundary behavior.
- `crates/agent-service/tests`: real local API/WebSocket transport,
  authorization, publisher role checks, service read-model responses, and
  transport error/manual-required states after service seams are importable.
- Private `#[cfg(test)]` module tests are allowed only for internal helper
  invariants, private binary-service transport seams, or behavior that is not
  exposed through a public crate API.

## Eventing Crate Unit Tests

Target:

```text
crates/ocentra-eventing
```

Required coverage:

- event type grammar valid/invalid;
- event id, correlation id, causation id, aggregate key, idempotency key,
  event source, runtime role, and event custody newtypes;
- live envelope and stored-envelope roundtrip;
- sequential dispatch order;
- concurrent dispatch with real Tokio handlers;
- aggregate-ordered dispatch for same-key ordering and different-key
  parallelism;
- nested publish without deadlock;
- no lock held across handler await;
- handler cannot mutate event payload;
- registrar dispose and wrong-target handler reports;
- no-subscriber bounded queue;
- queue overflow reporting;
- TTL expiry before dispatch;
- retry count and retry-limit dead letter;
- handler timeout and panic isolation;
- duplicate idempotency key rejection;
- request/response associated response type, timeout, late response, and
  double-completion rejection;
- NDJSON journal append/flush and replay projection-only behavior.

## Tracking Logic Unit Tests

Targets:

```text
packages/parent-domain
packages/activity-domain
crates/agent-core
```

Required coverage:

- observe-only policy;
- notify parent policy;
- ask child policy;
- ask parent acknowledgement policy;
- start temporary live tracking policy;
- escalation policy;
- critical alert policy;
- suppress by exception;
- critical not suppressed unless explicitly configured;
- AI-only cannot alert;
- ambiguous nearby place cannot accuse;
- low accuracy cannot create critical alert;
- stale/offline creates degraded state only;
- missing capability creates manual-required state.

Retention tests must cover last-known-only, 24h, 7d, 30d, custom retention,
delete-on-resolution, parent-owned export, remote-sync-disabled default,
remote-AI-disabled default, tombstone hiding, and replay preserving delete.

Live tracking tests must cover policy/parent-command requirement, TTL, reason,
child-visible state, stop condition, expiry, duplicate start idempotency, and
child-agent-unavailable manual-required state.

## Integration Tests

Use real local service, real temp SQLite, real temp NDJSON files, and real
event bus.

Required service/event bus tests:

- service starts with event bus;
- service registers tracking handlers;
- service publishes parent intent event;
- service publishes tracking config event;
- service projects read model after event;
- service journals before action;
- service exposes dead-letter/manual-required state after handler failure;
- service replay rebuilds read model;
- service replay does not re-execute child command.

Required parent config chain tests:

- enable last-known mode;
- disable tracking;
- enable arrival alerts;
- enable temporary live mode;
- update retention window;
- update delete-after-alert;
- reject invalid config before publish;
- dedupe duplicate idempotency key;
- child-agent unavailable produces manual-required state;
- journal-before-apply;
- read model comes from service state.

Required location evidence chain tests:

- expected-place sample observes only;
- geofence enter;
- geofence exit;
- geofence dwell;
- unexpected place triggers policy;
- nearby-place ambiguous state;
- low accuracy produces no alert;
- stale/offline produces degraded state;
- duplicate sample is idempotent.

AI is an external consumer/dependency boundary for tracking. Tracking does not
own AI provider selection, provider mesh, work lease/claim internals, model
quality, prompt tuning, prompt-injection model behavior, temperature behavior,
or summarizer accuracy beyond the contract boundary. Those tests belong to the
AI lane.

Tracking must prove only the AI handoff/result contract:

- tracking-does-not-call-ai-directly-without-event;
- tracking-publishes-ai-request-only-when-policy-or-detection-requires-analysis;
- ai-request-cites-location-geofence-expected-place-nearby-place-evidence-refs;
- ai-request-excludes-raw-private-location-history-bulk;
- ai-request-has-purpose-scope-and-retention-state;
- ai-result-contract-valid-accepted-as-evidence;
- ai-result-missing-evidence-ref-rejected;
- ai-result-hallucinated-evidence-ref-rejected;
- ai-result-wrong-child-or-device-ref-rejected;
- ai-result-stale-correlation-rejected;
- ai-result-cannot-start-live-mode;
- ai-result-cannot-create-notification;
- ai-result-cannot-create-escalation;
- ai-result-cannot-authorize-policy;
- ai-result-cannot-execute-enforcement;
- ai-unavailable-produces-manual-review-or-degraded-state;
- ai-timeout-produces-manual-review-or-degraded-state.

Accepted AI result events feed policy as evidence only. They must never create
policy authority, live tracking, notification, escalation, enforcement, or audit
authority directly.

Notification provider integration must not claim delivery unless real
credentials/provider/device proof exists. Provider unavailable, disabled, and
dry-run states must remain manual-required/no-delivery.

## End-To-End And Platform Tests

Windows local E2E uses:

```text
real Rust service
real Vite/React portal
real temp SQLite
real NDJSON journal
real Playwright Chromium
real filesystem
```

Required Windows flows:

- parent config tracking mode;
- location fixture to portal read model;
- unexpected place with manual-required provider state;
- live-mode start with TTL and audit;
- retention delete hides history;
- service restart replay restores read model.

WSL/Linux flows:

- eventing cargo tests;
- service read-model replay;
- NDJSON journal permissions;
- clock-skew replay boundary;
- concurrent event dispatch.

Docker flows, when Docker is available:

- clean service start;
- schema migration;
- rollback previous schema;
- network-disabled provider unavailable;
- resource limits.

If Docker is unavailable, output must be `docker-unavailable manual_required`,
not green.

Android emulator flows are emulator-tier only:

- package install;
- foreground location permission;
- background permission state;
- location sample to child-agent event;
- local geofence transition;
- battery/connectivity status;
- child-agent config applied;
- live-mode command applied;
- denied permission degraded state.

Android physical device flows are manual/local physical proof:

- install/launch;
- foreground service running;
- foreground and background location sample delivery;
- physical geofence enter/exit/dwell;
- battery optimization state;
- permission denied state;
- network offline last-known state;
- live-mode TTL stop;
- child disclosure visible.

iOS simulator flows are simulator/package proof only on macOS CI:

- build/install/launch;
- permission screen visible;
- child disclosure visible;
- config screen state;
- no background region claim.

iOS physical device flows are manual/macOS physical proof:

- install/launch;
- When In Use location;
- Always permission state;
- region monitoring enter/exit;
- significant-change;
- visit event if supported;
- background delivery;
- low-power degraded state;
- live-mode TTL stop;
- child disclosure visible.

## Playwright UI Tests

Portal tests must run with real service-backed state, not portal-local fake
state.

Required coverage:

- parent config loads from service;
- parent config change submits typed intent;
- invalid config shows service error;
- success shows audit ref;
- child-agent unavailable shows manual-required;
- tracking status renders live, stale, offline-last-known, low-accuracy,
  permission-denied, service-disabled, battery-throttled, ambiguous nearby
  place, unexpected place, and critical-boundary states;
- evidence drawer shows correlation id, causation chain, evidence refs, policy
  ref, AI uncertainty, manual-required gap, and no raw private AI input;
- live-mode requested/started/expired/manual-required states render TTL, reason,
  stop condition, and no-policy no-start state;
- notification/escalation states render intent, provider unavailable, dispatch
  result, no delivery claim on dry-run, policy refs, and manual-required state;
- child-facing disclosure, safe/help actions, share-location consent, no
  accusation copy, and live-mode disclosure are visible.

## Property, Differential, Mutation, And Fuzz Tests

Property/invariant tests:

- AI is never authority;
- weak signal never becomes critical;
- LAN/IP never becomes precise location;
- nearby-place ambiguity never becomes accusation;
- policy is required before notification/live-mode/escalation;
- live-mode always has TTL;
- notification never claims provider success without result;
- replay is idempotent and projection-only;
- replay preserves tombstones.

Differential tests:

- TypeScript and Rust tracking event schemas match;
- live chain and replayed chain produce equivalent read models;
- journal read model and SQLite read model agree;
- service API state and portal rendered state agree;
- Android emulator and physical outputs share schema shape without claiming the
  same capability.

Mutation tests must fail if code allows AI to alert, removes policy decision
refs, removes live TTL, lets replay dispatch actions, removes tombstone filters,
changes low-accuracy to critical, or duplicates notifications.

Fuzzing must cover tracking event envelopes, location evidence, geofence
transitions, nearby-place payloads, AI results, policy decisions, notification
results, portal intents, API commands, and journal replay. Fuzz tests must
reject malformed timestamps, bad coordinates, huge payloads, overlong strings,
bad JSON, expired commands, duplicate commands, corrupt lines, and partial
journal writes without panic or silent success.

## Security, Abuse, And Operations Tests

Security tests must cover:

- missing/expired/invalid/wrong-device tokens;
- child token cannot perform parent action;
- parent token cannot publish child-agent internal events;
- child-agent, AI provider, notification provider, portal, and unknown peers
  cannot publish policy/enforcement/business events outside their authority;
- replayed old config/live-mode/notification commands are rejected;
- idempotency-key reuse is reported;
- correlation/causation spoofing is rejected;
- CORS, origin, host header, forwarded-host, header injection, request
  splitting, open redirect, URL hijack, WebSocket origin, request smuggling,
  desync, and cache poisoning are rejected where applicable;
- malformed, stale, wrong-child/device, hallucinated-ref, or unauthorized AI
  result events are rejected at the tracking boundary;
- accepted AI result evidence cannot create policy/live-mode/notification/
  escalation/enforcement authority or unsafe accusation copy.

Concurrency/race tests must cover duplicate samples, config change while
live-mode starts, parent stop while policy starts live, retention delete while
read model renders, duplicate notification dispatch, late provider result,
child-agent offline during command, restart during journal append, two parents
changing the same child config, and acknowledgement versus critical alert.

Load/spike/soak/resource tests must report dispatch latency, journal latency,
SQLite latency, portal API latency, memory growth, file descriptor count, queue
depth, dead-letter count, dropped event count, and provider retry storm bounds.

Migration/rollback/version-skew tests must preserve tombstones, avoid action
re-execution, and degrade unknown future events instead of silently accepting
unsupported commands.

Chaos tests must cover SQLite lock, journal write failure, provider timeout, AI
provider unavailable, child-agent disconnect, WebSocket drop, service restart
after policy-before-action, service restart after action-before-result,
parent/child clock skew, disk full journal, and network-offline provider state.

Clock tests must cover live-mode TTL, deadlines, notification retry deadlines,
token expiry, retention delete boundaries, DST expected-place schedules,
timezone travel, child/parent clock skew, and journal replay monotonic order.

Logging/metrics/tracing tests must prove precise location and child-sensitive
text are not leaked in info/error logs, correlation/event/policy/provider refs
are present, dead letters are visible, and operational alerts fire for
dead-letter rate, provider-unavailable rate, journal write failure, queue
overflow, policy latency SLO burn, and stuck live sessions.

Misuse/rate-limit tests must cover double-click live start, refresh abuse,
parent notification spam, live start without rule, critical attempt for weak
signal, child check-in spam, denied permission, duplicate provider result, late
AI provider result, large JSON, many WebSocket connections, and retry storms.

## CI And Manual Matrix

Windows runner:

- workspace/local tests;
- Rust workspace tests and clippy;
- portal tests and Playwright;
- tracking local service E2E;
- Android emulator if AVD is available;
- Docker only if Docker is available.

Ubuntu/Linux runner:

- Rust workspace tests and clippy;
- eventing crate tests;
- journal/replay tests;
- SQLite tests;
- Docker tests;
- API/schema fuzzing;
- load smoke.

macOS runner:

- Rust workspace tests and clippy;
- iOS simulator package/build tests;
- macOS portal/service tests;
- iOS simulator proof when Xcode is available.

Manual/local Windows:

- Android physical device tests;
- WSL/local replay;
- Docker Desktop resource tests if installed;
- Windows service/firewall/location hint tests where needed.

Manual/macOS physical:

- iOS physical Core Location;
- iOS region monitoring;
- iOS background significant-change;
- iOS notification proof.

## Proof Acceptance

Proof roots after tests pass:

```text
output/eventing-plan-proof/generic-crate-runtime/proof.json
output/tracking-plan-proof/34-tracking-event-contracts/proof.json
output/tracking-plan-proof/35-parent-tracking-config-event-flow/proof.json
output/tracking-plan-proof/36-tracking-detection-cascade-event-flow/proof.json
output/tracking-plan-proof/37-tracking-event-journal-replay-projection/proof.json
output/tracking-plan-proof/38-tracking-notification-escalation-event-flow/proof.json
output/tracking-plan-proof/39-tracking-portal-event-read-model-proof/proof.json
output/tracking-plan-proof/android-emulator-event-flow/proof.json
output/tracking-plan-proof/android-physical-device-event-flow/proof.json
output/tracking-plan-proof/ios-simulator-event-flow/proof.json
output/tracking-plan-proof/ios-physical-device-event-flow/proof.json
```

Each proof must include:

```json
{
  "assignedWorkpack": "",
  "commit": "",
  "sourceFilesChanged": [],
  "testFilesChanged": [],
  "commandsRun": [],
  "passed": true,
  "runtimeServicesStarted": [],
  "eventChainsObserved": [],
  "journalArtifacts": [],
  "sqliteArtifacts": [],
  "readModelArtifacts": [],
  "uiArtifacts": [],
  "platformArtifacts": [],
  "proofTier": "",
  "claimsProven": [],
  "claimsNotProven": [],
  "manualRequiredGaps": []
}
```

Reject proof when:

- `sourceFilesChanged` is empty;
- `sourceFilesChanged` only contains proof files;
- tests are proof tests only;
- commands did not run real code;
- proof claims product readiness without platform/provider evidence.
