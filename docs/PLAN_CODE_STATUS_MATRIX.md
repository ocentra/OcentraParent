# Project Progress Matrix

This is the code-backed execution dashboard for Ocentra Parent. It supplements
`PLAN_INDEX.md`; it does not replace plan-local workpacks, proof roots, or
checklists.

Last broad source inventory: 2026-08-15, on the merged `main` organization
baseline at `82123c16c`. The dated
merged-code delta immediately below overrides older snapshot wording for its
named plans; historical rows remain routing context, not current closure proof.

## Current merged repository and code/test audit baseline - 2026-08-15

Repository organization is complete for this audit baseline. Consolidation PR
`#646` merged normally to `main` as `608ef84fb` after CI run `31862487297`
completed with 61 successful jobs and all three required gates green. `develop`
was created at the same commit and has the same required checks, pull-request,
conversation-resolution, no-force-push, and no-delete protections as `main`.
There are no open pull requests. Remote branches are reduced to `main`,
`develop`, `production`, and the protected
`codex/archive/all-remote-tips-20260815`; the archive contains every deleted
remote tip. Locally, only `main` and `develop` remain, and the only registered
OcentraParent worktree is `E:/OcentraParent`.

The executable graph validates at 703 nodes and 705 edges. It imports **23**
actual plan rows and 679 workpacks. The earlier apparent count of 24 included
the Markdown table header in `PLAN_INDEX.md`; no plan directory is missing.
Current derived state is 453 planned, 9 blocked, 0 ready, 1 active, 215 in
validation, and 1 done.

This is the key code-first limitation: live plan roots contain 2,911
implementation files and 1,213 test files, but only **23 of 679 workpacks**
(3.39%) have reviewed, exact code/test ownership maps. All 13 Eventing
workpacks are now reviewed, while 656 workpacks across the repository remain
unmapped. An unmapped workpack is
therefore **unattributed**, not proven absent and not proven implemented. Do not
turn the graph state or a checklist mark into a code-completion percentage.
`npm run graph:matrix -- --json` is the complete 679-row table; the reviewed
coverage below states how much of that table currently has source/test evidence
strong enough for workpack-level decisions.

| Plan | Workpacks | P/B/R/A/V/D | Live implementation/test files | Reviewed workpack maps | Code-first audit state |
| --- | ---: | ---: | ---: | ---: | --- |
| Account identity/family | 8 | 1/0/0/0/7/0 | 159 / 82 | 0 / 8 | Unattributed; plan roots exist, but no exact workpack map is reviewed. |
| AI | 48 | 47/0/0/0/1/0 | 690 / 421 | 0 / 48 | Unattributed; foundation code exists, workpack ownership is unaudited. |
| App/game | 220 | 132/0/0/0/88/0 | 688 / 436 | 0 / 220 | Unattributed; the large validation set cannot be treated as implemented. |
| App | 95 | 94/0/0/0/1/0 | 670 / 421 | 1 / 95 | Partial; WP01 contract/runtime-decision roots are mapped. |
| Browser | 30 | 30/0/0/0/0/0 | 620 / 452 | 0 / 30 | Unattributed. |
| Child-agent runtime distribution | 11 | 0/1/0/0/10/0 | 33 / 10 | 0 / 11 | Unattributed; Windows service/package remains graph-blocked. |
| Cloudflare control plane | 13 | 13/0/0/0/0/0 | 183 / 63 | 0 / 13 | Unattributed; source presence is not deployment/runtime completion. |
| Data custody/storage | 9 | 1/0/0/1/7/0 | 653 / 410 | 8 / 9 | Partial; WP01-WP08 are mapped. WP04 is Phase 1 complete; WP01-WP03, WP05-WP06, and WP08 remain incomplete. |
| Device trust bootstrap | 9 | 1/2/0/0/6/0 | 307 / 104 | 1 / 9 | Partial; WP08 dependency review is mapped, runtime lifecycle remains open. |
| Eventing | 13 | 1/0/0/0/11/1 | 777 / 492 | 13 / 13 | Fully code-mapped; Phase 1 is complete for 3 workpacks and incomplete for 10. Only WP06 is graph-done. |
| LAN | 25 | 0/0/0/0/25/0 | 1,370 / 638 | 0 / 25 | Unattributed; validation labels do not prove paired-device completion. |
| Logging domain parity | 10 | 5/0/0/0/5/0 | 693 / 470 | 0 / 10 | Unattributed at workpack level despite current crate tests and CI. |
| Network | 8 | 7/0/0/0/1/0 | 942 / 520 | 1 / 8 | Partial; WP08 reference routing is mapped. |
| Parent desktop/runtime package | 11 | 4/0/0/0/7/0 | 107 / 21 | 0 / 11 | Unattributed. |
| Payment/subscription | 13 | 8/2/0/0/3/0 | 44 / 39 | 0 / 13 | Unattributed; two workpacks are dependency-blocked. |
| Policy control plane | 8 | 0/2/0/0/6/0 | 911 / 481 | 1 / 8 | Partial; WP04 delivery/receipt roots are mapped but still blocked. |
| Portal UX/household surfaces | 20 | 15/0/0/0/5/0 | 683 / 448 | 0 / 20 | Unattributed. |
| Remote access | 6 | 4/0/0/0/2/0 | 371 / 137 | 2 / 6 | Partial; WP01 capability and WP04 pairing-grant roots are mapped. |
| Screen AI pipeline | 10 | 10/0/0/0/0/0 | 517 / 361 | 0 / 10 | Unattributed. |
| Screen | 43 | 25/0/0/0/18/0 | 95 / 26 | 0 / 43 | Unattributed. |
| Setup/install/provisioning | 7 | 0/1/0/0/6/0 | 271 / 105 | 0 / 7 | Unattributed; rollout gate is dependency-blocked. |
| Tracking | 42 | 42/0/0/0/0/0 | 1,034 / 555 | 1 / 42 | Partial; WP34 event contracts are mapped. |
| V0.8 enforcement | 20 | 13/1/0/0/6/0 | 901 / 498 | 2 / 20 | Partial; WP04 and WP11 roots are mapped, with WP04 blocked on WP11. |

The next organization phase is not feature coding. Audit one plan at a time,
map each workpack to exact implementation and test roots, classify code/test
gaps, then rebuild the graph. Only after a plan's Phase 1 map is complete may
its focused tests and Enforcer checks be used for Phase 2 scheduling. Proof is
the later acceptance phase, not a substitute for missing code or tests.

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
| WP09 Network Consumer Event Chain | 3 implementation and 2 test files cover observation/runtime event contracts and serialized downstream references. | **Incomplete** | The full classification/AI/policy/enforcement/audit/portal chain is not publishable through reusable Eventing; weak-evidence and AI-cannot-enforce negatives are absent. |
| WP10 LAN Household Mesh Consumer | 2 implementation and 1 test file define transport/event types, fixtures, roundtrip, enum-wire, and a happy-path local-republish assertion. | **Incomplete** | There is no production import validator/republisher and no tests for unauthenticated/unauthorized, direct remote publish, replay/idempotency, stale, family/target mismatch, or provider policy/enforcement rejection. |
| WP11 Type Safety And Ownership Hardening | 4 implementation and 4 test files cover live/stored types, associated responses, Rust fixture parsing, and the naked-domain-string guard. | **Incomplete** | The shared fixture has a Rust consumer but no TypeScript consumer was found; explicit payload-mutation and no-lock-held-await audit tests are not retained as test code. |
| WP12 Rollout Proof And PR Gate | Expected topology is `tests-only`, but `scripts/test/eventing-rollout-proof.mjs` is missing. | **Incomplete** | All five named rollout reconciliation/negative checks lack their declared runner. Proof generation is Phase 3, but the expected test code itself is absent in Phase 1. |
| WP13 Test Folder Layout Regression Audit | Expected topology is `tests-only`; 38 external test files exist under `crates/ocentra-eventing/tests`, and no `src/` test module/entrypoint was found. | **Complete for Phase 1** | The focused Cargo/architecture rerun and fresh generated proof are Phase 2/3 work, not a missing-test-code gap. |

**Eventing Phase 1 result:** 13/13 workpacks inspected and mapped; 3/13
complete for code/test-writing scope, 10/13 still need code or expected tests.
The graph remains 1 planned, 11 validation, and 1 done because code topology
does not override checklist, proof, dependency, or acceptance state.

### Data Custody plan Phase 1 code/test audit - 2026-08-15

This table records current source and test code, not the workpack's checked
status or ignored proof roots. Tests are not claimed passing unless a current
run is named.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Custody Source Of Truth | Six Rust source/generator files, one generated TypeScript contract, and one Rust contract-test file implement the 28-class custody matrix and generated-edge drift check. | **Incomplete** | The claimed TypeScript boundary modules, TypeScript contract test, and proof runner do not exist on `main`. The two Rust tests cover serde and generated-file drift, but do not test the advertised class/hosting counts, unique row/class IDs, derived-source validity, redaction/notification rules, or forbidden-hosting invariants. |
| WP02 Encryption Key Custody | Five Rust implementation/contract files and two test files cover platform rows, household/device mismatch, revoked/lost keys, hosted portal, universal-key rejection, mobile proof, and recovery states. | **Incomplete** | The decrypt decision carries `requested_scope` but never checks it against the selected platform row's `decrypt_authority`; a child-service row can therefore authorize a parent-owned-bundle request when caller-supplied match flags are true. No cross-scope authority negative test exists. |
| WP03 Parent Owned Cloud Sync | Rust schema/generator and storage-custody derivation modules plus three Rust test files cover provider modes, ready/revoked refs, manifest integrity, conflict/retry state, tombstone separation, and no-default-Ocentra claims. | **Incomplete** | The claimed TypeScript contract/validation modules, TypeScript contract test, and manifest proof runner do not exist on `main`. Rust tests do not directly cover the declared wrong-account, folder-unavailable, partial-upload, provider-disconnect, and provider-delete visibility matrix. |
| WP04 Retention Delete Tombstone | Rust schema, state-machine, proof-builder, durable typed outbox, and three test files cover all nine required states, wrong-role/expiry negatives, ordering/duplicates, redaction-before-propagation, replay/restore blocking, minimal audit, hard delete, durable reopen/concurrency/migration, corrupt metadata, incoherent actions, and unknown acknowledgement. | **Complete for Phase 1** | No missing shared-boundary code/test-writing gap found. Concrete child-service startup composition remains a WP07 integration gap; provider/device propagation stays an adjacent-owner runtime boundary. |
| WP05 Export Import Backup Recovery | Rust schema, bundle builder, import preflight, restore/apply derivation, and two test files cover encrypted sections, integrity, redacted summaries, non-mutating/partial preview, wrong household/key/corrupt/expired/duplicate/migration negatives, confirmation, tombstone preservation, idempotency, and no support decrypt. | **Incomplete** | The in-scope backup cadence/manual-backup contract is absent. Supported migration is only a preflight label; no migration execution/rollback state machine or named migration-rollback test exists. Actual restore mutation remains outside this derivation-only surface. |
| WP06 Report Query Custody | Rust schema/generator, generated TypeScript contracts, storage-custody derivation/proof modules, and three Rust test files cover all seven required states, stable page ordering, duplicate cursors, disallowed source classes, tombstone-required deletion, conflict metadata, cursor expiry, rate limiting, and generated-contract drift. | **Incomplete** | Runtime derivation trusts request-owned authority and citation fields: it does not reject an unauthorized/non-parent request, raw-child-evidence request, wrong household/child ownership, or unauthorized citation reference. The workpack-named TypeScript adapter/rules modules and TypeScript negative contract test do not exist. |
| WP08 Parent Storage Settings Apply Flow | Rust schema/generator, generated TypeScript contracts, storage-custody card/preview/apply/action/proof modules, and two Rust test files cover explicit storage modes, visible manual-required state, restore preview, wrong-household and partial-restore negatives, separate disconnect/delete actions, delete-kind coverage, and generated-contract drift. | **Incomplete** | The apply input has no confirmation receipt or confirmed flag. Every preview sets `confirmation_required = true`, so runtime derivation rejects `Applied` and `Partial` unconditionally and cannot model a completed confirmed apply. The claimed TypeScript adapter/rules modules, TypeScript contract test, and focused proof runner do not exist. |

**Data Custody Phase 1 result so far:** 7/9 workpacks inspected and newly
mapped in this audit; WP04 is complete for code/test-writing scope, while
WP01-WP03, WP05-WP06, and WP08 still need code or expected tests. WP07 is next
for a current integration/rollout re-audit.

## Consolidated branch code/test inventory - 2026-08-09

This is the recorded **source and test-topology** pass from 2026-08-09 on the
then-consolidated E: integration worktree. Verify the current checkout with
`git rev-parse HEAD`; the executable graph snapshot below is the current
authority. It counted
the plan-owned Rust/worker/UI source and crate-visible test files, then traced
the active policy request path. Counts mean files exist; they do **not** mean
the feature is accepted, a test was run today, or a workpack is complete.

## Executable graph control-plane snapshot - 2026-08-10

The repo-owned graph is now the mechanical status source over these plan rows;
the matrix remains the human-readable code/test audit. `graph.json` imports 23
plan directories and 679 workpack rows (the older 526-row figure above is the
matrix's narrower scheduled-row view). Run `npm run graph:report` for the
joined state/topology view or `npm run graph:report -- --json` for machine
consumption. This snapshot was refreshed from the single E: checkout on
2026-08-10 after the WP07 code/test ownership map and WP04 review-repair slice
were reviewed. Current
derived workpack state is:

| Planned | Blocked | Ready | Active | Validation | Done |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 453 | 9 | 0 | 1 | 215 | 1 |

### Graph-derived plan/workpack matrix

The rows below are the current graph projection for every plan directory. The
`P/B/R/A/V/D` column is the workpack count in `planned/blocked/ready/active/
validation/done` order. Code/test counts are reviewed-root topology for that
plan and may overlap because shared crates are intentionally visible in more
than one plan; they are not completion percentages.

| Plan | Workpacks | P/B/R/A/V/D | Implementation files | Test files |
| --- | ---: | ---: | ---: | ---: |
| Account identity/family | 8 | 1/0/0/0/7/0 | 157 | 81 |
| AI | 48 | 47/0/0/0/1/0 | 669 | 419 |
| App/game | 220 | 132/0/0/0/88/0 | 667 | 434 |
| App | 95 | 94/0/0/0/1/0 | 649 | 419 |
| Browser | 30 | 30/0/0/0/0/0 | 610 | 452 |
| Child-agent runtime distribution | 11 | 0/1/0/0/10/0 | 31 | 9 |
| Cloudflare control plane | 13 | 13/0/0/0/0/0 | 183 | 63 |
| Data custody/storage | 9 | 1/0/0/1/7/0 | 636 | 406 |
| Device trust bootstrap | 9 | 1/2/0/0/6/0 | 294 | 101 |
| Eventing | 13 | 1/0/0/0/11/1 | 759 | 487 |
| LAN | 25 | 0/0/0/0/25/0 | 1,335 | 627 |
| Logging domain parity | 10 | 5/0/0/0/5/0 | 652 | 463 |
| Network | 8 | 7/0/0/0/1/0 | 931 | 519 |
| Parent desktop/runtime package | 11 | 4/0/0/0/7/0 | 107 | 21 |
| Payment/subscription | 13 | 8/2/0/0/3/0 | 44 | 39 |
| Policy control plane | 8 | 0/2/0/0/6/0 | 881 | 472 |
| Portal UX/household surfaces | 20 | 15/0/0/0/5/0 | 673 | 448 |
| Remote access | 6 | 4/0/0/0/2/0 | 368 | 134 |
| Screen AI pipeline | 10 | 10/0/0/0/0/0 | 507 | 361 |
| Screen | 43 | 25/0/0/0/18/0 | 95 | 26 |
| Setup/install/provisioning | 7 | 0/1/0/0/6/0 | 256 | 101 |
| Tracking | 42 | 42/0/0/0/0/0 | 1,006 | 549 |
| V0.8 enforcement | 20 | 13/1/0/0/6/0 | 864 | 485 |

The graph validates at 703 nodes and 705 edges, with 34 migration/dependency
review items. Twelve workpacks now have explicit reviewed code/test maps;
WP07 is mapped to its storage-custody and child-runtime lifecycle files while
remaining `active` until its aggregate proof contract is satisfied. WP04 is
mapped to its remote-access/schema lifecycle code and focused tests while
remaining `validation` until runtime integration and its expected proof root
exist. Historical
source/test rows are now classified as `validation` instead of being counted
as unreviewed planned work. Use `npm run graph:status`, `graph:ready`, `graph:blocked`,
`graph:inspect <id>`, and `graph:why <id>` instead of inferring readiness from
checklist prose. A graph `DONE` state requires the referenced implementation,
test, proof, checklist, and any detected ADR paths to exist; it does not claim
that CI or a product path has been merged.

There are **23 plan folders** and **526 scheduled workpack rows** in this
checkout (not 24 plans). The consolidated code train is merged to current
`main` and contains the LAN target repair, enforcement-journal sequence
repairs, parent policy-resolution/delivery binding, remote/tracking contracts,
and app runtime-decision contracts. The live reviewed-root topology is 2,831
implementation files and 1,182 test files. Scoped Enforcer, architecture,
generated-artifact, Rust, TypeScript, portal, and pre-commit validation pass.
Required CI run `31366692141` passed and PR #643 merged this train to `main` as
`47a2ac717`; proof custody and product-workpack acceptance remain separate
gates.

| Plan | Live code/test topology observed | Code-first state and next executable dependency |
| --- | --- | --- |
| Account identity/family | `family-identity-core` 43/23, `provisioning-core` 9/7, `storage-custody-core` 78/16 source/test files. | Core authority/custody exists; Cloudflare binding/migration and real provider/runtime routes remain separate work. |
| AI | `child-ai-core` 3/2 and `screen-ai-core` 20/3, joined through the shared agent/eventing stack. | Foundation only; next is one typed AI-result-to-policy consumer with negative safety coverage. |
| App/game | `app-game-core` 21/15 plus service adapters/read models. | Integration source exists; choose one live Windows capture-to-read-model path before treating any customer control path as ready. |
| App | `app-core` 3/5 plus shared agent-service integration and the versioned runtime-decision contract slice. | WP01 runtime-decision envelope/tuple sub-slice is code/test/proof/CI/main-merge verified; the broader workpack and app identity/evidence/session/service path remain open. |
| Browser | `browser-core` 33/20 plus service policy/adapter surfaces. | Integration source exists; managed command, adapter result, rollback, and parent-visible state are not a closed product path. |
| Child runtime distribution | `child-runtime` 31/9 plus service/protocol sources. | Runtime source exists; Windows service lifecycle/package smoke remains the next physical blocker. |
| Cloudflare control plane | `infra/cloudflare` 20/29 worker source/test files. | Local worker/contract source exists; deployment, account binding/migration, and payment runtime are not proven. |
| Data custody/storage | `storage-custody-core` 78/16, `ocentra-evidence` 1/4, eventing 80/34. | Core custody shapes exist; select retention/delete/export through service and journal. |
| Device trust bootstrap | `family-identity-core` 43/23 and `storage-custody-core` 78/16. | Windows custody/step-up slices exist, but durable trusted-device lifecycle, recovery, and cross-platform proof remain open. |
| Eventing | `ocentra-eventing` 80/34 plus protocol/service consumers. | Journal/replay and WP11 sequence repairs are freshly tested (24/24 journal/replay, 2/2 enforcement-eventing, 43/43 enforcement-runtime filtered); WP10 LAN consumer/product proof remains open. |
| LAN | `lan-core` 241/42 and `parent-runtime-core` 102/18. | Substantial pairing/runtime source exists; a real paired-device lifecycle through service and portal is still required. |
| Logging parity | `logging-core` 19/7 plus agent-service/portal integrations. | Foundation exists; enforce correlated logging on one product path rather than counting instrumentation alone. |
| Network | `network-core` 19/6 and `ocentra-network-evidence` 237/60. | Typed eventing foundation exists; complete parser-to-policy/service runtime before platform claims. |
| Parent desktop/runtime distribution | `parent-runtime-core` 102/18 plus parent shell/package surfaces. | Runtime surfaces exist; signed package launch/rollback smoke is not yet product closure. |
| Payment/subscription | `billing-core` 17/4 and `entitlement-core` 8/5. | Core source exists; Cloudflare/account/trust dependency chain blocks a real checkout-to-entitlement path. |
| Policy control plane | `policy-control-core` 126/34, child policy/runtime and service seams. | Parent-resolution contract/service/UI and replay/audit persistence now exist; approved-request → compiled-artifact → child/device/domain binding is now code-backed and focused-tested. Trusted adapter execution and product delivery remain open and unclaimed. |
| Portal UX | `apps/portal` 104/50 and `portal-domain` 112/14. | Real presentation/test topology exists; it needs service-backed actions, not more presentation-only completion claims. |
| Remote access | `remote-access-core` 2/5 with adjacent screen/LAN surfaces plus the Rust-owned schema capability-fabric contract. | WP01 view-only capability authorization contract and focused schema tests now exist; pairing/relay/session runtime, device-trust integration, revoke/remove flow, and proof remain open. |
| Screen AI pipeline | `screen-ai-core` 20/3 and capture/agent/eventing seams. | Foundation; next is a redacted selected-window capture to typed AI-result chain. |
| Screen | `screen-core` 3/3, live view 3/4, capture adapter 7/3. | Foundation; prove custody/delete on one supported capture platform. |
| Setup/install/provisioning | `provisioning-core` 9/7 and `child-runtime` 31/9. | Integration source exists; parent setup-to-child trust/install depends on device-trust lifecycle truth. |
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
| Enforcement WP11 journal consumer slice | Service enforcement-eventing 2/2; enforcement-runtime focused target 43/43 passed (7 filtered). Rust format check, scoped architecture gate, and Enforcer guard passed. | Code/test slice verified; the workpack remains open until its enforcement-specific durable query/audit proof and transition-family coverage are retained. |
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
| `account-identity-family-plan` | Foundation | `family-identity-core`, `provisioning-core`, `entitlement-core`, `schema`, narrow `infra/cloudflare` D1 adapter | Family identity has 13 source / 7 test files; setup and signed-entitlement paths exist. Merged [#632](https://github.com/ocentra/OcentraParent/pull/632) adds only a retained, optional/manual-required provider-subject mapping adapter with D1-shaped tests; the old #607 TypeScript test-double remains closed-unmerged and is not authority evidence. | Account WP08 Rust schema/account-authority proof is merged, but Cloudflare WP06 D1/DO/KV binding/migration proof, Cloudflare WP08 runner proof, external provider verification, and account runtime routes remain absent; Account WP06 is reopened to aggregate the required handoffs. | Keep #632's narrow custody boundary fail-closed; execute Cloudflare WP06 then WP08 from the Account WP08 handoff, prove deployment/migration and runtime routes, then rerun Account WP06 only after all required inputs are green. Any missing input remains a payment/policy/remote/device-trust scheduling block. |
| `ai-plan` | Foundation | `child-ai-core`, `screen-ai-core`, `agent-service`, `schema` | AI runtime and service seams exist. | Safety/output invariants and consumer proof remain fragmented. | Close one typed AI-result-to-policy handoff with safety and negative-path proof. |
| `app-plan` | Foundation | `app-core`, `agent-service`, `schema` | `app-core` has 3 source / 5 test files; service owns wider integration. | App-only authority and runtime evidence are incomplete. | Make app identity/evidence flow a single Rust-owned service path. |
| `app-game-plan` | Integration | `app-game-core`, `agent-service`, `schema` | 25 source / 20 test files; inventory, runtime, journal, and policy code exist. | Live platform metadata/crawling and portal product rows are incomplete. | Finish one live Windows app/game capture-to-read-model path. |
| `browser-plan` | Integration | `browser-core`, `agent-service`, `portal` | 43 source / 20 test files; managed-browser and intervention paths exist. | Managed/unmanaged execution and policy rollback are not closed. | Prove browser policy command through service, adapter, and visible portal state. |
| `child-agent-runtime-distribution-plan` | Integration | `child-runtime`, platform projects, release scripts | Child runtime, Android/iOS/Linux/macOS artifacts and proof surfaces exist. | Windows lifecycle/package proof is blocked; release proof is not whole-product readiness. | Resolve Windows service lifecycle and package smoke proof. |
| `cloudflare-control-plane-plan` | Integration / validation-open | `infra/cloudflare`, `billing-core`, account/billing contracts | Worker has 19 source / 28 test files, real route handlers, generated billing contracts, and the merged #608 local dev/seed/proof hardening. Fresh #608 full CI passed product, security, and platform jobs. PR #604 is closed without merge; its overlapping branch/evidence are preserved but superseded/conflicting. | #608 validates only a local WP07 boundary; no tracked workpack proof bundle exists, payment/runtime/deployment authority remains open, and WP01 dependency resolution still gates WP07. Account storage additionally needs Account WP08 contract -> Cloudflare WP06 binding/migration -> Cloudflare WP08 runner proof. | Retain the local-only no-claim boundary. After dependency resolution, run the proof-only WP07 successor from current source; separately execute Cloudflare WP06 then WP08 from the Account WP08 handoff and retain their exact proof or blockers before Account WP06 aggregation. |
| `data-custody-storage-plan` | Integration | `storage-custody-core`, `ocentra-evidence`, `ocentra-eventing` | Storage core has 63 source / 12 test files; custody/delete/export shapes exist. | Rollout/route-gate aggregation and cross-runtime custody proof remain open. | Prove one retention/delete/export flow through storage, eventing, and service. |
| `device-trust-bootstrap-plan` | Foundation / blocked | `schema`, `family-identity-core`, platform secure stores | Parent step-up validation, handoff schemas, trust helpers, and focused tests exist. PR #605 merged with fresh 60-job CI, but it is narrow unissued-parent-challenge test evidence only. | Concrete platform key-sealing adapters and the complete trusted-device product chain remain open; the plan stays partial/open. | Freeze the minimal parent-presence plus platform-sealed trust interface inside the owning core before shared-service integration. |
| `eventing-plan` | Integration | `ocentra-eventing`, `agent-protocol`, `agent-service` | 76 source / 34 test files; WP06 journal/replay, topology, version-skew, and typed handoff surfaces are retained and graph-mapped. | WP10 LAN consumer proof remains open; downstream Enforcement WP11 owns the enforcement-specific durable journal contract. | Select the WP10 consumer path and prove replay/idempotency end to end; then complete Enforcement WP11 before WP04. |
| `lan-plan` | Integration | `lan-core`, `agent-service`, `agent-core`, `schema` | 241 source / 91 test files; pairing, discovery, heartbeat, revocation, inventory, and read models exist. | Physical/consumer product proof and open follow-on workpacks remain. | Close a paired-device lifecycle through service and portal on a real platform. |
| `logging-domain-parity` | Foundation | `logging-core`, `logging-domain`, `agent-service`, portal | Logger, local evidence, MCP/query, and portal paths exist. | Broad adoption and several proof-root closeouts remain. | Make logging/proof correlation mandatory for one high-value product chain. |
| `network-plan` | Foundation | `network-core`, `ocentra-network-evidence`, `agent-protocol`, `agent-service` | Merged #617 added the bounded WP01 typed-eventing handoff: `NetworkFlowObservedEvent` maps directly to reusable `DomainEvent`/`EventEnvelope` under the distinct `network.flow.eventing.observed` contract. The retained [contract proof](proof/network-plan/01-network-foundation-eventing-contract.md) covers stored-envelope round trip, blank-device rejection, canonical schema enforcement, and collision-safe length-prefixed idempotency keys. | WP01 remains open: schema parity, evidence grade, policy action/handoff, no-private-bus audit, service-runtime, and platform proof are still incomplete; broader parser and policy bundles also remain open. | Complete the remaining WP01 contract obligations before escalating to parser-to-policy runtime proof. |
| `parent-client-runtime-distribution-plan` | Integration | Tauri parent desktop, Android/iOS parent projects, `parent-runtime-core` | Tauri shell and Android/iOS roots exist; focused package proof paths exist. | Whole release/signing/rollback readiness is unproven. | Produce one signed desktop package plus launch/rollback smoke. |
| `payment-subscription-plan` | Foundation / dependency-gated | `billing-core`, `entitlement-core`, Cloudflare worker | Billing core has 17 source / 4 test files; webhook and entitlement code exist. | Current Cloudflare/billing focused gates are not freshly green, and provider, account-authority, device-trust, and deployment proof remain open. | Refresh Cloudflare plus billing-core gates, then run one checkout/webhook-to-entitlement path without restoring obsolete TS contract ownership. |
| `policy-control-plane-plan` | Integration | `policy-control-core`, `agent-service`, `schema`, eventing | 126 source / 25 test files; compiler, preview, delivery, conflict, and authority code exist. | Policy-to-enforcement command/rollback product proof is incomplete. | Prove typed policy compile, delivery, execution receipt, and rollback. |
| `portal-ux-household-surfaces-plan` | Integration | `apps/portal`, `portal-domain`, HostBridge/service read models | Portal has 113 source / 87 test files and real route/panel code. | Several screens remain proof/presentation surfaces without completed backend actions. | Choose a service-backed household flow and prove the full click-through. |
| `remote-access-plan` | Scaffold | `remote-access-core`, `screen-live-view-core`, LAN, portal | Remote core has 2 source / 5 test files; adjacent live-view pieces exist. | Session grants, relay, revocation, and safety proof are not implemented as a product path. | Build view-only session grant/revoke state before any control feature. |
| `screen-ai-pipeline-plan` | Foundation | `screen-core`, `screen-ai-core`, capture adapter, `agent-service` | Capture/AI/service source and tests exist. | Trigger-to-capture-to-AI-to-policy operational proof remains open. | Close a redacted selected-window capture to typed AI-result proof. |
| `screen-plan` | Foundation | `screen-core`, `screen-capture-adapter`, `screen-live-view-core` | Capture adapters and platform paths exist; screen core has 3 source / 3 test files. | Cross-platform custody and live-view closure are incomplete. | Prove custody/delete behavior for one supported OS capture path. |
| `setup-install-provisioning-plan` | Integration | `provisioning-core`, setup/identity schemas, platform installers | Provisioning and setup readiness code exists. | Depends on identity, device trust, and child/parent installation truth. | Close a parent setup-to-child trust/install handoff after device trust exists. |
| `tracking-plan` | Integration | `tracking-core`, `agent-service`, `schema` | 70 source / 41 test files; location/geofence/device-status runtime exists. | Real device/provider/retention product proof remains incomplete. | Run a provider-to-read-model-to-portal tracking path with retention proof. |
| `v0-8-enforcement-control-plan` | Foundation | `child-enforcement-core`, `policy-control-core`, `agent-core`, `agent-service`, schema | Contract/action-state surfaces and a managed-browser profile/launch boundary exist. PR #606 is closed without merge because its policy slice was unsafe/no-op and is not implementation evidence. | Eventing WP06 generic journal/replay handoff is complete, but WP11 enforcement-specific durable journal and WP04 trusted dispatch remain open. Browser policy targets are manual-required; managed-browser proof/read models do not establish an adapter-backed action receipt, rollback, audit trace, or parent-visible result. | Complete WP11's durable journal/query contract, then schedule WP04 trusted dispatch and one bounded managed-session adapter slice with receipt, rollback, journal, and visible status; keep exact URL and unmanaged browsers out of scope. |

## Workpack execution audit

This table is the current scheduling baseline derived from every routed
`WORKPACK_INDEX.md`, not a completion certificate. `Doc-claimed closed` means
the plan index currently marks the row checked/done. `Freshly reverified` stays
zero until the current branch regenerates the named proof and passes the
focused acceptance gate. Gitignored or absent historical `output/` and
`test-results/` paths cannot be used as retained current proof.

| Plan | Execution rows | Doc-claimed closed | Open / partial / blocked / unknown | Freshly reverified | Scheduling note |
| --- | ---: | ---: | ---: | ---: | --- |
| `account-identity-family-plan` | 8 | 5 | 3 | 0 | Checklist now has 90/103 checked: WP01 remains partial. Merged #632 adds retained D1 storage-adapter proof, but no checklist row changes because token verification, runtime routes, authority, deployment/migration, and WP01 acceptance remain open. Account WP08 is 0/9 open for Rust-schema/account-authority proof, and WP06 is reopened at 14/18 to aggregate Account WP08 plus Cloudflare WP06/WP08 evidence. PR #607's TS adapter/D1-test-double is not a workpack closure. |
| `ai-plan` | 48 | 1 | 47 | 0 | Generic reset checklist does not reflect workpack state. |
| `app-game-plan` | 88 | 54 | 34 | 0 | The remaining 34 are only `possibly done`; audit before implementation. |
| `app-plan` | 95 | 0 | 95 | 0 | Reconciliation rows overlap app/game heavily; deduplicate before delegation. |
| `browser-plan` | 24 | 0 | 24 | 0 | Substantial runtime exists, but every execution row remains open. |
| `child-agent-runtime-distribution-plan` | 11 | 10 | 1 | 0 | Index claims ten complete while the generic checklist reports none. |
| `cloudflare-control-plane-plan` | 13 | 0 | 13 | 0 | #608 is merged with fresh full CI and local WP07 evidence, but no workpack is closed. #604 closed unmerged as superseded/conflicting; retain its branch/evidence and schedule a proof-only WP07 successor only after dependency resolution. |
| `data-custody-storage-plan` | 8 | 7 | 1 | 0 | Workpack index and checklist disagree in both directions on several rows. |
| `device-trust-bootstrap-plan` | 9 | 0 | 9 | 0 | #605 merged with fresh 60-job CI, but it is narrow test evidence only. Five partial, three blocked, one docs-only; adapter-backed runtime closure is missing. |
| `eventing-plan` | 5 | 3 | 2 | 0 | Five selectable workpacks: WP06 and WP10 are open; eight historical rows are excluded and must not be rescheduled. |
| `lan-plan` | 25 | 13 | 12 | 0 | Remaining rows are mainly partial/manual physical proof, not twelve ordinary code packets. |
| `logging-domain-parity` | 10 | 0 | 10 | 0 | Five partial-proof, four source-present, one audit-open. |
| `network-plan` | 8 | 0 | 8 | 0 | Merged #617 proves one bounded WP01 typed-eventing contract handoff; WP01 and its seven sibling workpacks remain open, so this is not a workpack closure. |
| `parent-client-runtime-distribution-plan` | 11 | 7 | 4 | 0 | Routed through `parent-desktop-runtime-package-plan`; state/index and checklist disagree on WP03/WP04. |
| `payment-subscription-plan` | 13 | 3 | 10 | 0 | Engineering specification is not runtime closure; Cloudflare/trust dependencies remain. |
| `policy-control-plane-plan` | 8 | 6 | 2 | 0 | Six checked workpacks are not reflected by the generic checklist status. |
| `portal-ux-household-surfaces-plan` | 20 | 5 | 15 | 0 | Checklist is stale relative to the workpack index. |
| `remote-access-plan` | 6 | 0 | 6 | 0 | Five planned rows and one deferred control row. |
| `screen-ai-pipeline-plan` | 10 | 0 | 10 | 0 | Proof manifest/root is absent; rows correctly remain open. |
| `screen-plan` | 40 | 18 | 22 | 0 | Eighteen checked workpacks are not reflected by the generic checklist. |
| `setup-install-provisioning-plan` | 7 | 6 | 1 | 0 | WP06 is done as a blocker/aggregation packet but remains open for whole-plan scheduling; 93/93 checklist is not product completion. |
| `tracking-plan` | 39 | 0 | 39 | 0 | Internally checked rows were intentionally reopened for audit/proof reruns. |
| `v0-8-enforcement-control-plan` | 20 | 6 | 14 | 0 | #606 closed unmerged as unsafe/no-op. Eventing WP06 Journal Replay And Lineage is now graph-complete with reviewed code/test/proof evidence; WP11 remains open and WP04 remains unscheduled/manual-required. Six checked workpacks are not reflected by the generic checklist. |
| **Total** | **526** | **143** | **383** | **0** | Plus 145 reference/source-only rows and 8 historical rows excluded from execution scheduling. |

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

1. **Reconcile Cloudflare module dependencies, then run the proof-only WP07 successor**: resolve the current Wrangler/Workers-types module prerequisite first; from current source, rerun WP07's focused lint, unit, contract, and worker-boot gates and retain its bundle. Separately route Account WP08 Rust contract -> Cloudflare WP06 binding/migration -> Cloudflare WP08 module-runner proof -> Account WP06 aggregation. Only then schedule dependent payment/billing gates. Do not restore removed TypeScript contract ownership or revive #604.
2. **Build device trust runtime**: parent presence and sealed device trust unblock safe account, setup, payment, remote, and enforcement decisions.
3. **Close policy to enforcement in dependency order**: Eventing WP06's generic handoff is now complete; next is WP11's enforcement-specific durable-journal contract, then WP04 trusted dispatch -> adapter -> receipt -> rollback. Until WP11's actual handoff exists, WP04 remains unscheduled/manual-required. This becomes the reusable control path for browser, app/game, network, and screen.
4. **Use LAN/service as the first physical household proof**: pairing -> device state -> portal read model establishes the multi-device integration baseline.
5. **Close custody and observability on that vertical slice**: correlated logs, retention/delete, and replay make later feature work trustworthy.
6. **Scale feature producers**: browser, app/game, network, screen, tracking, and AI can then feed the same decision and evidence spine.
7. **Finish portal and distribution proof**: run real click-through and package/smoke proof only after the underlying product paths exist.

## Checklist synchronization rule

Every plan checklist should retain unchecked rows unless the named workpack has
all of the following: implementation, focused tests, retained proof artifact,
and an accepted merge state. A checklist audit may record current code evidence
and a blocker, but must never turn a code inventory into a completion claim.
