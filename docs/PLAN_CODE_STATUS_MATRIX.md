# Project Progress Matrix

This is the code-backed execution dashboard for Ocentra Parent. It supplements
`PLAN_INDEX.md`; it does not replace plan-local workpacks, proof roots, or
checklists.

Last broad source inventory: 2026-08-15, integrated to `main` at
`eb4e66a79` and back-synced to `develop` at `4ece51528` with tree-equal
contents. The live source-integration branch is
`codex/eventing-wp09-production`; the latest accepted production-source commit
covered by this matrix is `fa1230661`. Source checkpoints on this branch are
not thereby fully validated, PR-ready, or merged to `develop`/`main`. Current
branch, worktree, archive, and promotion custody is recorded in
`docs/REPOSITORY_CUSTODY_STATUS.md`.

## Completion operating contract - 2026-08-17

The `P/B/R/A/V/D` graph states below are scheduling metadata, not the requested
completion answer. For every workpack, completion work is driven from its
intended product behavior and the live call path:

1. identify what the workpack promises, what production entrypoint reaches a
   trusted caller today, and what code is missing or was not knowable when the
   plan was drafted;
2. write the complete coherent production-source packet for that owner boundary;
3. only after that source packet is stable, write all expected positive,
   negative, restart, replay, authority, and compatibility test source;
4. run and repair focused tests, then focused Enforcer and architecture checks
   for the touched workpack/crate/domain;
5. after all planned source and test source is written, run repo-wide tests and
   Enforcer once, regenerate proof, run precommit, open one coherent PR, pass
   CI, and promote normally through `develop` to `main`.

Writing a line and immediately launching a broad test cycle is not this
program's workflow. DTOs, constants, projections, fake adapters, caller-minted
authority, mocks, and tests that merely bless disconnected code do not count as
production completion. A source lane either lands a reachable material effect
or reports the exact upstream owner that must be implemented first.

The detailed per-workpack gap register is the plan table in this document,
except for the four large plans whose complete rows live in
`docs/plans/ai-plan/CODE_AUDIT.md`, `docs/plans/app-plan/CODE_AUDIT.md`,
`docs/plans/app-game-plan/CODE_AUDIT.md`, and
`docs/plans/tracking-plan/CODE_AUDIT.md`. The 683-row `graph:matrix` remains the
topology/dependency join; it must not overwrite the code-reachability finding.

## Live executable topology matrix - 2026-08-17

This table is generated from the validated engineering graph and its reviewed
code map, not from plan checkboxes. The graph currently contains 707 nodes, 802
edges, 23 plans, and 683 workpacks. It maps 3,060 implementation files and 1,146
test files: 587 workpacks have both code and tests, 4 are source-only, 8 are
tests-only, and 84 currently have no source or test files. Of those 84, 82 are
expected no-code coordination/reference packets; Data WP09/WP10 instead require
missing code-and-tests roots. No workpack has unknown ownership. Topology
expectations match for 665/683 workpacks.
Derived state is 360 planned, 35 blocked, 5 ready, 8 active, 274 in
validation, and 1 done.

These counts prove file ownership and expected topology only. They do not prove
that the mapped code is production-reachable, that the tests cover the workpack,
or that either passes. Only one workpack currently satisfies the complete graph
contract; 682/683 still lack recorded reviewed completion evidence.

Legend: `C+T/S/T/N/U` = code-and-tests / source-only / tests-only / no-source /
unknown. State counts use `P/B/R/A/V/D` = planned / blocked / ready / active /
validation / done.

| Plan | WPs | C+T/S/T/N/U | Impl/Test files | Planned / Blocked / Ready / Active / Validation / Done | Highest-impact current gap |
| --- | ---: | ---: | ---: | ---: | --- |
| Account identity/family | 8 | 7/0/0/1/0 | 104/32 | 0/6/0/0/2/0 | Source checkpoint `1cf0742c9`: accepted sealed authority/repository source remains, but WP02 target child/device resolution is conflated for several actions; provider/account route composition, expected tests, proof, and DONE remain open. |
| AI | 48 | 44/0/0/4/0 | 195/98 | 46/0/0/0/2/0 | All 48 lack retained/referenced completion proof; mapped code is not whole-chain acceptance. |
| App/game | 220 | 201/0/0/19/0 | 2369/867 | 126/2/0/2/90/0 | Two workpacks are dependency-blocked and the plan remains integration-open. |
| Browser | 30 | 22/0/0/8/0 | 145/45 | 30/0/0/0/0/0 | All workpacks remain planned; proof and several native/runtime authority paths are open. |
| Child-agent runtime distribution | 11 | 10/0/0/1/0 | 79/17 | 8/0/0/0/3/0 | Distribution/runtime proof remains unreferenced across the plan. |
| Cloudflare control plane | 13 | 9/0/1/3/0 | 186/63 | 10/2/0/0/1/0 | WP01's bounded scaffold is in validation. WP06's bounded durable adapter/auth source packet is independently accepted, but provider verification, migration, tests, proof, and deployment remain open; normal WP06 is blocked and not DONE. |
| Data custody/storage | 9 | 8/0/0/1/0 | 971/482 | 1/1/3/0/4/0 | WP05 backup/migration, WP06's thin TypeScript edge, and WP08 confirmation authority are READY source packets; WP07 is blocked on the Account WP04/WP05 trusted export/delete chain. Tests, proof, and DONE remain open. |
| Device trust bootstrap | 9 | 6/0/3/0/0 | 108/41 | 0/2/1/0/6/0 | Accepted Device source is integrated through `68717b5b7`: WP01 is a durable foundation/current-binding source only, while WP05 unsigned entitlement, WP06 fail-closed restore authority, and WP07 durable removal/readiness remain bounded source. No shipped authority issuer/caller exists. The next owner route is Account WP08 canonical binding -> Cloudflare WP06 current-authority bridge -> WP03 ceremony; WP02 is a conditional sealing/lifecycle-revocation gate only when that platform path is selected, then LAN/child consumers follow WP03. Platform/passkey ceremony, issuer/revocation provider, restore/platform callers, expected tests, proof, and completion remain open. |
| LAN | 26 | 22/1/2/1/0 | 305/63 | 0/1/0/0/25/0 | WP26 is source-only with zero mapped tests and is ordered after Device Trust WP03: it consumes WP01 current binding/revocation and cannot register signer authority locally. |
| Logging domain parity | 10 | 9/0/0/1/0 | 141/59 | 5/0/0/0/5/0 | Accepted source wave establishes Rust-owned exact 18-key redaction policy and generated TS parity; tests, proof, and external composition remain open. |
| App | 95 | 88/1/0/6/0 | 258/115 | 13/0/0/0/82/0 | One source-only mismatch and remaining compiler/runtime/native gaps. |
| Network | 8 | 8/0/0/0/0 | 349/88 | 6/1/0/0/1/0 | WP04 remains dependency-blocked despite complete file topology. |
| Parent desktop/runtime package | 11 | 10/0/0/1/0 | 131/32 | 4/0/0/2/5/0 | Signed package/update/rollback and retained release proof remain open. |
| Payment/subscription | 13 | 11/0/0/2/0 | 90/53 | 4/5/0/4/0/0 | Accepted source closes caller-minted entitlement, in-memory DO, provider-identity, and pending-recovery defects; Account migration, real issuer/provider authority, and expected tests remain open. |
| Policy control plane | 8 | 7/0/0/1/0 | 210/46 | 1/4/0/0/3/0 | Four workpacks are blocked on trusted source, Device Trust, and delivery owners. |
| Portal UX/household surfaces | 20 | 17/0/1/2/0 | 95/67 | 15/0/0/0/5/0 | One tests-only mismatch; upstream live authority remains absent for several surfaces. |
| Remote access | 6 | 5/0/0/1/0 | 40/22 | 4/0/0/0/2/0 | Authenticated relay/session runtime and proof remain open. |
| Eventing | 13 | 10/0/1/2/0 | 193/112 | 0/2/1/0/9/1 | WP11 production source is independently accepted through `fa1230661`: envelope/request/journal/replay authority is fail-closed and single-use. Its old replay tests and required negative/audit families remain for the test-writing wave; WP10/WP12 stay blocked. |
| Screen AI pipeline | 10 | 7/0/0/3/0 | 150/43 | 9/1/0/0/0/0 | One blocked workpack and unresolved policy/custody authority chain. |
| Screen | 43 | 29/2/0/12/0 | 230/80 | 25/0/0/0/18/0 | Two source-only mappings plus platform/runtime/custody gaps. |
| Setup/install/provisioning | 7 | 2/0/0/5/0 | 29/10 | 0/2/0/0/5/0 | Rollout gate is blocked; trusted readiness aggregation remains incomplete. |
| Tracking | 43 | 36/0/0/7/0 | 134/82 | 41/2/0/0/0/0 | WP40 is now reviewed as an 18-source/2-test composition owner; the shipped caller that connects child tracking flow to the durable journal/ActivityStore is still absent. |
| V0.8 enforcement | 20 | 19/0/0/1/0 | 94/72 | 13/1/0/0/6/0 | WP04 remains blocked on the enforcement audit-journal owner. |

All rows describe the current integration checkout. Git does not encode
per-plan branch custody; the safe pushed checkpoint and any active local source
batch are recorded separately in the repository custody status. Use
`npm run graph:matrix -- --json` for all 683 workpack rows.

## Authoritative production reachability dashboard - 2026-08-16

This section supersedes every older Phase-1 count, checkbox summary, graph
state, and historical completion sentence below. It answers a narrower
question: does a shipped entrypoint reach a real caller with trusted input and
produce a material product effect? A mapped file, DTO, status/read-model
panel, mock, fixture, `TEST_*` adapter, synthetic reference/outcome, generated
proof artifact, or shape-only test does not count as product implementation.
Phase 2 tests/Enforcer and Phase 3 proof are not claimed here.

| Plan | Exact workpacks | Actual shipped caller / material effect | Principal production-code gap | Honest production phase state |
| --- | ---: | --- | --- | --- |
| Account identity/family | 8 | Family/setup/provisioning helpers remain reachable. Accepted source through `e69acf279` adds Rust-owned schema validation, a sealed current Account capability, durable repository/CAS, session and invite/recovery records, Cloudflare D1 adapter/migrations, and Account-bound billing/support consumers. | WP02 still conflates actor parent-controller and target child/device for Pair/Register/Revoke/View/ChangePolicy/Remote actions; target-aware resolver, capability/lease/step-up binding, provider-to-authority caller, route composition, and expected tests remain unresolved. | WP02-WP05/WP08 source accepted with WP02 review reopened; composition, tests, proof, and DONE remain open. |
| AI | 48 | Local chat/assistant and screen-analysis paths can reach the configured local runner; deterministic activity classifiers and typed status/read models exist. | No neutral durable AI work-item lifecycle, complete custody context, verified provider/model authority, or trusted AI-to-policy handoff. | Incomplete; runtime foundations exist without a complete product chain. |
| App/game | 220 | `service_runtime` captures Windows inventory/process/foreground/launcher observations into encrypted journal/SQLite and agent-service read models. | Parent-authored compiler/evaluator caller, approval service composition, scheduler, child delivery/provider, and authenticated adapter execution are absent; timer is only PID/name scoped. | Incomplete/blocked; no legal source slice found. |
| App | 95 | Agent-service/app-game-core read models, risk readiness, route/compiler composition, and portal projections are reachable. | Live OS/app policy authority, service evaluator runtime, notification/child delivery, durable timer execution, and native action owner are missing. | Incomplete/manual-required. |
| Browser | 30 | Windows inventory, managed launch/profile custody, CDP target-list evidence, service events, and portal status paths are reachable. | Active-focus authority, cross-platform inventory, trusted intervention delivery, AppLocker/WDAC, native-host lifecycle, and authenticated action receipts are missing. | Incomplete; native authority gaps remain. |
| Child-agent runtime distribution | 11 | A real `ocentra-child-agent-service` binary constructs durable journal/tombstone/removal stores, recovers before readiness, and fails closed for pending recovery, tamper, or revoked state. The shipped Android Activity/foreground-service/JNI path starts and stops that Rust service. | Android still has no live event-submission transport, supervised command consumer, verified trust-identity input, ongoing bound health consumer, or APK/device lifecycle proof. Signed distribution, platform install/permission ownership, rollback, and complete multi-platform release remain open. | Real service/startup foundation; production delivery and distribution remain incomplete. |
| Cloudflare control plane | 13 | Worker route dispatcher reaches billing/admin/webhook and auth-verifier boundaries; WP01's bounded scaffold and WP06's source adapter/auth chain passed independent source review, with storage/migration configuration present. | Provider verification is manual-required, the account D1 migration is unapplied, tests/proof/deployment are open, fixture paths cannot authorize production, and deployed binding ownership is absent. | WP06 bounded source accepted; normal runtime remains blocked/manual-required and WP06 is not DONE. |
| Data custody/storage | 9 | The shipped child-service startup opens the durable journal/effect/tombstone stores, recovers them before readiness, and routes `PublishStorageCustody` through dynamic Device Trust readiness into `ChildStorageCustodyRuntime::execute`. | Default composition deliberately installs manual-required custody authority. The real missing chain is Account WP08 current authority → WP04 correlated export/delete lifecycle → WP05 account/session/device/capability/lease composer → a private child-runtime adapter. WP05 backup/migration source and WP06's thin TypeScript edge also remain open. | Real internal custody lifecycle; trusted external authority composition, expected tests, proof, and DONE remain open. |
| Device trust bootstrap | 9 | Typed challenge/presence/identity contracts and local bootstrap state are reachable in bounded flows. Accepted source through `68717b5b7` adds owner-resolved current binding, an unsigned entitlement projection, fail-closed restore authority, and durable trust-bound removal readiness. WP01 is foundation/source only. | No shipped parent/device enrollment or passkey caller, authority issuer, entitlement issuer/revocation provider, restore executor/key-custody composition, parent transport, or platform removal owner. The full expected-test wave and functional validation remain open. | Source checkpoint accepted; plan remains open/manual-required until Account WP08 -> Cloudflare WP06 current-authority, WP03 ceremony, conditional WP02 sealing/revocation, and post-WP03 LAN/child consumers are real and separately validated/proven. |
| Eventing | 13 | Shared event contracts, journal append/replay, and ActivityStore projections are used by selected agent-service/runtime paths. | Generic eventing does not supply every domain’s durable policy dispatch, trusted authority, replay/idempotency, or receipt lifecycle. | Incomplete; foundational runtime only. |
| LAN | 26 | Agent-service discovery/pairing commands and parent read paths produce bounded network observations and fail-closed state. | WP26 has a partial custody/registry/transport draft but still needs a shipped signed-child beacon ingress, durable receipt/idempotency custody, selected non-revoked route composition, and private Eventing WP10 authority handoff. It is ordered after Account WP08 -> Cloudflare WP06 -> Device Trust WP03 one-time `RegisterLanSignerAnchor` authorization and consumes WP01 current binding/revocation; physical proof remains later. | Blocked; no service route or completion claim. |
| Logging domain parity | 10 | Rust logging markers and structured event fields are emitted by service/runtime paths; the accepted source wave centralizes sensitive-key redaction in Rust with generated TS consumption and pre-serialization sanitization in Logger/portal fallback. | External product/runtime composition, enforcement instrumentation, durable audit custody, and executable abuse/lifecycle coverage remain incomplete; tests and proof are deferred. | Incomplete; observability is not control authority. |
| Network | 8 | Agent-service/agent-core discovery adapters and ActivityStore/network read models produce bounded visibility evidence. The shipped read API preserves real observations and runtime-delivery results. | The fabricated product-path caller, payload fields, bridge, and disconnected pipeline were removed in `9e9f9ac51`; tests that imported or blessed those APIs must be deleted or rewritten against future real owners. No live analyzer-to-policy handoff, blocking adapter, durable rule authority, or provider/runtime execution exists. | Fail-honest cleanup landed; WP04 and the plan remain incomplete. |
| Parent desktop/runtime package | 11 | Tauri route commands and provisioning/install-state contracts are reachable from the desktop shell. | Signed package/build/update/rollback, integrity verification, installer distribution, and release publishing callers are absent. | Blocked on runtime-distribution ownership. |
| Payment/subscription | 13 | Cloudflare billing routes/read models now reach durable provider-mapping, lease/retry/outbox, receipt/CAS, and stale-cursor paths; entitlement output is explicitly unsigned. | Account authority migration must land first, and no genuine provider/entitlement issuer-verifier path exists; expected tests are still open. | Production source accepted; dependency/manual-required and test wave open. |
| Policy control plane | 8 | Rust policy source/validator/compiler contracts, decision/read models, and service boundary guards exist. | Source authority is caller-supplied, source registration and domain compilers have no shipped caller, confirmation persists an audit row rather than an active policy source, and durable compile/delivery/rollback authority remains missing. | Contract-drafted; production source and compiler composition are open. |
| Portal UX/household surfaces | 20 | Portal routes render Rust/service snapshots and send typed parent intents. | Portal owns no account, child, custody, policy, notification, or native execution authority; many upstream live callers are absent. | Projection-only/incomplete. |
| Remote access | 6 | Typed remote route/capability/status boundaries and local read models are present. | No authenticated relay/session owner, durable authorization, transport provider, or production remote execution path is wired. | Blocked/manual-required. |
| Screen AI pipeline | 10 | Screen startup/capture, encrypted queue, local adapter process, ActivityStore summaries, and portal status are reachable; queue completion requires successful downstream handlers and no dead letters. Source checkpoint `7dc09c25f` removes the synthetic policy decision/action/rule/explanation values and forces AI eligibility false until a trusted decision exists. | Policy Control still needs a real resolver/handoff; canonical AI routing, durable replay, custody-negative enforcement, and rewritten fail-closed tests remain missing. | Fail-honest source correction integrated; WP04 and the plan remain open. |
| Screen | 43 | Desktop capture, Android MediaProjection entrypoint, encrypted image queue, deletion/read models, and portal settings/status are reachable. | Unified platform capability/permission owner, real OCR/VLM composition, policy authority, child disclosure/live view, and complete custody lifecycle are missing. | Incomplete/manual-required. |
| Setup/install/provisioning | 7 | Rust-owned WP07 Start-route source now exposes all 15 owner inputs as manual-required/unavailable; `AgentCommandName::is_lan_command` canonically classifies 13 LAN commands and generic parent dispatch rejects LAN commands on non-LAN routes. | No authenticated account/session/household, signed parent package, child package/service/permission, device-trust, trusted pairing, custody sync, policy baseline, network, or recovery aggregation reaches setup progression. LAN selected/paired/reachability state is observation-only; Start does not invoke provisioning evaluation or action planning. | Source integrated through `ca230550b`; tests, builds, proof, precommit, CI, and PR are deferred; route remains manual-required/unavailable. |
| Tracking | 43 | Typed config/check-in/geofence flows and ActivityStore/portal read models exist; parent/child process-local cascades run. WP40 now has a reviewed 18-source/2-test ownership map. | `TrackingRuntimeEventFlow::new()` still creates a process-local `EventBus`; no shipped caller connects the child tracking flow to `ActivityJournal` or `ActivityStore`. WP40 owns that missing composition and blocks WP37; notifications, adapters, providers, and full UI remain open. | Incomplete; WP40 → WP37 → WP38/WP39. |
| V0.8 enforcement | 20 | Agent-service command dispatch and the Windows session-validated PID/name time-limit path are reachable; generic adapter execution returns `ManualRequired`. Integration head `8e9a6622a` also removes the false managed-browser `ExecutesRealService` / `ControlCapable` claim and reports that surface as manual-required. | Trusted policy decision refs, authenticated native adapter/grant, durable dispatch journal, real managed-browser profile/active-tab execution, broad app/browser/network/mobile execution, rollback, and receipt custody remain absent. | Fail-honest source correction integrated; plan remains blocked/manual-required with a narrow timer only. |

### Next dependency chains

These are routing candidates, not authorization to bypass the graph READY gate:

- Device Trust: Account WP08 canonical binding → Cloudflare WP06 durable
  current-authority bridge → WP03 one-time parent ceremony, with WP01 as the
  foundation/source input rather than a ceremony issuer. A reviewed conditional
  WP02 sealing/lifecycle-revocation gate is selected only for a demonstrated
  private-key/install custody path; the LAN/child consumer route cannot pass
  that selected gate until WP02 is complete, while the default non-sealing path
  does not add WP02 as a hard dependency.
- Tracking: WP34-WP37 → WP39, with durable journal/replay ownership before the
  portal event-to-read-model chain is assigned.
- Data Custody: WP04 → WP07, with the source/export/delete custody owner and
  durable lifecycle routed before implementation assignment.

No plan is production-code complete from this dashboard. A plan may have
bounded runtime foundations or a real presentation surface while its trusted
authority, durability, provider, child-delivery, native-adapter, or rollback
chain remains absent.

## Historical merged repository and code/test audit baseline - 2026-08-15

Repository organization is complete for this audit baseline. Consolidation PR
`#646` merged normally to `main` as `608ef84fb` after CI run `31862487297`
completed with 61 successful jobs and all three required gates green. `develop`
was created at the same commit and has the same required checks, pull-request,
conversation-resolution, no-force-push, and no-delete protections as `main`.
There are no open pull requests. Current remote and worktree custody is recorded
in `docs/REPOSITORY_CUSTODY_STATUS.md`: accepted work is pushed on the source
integration ref, rejected tips remain explicitly quarantined, and every active
OcentraParent worker worktree is on `E:`. Branch existence is recoverability,
not acceptance or completion.

The executable graph validates at 705 nodes and 765 edges. It imports **23**
actual plan rows and 681 workpacks. Current derived state is 364 planned, 28
blocked, 2 ready, 5 active, 281 in validation, and 1 done.

**681 of 681 workpacks** have reviewed code/test ownership maps. Account Identity, AI,
App/Game, App, Browser, LAN,
Child Agent Runtime Distribution, Cloudflare Control Plane, Data Custody,
Device Trust, Eventing, Logging Domain Parity, Parent Client Runtime Distribution,
Payment/Subscription, Policy Control Plane, Portal UX/Household Surfaces,
Remote Access, Network, Screen AI, Screen, Setup/Install/Provisioning, Tracking,
and V0.8 Enforcement are mapped. A reviewed map establishes current
ownership/topology, not acceptance. Do not turn the graph state or a checklist
mark into a code-completion percentage.
`npm run graph:matrix -- --json` is the complete 681-workpack table; the reviewed
coverage below states how much of that table currently has source/test evidence
strong enough for workpack-level decisions.

| Plan | Workpacks | Planned / Blocked / Ready / Active / Validation / Done | Live implementation/test files | Reviewed workpack maps | Code-first audit state |
| --- | ---: | ---: | ---: | ---: | --- |
| Account identity/family | 8 | 0/6/0/0/2/0 | 207 / 82 | 8 / 8 | Fully code-mapped. WP02/WP08 have reviewed implementation completion evidence; WP03-WP05 have accepted partial source but retain production composition gaps. All eight still retain expected-test, proof, or aggregate-gate gaps. |
| AI | 48 | 46/0/0/0/2/0 | 118 / 39 | 48 / 48 | Fully code-mapped; 11 workpacks are Phase 1 complete for bounded scope and 37 retain concrete production-code or expected-test gaps. |
| App/game | 220 | 126/2/0/1/91/0 | 606 / 62 | 220 / 220 | Fully code-mapped; 169 bounded packets have current code plus expected tests, 19 are reviewed no-code packets, and 32 retain concrete production-code or expected-test gaps. |
| App | 95 | 13/0/0/0/82/0 | 185 / 67 | 95 / 95 | Fully code-mapped; 81 bounded workpacks have no Phase 1 source/expected-test writing gap and 14 retain concrete compiler, durability, runtime, UI, notification, performance, or followthrough gaps. |
| Browser | 30 | 30/0/0/0/0/0 | 72 / 13 | 30 / 30 | Fully code-mapped; 14 executable workpacks are Phase 1 complete for bounded scope, 10 retain concrete code/test gaps, and six imported packets are reference-only. |
| Child-agent runtime distribution | 11 | 8/0/0/0/3/0 | 104 / 10 | 11 / 11 | Fully code-mapped; WP01, WP02, WP05, WP06, and WP09 are Phase 1 complete for their bounded scope, while six workpacks retain runtime, lifecycle-test, handoff, or release-gate gaps. The new service/JNI startup is real but does not close Android transport, supervision, identity, health-consumption, or device-proof work. |
| Cloudflare control plane | 13 | 9/3/0/0/1/0 | 190 / 63 | 13 / 13 | Fully code-mapped; WP00-WP02 and WP04 are Phase 1 complete, WP06 has an independently accepted bounded source adapter/auth chain, and the remaining workpacks retain concrete code/test gaps. WP06 migration/tests/proof remain open. |
| Data custody/storage | 9 | 1/1/3/0/4/0 | 971 / 482 | 9 / 9 | Fully code-mapped. WP05, WP06, and WP08 are legal remaining source packets; WP07 is dependency-blocked on the Account authority chain. No workpack is DONE, and all current expected-test/proof acceptance remains open. |
| Device trust bootstrap | 9 | 0/2/1/0/6/0 | 553 / 143 | 9 / 9 | Fully code-mapped; all nine remain graph-open. Accepted source through `68717b5b7` covers WP01 current binding, WP05 unsigned entitlement, WP06 fail-closed restore authority, and WP07 durable removal/readiness. External ceremony/provider/platform callers, expected tests, and proof remain open. |
| Eventing | 13 | 0/2/1/0/9/1 | 824 / 490 | 13 / 13 | Fully code-mapped. WP06 is graph-done; WP09 is validation/integration-open; WP11 has its production source and caller migration integrated but remains READY for the later expected-test phase; WP10 and WP12 are blocked, and WP13 remains validation/proof-open. |
| LAN | 26 | 0/1/0/0/25/0 | 320 / 60 | 26 / 26 | The 25 pre-existing workpacks are code-mapped and 22 have bounded Phase 1 code/expected tests written. WP16, WP20, and WP25 retain integrated-validation or executable-verifier gaps; WP26 is code-mapped as a partial draft but remains blocked on Account WP08 -> Cloudflare WP06 current authority and Device Trust WP03 ceremony, with no completion claim. A selected platform sealing/lifecycle-revocation path also requires the reviewed WP02 gate; the default non-sealing route does not force it. |
| Logging domain parity | 10 | 5/0/0/0/5/0 | 133 / 49 | 10 / 10 | Fully code-mapped. Accepted source hardening gives Rust one exact 18-key policy and generated TypeScript parity, makes unsupported/reflection failures JSON-safe, routes the dev writer through the canonical owner, sanitizes Logger/portal fallback serialization, and closes realpath/symlink containment. WP01, WP04, and WP09 have no bounded writing gap; WP02, WP03, WP05-WP08, and WP10 still require expected-test or enforcement code. |
| Network | 8 | 6/1/0/0/1/0 | 378 / 97 | 8 / 8 | Fully code-mapped; WP05 and WP08 are Phase 1 complete for their bounded scopes, while WP01-WP04 and WP06-WP07 retain canonical-contract, live-runtime, production-wiring, or executable-harness gaps. WP04 is blocked behind Eventing WP09 and its other reviewed owners. |
| Parent desktop/runtime package | 11 | 4/0/0/0/7/0 | 528 / 183 | 11 / 11 | Fully code-mapped; WP01, WP04, WP05, and WP09 are Phase 1 complete for their bounded scope, while seven workpacks retain concrete runtime or expected-test gaps. |
| Payment/subscription | 13 | 4/5/0/4/0/0 | 49 / 39 | 13 / 13 | Fully code-mapped; accepted production source closes four major authority/durability defects, while Account/provider/issuer dependencies and the complete expected-test delta remain open. |
| Policy control plane | 8 | 1/4/0/0/3/0 | 976 / 480 | 8 / 8 | Fully code-mapped; all eight workpacks retain production reachability or expected-test gaps. WP01 is blocked on the reviewed Cloudflare and Device Trust owners, and WP03's deterministic compiler remains library code without a shipped authoritative-source-to-domain caller. |
| Portal UX/household surfaces | 20 | 15/0/0/0/5/0 | 1040 / 531 | 20 / 20 | Fully code-mapped; 9 workpacks have no Phase 1 writing gap in their bounded scope, while 11 retain concrete product-code or expected-test gaps. |
| Remote access | 6 | 4/0/0/0/2/0 | 35 / 19 | 6 / 6 | Fully code-mapped; WP01 and proof-only WP06 have no Phase 1 writing gap, while WP02-WP05 retain concrete runtime, deferred-control, persistence, or relay-security gaps. |
| Screen AI pipeline | 10 | 9/1/0/0/0/0 | 124 / 33 | 10 / 10 | Fully code-mapped; prerequisite routing is the only bounded Phase 1 row without a writing gap. WP02-WP10 retain production-composition, authority, durability, custody-negative, performance-test, or missing executable-harness gaps. |
| Screen | 43 | 25/0/0/0/18/0 | 95 / 26 | 43 / 43 | Fully code-mapped; 9 of 40 executable workpacks are complete for bounded Phase 1 code/expected-test writing, 31 retain concrete gaps, and three imported reference packets own no executable code. |
| Setup/install/provisioning | 7 | 0/2/0/0/5/0 | 594 / 198 | 7 / 7 | Fully code-mapped; five workpacks are in validation and two are blocked. WP07 has a reachable Rust-owned 15-row fail-closed setup matrix, a canonical 13-variant LAN classifier, and non-LAN dispatch rejection; evaluator/actions remain deliberately not run, LAN is observation-only, and real authority inputs, the state machine/completion guard, and current expected tests remain open. |
| Tracking | 43 | 41/2/0/0/0/0 | 94 / 65 | 43 / 43 | WP40 now has reviewed child-runtime/journal ownership but is correctly blocked on WP32/WP34/WP36; it unlocks WP37 only after those real owners are complete. Twenty-four bounded packets have code/expected tests and 19 retain production-code or expected-test gaps. |
| V0.8 enforcement | 20 | 13/1/0/0/6/0 | 942 / 496 | 20 / 20 | Fully code-mapped; 7 workpacks have no Phase 1 writing gap in their bounded scope, while 13 retain concrete runtime, surface, lifecycle-test, or executable-harness gaps. |

The repository-wide reviewed ownership map now covers all 681 workpacks,
including WP40's explicitly blocked child-runtime/journal owner set. Work
proceeds from the code-first gap matrices: write missing real
production code across the authorized dependency frontier first; only after
that write/migrate production-path tests, then run focused tests and Enforcer,
then
regenerate proof in Phase 3. Proof is not a substitute for missing code/tests.

### Browser plan Phase 1 code/test audit - 2026-08-15

This audit maps the current Rust/service/portal implementation and expected
tests. It does not use historical `output/` proof as implementation evidence
and does not claim that the focused test families have been rerun in Phase 2.

| Workpack | Actual code/test evidence | Phase 1 | Remaining code or expected-test gap |
| --- | --- | --- | --- |
| WP01 Contract Boundary And Effect Schemas | Rust protocol inventory, managed-session, policy, intervention, and read-model contracts have contract/unit tests, including dishonest exact-URL and typed rejection cases. | **Complete for Phase 1** | Generated-edge/proof reconciliation is later; no missing bounded contract family was found. |
| WP02 Source Index And Doc Reconciliation | Coordination-only packet; it owns no product code. | **Complete for Phase 1** | Current source paths still need Phase 3 documentation reconciliation, but no implementation belongs here. |
| WP03 Browser Inventory Model | Typed inventory/read-model contracts and service row conversion have managed/unmanaged and dishonest-claim tests. | **Complete for Phase 1** | Live platform collection belongs to WP04/WP05. |
| WP04 Windows Browser Inventory Adapter | Registry, shortcut, package, known-path, and process inventory code plus service conversion is covered by focused fixture/source tests. | **Complete for Phase 1** | Real-host proof is Phase 3; the bounded Windows adapter and expected tests are written. |
| WP05 Cross-Platform Inventory Matrix | Windows inventory and an Android owned-shell implementation exist. | **Incomplete** | No current macOS/iOS inventory adapter or focused platform-matrix implementation/test family exists; Linux/Android visibility is not represented by one runtime-owned matrix. |
| WP06 Managed Profile Store | Owned-path store code covers create/reload/delete/repair, redacted metadata, and default/unowned path rejection. | **Incomplete** | Corrupt/truncated metadata, atomic-write crash behavior, and concurrent-writer negatives are absent. |
| WP07 Managed Chromium Launcher | Launch planning, executable identity, owned-profile enforcement, loopback port reservation, failed-spawn behavior, and service status tests are written. | **Complete for Phase 1** | Real browser execution is Phase 2/3 validation, not a missing code family here. |
| WP08 Bridge Custody And Security | Loopback HTTP/CDP polling enforces port, process, profile, session, browser identity, timeout/size, and debugger-URL redaction boundaries with negatives. | **Complete for Phase 1** | Runtime proof remains later. |
| WP09 CDP Version And Target Adapter | Version/target parsing rejects malformed JSON, wrong shapes, missing identifiers/URLs, oversized responses, and timeouts. | **Complete for Phase 1** | No active-focus claim is made; that belongs to WP11. |
| WP10 Tab Evidence Mapper | Target observations map stable custody/tab/window/origin fields, strip credentials, normalize URLs, and reject invalid URLs/empty target IDs. | **Complete for Phase 1** | The mapper intentionally produces target-list evidence, not active-tab proof. |
| WP11 Active-Tab Proof Model | Typed active-state/capability/proof-source fields and no-overclaim tests exist. | **Incomplete** | No focus/activation observer feeds the model; `/json/list` remains target-list evidence and cannot establish the active tab. |
| WP12 Journal And SQLite Browser Ingest | Ordered event runtime, encrypted journal replay, SQLite projection, restart recovery, duplicate replay, stale/degraded reconstruction, and empty-state tests are written. | **Complete for Phase 1** | Focused execution remains Phase 2. |
| WP13 Browser Read Models And Service Events | Service inventory and runtime stream APIs project event chains, pending candidates, policy previews, unavailable/stale states, and WebSocket command reports with tests. | **Complete for Phase 1** | Product action execution belongs to WP17/WP19, not this read-model packet. |
| WP14 Portal Browser Status Surfaces | Portal status/intervention renderers consume Rust-owned snapshots and retain explicit empty states. | **Incomplete** | Current tests mainly inspect source shape/intervention separation; dedicated rendered status cases for missing, unmanaged, managed-ready, running, disconnected, stale, and unsupported states are missing. |
| WP15 Browser Policy Authoring Manifest | Generated manifest identifiers feed schema-domain and service patch/store runtime tests, including dishonest update rejection. | **Complete for Phase 1** | Parent-facing authoring UX is outside this bounded manifest packet. |
| WP16 Policy Target Compiler | Compiler code/tests label target requirements, preserve parent authority, keep observe/dry-run non-executing, require adapter proof, and expose manual-required policy-writer state. | **Complete for Phase 1** | Trusted adapter execution is an Enforcement dependency. |
| WP17 Managed Intervention And Block Page | Typed intervention events, a served child page, a portal-domain renderer, and SQLite read-model tests exist. | **Incomplete** | No production policy-decision-to-managed-navigation/block-page delivery owner or receipt/rollback integration test exists. |
| WP18 Unmanaged Browser Detection | Process-only discovery classifies supported, unsupported, and unknown browser-like processes; contracts forbid exact URL claims. | **Complete for Phase 1** | Enforcement/action behavior belongs to WP19. |
| WP19 Unmanaged Fallback UX And Actions | Read-model fallback derivation and portal rendering distinguish warn/terminate/relaunch/manual states without URL overclaim. | **Incomplete** | No trusted terminate/warn/relaunch dispatch owner, durable execution receipt, denial, or rollback test closes the action path. |
| WP20 Windows AppLocker And App Control Proof | Enforcement exposes typed app-control proof states and a tested read model. | **Incomplete** | No AppLocker/WDAC policy application, event capture, rollback, or real-host harness code exists. |
| WP21 Extension And Native Host Boundary | Native-host frame validation covers managed-profile binding, trusted origin, length/schema drift, default/missing binding, and stale heartbeat. | **Incomplete** | A browser extension/runtime connector, installation/registration path, handshake lifecycle, and integration tests are absent. |
| WP22 Performance And Service Health | A browser-core performance-budget model/test and service status serialization tests exist. | **Incomplete** | No executable load/latency/memory/long-run bridge-health harness or recovery-under-load test exists. |
| WP23 E2E And Manual Proof Artifacts | One Playwright browser-AI explanation scenario and failure helper are present. | **Incomplete** | The advertised aggregate browser E2E/manual-artifact verifier is absent; managed launch, custody loss, intervention, unmanaged fallback, and service restart are not covered by one executable suite. |
| WP24 Rollout, Checklist, And PR Gate | Coordination-only final gate; it owns no product implementation. | **Complete for Phase 1** | It remains operationally open until Phase 2/3 inputs are green and merged. |
| Browser Control 1057 Settings Inventory | Imported design/reference packet; no executable code ownership. | **Reference only** | Route implementation through WP15/WP16. |
| Browser Control Coverage Matrix | Imported coverage/reference packet; no executable code ownership. | **Reference only** | Reconcile against the executable workpacks; do not schedule as a product slice. |
| Browser Control Schema Proposal | Imported schema proposal; current schema authority is WP01/WP15. | **Reference only** | Do not restore a parallel contract owner. |
| Browser Policy Questionnaire Forest V1 | Imported authoring reference; generated schema tests exist under WP15 ownership. | **Reference only** | Treat changes as WP15 authoring-manifest work. |
| Browser Policy Settings Catalog | Imported settings reference; current catalog/manifest implementation is WP15. | **Reference only** | Do not count it as another execution packet. |
| Managed Unmanaged Browser | Imported product-boundary reference; executable work is split across WP03-WP21. | **Reference only** | Keep managed exact-URL evidence separate from unmanaged process-only control. |

**Browser Phase 1 result:** all 30 imported packets now have reviewed ownership.
Fourteen of the 24 executable workpacks have their bounded core code and
expected tests written; ten remain incomplete. The first dependency chain is
WP05/WP06 platform and custody hardening, WP11 active-focus evidence, then
WP17/WP19 trusted intervention execution. WP14, WP20-WP23 close the product
surface, platform control, extension, health, and aggregate-test gaps before
any whole-plan Phase 2 or proof claim.

### LAN plan Phase 1 code/test audit - 2026-08-15

This audit maps the current Rust/service/desktop/portal paths and their expected
tests. Historical generated `output/` or `test-results/` files are not counted
as source, and manual physical-device proof is deferred to Phase 3.

| Workpack | Actual code/test evidence | Phase 1 | Remaining code or expected-test gap |
| --- | --- | --- | --- |
| WP01 Contract Boundary And Effect Schemas | Rust protocol pairing, source, route-snapshot, and add-device-state contracts have focused contract tests and fail-closed version handling. | **Complete for Phase 1** | Contract generation/proof reconciliation is later. |
| WP02 Evidence Model And Device Record | Rust read models and service registry projection preserve source evidence and canonical household rows with unit tests. | **Complete for Phase 1** | Physical evidence quality is Phase 3. |
| WP03 Interface Detection | Interface hardware/default-route discovery captures gateway, DNS, DHCP, broadcast, IPv6, and selection reasons with unit tests. | **Complete for Phase 1** | Real-host platform proof remains later. |
| WP04 Neighbor Table Ingestion | Windows, Linux, and macOS parsers normalize IPv4/IPv6, malformed, incomplete, and duplicate rows under focused tests. | **Complete for Phase 1** | Live macOS validation is Phase 3, not missing parser code. |
| WP05 Targeted ARP Checks | Bounded targeted refresh enforces interface/subnet/throttle rules and covers response/no-response packet-IO behavior. | **Complete for Phase 1** | Packet capture is Phase 3. |
| WP06 Bounded ARP Sweep | Scan planning, bounded refresh, command handling, service physical-scan integration, and suppression behavior have Rust tests. | **Complete for Phase 1** | Real-router proof is Phase 3. |
| WP07 Passive Discovery Listeners | Passive ARP, DHCP, mDNS, SSDP, WSD, LLMNR, NetBIOS, beacon, and allowed SNMP observations feed hint-only runtime state with tests. | **Complete for Phase 1** | Long-running listener and packet proof are Phase 3. |
| WP08 mDNS And DNS-SD Discovery | PTR/SRV/TXT/A/AAAA parsing, service enumeration, hostile-name rejection, and hint-only handling have focused tests. | **Complete for Phase 1** | Broader packet/platform proof remains later. |
| WP09 SSDP And UPnP Discovery | Bounded search, safe private descriptor fetch, parsing, unsupported-device handling, timeout, malformed, and oversize negatives are written. | **Complete for Phase 1** | Physical UPnP coverage is Phase 3. |
| WP10 NetBIOS LLMNR Reverse DNS | Weak name evidence and DNS-like passive parsing reject unsafe/malformed values and avoid identity overclaim under tests. | **Complete for Phase 1** | No code gap found in the bounded name-evidence scope. |
| WP11 Light Service Probing | Bounded HTTP/HTTPS/TLS, WSD, and SNMP identity probing plus interface/trusted-device suppression have focused tests. | **Complete for Phase 1** | Optional OS fingerprinting is manual-gated, not required production code. |
| WP12 OUI Vendor Lookup | MAC normalization, private/randomized handling, and vendor lookup behavior have unit tests. | **Complete for Phase 1** | Dataset/provenance refresh is later acceptance work. |
| WP13 Merge And Deduplication Engine | Canonical household merge, strong/weak identity scoring, registry continuity, explicit decisions/reasons, and property tests are written. | **Complete for Phase 1** | Physical collision proof is Phase 3. |
| WP14 Explainable Classification | Weighted classification, explicit reasons/confidence, router/unsupported/unknown fencing, and portal label rendering have tests. | **Complete for Phase 1** | Manual installability proof is later. |
| WP15 Household Device Store | Persistent registry, scan history, replay projection, restart continuity, and stale-address suppression have service tests. | **Complete for Phase 1** | Cross-session physical proof remains later. |
| WP16 Read Models And LAN Events | Service event-chain stream, parent replay validation, desktop delivery decision, and portal replay/state seams each have focused tests. | **Incomplete** | No integrated backend -> real Tauri `AppHandle` emit -> portal listener test proves the complete delivery chain. |
| WP17 Parent Child mDNS Advertisements | Advertisement encoding, opaque metadata, lifecycle evaluation, and service synchronization have focused tests. | **Complete for Phase 1** | Multicast/platform proof is Phase 3. |
| WP18 Signed Child Hello And Heartbeat | Signature/capability/replay rejection, unpaired handling, heartbeat/offline/manual-required state, and service projection have tests. | **Complete for Phase 1** | Second-device and iOS proof are Phase 3. |
| WP19 Assignment Revocation And Audit | Trusted route commands, rename, ignore/restore, select/revoke, audit, persistence/restart, and portal dispatch paths have tests. | **Complete for Phase 1** | Broader physical topology proof remains later. |
| WP20 Proof Gates Fixtures And Rollout | Fixture, property, performance, and portal visual test families exist. | **Incomplete** | Six LAN aggregate verifier commands named by the current plan docs point to absent scripts; two of them are incorrectly checked complete in this workpack. |
| WP21 Contract Boundary And Domain Schemas | Family identity contracts and setup lifecycle records needed by LAN pairing have contract/unit tests. | **Complete for Phase 1** | This is a bounded dependency contract packet. |
| WP22 Current State And Gap Map | Documentation-only reconciliation packet; it owns no product code. | **Complete for Phase 1** | Status must follow this live audit. |
| WP23 Pairing And Route Proof | Browser runtime pairing contract, service runtime command path, route selection, and restart behavior have focused tests. | **Complete for Phase 1** | Real two-device proof is Phase 3. |
| WP24 Portal UX And First Run Handoff | Parent LAN route snapshots, portal-domain device rows, and Devices/command-center E2E surfaces have tests. | **Complete for Phase 1** | Physical first-run proof is later. |
| WP25 Rollout Checklist And PR Gate | Parent replay, desktop host decision, portal state edge, and visual E2E tests exist. | **Incomplete** | The aggregate source-matrix verifier is absent and the integrated backend-to-AppHandle-to-listener regression is still missing. |
| WP26 Signed Child Beacon Ingress And Household Mesh Authority Handoff | Partial custody/registry/transport draft; no shipped ingress, organized real-ingress tests, or current proof root is claimed. | **Blocked** | Account WP08 -> Cloudflare WP06 current-authority resolution and Device Trust WP03 one-time `RegisterLanSignerAnchor` authorization are missing, along with real signed child/runtime ingress, durable custody/restart recovery, W15/W18/W19 authority composition, and the private Eventing WP10 handoff. If the platform sealing/lifecycle-revocation path is selected, the reviewed WP02 gate must also complete before this consumer route can proceed; the non-sealing route does not force WP02. |

**LAN Phase 1 result:** 26 rows are now routed: 13 document-closed and 13
open. Four Phase 1 gaps are explicit: WP16 cross-process delivery, WP20
aggregate verifiers, WP25 integrated rollout delivery, and the new WP26 signed
child beacon/household authority handoff. WP26 is the LAN dependency that
blocks Eventing WP10.
Manual household/router/platform artifacts remain Phase 3 and do not reopen the
22 code-complete rows.

### Network plan Phase 1 code/test audit - 2026-08-15

This is a source-and-test inspection of the production call paths and their
focused tests. Graph topology satisfaction means the mapped files exist; it
does not mean a fixture/proof builder is a live product path or that tests have
been rerun in Phase 2.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Foundation Contracts And Eventing | 60 implementation and 11 test files cover typed network observations, reusable event envelopes, runtime phase records, delivery, queueing, and stream serialization. | **Incomplete** | Canonical evidence/domain truth is duplicated between `agent-protocol` and `ocentra-network-evidence`; the runtime creates an in-memory bus/journal per observation and synthesizes downstream phase records instead of consuming durable owners. Contract parity, durable composition, and publisher-authority negatives remain unwritten. |
| WP02 Passive Capture And Parsing | 71 implementation and 9 test files cover deterministic PCAP/Ethernet/IP/TCP/UDP/DNS/TLS/HTTP parsing, encrypted-visibility limits, flow sessionization, raw-capture custody, Windows `netstat -ano` metadata capture, and service readiness bridges. | **Incomplete** | The parser stack is not fed by the production capture path. Windows production captures connection metadata only; no Npcap/libpcap driver or live packet-to-parser integration exists, and no real live-capture parser test covers that path. |
| WP03 Classification And Correlation | 19 implementation and 11 test files cover normalized domains, categories, classification, tunnel/transfer signals, process, managed/unmanaged browser, app/game, and screen-summary correlation. | **Incomplete** | Production service code does not invoke this classifier/correlation composition; it derives a source kind from whether a process name exists and assigns fixed evidence grades. A live parser-to-correlation-to-read-model path and its negative tests are absent. |
| WP04 Cross-Slice Cascade And Parent Surface | Surviving production code exposes real ActivityStore observations, runtime-delivery state, and the service-backed portal evidence drawer. The fabricated product-path bridge, optional payload fields, and disconnected evidence pipeline were removed rather than replaced with synthetic success. The shipped-call audit also found no typed durable `NetworkCascadeObligation`, cascade table, or composition owner; the apparent cascade is runtime `OnceCell`/spine state, read-time republish, and manufactured refs. | **Blocked** | Tests/support that import or bless the deleted APIs remain test-phase debt. No shipped caller composes a real queued AI job, policy request, provider notification, adapter result, or custody lifecycle from network evidence. Direct legal-composition prerequisites are Eventing WP09, AI WP07/WP19, Policy WP05/WP08, Custody WP04/WP06, and Portal WP09/WP12; none is marked done by this audit. |
| WP05 Intervention Adapter Proof Gates | 84 implementation and 26 test files cover DNS, Windows Firewall/WFP, Android VPN, Apple Network Extension, Linux nftables, rollback/unavailable/no-overclaim states, protocol status contracts, and service status bridges. | **Complete for Phase 1** | No missing bounded gate code or expected negative-test family was found. Actual OS mutation and physical-platform acceptance remain Phase 2/3 and owning-platform work. |
| WP06 Analyzer, AI Audit, And Risk Budget | 48 implementation and 12 test files cover Zeek/signature ingestion, deterministic AI evaluation/audit models, thresholds, and risk-budget decisions. | **Incomplete** | No live analyzer/model runtime feeds production. The service proof builder compares a hard-coded `VpnProxyTunnel` expectation to the same hard-coded prediction and supplies fixed risk inputs, so drift/precision monitoring and a real analyzer-to-policy handoff are unwritten. |
| WP07 Performance, Security, And Rollout | 20 implementation and 5 test files cover deterministic performance/readiness rows, local platform probes, and adapter-capability status validation. | **Incomplete** | There is no executable concurrency/spike/soak/load harness, live metrics or alert observation, abuse/rate-limit/DoS runner, or rollout/rollback automation. These are missing expected test/harness code, not merely missing proof. |
| WP08 Control Catalog Reference Routing | 16 implementation and 3 test files cover the Rust-owned generated control catalog, generated-drift contract, and exact reference-routing/no-runtime-claim boundary. | **Complete for Phase 1** | No code/test-writing gap exists inside this reference-only workpack. Runtime control authority is explicitly outside its scope. |

**Network Phase 1 result:** 8/8 workpacks inspected and mapped; 2/8 are
complete for code/test-writing scope and 6/8 still require production code or
expected tests. No Network test pass, Enforcer acceptance, proof, or whole-plan
completion is claimed by this Phase 1 audit.

### Screen AI Pipeline Phase 1 code/test audit - 2026-08-15

This audit follows the production startup, capture, queue, local-AI, event,
read-model, and portal paths and compares them with the expected test and
harness code. It does not use the unchecked plan rows as implementation truth,
and it does not treat a generated event chain or proof description as a live
policy/action path.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Prerequisite Merge And Branch Gate | This is a coordination-only prerequisite row with no owned production or test code. Current branch promotion is handled by the repository feature-to-`develop`-to-`main` process. | **Complete for Phase 1** | No code or expected-test writing belongs to this workpack. Exact prerequisite commit/PR records and stale-proof reconciliation remain Phase 3 coordination evidence. |
| WP02 Real Trigger To Capture Gate | 31 implementation and 10 test files cover real selected-window capture, scheduler suppression/debounce, native foreground observation, timed cadence, encrypted queue writes, screen settings commands, and focused capture/runtime tests. | **Incomplete** | Production capture reads environment flags rather than the parent screen-settings store. Every foreground change is labeled `NativeAppForegroundStart`; browser URL/video/feed, browser game, native game, launcher, and unknown-process trigger kinds exist only as scheduler vocabulary or isolated tests and are not fed by owning runtimes. Cross-trigger and parent-disable integration tests are missing. |
| WP03 Capture To AI Analysis Gate | 47 implementation and 9 test files cover encrypted queue leasing, a bounded local adapter process, schema-shaped result parsing, OCR redaction, provider unavailable/invalid states, and separate typed router/pipeline libraries. | **Incomplete** | Production never calls the `screen-ai-core` intelligence router, AI-hub router, or pipeline decision. The service sends the frame to one configured adapter process and accepts only `localVision`/`localOCR`; OCR-versus-VLM-versus-text-versus-deterministic routing and typed-context-only model consumption are not composed or tested end to end. |
| WP04 AI Result To Policy Gate | 6 implementation and 4 test files validate local-AI output shape, confidence, provider kind, policy eligibility, and the screen enforcement handoff contract. | **Incomplete** | `service_policy_refs` creates a synthetic decision ID, fixed `allow` action, fixed reason, and fixed parent-rule ref whenever adapter output says `policyEligible`. No policy-control owner is invoked, stricter-parent-rule precedence is absent, and tests currently assert the fabricated metadata instead of proving a trusted AI-result-to-policy boundary. |
| WP05 Policy Action Dry-Run Gate | 5 implementation and 3 test files provide a Rust handoff guard, policy/action phase-state helpers, and event-bridge rejection tests. | **Incomplete** | These are contract and event-shape guards only. There is no production dry-run dispatcher for observe, allow, warn, ask-parent, time-limit, block, or manual-required; no timer/expiry or owned-process adapter handoff is driven from a real policy decision, and no action-matrix integration tests exist. |
| WP06 Journal Read Model And Portal Gate | 31 implementation and 8 test files cover typed screen phases, ActivityStore projection/query behavior, service event bridging, a service-backed portal summary panel, unavailable state, redaction, and UI screenshot test code. | **Incomplete** | The normal chain creates a new in-memory event bus per row; only deletion has a durable hash-chain journal. The portal reads ActivityStore summaries, but there is no durable replay of a real trigger/capture/AI/policy/action chain, and the displayed policy/action refs can originate from WP04's fabricated metadata. Durable full-chain replay and authority-negative tests are missing. |
| WP07 Deletion Retention And Custody Gate | 17 implementation and 5 test files cover encrypted durable queue records, leases, delete-after-analysis, TTL sweeping, durable deletion outbox/restart replay, quarantine, delete-failure projection, and raw-image exclusion from policy/portal event payloads. | **Incomplete** | The core local custody mechanics are substantial, but the explicit no-remote/cloud-upload boundary and retention-only-with-parent-opt-in contract are not enforced by a production capability or dedicated negative tests. The external adapter process boundary also lacks a test proving it cannot select a remote transport while claiming local-only custody. |
| WP08 Live Operator Proof Gate | The two workpack-owned executable test roots are mapped explicitly. Both are absent: `screen-ai-live-operator-proof.mjs` and `screen-ai-live-operator-artifact-gate.mjs`. | **Incomplete** | The plan describes a nine-scenario manifest harness and retained-artifact validator that do not exist in this checkout. This is missing expected harness/test code before it is a missing-proof problem. |
| WP09 Performance Cadence And Backpressure Gate | 13 implementation and 4 test files cover cadence/debounce decisions, queue-size limits, foreground/cadence single-tick capture, analysis `max_jobs`/`max_ticks`, polling, and adapter timeout controls. | **Incomplete** | Existing tests prove isolated decisions or one recorded capture, not the required three-capture cadence, full-queue/no-fourth-row behavior, disable-stops-future-jobs runtime behavior, or repeated-analysis no-flood behavior. The multi-tick/backpressure test harness is unwritten. |
| WP10 Final Rollout And PR Gate | The three named executable rollout test roots are mapped explicitly. `screen-ai-final-product-path-proof.mjs`, `screen-ai-service-winrt-ocr-proof.mjs`, and `screen-ai-household-mesh-proof.mjs` are absent. | **Incomplete** | The aggregate verifier/runner code is missing and every upstream production gap remains open. Final validation, proof, and PR promotion cannot honestly close this workpack until the executable gate exists and WP02-WP09 are complete. |

**Screen AI Pipeline Phase 1 result:** 10/10 workpacks inspected and mapped;
only the no-code prerequisite WP01 has no Phase 1 writing gap. Nine workpacks
still require production composition, authority/custody corrections, expected
integration tests, or executable harnesses. No Screen AI test pass, Enforcer
acceptance, proof, or whole-plan completion is claimed by this audit.

### Screen Plan Phase 1 code/test audit - 2026-08-15

This audit follows the current Rust capture, settings, encrypted queue,
analysis, journal/read-model, deletion, live-view, household-mesh, and portal
paths plus the Android Java capture adapter. It treats cited files that no
longer exist under `packages/*-domain` or `scripts/test/screen-*` as missing
code, not retained implementation, and it does not use the 100/100 legacy
checklist as runtime evidence.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Source Index And Doc Reconciliation | Coordination-only packet; no product code is expected. | **Incomplete** | The routed source index still describes removed TypeScript ownership and has not been reconciled to the Rust owners inspected here. |
| WP02 Current Screen Snapshot And Gap Map | Coordination-only packet; no product code is expected. | **Incomplete** | The snapshot still cites removed packages and proof scripts and does not reflect the production-wiring gaps found by this audit. |
| WP03 Contract Boundary And Effect Schemas | Five implementation and three test files provide Rust settings, queue/result, runtime-event, deletion, policy-ref, read-model, and generated parent-screen shapes. | **Incomplete** | Several states remain raw strings rather than closed schema enums; there is no single strict malformed-payload matrix covering settings, capability, queue, analysis result, deletion, and policy evidence together. |
| WP04 Parent Opt-In Settings Contract | Twelve implementation and five test files implement disabled defaults, typed get/replace commands, JSON persistence/reload, stale-version rejection, unsafe-retention rejection, generated bridge state, and portal controls. | **Complete for Phase 1** | Focused execution and acceptance proof remain Phase 2/3. |
| WP05 Capability Status Contract | Five implementation and three test files expose generic capture status, queue health, service screen rows, and capability cards. | **Incomplete** | No unified typed contract covers every required disabled, unsupported, permission, protected, locked, model, queue, degraded, ready, proof-tier, and scope-availability state; portal negative-state tests are incomplete. |
| WP06 Capture Scope Model | Three implementation and three test files model active window, selected window, and primary display. | **Incomplete** | Full-screen/display identity, selected app, managed-browser/window, Android app-window, and explicit unsupported scopes are absent, as are platform gates and complete portal labels. |
| WP07 Capture Trigger Model | Ten implementation and three test files define the required trigger vocabulary, cadence/debounce suppression, and cadence/foreground service loops. | **Incomplete** | Production foreground capture labels every change as native app; browser/game/launcher/unknown/network/policy/manual owners do not feed it, and capability-ready gating is not part of the scheduler decision. |
| WP08 Platform Adapter Abstraction | Seven implementation and two test files provide concrete desktop/Linux capture functions and Android MediaProjection entrypoints. | **Incomplete** | There is no shared adapter trait with adapter identity, capability probe, typed request/result, proof tier, and fake/dev non-proof boundary across Windows, macOS, Linux, Android, and iOS. |
| WP09 Windows Capture Adapter Plan Proof | Five implementation and four test files provide xcap display/window capture plus service cadence/foreground queueing. | **Incomplete** | The adapter lacks a Windows-specific permission/protection capability model and expected tests for protected surfaces, managed-browser scope, and capture-to-queue-to-delete behavior. |
| WP10 macOS Capture Adapter Plan Proof | Three shared xcap implementation files and one generic capture test compile the desktop path for macOS. | **Incomplete** | No macOS-specific capability/permission tests, live Screen Recording path, PPPC/MDM state, OCR/deletion integration, or macOS-host executable test exists. |
| WP11 Linux Capture Adapter Plan Proof | One X11 implementation and two tests cover X11 window-title safety and generic capture behavior. | **Incomplete** | Wayland/PipeWire portal, GNOME/KDE/wlroots state, unsupported-compositor handling, root-display behavior, and OCR/deletion integration tests are absent. |
| WP12 Android MediaProjection Adapter Plan Proof | Four Java production files implement consent activity, foreground capture service, proof metadata, and local proof storage. | **Incomplete** | There is no Android unit/instrumentation test code for consent denial, stop callback, no-silent-background behavior, deletion, physical-device capture, or OCR. |
| WP13 iOS ReplayKit Adapter Plan Proof | The iOS agent application shell exists. | **Incomplete** | No ReplayKit/broadcast-extension capture implementation or test target exists; `OcentraParentAgentTests` is absent. |
| WP14 Protected Surface Detector | Ten router/redaction implementation files and three tests can reject caller-supplied protected/credential flags and redact OCR tokens. | **Incomplete** | No production detector supplies lock-screen, secure-desktop, credential-prompt, password-field, DRM, or OS-protected classifications before capture; capture/portal tests for those states are absent. |
| WP15 Encrypted Temporary Image Queue | Nine implementation and two test files implement encrypted records, digest/source metadata, leases, TTL sweep, retry/outbox, quarantine, atomic mutation, and queue health. | **Complete for Phase 1** | Focused queue/security execution remains Phase 2; custody proof remains Phase 3. |
| WP16 Queue Scheduler And Debouncer | Eight implementation and three test files cover cadence, trigger debounce, queue scan limits, max pending, timeouts, and disabled suppression. | **Incomplete** | Strict-mode composition and end-to-end no-flood/backpressure tests are missing, including repeated-trigger and queue-capacity integration through the service. |
| WP17 Local OCR Vision Runtime Model | Thirty implementation and three test files provide a bounded external local adapter process, structured-output parsing, redaction, timeout/unavailable states, and local-AI contracts. | **Incomplete** | No production OCR or VLM worker is selected/composed; the service executes one configured external command and does not use the canonical screen router/provider scheduler to prove local-only OCR/VLM runtime behavior. |
| WP18 Screen Analysis Result Schema | Five implementation and two test files define the result/event record and serialize category, risk, OCR, confidence, refs, digest, deletion, policy, and explanation fields. | **Incomplete** | Category, risk, status, custody, and deletion values remain open strings and strict invalid-result conversion tests do not cover all required fields. |
| WP19 Sensitive Text And Redaction Model | Five implementation and five test files enforce OCR disablement, snippet limits, credential suppression, PII token redaction, persisted parent policy, and portal rendering without raw text. | **Complete for Phase 1** | Focused security/runtime execution remains Phase 2; real OCR quality and retained proof remain later scopes. |
| WP20 Result Validator And Invalid Output Handling | Two parser/record implementation files and three test files reject invalid JSON, incomplete results, unsupported provider kinds, and out-of-range confidence. | **Incomplete** | Missing refs, unsupported categories/signals, raw-text overflow, malformed deletion/custody state, and policy-driving rejection are not enforced as one fail-closed validator. |
| WP21 Journal And SQLite Ingest | Six implementation and four test files provide screen activity-store projection, runtime phase events, service bridge/subscription, and summary query tests. | **Incomplete** | The normal nine-phase chain uses ephemeral in-memory publication; only deletion has durable journaling. There is no durable full-chain replay/rebuild test proving summary-only SQLite state after restart. |
| WP22 Deletion And Retention Proof | Twelve implementation and five test files cover success/expiry deletion, delete-failed projection, durable outbox replay, quarantine, atomic acknowledgement, bounded reports, and portal custody state. | **Complete for Phase 1** | Focused Rust/portal execution is Phase 2 and the accepted deletion artifact pack is Phase 3. |
| WP23 Policy Compiler For Screen Derived Evidence | Three implementation and two test files expose a guard contract, a policy helper, and service policy-reference construction. | **Incomplete** | Production does not call policy-control authority; the service fabricates an allow decision and parent-rule reference from adapter `policyEligible`, with no real category/risk compiler, confidence/unknown behavior, or raw-input negative path. |
| WP24 Enforcement Handoff Guard | Three implementation and two test files enforce summary/rule/confidence/audit refs and reject raw pixels, raw model text, retained screenshots, and claimed local-AI authority. | **Complete for Phase 1** | This is intentionally a guard/dry-run boundary; adapter execution remains enforcement-plan scope. |
| WP25 Parent Portal Summary UI | Five implementation and four test files render service-backed settings, queue/summary, confidence, refs, deletion, model, policy, audit, and no-enforcement state, including Playwright route coverage. | **Complete for Phase 1** | Focused portal/service execution and screenshots remain Phase 2/3. |
| WP26 Child Disclosure UX | No production or test file exists at the expected child-disclosure UI boundary. | **Incomplete** | Child-visible enabled/paused/active/disabled state, calm copy, runtime delivery, and desktop/mobile UI tests are unwritten. |
| WP27 Screenshot Retention Optional Mode | Eight implementation and two test files provide disabled defaults, parent-approved short-TTL setting validation, persistence, generated UI examples, and portal controls. | **Incomplete** | There is no production raw-retention runtime/export/delete-on-disable composition or end-to-end custody test; settings acceptance alone does not enable the mode safely. |
| WP28 Live View Optional Mode | Seven implementation and three test files provide fail-closed runtime/startup decisions, service readiness mapping, unsafe-retention/control rejection, and portal capability state. | **Incomplete** | The worker `start` function records a decision only; no frame transport, LAN mutual-auth session, relay/cache execution, viewer audit, platform prompt, teardown, or physical-device test is implemented. |
| WP29 Proof Tiers And Proof Packs | This is a no-code proof-routing packet. | **Complete for Phase 1** | Artifact generation and acceptance are Phase 3 and do not substitute for open implementation workpacks. |
| WP30 Test Suite Playwright Rollout PR Gate | Existing Rust and portal tests cover bounded slices, but the two named aggregate executable harnesses are absent. | **Incomplete** | `screen-plan-closure-audit.mjs` and `screen-plan-external-gates-proof.mjs` are missing, along with the final cross-platform/model/live-view rollout acceptance runner. |
| WP31 Screen Intelligence Router | Nine implementation and two test files implement typed route input/output, structured-first selection, protected/manual-required states, capture choices, policy sensitivity, and generated bridge drift coverage. | **Complete for Phase 1** | Production integration belongs to later pipeline work; this bounded router contract has its expected code/tests. |
| WP32 Browser Structured Extraction Before Screenshot | Nine implementation and one test file model bounded structured evidence and `no_screen_needed` selection. | **Incomplete** | No managed-browser DOM/accessibility producer invokes the router, and there is no integration test proving a real browser source skips screenshot capture when structured evidence is sufficient. |
| WP33 Managed Browser CDP Screenshot Capture Path | Both files cited by the workpack are absent. | **Incomplete** | The CDP page/viewport/crop capture harness and its bounds/custody/desktop-exclusion tests must be rewritten against current Rust ownership. |
| WP34 OCR Tesseract Baseline | The named executable evaluation script is absent. | **Incomplete** | Packaging, extraction, resource/failure-mode measurement, and comparison test code must be restored or replaced; prose about historical output is not executable evidence. |
| WP35 OCR PaddleOCR PP-OCR Evaluation | The named executable evaluation script is absent. | **Incomplete** | Local packaging/runtime, quality comparison, resource capture, and no-upload harness code must be restored or replaced. |
| WP36 Small VLM Guided Classifier Evaluation | Twenty-five implementation and two test files provide generic screen pipeline/local-adapter contracts, but the named classifier-readiness harness is absent. | **Incomplete** | No selected VLM worker executes detector-specific bounded crops in production, and model capability/resource/quality/fallback evaluation code is missing. |
| WP37 Household Mesh Screen Analysis Queue | Eight implementation and one test file define redacted-summary custody, provider claim/lease/result validation, ordered events, rejection reasons, and child-owned policy authority. | **Incomplete** | No service startup/composition publishes or consumes this runtime, and there is no real household-provider transfer/restart test. |
| WP38 Local AI Resource Scheduler Priority Queue | Four implementation and two test files implement priorities, singleton heavy-job admission, queue state, timeout/degraded behavior, and provider scheduling. | **Complete for Phase 1** | Screen-analysis production does not yet consume this scheduler, which is a WP17/pipeline composition gap rather than missing scheduler code. |
| WP39 Redacted Summary Only Remote Boundary | Five implementation and three test files enforce redacted-summary mesh payloads, no raw-image/provider authority, and unsafe live-view retention/control rejection. | **Incomplete** | There is no parent-approved remote summary delivery runtime with audit/custody, and no privacy/legal gate or end-to-end raw-upload-denial test at the external transport boundary. |
| WP40 Detector Prompt Packs And Schema Tests | The named detector prompt-pack executable is absent. | **Incomplete** | Detector IDs/prompts, closed output schema, malformed/confidence/uncertainty tests, and privacy-negative tests must be rewritten under Rust-owned contracts. |
| Screen Control Settings Inventory | Imported historical reference packet; no executable code is owned. | **Reference only** | Reconcile its claims through WP01/WP02; do not schedule it as a product implementation packet. |
| Screen Evidence Analysis Capability Guide | Imported historical reference packet; no executable code is owned. | **Reference only** | Reconcile its claims through WP01/WP02; do not schedule it as a product implementation packet. |
| Screen Evidence Analysis Schema Proposal | Imported historical reference packet; no executable code is owned. | **Reference only** | Reconcile its claims through WP01/WP02; do not restore its former TypeScript ownership. |

**Screen Phase 1 result:** all 43 imported packets are mapped. Of the 40
executable workpacks, 9 are complete for bounded code/expected-test writing and
31 still need production code or expected tests; the remaining three packets
are reference-only. No Screen tests, Enforcer acceptance, proof, physical
platform readiness, live-view readiness, or whole-plan completion is claimed.

### V0.8 Enforcement Control Phase 1 code/test audit - 2026-08-15

This audit traces the actual Rust/TypeScript production surfaces and focused
test source. It does not accept the six historically checked workpacks as
complete merely because their Markdown boxes are checked. Graph topology
satisfaction below means the reviewed files exist; the Phase 1 decision also
checks whether the named behavior and expected negative-test families are
written. No test-pass, proof, CI, or whole-plan completion claim is made here.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Contract Boundary And Effect Schemas | Four implementation and six test files provide Rust-owned generated Effect Schema contracts, branded non-empty identifiers, parsers, Rust protocol parity, and invalid capability/result/timer/audit cases. | **Complete for Phase 1** | No missing bounded contract or expected negative-test family was found. The workpack's old `packages/enforcement-domain` paths are stale; ownership now lives in schema-domain plus Rust. |
| WP02 Policy Decision Evidence References | Six implementation and three test files bind policy decision, evidence, target, source, schedule, approval, audit, and timer references; validation rejects incoherent/missing action-capable inputs and preserves dry-run, observe-only, ask-parent, report-only, and manual-required outcomes. | **Complete for Phase 1** | No missing bounded decision/evidence contract or expected negative-test family was found. Focused execution is Phase 2. |
| WP03 Adapter Capability Matrix | Six implementation and six test files define supported, unavailable, degraded, report-only, scaffold, manual-required, adapter execution, rollback, proof-level, and claim-state rows, with service read models and no-upgrade assertions. | **Complete for Phase 1** | No missing bounded matrix code/test family was found. Static capability truth does not establish live adapter execution. |
| WP04 Owned-Process Time Limit | Ten implementation and six test files implement PID/process validation, owned-process termination, already-exited/no-op, timer expiry/recovery/cancel, result journaling, and focused proof generation. | **Incomplete** | The executing command still lacks the complete persisted authenticated delivery/consume-once authority and receipt/rollback chain required before adapter dispatch. Only process termination executes; broad app blocking remains manual-required. |
| WP05 App And Game Session Handoff | Four implementation and four test files carry typed app/game timer/session summaries, source-gated preview evidence, persisted timer binding, unknown-session rejection, and parent-facing timer status. | **Incomplete** | The stored session binding is not yet a complete production enforcement handoff from authoritative app/game identity through trusted dispatch to an adapter result and rollback receipt; launcher/game/app foreground-duration coverage is not closed end to end. |
| WP06 Managed Browser Session Control | Five implementation and four test files implement owned profiles, loopback bridge reservation/polling, unmanaged/stale rejection, launch/status delivery, redacted metadata, and failure/degraded states. | **Incomplete** | Managed-browser evidence is not consumed by the enforcement executor: `adapter_outcome` executes only process termination. No managed-session intervention receipt, rollback owner, or exact service-level browser-evidence-to-action audit test exists. |
| WP07 Unmanaged Browser Fallback | Five implementation and two test files keep managed/unmanaged states separate, require process identity, map report/warn/ask-parent/terminate/relaunch/manual/unavailable outcomes, and explicitly reject exact URL/content claims for unmanaged processes. | **Complete for Phase 1** | No missing bounded fallback code/test family was found. The two workpack-named legacy scripts are absent, but their required code/negative behavior is present in Rust; rerunning it is Phase 2 and proof is Phase 3. |
| WP08 Network/Domain Report-Only Boundary | Six implementation and six test files preserve process/domain/IP/tunnel classifications, keep network evidence separate from browser URL evidence, force network blocking to report-only/manual-required, and reject exact-URL or unsupported authority upgrades. | **Complete for Phase 1** | No missing bounded report-only/no-claim code or expected negative-test family was found. A real filtering adapter is deliberately not claimed and would require a separate authorized workpack. |
| WP09 Timer Recovery And Rollback | Five implementation and four test files cover durable timer state, restart recovery, corrupt/unsafe recovery-needed state, expiry, next-check reporting, cancel/override, rollback success, and rollback-unavailable behavior. | **Complete for Phase 1** | No missing bounded timer/recovery code or expected negative-test family was found. The legacy TypeScript/script paths are stale; focused Rust execution remains Phase 2. |
| WP10 Parent Approval And Override | Six implementation and four test files provide typed request approval/resolution, replay/target/authority validation, child-runtime application, parent snapshot projection, and timer cancel override. | **Incomplete** | Approval, denial, expiry, duplicate, and override transitions are split across policy/activity/timer paths rather than one durable enforcement lifecycle. General approval expiry/override issuance-expiry and their enforcement-journal assertions are missing. |
| WP11 Audit And Journal Events | Thirteen implementation and four test files durably append rejected intent, before-dispatch, after-dispatch, adapter result, timer events, typed recent history, and completed-command retry recovery over the hash-chained Eventing journal. Recovery validates journal/store/outcome/timer/provenance identity, persists and returns the exact final report with its real completion time, and fails closed without reexecution on partial or mismatched custody. | **Incomplete** | Approval, denial, approval expiry, and general override transitions are not all produced through this journal, and no single focused matrix proves every adapter-result/timer/rollback family is durably ordered and queryable with all actor/target/policy/evidence/route refs. The retry slice is focused-green, but retained enforcement-specific proof remains open. |
| WP12 Child-Facing Status And Reasons | Four implementation and three test files define typed enforcement results, unavailable/manual states, reason constants, and a child enforcement decision boundary. | **Incomplete** | No production child-facing status/read-model consumer renders time-limit reached, ask-parent, allowed, warning, unavailable, degraded, schedule/target context, and offline behavior. Existing tests prove decision safety, not the required child surface. |
| WP13 Service Read Models And API | Six implementation and three test files expose separate policy-dispatch, capability-proof, integrity, and audit-history reports through service/WebSocket paths. | **Incomplete** | There is no coherent service-backed enforcement state contract/API joining capability row, proof level, evidence/route, active timer, pending approval, and recent action history, nor a consumer-validation test for that aggregate payload. |
| WP14 Portal Control State Consumption | Four portal panels and four tests render adjacent app/game, browser, network, and policy states from typed service data. | **Incomplete** | No dedicated enforcement control-state route consumes the missing WP13 aggregate, sends typed enforcement intents, and renders returned result/audit state. Current panels do not close dry-run/observe/active/degraded/manual/approval behavior as one surface. |
| WP15 Integrity Heartbeat And Permission Loss | Three implementation and three test files define integrity/permission/stale/stopped states and expose a service proof/read model with parent-visible alert rows. | **Incomplete** | The rows are constructed as a static proof report. No live heartbeat source drives fresh-to-stale transitions, no permission-loss runtime updates the model, and no service test advances time through heartbeat/stale/recovery behavior. |
| WP16 Tamper/Uninstall Non-Claim Design | One Rust implementation plus three tests generate and regex-check a typed static tamper/uninstall artifact-status contract with explicit manual-required/no-claim wording. | **Incomplete** | No production uninstall/removal/stop/permission-denied observation feeds parent-visible state, and no runtime lifecycle test proves residual-state visibility or cleanup. Static generated text is not a live integrity boundary. |
| WP17 Cross-Platform Unavailable States | Two implementation and two test files provide honest Windows/Linux/macOS/Android/iOS capability counts, manual requirements, unavailable states, and no-claim serialization. | **Incomplete** | The required parent desktop, parent mobile, child Windows, child Android, and web-authoring/visibility roles are collapsed into a generic platform matrix; web and owner-surface separation plus their negative tests are missing. |
| WP18 Proof Command And Matrix | One production proof-report builder and two test/proof files cover the supported-adapter report and owned-process slice. | **Incomplete** | `scripts/test/v0-8-enforcement-control-plan-proof.mjs` is absent, as are the declared managed/unmanaged browser and timer aggregate runners. The expected deterministic whole-plan matrix/verifier test code has not been written. |
| WP19 Playwright And UI Proof | Three adjacent portal panels and three tests provide browser/network/app-game rendering and one network E2E path. | **Incomplete** | No Playwright spec drives the real enforcement service through dry-run, observe-only, active timer, ask-parent pending/approved/denied/expired, unavailable, degraded, result, and audit states without mocks at desktop/mobile widths. |
| WP20 Rollout Docs And CI/PR Gate | Expected topology is `no-code-required`; this packet coordinates final docs, focused/full validation, PR review, CI, merge, and no-claim reporting. | **Complete for Phase 1** | No product or expected-test code belongs to this packet. It remains operationally open until the 13 incomplete workpacks close and Phase 2/3 promotion evidence exists. |

**V0.8 Enforcement Phase 1 result:** all 20 workpacks now have reviewed
code/test ownership. Six executable workpacks (WP01-WP03, WP07-WP09) and the
coordination-only WP20 have no remaining Phase 1 writing gap; 13 workpacks need
production code or expected tests. The first dependency chain is WP11 durable
transition-family completion -> WP04 trusted dispatch authority -> WP05/WP06
action handoffs -> WP13 aggregate service state -> WP14/WP19 product surfaces.
Proof regeneration remains Phase 3 and cannot close those code gaps.

### Portal UX / Household Surfaces Phase 1 code/test audit - 2026-08-15

This audit follows the current portal, portal-domain, parent-runtime bridge,
mobile shell, package, workflow, and focused test source. It does not treat a
rendered proof panel, fixture, screenshot helper, or checked workpack row as a
finished product path. Graph topology satisfaction means the reviewed roots
exist; the Phase 1 decision also checks whether the workpack's named product
behavior and expected negative-test families are written. No Phase 2 test-pass,
Enforcer, Phase 3 proof, CI, or whole-plan completion claim is made here.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Service-Backed Shell And Navigation | Portal shell, HostBridge, Rust-owned route/status snapshots, service-state decoding, and focused bridge/state tests are real. | **Incomplete** | The shell has no focused unauthenticated and no-household tests and derives access from LAN controller authority; it can fall back to proof-missing instead of owning the required authenticated household boundary. |
| WP02 Household First-Run And Profiles | A Rust-backed setup status panel, route snapshot, portal renderer, unit tests, and a screenshot E2E exist. | **Incomplete** | The Rust builder explicitly reports account/provider/trust/custody as not wired. There is no live first-run state machine, profile creation/selection, or transition/action test family. |
| WP03 Device Inventory And Source States | LAN/service read models feed portal device inventory and evidence/source states; focused Rust, portal, fixture, and E2E tests cover real snapshots and degraded inputs. | **Complete for Phase 1** | No missing bounded inventory/source-state writing gap was found. Runtime execution is Phase 2. |
| WP04 Selected Device Context | Normalized device selection persists through session storage without inventing stable IDs; unit and E2E tests cover persistence and wrong-target behavior. | **Complete for Phase 1** | No missing bounded selection-context or expected negative-test family was found. |
| WP05 Policy Authoring Control Center | Policy preview projection, precedence rules, generated cards, parent snapshot loading, and focused Rust/portal tests exist. | **Incomplete** | The portal only refreshes previews. Template/manual-rule authoring, save/confirm/cancel UX, and a trusted one-shot write boundary with replay/tamper/role/cancel tests are missing. |
| WP06 Schedules, Time Budgets, And Templates | Route metadata and narrow app/game schedule/session rows exist in the portal domain and scaffold tests. | **Incomplete** | There is no real schedule/time-budget/template authoring surface, typed save path, state matrix, or focused responsive/negative test family. |
| WP07 Parent Requests And Approvals | Narrow policy-resolution and app/game timer panels/actions have typed Rust and portal tests. | **Incomplete** | There is no general request inbox/detail/history surface covering approve, deny, bonus time, expiry, duplicate, stale, and unauthorized outcomes as one service-backed lifecycle. |
| WP08 Activity, Evidence, And Diagnostics | The network evidence drawer and diagnostics export render references, confidence, custody, degraded states, and live read models with focused tests. | **Incomplete** | Export copies broad live read models without an explicit sanitizer/redaction contract or secret/private-path negative tests, and evidence/custody behavior is not closed across the plan's activity surfaces. |
| WP09 Browser, App, And Network Surfaces | Browser intervention/explanation, app-game session duration, and network evidence panels are backed by stored typed data; tests cover managed/unmanaged, exact-URL boundaries, metadata-only network state, and degraded inputs. | **Complete for Phase 1** | No missing bounded surface/negative-test writing gap was found. |
| WP10 LAN Pairing State Consumption | LAN replay binding/provenance, device/source/diagnostic read models, generated command contracts, Rust projection, portal unit tests, and E2E fixtures exist. | **Incomplete** | The portal mainly displays state; first-class add, route, rename, trust, ignore, and revoke controls do not consume the generated command boundary with result/error tests. |
| WP11 Assistant Action Preview Flow | Assistant chat models, quick actions, scaffold/screenshot coverage, and a narrow policy-preview confirmation overlay exist. | **Incomplete** | No production assistant route consumes service events/citations/provider state and drives typed preview confirmation with authorization, cancellation, replay, stale, and safety tests. |
| WP12 Reports, Notifications, And Custody | App/game notification, social report, tracking export/retention, and custody-oriented proof panels plus focused render tests exist. | **Incomplete** | The surfaces explicitly avoid product-ready export/mutation/delivery claims; no cohesive service-backed report, notification, retention, delivery, and failure lifecycle is written. |
| WP13 Degraded, Empty, Stale, And Error States | Service-state decoding and multiple setup/network/screen/app-game panels have explicit missing, unavailable, degraded, empty, refresh, and retry tests. | **Complete for Phase 1** | No missing bounded honest-state writing gap was found. |
| WP14 Audit History And Copy Debug | Diagnostics panels expose event metadata and copy success/failure UI; export code and a focused export test exist. | **Incomplete** | Copy success/failure/reset has no focused UI test, redaction lacks a secret/private-path matrix, and there is no general ordered audit timeline across the touched routes. |
| WP15 Accessibility, Responsive, Keyboard UX | Responsive styles and selected mobile, keyboard, and accessibility E2E helpers/specs are present. | **Incomplete** | Coverage is limited to a few proof routes; the plan-wide key-flow, focus, long-label, error-overlap, and representative desktop/mobile route matrix is unwritten. |
| WP16 No-Fake-Data Contract Adapter | Strict bridge decoders, generated contracts, host failure behavior, labelled fixture/proof panels, and invalid/missing-boundary tests prevent silent fake-green fallback. | **Complete for Phase 1** | No missing bounded adapter/no-fake-data writing gap was found. |
| WP17 Playwright Screenshot Proof | Browser failure collection and several desktop/mobile screenshot specs cover console/page errors and named routes. | **Complete for Phase 1** | The executable harness is written; retained screenshots and DONE links belong to Phase 3. |
| WP18 Parent Mobile Shell Readiness | Real Android and iOS parent projects, dedicated package scripts/workflow smoke, portal responsive styles, and mobile E2E coverage exist. | **Complete for Phase 1** | No missing bounded mobile-shell/package-smoke writing gap was found; store/signing/runtime-authority closure belongs to owning distribution workpacks. |
| WP19 Product Docs And Checklist Sync | Expected topology is `no-code-required`; this packet reconciles product docs and checklist truth after implementation. | **Complete for Phase 1** | No product/test code belongs here. The packet remains operationally open until the other Phase 1 gaps and later validation/proof state are reconciled. |
| WP20 Manual User Review Gate | Expected topology is `no-code-required`; this is the final user-observation gate. | **Complete for Phase 1** | No product/test code belongs here. It remains open until the product paths are complete and the user performs Phase 3 review. |

**Portal UX Phase 1 result:** all 20 workpacks now have reviewed code/test
ownership. Nine workpacks (WP03, WP04, WP09, WP13, WP16-WP20) have no remaining
Phase 1 writing gap in their bounded scope; 11 need production code or expected
tests. The first dependency chain is WP01/WP02 household authority and first-run
state -> WP05-WP07 authoring/request actions -> WP10/WP11 typed command
consumption -> WP08/WP12/WP14/WP15 cross-surface safety and UX closure. Proof
regeneration remains Phase 3 and cannot close those code gaps.

### Eventing plan Phase 1 code/test audit - 2026-08-15

This is a source-and-test inspection, not a transcription of workpack status.
`code expectation satisfied` means only that the reviewed expected topology is
present; the Phase 1 column below additionally checks whether the workpack's
named behavior and negative tests are actually represented. Tests are not
claimed passing here unless a current run is explicitly named.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Source Boundary And Semantics Audit | Expected topology is `no-code-required`; the workpack is a decision and boundary audit. | **Complete for Phase 1** | No implementation is authorized by this workpack. Graph acceptance/proof remains a later gate. |
| WP02 Crate Contract And Type Boundary | 4 implementation and 5 test files cover branded IDs, live/stored envelopes, registry conflicts, serde, and version skew. | **Incomplete** | The plan-named public surface still lacks `SubscriberName`, `HandlerName`, and a production `JournalPath`; no property/fuzz family was found. |
| WP03 Dispatch Runtime And Lifecycle | 5 implementation and 4 test files cover aggregate ordering, cross-aggregate concurrency, nested/detached publish, panic isolation, retry, timeout, trace, and lifecycle disposal. | **Incomplete** | No exact multi-handler registration-order test, true parallel-handler timing/aggregation test, or explicit no-lock-held-await regression was found. |
| WP04 Queue, Idempotency, Dead Letter | 12 implementation and 2 test files cover overflow, TTL, duplicates, rollback, drain preservation, and journal failure. | **Incomplete** | The named retry-storm/DoS guard test is absent. |
| WP05 Request/Response Contracts | 4 implementation and 3 test files cover typed resolution, invalid/late/double response, timeout, abort, release, and durable result separation. | **Incomplete** | No compile-fail/static negative proves event payloads cannot carry deferred, socket, task, or completion handles. |
| WP06 Journal Replay And Lineage | 27 implementation and 16 test files cover NDJSON/hash-chain durability, replay, idempotent recovery, topology, compatibility, and the enforcement audit handoff. | **Complete for Phase 1** | No code/test-writing gap found. Current focused rerun and Enforcer belong to Phase 2; retained acceptance is already graph-done. |
| WP07 Parent Protocol Event Contracts | 8 implementation and 6 test files cover selected parent, child, network, enforcement, and household event contracts. | **Incomplete** | Several chain payloads are typed serde structs but not reusable `DomainEvent`s; no comprehensive namespace registry, publisher-authority negative, or cross-family version-skew suite exists. |
| WP08 Parent Runtime Integration | 7 implementation and 5 test files cover selected tracking, policy, child-domain, and enforcement-journal flows. | **Incomplete** | No generic validated parent-controller intent route, portal-cannot-publish static negative, or one adapter-result-to-audit-to-read-model integration chain was found. |
| WP09 Network Consumer Event Chain | Reviewed roots include exact-source capture ingestion, deterministic phase IDs/refs, recovered `ProductionFileEventJournal` startup wiring, startup/recurring reconciliation, fail-closed persisted-row validation, and projection-only service reads/streams. | **Phase 1 written; integration open** | Commit `4b7bf6e3f` has normal pre-commit/push complete; integration, CI, review, and merge remain open. |
| WP10 LAN Household Mesh Consumer | Code-drafted structural validation exists, but agent-core authority resolution is deliberately unavailable and cannot reach local republish. | **Incomplete / blocked** | Blocked on LAN WP26 signed-child beacon ingress and household-mesh authority handoff; canonical Eventing proof root is `output/eventing-plan-proof/10-lan-household-mesh-consumer/`. |
| WP11 Type Safety And Ownership Hardening | Accepted production source keeps live envelopes immutable, revalidates contract/aggregate/idempotency identity at live and stored decode, binds pending requests to their associated response type, fails closed for unsupported journal idempotency, and consumes a journal-minted non-cloneable replay authority. | **Production source complete; expected tests incomplete** | Existing journal-replay tests still call the retired records/mode API. Migrate them and add malformed envelope, aggregate/idempotency tamper, response mismatch, unsupported journal, replay single-use, mutation, lock/await, fixture-parity, and naked-string negatives before focused execution or proof. |
| WP12 Rollout Proof And PR Gate | Expected topology is `tests-only`, but `scripts/test/eventing-rollout-proof.mjs` is missing. | **Incomplete** | The five named rollout reconciliation/negative checks lack their runner; canonical proof root is `output/eventing-plan-proof/12-rollout-proof-and-pr-gate/`. |
| WP13 Test Folder Layout Regression Audit | Expected topology is `tests-only`; 38 external test files exist under `crates/ocentra-eventing/tests`, and no `src/` test module/entrypoint was found. | **Code complete; validation/proof open** | Current proof is absent; rerun must include `unit`, `contract`, `journal_replay`, `integration`, `version_skew`, and architecture lint before the canonical proof root can close the row. |

**Eventing Phase 1 result:** six current selectable rows remain after excluding
the historical rows: WP06 is the one doc-closed row; WP09-WP13 are open.
WP10 is blocked on LAN WP26, WP11 source is accepted but test-writing is incomplete, WP12
lacks its harness/root, and WP13 is code-complete but validation/proof-open.

### Data Custody plan Phase 1 code/test audit - 2026-08-15

This table records current source and test code, not the workpack's checked
status or ignored proof roots. Tests are not claimed passing unless a current
run is named.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Custody Source Of Truth | Six Rust source/generator files and the package-exported generated TypeScript contract implement the 28-class custody matrix. The old handwritten TypeScript adapters were deliberately removed by Rust-first convergence and are not required production source without a real consumer. | **Production source complete; expected tests incomplete** | The current Rust contract test covers serde and generated-file drift but not exact 28-row coverage, unique IDs, derived-source validity, hosting counts, redaction/notification rules, forbidden hosting, or no-claim flags. Write that invariant family in the expected-test wave; do not restore deleted TS test/proof files by habit. |
| WP02 Encryption Key Custody | The Rust contract/derivation now includes `encryption_key_custody_scope.rs`, which binds requested decrypt scope to the selected platform row instead of trusting caller match flags. | **Accepted source; expected tests open** | Write the complete cross-scope/wrong-holder/revoked/lost/manual-required negative matrix, then run focused schema/storage gates. Real platform key-wrapper consumers remain adjacent-owner work. |
| WP03 Parent Owned Cloud Sync | The Rust schema/derivation now includes explicit sync-manifest custody validation before claim-safe provider/sync state can be produced. | **Accepted source; expected tests open** | The declared provider-state negative matrix and edge tests are not current. Provider SDK/OAuth/upload/delete/retrieval execution remains adjacent-owner work and is not implied by manifest source. |
| WP04 Retention Delete Tombstone | Generic retention/delete derivation remains Rust-owned, while the durable tombstone/effect store, reconciliation, startup recovery, and terminal acknowledgement now live in the child-runtime owner and are reached by its internal custody command path. | **Accepted source; test migration open** | Two existing tests import the deleted storage-core store and must migrate or be rewritten against the child-runtime owner. Do not restore a core re-export. Provider/device propagation remains adjacent-owner work. |
| WP05 Export Import Backup Recovery | Import integrity now rejects dishonest bundles before preview/apply derivation. | **Source incomplete** | Backup cadence/manual-backup behavior and migration execution/rollback source remain absent. Complete those production boundaries before writing the full integrity/backup/migration/rollback expected-test matrix. |
| WP06 Report Query Custody | New Rust request/row validators fail closed on actor, household/child/source/citation binding before state derivation. | **Rust source accepted; edge source/tests open** | The declared thin TypeScript adapter/rules source is still absent, and the full authority/citation negative contract matrix is not written. Downstream report/notification/AI/portal execution stays adjacent-owned. |
| WP07 Rollout Proof And Route Gate | A real internal path now exists: `submit_storage_custody_action` → `PublishStorageCustody` → dynamic Device Trust readiness → `ChildStorageCustodyRuntime::execute` → durable effect/journal/tombstone lifecycle and startup recovery. | **Source reachable; trusted composition/tests open** | Default startup installs a manual-required authority; no Account/family trusted adapter or external upstream caller supplies the opaque handle. Moved-store tests are stale, and no current focused execution or aggregate clean-checkout proof is claimed. |
| WP08 Parent Storage Settings Apply Flow | Rust schema/generator, generated TypeScript contracts, storage-custody card/preview/apply/action/proof modules, and two Rust test files cover explicit storage modes, visible manual-required state, restore preview, wrong-household and partial-restore negatives, separate disconnect/delete actions, delete-kind coverage, and generated-contract drift. | **Production source incomplete; expected tests incomplete** | The apply input has no trusted confirmation receipt or confirmed state. Every preview sets `confirmation_required = true`, so runtime derivation rejects `Applied` and `Partial` unconditionally. Add the authority-bound confirmation/replay boundary and reachable confirmed outcomes, then write the positive and negative Rust test matrix. Deleted old TS adapters/tests/proof runners are not automatic requirements. |
| Migrated Data And AI UI Plan | The packet is a product/UI reference with proposed pre-contract read-model and intent names. Its explicit non-goals forbid UI implementation, route changes, provider/runtime work, model execution, and behavior changes in this slice. | **Complete for Phase 1** | No implementation or test code is required by this reference packet. Future production Data/AI UI work must be promoted into owning plan workpacks and typed contracts rather than attributed here. |

**Data Custody source-wave result:** all workpacks retain reviewed ownership,
but no DONE claim follows. WP02/WP03/WP04 Rust source is accepted, WP06 still
needs its declared TypeScript edge, WP05 retains backup/migration source gaps,
WP08 retains its confirmation-authority gap, and WP07 retains trusted
composition/external-caller gaps. Expected-test source,
focused execution, proof, precommit, CI, and PR remain deliberately later
phases. No Phase 2 passing-test or Phase 3 proof claim is inferred from this
ownership audit.

### Policy Control Plane Phase 1 code/test audit - 2026-08-15

This table records reviewed implementation and expected test code. It does not
promote a workpack from plan/checklist state or infer current passing tests and
proof. The plan is being audited in dependency order and will use one plan PR
after all eight workpacks are classified.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Policy Source Of Truth | Rust protocol/source modules, the generated-edge TypeScript policy contracts, one unit test file, and two version-skew test files define the complete 14-state lifecycle vocabulary, household/actor authority, versioned rules and schedules, custody metadata, compiled/enforcement/audit/rollback artifacts, duplicate/stale rejection, delivery-before-active, supersede/rollback, migration boundaries, and source-not-cache/UI/AI negatives. | **Incomplete** | `register_parent_policy_source_document*` has no production caller outside tests and there is no durable/query owner that makes the document the canonical household source or rejects wrong-household reads. The required `PolicyTemplate` entity is absent. The TypeScript `FamilyPolicySet` edge also has no direct TypeScript contract test. |
| WP02 Parent Authoring Preview | Rust preview/request contracts, parent-runtime projection/actions, the generated portal bridge, the rendered portal preview panel, five Rust test files, two parent-runtime integration files, and one portal test cover preview-before-save, conflicts, unsupported/manual/offline/stale visibility, assistant preview-only confirmation, authority rejection, attention precedence, and the no-enforcement claim. | **Incomplete** | No template or manual-rule authoring surface exists. The portal panel is read-only except refresh: there is no confirm/save/cancel control or typed one-shot opaque confirmed-request relay from UI to Rust source mutation. Parent-runtime tests manufacture actions directly, so they do not prove a portal caller. Mobile/accessibility behavior and cancel-no-mutation are not tested. |
| WP03 Domain Policy Compilers | Rust compiler code and generated literal contracts define all eight domain artifact shapes with deterministic/versioned/manual-required semantics. | **Incomplete** | No shipped caller loads an identity-backed durable `ParentPolicySourceDocument`, invokes `compile_screen_policy`, `compile_ai_policy_context`, or another domain compiler, and persists/delivers the result. Current callers are tests; the TypeScript surface is a schema adapter, and stale references to `packages/policy-domain` name a directory that is absent. Contract tests cannot close this production handoff. |
| WP04 Delivery Ack Audit | Rust delivery state-machine/receipt modules, child-policy and child-runtime handoffs, parent-notification projection, eleven policy delivery test files, and three cross-crate test files cover per-target state, ordering/idempotency, offline/retry/degraded states, audit/rollback/version linkage, redaction, schema hydration, request/artifact identity binding, and parent-visible manual-required fallback. | **Incomplete** | `PolicyDeliveryExecutionReceipt` has public constructible fields, and public `apply_policy_delivery_transition_with_execution_receipt` plus `apply_trusted_adapter_delivery_handoff` accept it without non-forgeable adapter authority or an inspectable execution trace. The only “trusted adapter” test fabricates the receipt in test code and advances `Applied`; no production caller owns that path. This is validated receipt evidence, not trusted execution authority. |
| WP05 Ask Parent Overrides | Rust request/approval/override state machines, child-policy/runtime handoffs, parent-notification projection, durable agent-service confirmation/resolution audit lookup, parent-runtime command actions, and seven focused test files cover parent confirmation, child/observer/revoked/wrong-household rejection, grant/deny/modify/request-expiry, double-submit/replay safety, audit refs, queued delivery binding, parent-visible degradation, and persisted resolution replay. | **Incomplete** | The portal exposes no approval/deny/modify/expire controls, so parent-runtime action tests construct payloads without a real UI caller. Agent-service resolution persists the decision but does not queue the child-runtime delivery path. Notification is a typed projection only, with no outbox/provider transport. Active overrides have no automatic expiry transition/restart test, and real apply/rollback still ends at WP04's untrusted receipt boundary. |
| WP06 Rollout Proof And Route Gate | This is legitimately a proof/routing workpack with no product implementation requirement. Its five named tracked proof artifacts and plan manifest exist; that proof content is intentionally not used to claim Phase 1 code completion. | **Incomplete** | The declared validation route is not executable as written: `packages/policy-domain` is absent, `packages/agent-protocol-domain` is an empty directory without a package, four named agent-protocol-domain tests are absent, and both portal test paths are stale (the live test is under `apps/portal/tests/policy/`). No dedicated rollout verifier checks accepted/missing roots and no-overclaim fields. |
| WP07 Schedule Time Budget Conflict Model | Rust source-time validation, schedule-contract validators, conflict detection, request/override models, generated TypeScript helpers, and five unit-test files cover reset/carryover shape, expiry ordering, explicit DST gap/overlap, clock-source/manual-required classification, deterministic priority conflicts, equal-priority manual review, request expiry, and bonus-time grant shape. | **Incomplete** | There is no runtime schedule evaluator computing window/DST/budget state from a trusted clock; validators accept caller-built boundary/status snapshots. No durable offline timer recovery exists, and `PolicyOverrideState::Expired` has no policy-request transition. Tests classify prepared DST boundaries but do not prove spring-forward cannot overgrant, fall-back cannot double-grant, clock-skew enforcement, restart recovery, or automatic bonus/override expiry. |
| WP08 Policy Event Model | Rust event registry/replay modules and two test files define all 23 required event families plus explicit dead-letter/manual-required events, typed scopes, stable aggregate/idempotency keys, Eventing `DomainEvent` contracts, duplicate/stale/conflicting-sequence handling, rollback scope, registry topology, version lock, and redacted summaries. | **Incomplete** | `PolicyEvent` contains no causation or correlation identifiers despite the workpack requirement. The event type and replay reducer have no production publisher, durable journal, consumer, or dead-letter projection usage outside tests; rollback linkage is typed but never resolves prior history. Tests exercise only constructed in-memory events and do not prove durable replay, parent-visible dead letters, or that serialized/logged paths cannot expose raw policy identifiers. |

**Policy Control Plane Phase 1 result:** all 8/8 workpacks are now inspected
and mapped from live source and test topology. All eight retain a concrete
production-reachability or expected-test gap. WP03's compiler contract is
substantial but has no shipped source-to-compiler caller; WP06 correctly maps
as no-product-code, but its stale/missing executable validation route also
prevents Phase 1 closure.
No Phase 2 passing-test or Phase 3 proof claim is inferred from this audit.

### Account Identity Family Phase 1 code/test audit - 2026-08-15

This table records reviewed implementation and expected test code. It does not
promote checked workpacks or historical proof into current completion. The
plan is being audited one workpack at a time before one whole-plan PR.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Auth Provider Decision | A narrow Cloudflare D1 adapter and environment ownership declaration store only provider-subject to Ocentra-account mappings. Two unit-test files cover persistence/update, lookup, uniqueness conflict, missing-binding manual-required behavior, malformed inputs, binding ownership, and configuration presence. | **Incomplete** | The adapter and schema initializer have no production caller outside tests. No external token-verification route, account/session runtime wiring, deployed D1 migration, or household/member/role/device authority implementation is connected to this boundary. The workpack itself records the provider decision as partial and forbids treating this storage helper as login or family-authority readiness. |
| WP02 Identity Household Role Model | Rust family records, authority/proof/record-derived handoff modules, three focused owner test files, and the provisioning consumer cover typed household/member/child/device records, role/action decisions, observer read-only behavior, cross-household/device denial, stale/revoked lease and device rejection, signed current-state authority proof, redacted identifier-only handoff, and setup readiness consumption. | **Incomplete** | Every TypeScript owner/test path named by the workpack is absent after the Rust migration. The general evaluator consumes caller-supplied `same_family`, membership, trust, freshness, and capability flags; both production consumers use that flag bundle, while the safer record-derived handoff has no production caller. `SupportAdmin` has no household action/audit-reference contract or dedicated minimized-boundary test, and Pending/Revoked/Disabled membership denial is not directly covered as a full state matrix. |
| WP03 Session Token Lifecycle | Rust lifecycle decisions and focused unit tests cover credential/action separation, session creation and rotation decisions, expiry and clock-skew states, logout/revocation/global-revocation rejection, replay rejection, freshness, scoped issuance, and audit-required/redacted result flags. Provisioning consumes the pairing-token authorization decision and has focused readiness tests. | **Incomplete** | The TypeScript owner package and tests named by the workpack are absent. The Rust API classifies caller-supplied lifecycle flags; it has no credential identifier/secret, issued/expiry time calculation, persistent session or refresh-family store, atomic rotation/replay registry, logout/global-revoke mutation, or real audit emission. Credential issuance has no production caller. Controller-lease and support/admin session classes are missing, and the only live CSRF/origin enforcement is on unrelated Cloudflare billing routes, not an account login/session surface. |
| WP04 Invites And Recovery Lifecycle | Rust invite/recovery decision modules and tests cover co-parent, observer, child-device, and household-transfer scopes; expired/revoked/replayed/non-single-use/wrong-household/wrong-role negatives; five recovery kinds; identity proof, throttling, uniform response timing, support-assisted child-evidence blocking, device-trust projection, and custody-handoff states. Provisioning consumes these decisions for pairing/readiness projections, while record contract tests cover invite, recovery, and setup-audit serialization. | **Incomplete** | The named TypeScript owner packages are absent. There is no durable invite/recovery store, runtime create/revoke/accept/consume flow, clock-derived expiry, atomic single-use transition, recovery transition engine, or audit persistence. The stored invite record omits purpose, inviter authority, device intent, lifecycle/replay state, and token custody. Rate limiting and enumeration resistance are caller-supplied flags. Provisioning fully validates only Pending invites; an Accepted invite bypasses replay, abuse, timing, and inviter checks. SupportAdmin can authorize any recovery kind, including household transfer, from caller flags without a persisted owner-approval/audit record. The custody handoff is only an enum consumed as a local blocker—no typed request/correlation reaches data custody. |
| WP05 Device Ownership Authorization | Rust household authority, signed current-state proof, record-derived handoff, focused authority/contract tests, the provisioning consumer, and the policy authenticated-delivery consumer cover household/account/member/device/trust/freshness checks, observer restrictions, child-agent denial, parent-only billing/export-delete decisions, remote-view/control action separation, controller-lease states, step-up context binding, wrong-household/device negatives, and proof anti-transplant checks. | **Incomplete** | The TypeScript owner package is absent, and four expected actions—create household, invite member, create child profile, and support/admin review—are not represented. The general production API still accepts caller-supplied ownership/trust/freshness state. `capability_granted` is one boolean, so it cannot distinguish a remote-view grant from remote-control authority. Controller lease is only a caller-supplied state with no lease identity, subject/action binding, or computed expiry. Authorization returns audit/elevated-confirmation flags but does not itself require the step-up assertion or emit an audit event. Only ChangePolicy has a downstream signed-proof consumer; remote, export/delete, and billing have no typed handoff/runtime consumer, and export plus delete are collapsed into one action. |
| WP06 Security Proof And Route Gate | This is legitimately an aggregation workpack with no product-code requirement. Its current route names the required security, negative-case, Cloudflare, UI, redaction, manual-gap, and adjacent-plan inputs. | **Incomplete** | The generated `output/` gate root is absent in this checkout; durable tracked proof exists only for the narrow WP01 storage adapter and WP08 authority packet. WP02-WP05 and WP07 have no clean-checkout retained aggregate artifacts. Every preceding workpack still has Phase-1 code/test gaps. The declared commands target absent `packages/family-domain`, a broad portal `account` filter, and an agent-protocol `account` filter with no dedicated aggregate verifier. The graph records no dependencies for this final gate despite its mandatory Account WP01-WP05/WP07/WP08 and Cloudflare WP06/WP08 inputs, so route ordering is not mechanically enforced. |
| WP07 Parent Account And Family Setup UI | A Rust-owned Start-route snapshot contract flows through the generated parent bridge into a portal route component. Rust integration, portal-domain projection, route-rendering unit, generated-bridge, and Playwright tests verify that the panel reaches the UI and honestly displays `unavailable`, `not wired`, route ownership, and withheld completion claims instead of fake readiness. | **Incomplete** | The implemented screen is deliberately a boundary-status placeholder, not the required setup state machine. The Rust snapshot contains no live account, household, member, child-profile, device, invite, recovery, support, security, or custody read model and exposes no setup actions. Tests explicitly assert that welcome and setup-complete states are absent. The portal-domain intent helper is not used by the actual route component, which renders the generated snapshot directly. Required create/join household, role visibility, device lifecycle, invite, recovery, support-access, account-security, manual-required, and full source/custody label states therefore have neither production UI behavior nor acceptance tests. |
| WP08 Rust Schema And Account Authority | Rust-owned family-reference templates/generators, generated TypeScript edge files, family-identity authority/lifecycle modules, two schema drift tests, two Rust contract suites, and three unit suites cover typed accounts/households/members/roles/devices/invites/recovery/sessions, cross-household rejection, observer restrictions, stale/revoked/replayed paths, controller leases, identifier-only handoff redaction, serde round trips, and checked-in edge drift. | **Incomplete** | The generated family-reference edge is copied from hand-authored TypeScript templates rather than projected from the Rust authority structs, and no test compares the TypeScript reference shape to the `family-identity-core` Rust records. The versioned household-authority handoff has no decoder/version-skew test that rejects an unsupported `schema_version` or malformed/unknown fields, so the required schema-incompatible fail-closed case is not written. Cloudflare persistence/runner wiring remains correctly outside this packet. |

#### Accepted Account source overlay - 2026-08-17

The audit rows above remain the pre-replacement baseline. Independent review
accepts `origin/codex/account-wp02-source-wave` at `35edb2830`, integrated
through `e69acf279`. The graph maps all five source deltas and records reviewed
implementation completion evidence only for the internally closed WP02 and
WP08 boundaries:

- WP02: sealed current account/member/household/role/device capability, durable
  repository/CAS/invariants, complete support-receipt binding, and no
  request/provider-subject fallback;
- WP03: repository-owned session identity, generation, expiry, freshness, and
  revocation plus an owner-derived lifecycle record;
- WP04: private owner-derived invite/recovery records with monotonic terminal
  state, while runtime issue/consume and custody delivery remain open;
- WP05: billing and support/admin operations consume current Account authority,
  while Device Trust/remote/export/delete composition remains open;
- WP08: strict Rust schema validation, non-forgeable capability issuance,
  durable repository ownership, and generated Account TypeScript parity;
- Cloudflare WP06: D1 authority adapter and the undeployed ordered migration
  source `0001`, `0002_account`, `0003_provider`, `0004_canonical`.

No test source was refreshed or executed in this source-only wave. Provider and
account-route composition, invite/recovery orchestration, remaining adjacent
consumers, the complete expected-test wave, focused validation, proof,
precommit, PR, CI, and merge are still open.

**Account Identity Family Phase 1 result:** all 8/8 indexed workpacks are now
inspected and mapped from live source and expected test topology. Five
workpacks have accepted replacement source; only WP02 and WP08 have reviewed
implementation completion evidence. None is DONE. Every row still has an
expected-test, composition, proof, or aggregate-gate gap. No Phase 2 passing-
test or Phase 3 proof claim is inferred from this audit.

The 2026-08-17 `ac03afee3a` Account WP02-WP05 source attempt was rejected after
caller tracing and independent P0/P1 review. Its new account/session/invite/
recovery records had zero production callers, no durable repository, public
deserialization of authority state, caller-supplied proof/replay/freshness, and
non-monotonic transitions. It remains quarantined remotely and is not counted
in any implementation total. It is superseded by the accepted `35edb2830`
packet and must not be revived. The remaining Account gaps are the shipped
provider/account-route composition, complete lifecycle and adjacent handoffs,
and the expected-test wave listed in the overlay above.

### Cloudflare Control Plane Phase 1 code/test audit - 2026-08-15

This table is based on the current Worker, binding, script, and test source. It
does not inherit the plan's historical dependency blockers or ignored proof
roots as present-day code truth, and it does not claim that any mapped test is
currently passing.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP00 Games Infra Parity Extraction | This is a Parent-only keep/adapt/strip decision packet with an explicit no-code boundary; the parity and source-surface documents reject game economy, Solana, matchmaking, social, AI proxy, and asset-storage expansion. | **Complete for Phase 1** | No product implementation or test code is authorized by this workpack. Current docs validation and retained proof are later acceptance gates. |
| WP01 Cloudflare Module Scaffold | Three mapped implementation files and four focused tests sit inside a real separate `infra/cloudflare` package with explicit scripts, dev/prod Wrangler files, generated billing contracts, runner/seed scripts, all seven test-family directories, and honest README-only placeholder subdirectories. | **Complete for Phase 1** | No missing scaffold/package-script test code was found. Placeholder runtime subdirectories remain deliberately non-claims and belong to their functional workpacks. |
| WP02 Wrangler Env Bindings | `env.ts`, separate dev/prod Wrangler configs, `.dev.vars.example`, and the binding test define exact D1/DO/KV/Queue/R2/analytics names, ownership/privacy states, placeholder-only secrets, explicit production origin, optional bindings, and unknown-key rejection. | **Complete for Phase 1** | Real resource IDs, secret custody, and deployed environment verification are Phase 2/3 or manual environment work, not missing config/test code for this boundary. |
| WP03 Worker Entrypoint Runtime Guards | The Worker entrypoint, env/redaction helpers, three unit tests, three security tests, and Worker boot/health tests cover env failure, CORS, content-length framing, request-size limits, kill switch, redacted headers, and dispatch ordering. | **Incomplete** | `scheduled()` is empty and has no reconciliation test. The core Worker has no production structured logger, and both dispatch catch paths return raw `Error.message`; no injected secret/path exception test proves those messages are support-safe. |
| WP04 Route Manifest And Domain Contracts | `routes.ts`, auth-route validation, generated billing contracts, exact-manifest unit tests, fixed-manifest property checks, and the API contract suite define all 25 route rows with method, auth, handler, request/response model, audit event/rule, and proof family. | **Complete for Phase 1** | No missing route-manifest or expected contract-test code was found. Provider semantics, real auth authority, and consumer runtime remain separately owned. |
| WP05 Auth Admin Support Boundary | Auth model/verifier, Worker handlers, unit/integration/security tests cover all seven auth states, no downgrade, parent/trusted-device/admin/support/queue rejection, manual-required adapter modes, Stripe HMAC, role separation, and redacted failures. | **Incomplete** | `local-safe-fixture` accepts arbitrary bearer text plus caller-controlled role/trusted-device headers and production remains manual-required, so no real account/device authority adapter exists. PayPal, Apple, and Google verification use locally invented HMAC or bearer-equality fixtures rather than the providers' authenticated webhook protocols; tests currently enshrine those fixtures instead of a manual-required/real-provider boundary. |
| WP06 Storage DO D1 KV R2 Queue Bindings | Env/config ownership, billing D1/KV/R2 read-model code, Worker DO/queue paths, and five existing tests cover binding names, privacy constraints, seed/read behavior, idempotent writes, enqueue failure, and dead-letter capture. The independently accepted source packet adds the binding-specific account migration, a canonical Account WP08 `v0.7` D1 read adapter, fail-closed provider-bound auth composition, production rejection of local fixture auth and missing queue secrets, and deterministic schema-contract build order. | **Incomplete** | No provider verifier is installed, so the Worker remains `503` / `manual-required`; the account migration has not run and the new authority adapter has no focused tests or retained proof. Account DO/KV remain absent, and `BillingControlDO`/`ReferralControlDO` still keep idempotency in an in-memory `Map` rather than `DurableObjectState.storage`, so restart durability is not implemented or tested. |
| WP07 Local Dev Seeding And Fixtures | Eight scripts/source files and four tests define start/dependency probes, generated fixture families, redacted correlated proof milestones, teardown ownership, Worker boot, webhook fixtures, and replay fixtures. | **Incomplete** | The seed commands only serialize fixture objects to stdout; they do not populate local D1, KV, R2, Queue, or Durable Object state. The workflow can label the seed path runnable without verifying persisted state through a booted Worker, and it probes Wrangler availability rather than launching and smoking the declared local command itself. |
| WP08 Testing Runner And Test Pyramid | The module runner enumerates all seven test families and the complete 30-file test tree; it rejects missing required files and emits family, proof-ID, and assertion-ID manifests. | **Incomplete** | The runner blindly reports declared assertion IDs but does not bind them to executed test cases or verify matrix coverage. The required account-identity migration/adapter integration test is absent, so the storage-facing handoff cannot be represented by the integration family. |
| WP09 Portal To Worker E2E Smoke | The Worker harness and one four-case test exercise `/auth/billing/status` for unauthenticated, active, grace, and manual-review responses with portal-safe fields. | **Incomplete** | This is a direct in-process Worker test, not portal-to-worker E2E: no portal component, portal transport, HostBridge/dev transport, or portal test imports or consumes the billing-status route. The repository search found no billing-status consumer under portal source. |
| WP10 Security Fuzz Property Observability | Worker/auth/redaction source plus 13 security, property, fuzz, and carried integration files cover secret non-disclosure, CORS/CSRF/framing rejection, fixed route invariants, idempotency matrices, malformed webhook bodies, and redacted response boundaries. | **Incomplete** | The “property” tests iterate fixed examples and the “fuzz” test is a deterministic ten-payload smoke rather than generated property/fuzz coverage. The core Worker still lacks correlated structured observability, raw exception messages remain exposed, and the request-smuggling family does not exercise its declared header-injection/newline cases. |
| WP11 Deployment And Environment Promotion | Package deploy commands and separate dev/prod Wrangler configs exist with placeholder IDs and explicit origins. | **Incomplete** | There is no deployment/promotion implementation, CI environment gate, post-deploy smoke runner, version capture, rollback command, or automated rollback test. Both commands still pass `--env` names that have no matching `[env.*]` sections, while all resource IDs and auth authority remain placeholders. |
| WP12 Payment Plan Handoff Gate | This is an aggregation/handoff workpack with no product-code requirement. A tracked blocked-state receipt exists and the payment plan consumes the upstream route as blocked. | **Incomplete** | No executable validator checks the required handoff fields, current accepted/missing roots, downstream acknowledgment, and no-overclaim rules. The workpack still describes ignored `output/` roots as accepted while the tracked receipt says none are accepted, and payment-side wording carries a different missing-root set. |

**Cloudflare Control Plane Phase 1 result:** all 13/13 workpacks are now
mapped from live code and expected tests. WP00-WP02 and WP04 are complete for
the code-and-expected-test-writing phase. WP03 and WP05-WP12 retain concrete
runtime, authority, persistence, local-dev, runner, consumer, security,
deployment, or gate-verifier gaps. No current test-pass, deploy, proof, or
payment-unblock claim is inferred from this audit.

### Payment Subscription Phase 1 code/test audit - 2026-08-15

This table is based on the current Rust billing and entitlement crates,
Rust-owned/generated billing schemas, Cloudflare Worker implementation, and
portal source/tests. A generated contract, fixture, or passing historical proof
claim is not counted as a live provider, durable ledger, or parent product path.
No mapped test is claimed passing until Phase 2 reruns it.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP00 Cloudflare Control Plane Handoff | This is an upstream dependency/handoff packet with an explicit no-product-code boundary. The Cloudflare plan and tracked handoff state can describe whether payment work is blocked without duplicating Worker implementation here. | **Complete for Phase 1** | No Payment-owned implementation or expected test code is authorized by this workpack. Its blocked/accepted handoff and retained proof remain dependency and Phase 3 concerns. |
| WP01 Product Pricing Entitlement | Seven mapped schema/generated-edge implementation files and two Rust contract tests define entitlement value shapes and validate proof-witness honesty. | **Incomplete** | The named pricing-matrix and entitlement-proof read-model modules and `packages/billing-domain` tests do not exist. No executable product/seat/referral pricing calculation, proration, or pricing-matrix negative suite was found; generated schemas and witness validation do not implement pricing authority. |
| WP02 Checkout Billing Portal | Schema/Worker source now includes Account-composite provider identity, exact nested provider-object binding, D1 mapping storage, durable mutation ownership, and hosted-route fail-closed states. Existing mapped tests cover typed requests, auth states, URL allowlists, idempotency, audit rows, and secret-free responses. | **Incomplete** | Account WP02's current-authority migration must precede Payment migration `0003`; no real provider session adapter or trusted account/device verifier exists. Expected migration ambiguity/collision and production-provider tests remain to be written. |
| WP03 Subscription Webhook Lifecycle | Billing-core classifiers plus Cloudflare Worker/DO source now include real `DurableObjectState.storage`, versioned leases, bounded retry/backoff, stale cursor rejection, monotonic receipt/outbox state, and replay that avoids re-running an accepted mutation. | **Incomplete** | Provider authentication/normalization remains incomplete outside the existing Stripe-shaped path, and the complete crash-window, lease-expiry, retry-exhaustion, collision, and Rust-lifecycle-to-ledger expected-test wave is not yet written. |
| WP04 Entitlement Delivery Gates | Entitlement access/snapshot source now removes caller-supplied signature/key authority and exposes only an explicitly unsigned, non-authoritative projection; TypeScript signed/status records fail closed to manual review. | **Incomplete** | No genuine issuer/verifier/device-trust bridge exists, so signed delivery remains intentionally unavailable. Existing signed-API tests/fixtures are stale and must be rewritten; no production consumer may treat the unsigned projection as access authority. |
| WP05 Invoice Tax Refund Dispute | Worker/read-model source now serializes provider mutations by canonical subject authority, persists monotonic receipts/outbox rows, binds nested provider object IDs, and provides bounded lease/retry/manual-required recovery. | **Incomplete** | A real provider-owned invoice/refund/dispute adapter and complete transition/rollback tests are still absent. Account authority migration ordering and legal/tax/region policy remain external/manual-required. |
| WP06 Security Privacy Observability | Billing review modules plus Worker auth/redaction code and ten mapped security/property/fuzz/unit tests cover secret non-disclosure, CORS/CSRF/framing rejection, idempotency examples, malformed payloads, and redacted boundaries. | **Incomplete** | The dedicated billing security/privacy/observability owner and its expected test are absent. The Worker has no production structured correlated logger, raw caught `Error.message` values can reach responses, and retry/dead-letter/provider-mode state is not durably observable. Fixed examples do not establish the required abuse/rate-limit/property/fuzz matrix. |
| WP08 Provider Adapter Portability | Billing provider enums/classifiers, Cloudflare verifier/dispatch code, and three mapped tests exercise Stripe plus fixture paths for Razorpay, PayPal, Google, Apple, and manual invoice. | **Incomplete** | Only Stripe has an HMAC-shaped verification path. Other providers use invented local HMAC or bearer-equality fixtures rather than official protocols, and there is no normalized adapter interface, server-owned provider selection/configuration policy, store verification, or missing-config fail-closed matrix. |
| WP09 Regional Payment Rollout | Worker fixtures/read model plus three integration tests expose public pricing, payment routes, and booted-worker behavior. | **Incomplete** | Runtime behavior is hard-coded to USD and treats Pakistan/manual invoice as a fixture string. There is no region/provider/currency/tax eligibility matrix, fallback policy, rollout gate, or negative test proving an unsupported or misconfigured region cannot charge. |
| WP10 Referral Growth Entitlement | Entitlement/Worker/read-model source now shares durable mutation ownership and rejects stale/conflicting replay; unsigned entitlement output cannot mint access. | **Incomplete** | Referral qualification still lacks trusted household/device/account authority and a complete durable grant/revoke/grace history contract. The expected anti-abuse, recalculation, crash-recovery, and audit test matrix remains open. |
| WP11 Parent Website Billing Dashboard | Rust-owned/generated parent-visible summary contracts plus generic portal manage-route projection/rendering code and four mapped tests prove the route scaffold and summary shape exist. | **Incomplete** | The named parent-domain billing dashboard source/tests are absent. The portal exposes only a generic subscription route scaffold; there is no live billing transport/read model, plan/seat/referral/invoice state, checkout/portal action, manual-required handling, or billing-dashboard interaction/accessibility test. |
| WP12 Support Admin Billing Ops | Support/admin Worker source now binds provider operations to Account-composite identity, exact provider object IDs, monotonic mutation receipts, and durable retry/manual-required state. | **Incomplete** | Account WP02 authority must land first, a real support/admin identity and provider adapter remain absent, and the expected authorization/replay/rollback/UI test wave is not written. |
| WP07 Rollout Proof And Route Gate | This is a final aggregation packet with no product-code ownership. Its expected role is to validate the preceding workpack outputs and routing without inventing implementation. | **Incomplete** | `output/payment-subscription-plan-proof/` is absent and the existing generic real-evidence script contains no payment assertions. No payment-specific executable verifier checks accepted/missing roots, assertion IDs, negative/rollback coverage, or no-overclaim behavior; that missing expected test code prevents Phase 1 closure even though proof generation itself belongs to Phase 3. |

**Payment Subscription Phase 1 result:** all 13/13 workpacks have reviewed
ownership. The independently accepted 2026-08-17 production-source wave closes
the caller-minted entitlement, provider-identity, in-memory Durable Object, and
non-recovering pending-mutation defects recorded by the earlier audit. It does
not complete Phase 1: Account migration ordering, genuine provider/entitlement
authority, several product surfaces, and the complete expected-test delta are
still open. No Phase 2 passing-test or Phase 3 proof claim is inferred.

### Device Trust Bootstrap Phase 1 code/test audit - 2026-08-15

This table follows the live Rust/runtime/test paths, including code outside the
plan's older three-root summary. Plan-local Node tests that only read Markdown
are recorded as document tests, not runtime evidence. No mapped test is claimed
passing until the Phase 2 rerun.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Device Trust Source Of Truth | Thirty-eight mapped implementation files and twelve existing tests cover the lifecycle repository, transition journal/sidecar generations, current device/signer binding, parent-presence custody, restart/replay, schema/path integrity, opaque references, and fail-closed Eventing delivery. The accepted continuation is integrated through `68717b5b7`; public household signer/verifier mint paths are removed and independent source review found no remaining internal P0/P1. | **Foundation/source accepted; expected tests and authority bridge open** | No shipped platform/passkey ceremony issuer or complete product composition owns bootstrap-through-revoke/reset/re-pair state. Existing tests predate parts of the accepted surface; the complete expected-test migration, focused validation, proof, caller integration, platform custody, Account WP08 -> Cloudflare WP06 current-authority bridge, and DONE remain open. |
| WP02 Local Key Sealing | Fourteen mapped implementation files and six tests cover a Windows DPAPI/current-user registry epoch, atomic sealed-record custody, wrong binding/revoked/generation negatives, an opaque one-shot staged-ceremony facade, and unsupported-platform behavior. | **Incomplete** | The parent-runtime facade is not registered in a desktop/native command path and no operational ceremony issuer exists. Android, Linux, iOS, and macOS custody plus encrypted recovery fallback are absent; the current slice is Windows-only and cannot close wrong-user/device/key/reinstall behavior across supported platforms. |
| WP03 Parent Step-Up Auth | Twelve mapped implementation files and four tests cover signed action/household/device/target-bound receipts, atomic ceremony custody, linked lifecycle validation, restart reconciliation, expiry, replay, trust-epoch changes, tampering, and schema round trips. | **Implementation-only authorized; normal dependency blocked** | The only production verifier is the fail-closed unavailable verifier. No passkey/WebAuthn, biometric, or OS-native ceremony acquires the proof, no runtime caller consumes it as a live high-risk action boundary, and no durable one-shot replay owner exists. Reviewed-implementation gates authorize only the bounded WP03 source packet against WP01, Account WP08, and Cloudflare WP06; they do not provide ceremony authority, tests, proof, runtime reachability, or completion. |
| WP04 Phone QR Approval Bridge | The sole mapped test asserts wording in the plan model. | **Incomplete** | There is no typed QR challenge/response contract, phone or desktop runtime bridge, one-shot/expiry store, audit append, or executable wrong-household/action/target/device/replay test. This is document coverage only. |
| WP05 Entitlement Device License | Eight mapped implementation files preserve an unsigned entitlement projection, crate-private fail-closed trust context, access decisions, limits, and freshness/expiry/revocation labels. Wire input cannot manufacture trusted snapshot state. Six existing tests are mapped but not accepted for the new surface. | **Source accepted; expected tests open** | No real issuer, signature/revocation provider, or production capability-unlock consumer exists. Existing signed-snapshot/private-context tests require migration, followed by focused runtime validation. |
| WP06 Recovery Reset Re-Pair | Sixteen mapped implementation files cover durable lifecycle reset/revoke/re-pair generations, encrypted versioned bundle construction, corruption/preflight handling, tombstone preservation, and a verified-parent/unavailable-executor boundary. Caller-minted restore authority is removed. | **Source accepted; expected tests open** | No real encrypted key-custody owner, restore executor, rollback/mutation composition, or shipped restore caller exists. The five mapped tests require review/migration before focused execution. |
| WP07 Child Tamper Uninstall | Twenty mapped implementation files now own durable removal/tamper custody, validation, transitions, restart recovery, current trust binding, service readiness/dispatch blocking, and Android bridge health failure; three older tests remain mapped. | **Source accepted; expected tests open** | No attested tamper producer, parent transport, or package/device-owner platform removal caller exists. The expected test family must prove restart blocking, identity/current-trust binding, and fail-closed platform handoff without fake anti-root claims. |
| WP08 Open Source Dependency Adoption | The tests-only packet and retained review matrix classify WebAuthn, passkey, keyring, encrypted-bundle, and RustDesk candidates while keeping every trust root explicit. | **Bounded research writing present; graph validation open** | No implementation is owned by this research workpack and graph state is VALIDATION, not DONE. Any selected dependency still requires a separately authorized runtime adapter slice and later validation/proof. |
| WP09 Cross Plan Route Gate | Three mapped tests verify the test-category folders and selected plan/index wording. | **Incomplete** | No executable aggregator validates the required route-gate fields, accepted/missing proof roots, adjacent typed handoffs, blockers, manual-required gaps, or allowed/blocked claims. Document/taxonomy assertions cannot authorize readiness. |

**Device Trust Bootstrap source-wave result:** all 9/9 workpacks have reviewed
code/test ownership, but zero are graph-DONE. WP01/WP05/WP06/WP07 have accepted
production-source checkpoints through `68717b5b7`; every workpack still retains
external runtime, expected-test, validation, or proof gaps. WP01 has reviewed
implementation evidence for its bounded current owner packet, but tests,
focused validation, proof, production caller integration, and DONE remain open.
WP01 is not a shipped authority path: Account WP08 and Cloudflare WP06 must
bridge current authority before WP03's parent ceremony, and LAN/child current-
binding consumers follow WP03. WP02 remains conditional for platform sealing
and lifecycle/revocation composition.
WP03 remains blocked in the default graph; its implementation-only phase route
is authorized against the reviewed WP01, Account WP08, and Cloudflare WP06
source owners and does not alter normal READY/DONE state. The plan's real Windows custody,
lifecycle, step-up-proof, entitlement, recovery-contract, and tamper-status
libraries materially reduce the remaining work, but none is a live end-to-end
trusted-device product path. No Phase 2 passing-test, platform, or Phase 3 proof
claim is inferred.

### Setup Install Provisioning Phase 1 code/test audit - 2026-08-15

This audit separates setup-owned journey contracts from sibling-owned account,
package, runtime, LAN, trust, custody, policy, payment, and portal completion.
Historical workpack completion text is not treated as code evidence: several
named `setup-domain`, `production-domain`, and `parent-domain` source/test files
are absent from the current repository. No mapped test is claimed passing until
Phase 2.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Family Web Info Site | Expected topology is `no-code-required`; this workpack owns the public route/data-boundary decision and deployment blocker, not a site implementation. | **Complete for Phase 1** | No setup-owned product code or expected test code is authorized. A deployable family-site package, browser accessibility, preview, and custom-domain proof remain future implementation/adjacent-owner work, not evidence supplied by this packet. |
| WP02 Registration Login Entry | Expected topology is `no-code-required`; the workpack defines route labels and an account-identity handoff while explicitly excluding provider, session, household, invite, and recovery implementation. | **Complete for Phase 1** | No setup-owned implementation is authorized. The historical fill section names `setup-domain` and `production-domain` files that do not exist on current `main`; that stale proof wording must be reconciled in Phase 3 and cannot establish account readiness. |
| WP03 Parent Install Journey | Expected topology is `no-code-required`; the workpack owns visible install/platform/version/integrity labels and a runtime-distribution handoff, not package mechanics. | **Complete for Phase 1** | No setup-owned package code is authorized. The historical fill section names removed/nonexistent setup, production, and parent-domain files, so it cannot prove a live parent install journey; signed package/update/rollback execution remains with the distribution plan. |
| WP04 Child Install Permission Journey | Expected topology is `no-code-required`; the workpack owns the journey vocabulary and sibling-owner handoff, not child runtime, platform permission, package, trust, LAN, or policy code. | **Complete for Phase 1** | No setup-owned implementation is authorized. The old proof narrative references workspace packages no longer present; real installed/running/permissioned/paired/trusted/policy-ready inputs remain sibling-owned and must not be inferred from those claims. |
| WP05 Pairing Readiness Recovery | Nine mapped implementation files and five tests cover typed account/parent/child/permission/pairing/policy/custody/network/recovery states, readiness and action decisions, Eventing contracts, wrong-household/device/anonymous/replay/stale/revoked/offline/permission negatives, redacted audit events, and a child-runtime preflight consumer. | **Complete for Phase 1** | No missing setup-owned code or expected test family was found. The crate/runtime APIs have no concrete service composition caller, but physical LAN, trusted-device authority, and product service startup are explicitly sibling-owner/Phase 2 integration boundaries rather than missing WP05 state-model code. |
| WP07 First Run Setup UI And State Machine | The reachable desktop-to-Rust-to-generated-bridge-to-portal path now renders a Rust-owned 15-of-15 explicit `not-run`/`manual-required` authority matrix. LAN selected/paired/reachability is kept as separately labelled non-authoritative observation, Start never dispatches a LAN command, and an attempted discovery scan on Start is rejected. | **Accepted fail-closed source; tests open** | No authenticated owner supplies the 15 readiness authorities, so the evaluator and action planner correctly do not run. The state machine, guided actions, completion guard, and current Rust/portal-domain/portal/E2E expected-test families remain unwritten or stale. |
| WP06 Rollout Proof And Route Gate | Expected topology is `no-code-required`; this final packet aggregates proof and blockers without owning product implementation. | **Complete for Phase 1** | No product code is authorized. Its ignored/local proof roots, stale package references, route reconciliation, and sibling blocker acceptance remain Phase 3 work and cannot produce PR_READY from the current workpack labels alone. |

**Setup Install Provisioning Phase 1 result:** all 7/7 workpacks now have
reviewed code/test ownership. WP01-WP06 have no remaining bounded Phase 1
writing gap, including the real WP05 readiness model. WP07's accepted source
is now honest and reachable, but deliberately refuses to invent readiness: it
still needs authenticated owner inputs, the live first-run state machine and
completion guard, plus the complete current expected-test wave. Thus 6/7 are
complete for code/expected-test writing; accepted fail-closed source is not
product-journey completion.
No Phase 2 passing-test or Phase 3 proof/PR_READY claim is inferred.

### Parent Client Runtime Distribution Phase 1 code/test audit - 2026-08-15

This audit follows the current parent web, Tauri desktop, Android parent, iOS
parent, route-bridge, workflow, and release-helper source. The 20 historical
desktop-only workpack files that are not selected by `WORKPACK_INDEX.md` remain
legacy reference material and are not silently counted as additional current
workpacks. Enforcer-hosted proof scripts were inspected where package scripts
route to them, but only repository-owned files are recorded as graph roots. No
mapped test is claimed passing until the Phase 2 rerun.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Parent Client Scope And Route Boundary | Expected topology is `no-code-required`; this packet owns the canonical parent-client versus setup/child-runtime/portal ownership decision. | **Complete for Phase 1** | No product implementation is owned by this routing packet. Its boundary still has to be obeyed by WP02/WP03/WP06; their current code does not become correct merely because WP01 documents the split. |
| WP02 Parent Web Portal Distribution | Six implementation files and three tests provide pathname-based preview/staging/production rendering, wrong-route rejection, parent-only copy, and deterministic unit/Playwright coverage. | **Incomplete** | Authentication is considered successful unless the URL says `?auth=missing`; cache freshness is likewise a query parameter, and the enabled action button has no production action. There is no hosted deployment configuration or real session/cache input. The generated TypeScript claims a Rust source file that is absent. This is a distribution-status demo, not a real hosted authenticated parent portal boundary. |
| WP03 Parent Desktop Shell Package | Five implementation files and three tests provide a real Tauri shell, built-portal frontend binding, invoke/listen route commands, subscriptions, a TCP reachability probe, and Windows package/build anchors. | **Incomplete** | The platform state hard-codes active controller lease, local-network route/custody, and implemented child-agent/AI-provider roles even when the service is unavailable. Route loading calls `parent-runtime-core` in-process rather than decoding a typed local-service response, and the dedicated CI workflow type-checks/builds but does not package and exercise the Tauri artifact. Expected negative tests for unauthorized/version-mismatch/stale service responses are absent. |
| WP04 Parent Android Package | The parent-only Gradle project, launcher activity, versioned/checksummed APK builder, dedicated CI emulator install/launch/uninstall workflow, repository topology test, and Enforcer parent-package/source-boundary harness are written. | **Complete for Phase 1** | Release signing and Google Play publication remain manual/Phase 3 inputs. Phase 2 must rerun the build and emulator smoke; this row does not claim store readiness or child-agent authority. |
| WP05 Parent iOS Package | The parent-only Xcode project, simulator status app, versioned/checksummed package builder, dedicated macOS CI simulator install/launch workflow, repository topology test, and Enforcer source-boundary harness are written. | **Complete for Phase 1** | Device signing, provisioning profiles, TestFlight, App Store review, and a physical-device run remain manual/Phase 3 inputs. Phase 2 still must rerun the simulator build/smoke. |
| WP06 Parent Local-Service Route Bridge | Twelve implementation files and twelve tests cover the Rust route snapshot schema, route subscription delivery/deduplication, LAN read-model projection, and desktop command surface. | **Incomplete** | The desktop shell only probes whether a TCP socket accepts, then constructs package/authority fields locally. It does not perform a typed health/version/auth handshake or load route state from a service transport; `parent_load_route` directly calls the in-process parent runtime. Hard-coded controller, source, and custody state can therefore disagree with the reachable service. |
| WP07 Parent Client Signing Store Matrix | The desktop state and release-support helper expose manual-required signing/notarization/store labels, with general packaging assertions. | **Incomplete** | There is no parent-client per-artifact authority matrix or focused test. The helper incorrectly labels Android Play and iOS TestFlight rows as `child-mobile`, and it records no parent Android/iOS artifact hash, certificate, provisioning, notarization, store-review, or signing-authority state. |
| WP08 Parent Client Update Rollback | The desktop status shape and release-support helper distinguish scaffold, unavailable, and manual-required labels; selected assertions preserve those labels. | **Incomplete** | There is no parent-client updater: no manifest fetch, signature/checksum verification, apply state machine, durable update journal, rollback executor, failure recovery, or tampered/replayed manifest tests. Static unavailable/manual labels are honest but are not update/rollback implementation. |
| WP09 Parent Client Launch Smoke Matrix | Web Playwright launch coverage, desktop launch/build anchors, Android emulator smoke, iOS simulator smoke, repository packaging assertions, and the Enforcer four-row matrix harness are written. | **Complete for Phase 1** | The matrix harness is Enforcer-owned rather than a repository code root, and actual smoke outcomes remain Phase 2/manual evidence. Launch smoke does not establish auth, setup, signing/store, update/rollback, or child-runtime readiness. |
| WP10 Setup Handoff Contracts | Six nearby implementation files and three tests expose the Start-route setup panel through the Rust bridge and portal projection. | **Incomplete** | This is the setup status panel already audited under Setup WP07, not an explicit setup-to-parent-client-distribution request/response contract. There is no package target/version/integrity/install-precondition handoff, expiry/replay protection, consumer acknowledgement, compatibility contract, or focused boundary test. |
| WP11 Proof CI Release Gate | CI detects parent desktop/mobile changes and has dedicated desktop build, Android emulator, and iOS simulator jobs; generic release/packaging assertions and a release-support helper exist. | **Incomplete** | There is no executable parent-client aggregate gate that validates WP01-WP10 required fields and accepted/missing roots. The production release workflow builds the child Windows agent MSI, not signed parent desktop/mobile artifacts, and no parent-client promotion gate enforces signing/store, setup handoff, negative, teardown, rollback, or allowed/blocked claim truth. |

**Parent Client Runtime Distribution Phase 1 result:** all 11/11 current
workpacks now have reviewed code/test ownership. WP01, WP04, WP05, and WP09 are
complete for code/expected-test writing within their bounded routing/package/
smoke scope. WP02, WP03, WP06, WP07, WP08, WP10, and WP11 retain real runtime
or expected-test gaps. The plan's seven document-claimed closures therefore do
not establish product or release completion. No Phase 2 passing-test, signing,
store, or Phase 3 proof/PR_READY claim is inferred.

### Child Agent Runtime Distribution Phase 1 code/test audit - 2026-08-15

This audit follows the actual Windows MSI/service harness, macOS launchd
package, Linux systemd package, Android child application, iOS capability
application, Rust-owned shared contracts, smoke scripts, and CI/release
workflows. Enforcer-hosted legacy proof runners were inspected where repository
commands route to them, but external Enforcer files are not counted as
repository code roots. No mapped test is claimed passing until Phase 2.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Child Agent Scope And Route Boundary | Expected topology is `no-code-required`; this packet owns the child-versus-parent/setup/trust/runtime distribution boundary. | **Complete for Phase 1** | No product implementation is authorized by the routing packet. The remaining workpacks must still obey its package/install/runtime/respawn/uninstall separation. |
| WP02 Child Windows Service Package | The WiX MSI, WinSW/service configuration, signed-update-manifest builder, installer, artifact verifier, elevated lifecycle harness, CI MSI install/uninstall smoke, and repository asset test are written. The lifecycle harness distinguishes install, start, stop, restart, service-manager recovery state, uninstall, and residual authority cleanup. | **Complete for Phase 1** | The existing CI smoke exercises install/start/uninstall only; the fuller harness still needs an elevated Phase 2 run. Crash-loop and reboot recovery remain explicitly unexercised/manual rather than missing core harness code. |
| WP03 Child macOS Service Package | The launchd plist, `pkgbuild` script, pre/post-install launchctl hooks, checksum output, payload smoke, thin package-proof contract/test, and macOS CI package job are written. | **Incomplete** | The smoke only expands and inspects the package payload. No real-host install/bootstrap/health/kill-recovery/disable/uninstall/cleanup harness or corresponding negative tests exist; signing and notarization automation are also absent. |
| WP04 Child Linux Service Package | The baseline-pinned `.deb` builder, systemd unit, package hooks, checksum/baseline test, contract test, CI job, and smoke code cover extraction health plus `dpkg` install/remove/purge. | **Incomplete** | Maintainer scripts ignore `systemctl` failures, and the smoke never asserts the installed service becomes active, survives a crash through `Restart=always`, or cleans runtime/data state after service-managed execution. Those expected lifecycle tests are not written. |
| WP05 Child Android Agent Package | The Gradle child app, declared foreground service, debug-APK/checksum builder, lifecycle/device-proof contracts, CI contract jobs, and emulator install/launch/process/uninstall smoke are written for the explicit debug-sideload mode. | **Complete for Phase 1** | Device-owner, managed-profile, Play Store, reboot recovery, and physical-device authority remain intentionally manual/Phase 2-3 boundaries; this row does not infer them from emulator launch. |
| WP06 Child iOS Capability Package | Rust owns the capability contract and generated TypeScript, with Rust contract tests, a current repository proof harness, the Xcode capability-only app, simulator ZIP builder, simulator install/launch/uninstall smoke, and dedicated CI jobs. | **Complete for Phase 1** | Physical-device provisioning/signing, supervision, store distribution, and background execution remain explicit manual or unsupported boundaries rather than hidden daemon claims. |
| WP07 Child Managed Service Respawn | Windows, macOS, and Linux service-manager declarations plus Android/iOS limitation surfaces are present, with nearby platform contract tests. | **Incomplete** | No repository-owned managed-respawn contract/test exists. The Enforcer legacy runner still imports deleted `schema-domain` source/test files and proves configuration strings rather than executing kill, reboot, service-manager restart, and deliberate-stop behavior. Real host/emulator respawn test code is missing. |
| WP08 Child Parent Authorized Uninstall | `child-enforcement-core` retains a Rust-owned generator for the uninstall/tamper status read model and one contract test; platform package smoke paths provide nearby removal mechanics. | **Incomplete** | No production parent authorization, one-shot/replay guard, trust revocation mutation, audit append, platform adapter dispatch, or teardown workflow consumes this contract. The legacy Enforcer runner targets deleted TypeScript schema/consumer tests, so it cannot establish current expected-test coverage. |
| WP09 Child Signing Store Device Owner Matrix | Rust owns the five-platform matrix and generated TypeScript with a focused Rust contract test; release builders expose the actual unsigned/debug/simulator artifact states. | **Complete for Phase 1** | The legacy Enforcer proof command still expects removed TypeScript adapter/test files and must be rerouted during Phase 2. Actual signing, notarization, repository/store publication, and mobile enrollment artifacts remain later platform/release work, not missing matrix code. |
| WP10 Setup Device Trust Handoff | Eight Rust contract modules and one contract test define branded request/response identities, platform/artifact state, manual-required state, replay-guard reference, route sync, and no-claim boundaries. | **Incomplete** | Repository-wide usage is limited to schema tests. No setup/trust producer constructs the request, no distribution consumer validates/acknowledges it, no durable replay/expiry owner exists, and no end-to-end wrong-household/device/package/replay test is written. |
| WP11 Proof CI Release Gate | CI detects and builds all five child package targets; package-preview runs Windows/Linux/macOS/Android/iOS smoke code, while repository release/version/packaging tests cover the workflow surface. | **Incomplete** | The production release workflow publishes only the Windows child MSI. There is no executable child-plan aggregate gate over WP01-WP10, no signed/notarized/store promotion path for the other platforms, and no release decision that rejects missing lifecycle, respawn, uninstall, handoff, or platform-authority evidence. |

**Child Agent Runtime Distribution Phase 1 result:** all 11/11 current
workpacks now have reviewed code/test ownership. WP01, WP02, WP05, WP06, and
WP09 are complete for code/expected-test writing within their bounded route,
Windows-harness, debug-emulator, capability-only, and matrix scopes. WP03,
WP04, WP07, WP08, WP10, and WP11 retain concrete lifecycle-test, runtime,
handoff, or release-gate gaps. The plan's ten document-claimed closures
therefore do not establish child runtime or release completion. No Phase 2
passing-test or Enforcer claim and no Phase 3 proof/PR_READY claim is inferred.

### Logging Domain Parity Phase 1 code/test audit - 2026-08-15

This audit follows the live TypeScript logging package, Rust logging core,
portal and agent-service routing, deterministic evidence wrappers, MCP query
server, lifecycle controls, and proof-trace test code. Historical proof roots
and checklist marks were not used to decide whether implementation or expected
tests exist. No mapped test is claimed passing until the Phase 2 rerun.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Current State And Reference Audit | Expected topology is `no-code-required`; the packet owns the reference/current-state inventory and routes implementation to the remaining workpacks. | **Complete for Phase 1** | No product implementation or test code is authorized by this audit-only packet. Its missing retained proof is Phase 3 work. |
| WP02 TypeScript Logging Package Parity | The package now exports one canonical redaction owner. Its sanitizer preserves safe Date/URL/custom-`toJSON` values, recursively redacts serialized keys, emits explicit JSON-safe markers for unsupported/reflection failures, and never leaks through cycles, getters, proxies, or throwing `toJSON`. | **Accepted source; tests open** | The expected edge matrix is not yet written: unsupported primitive/object values at root/object/array positions, Map/Set/class values, Date/URL, native custom-`toJSON` keys and one-call semantics, nested secrets, cycles, throwing getters/traps/proxies, JSON-stringify safety, and package export resolution. |
| WP03 Parent Logging Architecture And Routing | The portal bridge-first path remains, and the real Vite dev-log route now consumes the canonical logging-domain writer instead of a duplicate policy. | **Accepted source; tests open** | A real Vite-route/middleware regression must prove canonical import resolution, nested redaction, invalid payload behavior, and no duplicate policy path. Existing stand-in-server coverage is insufficient. |
| WP04 Rust Logging Core Crate | Fifty-two Rust/fixture implementation files and seventeen tests cover typed events/levels/sources/fields, NDJSON durability and recovery, artifacts, redaction, path safety, diagnostics/run records, snapshots, concurrency, subprocess recovery, TypeScript fixture parity, and the agent-service consumer. | **Complete for Phase 1** | No missing core implementation or expected test family was found. Cargo, clippy, consumer, and fixture reruns remain Phase 2. |
| WP05 Local Validation Evidence | Eight implementation files and two tests provide `agent:run`, `agent:query`, `codex:evidence`, artifact/NDJSON/DuckDB storage, compact summaries, and parsers for rustc, clippy, Cargo tests, TypeScript, ESLint, npm, architecture, and no-reexport diagnostics. | **Incomplete** | The parser families have no direct fixture-driven unit matrix. Current tests prove run/command identity and a generic controlled failure, but do not exercise each required diagnostic grammar, duplicate aggregation, or malformed-line fallback. |
| WP06 Validation And Enforcement | Four validator scripts and five nearby tests/smokes check package layout, exports, wrapper guidance, portal/service routing, local evidence, and invalid bridge payload handling. The current route checker recognizes the implemented portal-domain bridge-first path and logging-core delegation. | **Incomplete** | No fixture-based negative test invokes the validators against a missing bridge, an unimplemented endpoint, or missing exports/wrappers. Invalid payload rejection is tested, but the required validator failure matrix itself is unwritten. |
| WP07 MCP Query Interface | Query reads now validate lexical and resolved containment, reject symlinked path components and recursive escapes, and redact local absolute paths from malformed-NDJSON diagnostics. The existing thirteen-tool surface remains routed through the shared query service. | **Accepted source; tests open** | Add symlink/junction artifact and recursive-NDJSON escape negatives, valid contained-read coverage, malformed-NDJSON path-redaction assertions, all general tool families, and the DuckDB-absent/stale NDJSON fallback. |
| WP08 Logger Instrumentation And Adoption | Logger calls now consume the canonical fail-closed sanitizer, so serialization cannot silently omit unsupported values or escape nested redaction. Existing portal/Rust/evidence instrumentation remains. | **Accepted source; tests and enforcement open** | Add logger-bridge serialization/non-throw regressions for the full sanitizer edge matrix. A checker/negative test still must prevent new raw console/ad-hoc JSON writers, and health/runtime adoption remains narrower than the workpack target. |
| WP09 Log Control Retention And Bridge Lifecycle | Ten implementation and seven tests cover separate console/storage decisions, always-stored warning/error levels, source/file/run debug selection, local/tunnel/disabled modes, scoped wipe, configurable retention, bridge health, run-start metadata, stale-run rejection, invalid payload rejection, and script behavior. | **Complete for Phase 1** | No missing lifecycle/control implementation or expected test family was found. Focused test execution and retained proof remain Phase 2/3. |
| WP10 Proof Trace Pipeline | Six implementation and five test files provide proof/correlation fields, bridge run-start and stale wipe, ordered portal trace emission, flush, DuckDB ingest, CLI/MCP queries, missing-step reporting, and cleanup of proof-mode globals. | **Incomplete** | The query service computes `outOfOrderSteps`, but no test creates an out-of-order trace and asserts that failure. The happy path only proves the empty result, leaving one explicit negative behavior untested. |

**Logging Domain Parity Phase 1 result:** all 10/10 workpacks have reviewed
code/test ownership. The accepted source repair materially advances WP02,
WP03, WP07, and WP08 without claiming their deferred tests. WP01, WP04, and
WP09 have no bounded writing gap; WP02, WP03, WP05-WP08, and WP10 retain
concrete expected-test or instrumentation-enforcement gaps. The plan's source-present
and partial-proof labels therefore do not establish Phase 1 completion, and no
Phase 2 passing-test/Enforcer or Phase 3 proof/PR_READY claim is inferred.

The accepted Logging source integrated through `3fec0793a` additionally makes
Rust the exact 18-key sensitive-key policy owner, regenerates the checked-in
TypeScript parity artifact, and routes the TypeScript sanitizer, canonical dev
writer, Logger, and portal compatibility fallback through that one fail-closed
policy. The explicit expected-test matrix, focused execution, proof, and
external runtime composition remain deferred; this source checkpoint does not
change any DONE or PR-ready state.

### Remote Access Phase 1 code/test audit - 2026-08-15

This audit follows the live Rust capability, grant, session, child-runtime,
screen-live-view, and agent-service paths. It also searches for production
callers and relay/security behavior, so a type name, environment flag, or
boolean execution record is not counted as a working relay. Historical proof
roots and checklist marks were not used to decide whether implementation or
expected test code exists. No mapped test is claimed passing until Phase 2.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Remote Capability Fabric | Three implementation files and two contract test files define a Rust-owned view-only capability grant with explicit household, parent/support actor, child device, route, pairing, grant, session, device-trust, audit, and diagnostic-redaction fields. Tests cover serialization/versioning plus wrong household, actor, device, route, pairing, trust, revoked/removed, missing audit, raw diagnostics, and deferred-control rejection. | **Complete for Phase 1** | No missing contract behavior or named negative-test family was found. Runtime pairing, relay, custody, and rendered disclosure belong to later workpacks; focused execution remains Phase 2. |
| WP02 Live Screen Relay | Nine implementation and seven test files provide typed remote session authorization/effect decisions, child-runtime gating, live-view readiness/worker decisions, agent-service startup wiring, relay/LAN mode selection, bounded duration, replay rejection, no-raw-cache/no-recording/no-input guards, and prerequisite tests. | **Incomplete** | `start_screen_live_view_worker` only returns a `worker_started` record; no capture-to-transport/relay worker, protected-surface handling, degraded/reconnect state machine, child disclosure delivery, parent rendered state, or executable deletion/custody boundary exists. Relay readiness is supplied by environment proof booleans, not verified runtime behavior. |
| WP03 Remote Input Control Authority | The workpack is explicitly deferred and expects no current-pass code. One implementation file and two test files nevertheless expose `InputAllowed` and produce `RemoteAccessInputBridgeState::Start` when the generic session gates pass; a test asserts that start path. | **Incomplete** | Current code contradicts the live-view-only no-code/no-claim boundary. It does not implement a real input bridge, but it models control as startable without the workpack's fresh confirmation, scoped input, blocked-surface, stop/escape, policy, platform-permission, replay-input, or privilege-escalation authority. This legacy surface must be removed or fail-closed until WP03 is explicitly opened. |
| WP04 Session Pairing Grants | Twenty-four implementation and seven test files implement a detailed in-memory/serializable live-view grant lifecycle with parent confirmation, disclosure, paired/active/paused/stopped/reconnect/terminal states, current-authority and device-trust checks, support visibility, revocation/removal precedence, supersession, bounded replay identity, restart-recovery evidence, durable event-shaped audit milestones, and extensive negative/round-trip tests. | **Incomplete** | Production search finds grant construction and transitions only in this crate's tests. There is no persistence adapter/store, composition-root loading, live session/relay consumer, device-trust authority port, child/portal disclosure delivery, or durable audit-journal owner. JSON round trips and replay history are contract code, not a persistence-backed runtime. |
| WP05 Relay Security Abuse Controls | Three implementation and four test files contain relay available/unavailable, request replay, relay-mode/cache prerequisite, transport selection, and unsafe-retention/control blocking states. | **Incomplete** | No authenticated capability-scoped relay token, expiry/replay store, rate limiting, backpressure, per-household/device connection limits, cross-household isolation, origin/host/redirect defense, stale-grant cache control, partial-outage/slow-dependency/reconnect-storm handling, redacted diagnostics, metrics, or abuse alert implementation/test matrix exists. Current relay claims are prerequisite booleans only. |
| WP06 Rollout Proof And Route Gate | Expected topology is `no-code-required`; this packet aggregates the five current-pass workpacks and explicitly keeps WP03 deferred. | **Complete for Phase 1** | No product implementation is authorized by this proof-only packet. Its accepted/missing proof roots, route synchronization, manual gaps, and no-overclaim result remain Phase 3 and cannot close while WP02, WP04, and WP05 are incomplete. |

**Remote Access Phase 1 result:** all 6/6 workpacks now have reviewed
code/test ownership. WP01 and the proof-only WP06 have no Phase 1 writing gap.
WP02-WP05 remain incomplete: the repository has strong capability and grant
contracts, but no real relay/live-view data path, the deferred input surface is
incorrectly startable in legacy effect-plan code, pairing grants are not owned
by a persistence-backed production runtime, and relay abuse controls are
largely absent. No Phase 2 passing-test/Enforcer or Phase 3 proof/PR_READY claim
is inferred.

### Tracking Phase 1 code/test audit - 2026-08-15

The code-first audit maps all 43 imported Tracking packets to precise current
Rust, service, policy, child-runtime, notification, AI, and portal roots. The
full row-by-row result is in
`docs/plans/tracking-plan/CODE_AUDIT.md`; stale references to the absent
`packages/tracking-domain` package and absent `scripts/test/tracking-*.mjs`
verifiers were removed from the plan routing documents.

**Tracking Phase 1 result:** 24 bounded packets have their core production code
and expected test code written; 19 are incomplete. Typed contracts, validation,
device/capability state, geofence and expected-place decisions,
acknowledgement/check-in, policy compilation, alert/notification intents, a
SQLite ActivityStore read model, and portal presentation are real. The live
product chain is not complete: `TrackingRuntimeEventFlow` uses a process-local
event bus, there is no durable cascade-to-journal-to-SQLite replay path, no
production Android/iOS/desktop sensor adapters, no concrete places or AI
provider route, no durable place store, and no durable notification/escalation
outbox with provider receipts. WP40 now owns the missing trusted service ingress
and durable journal composition; it must land before WP37 replay/projection. No
Phase 2 passing-test/Enforcer or Phase 3
proof/PR_READY claim is inferred.

## Consolidated branch code/test inventory - 2026-08-09

This is the recorded **source and test-topology** pass from 2026-08-09 on the
then-consolidated E: integration worktree. Verify the current checkout with
`git rev-parse HEAD`; the executable graph snapshot below is the current
authority. It counted
the plan-owned Rust/worker/UI source and crate-visible test files, then traced
the active policy request path. Counts mean files exist; they do **not** mean
the feature is accepted, a test was run today, or a workpack is complete.

## Historical executable graph snapshot - 2026-08-16

This retained checkpoint is historical and must not be used as current status.
The repo-owned graph remains the mechanical status source; run
`npm run graph:report` or use the current reviewed matrix above. At this older
checkpoint, `graph.json` imported 23
plan directories and 681 workpack rows (the older 526-row figure above is the
matrix's narrower scheduled-row view). This snapshot was refreshed from
consolidated head `360441362` on 2026-08-16. Its recorded derived workpack state
was:

| Planned | Blocked | Ready | Active | Validation | Done |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 367 | 21 | 3 | 2 | 287 | 1 |

### Graph-derived plan/workpack matrix

The rows below are the retained historical graph projection for every plan
directory. The
`P/B/R/A/V/D` column is the workpack count in `planned/blocked/ready/active/
validation/done` order. Code/test counts are reviewed-root topology for that
plan and may overlap because shared crates are intentionally visible in more
than one plan; they are not completion percentages.

| Plan | Workpacks | Planned / Blocked / Ready / Active / Validation / Done | Implementation files | Test files |
| --- | ---: | ---: | ---: | ---: |
| Account identity/family | 8 | 1/0/1/0/6/0 | 184 | 82 |
| AI | 48 | 46/0/0/0/2/0 | 118 | 39 |
| App/game | 220 | 126/2/0/1/91/0 | 605 | 61 |
| App | 95 | 13/0/0/0/82/0 | 184 | 67 |
| Browser | 30 | 30/0/0/0/0/0 | 72 | 13 |
| Child-agent runtime distribution | 11 | 8/0/0/0/3/0 | 104 | 10 |
| Cloudflare control plane | 13 | 10/2/0/1/0/0 | 186 | 63 |
| Data custody/storage | 9 | 1/1/3/0/4/0 | 971 | 482 |
| Device trust bootstrap | 9 | 0/2/1/0/6/0 | 553 | 143 |
| Eventing | 13 | 0/2/1/0/9/1 | 801 | 490 |
| LAN | 26 | 0/0/1/0/25/0 | 308 | 60 |
| Logging domain parity | 10 | 5/0/0/0/5/0 | 127 | 49 |
| Network | 8 | 6/1/0/0/1/0 | 372 | 97 |
| Parent desktop/runtime package | 11 | 4/0/0/0/7/0 | 526 | 182 |
| Payment/subscription | 13 | 4/5/0/4/0/0 | 44 | 39 |
| Policy control plane | 8 | 1/4/0/0/3/0 | 957 | 480 |
| Portal UX/household surfaces | 20 | 15/0/0/0/5/0 | 1020 | 530 |
| Remote access | 6 | 4/0/0/0/2/0 | 35 | 19 |
| Screen AI pipeline | 10 | 9/1/0/0/0/0 | 124 | 33 |
| Screen | 43 | 25/0/0/0/18/0 | 95 | 26 |
| Setup/install/provisioning | 7 | 0/1/0/0/6/0 | 577 | 197 |
| Tracking | 43 | 41/2/0/0/0/0 | 94 | 65 |
| V0.8 enforcement | 20 | 13/1/0/0/6/0 | 922 | 496 |

The graph validates at 705 nodes and 765 edges, with 34 migration/dependency
review items. The live map covers all 681 workpacks; Tracking WP40 now carries
the reviewed trusted-ingress/journal-composition ownership map. App/Game
contributes 220 reviewed rows, while Tracking contributes all 43 reviewed rows.
Graph states remain separate from code-first classification. Historical
source/test rows are now classified as `validation` instead of being counted
as unreviewed planned work. Use `npm run graph:status`, `graph:ready`, `graph:blocked`,
`graph:inspect <id>`, and `graph:why <id>` instead of inferring readiness from
checklist prose. A graph `DONE` state requires the referenced implementation,
test, proof, checklist, and any detected ADR paths to exist; it does not claim
that CI or a product path has been merged.

At that checkpoint there were **23 plan folders** and **680 imported workpack
rows** (not 24 plans). The then-reviewed-root topology was 2,902
implementation files and 1,144 test files. Those are ownership/topology counts,
not proof that the files are production-reachable or that tests pass. The
current consolidated head has not run final product CI and has not merged to
`develop` or `main`; older CI/PR records below apply only to their historical
heads.

### Historical 2026-08-10 topology narrative

The detailed table below is retained as dated routing context. The 2026-08-16
production-reachability dashboard and executable graph counts above supersede
its completion wording and source/test totals.

| Plan | Live code/test topology observed | Code-first state and next executable dependency |
| --- | --- | --- |
| Account identity/family | `family-identity-core` 43/23, `provisioning-core` 9/7, `storage-custody-core` 78/16 source/test files. | Core authority/custody exists; Cloudflare binding/migration and real provider/runtime routes remain separate work. |
| AI | `child-ai-core` 3/2 and `screen-ai-core` 20/3, joined through the shared agent/eventing stack. | Foundation only; next is one typed AI-result-to-policy consumer with negative safety coverage. |
| App/game | 220/220 reviewed maps; 169 code+test packets, 19 no-code packets, 32 writing gaps. | WP59 scheduler, WP60 metadata-history, and WP61 persisted provider-preflight bridges are code/test complete and focused-green; close WP62-WP65 preference/status, Android/Linux runtime tests, and dashboard/security gaps before whole-plan Phase 2. |
| App | `app-core`, `app-game-core`, `agent-protocol`, `agent-core`, `agent-service`, schema/generated contracts, parent runtime, and portal now have 95 exact reviewed workpack maps. | Eighty-one bounded workpacks have their Phase 1 source and expected tests written. Fourteen remain incomplete: WP15-WP20, WP26, WP48-WP49, WP62-WP65, and WP102. See `docs/plans/app-plan/CODE_AUDIT.md`; Phase 2 tests, Enforcer, proof, and release acceptance remain separate. |
| Browser | `browser-core` 33/20 plus service policy/adapter surfaces. | Integration source exists; managed command, adapter result, rollback, and parent-visible state are not a closed product path. |
| Child runtime distribution | `child-runtime` 31/9 plus service/protocol sources. | Runtime source exists; Windows service lifecycle/package smoke remains the next physical blocker. |
| Cloudflare control plane | `infra/cloudflare` 20/29 worker source/test files. | Local worker/contract source exists; deployment, account binding/migration, and payment runtime are not proven. |
| Data custody/storage | `storage-custody-core` 78/16, `ocentra-evidence` 1/4, eventing 80/34. | Core custody shapes exist; select retention/delete/export through service and journal. |
| Device trust bootstrap | `family-identity-core` 43/23 and `storage-custody-core` 78/16. | Windows custody/step-up slices exist, but durable trusted-device lifecycle, recovery, and cross-platform proof remain open. |
| Eventing | `ocentra-eventing` 80/34 plus protocol/service consumers. | Journal/replay and WP11 sequence repairs are freshly tested (24/24 journal/replay, 2/2 enforcement-eventing, 43/43 enforcement-runtime filtered); WP10 LAN consumer/product proof remains open. |
| LAN | `lan-core` 241/42 and `parent-runtime-core` 102/18. | Substantial pairing/runtime source exists; a real paired-device lifecycle through service and portal is still required. |
| Logging parity | `logging-core` 19/7 plus agent-service/portal integrations. | Foundation exists; enforce correlated logging on one product path rather than counting instrumentation alone. |
| Network | `network-core` 19/6 and `ocentra-network-evidence` 237/60. | Typed eventing foundation exists; complete parser-to-policy/service runtime before platform claims. |
| Parent desktop/runtime distribution | 497/181 across the reviewed parent runtime, schema, portal, Tauri, Android/iOS, and package-helper roots. | Package and smoke mechanics exist, but hosted authority, typed service truth, signing/store, updater/rollback, setup handoff, and the aggregate release gate remain open. |
| Payment/subscription | `billing-core` 17/4 and `entitlement-core` 8/5. | Core source exists; Cloudflare/account/trust dependency chain blocks a real checkout-to-entitlement path. |
| Policy control plane | `policy-control-core` 126/34, child policy/runtime and service seams. | Parent-resolution contract/service/UI and replay/audit persistence now exist; approved-request → compiled-artifact → child/device/domain binding is now code-backed and focused-tested. Trusted adapter execution and product delivery remain open and unclaimed. |
| Portal UX | `apps/portal` 104/50 and `portal-domain` 112/14. | Real presentation/test topology exists; it needs service-backed actions, not more presentation-only completion claims. |
| Remote access | `remote-access-core` 2/5 with adjacent screen/LAN surfaces plus the Rust-owned schema capability-fabric contract. | WP01 view-only capability authorization contract and focused schema tests now exist; pairing/relay/session runtime, device-trust integration, revoke/remove flow, and proof remain open. |
| Screen AI pipeline | `screen-ai-core` 20/3 and capture/agent/eventing seams. | Foundation; next is a redacted selected-window capture to typed AI-result chain. |
| Screen | `screen-core` 3/3, live view 3/4, capture adapter 7/3. | Foundation; prove custody/delete on one supported capture platform. |
| Setup/install/provisioning | `parent-runtime-core` WP07 source boundary plus `provisioning-core` 9/7 and `child-runtime` 31/9. | Rust Start status is reachable and fail-closed, but authenticated account/package/child/device-trust/LAN/custody/policy/network/recovery composition is absent; tests and proof remain deferred. |
| Tracking | `tracking-core` 39/41 plus agent/eventing/network evidence owners and the Rust-owned WP34 event catalog. | WP34 event identity/required-field validation and focused schema tests now exist; runtime publishers, journal/read-model consumers, provider path, and proof remain open. |
| V0.8 enforcement | `child-enforcement-core` 8/3 plus policy/service/eventing owners. | Journal sequencing, parent-resolution, and request/artifact/target binding slices are code/test verified; trusted adapter execution, child delivery, rollback, and product-visible receipts remain open/unclaimed. |

## Fresh code/test verification on 2026-08-10

This is the current implementation evidence for the consolidated branch, not a
completion certificate. Local code/test/pre-commit gates pass; retained proof
artifacts and product-workpack acceptance remain separate gates. The
consolidated train's required CI and normal merge are recorded above.

| Slice | Code and focused validation observed | Honest state |
| --- | --- | --- |
| Policy WP05 / Enforcement WP10 parent resolution and delivery binding | Protocol 2/2; agent-service parent-resolution 2/2 plus the persistence-failure unit test 1/1 and confirmation target 3/3; parent-runtime UI 1/1; child-policy delivery binding 11/11; child-runtime delivery facade 5/5. Successful parent resolution now fails closed when durable activity-audit persistence cannot open or ingest, with `ACTIVITY_STORE_UNAVAILABLE` returned to the caller. Rust format check, scoped architecture gates, generated-artifact check, and Enforcer guard passed. | Request/artifact/target identity binding and audit-persistence failure propagation are code/test verified; consolidated CI and main merge are complete. Trusted adapter execution, product notification, retained proof, and workpack closure remain open. |
| Eventing WP06 Journal Replay And Lineage | `ocentra-eventing` journal/replay 24/24; topology/lineage and version-skew targets pass; the typed enforcement-audit handoff is retained in four durable proof records. The graph now maps 21 implementation and 13 test files and marks WP06 `done` only after reviewed code/test/proof/checklist evidence. | Generic journal/replay/handoff workpack is code/test/proof complete in the consolidated branch. CI/main merge and the downstream LAN consumer remain separate; Enforcement WP11 still owns its enforcement-specific journal contract. |
| App Plan WP01 Contract Boundary And Effect Schemas | Schema-domain build/type-check and contract tests 11/11; focused Rust app tests pass; scoped architecture and Enforcer guard pass; durable proof retained in `docs/proof/app-plan/slice-01-app-runtime-decision.md`; implementation merge PR #640 `ddec6d6c7d1a15e1a3d56562a3ab22ed2e990a3a`, fresh CI/main-state merge PR #643 `47a2ac717` recorded. | Focused contract sub-slice is code/test/proof/CI/main-merge verified; the workpack remains validation until its broader checklist and product paths are complete. |
| Enforcement WP11 journal consumer and completed-retry slice | Service enforcement-eventing 2/2; enforcement-runtime focused target 43/43 passed (7 filtered); completed-command retry recovery 7/7 and exact ActivityStore replacement/missing-row regressions 2/2 passed. The retry path returns the verbatim persisted report with a real completion timestamp and rejects partial, missing, mismatched, or corrupted custody before adapter execution. Rust format, focused compile, scoped architecture, nine routed Enforcer checks, diff check, and hub guard passed. | Code/test slices verified; the workpack remains open until its enforcement-specific durable query/audit proof and approval/denial/expiry/override transition-family coverage are retained. No trusted-adapter or WP04 dispatch authority is claimed. |
| Eventing WP04 queue/idempotency/dead-letter rollback | `cargo test -p ocentra-eventing --tests` passed 31 contract, 2 integration, 24 journal-replay, 57 unit, and 2 version-skew tests, including initial queued-publish rollback and drop-oldest overflow restoration after journal failure. `cargo fmt --all -- --check`, scoped architecture, `hub:guard`, and `git diff --check` passed. | Code/test repair is locally verified and merged with the consolidated CI train; the eventing workpack and plan remain open until retained proof and acceptance agree. |
| Enforcement WP04 owned-process terminate/time-limit adapter | Windows owned-process lookup now reports `NoOp` with `PROCESS_ALREADY_EXITED` when the target is already gone; the time-limit boundary still maps that terminal process result to `Expired`. Agent-core enforcement target: 53 passed; agent-service enforcement target: 40 passed; Rust format, targeted clippy, scoped architecture, `hub:guard`, and `git diff --check` passed. | Code/test correction is locally verified on the consolidated branch. WP04 remains open (0/5 checklist rows closed): trusted persisted dispatch authority, durable WP11 journal handoff, rollback, retained proof, and CI/main merge are still required. |
| Policy WP04 receipt-aware transition precondition | `policy-control-core` unit target: 46/46 focused tests passed; `child-policy-core` delivery-handoff replay target: 12/12 passed. The trusted-receipt transition seam persists a matching adapter receipt and keeps the ordinary receiptless path fail-closed; scoped architecture, Rust format, and Enforcer commit gates passed. | Code/test slice is `validation`, not WP04 closure; consolidated CI and main merge are complete. The parent-runtime-to-agent-service trusted dispatch ledger, Eventing WP06 -> WP11 durable-journal handoff, OS adapter side effect, rollback execution, retained WP04 proof bundle, and workpack acceptance remain open. |
| Remote WP01 capability fabric | `crates/schema` Rust-owned view-only capability/grant/session contract; 4 focused contract tests passed, including exact schema-version, authenticated-parent, requested-child-device, and nonblank-audit checks. Rust format, architecture, generated-artifact, and Enforcer gates passed. Durable manifest: `docs/proof/remote-access-plan/slice-01-capability-fabric.md`. | Contract slice is `validation`; consolidated CI and main merge are complete. Pairing/relay/device-trust/session runtime, revoke/remove flow, custody, abuse proof, and workpack acceptance remain open; remote input/control remains deferred. |
| Device-trust WP08 dependency adoption | Dependency matrix covers WebAuthn, passkey, keyring, encrypted-bundle, and RustDesk decisions; contract test 1/1 passed; scoped architecture and Enforcer guard passed. Durable manifest: `docs/proof/device-trust-bootstrap-plan/slice-08-dependency-adoption.md`. | Review slice is `validation`; consolidated CI and main merge are complete. Runtime adoption, platform ceremony, key sealing, recovery execution, retained proof, and workpack acceptance remain open. |
| Network WP08 control-catalog routing | Focused route-boundary contract test 1/1 passed; exact capability/schema/settings source docs and no-default-read/no-runtime-claim wording are asserted. Scoped architecture and Enforcer guard passed. Durable manifest: `docs/proof/network-plan/slice-08-control-catalog-routing.md`. | Reference-routing slice is `validation`; consolidated CI and main merge are complete. Network runtime/parser/classification/policy/enforcement/platform work and workpack acceptance remain open. |
| Tracking WP34 event contracts | `crates/schema` Rust-owned 19-event catalog with required causation/evidence/policy/live-mode TTL/audit/reason/transition and typed AI payload safety validation; 4 focused contract tests passed as part of the 107-test schema contract target. | Contract slice verified. Runtime event publishers, journal/replay projection, portal read model, and retained proof/audit remain open. |
| App WP01 runtime decision boundary | `ocentra-app-core` contract 4/4 and invariant 2/2 passed; schema-domain build/type-check and `app-runtime-decision.test.ts` passed 11/11; scoped architecture and Enforcer guards passed. | Contract/runtime-decision code/test slice and local proof are current; consolidated CI and main merge are complete. Installed inventory, broader app runtime/service/portal path, retained proof, and workpack acceptance remain open. |
| Other 18 plan folders | This pass rechecked topology and selected dependency rows only; no fresh focused implementation validation was run for those plans. | Not closure. Their existing matrix/workpack rows remain routing context until each code/test slice is audited. |

### Current dependency decision

The artifact/target identity contract is present at the child delivery boundary,
enforced by the child-runtime facade, and now carried through the
parent-resolution transport/service binding with focused mismatch coverage.
The policy owner now exposes a receipt-aware transition seam, but that seam only
accepts adapter evidence; it does not mint authority or perform an OS action.
The next implementation is the enforcement-owned trusted adapter and durable
WP11 journal route. Until that route supplies non-forgeable execution authority,
delivery remains manual-required rather than active. Actor/device/route/version,
expiry, duplicate/replay, audit, and notification-no-claim negatives remain
required for that next slice. This is still the shared unblocker for policy,
enforcement, portal, app/game, browser, network, and screen paths.

## Current merged-code train - 2026-08-06

Live baseline: `origin/main` at `0923976516098d36962566c2eea3933e6d878b00`.
This refresh inspected merged source, retained proof paths, and required CI for
the six PRs below. It records delivered slices only; no row upgrades a plan or
product path to complete.

| Plan / workpack | Merged source, tests, and proof observed on the baseline | Fresh required CI | Honest current state / next unblocker |
| --- | --- | --- | --- |
| Eventing WP06 journal/replay/lineage | [#621](https://github.com/ocentra/OcentraParent/pull/621) merged as `96735c5715cca0a59858febcfe43a2370707d43c`. `agent-protocol`, `agent-service`, and `ocentra-eventing` carry the typed enforcement audit-journal handoff and retry/idempotency coverage; retained evidence is `docs/proof/eventing-plan/wp06-00-enforcement-wp11-handoff.md` and `wp06-16-validation-commands.md`. | [run 31064093081](https://github.com/ocentra/OcentraParent/actions/runs/31064093081): required format/types/Rust, full validation, and package-preview gates passed. | WP06's merged handoff is evidenced. The plan stays open on WP10 LAN consumer proof and its consumer-plan handoff. |
| Account identity/family WP08 authority parity | [#622](https://github.com/ocentra/OcentraParent/pull/622) merged as `36969f92a6a41573284d1ae93e422640bd3d873e`. `family-identity-core` contains the account/household authority handoff and contract/negative tests; durable parity, negative, redaction, Cloudflare-handoff, and validation records are tracked under `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/`. | [run 31067638811](https://github.com/ocentra/OcentraParent/actions/runs/31067638811): required format/types/Rust, full validation, and package-preview gates passed. | The Rust authority/generated-edge slice is merged and proven. Cloudflare WP06/WP08 runtime/migration work and Account WP06 aggregation remain open. |
| Account identity/family WP01 D1 storage adapter | [#632](https://github.com/ocentra/OcentraParent/pull/632) merged as `0923976516098d36962566c2eea3933e6d878b00`. `infra/cloudflare` now contains the optional, fail-closed `ACCOUNT_IDENTITY_D1` provider-subject-to-Ocentra-account store, binding declarations, and D1-shaped unit/env coverage. Retained evidence is `docs/proof/account-identity-family-plan/01-auth-provider-decision/06-account-identity-storage-adapter-proof.md`. | [#632 required CI](https://github.com/ocentra/OcentraParent/pull/632/checks): required format/types/Rust, full validation, and package-preview gates passed before merge. | This is a narrow mapping boundary only: it does not prove external token verification, login/session routes, household/device authority, D1 deployment or migration, Worker production readiness, WP01 closure, or plan closure. Next is the separately owned Cloudflare WP06 binding/migration and WP08 runner proof, then Account WP06 aggregation. |
| Policy/enforcement authenticated delivery-grant boundary | [#633](https://github.com/ocentra/OcentraParent/pull/633) merged as `86e0b5d8ea071fe71970b9e231cd4bf92ca257e0`. `agent-protocol`, `policy-control-core`, and `agent-core` now issue, validate, and consume the signed delivery-grant prerequisite with contract/unit coverage. | [run 31096089598](https://github.com/ocentra/OcentraParent/actions/runs/31096089598): all required format/types/Rust, full validation, platform, and package-preview gates passed. | Merged prerequisite only. It does not establish trusted adapter execution, rollback, receipt visibility, broad enforcement completion, or any workpack/checklist closure; those remain owned by the policy and enforcement runtime workpacks. |
| Device trust WP02 Windows local custody | [#623](https://github.com/ocentra/OcentraParent/pull/623) merged as `46bb53da4d0dfbdd8d1b40937abfd67262aac8c3`. `storage-custody-core` has `windows_device_trust_custody` plus Windows/unsupported-platform tests, with the authority path in `family-identity-core` and `parent-runtime-core`. | [run 31057272950](https://github.com/ocentra/OcentraParent/actions/runs/31057272950): required format/types/Rust, full validation, and package-preview gates passed. | Narrow Windows-only custody slice is merged. Cross-platform custody, recovery/reset/re-pair, QR approval, and child tamper/uninstall proof remain open. |
| Device trust WP03 parent step-up | [#627](https://github.com/ocentra/OcentraParent/pull/627) merged as `1ce56056c8c233addafe89feec7008c2bdda7059`. `family-identity-core::parent_step_up_authority` and `schema::parent_step_up_receipt` provide the fail-closed authority/receipt boundary with unit and contract coverage. | [run 31061365961](https://github.com/ocentra/OcentraParent/actions/runs/31061365961): required format/types/Rust, full validation, and package-preview gates passed. | The record-backed parent-step-up boundary is merged; it is not phone-QR, recovery-bundle, entitlement, or child-uninstall runtime closure. |
| Network WP01 typed-eventing handoff | [#617](https://github.com/ocentra/OcentraParent/pull/617) remains the merged bounded `NetworkFlowObservedEvent` to reusable `DomainEvent`/`EventEnvelope` contract handoff. Its retained contract proof is `docs/proof/network-plan/01-network-foundation-eventing-contract.md`. | Historical merged evidence; no fresh network-runtime CI assertion is made by this docs refresh. | WP01 and all seven sibling network workpacks remain open; next is the remaining WP01 contract obligations, then parser-to-policy runtime proof. |

## Merged-baseline evidence delta — 2026-08-05

This narrow refresh inspected `origin/main` at `580fb9ec` plus the live PR and
CI state named below. It updates only the affected routing decisions; it does
not turn a merged CI/tooling repair or an open slice into workpack closure.

| Scope | Current evidence | Status / no-claim boundary | Next unblocker |
| --- | --- | --- | --- |
| CI setup reliability | PR [#619](https://github.com/ocentra/OcentraParent/pull/619) merged as `580fb9ec`; its fresh CI Gate passed. `.github/workflows/dependency-policy.yml` now explicitly skips unused Linux Tauri packages while retaining Rust setup, and `tests/repo-tooling/workflow-ci-trigger.test.mjs` covers that call. | Merged CI reliability/tooling repair only; it is not policy, device-trust, or product-workpack proof. | Treat the repaired Dependency Policy runner as normal CI infrastructure; validate each product slice on its own evidence. |
| Policy WP02 parent authoring/preview | PR [#615](https://github.com/ocentra/OcentraParent/pull/615), head `cc01f9ae`, is an open, reviewed partial slice with CI still active. It projects Rust-owned conflict/manual-required/unsupported attention cards and adds scoped portal/integration tests plus `02-conflict-visible-proof.md`. | WP02 remains open: templates/manual-rule authoring, preview-to-save confirmation, opaque confirmed-request relay, delivery/enforcement, child-device application, and full proof fields are outside #615. | Finish the remaining WP02 authoring and confirmation path with targeted portal/accessibility/mobile proof, then reconcile the workpack from merged evidence. |
| Device Trust WP02 local key sealing | PR [#616](https://github.com/ocentra/OcentraParent/pull/616) is open and unmerged. Current `main` has neither `crates/storage-custody-core/src/windows_dpapi_key_sealing.rs` nor `crates/family-identity-core/src/trust_bootstrap/current_authority.rs`; the required `output/device-trust-bootstrap-plan-proof/02-local-key-sealing/` root is absent. Review findings also reject the proposed lifecycle/custody model as an authority closure. | Blocked. There is no merged production Windows custody coordinator, sealed-key persistence/current-authority owner, or accepted runtime proof; model and narrow parent-presence code are not WP02 closure. | Assign a single production custody owner that durably coordinates Windows sealing with non-restorable current authority, household/device binding, revocation/reset, and Windows proof before reopening merge readiness. |

## Code-backed refresh — 2026-07-31

This refresh is a scheduling override for the older rows below. It was produced
from plan state/workpack indexes plus current source and test surfaces. A
checked plan row is **not** reproducible closure when its named `output/*proof*`
root is absent from the checkout. Use the four states below when assigning work:

- **Source + focused tests**: implementation and relevant test source exist;
  the test was not re-run by this refresh.
- **Locally proven**: a current focused validation/proof artifact was observed.
- **Proof absent**: implementation may exist, but the plan's required proof
  output is unavailable here and must be recreated before closure.
- **Blocked**: one exact missing runtime contract, provider, platform, or
  external prerequisite prevents the next validation.

| Plan | 2026-07-31 code-backed state | Next owning workpack / dependency |
| --- | --- | --- |
| account identity/family | Source + focused tests; named proof roots absent. | Account WP08 Rust-owned schema/authority, then Cloudflare WP06 D1/DO/KV binding/migration and Cloudflare WP08 runner/proof, after the provider/custody decision. |
| AI | Precursor source only; 47 of 48 workpacks remain open. | WP03 Rust contract, then WP07 queue and WP08 routing. |
| app | Source + focused tests; all app workpacks remain open/unknown. | WP06–09 Windows capture/foreground path, then WP12–15 service/journal. |
| app/game | Source + focused tests; customer control path unproven. | WP19 policy compiler, then WP20 budget evaluator. |
| browser | Source + focused tests; plan still has all primary workpacks open. | WP01 Rust-owner contract reconciliation before launcher/custody work. |
| child runtime distribution | Source present; package/service proof roots absent. | WP02 Windows service lifecycle, then recreate package/install proof. |
| Cloudflare control plane | #608 merged to `main` as `5af4a1a92` after fresh full CI; the worker and generated billing contracts are present, and the local WP07 dev/seed/proof boundary is validated. PR #604 is closed without merge: its branch/evidence are preserved, but its overlapping implementation is superseded/conflicting. | Keep WP07 local-only/open. Its successor is proof-only and must start from current source after the real Cloudflare dependency-resolution gap is fixed; then rerun selected gates and retain a tracked bundle. |
| data custody/storage | Source + focused tests; rollout proof absent. | WP07 regenerate current proof manifest and verify runtime gaps. |
| device trust bootstrap | Source + focused tests; platform sealing/product chain absent. | WP02 local key sealing, then step-up/recovery/tamper workpacks. |
| eventing | WP06 Journal Replay And Lineage is graph-complete: reviewed code/test ownership and the durable generic-to-enforcement handoff are present. WP10 LAN household mesh proof and downstream enforcement work remain open. | WP10 LAN household mesh consumer and route proof; separately, Enforcement WP11 consumes the WP06 handoff. |
| LAN | Strong source/test foundation; physical product proof absent. | WP23 two-device proof, then WP25 backend-to-Tauri-to-portal chain. |
| logging parity | Source + focused tests; proof restoration and routing lint remain. | WP03 Rust service-to-logging mapping, then routing-lint repair. |
| network | Source + focused tests; all workpacks open. | WP01 foundation contracts/eventing. |
| parent runtime distribution | Source present; signing/install proof absent. | WP10 setup handoff contracts, then signing/store gates. |
| payment/subscription | Source + focused tests; Cloudflare dependency blocks runtime. | Cloudflare billing boundary, then WP05 tax/refund/dispute policy. |
| policy control plane | Strong focused contracts; adapter execution remains an external enforcement dependency, not a WP04 contract task. | WP02 authoring/preview and WP05 ask-parent/override work; track trusted-adapter execution with the enforcement owner. |
| portal UX | Real shell/UI; most product flows open. | WP05 policy authoring after owning policy read model is ready. |
| remote access | Scaffold/adjacent route only. | WP01 capability fabric, then WP04 pairing/revocation. |
| screen AI pipeline | Source pieces only; no pipeline proof root. | WP01 prerequisite gate then WP02 trigger-to-capture proof. |
| screen | Capture/AI/live-view source exists; custody/platform closure open. | WP03 contract, then WP06–08 capture model/adapter. |
| setup/install/provisioning | Source + focused tests; depends on identity/device/install truth. | Parent setup-to-child trust/install handoff. |
| tracking | Strong source/test foundation; event spine and provider proof open. | WP34 tracking event contracts, then WP35–39 chains. |
| v0.8 enforcement | Source + focused tests; Eventing WP06 generic journal/replay handoff is now graph-complete, while enforcement-specific journal, adapters, and parent-visible receipts remain open. | WP11 durable enforcement-journal handoff -> WP04 trusted dispatch -> adapter/receipt/rollback. WP11 and WP04 remain unscheduled/manual-required until their own evidence exists. |

### Priority order

1. Cloudflare module dependency reconciliation; separately, Account WP08 Rust contract -> Cloudflare WP06 D1/DO/KV binding/migration -> Cloudflare WP08 runner/proof -> Account WP06 aggregation (unblocks accountable Cloudflare verification and payment routing).
2. Eventing WP06 Journal Replay And Lineage: retain the actual journal/replay proof and typed enforcement handoff before WP10 or enforcement WP11/WP04 scheduling.
3. Network WP01 eventing foundation.
4. Tracking WP34 event contracts.
5. Screen-AI WP01 → WP02 trigger/capture chain.
6. AI WP03 → WP07 → WP08; Remote WP01 → WP04.

Proof-root restoration is a verification/reproducibility packet. It must not be
used to claim a product path complete without the selected runtime proof.

## Live integration refresh — 2026-07-28

This is a merge-queue snapshot, separate from the 2026-07-17 whole-plan code
audit below. It records work that has current source, tests, proof, and a
pushed review branch; it is **not** a completion claim until the PR has green
CI, an acceptable review, is merged, and the named checklist row is reconciled.

| Plan / workpack | Current implementation evidence | Validation / integration state | Next action |
| --- | --- | --- | --- |
| Enforcement / WP05 app-game session handoff | Service validates persisted runtime/session evidence, rejects unknown identity, and asserts stored timer-binding fields. Protocol, core, and service focused tests are present. | PR [#584](https://github.com/ocentra/OcentraParent/pull/584), head `098b18acd`; all required jobs except the rerun service Clippy chain are green. The rerun is actively executing, not failed. | Let Clippy complete; then run service-test/aggregate checks, review, reconcile WP05 rows, and merge only if all are green. |
| App / WP01 runtime-decision contract boundary | Rust-owned app runtime contract change and focused branch are pushed. | PR [#577](https://github.com/ocentra/OcentraParent/pull/577) awaits CI/review. | Audit exact checklist/proof rows before treating it as closed. |
| Device trust / WP01 durable registry | Durable family/device trust registry branch is pushed. | PR [#576](https://github.com/ocentra/OcentraParent/pull/576) awaits CI/review. | Validate platform-sealing and parent-presence boundary before merge. |
| Network / WP01 foundation contract/eventing | Runtime contract/eventing repair is pushed. | PR [#573](https://github.com/ocentra/OcentraParent/pull/573) is behind `main`. | Rebase, rerun focused gates/CI, then review. |
| Cloudflare / WP12 handoff | Current generated billing-contract source, local seed/runtime proof, and focused worker tests are in the branch. | PR [#568](https://github.com/ocentra/OcentraParent/pull/568) is behind `main`. | Rebase, rerun worker/billing gates, reconcile stale plan wording. |
| Logging / WP04 parity, WP04/WP05/WP10 proof restoration | Three proof-restoration branches and one parity branch are pushed. | PRs [#572](https://github.com/ocentra/OcentraParent/pull/572), [#580](https://github.com/ocentra/OcentraParent/pull/580), [#581](https://github.com/ocentra/OcentraParent/pull/581), [#582](https://github.com/ocentra/OcentraParent/pull/582); #572 is behind `main`, the others await CI/review. | Regenerate each proof at its exact head; docs-only repair is not plan closure. |
| Portal / LAN result flow and E2E cleanup | Selected LAN-target persistence and E2E process cleanup are pushed. | PRs [#578](https://github.com/ocentra/OcentraParent/pull/578) and [#579](https://github.com/ocentra/OcentraParent/pull/579) await CI/review. | Run real portal/service click-through after CI; map accepted work to LAN/portal rows. |
| Shared tooling / schema build order | Test-build sequencing repair is pushed. | PR [#575](https://github.com/ocentra/OcentraParent/pull/575) is behind `main`. | Rebase and validate before wider test claims. |
| Local proof environment | Local proof isolation from LAN environment is pushed. | PR [#583](https://github.com/ocentra/OcentraParent/pull/583) awaits CI/review. | Treat as test infrastructure, not product-workpack closure. |

Only #584 is actively executing. The queue is parked for integration triage:
rebase-required first, then CI/review, then checklist and post-merge proof
reconciliation. No workpack here is done solely because a branch or PR exists.

## Status vocabulary

- **Foundation**: meaningful source and focused tests exist, but the product
  path is not yet proven end to end.
- **Integration**: multiple runtime layers exist; remaining work is joining,
  proving, or operating the path.
- **Blocked**: code or an external/runtime prerequisite prevents the next
  required proof.
- **Scaffold**: models/contracts or a thin implementation exist, but no
  credible product-runtime closure exists yet.

`Implemented` is not `done`. A workpack is only done after its selected tests,
proof artifact, checklist row, and merge state agree.

## Plan execution matrix

| Plan | Code state | Main runtime owners | Evidence observed | Current blocker / dependency | Next unblocker |
| --- | --- | --- | --- | --- | --- |
| `account-identity-family-plan` | Accepted multi-workpack source / Phase 1 incomplete | `family-identity-core`, `schema`, `schema-domain`, `infra/cloudflare`, provisioning and billing consumers | All eight workpacks are mapped at 207 implementation / 82 test files. Accepted source through `e69acf279` adds strict schema validation, a sealed current Account capability, durable repository/CAS/invariants, lifecycle records, D1 adapter and ordered `0001`-`0004` migrations, plus Account-bound billing/support consumers. | No real provider-to-authority producer or account/session route is composed. Invite/recovery orchestration, Device Trust/remote/export/delete handoffs, expected tests, migration execution, proof, and deployment remain open. | Finish the remaining source composition seams first; then write the whole WP02-WP05/WP08 expected-test wave, run focused tests/Enforcer, and only afterward regenerate Cloudflare WP08 and Account WP06 proof. |
| `ai-plan` | Fully audited / Phase 1 incomplete | `agent-protocol`, `agent-core`, `agent-service`, `child-ai-core`, `screen-ai-core`, browser/app-game/tracking evidence owners, portal-domain, portal | All 48 workpacks now have reviewed topology: 118 implementation and 39 test files in the narrowed plan roots. Eleven workpacks have bounded Phase 1 code/expected tests written. Real foundations include configured local `llama.cpp` execution, singleton scheduling, household LAN claim/lease/idempotency, Activity memory graph, tracking AI validation, Parent Assistant boundaries, and portal AI runtime cards. | Thirty-seven workpacks retain code/test gaps. Highest-impact missing systems are the canonical general AI contract family, durable general work lifecycle, SQLite-backed context/result journal, semantic memory, verified model artifact installer, owned OCR/VLM workers, unified explanation surface, trusted remote authorization/adapter, and complete security/performance tests. | Finish WP03/WP04, WP07, WP09/WP12, and WP14-WP19 in dependency order before memory/model/screen/feature closeout; run Phase 2 and Phase 3 only after the corresponding Phase 1 rows are complete. |
| `app-plan` | Fully audited / Phase 1 incomplete | `app-core`, `app-game-core`, `agent-protocol`, `agent-core`, `agent-service`, schema/generated contracts, `parent-runtime-core`, portal-domain, portal | All 95 workpacks have reviewed ownership. Eighty-one bounded workpacks have source and expected tests written across Windows inventory/process/foreground capture, journal/SQLite/sessionization, freshness/read models, policy/timer models, runtime capture, enforcement, notification outbox/scheduling/metadata history/provider preflight, and parent-facing projections. | Fourteen workpacks remain incomplete: portal inventory/session rendering; durable unknown-app review and risk lifecycles; a real policy compiler/routing owner; integrated child warning/budget flows; notification preference/status; performance harnesses; and the missing WP102 parent-domain followthrough step. | Implement WP18/WP49 first, then WP16/WP17, WP19/WP20, WP62-WP65, UI/performance gaps, and WP102. Run Phase 2 only after each writing gap closes; proof is Phase 3. |
| `app-game-plan` | Accepted WP10 source / Phase 1 incomplete | `agent-protocol`, `agent-core`, `agent-service`, `app-game-core`, schema, portal | WP10 now has independently accepted production source for one shared Windows process snapshot, generation-safe process/launcher/foreground identity, conservative launcher candidates, targeted-PID refresh, and one canonical app/game session join while preserving generic window identity. The remaining mapped inventory, journal, policy, timer, notification, and portal foundations stay present. | WP10's expected-test family has not yet been refreshed or run; no proof or checklist closure is inferred. The broader plan still lacks the UI, preference/status, Android/Linux, load, and stale WP102 source/test packets listed in its code audit. | Continue the production-source wave for the remaining app/game gaps; in the later test wave write all WP10 generation/launcher/foreground negative cases before focused execution. |
| `browser-plan` | Fully audited / Phase 1 incomplete | `agent-protocol`, `agent-core`, `agent-service`, `browser-core`, schema-domain, portal-domain, portal, Android owned shell | All 30 imported packets have reviewed ownership. Fourteen executable workpacks have bounded code/expected tests written; contracts, Windows inventory, managed launch, custody/CDP mapping, durable ingest/read models, policy manifest/compiler, and unmanaged detection are real. | Ten workpacks still lack cross-platform inventory, corrupt/concurrent profile-store negatives, active-focus evidence, complete portal status tests, trusted managed/unmanaged action execution, live AppLocker control, extension integration, health/load harnesses, or aggregate E2E. | Complete WP05/WP06 and WP11 first, then WP17/WP19; finish WP14 and WP20-WP23 before Phase 2 focused execution and Phase 3 proof. |
| `child-agent-runtime-distribution-plan` | Fully audited / Phase 1 incomplete | `child-runtime`, `child-runtime-android-bridge`, Rust schema/child-enforcement contracts, Android/iOS child apps, desktop package/service scripts, CI/release workflows | All 11 workpacks have reviewed code/test ownership. The real child-service binary now owns durable pre-readiness recovery and fail-closed dispatch gates; Android packaging builds the JNI library and the Activity/foreground-service composition starts and stops the Rust service. Existing Windows lifecycle, iOS capability, platform-matrix, and package-builder source remains real. | The Android bridge does not start a supervised command consumer or expose live event submission, uses no verified trust identity, has no shipped ongoing health consumer, and lacks APK/device ABI proof. Real-host lifecycle, executable respawn, parent-authorized uninstall, consumed setup/trust handoff, and the aggregate multi-platform release gate also remain open. | Wire supervised Android ingress/stop and trusted identity first, then add bound health consumption and APK/device lifecycle proof; continue with host lifecycle, respawn/uninstall, setup handoff, and the aggregate release gate. |
| `cloudflare-control-plane-plan` | Fully audited / Phase 1 incomplete | `infra/cloudflare`, account/billing contracts, portal consumer boundary | All 13 workpacks have reviewed code/test ownership. WP05 has real Firebase subject verification, and the accepted Account/Payment source now adds WP06's current-authority D1 adapter plus ordered Account/Payment migration source and authority-bound billing/support consumers. | No shipped provider-to-authority/account-session route composition exists; the migration has not been executed. Durable DO state, expected tests, persisted local seeding, portal smoke, deployment/rollback automation, and proof remain open. | Finish provider/account route composition and remaining runtime source, then write WP05/WP06 expected tests before broad Cloudflare validation or proof regeneration; follow with focused module gates, Enforcer, retained proof, and payment handoff. |
| `data-custody-storage-plan` | Accepted multi-workpack source / Phase 1 incomplete | `schema`, `storage-custody-core`, `child-runtime`, `ocentra-evidence`, `ocentra-eventing` | WP01's Rust-owned matrix, WP02 scope authority, WP03 manifest custody, WP04 child-owned durable tombstone/effect lifecycle, WP05 import integrity, WP06 request/row authority, and WP07 internal service command/recovery source are present. | The Account WP08 → WP04 → WP05 trusted authority chain and private child-runtime adapter are absent; WP05 backup/migration, WP06 TypeScript edge, and WP08 confirmation-authority source remain open; expected tests are incomplete and moved-store tests are stale; no current proof/DONE. | Finish the remaining source owners first, then write the complete expected-test delta, run focused crate/domain gates, repo-wide Enforcer, proof, precommit, and one PR/CI cycle. |
| `device-trust-bootstrap-plan` | Accepted multi-workpack source / Phase 1 incomplete | `family-identity-core`, `storage-custody-core`, `parent-runtime-core`, `entitlement-core`, `child-runtime`, Android child bridge, `child-enforcement-core`, `schema` | Accepted source through `68717b5b7` preserves owner-resolved current device/signer authority, unsigned entitlement projection, fail-closed restore authorization, and durable trust-bound child removal/readiness. Public/caller-minted household and restore authority paths were removed. | Real platform/passkey enrollment, entitlement issuer/revocation, encrypted restore executor/key custody, parent transport/platform removal, expected tests, and shipped composition remain open. No proof or DONE follows from source acceptance. | Finish the repository source wave, then write the complete WP01/WP05/WP06/WP07 expected-test delta before focused crate/domain execution and Enforcer. |
| `eventing-plan` | Integration | `ocentra-eventing`, `agent-protocol`, `agent-core`, `agent-service`, portal host bridge | WP06 journal/replay, topology, version-skew, and typed handoff surfaces are retained. WP09 Phase 1 production code and expected tests are written. WP11 production source is independently accepted through `fa1230661`, including fail-closed envelope/request/journal identity and single-use replay authority. | WP11 remains READY because its existing replay tests still target the retired API and the full negative/audit test family, focused execution, and proof are open. WP09 whole-plan integration/CI/merge remains open; WP10 is blocked on LAN WP26; WP12 lacks its harness/root and WP13 lacks current validation/proof. | After the repo source wave, migrate and complete WP11 expected tests, then run focused gates. Continue LAN WP26 -> Eventing WP10 and finish WP13/WP12 in dependency order. |
| `lan-plan` | Fully audited / Phase 1 incomplete | `lan-core`, `agent-protocol`, `agent-service`, `parent-runtime-core`, parent desktop, portal-domain, portal | All 25 pre-existing workpacks have reviewed ownership, and new WP26 has an explicit routed owner boundary. Twenty-two rows have bounded code/expected tests written across discovery, identity, merge/classification, persistence, pairing, routing, and portal surfaces. | WP26 lacks the real signed-child ingress, durable receipt/idempotency custody, trust/route composition, and private Eventing WP10 handoff. WP16 lacks one integrated backend-to-Tauri-AppHandle-to-portal-listener regression. Six aggregate verifier commands are absent, and WP25 depends on those validation gaps; physical/manual artifacts remain Phase 3. | Implement graph-authorized WP26 without fake transport or portal authority, then add the WP16 integrated delivery regression and restore or replace the six executable verifiers so WP25 can enter focused validation before physical proof. |
| `logging-domain-parity` | Accepted source hardening / Phase 1 incomplete | `logging-core`, `logging-domain`, `agent-service`, portal, dev-log/Vite route | Canonical redaction ownership, JSON-safe fail-closed serialization, canonical dev-writer routing, realpath/symlink containment, and path-redacted query diagnostics are integrated. | WP02/WP03/WP07/WP08 expected-test families are deferred; WP05/WP06/WP10 and broader adoption/enforcement remain open. No focused run, proof, or acceptance is inferred. | Finish the complete logging expected-test wave, then run focused package/query/route gates and only later proof correlation. |
| `network-plan` | Fully audited / Phase 1 incomplete | `network-core`, `ocentra-network-evidence`, `agent-protocol`, `agent-core`, `agent-service`, portal | All eight workpacks have reviewed code/test ownership. Real observations, runtime-delivery state, service read models, and the portal drawer remain; the fabricated product-path caller, payload, bridge, and pipeline are gone. WP04 is now graph-blocked because no durable cascade/composition owner exists. | Six workpacks remain incomplete. Production captures only Windows connection metadata; parser/classifier/AI/policy/notification/adapter/custody owners are not composed into a live path, fake-path tests still require delete/rewrite, canonical contract truth is duplicated, and performance/security rollout lacks executable load/abuse/rollback harnesses. | Keep WP04 blocked behind direct Eventing WP09, AI WP07/WP19, Policy WP05/WP08, Custody WP04/WP06, and Portal WP09/WP12 handoffs; then rewrite/delete invalidated tests, fix WP01 durable canonical contracts, and compose WP02/WP03 through shipped owners before any whole-plan Phase 2 or proof claim. |
| `parent-client-runtime-distribution-plan` | Accepted WP03/WP06 source / Phase 1 incomplete | Tauri parent desktop, hosted portal, `parent-runtime-core`, agent service/protocol, dev bridge, generated portal bridge, Android/iOS parent projects | WP03/WP06 now have independently accepted reachable source for typed health, all-command response identity/freshness validation, hard monotonic WebSocket deadlines, serialized Tauri/dev-web polling, and fail-closed dependency/LAN route state. Existing parent package and smoke foundations remain. | The refreshed WP03/WP06 expected-test families have not been written or run, and their old proof cannot re-close the new source. Hosted account authority, signing/store, updater/rollback runtime, setup handoff, iOS/device/store proof, and aggregate release work remain open. | Continue the remaining parent-client source gaps; later write the complete WP03/WP06 negative/deadline/dependency test wave before focused execution or proof. |
| `payment-subscription-plan` | Production source wave accepted / Phase 1 tests open | `billing-core`, `entitlement-core`, Rust billing schemas, Cloudflare Worker/DO, D1 provider mapping, portal consumer boundary | All 13 workpacks have reviewed ownership. The accepted source wave adds Account-composite provider identity, durable mutation leases/retry/outbox/CAS, stale cursor rejection, a fail-closed forward migration, removes caller-minted signed entitlement authority, and is reconciled with the ordered Account `0002` -> Payment `0003`/`0004` source chain. | Real provider and entitlement issuer/verifier authority, migration execution, regional/referral/parent/admin surfaces, stale signed-API tests, and the rollout verifier remain open. | Finish the remaining Account/provider composition, then write the complete Payment expected-test delta; only afterward run focused crate/domain validation. Proof and PR stay last. |
| `policy-control-plane-plan` | Integration / production incomplete | `policy-control-core`, `agent-service`, `schema`, eventing | Rust source, validation, compiler, preview, delivery, conflict, and audit contracts exist. | There is no identity-backed durable policy-source owner or shipped source-to-compiler-to-Screen/AI delivery caller; confirmation writes audit evidence, not active source truth. Policy-to-enforcement execution/rollback authority is also incomplete. | Build the authoritative policy-source persistence boundary, then a shipped compiler service, then consume only its real artifact in Screen-AI and durable delivery. |
| `portal-ux-household-surfaces-plan` | Fully audited / Phase 1 incomplete | Portal, `portal-domain`, `parent-runtime-core`, service read models, Android/iOS parent shells, package/CI helpers | All 20 workpacks have reviewed code/test ownership. Nine bounded workpacks have their core code/expected tests written; real shell, device targeting, browser/app/network state, honest degraded handling, no-fake-data contracts, screenshot harnesses, and mobile shells exist. | Eleven workpacks still lack household authority/first-run state, authoring/request actions, LAN/assistant command consumption, diagnostics redaction/history, cohesive report/notification custody, or plan-wide accessibility tests. | Complete WP01/WP02 first, then WP05-WP07 and WP10/WP11; finish WP08/WP12/WP14/WP15 before Phase 2 focused tests and Phase 3 proof. |
| `remote-access-plan` | Scaffold | `remote-access-core`, `screen-live-view-core`, LAN, portal | Remote core has 2 source / 5 test files; adjacent live-view pieces exist. | Session grants, relay, revocation, and safety proof are not implemented as a product path. | Build view-only session grant/revoke state before any control feature. |
| `screen-ai-pipeline-plan` | Fully audited / Phase 1 incomplete | `screen-ai-core`, capture adapter, `agent-protocol`, `agent-core`, `agent-service`, portal | All 10 workpacks have reviewed code/test ownership. Real capture, encrypted queueing, local adapter execution, deletion, read models, and portal rendering exist; only WP01 has no bounded Phase 1 writing gap. | Nine workpacks remain incomplete: trigger ownership and parent settings are disconnected, canonical AI routing is not production-wired, policy/action authority is fabricated or absent, the normal event chain is not durably replayable, custody negatives are missing, and live/performance/final harnesses are unwritten. | Wire WP02 parent settings and real trigger owners first, then WP03 canonical AI routing and WP04 trusted policy handoff; complete action, journal, custody, performance, and operator/final gates in dependency order before Phase 2. |
| `screen-plan` | Fully audited / Phase 1 incomplete | `screen-capture-adapter`, `agent-protocol`, `agent-core`, `agent-service`, `screen-ai-core`, `screen-live-view-core`, schema, Android agent, portal | All 43 imported packets have reviewed ownership: 40 executable workpacks plus three reference-only packets. Real parent settings, desktop/Linux capture, Android MediaProjection code, encrypted queueing, redaction, deletion, read models, portal UI, route guards, live-view gates, and local-AI scheduling exist. | Thirty-one executable workpacks retain code/test gaps. The sharpest are stale source/snapshot routing, missing iOS/Android/macOS/Linux platform tests, no protected-surface detector, no durable full-chain replay, fabricated policy refs, gate-only live view, no child disclosure, and missing CDP/OCR/VLM/detector/rollout harnesses. | Reconcile WP01/WP02 first, then close contracts/scope/platform safety (WP03/WP05-WP14) and queue/runtime composition (WP16-WP21) before policy/live/AI/rollout packets. Do not regenerate proof until those Phase 1 gaps close. |
| `setup-install-provisioning-plan` | Accepted fail-closed source / Phase 1 incomplete | `provisioning-core`, `child-runtime`, `parent-runtime-core`, parent desktop, generated bridge, `portal-domain`, portal | All 7 workpacks have reviewed ownership. WP05's readiness model is real; WP07 now has a reachable Rust-owned 15-row `not-run`/`manual-required` matrix and preserves LAN as observation-only without dispatching it as Start authority. | Authenticated owner inputs for all readiness authorities, the live state machine/actions/completion guard, and current Rust/portal/E2E expected tests remain open. Historical proof is stale. | Bind real typed owner inputs first, finish the whole WP07 state-machine source packet, then write all WP07 expected tests before any focused execution or proof. |
| `tracking-plan` | Fully audited / Phase 1 incomplete | `tracking-core`, `schema`, `agent-protocol`, `child-runtime`, `parent-runtime-core`, `child-policy-core`, `child-notification-core`, `child-ai-core`, `policy-control-core`, `agent-core`, `agent-service`, portal-domain, portal | All 43 packets have reviewed code/test ownership. Twenty-four bounded packets have core code and expected tests; WP40 is mapped to the child-runtime/journal composition but remains dependency-blocked. | Nineteen packets remain incomplete. The process-local event cascade is not durably journaled/replayed/projected; production platform sensor adapters, concrete places/AI providers, durable local places, notification/escalation delivery state, and an end-to-end restart-safe portal chain are absent. | Complete WP32/WP34/WP36 before WP40, then WP37 replay/projection, WP38/WP27 delivery and escalation, WP22/WP07 persistence/custody, platform adapters, providers, and final composition/UI packets before Phase 2 or proof. |
| `v0-8-enforcement-control-plan` | Foundation / fully audited, Phase 1 incomplete | `schema-domain`, `schema`, `agent-protocol`, `child-enforcement-core`, `policy-control-core`, `agent-core`, `agent-service`, browser/network owners, portal | All 20 workpacks have reviewed code/test maps. Typed contracts, evidence-bound dispatch, honest capability matrices, unmanaged fallback, report-only network state, and durable timer recovery are written. | Thirteen workpacks still lack trusted delivery/execution authority, complete audit transition families, app/game and managed-browser action receipts, aggregate service state, child/portal surfaces, live integrity/uninstall observation, platform-role separation, or whole-plan verifier/UI tests. | Complete WP11 -> WP04 -> WP05/WP06, then WP10/WP12/WP13, portal/integrity/platform surfaces, and finally WP18/WP19/WP20. Run focused Phase 2 validation only after each Phase 1 slice is written. |

## Workpack execution audit

This table is the current scheduling baseline derived from every routed
`WORKPACK_INDEX.md`, not a completion certificate. `Doc-claimed closed` means
the plan index currently marks the row checked/done. `Freshly reverified` stays
zero until the current branch regenerates the named proof and passes the
focused acceptance gate. Gitignored or absent historical `output/` and
`test-results/` paths cannot be used as retained current proof.

| Plan | Execution rows | Doc-claimed closed | Open / partial / blocked / unknown | Freshly reverified | Scheduling note |
| --- | ---: | ---: | ---: | ---: | --- |
| `account-identity-family-plan` | 8 | 5 | 3 | 0 | Replacement Account source through `e69acf279` now includes sealed current authority, durable CAS repository, lifecycle records, D1 adapter/migrations, and Account-bound billing/support consumption. Expected tests, provider/account routes, remaining handoffs, deployment/proof, and Account WP06 aggregation remain open. |
| `ai-plan` | 48 | 2 | 46 | 0 | Fully mapped from live source/tests. Eleven workpacks are Phase 1 complete for bounded code/test scope; 37 retain concrete production-code or expected-test gaps. WP01/WP02 now close the source-reconciliation/snapshot documentation work; the remaining checkbox split still does not describe implementation maturity. |
| `app-game-plan` | 88 | 53 | 35 | 0 | WP10 is explicitly source-accepted but remains open for expected-test refresh, focused execution, proof, and checklist closeout; the other open rows follow the live code audit rather than historical checkboxes. |
| `app-plan` | 95 | 0 | 95 | 0 | Fully mapped from live code/tests. Eighty-one bounded workpacks have no Phase 1 source/expected-test writing gap; WP15-WP20, WP26, WP48-WP49, WP62-WP65, and WP102 remain incomplete. This is code/test maturity, not proof or product acceptance. |
| `browser-plan` | 24 | 0 | 24 | 0 | Fully mapped from live code/tests. Fourteen executable workpacks are Phase 1 complete for bounded scope; ten retain concrete product-code or expected-test gaps. Six additional imported packets are reference-only and excluded from the 24 execution rows. |
| `child-agent-runtime-distribution-plan` | 11 | 10 | 1 | 0 | Fully mapped from live code/tests. Five workpacks are Phase 1 complete for bounded scope; six retain concrete lifecycle/runtime/handoff/release-gate gaps despite ten index-level completion labels. |
| `cloudflare-control-plane-plan` | 13 | 0 | 13 | 0 | Fully mapped from live source/tests. WP05 now includes the reviewed Firebase verifier source but lacks expected tests and server-derived binding authority; WP06 has a bounded adapter/auth chain without test/proof closure, and the remaining runtime/deployment workpacks retain concrete Phase 1 gaps. |
| `data-custody-storage-plan` | 8 | 0 | 8 | 0 | Current graph/topology keeps every executable workpack open. Source has advanced, but tests, retained proof, checklist reconciliation, the Account WP08 → WP04 → WP05 composition chain, WP05 backup/migration behavior, the WP06 TypeScript edge, and WP08 confirmation authority remain incomplete. |
| `device-trust-bootstrap-plan` | 9 | 0 | 9 | 0 | Live source now includes accepted WP01/WP05/WP06/WP07 packets through `68717b5b7`, but the plan has zero graph-DONE workpacks. WP08 has bounded research/review only; expected tests, external authority/composition, focused validation, and proof remain open. |
| `eventing-plan` | 6 | 1 | 5 | 0 | Six selectable workpacks: WP06 is the one doc-closed row; WP09-WP13 are open. WP11 production source is accepted, while replay-test migration, negative/audit tests, focused execution, and proof remain open; WP10 is blocked on LAN WP26, WP12 lacks its harness/root, and WP13 lacks current validation/proof. |
| `lan-plan` | 26 | 13 | 13 | 0 | Twenty-six routed rows: 13 document-closed and 13 open. Four Phase 1 gaps remain explicit at WP16, WP20, WP25, and new WP26; WP26 is the signed-child beacon/household-mesh authority handoff required by Eventing WP10. |
| `logging-domain-parity` | 10 | 0 | 10 | 0 | Accepted source hardening landed for WP02/WP03/WP07/WP08; their complete edge/route/query/logger test families remain open alongside WP05/WP06/WP10. No row is freshly reverified. |
| `network-plan` | 8 | 0 | 8 | 0 | Fully mapped from live source/tests. WP05 and WP08 are Phase 1 complete for bounded gate/reference scope; WP01-WP04 and WP06-WP07 retain concrete canonical-contract, production-composition, live-runtime, or executable-harness gaps. No proof row is freshly reverified. |
| `parent-client-runtime-distribution-plan` | 11 | 5 | 6 | 0 | WP03 and WP06 are now active with accepted refreshed source but open expected tests/execution/proof; five historical bounded closures remain, and the remaining plan gaps stay routed through the live audit. |
| `payment-subscription-plan` | 13 | 0 | 13 | 0 | Route labels now match live source truth: accepted source is integrated, but no implementation workpack is claimed DONE while Account/provider/issuer dependencies and expected tests remain open. |
| `policy-control-plane-plan` | 8 | 6 | 2 | 0 | Six checked workpacks are not reflected by the generic checklist status. |
| `portal-ux-household-surfaces-plan` | 20 | 5 | 15 | 0 | Fully mapped from live code/tests. Nine workpacks have no Phase 1 writing gap in their bounded scope; eleven retain concrete product-code or expected-test gaps. The five doc-claimed closures are not used as implementation truth. |
| `remote-access-plan` | 6 | 0 | 6 | 0 | Five planned rows and one deferred control row. |
| `screen-ai-pipeline-plan` | 10 | 0 | 10 | 0 | Fully mapped from live code/tests. WP01 is complete for its no-code prerequisite scope; WP02-WP10 retain concrete production-composition, authority, durability, custody-negative, performance-test, or missing executable-harness gaps. Proof remains deferred until those Phase 1 gaps close. |
| `screen-plan` | 40 | 18 | 22 | 0 | Fully mapped from live code/tests: 9/40 executable workpacks are Phase 1 complete, 31 retain concrete code/test gaps, and three additional imported packets are reference-only. The 100/100 legacy checklist and 18 checked workpack labels overstate current runtime truth. |
| `setup-install-provisioning-plan` | 7 | 6 | 1 | 0 | Fully mapped from live code/tests. Six workpacks have no bounded Phase 1 writing gap; WP07 now has accepted reachable fail-closed source but remains incomplete without real authority inputs, state-machine/actions/completion guard, and refreshed expected tests. The historical 93/93 checklist is not product completion. |
| `tracking-plan` | 40 | 0 | 40 | 0 | All 43 imported packets are mapped; the 40 execution rows remain open. WP40 owns the trusted runtime ingress/journal composition, is blocked on WP32/WP34/WP36, and in turn blocks WP37. |
| `v0-8-enforcement-control-plan` | 20 | 6 | 14 | 0 | Fully mapped from live source/tests. WP01-WP03 and WP07-WP09 are Phase 1 complete for bounded executable scope, and WP20 is coordination-only; 13 workpacks retain trusted-dispatch, lifecycle, aggregate-state, child/portal surface, live-integrity, platform-role, or executable-harness gaps. Historical checked boxes overstate WP18 because its declared umbrella verifier is absent. |
| **Total** | **527** | **143** | **384** | **0** | Plus 145 reference/source-only rows and 8 historical rows excluded from execution scheduling. |

### Acceptance state for each workpack

A workpack advances independently through `implemented -> focused gate green ->
retained proof regenerated -> checklist reconciled -> accepted commit -> merged ->
post-merge reverified`. Only the final state counts toward `Freshly reverified`.

## Crate to plan matrix

| Runtime owner | Plans unblocked or directly served | Priority reason |
| --- | --- | --- |
| `schema` + `agent-protocol` | every plan | Stable shared contracts prevent duplicate ownership and unblock service integration. |
| `ocentra-eventing` | custody, policy, tracking, LAN, enforcement, portal | Replay/idempotency/journal semantics are the common durability spine. |
| `family-identity-core` + device trust | account, setup, payment, remote, enforcement | Parent authority and device presence are required before sensitive actions. |
| `policy-control-core` | app/game, browser, LAN, network, screen, enforcement, portal | Policy is the shared decision producer for product controls. |
| `child-enforcement-core` + adapters | policy, browser, app/game, network, screen, enforcement | Converts approved policy into a reversible platform action. |
| `agent-service` | nearly every product plan | It joins feature crates into commands, read models, proof logs, and HostBridge state. |
| `storage-custody-core` | custody, tracking, screen, AI, account, payment | Retention/delete/export safety is a cross-cutting closure requirement. |
| `lan-core` | LAN, setup, remote, portal, child runtime | Device pairing and household transport unblock multi-device product proof. |
| `infra/cloudflare` + generated billing contracts | payment, account, setup, portal | Current source exists; focused worker/billing gates and stale proof reconciliation decide the next cloud packet. |
| `apps/portal` | portal, setup, policy, tracking, LAN, remote, reports | Presentation closes only after service read models and typed actions are available. |

## Dependency and unblock order

1. **Complete the account-authority runtime chain**: Firebase subject-verification source now exists and fails closed, while Account WP08 and Cloudflare WP06 carry bounded authority/storage source. Server-derived family/device binding, migration, expected tests, retained proof, deployment, and runtime reachability remain open. Supply those seams before WP03's native parent authority, Cloudflare WP08 test proof, and Account WP06 aggregation.
2. **Build device trust runtime**: parent presence and sealed device trust unblock safe account, setup, payment, remote, and enforcement decisions.
3. **Keep the two WP11 owners distinct**: Eventing WP11's generic type-safety source repair is integrated but still needs expected tests and acceptance. Separately, enforcement-plan WP11 still owns enforcement-specific durable audit history before enforcement WP04 trusted dispatch -> adapter -> receipt -> rollback. Do not use the Eventing source fix as enforcement completion.
4. **Use LAN/service as the first physical household proof**: pairing -> device state -> portal read model establishes the multi-device integration baseline.
5. **Close custody and observability on that vertical slice**: correlated logs, retention/delete, and replay make later feature work trustworthy.
6. **Scale feature producers**: browser, app/game, network, screen, tracking, and AI can then feed the same decision and evidence spine.
7. **Finish portal and distribution proof**: run real click-through and package/smoke proof only after the underlying product paths exist.

## Checklist synchronization rule

Every plan checklist should retain unchecked rows unless the named workpack has
all of the following: implementation, focused tests, retained proof artifact,
and an accepted merge state. A checklist audit may record current code evidence
and a blocker, but must never turn a code inventory into a completion claim.
