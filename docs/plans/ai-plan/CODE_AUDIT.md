# AI Plan Code Audit

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Code Audit`
> Kind: code-and-test implementation audit; use for Phase 1 status.
> Read when: Asked how much AI work is actually implemented, or before selecting the next AI workpack.
> Stop rule: This audit records source/test presence and gaps; it does not authorize a blocked workpack or replace focused validation and proof.
> Proves: reviewed implementation and expected-test topology as of 2026-08-15.
> Does not prove: tests currently pass, Enforcer is green, proof is retained, CI is green, or a workpack is DONE.

<!-- /agent-capsule -->

## Audit method

This is a source-first audit. Each row was checked against production source and
test source in the current repository. Plan checkboxes and historical proof were
not accepted as implementation evidence. Phase 1 means the bounded production
code and expected tests are written. Phase 2 (executing focused tests and
Enforcer) and Phase 3 (proof regeneration) were intentionally not run here.

`packages/ai-domain` is absent. General cross-boundary AI contracts currently
live mainly in `crates/agent-protocol`; Rust service execution lives mainly in
`crates/agent-service`; `child-ai-core` contains only the child-domain completion
handoff and tracking boundary, not a general evaluator/runtime.

## Current WP03 source checkpoint — 2026-08-25

WP03's Rust-owned source is present in the canonical consolidation at source
commit `6318d5e3d`. Independent review accepted the source-preserving move of
the full contract family into the neutral `ocentra-ai-contracts` leaf at
`crates/ai-contracts`; the schema exporter now consumes that leaf directly and
continues to produce the generated TypeScript edge surface. The packet changed
source/metadata only; it did not add a general production caller. The WP04
agent-protocol adapter remains a separate implementation task. The expected
test source is still missing at `crates/ai-contracts/tests/contract/ai_contracts.rs`,
`crates/ai-contracts/tests/contract/ai_contracts_negative.rs`, and
`packages/schema-domain/tests/contract/ai-contracts.test.ts`. Therefore this
audit records accepted implementation-only source evidence with caller, tests,
WP04 adapter, proof, READY, and DONE open.

## Summary

| Phase 1 state | Workpacks | Count |
| --- | --- | ---: |
| Complete for bounded scope | 01, 02, 05, 06, 08, 10, 11, 24, 26, 32, 37 | 11 |
| Incomplete or partial | 03, 04, 07, 09, 12-23 except 24, 25, 27-31, 33-36, 38-48 | 37 |

No row below is a Phase 2 or Phase 3 claim.

## Workpack matrix

| WP | Phase 1 | Production code and test code found | Remaining code/test gap |
| ---: | --- | --- | --- |
| 01 | Complete | Current repository and external TabAgent reference roots were reconciled during this audit. | Keep the source index synchronized when owners move. |
| 02 | Complete | This audit and the refreshed current snapshot now separate implemented, partial, missing, and proof-only state. | Refresh after each merged AI slice. |
| 03 | Incomplete | Rust-owned AI contract family is independently accepted at source commit `6318d5e3d` in the neutral `crates/ai-contracts` leaf, including journal/result digests and the schema-generated `packages/schema-domain` edge source. | Explicit WP04 adapter, no general production caller, and the three WP03 contract/negative/parity test roots remain open; Phase 1 is not closed. |
| 04 | Incomplete | Rust structs/constants and serialization tests exist for runtime status, context wire, parent assistant, and memory graph. | No complete generated TypeScript parity fixture/negative decoder set for the whole AI contract family. |
| 05 | Complete | Typed runtime/load/degraded/unavailable states, service payload/readiness logic, portal runtime card, and focused source tests exist. | Execute focused tests in Phase 2. |
| 06 | Complete | Typed task capabilities, resource classes, local-only boundary, scheduler state, household fallback ordering, and rejection tests exist. | Execute focused protocol/service/route tests in Phase 2. |
| 07 | Incomplete | Local singleton queueing and LAN claim/lease/idempotency/requeue/dead-letter implementations have tests. | No neutral general `AiWorkItem`/`AiWorkState` contract or one durable lifecycle covering queue, claim, start, validate, accept/reject, cancellation, TTL/deadline, retry, and replay across AI lanes. |
| 08 | Complete | Same-device scheduler, household provider preference, mobile dormant/fallback, stale/offline/revoked/custody rejection, LAN route metadata, and route tests exist. | Physical/multi-service route proof remains Phase 3. |
| 09 | Incomplete | Parent Assistant builds bounded Activity/report evidence summaries; AI context request/result wire shapes exist. | No general builder reads the SQLite/read-model sources into `LocalAiEvidenceContext`, validates all custody/source combinations, and returns every typed ready/partial/rejected state. |
| 10 | Complete | Context source refs include evidence identity, custody, freshness, confidence, capability/degraded state, and derived source refs with serialization tests. | Execute focused contract tests in Phase 2. |
| 11 | Complete | ActivityStore parent-rule selection handles child/device/target/time, expiry, schedules, conflicts, grounding, and stricter precedence with focused tests. | The generic AI context builder still needs to consume this boundary under WP09. |
| 12 | Incomplete | Version constants exist for Parent Assistant, network AI, app/game classifier, and screen adapter prompts. | No task-keyed prompt registry, migration/deprecation policy, or cross-task regression fixture catalog. |
| 13 | Incomplete | Deterministic browser/app-game/tracking classifiers and policy precedence code exist in owning crates. | No unified no-model classifier lane escalates only ambiguous results into the general AI job route with one typed result contract. |
| 14 | Incomplete | A real local `llama.cpp` process adapter and bounded request parser exist. | It accepts a prompt string rather than only validated context-builder output and has no general raw-source rejection or parser-boundary contract. |
| 15 | Incomplete | The service can execute configured `llama.cpp`, enforce timeout/max tokens, and return typed generation state. | No end-to-end dry-run path builds validated context, parses/schema-validates output, journals the candidate, and proves enforcement remains disabled. |
| 16 | Incomplete | Screen adapter JSON parsing/redaction and typed generation result code exist. | No general AI result parser rejects invalid JSON, missing evidence/rule refs, direct enforcement fields, contradiction, and low confidence as one tested boundary. |
| 17 | Incomplete | Runtime process failure/timeout/empty-output/unavailable and busy scheduler states are typed and tested. | No integrated deterministic fallback plus durable failure journal/read model for invalid/overloaded/timed-out AI output. |
| 18 | Incomplete | `policy_dry_run_evaluator` consumes a typed local AI result, preserves evidence, applies parent-rule precedence, and fails safe in tests. | The integrated path does not durably journal both accepted AI result and resulting policy decision, and policy-version/context linkage is incomplete. |
| 19 | Incomplete | AI generation and screen analysis emit service/activity events. | No canonical AI-result journal event with all evidence/rule/runtime/prompt/memory/graph refs, no dedicated SQLite ingest/read model, and no replay tests. |
| 20 | Incomplete | Parent Assistant returns cited local answers, degraded/runtime metadata, preview-only actions, and portal boundary cards. | No unified explanation read model joins policy action, AI result, rule refs, confidence/degraded state, runtime/prompt refs, and portal rendering. |
| 21 | Incomplete | Memory reference kinds, source evidence, policy version, generation time, confidence, and index version exist. | The general memory reference lacks expiry/invalidation and parent-action linkage, plus a fail-closed unsourced-memory validator. |
| 22 | Incomplete | Durable recent Activity/report history and a cited Activity memory graph exist. | No bounded recent-memory product contract applies expiry/invalidation and feeds the general AI context builder only through validated memory refs. |
| 23 | Incomplete | The contract enum reserves semantic memory. | No semantic entry/index/embedding implementation, rebuild/invalidation flow, or tests exist. |
| 24 | Complete | Activity memory graph contracts include typed nodes/edges, source evidence, policy/action refs, generated/expiry/confidence/index trace, plus serialization tests. | Execute focused contract tests in Phase 2. |
| 25 | Incomplete | SQLite-derived activity graph nodes and device-to-activity edges persist/rebuild with citation tests. | Policy-rule/target, AI-result/evidence, policy-decision/result, and parent-action/decision edges are missing. |
| 26 | Complete | All listed `E:\Desktop\TabAgent` reference files still exist; the reuse/non-reuse map is retained and no TabAgent code has been copied. | If extraction starts later, add license/ownership plus extraction-specific parity and route-negative tests before copying code. |
| 27 | Incomplete | Ocentra has typed local runtime status, queueing, timeout, and degraded states. | No reused/adapted persistent native transport boundary with reconnect/invalid-payload tests exists. |
| 28 | Incomplete | Model lifecycle/cache/download/corruption/resume state enums and local install-plan metadata exist. | No actual verified download/progress/cache implementation, checksum/license validation, corruption recovery, or resume tests. |
| 29 | Incomplete | The Activity memory graph persists source-cited nodes/edges and rebuilds from SQLite rows. | Expiry/invalidation and explicit journal-driven rebuild semantics required by the reuse candidate are incomplete. |
| 30 | Incomplete | Screen queue, digest/custody fields, local adapter request, redacted OCR-shaped parser, deletion lifecycle, and focused source tests exist. | No owned OCR engine/worker execution test and no real browser/app/cadence capture OCR tests are written. |
| 31 | Incomplete | Screen routing contracts can select an intelligence route and reject protected/credential surfaces. | No guided question set, VLM worker/result implementation, raw-image deletion integration, or real browser/app VLM tests exist. |
| 32 | Complete | Household provider roles/routes, mobile dormant/fallback, LAN job claim/lease/idempotency, worker-only custody, child validation/authority, no-raw-transfer, and optional remote-assistant boundary all have source tests. | Multi-device physical proof remains Phase 3; remote provider execution is intentionally unavailable and belongs to WP45. |
| 33 | Incomplete | Managed browser URL/video evidence and deterministic social/video classification code exist. | Ambiguous evidence is not routed through the general local text-model/context/result/policy spine. |
| 34 | Incomplete | Typed social/feed/video/signup route and privacy contracts exist in browser-owned code. | No complete AI result path joins parent targets, approved screen fallback, confidence/degraded output, and policy handoff. |
| 35 | Incomplete | Browser-game URL shape/catalog and policy-candidate compilation code/tests exist. | Cloud-game ambiguity and approved screen/local-AI fallback are not integrated into one browser-game AI evidence result. |
| 36 | Incomplete | App/game inventory, launcher, foreground, authority-classifier, and read-model classifications exist with tests. | No complete unknown-classifier route joins duration evidence, approved screen summary, confidence/risk result, and AI result validation. |
| 37 | Complete | Tracking request/result boundaries consume cited evidence, expected-place/schedule context, stale/correlation/ambiguity states, local nearby-place classification, and deterministic policy handoff tests. | Execute focused child-ai/tracking tests in Phase 2. |
| 38 | Incomplete | Screen route planning, protected/manual states, evidence-first rules, queue/lease/cadence, local adapter, and deletion source exist. | Router modules lack direct unit coverage and do not prove OCR-vs-guided-VLM-vs-text fallback over real browser/app/cadence inputs without queue flood. |
| 39 | Incomplete | Runtime resource classes, acceleration config, desktop/mobile preference, and battery/thermal route rejection exist. | No real CPU/RAM/GPU capability capture and no explicit fits/maybe/too-large/unsupported model-task fit engine exposed end to end. |
| 40 | Incomplete | A one-model registry, manifest reference, cache paths, and typed integrity/download/corruption states exist. | Registry entries lack enforced checksum/version/license/source; no downloader, resume, integrity verification, corruption quarantine, or tests exist. |
| 41 | Incomplete | Platform-specific `llama.cpp` asset selection, cache/install-plan paths, acceleration args, and configured execution exist with tests. | No owned download/extract/checksum/version/repair/uninstall pipeline or real generation smoke/degraded packaging test exists. |
| 42 | Incomplete | Max token/timeout guards, deterministic temperature, acceleration settings, and prompt version constants exist. | No per-task versioned inference-settings registry, settings-in-result contract, or prompt regression fixture family. |
| 43 | Incomplete | Portal renders live AI runtime, household job, memory graph citations, remote boundary, and degraded/unavailable cards; focused portal tests exist. | No unified policy/AI decision-explanation card and no full job/activity history surface; current rows are latest service projections. |
| 44 | Incomplete | Typed API authorization/custody/retention/deletion boundary defaults remote AI off and rejects child-safety/enforcement use; tests cover unavailable/degraded states. | Authorization is env plus caller payload state and lacks a trusted one-shot parent-action reference/consumption boundary. |
| 45 | Incomplete | Parent Assistant can build minimized cited Activity/report prompts and falls back to local/unavailable states. | No remote provider adapter, trusted approval consumption, redacted bundle transport, cited remote response validation, or remote-failure integration test exists. |
| 46 | Incomplete | Multiple boundaries reject raw screenshot transfer, direct enforcement authority, stale/custody-mismatched providers, and unauthorized remote use. | No complete negative suite proves no direct OS/browser/network/screen scans, prompt minimization, unsourced memory/graph rejection, and custody guards together. |
| 47 | Incomplete | Singleton queue limits, child-safety priority, desktop preference, mobile battery/thermal rejection, queue state, and portal status code/tests exist. | Real hardware-fit measurement, backpressure load tests, foreground/user-active behavior, portal responsiveness, and school/out-of-LAN fallback policy are missing. |
| 48 | Incomplete | Source topology is now reviewed and mapped for all 48 workpacks. | Phase 1 has 37 incomplete rows; Phase 2 focused execution/Enforcer and Phase 3 proof/CI cannot start as plan-close gates yet. |

## Dependency-first Phase 1 order

1. WP03/WP04 contract ownership and parity.
2. WP07 general durable AI work lifecycle.
3. WP09 context builder plus WP12 prompt registry.
4. WP14-WP17 local text execution/parser/degraded boundary.
5. WP19 AI result journal/SQLite ingest, then WP18/WP20 integration.
6. WP21-WP23/WP25 memory and graph closure.
7. WP40/WP41/WP42 artifact/runtime governance.
8. WP30/WP31/WP38 screen OCR/VLM execution.
9. WP33-WP36 feature bridges, then WP43-WP47 product/security/performance closeout.
10. WP48 only after the preceding rows are Phase 1 complete.
