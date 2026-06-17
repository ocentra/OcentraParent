# WP07 Orphaned Legacy Surface Inventory

## Objective

Find old, weakly-owned, pre-eventing, transitional, or shadow source paths that still exist in the repo and can mislead plan workers.

This is a structural audit workpack. It does not move code by itself.

## Scope

Inspect broad or historically overloaded surfaces first:

- `packages/parent-domain/src/**`
- `packages/portal-domain/src/**`
- `packages/agent-protocol-domain/src/**`
- `crates/agent-core/src/**`
- `crates/agent-service/src/**`
- `crates/agent-protocol/src/**`
- `scripts/test/**`
- plan proof/index docs that still reference old owners

## Classification

| Class | Meaning |
| --- | --- |
| active owner | Current source of truth for behavior. |
| adapter/frontage | Allowed bridge, display, or transport surface. |
| legacy shim | Kept only for compatibility; must not drive proof or ownership. |
| orphan | No current consumer or proof route justifies the file. |
| pre-eventing shadow | Old local implementation parallel to the eventing/runtime model. |
| stale proof wrapper | Script or doc still pointing at old owner/test path. |

## Output table

| File / surface | Class | Current consumer | Preferred owner | Risk | Action |
| --- | --- | --- | --- | --- | --- |
| `packages/parent-domain/src/app-game-*.ts` | legacy shim | Parent-facing imports and app-game proof harnesses such as `scripts/test/app-game-ai-output-direct-enforcement-gate-proof.mjs` | `packages/app-game-domain/src/**` | Broad `parent-domain` frontage makes app-game ownership and proof routing look parent-owned even when the files only re-export `@ocentra-parent/app-game-domain/*`. | convert to adapter |
| `packages/parent-domain/src/tracking-*.ts` | legacy shim | Tracking-facing parent imports and tracking proof/status surfaces | `packages/tracking-domain/src/**` | Tracking runtime, read-model, and proof surfaces stay shadowed behind `parent-domain`, which hides the narrow owner package from plan workers. | convert to adapter |
| `packages/parent-domain/src/local-ai-runtime.ts` | legacy shim | Parent-facing callers and screen-AI pipeline routing that still cite `parent-domain` | `packages/ai-domain/src/local-ai-runtime.ts` | AI runtime ownership drifts behind `parent-domain` even though the file is only `export * from '@ocentra-parent/ai-domain/local-ai-runtime';`. | convert to adapter |
| `packages/portal-domain/src/contracts.ts` | legacy shim | Portal callers that import one broad surface for panels, commands, proofs, and route helpers | Narrow files under `packages/portal-domain/src/**` | The file is a large re-export frontage surface, so UI proof/read-model access can blur into source-truth claims and keep architecture-ban export debt alive. | block pending owner decision |
| `packages/agent-protocol-domain/src/primitives.ts` | legacy shim | TS protocol callers importing protocol-domain primitives | `packages/event-domain/src/primitives.ts` | Transport-facing consumers can mistake protocol-domain for the canonical primitive owner even though the file only re-exports event primitives. | convert to adapter |
| `packages/agent-protocol-domain/src/activity-surface-adapter.ts` | adapter/frontage | Activity service/UI spine that translates protocol envelopes into read-model state | `packages/agent-protocol-domain/src/activity-surface-adapter.ts` | This is a valid adapter seam today, but it becomes risky if product decisions migrate into the protocol boundary instead of staying in narrow domains or Rust owners. | keep |
| `crates/agent-protocol/src/lib.rs` | legacy shim | Downstream crates importing the crate root | Narrow modules under `crates/agent-protocol/src/**` | The crate root re-exports a large cross-plan surface, which hides transport-schema ownership boundaries and keeps `pub use` debt alive at the broadest ingress. | block pending owner decision |
| `crates/agent-core/src/lib.rs` | legacy shim | Downstream runtime crates and tests importing the crate root | Narrow modules under `crates/agent-core/src/**` | `agent-core` is already plan-dense; the crate-root `pub use` surface makes local runtime composition look like the owner for every sub-surface and blocks honest shrinkage into focused crates. | block pending owner decision |
| `scripts/test/app-game-ai-output-direct-enforcement-gate-proof.mjs` | stale proof wrapper | `test-results/app-game-ai-output-direct-enforcement-gate-proof/` and `output/app-game-plan-proof/merge-gates/ai-output-direct-enforcement/` | Narrow app-game-domain proof rooted on canonical owner contracts, with parent-domain used only when a parent-facing adapter is the explicit subject | The harness reads and tests `packages/parent-domain/src/app-game-category-risk-policy-routing.ts`, so proof can reinforce a re-exported parent frontage as if it were the canonical owner surface. | block pending owner decision |
| `scripts/test/app-game-ai-classifier-boundary-proof.mjs` | stale proof wrapper | `test-results/app-game-ai-classifier-boundary-proof/`, `output/app-game-plan-proof/24-ai-classifier-digest-boundary/`, and `output/app-plan-proof/23-app-ai-classifier-digest-boundary/` | Narrow app-game-domain classifier proof, with parent-domain only for explicit parent-surface adapter assertions | The harness imports `../../packages/parent-domain/dist/app-game-ai-classifier-boundary*.js`, which keeps parent-domain in the proof spine even though the evidence contract also lives in `app-game-domain`. | block pending owner decision |
| `docs/plans/lan-plan/PROOF_INDEX.md` | stale proof wrapper | LAN workers selecting authoritative proof roots | `output/lan-plan-proof/<workpack-id>-<short-slug>/` only | The doc still records `docs/proof/lan-plan/PLAN_PROOF_MANIFEST.md` as an absent previously cited path, so workers can still be pulled toward a dead proof root if they skim instead of reading the warning closely. | delete |
| `docs/plans/network-plan/NEXT_ACTIONS.md` | stale proof wrapper | Network workers using the resume queue to choose the next slice | Current branch/workpack proof roots after plan-truth repair | The resume doc explicitly normalizes missing `docs/proof/network-plan/` and missing `output/network-plan-proof/` roots, which means workers are still being routed around absent legacy proof paths instead of a repaired canonical route. | move |
| `docs/plans/screen-ai-pipeline-plan/NEXT_ACTIONS.md` | stale proof wrapper | Screen-AI workers using the audit-first blocker list | Retained `output/screen-ai-pipeline-proof/**` plus a real `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` before closure claims | The doc states that both the retained proof root and the manifest are missing, so plan routing still depends on declared-but-absent proof artifacts. | move |
| `docs/plans/parent-desktop-runtime-package-plan/PROOF_INDEX.md` | stale proof wrapper | Parent desktop runtime package workers using proof routing | Proof namespace aligned to the actual plan folder/name on this branch | The doc is titled `Parent Client Runtime Distribution Proof Index` and routes to `output/parent-client-runtime-distribution-plan-proof/...`, so workers can prove the wrong plan namespace from the start. | move |

### Notes

- `docs/repo-audits/2026-06-17-structural-truth-audit.md` already flags the same broad-surface risks directly:
  - `OWN-01A`: `agent-protocol-domain` aggregates many plan surfaces and should own transport-only concerns.
  - `OWN-01B`: `parent-domain` contains many cross-plan proof/frontage surfaces and must not become a proof dumping ground.
  - `OWN-01C`: `portal-domain` and portal rendering can blur UI proof with source truth.
  - `OWN-01D`: `agent-core` is large and plan-dense and should not keep expanding as the default owner.
- No allowed-path file was elevated to `orphan` in this pass. Every row above still has a live consumer, even where the owner boundary is weak or stale.
- No allowed-path file was elevated to `pre-eventing shadow` in this pass. The closest runtime candidates inspected under `crates/agent-core/src/network_event_runtime.rs` and sibling runtime files are already wired directly to `ocentra_eventing`, so this slice did not mark them as parallel pre-eventing implementations without narrower-source inspection.

## Acceptance

- Parent-domain and portal-domain shadow exports are listed explicitly.
- Old proof wrappers that target wrong owner packages are listed.
- Pre-eventing or pre-runtime parallel implementations are identified before DRY extraction.
- Each action is one of: keep, convert to adapter, move, delete, or block pending owner decision.

## Failure conditions

- Treating old source existence as proof of ownership.
- Moving code without a current consumer map.
- Deleting code before ownership and proof replacement are clear.
