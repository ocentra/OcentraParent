# Native Apps Plan Code/Test Audit

Date: 2026-08-15
Audited branch baseline: `codex/app-plan-code-audit` at `2b79285f0`
Integrated baseline: `main` at `eb4e66a79`; tree-equal `develop` back-sync at
`4ece51528` through PRs `#706`, `#707`, and `#708`.

## Authority

This is the code-first Phase 1 status for all 95 graph-imported native-app
workpacks. It was derived from tracked production source, actual call sites,
and checked-in tests. Markdown checkboxes, historical branch notes, and ignored
`output/` proof were not accepted as implementation evidence.

`Complete for bounded Phase 1` means the source and expected tests required by
that workpack's deliberately narrow contract, projection, handoff, or
coordination scope are written. It does not mean focused tests were rerun,
Enforcer accepted the slice, proof was regenerated, or the product flow is
release-ready. Those are Phase 2 and Phase 3.

## Result

- 95/95 workpacks have reviewed code/test topology in the engineering graph.
- 80/95 have no remaining source/test-writing gap in their bounded Phase 1
  scope. Many are intentionally narrow contract or handoff packets.
- 15/95 retain a concrete production-code or expected-test gap.
- Real Windows inventory, process, foreground, recurring service capture,
  encrypted journal/SQLite projection, source-status rows, scoped owned-process
  time-limit execution, and several Rust/service/portal read models exist.
- The former `packages/activity-domain`, `packages/parent-domain`,
  `packages/agent-protocol-domain`, and `packages/text-domain` App owners are
  absent from the tracked tree. The advertised `scripts/test/app-game-*` and
  `scripts/test/app-risk*` proof runners are also absent. Current authority is
  Rust-first plus generated TypeScript contracts.
- WP74-WP101 are mostly tested Rust projections that truthfully preserve
  no-runtime/no-enforcement boundaries. They are not service composition merely
  because their handoff rows exist.

## Workpack matrix

| Workpack | Current code/test evidence | Phase 1 | Remaining source/test gap |
| --- | --- | --- | --- |
| WP01 Contract boundary/effect schemas | `app-core`, Rust app/game protocol shapes, generated schema-domain contracts, and contract/invariant negatives cover the active boundary families. | **Complete for bounded Phase 1** | Broader product acceptance and proof remain later phases. |
| WP02 Source index/doc reconciliation | Current owners and missing legacy packages/scripts are reconciled in this audit and `source-index.md`. | **Complete for bounded Phase 1** | None in this docs-only scope. |
| WP03 Current snapshot/gap map | The live source/call-site gaps are recorded in `current-app-snapshot.md` and here. | **Complete for bounded Phase 1** | None in this docs-only scope. |
| WP04 App identity model | `AppGameIdentity`, merge proof, evidence identity validation, and focused protocol tests are written. | **Complete for Phase 1** | Live enrichment quality is a later product concern. |
| WP05 Installed inventory model | Typed inventory rows, category candidates, identity/capability states, and focused tests are written. | **Complete for Phase 1** | Platform acquisition is owned by WP06/WP07/WP41-WP46. |
| WP06 Windows installed inventory adapter | Real bounded Start Menu inventory source, journal events, service capture, and real-filesystem tests exist. | **Complete for Phase 1** | Phase 2 must rerun focused Windows tests. |
| WP07 Windows Store/UWP/AppX adapter | Real manifest/package source, identity matching, journal/service capture, and focused tests exist. | **Complete for Phase 1** | Store lifecycle and physical-host proof remain later. |
| WP08 Windows process runtime adapter | Real `sysinfo` process snapshots, opaque path refs, journal/service capture, and focused tests exist. | **Complete for Phase 1** | Process presence remains distinct from foreground/content. |
| WP09 Windows foreground adapter | Real active-window source, redacted refs, journal/service capture, and focused tests exist. | **Complete for Phase 1** | Physical-host proof remains Phase 3. |
| WP10 Cross-platform authority matrix | Typed authority/capability/manual-required rows and negative protocol tests exist. | **Complete for Phase 1** | Non-Windows platform execution is not implemented here. |
| WP11 Category/risk taxonomy | Category candidates, confidence/evidence state, classifier results, and tests exist in the Rust protocol/read model. | **Complete for Phase 1** | The live detector/catalog is WP17. |
| WP12 Sessionization/duration | Runtime and foreground sessionization, daily rollups, ordering logic, and unit tests are written. | **Complete for Phase 1** | Phase 2 must rerun replay/order tests. |
| WP13 Journal/SQLite ingest | Inventory/runtime/foreground/launcher journal records, SQLite projection, validation, and tests are written. | **Complete for Phase 1** | Proof/corruption runs are later. |
| WP14 Read models/service events | App-use/game read models, service payloads/reports, source rows, and focused service tests exist. | **Complete for Phase 1** | Downstream product surfaces remain separately owned. |
| WP15 Portal inventory/running/session surfaces | Service rows and several App/Game proof panels exist. | **Incomplete** | No cohesive parent surface renders installed inventory, running, foreground, recent sessions, rollups, unknown/risk cards, evidence drill-in, and malicious/long metadata states with the expected tests. |
| WP16 New/unknown approval flow | Approval authority/request/decision/action-result DTOs and projection tests exist. | **Incomplete** | No production candidate creator, durable one-shot/persistent approval lifecycle, replay/expiry owner, parent/child UI, or integrated tests exist. |
| WP17 Risk app detection | Taxonomy and classifier-result evidence shapes exist. | **Incomplete** | The former detector/catalog implementation is gone; no live known/unknown risk classifier, enrichment path, or focused detector matrix exists. |
| WP18 Policy target compiler | Generated target/request/decision schemas and drift tests exist. | **Incomplete** | There is no current compiler algorithm or tests for proof gating, stale/wrong-device rejection, dry-run, and manual-required hard actions. |
| WP19 Time budget/schedule/bonus integration | Generic policy schedule/bonus primitives and scoped timer execution exist. | **Incomplete** | No native-app composition consumes authoritative sessions plus schedule/bonus/allow-once state through compile, runtime, restart, and expected integration tests. |
| WP20 Child warning/block/request UX | Protocol/manual states and parent timer surfaces exist. | **Incomplete** | No live child app warning/request UI, overlay/runtime delivery, persistence, or state-transition tests exist. |
| WP21 Windows owned-process terminate/time limit | Owned-process target validation, timer execution/recovery/cancel, evidence binding, service dispatch, rollback states, and focused tests exist. | **Complete for Phase 1** | This is not broad package/app blocking. |
| WP22 Broad-blocking proof gates | Coordination/no-claim packet correctly leaves broad controls manual-required. | **Complete for bounded Phase 1** | Phase 3 platform proof is still absent. |
| WP23 AI classifier digest boundary | Evidence refs, digest/confidence bounds, evidence-only authority, serialization, and negative tests exist. | **Complete for Phase 1** | Provider execution/quality is outside this boundary. |
| WP24 Platform extension routing | Coordination-only routing correctly keeps unsupported platform actions manual-required. | **Complete for bounded Phase 1** | Platform owners must supply real adapters/proof. |
| WP25 Install/uninstall approval handoff | The bounded handoff can cite inventory/install/uninstall/tamper evidence without inventing adapter authority. | **Complete for bounded Phase 1** | Store/package-manager interception and approval UX are adjacent product work. |
| WP26 Performance/service health | Capture cadence/limits and degraded states have ordinary tests. | **Incomplete** | The specified 1k-app, 500-process, 100k-observation, 1k-policy, and 500-row portal performance/load harnesses do not exist. |
| WP27 E2E/manual proof artifacts | Proof-routing packet owns no product source. | **Complete for bounded Phase 1** | Phase 3 scenarios remain to be generated. |
| WP28 Rollout/PR gate | Coordination-only rollout packet owns no product source. | **Complete for bounded Phase 1** | It cannot close until Phase 2/3 acceptance is complete. |
| WP29 Rust evidence/identity parity | Shared Rust evidence, identity, merge, AI digest shapes and protocol tests exist. | **Complete for Phase 1** | Runtime enrichment is separately owned. |
| WP30 Rust authority classifier parity | Approval/action/platform/classifier DTOs and negative tests exist. | **Complete for Phase 1** | Live authority acquisition is separately owned. |
| WP31 Journal/SQLite classifier storage | Staged authority/action/platform/classifier rows replay and project through ActivityStore with focused tests. | **Complete for Phase 1** | Live producers remain separate work. |
| WP32 Live process source | Real bounded process snapshot source and opaque-path tests exist. | **Complete for Phase 1** | None in this source-only scope. |
| WP33 Process journal/SQLite bridge | Process rows append/project/query through the journal/SQLite path with tests. | **Complete for Phase 1** | None in this bounded bridge scope. |
| WP34 Service live-process capture | Service capture invokes the real process source and persists/query-projects rows with tests. | **Complete for Phase 1** | None in this bounded bridge scope. |
| WP35 Recurring freshness | Service startup starts recurring bounded capture and freshness rows have focused tests. | **Complete for Phase 1** | Operational/long-duration proof is later. |
| WP36 Live foreground source | Real active-window source and privacy/no-content tests exist. | **Complete for Phase 1** | None in this source-only scope. |
| WP37 Service foreground capture | Service capture invokes the foreground source and persists optional rows with tests. | **Complete for Phase 1** | None in this bounded bridge scope. |
| WP38 Authority classifier service surface | Service models expose staged authority/classifier evidence with focused tests. | **Complete for Phase 1** | This is projection, not live classifier execution. |
| WP39 Classifier read-model counts | Explicit authority/action/platform/classifier counts and refs are projected and tested. | **Complete for Phase 1** | None in this projection scope. |
| WP40 Boundary read-model event | Typed service event/payload exposes bounded app/game evidence counts and refs with tests. | **Complete for Phase 1** | None in this event scope. |
| WP41 Live Windows inventory source | Real bounded shortcut inventory acquisition and filesystem tests exist. | **Complete for Phase 1** | None in this source-only scope. |
| WP42 Service inventory capture | Service capture invokes and persists Windows inventory rows with tests. | **Complete for Phase 1** | None in this bounded bridge scope. |
| WP43 Live Windows Store source | Real package-manifest source and tests exist. | **Complete for Phase 1** | None in this source-only scope. |
| WP44 Service Store capture | Service capture invokes/persists Store package rows with tests. | **Complete for Phase 1** | None in this bounded bridge scope. |
| WP45 Live registry inventory | Real uninstall-registry acquisition plus export/test seam and tests exist. | **Complete for Phase 1** | None in this source-only scope. |
| WP46 Service registry capture | Service capture invokes/persists registry inventory rows with tests. | **Complete for Phase 1** | None in this bounded bridge scope. |
| WP47 Backend source freshness | Service read models derive per-source row counts, freshness, capability, timestamps, and evidence refs with tests. | **Complete for Phase 1** | Policy consumption is separately owned. |
| WP48 Portal source freshness | Generated portal contracts decode `sourceStatusRows`. | **Incomplete** | Current portal source does not render source-kind freshness/capability/evidence rows or the specified empty/stale/degraded UI tests. |
| WP49 Category/risk policy routing | Target schemas and source-gating models exist. | **Incomplete** | The former routing implementation/tests are absent; no current compiler routes category/risk/AI candidates with stale/manual/no-adapter negatives. |
| WP53 Notification intent contract | Rust notification-readiness rows cover time-limit, approval, suspicious-unknown, manual, unavailable, redaction, and no-delivery claims with tests. | **Complete for Phase 1** | Delivery starts at WP58. |
| WP54 Policy-readiness portal renderer | Rust-owned policy readiness is rendered in the portal with empty/manual states and focused tests. | **Complete for Phase 1** | Authoring/mutation remains elsewhere. |
| WP56 Notification service read model | Agent service builds and reports notification-readiness rows; payload/service tests exist. | **Complete for Phase 1** | It explicitly reports provider/outbox/scheduler delivery as unclaimed. |
| WP58 Notification local outbox | Rust readiness-row bridge creates canonical local-outbox records, reuses the atomic store, round-trips deterministic JSONL, persists/reopens/replays idempotently, rejects conflicts, and excludes manual/unavailable rows; service regression keeps the unrelated setup outbox from claiming WP58 runtime. | **Complete for bounded Phase 1; Phase 2 passed** | Phase 3 proof and live service/provider composition remain open; no delivery or receipt claim is made. |
| WP59 Notification scheduler | Shared Rust WP58-to-scheduler bridge, canonical per-record scheduler route/store, and contract tests. | **Complete for bounded Phase 1; Phase 2 green** | The bridge validates WP58, schedules linked rows only, retains blocked manual/unavailable rows, round-trips deterministic scheduler JSONL, and proves atomic reopen/idempotency/conflict behavior at `4cf6a11c9`. Production quiet-hours/retry execution, provider delivery, receipts, and retained proof remain outside this bounded bridge. |
| WP60 Notification audit history | Shared Rust WP58-to-audit-history bridge/read model plus focused tests. | **Complete for bounded Phase 1; Phase 2 green** | Ordered queued/manual/unavailable metadata entries preserve audit/evidence/policy refs, deterministic JSONL includes blocked rows, and tampered refs/claims/identities fail at `bae505ce8`. Durable production history/query and delivery runtime remain later boundaries. |
| WP61 Provider preflight | Shared Rust per-record provider preflight validates persisted scheduler/outbox identity and models adapter/credential/smoke requirements. | **Incomplete** | The safe per-record boundary and focused negative matrix exist, but no WP61 owner consumes the complete WP59 scheduler read model, generates deterministic per-row requirements, or preserves manual/unavailable blocked rows as one native-app read model. Provider execution and credential custody remain non-goals. |
| WP62 Preference preflight | Preference-required state is representable. | **Incomplete** | No durable parent preference/quiet-hours/frequency preflight owner or mutation/replay tests exist. |
| WP63 Source panel polish | Generated source rows exist. | **Incomplete** | The portal source-status panel/polish and its focused UI tests are absent. |
| WP64 Provider status handoff | Provider-status input DTOs exist inside the parent-surface model. | **Incomplete** | No builder/producer derives provider status from attempts/receipts and no dedicated tests prove the handoff. |
| WP65 Preference status handoff | Preference-status input DTOs exist inside the parent-surface model. | **Incomplete** | No builder/producer derives preference status from durable settings/delivery outcomes and no dedicated tests prove it. |
| WP66 Notification parent-surface intent | Redacted provider/preference status projection, no-claim flags, mismatch negatives, and generated drift test exist. | **Complete for Phase 1** | Its upstream WP64/WP65 producers remain missing. |
| WP67 Notification renderer | Parent-runtime snapshot and portal renderer show Rust-owned readiness/manual states with focused tests. | **Complete for Phase 1** | Provider delivery and preference mutation remain false. |
| WP74 Source freshness policy consumption | Canonical generated contract validates app inventory/runtime/foreground freshness, app/game separation, evidence refs, and manual-required fallback with tests. | **Complete for bounded Phase 1** | No policy runtime/enforcement claim. |
| WP75 Source freshness preview gate | Rust gate builder and contract negatives are written. | **Complete for bounded Phase 1** | No service/portal/runtime claim. |
| WP76 Source-gated preview read model | Rust redacted projection and tests are written. | **Complete for bounded Phase 1** | No service/portal/runtime claim. |
| WP78 Timer handoff | Tested Rust handoff readiness projection exists. | **Complete for bounded Phase 1** | It does not start a timer. |
| WP79 Timer status | Tested Rust status classification exists. | **Complete for bounded Phase 1** | It does not schedule a timer. |
| WP81 Timer runtime readiness | Tested Rust readiness projection exists. | **Complete for bounded Phase 1** | Runtime execution remains separately owned. |
| WP82 Scheduler persistence status | Tested Rust projection records missing scheduler/durable-state requirements. | **Complete for bounded Phase 1** | It intentionally does not provide scheduler persistence runtime. |
| WP83 Audit/rollback handoff | Tested Rust projection records audit/rollback requirements. | **Complete for bounded Phase 1** | No audit/rollback execution claim. |
| WP84 Audit/rollback read model | Tested Rust read-model projection exists. | **Complete for bounded Phase 1** | No durable audit runtime claim. |
| WP85 Audit/rollback parent intent | Tested Rust parent-intent projection exists. | **Complete for bounded Phase 1** | No parent runtime/UI claim. |
| WP86 Service readiness handoff | Tested Rust service-readiness handoff exists. | **Complete for bounded Phase 1** | No service API claim. |
| WP87 Service readiness read model | Tested Rust service-readiness projection exists. | **Complete for bounded Phase 1** | No protocol/service runtime claim. |
| WP88 Protocol handoff | Tested Rust protocol-handoff projection exists. | **Complete for bounded Phase 1** | It does not add a wire command/event. |
| WP89 Protocol read model | Tested Rust protocol read-model projection exists. | **Complete for bounded Phase 1** | It does not expose a service endpoint. |
| WP90 Protocol command handoff | Tested Rust command-handoff requirement projection exists. | **Complete for bounded Phase 1** | No agent command is claimed by this row. |
| WP91 Service handler handoff | Tested Rust handler-handoff requirement projection exists. | **Complete for bounded Phase 1** | No service handler is claimed by this row. |
| WP92 Read API handoff | Tested Rust read-API requirement projection exists. | **Complete for bounded Phase 1** | No endpoint is claimed by this row. |
| WP93 Read API response handoff | Tested Rust response requirement projection exists. | **Complete for bounded Phase 1** | No response runtime is claimed by this row. |
| WP94 Response consumer handoff | Tested Rust consumer requirement projection exists. | **Complete for bounded Phase 1** | No consumer runtime is claimed by this row. |
| WP95 Parent-surface handoff | Tested Rust parent-surface handoff projection exists. | **Complete for bounded Phase 1** | No rendering claim. |
| WP96 Parent-surface read-model handoff | Tested Rust handoff projection exists. | **Complete for bounded Phase 1** | No parent runtime claim. |
| WP97 Parent-surface status handoff | Tested Rust status handoff exists. | **Complete for bounded Phase 1** | No service/portal status runtime claim. |
| WP98 Parent-surface status read model | Tested Rust status read-model handoff exists. | **Complete for bounded Phase 1** | No service/portal claim. |
| WP99 Status-read-model parent-surface handoff | Tested Rust handoff exists. | **Complete for bounded Phase 1** | No renderer claim. |
| WP100 Parent-surface read-model handoff | Tested Rust handoff exists. | **Complete for bounded Phase 1** | No service/portal claim. |
| WP101 Parent-surface read model | Tested Rust terminal projection exists. | **Complete for bounded Phase 1** | It remains a pure model, not live composition. |
| WP102 Parent-surface read-model service handoff | No current implementation matches the documented builder/test after legacy parent-domain removal. | **Incomplete** | Add the bounded Rust service-handoff model/test or retire/merge this redundant packet into the actual WP103 service model. |
| WP103 Timer service read model | Typed protocol model and agent-service builder/report with tests exist. | **Complete for Phase 1** | Runtime acceptance remains Phase 2. |
| WP104 Timer service event | Agent service emits the typed timer parent-surface report/event with tests. | **Complete for Phase 1** | Transport proof remains later. |
| WP105 Timer service read API | Activity API command/report routing for the timer surface is written and tested. | **Complete for Phase 1** | Phase 2 must rerun focused service tests. |
| WP106 Timer read API response | Typed payload/response encoding and negative tests exist. | **Complete for Phase 1** | Transport proof remains later. |
| WP107 Timer response consumer | Parent-runtime consumes the Rust service snapshot into the UI bridge with integration tests. | **Complete for Phase 1** | Live desktop proof remains later. |
| WP108 Timer parent-surface handoff | Portal route renderer and focused tests consume the parent bridge snapshot. | **Complete for Phase 1** | Physical/UI proof remains Phase 3. |

## Highest-impact implementation order

1. WP18 + WP49: restore one Rust-owned policy compiler/routing path. Without
   it, live source evidence cannot become authoritative native-app policy.
2. WP16 + WP17: create durable unknown/new-app review and live risk-candidate
   production, feeding the compiler rather than presentation-only DTOs.
3. WP19 + WP20: compose authoritative sessions, schedules, bonus/allow-once,
   child warning/request UX, restart recovery, and focused integration tests.
4. WP61-WP65: implement provider/preference ownership and status,
   provider/preference preflight, and status producers already expected by the
   WP66/WP67 parent surface.
5. WP15 + WP48 + WP63: finish the cohesive inventory/running/foreground/session
   and source-freshness parent UI with security/large-metadata tests.
6. WP26: add the specified load/performance harnesses.
7. WP102: either implement the remaining bounded service-handoff model or
   delete/merge the redundant packet with WP103 through an explicit plan edit.

## Release interpretation

The native-app plan has a strong Windows observation/storage spine and a real
scoped time-limit adapter path, but it is not release-ready. The central
missing chain is:

```text
live evidence -> durable review/risk state -> policy compiler/runtime
-> child request UX -> durable notification delivery/history
-> complete parent inventory/freshness surfaces
```

Phase 2 must run focused tests and Enforcer only after the 15 writing gaps are
closed or explicitly reduced. Phase 3 then regenerates proof from a clean
checkout; historical ignored proof is not a substitute.
