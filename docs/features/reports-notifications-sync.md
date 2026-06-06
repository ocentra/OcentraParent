<!-- agent-capsule -->

> Agent Capsule
> Doc: Reports, Notifications, And Sync
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Reports, Notifications, And Sync

## Parent Outcome

Parents get useful alerts, summaries, history, and reports without Ocentra
becoming the default store for child activity data.

## Ocentra Requirement

Reports cite local or parent-owned evidence. Notifications carry minimal
detail and link to authenticated parent surfaces. Sync/export goes to
parent-selected storage unless a remote path is explicitly authorized.

## Roadmap And Expectations

- Roadmap: V2 remote access, V3 notifications, V4 reports/assistant.
- Expectations: [notifications](../expectations/notifications.md),
  [sync/export](../expectations/sync-export.md),
  [data custody](../expectations/data-custody.md),
  [roadmap V4](../roadmaps/roadmap-v4-parent-owned-reports-optional-assistant.md).
- Modules: `packages/activity-domain`, `packages/parent-domain`,
  `packages/logging-domain`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
notifications/alerts, reports/digests, remote parent access, and local-first
privacy.

Mature competitors provide alerts, weekly reports, activity history, and remote
parent review. Ocentra must match usefulness while preserving local-first data
custody.

## Current Ocentra State

- Activity report persistence/family fanout/MIA context proof exists for the
  backend/read-model boundary.
- Saved Activity reports are stored as local JSON report documents and
  historical report queries now preserve scope, requested window, parsed report
  metadata, typed storage fallback states, parent-local custody/source labels,
  and `rawChildEvidenceIncluded=false`.
- Family Activity report aggregation preserves ready/offline/stale/unavailable,
  unreachable, and error source states for service-owned UI handoff without
  making Vite or portal UI the product-data owner.
- The Activity UI intent layer consumes service-backed report, history, and tab
  read-model adapter results for Reports, Screen, App Use, Browser, Games, and
  Network. Failed or absent adapter results stay explicit instead of creating
  fixture-backed devices.
- Parent Assistant/MIA evidence context can cite saved Activity report history
  with parent-owned report custody and `directEnforcementAllowed=false`; it is
  citation context only.
- Notification and sync/export expectation docs exist.
- V0.8 integrity alert/status bridge proof now creates minimal notification
  intent/status refs for permission-loss, stale heartbeat, stopped-or-removed,
  and tamper/manual-required enforcement integrity states, with authenticated
  drill-in refs back to audit evidence.
- V0.8 notification provider status boundary proof now represents queued,
  delivered, failed, unavailable, and manual-required provider status contract
  states, plus quiet-hours and escalation readiness refs, through the existing
  supported-adapter runtime proof event payload.
- V3 notification rule/provider retry contract proof now represents alert rule,
  reason code, provider channel, delivery attempt/result, retry policy,
  quiet-hours, escalation, parent preference, audit, and evidence refs without
  claiming provider adapters, delivery execution, provider receipts, raw
  evidence payloads, or provider child-evidence storage.
- Notification audit/history logging contract proof now represents provider
  status, retry lifecycle, receipt/manual-required refs, quiet-hours/escalation
  refs, redaction-safe payload fields, and child-data non-custody without
  claiming provider adapters, send/retry execution, webhook receipt ingestion,
  notification history UI, credentials, or Ocentra-hosted child evidence.
- Notification local outbox adapter-boundary proof now writes and rereads a
  deterministic parent-owned JSONL outbox artifact with minimal alert
  envelopes, provider-channel abstraction, quiet-hours defer, retry,
  dead-letter, receipt-required, manual-required states, audit/evidence/policy
  refs, and sensitive-detail minimization without claiming provider delivery,
  receipt ingestion, credentials, cloud routing, parent notification UI, raw
  child evidence, or sensitive provider metadata storage.
- Notification local outbox scheduler proof now writes and rereads a
  deterministic parent-owned JSONL scheduler artifact for due, held
  quiet-hours, retry-window scheduled, dead-letter review, receipt-required, and
  manual-required states. It proves deterministic `nextAttemptAt` and retry
  window rows while keeping provider delivery, receipt ingestion, credentials,
  cloud routing, parent notification UI, production retry workers, production
  quiet-hours timers, durable production outbox storage, raw child evidence, and
  sensitive provider metadata unclaimed.
- App/game notification intent contract proof now represents time-limit,
  approval request, suspicious unknown, manual-required, and unavailable
  app/game notification intents with evidence, policy, audit, child
  reason/status, minimal payload refs, local-outbox-only readiness, and explicit
  no-provider, no-cloud, no-parent-UI, and no-adapter-dispatch claims.
- App/game notification readiness service proof now exposes a dedicated
  command/event that derives time-limit, approval-request, suspicious-unknown,
  manual-required, and unavailable readiness rows from the existing app/game
  service read model while preserving explicit no-provider-delivery,
  no-receipt-ingestion, no-outbox-runtime, no-scheduler-runtime,
  no-parent-UI, no-child-delivery, and no-adapter-dispatch claims.
- App/game notification local outbox bridge proof now maps validated
  local-outbox-eligible app/game notification intents into the existing
  parent-owned local outbox JSONL record schema, and keeps manual-required and
  unavailable app/game intents visible without queueing them for delivery.
- App/game notification scheduler bridge proof now maps those linked local
  outbox records into existing notification scheduler JSONL rows, while keeping
  manual-required and unavailable app/game rows visible without scheduling them.
- App/game notification audit-history bridge proof now maps linked local outbox
  rows into existing logging-domain notification audit/history entries, while
  keeping manual-required and unavailable app/game rows visible as
  blocked/manual audit rows without provider sends.
- App/game notification provider preflight proof now maps scheduled scheduler
  rows into provider-adapter-required rows with scheduler, outbox, decision,
  provider-channel, reason, adapter-requirement, credential, and provider smoke
  proof refs, while keeping manual-required and unavailable rows blocked before
  provider setup.
- App/game notification preference preflight proof now maps scheduled scheduler
  rows into parent-preference-required rows with provider-channel, reason,
  parent preference, frequency-control, and quiet-hours proof refs, while
  keeping manual-required and unavailable rows blocked before parent controls or
  delivery.
- App/game notification provider-status handoff proof now maps those provider
  preflight rows into existing V0.8 provider-status boundary rows for
  manual-required and unavailable states, preserving scheduler/outbox/provider
  refs while keeping delivery, receipt, credential, runtime, UI, child delivery,
  adapter dispatch, broad-blocking, and platform claims false.
- App/game notification preference-status handoff proof now maps preference
  preflight rows into V3 notification preference and quiet-hours status entries,
  preserving scheduler/outbox/provider/reason/preference/quiet-hours refs while
  keeping parent preference UI, notification UI, delivery, receipt, credential,
  runtime, child delivery, adapter dispatch, broad-blocking, and platform claims
  false.
- App/game notification parent-surface intent proof now combines provider-status
  and preference-status rows into redacted history/preference intent rows for a
  future parent surface, preserving drill-in, audit, scheduler/outbox, provider,
  preference, quiet-hours, and manual-proof refs while keeping rendered UI,
  provider delivery, receipt ingestion, credentials, runtime, child delivery,
  adapter dispatch, broad-blocking, and platform claims false.
- App/game notification parent-surface route proof now renders those
  schema-backed intent rows in the App/Game Sessions portal route when a
  read-model is supplied, while the live route shows an explicit missing-service
  state and still makes no provider delivery, receipt, credential, parent
  preference mutation, child delivery, production runtime, adapter dispatch,
  broad-blocking, or platform claim.
- Tracking provider-notification proof now maps tracking location/geofence alert
  intents from the tracking policy read model into existing V0.8 provider-status
  boundary rows. It preserves tracking evidence refs, policy decision refs,
  notification status refs, reason refs, and sensitive-detail modes while
  keeping provider delivery, receipt ingestion, credentials, cloud routing,
  parent notification UI, child-device delivery, physical-device proof,
  production retry/quiet-hours runtime, durable outbox storage, and adapter
  dispatch unclaimed.
- Tracking escalation readiness proof now derives parent acknowledgement,
  child check-in, urgent second-guardian, critical multi-channel, manual, and
  unavailable readiness rows from the tracking policy read model while keeping
  AI scheduling, emergency auto-contact, provider delivery, receipt ingestion,
  credentials, cloud routing, parent notification UI, child-device delivery,
  production workers, and physical-device proof unclaimed.
- Tracking notification parent-surface history intent proof now joins tracking
  provider-notification, receipt boundary, and preference preflight rows into
  redacted parent history/preference intent rows for a future parent surface.
  It preserves provider, receipt, preference, quiet-hours, evidence, policy,
  notification-status, audit, manual-proof, and authenticated drill-in refs
  while keeping rendered notification UI, parent preference mutation, provider
  delivery, receipt ingestion runtime, credentials, cloud routing,
  child-device delivery, physical-device proof, authority proof, retry workers,
  durable production history/outbox storage, and adapter dispatch unclaimed.
- Parent-owned sync/export manifest contract proof now represents export
  manifest data classes, export formats, encryption metadata, retention/delete
  policy, connector status, sync cursor states, conflict records, import
  results, and delete results without claiming transfer runtime, connector
  OAuth, portal UI, report compiler runtime, account/subscription backend, raw
  child evidence upload by default, or Ocentra-hosted child evidence custody.
- Parent-owned local export/delete runtime proof now represents
  parent-authorized Windows local export and delete jobs with queued, running,
  written, delete requested, delete confirmed, delete failed, offline queued,
  and manual-required states; encrypted local output metadata; delete
  confirmation; audit refs; and source evidence retention for local safety
  without claiming cloud transfer, connector OAuth/provider API, portal UI,
  remote report compilation, child-device mutation, raw evidence upload, or
  Ocentra-hosted family data custody.
- Stateless report compiler status contract proof now represents parent-
  authorized remote report compilation requests from parent-owned storage,
  source connector/cursor refs, requested data classes and time windows,
  parent-owned output destination, queued/running/succeeded/failed/expired/
  manual-required states, temporary input/output TTL and deletion confirmation,
  redaction/minimization flags, audit refs, and non-mutating failure behavior
  without claiming a report compiler runtime, cloud worker, connector OAuth/
  provider API, portal UI, upload/download implementation, child-device
  mutation, retained temp child evidence, or Ocentra-hosted family data custody.
- Parent-facing reports, alert delivery, and connectors are incomplete.

## Current Gap

Need report UI polish, trend summaries, evidence citations in report sections,
cited assistant Q&A as a complete product flow, real notification provider
adapters, retry execution, provider receipt ingestion, parent controls,
connectors, retention, and delete/export controls.
The V0.8 integrity bridge and provider status boundary prove only minimal
notification intent/status/readiness references and audit drill-in, not provider
delivery or UI. The V3 notification rule/provider retry contract proves the
typed notification rule, reason/channel, attempt/result, retry, preference, and
audit/evidence contract shape, not provider adapters, actual sends, receipts,
parent controls, or notification history UI. The notification audit/history
logging proof adds redaction-safe operational log history shape and child-data
non-custody flags, but does not claim provider delivery, retry execution,
webhook receipt ingestion, credentials, or parent-facing notification history.
The notification local outbox adapter-boundary proof adds deterministic local
outbox artifact writing and parsing for minimal notification envelopes and
defer/retry/dead-letter/receipt/manual states, but does not claim provider
delivery, receipt ingestion, credentials, cloud routing, parent notification UI,
quiet-hours scheduler execution, retry execution, or production durable outbox
storage.
The notification local outbox scheduler proof adds deterministic scheduler
read-model rows and parent-owned scheduler artifact writing/parsing for
due/held/retry/dead-letter/receipt/manual states, but does not claim provider
delivery, receipt ingestion, credentials, cloud routing, parent notification UI,
production retry worker execution, production quiet-hours timer execution, or
durable production outbox storage.
The app/game notification readiness service proof adds typed service rows for
local alert readiness, but does not claim provider delivery, provider receipt
ingestion, production local outbox or scheduler runtime, child-device delivery,
parent notification UI, policy evaluator execution, adapter dispatch, broad app
blocking, or platform support.
The app/game notification local outbox bridge proof adds deterministic
intent-to-local-outbox JSONL writing/parsing for eligible app/game notification
intents, but does not claim durable production outbox storage, provider
delivery, receipt ingestion, quiet-hours/retry scheduler execution, parent UI,
child delivery, adapter dispatch, broad blocking, or platform support.
The app/game notification scheduler bridge proof adds deterministic
local-outbox-to-scheduler JSONL writing/parsing for linked app/game local outbox
records, but does not claim production retry workers, production quiet-hours
timer execution, durable production outbox storage, provider delivery, receipt
ingestion, parent UI, child delivery, adapter dispatch, broad blocking, or
platform support.
The app/game notification audit-history bridge proof adds deterministic
local-outbox-to-audit-history handoff rows for linked, manual-required, and
unavailable app/game notification states, but does not claim provider delivery,
receipt ingestion, production retry workers, production quiet-hours timers,
durable production outbox/history storage, parent UI, child delivery, adapter
dispatch, broad blocking, or platform support.
The app/game notification provider preflight proof adds deterministic
scheduler-to-provider-preflight rows for scheduled app/game alerts, but does not
claim provider delivery, receipt ingestion, credentials, production retry
workers, production quiet-hours timer execution, durable production outbox
storage, parent UI, child delivery, adapter dispatch, broad blocking, or
platform support.
The app/game notification preference preflight proof adds deterministic
scheduler-to-parent-preference rows for scheduled app/game alerts, but does not
claim parent preference UI, frequency controls, provider delivery, receipt
ingestion, credentials, production retry workers, production quiet-hours timer
execution, durable production outbox storage, child delivery, adapter dispatch,
broad blocking, or platform support.
The app/game notification provider-status handoff proof adds deterministic
provider-preflight-to-provider-status rows for app/game alerts, but does not
claim provider delivery, receipt ingestion, credentials, production retry
workers, production quiet-hours timer execution, durable production outbox
storage, parent UI/history/preferences, child delivery, adapter dispatch, broad
blocking, or platform support.
The app/game notification preference-status handoff proof adds deterministic
preference-preflight-to-V3 preference/quiet-hours status rows for app/game
alerts, but does not claim parent preference UI, frequency controls, parent
notification UI/history/preferences, provider delivery, receipt ingestion,
credentials, production retry workers, production quiet-hours timer execution,
durable production outbox storage, child delivery, adapter dispatch, broad
blocking, or platform support.
The app/game notification parent-surface intent proof adds redacted future
history/preference intent rows over provider/preference status handoffs, and
the App/Game Sessions route now has a route-level renderer for those rows when
the read model is supplied. It still does not claim product notification
delivery, parent preference mutation, provider delivery, provider receipts,
credentials, production runtime, child delivery, adapter dispatch, broad
blocking, mobile UI, or platform support.
The tracking provider-notification proof adds deterministic
tracking-alert-to-provider-status rows for location/geofence alerts, but does
not claim provider delivery, receipt ingestion, credentials, cloud routing,
parent notification UI/history/preferences, child-device delivery,
physical-device proof, production retry workers, production quiet-hours timers,
durable production outbox storage, or adapter dispatch.
The tracking notification parent-surface history intent proof adds redacted
future history/preference intent rows over tracking provider-notification,
receipt boundary, and preference preflight rows. It does not claim rendered
tracking notification UI, preference mutation, provider delivery, receipt
ingestion runtime, credentials, production retry/quiet-hours workers, durable
production history/outbox storage, child-device delivery, physical-device
proof, authority proof, or adapter dispatch.
The tracking escalation readiness proof adds deterministic tracking-policy
read-model rows for acknowledgement cancellation, child check-in resolution,
urgent second-guardian manual readiness, critical multi-channel manual
readiness, manual-required, and unavailable states, but does not claim AI direct
scheduling, emergency auto-contact, provider delivery, receipt ingestion,
credentials, cloud routing, parent notification UI/history/preferences,
child-device delivery, production escalation workers, production quiet-hours
timers, durable storage, physical-device proof, or platform support.
The parent-owned sync/export manifest proof adds typed export/retention/delete,
connector status, cursor, conflict, import, and delete result states, but does
not claim real export/import/upload/download runtime, connector OAuth,
provider API calls, portal controls, report compiler runtime, or Ocentra-hosted
child data custody.
The parent-owned local export/delete runtime proof adds typed local export queue,
encrypted local output, delete confirmation/failure, offline queue, and
manual-required states, but does not claim cloud transfer, connector OAuth,
provider API calls, portal controls, remote report compilation, child-device
mutation, raw evidence upload, or Ocentra-hosted child data custody.
The stateless report compiler status proof adds typed request/status/result
state for parent-authorized report compilation from parent-owned storage, but
does not claim a compiler runtime, cloud worker, connector OAuth/provider API,
portal controls, upload/download implementation, retained temporary evidence,
child-device mutation, or Ocentra-hosted child data custody.
Activity report persistence/family fanout/MIA context proof does not claim
physical household fanout, raw child evidence transfer, provider notification
delivery, policy writes, or child-device enforcement.

## Checklist

- [x] Report contract and storage/custody state.
- [x] Activity service adapter feeds the UI intent seam for reports and tab
      read-model states without Vite-owned product data.
- [ ] Evidence citations in reports.
- [x] Parent-owned export/sync connector status contract proof exists without
      transfer runtime, connector OAuth, provider API, portal UI, or
      Ocentra-hosted child data custody claims.
- [x] Parent-owned local export/delete runtime proof exists for local queued,
      written, delete confirmed/failed, offline queued, and manual-required
      states without cloud transfer, connector OAuth/provider API, portal UI,
      remote compiler, child mutation, raw evidence upload, or Ocentra-hosted
      custody claims.
- [x] Stateless report compiler status contract proof exists for
      parent-authorized compilation from parent-owned storage without compiler
      runtime, cloud worker, connector OAuth/provider API, portal UI,
      upload/download implementation, child-device mutation, retained temporary
      evidence, or Ocentra-hosted family data custody claims.
- [x] Notification rule/reason/channel/retry/preference contract proof exists
      without provider delivery claims.
- [x] Minimal payload and authenticated drill-in refs exist for V0.8 integrity
      alert/status bridge states.
- [x] Delivery/queued/failed/unavailable/manual-required provider status
      read-model proof exists without provider delivery claims.
- [x] Quiet-hours and escalation readiness read-model proof exists without
      parent controls or provider delivery.
- [x] Tracking escalation readiness proof exists with parent acknowledgement,
      child check-in, urgent second-guardian, critical multi-channel,
      manual-required, and unavailable readiness rows, without AI direct
      scheduling, emergency auto-contact, provider delivery, receipt ingestion,
      credentials, cloud routing, parent UI/history/preferences, child-device
      delivery, production worker, durable storage, physical-device, or platform
      claims.
- [x] Notification audit/history logging contract proof exists with
      redaction-safe payload fields and child-data non-custody flags, without
      provider delivery, receipt ingestion, credentials, or history UI claims.
- [x] Notification local outbox adapter-boundary proof exists with deterministic
      parent-owned JSONL artifact writing/parsing, minimal alert envelopes,
      quiet-hours defer, retry, dead-letter, receipt-required, and manual states,
      without provider delivery, receipt ingestion, credentials, cloud routing,
      parent notification UI, or sensitive detail storage claims.
- [x] Notification local outbox scheduler proof exists with deterministic
      due/held quiet-hours/retry-window/dead-letter/receipt/manual scheduler
      states and parent-owned JSONL artifact writing/parsing, without provider
      delivery, receipt ingestion, credentials, cloud routing, parent
      notification UI, production retry workers, production quiet-hours timers,
      durable production outbox storage, or sensitive detail storage claims.
- [x] App/game notification intent contract proof exists with time-limit,
      approval request, suspicious unknown, manual-required, and unavailable
      app/game alert intents, minimal payload refs, local-outbox-only readiness,
      and explicit no-provider/no-adapter/no-UI claims.
- [x] App/game notification readiness service proof exists with dedicated
      command/event rows for time-limit, approval-request, suspicious-unknown,
      manual-required, and unavailable states, without provider delivery,
      receipts, production outbox/scheduler runtime, parent UI, child delivery,
      or adapter claims.
- [x] App/game notification local outbox bridge proof exists with deterministic
      parent-owned JSONL records for eligible app/game notification intents,
      while manual-required and unavailable intents do not queue delivery and no
      provider/scheduler/UI/child/adapter claims are made.
- [x] App/game notification scheduler bridge proof exists with deterministic
      scheduler JSONL rows for linked app/game local outbox records, while
      manual-required and unavailable rows stay unscheduled and no production
      runtime/provider/UI/child/adapter claims are made.
- [x] App/game notification audit-history bridge proof exists with
      metadata-only logging-domain audit rows for linked, manual-required, and
      unavailable app/game notification states, without
      runtime/provider/UI/child/adapter claims.
- [x] App/game notification provider preflight proof exists with scheduled
      app/game scheduler rows becoming provider-adapter-required rows, while
      manual-required and unavailable rows stay blocked and no delivery,
      receipt, credential, UI, child, or adapter-dispatch claims are made.
- [x] App/game notification preference preflight proof exists with scheduled
      app/game scheduler rows becoming parent-preference-required rows, while
      manual-required and unavailable rows stay blocked and no parent UI,
      delivery, receipt, credential, child, or adapter-dispatch claims are made.
- [x] App/game notification provider-status handoff proof exists with provider
      preflight rows becoming V0.8 provider-status manual-required/unavailable
      rows, while delivery, receipts, credentials, runtime, UI, child delivery,
      adapter dispatch, broad blocking, and platform claims remain false.
- [x] App/game notification preference-status handoff proof exists with
      preference preflight rows becoming V3 parent preference/quiet-hours status
      entries, while parent preference UI, delivery, receipts, credentials,
      runtime, UI, child delivery, adapter dispatch, broad blocking, and
      platform claims remain false.
- [x] App/game notification parent-surface intent proof exists with provider and
      preference status rows becoming redacted parent history/preference intent
      rows, while rendered UI, parent preference mutation, provider delivery,
      receipts, credentials, runtime, child delivery, adapter dispatch, broad
      blocking, and platform claims remain false.
- [x] App/game notification parent-surface route renderer exists for the
      App/Game Sessions route and consumes schema-backed intent read models
      without inventing rows when the service event is absent, while provider
      delivery, parent preference mutation, child delivery, runtime dispatch,
      broad blocking, and platform claims remain false.
- [x] Tracking provider-notification proof exists with tracking alert intents
      becoming V0.8 provider-status manual-required/unavailable rows, while
      provider delivery, receipts, credentials, runtime, parent UI,
      child-device delivery, physical-device proof, durable outbox storage, and
      adapter dispatch remain false.
- [x] Tracking notification parent-surface history intent proof exists with
      tracking provider, receipt, and preference rows becoming redacted future
      parent history/preference rows, while rendered UI, preference mutation,
      provider delivery, receipt runtime, credentials, production storage,
      child-device delivery, physical-device proof, authority, and adapter
      dispatch remain false.
- [ ] Retention/delete controls.

## Next AI Instructions

Do not put sensitive child detail in notification payloads. Do not store reports
in Ocentra-hosted systems by default. Keep report source and custody labels
visible. Treat `scripts/test/v0-8-integrity-alert-status-bridge.mjs` as
notification intent/status proof only; require provider artifacts before claiming
delivery. Treat `scripts/test/v0-8-notification-provider-status-boundary.mjs`
as provider status/readiness plus V3 notification rule/provider retry contract
proof only. Treat `scripts/test/notification-audit-history-contract-proof.mjs`
as logging-domain audit/history contract proof only; require provider adapter,
real send/retry execution, receipt ingestion, credentials, parent-control, and
notification-history UI artifacts before claiming notification delivery or
parent-facing notification history. Treat
`scripts/test/notification-local-outbox-adapter-proof.mjs` as parent-domain
local outbox adapter-boundary proof only; require provider adapters, real
send/retry execution, receipt ingestion, quiet-hours scheduler, parent-visible
history/preferences UI, production durable storage, and provider smoke proof
before claiming notification delivery or product notification runtime. Treat
`scripts/test/notification-local-outbox-scheduler-proof.mjs` as parent-domain
local outbox scheduler proof only; require provider adapters, production retry
workers, production quiet-hours timers, receipt ingestion, parent-visible
history/preferences UI, production durable storage, and provider smoke proof
before claiming notification delivery or product notification runtime. Treat
`scripts/test/app-game-notification-service-read-model-proof.mjs` as
service-backed app/game local notification-readiness proof only; require
provider adapters, production outbox/scheduler runtime, receipt ingestion,
parent-visible history/preferences UI, child delivery proof, and provider smoke
proof before claiming app/game notification delivery or product notification
runtime. Treat
`scripts/test/app-game-notification-local-outbox-bridge-proof.mjs` as
parent-domain app/game local outbox bridge proof only; require durable
production outbox storage, provider adapters, production retry workers,
quiet-hours timers, receipt ingestion, parent-visible history/preferences UI,
child delivery proof, and provider smoke proof before claiming app/game
notification delivery or product notification runtime. Treat
`scripts/test/app-game-notification-scheduler-bridge-proof.mjs` as parent-domain
app/game scheduler bridge proof only; require production retry workers,
production quiet-hours timers, durable production outbox storage, provider
adapters, receipt ingestion, parent-visible history/preferences UI, child
delivery proof, and provider smoke proof before claiming app/game notification
delivery or product notification runtime. Treat
`scripts/test/app-game-notification-audit-history-bridge-proof.mjs` as
logging-domain app/game audit-history handoff proof only; require provider
adapters, production retry workers, production quiet-hours timers, receipt
ingestion, parent-visible history/preferences UI, durable production history or
outbox storage, child delivery proof, and provider smoke proof before claiming
app/game notification delivery or product notification runtime. Treat
`scripts/test/app-game-notification-provider-preflight-proof.mjs` as
parent-domain app/game provider preflight boundary proof only; require real
provider adapters, credentials and secret review, production send/retry workers,
production quiet-hours timers, receipt ingestion, parent-visible
history/preferences UI, durable production storage, child delivery proof, and
provider smoke proof before claiming app/game notification delivery or product
notification runtime. Treat
`scripts/test/app-game-notification-preference-preflight-proof.mjs` as
parent-domain app/game parent preference preflight proof only; require
parent-visible history/preferences UI, frequency controls, real provider
adapters, credentials and secret review, production send/retry workers,
production quiet-hours timers, receipt ingestion, durable production storage,
child delivery proof, and provider smoke proof before claiming app/game
notification delivery or product notification runtime. Treat
`scripts/test/app-game-notification-preference-status-handoff-proof.mjs` as
parent-domain app/game notification preference/quiet-hours status handoff proof
only; require parent-visible history/preferences UI, frequency controls, real
provider adapters, credentials and secret review, production send/retry workers,
production quiet-hours timers, receipt ingestion, durable production storage,
child delivery proof, and provider smoke proof before claiming app/game
notification delivery or product notification runtime. Treat
`scripts/test/app-game-notification-parent-surface-intent-proof.mjs` as
parent-domain app/game notification parent-surface intent proof only; require
product notification history controls, parent preference mutation controls,
real provider adapters, credentials and secret review, production send/retry
workers, production quiet-hours timers, receipt ingestion, durable production
storage, child delivery proof, mobile UI proof, and provider smoke proof before
claiming parent-facing app/game notification delivery or product notification
runtime. Treat
`apps/portal/tests/app-game-notification-parent-surface-panel.test.ts` as
portal route rendering proof for schema-backed parent-surface intent rows only;
it does not prove a live service event, preference mutation, provider delivery,
child delivery, or production notification runtime. Treat
`scripts/test/parent-owned-sync-export-manifest-proof.mjs` as parent-domain
sync/export manifest and connector-status contract proof only; require real
transfer runtime, connector OAuth/provider artifacts, parent-visible controls,
retention/delete execution, and import/rebuild proof before claiming product
sync/export.
Treat `scripts/test/tracking-provider-notification-proof.mjs` as tracking alert
intent to provider-status boundary proof only; require real provider adapters,
receipt ingestion, credentials, parent notification UI/history/preferences,
child-device delivery, physical-device proof, production retry and quiet-hours
runtimes, durable outbox storage, and adapter dispatch before claiming tracking
notification delivery.
Treat `scripts/test/parent-owned-local-export-runtime-proof.mjs` as
parent-domain local export/delete runtime-state proof only; require a real
filesystem writer, retention scheduler, delete executor, durable audit
persistence, physical Windows smoke proof, and parent-visible controls before
claiming product export/delete runtime.
Treat `scripts/test/stateless-report-compiler-status-proof.mjs` as
parent-domain stateless report compiler request/status/result proof only;
require a real compiler runtime, cloud worker, connector OAuth/provider
artifacts, parent UI, upload/download implementation, deletion execution, and
custody/security review before claiming remote report compilation.
