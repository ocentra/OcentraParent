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
  [roadmap V4](../expectations/roadmap-v4-parent-owned-reports-optional-assistant.md).
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
- App/game notification local outbox bridge proof now maps eligible app/game
  notification intents into queued parent-owned local outbox records with
  minimal ref-only envelopes and records manual-required/unavailable app/game
  intents as blocked, without provider delivery, parent notification UI,
  adapter dispatch, durable service persistence, broad blocking, or platform
  claims.
- App/game notification scheduler handoff proof now maps those eligible
  app/game local outbox bridge rows into `due-local` scheduler records with
  parent-owned scheduler artifact refs, while manual-required/unavailable
  app/game intents remain blocked from scheduler rows and provider/runtime
  claims stay false.
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
The app/game notification intent proof adds typed app/game alert intent
readiness and minimal payload boundaries, but does not claim provider delivery,
provider receipt ingestion, service persistence, child-device delivery, parent
notification UI, policy evaluator execution, adapter dispatch, broad app
blocking, or platform support.
The app/game notification local outbox bridge proof adds a typed bridge from
eligible app/game intents into parent-owned local outbox records, but does not
claim provider delivery, receipt ingestion, durable service persistence,
child-device delivery, parent notification UI, policy evaluator execution,
adapter dispatch, broad app blocking, or platform support.
The app/game notification scheduler handoff proof adds a typed handoff from
eligible app/game local outbox bridge rows into due-local scheduler records,
but does not claim production retry workers, quiet-hours timer execution,
provider delivery, receipt ingestion, credentials, durable service persistence,
child-device delivery, parent notification UI, policy evaluator execution,
adapter dispatch, broad app blocking, or platform support.
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
- [x] App/game notification local outbox bridge proof exists with eligible
      app/game alert intents mapped to queued parent-owned local outbox records
      and manual-required/unavailable intents blocked from outbox records,
      without provider delivery, adapter dispatch, durable service persistence,
      parent notification UI, broad blocking, or platform claims.
- [x] App/game notification scheduler handoff proof exists with eligible
      app/game local outbox bridge rows mapped to `due-local` scheduler records
      and manual-required/unavailable intents blocked from scheduler rows,
      without provider delivery, provider receipt ingestion, credentials,
      production retry workers, quiet-hours timer execution, durable service
      persistence, parent notification UI, broad blocking, or platform claims.
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
`scripts/test/app-game-notification-local-outbox-bridge-proof.mjs` as
parent-domain app/game intent-to-local-outbox bridge proof only; require service
persistence, provider adapters, real send/retry execution, receipt ingestion,
parent-visible notification UI, child-device delivery, adapter dispatch, broad
blocking, and platform proof before claiming product notification runtime for
app/game controls. Treat
`scripts/test/parent-owned-sync-export-manifest-proof.mjs` as parent-domain
sync/export manifest and connector-status contract proof only; require real
transfer runtime, connector OAuth/provider artifacts, parent-visible controls,
retention/delete execution, and import/rebuild proof before claiming product
sync/export.
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
