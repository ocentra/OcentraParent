# @ocentra-parent/parent-domain

Shared product contracts for family safety, policy, enforcement, local AI, LAN,
mobile readiness, and control catalogs.

## Owns

- Parent/family/child/device product contracts.
- Policy rules, schedules, targets, decisions, permissions, and audit shapes.
- Enforcement intents, results, capability states, timers, and readiness.
- V0.8 enforcement product-control spine contracts that separate implemented,
  degraded, dry-run, manual-required, unavailable, and not-claimed states.
- V0.8 enforcement policy-dispatch contracts that validate parent-authored
  intents, evidence refs, adapter matrix rows, timer/approval/audit state, and
  child-facing reason codes before dispatch-ready claims.
- App/game policy preview handoff contracts that map compiled dry-run native
  app and native game decisions into read-only parent preview rows while
  keeping evaluator runtime, timers, child delivery, adapter dispatch, broad
  blocking, and platform enforcement unclaimed.
- App/game source-gated policy preview timer-handoff contracts that consume the
  redacted source-gated read model, identify future timer sequencing candidates,
  and keep manual rows blocked before timer runtime without service, UI,
  adapter, child delivery, platform, or raw source-row claims.
- App/game source-gated policy preview timer scheduler-persistence contracts
  that consume runtime-readiness rows and keep service timer runtime, scheduler
  persistence, durable scheduler state-store, audit, and rollback proof required
  before any future scheduling claim.
- App/game source-gated policy preview timer audit/rollback handoff contracts
  that consume scheduler-persistence rows and keep service timer runtime,
  scheduler persistence, durable scheduler state-store, audit trail, rollback
  plan, and audit/rollback read-model proof required before any future
  scheduling claim.
- App/game source-gated policy preview timer audit/rollback read-model
  contracts that consume audit/rollback handoff rows and keep those same proof
  requirements visible without claiming service read APIs, portal UI, durable
  audit storage, rollback execution, or timer scheduling.
- App/game source-gated policy preview timer audit/rollback parent-surface
  intent contracts that consume audit/rollback read-model rows and attach
  future parent-surface proof/drill-in refs without claiming rendered UI or
  service read APIs.
- App/game source-gated policy preview timer service-readiness handoff
  contracts that consume audit/rollback parent-surface intent rows and attach
  future service-readiness/read-API proof refs without claiming service runtime
  events, read API implementation, rendered UI, durable audit storage, rollback
  execution, or timer scheduling.
- App/game source-gated policy preview timer service-readiness read-model
  contracts that consume service-readiness handoff rows and keep future
  service-readiness/read-API proof refs visible without claiming agent protocol,
  service runtime events, read API implementation, rendered UI, durable audit
  storage, rollback execution, or timer scheduling.
- App/game source-gated policy preview timer service-readiness protocol handoff
  contracts that consume service-readiness read-model rows and keep future
  agent-protocol command/event, Rust protocol mirror, service handler, and
  service read-API proof refs visible without claiming protocol implementation,
  service command registration, service event emission, rendered UI, durable
  audit storage, rollback execution, or timer scheduling.
- App/game source-gated policy preview timer service-readiness protocol
  read-model contracts that consume protocol handoff rows and keep future
  protocol proof refs visible without claiming protocol implementation, service
  command registration, service event/read-model emission, rendered UI, durable
  audit storage, rollback execution, or timer scheduling.
- V0.8 enforcement integrity runtime audit contracts that link supported action
  results, timer recovery/rollback, child-status refs, parent-override audit
  refs, permission-loss, integrity heartbeat, and tamper/manual states.
- V0.8 integrity alert/status bridge contracts that expose permission-loss,
  stale-heartbeat, stopped-or-removed, and tamper/manual parent-visible status
  rows with notification intent/status refs, audit refs, integrity refs, and
  drill-in refs.
- V0.8 notification provider status boundary contracts that represent queued,
  delivered, failed, unavailable, and manual-required provider states plus
  quiet-hours/escalation readiness as read-model proof without provider
  delivery claims.
- Notification local outbox adapter-boundary contracts that represent a
  parent-owned local outbox artifact, minimal alert envelopes, provider-channel
  abstraction, quiet-hours defer, retry, dead-letter, receipt-required, and
  manual-required states without provider delivery, receipt ingestion,
  credentials, cloud routing, parent UI, or sensitive detail storage claims.
- Notification local outbox scheduler proof contracts that represent
  deterministic due, held quiet-hours, retry-window, dead-letter review,
  receipt-required, and manual-required scheduler states with parent-owned
  artifact write/read proof, without provider delivery, receipt ingestion,
  credentials, cloud routing, parent UI, production durable storage, or
  sensitive detail storage claims.
- App/game notification preference status handoff proof contracts that map
  preference-preflight rows into V3 notification preference and quiet-hours
  manual-required or disabled status entries without parent preference UI,
  provider delivery, receipt ingestion, production quiet-hours timers, durable
  outbox storage, child delivery, policy execution, or adapter dispatch claims.
- App/game notification parent surface intent proof contracts that combine
  provider-status and preference-status handoff rows into redacted future parent
  history/preference intent rows with drill-in, audit, scheduler/outbox,
  provider, preference, quiet-hours, and manual-proof refs without rendered UI,
  parent preference mutation, provider delivery, receipt ingestion, durable
  storage, child delivery, policy execution, or adapter dispatch claims.
- Local AI runtime, provider, scheduler, context, and reference contracts,
  including screen summary context-builder replay proof from deleted local OCR
  evidence refs.
- Parent assistant and action-preview contracts.
- LAN pairing, device roles, controller/observer states, and provider routing.
- Billing/subscription entitlement contracts for plan rows, subscription status,
  device-limit decisions, failure behavior, retained evidence export, and
  local-safety continuation without billing provider SDK ownership.
- Billing entitlement runtime proof contracts for local runtime/status
  consumption of account entitlement snapshots, device-limit decisions, and
  billing failure states without live provider execution, provider contact,
  refund/credit runtime, child custody, production billing claims, or portal UI.
- Billing support/admin boundary proof contracts for support-case triage,
  account-status review, billing escalation request, provider-contact/manual,
  entitlement-admin-override/manual, refund-credit/manual states, redaction
  audit refs, and no provider/backend/admin runtime claims.
- `billing-support-admin-status-proof` contracts for parent-visible billing
  support/admin status rows, resolution-update readiness, manual-proof refs,
  evidence export retention, and explicit non-claims for provider contact,
  account lookup execution, entitlement override, refund/credit runtime, portal
  admin UI, support backend upload, and child activity custody.
- `production-support-publication-workflow-proof` contracts for public privacy
  policy publication, privacy/legal disclosure execution, support runbook
  publication, support incident status publication, support backend upload
  publication handoff, and public support contact publication while keeping real
  public runtime, support upload execution, account lookup, billing contact,
  production SLA, legal execution, remote support, and child custody unclaimed.
- `public-support-contact-status-proof` contracts for public support contact,
  support status page contact, support runbook contact, incident status contact,
  backend-upload support contact, and billing-support contact while keeping
  public runtime execution, support backend upload execution, account lookup,
  billing provider contact, remote support sessions, production SLA, legal
  disclosure execution, provider secrets, and child custody unclaimed.
- Parent-owned sync/export and stateless report compiler status contracts for
  parent-authorized remote compilation from parent-owned storage, source
  connector/cursor refs, requested data classes and time windows, temp TTL and
  deletion confirmation, redaction/minimization, audit refs, and custody
  non-mutation boundaries.
- Parent-owned local export/delete runtime-state contracts for
  parent-authorized Windows local export queues, encrypted local output
  metadata, delete confirmation/failure, offline/manual states, source evidence
  retention for local safety, and no cloud/provider/UI/custody overclaims.
- App install/purchase runtime proof boundary contracts that link platform/store
  metadata requirements, package-source artifact requirements, child
  pending/result delivery rows, report integration rows, and child status
  runtime readiness rows while keeping provider/store, runtime status reader,
  child-device delivery, runtime report delivery, and app blocking unclaimed.
- App install/purchase platform artifact proof contracts that attach
  parent-owned platform/store metadata artifact refs and report-runtime evidence
  refs to the existing runtime boundary while keeping provider/store APIs,
  platform adapters, child-device delivery, runtime report delivery, and app
  blocking unclaimed.
- App install/purchase child artifact delivery proof contracts that attach
  child package-source artifact refs and child-facing pending/result delivery
  artifact refs to the existing runtime/platform proof boundary while keeping
  production child-device capture, delivery, runtime report delivery,
  provider/store APIs, platform adapters, interception, and app blocking
  unclaimed.
- App install/purchase approved API/entitlement proof contracts that attach
  approved store API evidence refs, entitlement evidence refs, limitation report
  refs, and audit refs to child artifact rows while keeping provider execution,
  store integration, platform adapters, child delivery, runtime report delivery,
  interception, child activity data, and app blocking unclaimed.
- App install/purchase report runtime proof contracts that link app-install
  report surfaces and child artifact refs to stateless report compiler
  status/result refs while keeping portal report UI, runtime report delivery,
  provider/store execution, platform adapters, child-device delivery, child
  activity data, app blocking, and Ocentra-hosted family data custody
  unclaimed.
- App install/purchase platform adapter boundary proof contracts that link
  approved API/entitlement evidence rows and report-runtime refs to platform
  adapter readiness/manual/unavailable rows while keeping actual adapter
  implementation, provider execution, store integration, child delivery, report
  delivery, interception, child activity data, app blocking, and Ocentra-hosted
  family data custody unclaimed.
- App install/purchase parent review action proof contracts that link
  approve/deny/time-box/review-needed decision actions to approved API/
  entitlement evidence refs and report-runtime refs while keeping portal
  approval UI, parent action runtime delivery, provider/store execution,
  platform adapters, child-device delivery, child activity data, app blocking,
  and Ocentra-hosted family data custody unclaimed.
- App install/purchase parent action runtime handoff proof contracts that link
  parent review actions to runtime handoff status rows and platform adapter
  boundary refs while keeping portal approval UI, runtime action writer
  implementation, parent action runtime delivery, provider/store execution,
  platform adapter implementation, child-device delivery, runtime report
  delivery, child activity data, app blocking, and Ocentra-hosted family data
  custody unclaimed.
- App install/purchase store status handoff proof contracts that link parent
  action runtime handoff rows and platform adapter boundary rows to per-store
  status handoff states while keeping provider/store execution, platform
  adapter implementation, parent action runtime delivery, child-device
  delivery, runtime report delivery, interception, child activity data, app
  blocking, and Ocentra-hosted family data custody unclaimed.
- App install/purchase runtime writer delivery proof contracts that link parent
  action runtime handoff rows and per-store status handoff rows to writer
  envelope/manual-required states while keeping runtime writer implementation,
  runtime writer delivery, parent action runtime delivery, provider/store
  execution, platform adapter implementation, child-device delivery, runtime
  report delivery, interception, child activity data, app blocking, and
  Ocentra-hosted family data custody unclaimed.
- App install/purchase runtime writer execution delivery proof contracts that
  link runtime writer delivery rows and parent action delivery readiness rows
  into deterministic parent-owned writer envelopes and result receipt rows
  while keeping provider/store execution, platform interception, platform
  adapter implementation, child-device delivery, runtime report delivery,
  install/purchase interception, app blocking, child activity data, and
  Ocentra-hosted family data custody unclaimed.
- App install/purchase runtime report writer delivery proof contracts that link
  runtime writer execution delivery receipts and report-runtime compiler output
  rows into parent-owned report delivery-ready rows and report receipts while
  keeping portal report UI, external runtime report delivery, provider/store
  execution, platform interception, platform adapter implementation,
  child-device delivery, app blocking, child activity data, and Ocentra-hosted
  family data custody unclaimed.
- App install/purchase package-source capture status proof contracts that link
  child package-source artifact refs and store status handoff rows to captured,
  blocked, manual-required, and unavailable capture rows with artifact, audit,
  report, and platform limitation refs while keeping provider/store execution,
  portal approval UI, platform adapters, child-device delivery, report delivery,
  custody, interception, app blocking, and Ocentra-hosted family data custody
  unclaimed.
- App install/purchase child-device delivery runtime writer proof contracts that
  link runtime writer delivery rows and package-source capture/status rows to
  child delivery envelope/manual-required rows while keeping writer execution,
  writer delivery, parent action runtime delivery, provider/store execution,
  platform adapters, child-device delivery, report delivery, custody,
  interception, app blocking, and Ocentra-hosted family data custody unclaimed.
- App install/purchase package-source adapter execution proof contracts that
  link package-source capture/status rows to local Windows, manual macOS,
  unavailable Linux, and blocked Android/iOS adapter execution states while
  keeping provider/store execution, portal approval UI, production platform
  adapters, child-device delivery, report delivery, custody, interception, app
  blocking, and Ocentra-hosted family data custody unclaimed.
- App install/purchase parent action delivery readiness proof contracts that
  link parent action runtime handoff rows to child-device delivery
  runtime-writer envelope rows while keeping parent action runtime delivery,
  runtime writer execution/delivery, provider/store execution, platform
  adapters, child-device delivery, report delivery, custody, interception, app
  blocking, and Ocentra-hosted family data custody unclaimed.
- App install/purchase provider/store execution readiness proof contracts that
  link approved API/entitlement evidence, store status handoff, package-source
  adapter execution, and parent action delivery readiness rows into
  execution-ready/manual/unavailable states while keeping Google Play, Apple App
  Store, Microsoft Store, billing/provider contact, provider/store execution,
  platform interception, platform adapters, child-device delivery, runtime
  writer delivery, app blocking, child activity data, and Ocentra-hosted family
  data custody unclaimed.
- App install/purchase approval/report domain proof contracts that link parent
  review action decisions and report-runtime refs into approval/report-ready or
  manual-review rows while keeping portal approval UI, portal report UI,
  runtime report delivery, provider/store execution, platform adapters,
  child-device delivery, interception, app blocking, child activity data, and
  Ocentra-hosted family data custody unclaimed.
- V0.9 signed LAN discovery/relay spine contracts that keep adapter evidence,
  signed proof rejection, route safety, relay/cache availability, parent-owned
  storage, and child-data custody claims explicit.
- V0.9 LAN source-matrix plan-completion contracts that expose all 20 LAN
  workpacks and discovery source rows with honest proof statuses and weak-source
  fences.
- V0.9 parent mobile controller/observer runtime proof contracts that expose
  read-only controller lease visibility, rejected observer writes, explicit
  local/LAN/relay/cache/storage route states, degraded and unavailable LAN AI
  provider handoff states, and no mobile child-agent parity claims.
- Browser/app/game/network/screen/tracking control catalogs.
- Screen settings portal proof contracts that summarize the Screen control
  catalog for read-only parent Settings rendering without claiming writable
  opt-in or retention controls.
- Android/iOS/platform proof and capability status contracts where product
  meaning belongs in TypeScript first.
- Tracking location policy, AI routing, acknowledgement, child check-in, alert
  intent, escalation, temporary live, and missing-device product contracts plus
  P1 acknowledgement/check-in runtime helper proof.

## Must Not Own

- Raw evidence payloads that belong in `activity-domain`.
- WebSocket envelopes that belong in `agent-protocol-domain`.
- Portal route/layout details.
- Platform adapter implementation.
- Billing provider SDK logic.

## Flow

```mermaid
flowchart LR
  Evidence["activity-domain evidence refs"]
  Rules["parent rules and schedules"]
  AI["local AI references"]
  Decision["policy decision"]
  Enforcement["enforcement action"]
  Audit["audit/report/assistant context"]

  Evidence --> Decision
  Rules --> Decision
  AI --> Decision
  Decision --> Enforcement
  Decision --> Audit
  Enforcement --> Audit
```

## Connected Docs

- [Policy expectations](../../docs/expectations/policy.md)
- [Enforcement expectations](../../docs/expectations/enforcement.md)
- [AI expectations](../../docs/expectations/ai.md)
- [Parent assistant expectations](../../docs/expectations/parent-assistant-chat.md)
- [LAN pairing expectations](../../docs/expectations/lan-pairing.md)
- [Platform expectations](../../docs/expectations/platforms.md)
- [Competitor capability map](../../docs/competitor-capability-map.md)

## Gaps To Fill

- Family setup, child profiles, co-parent roles, and recovery need complete
  product contracts and UI flow.
- Social/message/video controls need explicit product contracts, privacy
  boundaries, and platform source rules.
- Location/geofence/SOS/battery now has tracking contract proof plus P1
  acknowledgement/check-in helper proof; platform adapters, provider delivery,
  notification delivery, and live UI proof remain open.
- Store/install approval and purchase controls now have contract, package-source
  artifact, runtime-boundary, platform-artifact, child-artifact-delivery, and
  approved API/entitlement evidence plus report-runtime status, platform
  adapter boundary, parent review action proof, child status runtime readiness
  proof, parent action runtime handoff, store status handoff proof, and runtime
  writer delivery proof plus package-source capture status, child-device
  delivery runtime writer proof, adapter execution proof, and parent action
  delivery readiness proof plus provider/store execution readiness and
  approval/report domain proofs plus runtime writer execution delivery proof
  and runtime report writer delivery proof;
  platform/store provider execution, actual provider contact, actual platform
  adapters, production child-device package capture, production package-source
  capture adapter execution,
  runtime status reader, child delivery, portal UX, external runtime writer
  device delivery, parent action runtime delivery, and external report runtime
  delivery remain unimplemented.
- Billing/subscription provider integration, account backend, entitlement
  signing/delivery runtime, provider-contact execution, entitlement admin
  override runtime, refund/credit runtime, portal billing/admin UI, support
  backend upload, production subscription support, and child-device consumption
  remain unimplemented; current contracts keep billing outside core safety
  decisions, prove local status consumption of entitlement/device-limit/failure
  state, and keep support/admin actions manual-required or not implemented.
- `production-support-publication-workflow-proof` remains source-contract proof
  only; real public runtime, support backend upload execution, account lookup
  execution, billing provider contact, legal disclosure execution, production
  SLA, remote support sessions, and child activity custody remain unclaimed.
- `public-support-contact-status-proof` remains source-contract proof only;
  public runtime execution, support backend upload execution, account lookup
  execution, billing provider contact, legal disclosure execution, remote
  support sessions, production SLA, provider secrets, and child activity custody
  remain unclaimed.
- Parent-owned sync/export and stateless report compiler proofs remain
  contract/status proof only; real compiler runtime/cloud worker, connector
  OAuth/provider APIs, portal controls/UI, upload/download, deletion execution,
  custody/security review, and real storage/cache implementation remain
  unclaimed.
- Parent-owned local export/delete runtime proof remains read-model proof only;
  real filesystem writers, retention schedulers, delete executors, durable audit
  persistence, physical Windows smoke proof, and parent-visible controls remain
  unclaimed.
- V0.8 broad app, network/domain, exact URL, notification, and tamper controls
  remain manual-required or not-claimed until platform adapter proof exists.
- Policy-dispatch proof is currently service/read-model proof; finished
  parent/child UX, notification delivery, network/domain blocking, broad app
  blocking, and tamper protection remain proof-gated gaps.
- App/game policy preview handoff and timer-status proofs remain read-only
  contract/read-model proof. Timer-status rows classify whether future
  timer-runtime proof, source-freshness proof, or compiler-decision proof is
  still required before scheduling. Timer runtime-readiness rows record the
  service timer runtime, scheduler persistence, audit, and rollback proof still
  required before any future scheduling can be claimed. Timer
  scheduler-persistence rows add the durable scheduler state-store proof still
  required before scheduling while preserving source/compile blockers. Timer
  audit/rollback handoff rows add audit trail, rollback plan, and
  audit/rollback read-model proof still required before scheduling while keeping
  durable audit logs and rollback execution unclaimed. Timer audit/rollback
  read-model rows project those proof requirements into a parent-visible
  contract while keeping service read APIs, portal UI, durable audit storage,
  rollback execution, and scheduling unclaimed. Timer audit/rollback
  parent-surface intent rows add future proof and drill-in refs for the next
  UI/service seam while keeping rendered UI and service read APIs unclaimed.
  Timer service-readiness protocol handoff rows add future agent-protocol
  command/event, Rust protocol mirror, service handler, and service read-API
  proof refs while keeping protocol implementation, service command
  registration, service event emission, service read API implementation,
  rendered UI, durable audit storage, rollback execution, and timer scheduling
  unclaimed.
  Timer service-readiness protocol read-model rows project those same proof
  requirements into a future consumer-facing contract while keeping service
  read-model event emission and read API implementation unclaimed.
  Portal authoring or preview UI, live evaluator execution, persistence,
  timers, child notification delivery, adapter dispatch, broad installed-app
  blocking, and platform enforcement remain unimplemented.
- Supported-adapter and integrity runtime audit proof remain contract/read-model
  proof; broad app/domain/browser blocking, notification delivery, tamper
  resistance, mobile enforcement, stealth/persistence, and privilege escalation
  remain unclaimed.
- Integrity alert/status bridge proof remains notification intent/status and
  audit drill-in proof only; provider delivery, UI, anti-tamper resistance,
  broad blocking, mobile enforcement, stealth/persistence, and privilege
  escalation stay unclaimed.
- Signed LAN hello/heartbeat and physical household readiness remain
  manual-required until real second-child-agent artifacts are attached.
- Notification provider status boundary proof remains status/readiness
  contract/read-model proof only; notification local outbox proof remains
  deterministic parent-owned JSONL artifact proof only; notification local
  outbox scheduler proof covers deterministic due/held/retry/dead-letter/
  receipt/manual scheduler rows and parent-owned artifact write/read proof only.
  App/game preference-status handoff and parent-surface intent proofs remain
  parent-domain status/intent proof only. Provider adapters, provider receipts,
  delivered receipt ingestion, production retry workers, production quiet-hours
  timers, escalation delivery, parent controls/history/preferences UI, provider
  credentials, cloud routing, durable production outbox storage, child delivery,
  policy execution, adapter dispatch, broad blocking, and platform support
  remain unclaimed.
- LAN source-matrix proof is contract/read-model proof. It does not implement
  targeted ARP, bounded ARP sweep, packet listeners, real mDNS/SSDP
  advertisements, relay/cache, or physical household validation.
