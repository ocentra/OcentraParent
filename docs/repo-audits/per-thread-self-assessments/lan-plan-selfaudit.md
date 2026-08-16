# lan-plan

## Normalized Header

- plan/thread name: `lan-plan`
- source thread label: dedicated `lan-plan` Codex worker thread
- source thread id: `019ed329-3916-7801-ac90-d0eb68254d3e`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: active; `Slice A` green for the reconciliation scope; not done
- claimed source files/crates/packages: `packages/lan-domain`; `packages/agent-protocol-domain`; `crates/agent-protocol`; `crates/agent-service`; `apps/portal`; `scripts/test/v0-9-*.mjs`; `output/lan-plan-proof/`
- claimed tests: `npx vitest run tests/unit/v0-9-production-discovery-household-proof.test.ts`; `npm run test --workspace @ocentra-parent/lan-domain`; missing honest `contract`, `integration`, `security`, `property`, portal LAN, and scoped Rust LAN coverage where applicable
- claimed proof commands/artifacts: `output/lan-plan-proof/00-plan-model-reconciliation/00-source-snapshot.md`; `output/lan-plan-proof/00-plan-model-reconciliation/01-lan-domain-validation.log`; `output/lan-plan-proof/00-plan-model-reconciliation/02-plan-truth-sync.md`; `output/lan-plan-proof/00-plan-model-reconciliation/03-missing-proof-inventory.md`; next named proof commands `node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs`, `node scripts/test/v0-9-household-lan-proof-readiness.mjs`, `node scripts/test/v0-9-production-discovery-household-proof.mjs`
- claimed blockers: real open implementation in `05`, `06`, `07`, `08`, `09`, `11`, `17`; regenerated proof missing for partial rows; physical/manual/runtime proof missing for `15`, `16`, `18`, `19`, `20`
- claimed next actions: `B1` LAN proof regeneration; `B2` test truth repair; `C1` protocol/service truth repair; then open implementation cluster and runtime/physical proof
- obvious missing evidence fields: current regenerated proof pack; scoped `cargo` validation; targeted portal LAN proof; Android/Linux/second-device artifacts; signed child hello/heartbeat artifacts; honest non-placeholder test category coverage
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**COMPLETION_ARCHITECTURE_REPORT**

`Slice A` is actually green for the narrow reconciliation scope. I repaired the `packages/lan-domain` export/ownership path, restored the failing focused proof-schema test, got the full `@ocentra-parent/lan-domain` test/build/architecture gate green, truth-synced the LAN docs to make `01-20` authoritative and `21-25` frozen, and bootstrapped an honest proof root under `output/lan-plan-proof/00-plan-model-reconciliation/`.

The plan is not close to honest closure yet. `05`, `06`, `07`, `08`, `09`, `11`, and `17` are still real implementation gaps. `15`, `16`, `18`, `19`, and `20` still need regenerated proof and some physical/manual evidence. Test organization is still misleading outside the green `lan-domain` slice, and historical proof references were overstating completion.

**Current Checkpoint**
Changed now:
- `packages/lan-domain/src/lan-pairing.ts`
- `packages/lan-domain/src/v0-9-production-discovery-household-proof.ts`
- `docs/plans/lan-plan/source-index.md`
- `docs/plans/lan-plan/PLAN_STATE.md`
- `docs/plans/lan-plan/WORKPACK_INDEX.md`
- `docs/plans/lan-plan/implementation-checklist.md`
- `docs/plans/lan-plan/PROOF_INDEX.md`
- `docs/plans/lan-plan/current-lan-snapshot.md`

Created now:
- `output/lan-plan-proof/00-plan-model-reconciliation/00-source-snapshot.md`
- `output/lan-plan-proof/00-plan-model-reconciliation/01-lan-domain-validation.log`
- `output/lan-plan-proof/00-plan-model-reconciliation/02-plan-truth-sync.md`
- `output/lan-plan-proof/00-plan-model-reconciliation/03-missing-proof-inventory.md`

**Exact Read Set**
Docs read in the plan lane:
- `docs/plans/lan-plan/AGENTS.md`
- `docs/plans/lan-plan/PLAN_STATE.md`
- `docs/plans/lan-plan/WORKPACK_INDEX.md`
- `docs/plans/lan-plan/implementation-checklist.md`
- `docs/plans/lan-plan/PROOF_INDEX.md`
- `docs/plans/lan-plan/source-index.md`
- `docs/plans/lan-plan/current-lan-snapshot.md`
- `docs/plans/lan-plan/TEST_PROOF_EXPECTATIONS.md`
- `docs/plans/lan-plan/PLAN_EXECUTION_BLUEPRINT.md`
- `docs/plans/lan-plan/ui-ux-requirements-guide.md`
- `docs/plans/lan-plan/v0-9-lan-discovery-20-step-plan.md`
- `docs/plans/lan-plan/v0-9-lan-discovery-test-blueprint.md`
- the LAN workpack corpus `01-25` in the plan folder tree

Source/test/proof surfaces read:
- `packages/lan-domain/src/*lan*`, `*household*`, `*mobile-controller*`
- `packages/lan-domain/tests/unit/*.test.ts`
- `packages/agent-protocol-domain/tests/unit/*lan*`
- `crates/agent-service/src/lan_*`
- `crates/agent-protocol/src/lan_*`
- `apps/portal/tests/live-activity-network-flow.test.ts`
- `apps/portal/tests/transport-lan-target.test.ts`
- `apps/portal/e2e/portal-ui.spec.ts`
- `scripts/test/v0-9-*.mjs` LAN proof generators
- current and missing proof roots under `output/lan-plan-proof/`, `output/playwright/`, `test-results/`, and `docs/proof/lan-plan/`

**Current Truth**

| Area | Status | Evidence | Remaining truth |
| --- | --- | --- | --- |
| Slice A reconciliation | done | `output/lan-plan-proof/00-plan-model-reconciliation/` | narrow scope only |
| Workpacks `01-04`, `10`, `12-16`, `19`, `20` | partial | current `lan-domain` source plus truth-synced docs | proof regeneration and some runtime/physical closure still missing |
| Workpacks `05`, `06`, `07`, `08`, `09`, `11`, `17` | missing | docs now say "no implementation claimed" | real implementation plus tests/proof still needed |
| Workpack `18` | partial/manual | contract/read-model rows exist | real signed artifacts and device proof missing |
| Workpacks `21-25` | frozen | docs now explicitly freeze them | not allowed to drive completion claims |
| `packages/lan-domain` category coverage | false-green | many top-level test category folders are `.gitkeep` only | must stop counting empty categories as coverage |
| historical LAN proof references | false-green | missing paths are now inventoried in `03-missing-proof-inventory.md` | must regenerate or remove from claims |

**Code Surface And Ownership**

| Surface | Authoritative owner now | Notes |
| --- | --- | --- |
| TypeScript LAN contracts, proofs, source matrix, household/device spine | `packages/lan-domain` | current executable owner |
| TS protocol/domain companion | `packages/agent-protocol-domain` | real downstream consumer/contract surface |
| Rust protocol serialization/boundary | `crates/agent-protocol` | LAN test placement still dirty |
| Rust runtime/service LAN inventory, pairing, device spine | `crates/agent-service` | major implementation/proof surface for open workpacks |
| Portal LAN consumers/proof | `apps/portal` | consumer only, not source-of-truth owner |
| legacy LAN references | `packages/parent-domain/src/lan-*` | stale/compatibility only; not valid for completion claims |

**Test / Proof / Validation**
Test inventory issues:
- Placeholder-only folders in `packages/lan-domain/tests`: `ai-safety`, `chaos`, `clock-skew`, `concurrency`, `consumer-driven`, `contract`, `differential`, `e2e`, `human-misuse`, `integration`, `invariant`, `load`, `migration`, `monitoring`, `mutation`, `property-based`, `release`, `security`.
- Honest unit tests can stay unit-scoped: `device-roles.test.ts`, `household-device-spine.test.ts`, `lan-pairing.test.ts`, `lan-pairing-browser-runtime.test.ts`, `lan-pairing-browser-add-device-state.test.ts`, `package-info.test.ts`.
- Tests that should move out of `tests/unit` into real `tests/contract` or `tests/integration`: `lan-discovery-source-matrix.test.ts`, `lan-pairing-product-proof.test.ts`, `lan-pairing-provider-selection-proof.test.ts`, `lan-production-household-proof.test.ts`, `v0-9-household-discovery-mobile-controller-product-proof.test.ts`, `v0-9-household-lan-pairing-proof.test.ts`, `v0-9-household-multi-device-proof-gates.test.ts`, `v0-9-household-physical-proof-artifact-gate.test.ts`, `v0-9-mobile-controller-discovery-runtime.test.ts`, `v0-9-mobile-controller-observer-runtime.test.ts`, `v0-9-production-discovery-household-proof.test.ts`.
- Rust LAN tests still embedded under `src`, including `crates/agent-service/src/lan_pairing_tests.rs` and `crates/agent-protocol/src/lan_pairing_tests.rs`; these should move into crate-level `tests/` major categories for honest closure.
- Applicable but missing as real coverage today: LAN contract tests, LAN integration/runtime tests, LAN security/abuse tests, and property/invariant tests around source weighting, dedupe, authority, and route rejection.
- `load` is not evidenced as a current `01-20` done-bar requirement; there is no honest load proof today and it should not be implied.

Proof inventory:
- Active canonical proof root today: `output/lan-plan-proof/00-plan-model-reconciliation/`
- Completion root convention should be `output/lan-plan-proof/<NN-slice-name>/`
- Real current artifacts: the four files in `00-plan-model-reconciliation/`
- Stale/missing proof called out explicitly: `test-results/v0-9-lan-source-matrix-plan-completion/proof.json`, `output/playwright/lan-source-matrix-plan-completion/*`, `output/lan-plan-proof/15-household-device-store/*`, `docs/proof/lan-plan/PLAN_PROOF_MANIFEST.md`

Scoped validation inventory:

| Command | State | Note |
| --- | --- | --- |
| `npx vitest run tests/unit/v0-9-production-discovery-household-proof.test.ts` | passed | focused regression fixed |
| `npm run test --workspace @ocentra-parent/lan-domain` | passed | 67 tests |
| `npm run build --workspace @ocentra-parent/lan-domain` | passed | package build green |
| `npm run lint:architecture -- --files packages/lan-domain` | passed | touched LAN package green |
| `node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs` | unrun | required next for honest proof regeneration |
| `node scripts/test/v0-9-household-lan-proof-readiness.mjs` | unrun | required next |
| `node scripts/test/v0-9-production-discovery-household-proof.mjs` | unrun | required next |
| scoped `agent-protocol-domain` LAN tests | unrun | needed before protocol closure claims |
| scoped `cargo` LAN tests / architecture gates | unrun | needed before Rust closure claims |
| targeted portal LAN tests/e2e | unrun | needed before portal proof claims |

**Dependency Map**

| Bucket | Item | Why it matters |
| --- | --- | --- |
| `local-now` | `B1` proof regeneration across `scripts/test/v0-9-*.mjs` and `output/lan-plan-proof/01-*` | no external dependency; restores honest proof for partial rows |
| `local-now` | `B2` test truth repair in `packages/lan-domain/tests` | removes fake-green coverage optics |
| `local-now` | `C1` Rust/TS LAN contract audit and test relocation | needed before honest service/protocol closure |
| `needs-coordinator-sequencing` | `eventing-plan`, `remote-access-plan` | both should consume post-`B1` LAN truth, not stale proofs |
| `needs-sibling-plan-contract` | `portal-ux-household-surfaces-plan` | only needed if `15/16/19/20` still claim portal household/device UI evidence |
| `needs-sibling-plan-contract` | `account-identity-family-plan` | only needed if LAN rows still claim family/account-bound selectors or flows |
| `host-platform-limited` | real iOS/macOS artifacts | only relevant if any remaining LAN claim explicitly names Apple-host proof; not required for current Windows-first execution |

Platform feasibility:
- Windows host now: all doc truth-sync, `lan-domain` work, targeted portal tests, proof scripts, and scoped Rust/TS validation.
- Android Studio / device: required for real mobile-controller and second-device household proof on `15`, `16`, `18`, `19`, `20`.
- Linux / WSL / Docker: required where `04`, `07`, `08`, `09`, `17` need packet, neighbor-table, or network-advertisement proof beyond Windows.
- Apple-host-only: only if docs insist on real iOS/macOS proof. Current authoritative `01-20` model does not require that to keep moving now.

**No-Hand-Wave Execution Plan**

| Slice | Main files / domains | Validation / proof | Exit criteria |
| --- | --- | --- | --- |
| `B1` LAN proof regeneration | `scripts/test/v0-9-lan-source-matrix-plan-completion.mjs`, `v0-9-household-lan-proof-readiness.mjs`, `v0-9-production-discovery-household-proof.mjs`, sibling LAN proof scripts, `output/lan-plan-proof/01-lan-proof-regeneration/` | rerun current `lan-domain` green set plus these scripts | every proof path cited for partial `01-04`, `10`, `12-16`, `19`, `20` exists and points at current `lan-domain` ownership |
| `B2` test truth repair | `packages/lan-domain/tests/*`, possible `vitest` config/package wiring | targeted `vitest` per moved category plus workspace test run | no empty-folder optics counted as coverage; contract/integration tests moved out of `unit` |
| `C1` protocol/service truth repair | `packages/agent-protocol-domain/*lan*`, `crates/agent-protocol/src/lan_*`, `crates/agent-service/src/lan_*` | scoped TS tests; scoped `cargo` tests and architecture gates | current Rust/TS LAN contracts align, and LAN tests stop living in `src` where they are being counted as implementation |
| `C2` open implementation cluster | workpacks `05`, `06`, `07`, `08`, `09`, `11`, `17` across `crates/agent-service`, `packages/lan-domain`, proof scripts | smallest scoped unit/integration/packet-proof coverage per workpack | those rows stop saying "no implementation claimed" |
| `D1` read-model / portal closure | `apps/portal/tests/live-activity-network-flow.test.ts`, `apps/portal/tests/transport-lan-target.test.ts`, any LAN-scoped e2e added under `apps/portal/e2e/` | targeted portal tests and honest screenshots/logs under `output/lan-plan-proof/03-portal-lan-proof/` | portal evidence matches LAN source/service truth and no longer depends on stale screenshots |
| `D2` runtime / physical proof | Android emulator/device flows, Windows runtime captures, Linux/WSL packet captures; `v0-9-mobile-controller-*`, `v0-9-household-physical-proof-artifact-gate.mjs`, signed-artifact proof scripts | real device/emulator/runtime artifacts under `output/lan-plan-proof/04-runtime-physical-proof/` | `15`, `16`, `18`, `19`, `20` have real restart/replay/second-device/signed-artifact evidence |
| `D3` final truth sync | `PLAN_STATE.md`, `WORKPACK_INDEX.md`, `implementation-checklist.md`, `PROOF_INDEX.md`, `current-lan-snapshot.md` | doc-only reconciliation against real artifacts | every `01-20` row is either genuinely proved or explicitly still open/manual-required and not counted as complete |

First coordinator ask:
- No predecessor plan should move before `B1`.
- The next sequencing intervention should be to keep `eventing-plan` and `remote-access-plan` behind the refreshed LAN proof set, and defer `portal-ux-household-surfaces-plan` / `account-identity-family-plan` involvement until a LAN row actually needs those contracts for closure.

**Strict Done Bar**
Before this plan can ever be marked done:
- `01-20` must remain the only authoritative execution rows.
- Every counted-complete row must have real code in the owning surface, real tests in proper `tests/` major categories where applicable, and a real artifact under `output/lan-plan-proof/`.
- No `.gitkeep` folder, old checkbox, or missing screenshot/log may be used as coverage.
- No stale `parent-domain` ownership or missing proof path may remain in the docs.
- Open/manual-required rows may remain only if they are explicitly not counted as done.
- Physical/runtime claims must have Windows/Android/Linux proof where relevant; Apple-only proof is required only if a remaining claim explicitly demands it.

**COORDINATOR_DECISION_REQUEST**
- Recommended next slice: `B1` local LAN proof regeneration.
- Recommended predecessor plans: none before `B1`; hold `eventing-plan` and `remote-access-plan` until `B1` artifacts exist.
- Estimated risk: medium-high for full plan closure because seven authoritative workpacks are still open implementation.
- Estimated proof difficulty: high because final closure needs regenerated proofs plus Android/Linux/physical evidence.
- Continue or pause: continue immediately with `B1`; do not pause for sequencing unless you want to pre-assign the later portal/account contract slices now.

<oai-mem-citation>
<citation_entries>
MEMORY.md:33-46|note=[codex-a lane and branch coordination expectations]
MEMORY.md:54-79|note=[Ocentra Parent lane and proof coordination context]
</citation_entries>
<rollout_ids>
019ec1fe-7411-7c22-be2f-382ce2665635
</rollout_ids>
</oai-mem-citation>

## Optional Addendum

- Earlier audit passes identified a broader remaining LAN proof-script surface than the three `B1` commands named in the raw report, including `scripts/test/v0-9-household-lan-product-proof.mjs`, `scripts/test/v0-9-production-lan-household-proof.mjs`, `scripts/test/v0-9-production-lan-mobile-controller-proof.mjs`, `scripts/test/v0-9-production-lan-multidevice-hardening.mjs`, `scripts/test/v0-9-lan-signed-discovery-relay-spine.mjs`, and the mobile-controller / physical-proof gate scripts. The raw report is a sequenced completion proposal, not a full proof-script inventory.
- Earlier audit passes also found additional inline/src-style LAN test surfaces beyond the two top-level Rust filenames named in the raw report, especially under `crates/agent-service/src/lan_pairing_browser_add_device_state/` and `crates/agent-service/src/lan_pairing_household_device_spine/`. Those should be treated as part of the remaining test-organization debt.
