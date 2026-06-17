# WP04 Ownership Drift Map

## Objective

Find code and tests that landed in broad frontage packages or wrong crates instead of the narrow owner.

## Scope

Inspect broad aggregators first:

- `packages/parent-domain`
- `packages/portal-domain`
- `packages/agent-protocol-domain`
- `crates/agent-core`
- `crates/agent-protocol`

Then compare against narrow owner packages/crates:

- `packages/family-domain`
- `packages/setup-domain`
- `packages/data-custody-domain`
- `packages/policy-domain`
- `packages/tracking-domain`
- `packages/browser-domain`
- `packages/network-domain`
- `packages/app-game-domain`
- focused Rust crates such as `tracking-core`, `network-core`, `storage-custody-core`, `policy-control-core`, `child-runtime`, and `ocentra-eventing`.

## Classification

| Class | Meaning |
| --- | --- |
| owner | Source owns product/domain behavior. |
| adapter | Source translates between owner and caller. |
| frontage | Parent/portal/read-model display surface only. |
| misplaced | Source should move or be rewritten around an owner package/crate. |
| duplicate | Same behavior exists in more than one owner. |

## Output table

| File/surface | Current location | Class | Preferred owner | Reason | Action |
| --- | --- | --- | --- | --- | --- |
| `packages/parent-domain/src/app-game-*.ts` | broad parent frontage | frontage | `packages/app-game-domain/src/**` | Bounded inventory found 241 `parent-domain` references to `app-game-domain`, and sampled files are one-line `export *` shells rather than narrow-domain owners. | keep as frontage only; do not count these files as app-game ownership proof |
| `packages/parent-domain/src/tracking-*.ts` | broad parent frontage | frontage | `packages/tracking-domain/src/**` | Bounded inventory found 91 `parent-domain` references to `tracking-domain`; sampled files such as `tracking-runtime.ts` are pure re-export shells and the proof-heavy surface stays broad only by path. | keep as frontage only; treat `tracking-domain` as the owner for plan completion claims |
| `packages/parent-domain/src/social-*.ts` | broad parent frontage | frontage | `packages/browser-domain/src/**` | Bounded inventory found 168 `parent-domain` references to `browser-domain`; sampled files such as `social-parent-approval.ts` are pure re-exports, and wrapper-style files such as `social-policy-compiler.ts` only alias browser-domain exports back out through parent-domain. | keep as frontage only; do not treat parent-domain social files as browser/runtime owners |
| `packages/parent-domain/src/network-*.ts` | broad parent frontage | frontage | `packages/network-domain/src/**` | Sampled file `network-flow.ts` is a direct re-export of `@ocentra-parent/network-domain/network-flow`, so the parent path is a caller convenience surface, not the narrow owner. | keep as frontage only |
| `packages/parent-domain/src/policy.ts` | broad parent frontage | frontage | `packages/policy-domain/src/policy.ts` | Sampled file `policy.ts` is a direct re-export of `@ocentra-parent/policy-domain/policy`, so policy authority does not live in parent-domain. | keep as frontage only |
| `packages/parent-domain/src/reference-primitives.ts` and `references.ts` | broad parent frontage | frontage | `packages/family-domain/src/reference-primitives.ts` and `packages/family-domain/src/references.ts` | These files re-export family-domain references/primitives into parent-domain; the sampled `family-domain` imports elsewhere in parent-domain are consumer imports, not evidence that parent-domain owns family authority. | keep as frontage only |
| `packages/parent-domain/src/local-ai-runtime.ts` | broad parent frontage | frontage | `packages/ai-domain/src/local-ai-runtime.ts` (supported by completed WP07 evidence) | Sampled file is a pure re-export to `@ocentra-parent/ai-domain/local-ai-runtime`, but direct `ai-domain` comparison was outside the current WP04 packet and comes from the completed WP07 evidence input. | keep as frontage only; mark owner call provisional until WP03 final wording lands |
| `packages/portal-domain/src/contracts.ts` | broad portal aggregator | frontage | narrow portal-domain files under `packages/portal-domain/src/**`; runtime owners remain outside portal-domain | The file is a large export surface for panels, commands, details, diagnostics, layouts, and route helpers. It is a UI composition/frontage surface, not a runtime-truth owner. | keep as frontage only; do not accept portal-domain contracts as runtime ownership proof |
| `packages/agent-protocol-domain/src/primitives.ts` | protocol aggregator edge | frontage | `packages/event-domain/src/primitives.ts` | Sampled file is a direct re-export of `@ocentra-parent/event-domain/primitives`, so protocol-domain is not the primitive owner at this path. | keep as frontage only |
| `packages/agent-protocol-domain/src/activity-surface-adapter.ts` | protocol adapter surface | adapter | `packages/agent-protocol-domain/src/activity-surface-adapter.ts` | Sampled file performs explicit translation between activity-domain read models and protocol envelopes and declares a service/UI spine. This is an adapter seam, not misplaced domain ownership. | keep as adapter only; do not treat it as product-domain authority |
| `packages/agent-protocol-domain/src/contracts.ts` | broad protocol aggregator | frontage | narrow protocol-domain files plus focused domain owners for product semantics | The file imports and re-exports a very wide transport/read-model surface across LAN, browser, app-game, activity, and protocol defaults. It is a broad aggregation boundary that can hide product ownership if treated as canonical. | keep as frontage only; mark product-logic boundary calls provisional until WP03 architecture wording is finalized |
| `crates/agent-core/src/lib.rs` | broad runtime crate root | frontage | narrow `agent-core` modules for composition and focused crates such as `child-runtime`, `tracking-core`, `network-core`, and `ocentra-eventing` for reusable engines | Sampled crate root re-exports dozens of runtime helpers, bridges, event runtimes, and capture surfaces, which makes the root path look like the owner for behavior that should be claimed at narrower module or focused-crate boundaries. | keep as crate-root frontage only; do not claim runtime ownership from `agent-core::lib` alone |
| `crates/agent-core/src/*_event_runtime*.rs` and `screen_household_mesh_runtime*.rs` | local runtime composition layer | adapter | same files for composition; reusable event machinery belongs in focused crates such as `crates/ocentra-eventing/src/**` where behavior is shared | The `agent-core` module inventory shows these files as composition/runtime seams, and WP07 already established that sampled runtime-shadow candidates are wired to `ocentra_eventing` rather than acting as parallel pre-eventing owners. | keep as adapter/composition only; do not count them as generic shared-engine owners |
| `crates/agent-protocol/src/lib.rs` | broad transport crate root | frontage | narrow `agent-protocol` modules and focused package/crate owners for product semantics | Sampled crate root re-exports nearly the whole transport/product-adjacent module surface, which hides whether a claim belongs to protocol shape, a read model, or a product-domain owner. | keep as crate-root frontage only; require narrower module paths for ownership claims |

### Notes

- Within the current bounded packet, the dominant ownership-drift shape is broad frontage rather than confirmed duplicate implementations. Most sampled `parent-domain` rows are alias or re-export shells over narrow owner packages.
- Within the current bounded packet, no source-move recommendation is strong enough to mark a row `misplaced` without widening into full implementation trees. The actionable truth here is to stop counting broad aggregator paths as ownership proof.
- Rows that depend on final architecture-policy wording remain provisional where a broad export surface could later be classified as policy-blocked frontage versus a required migration seam.

## Starting rules

- `parent-domain` should not become child runtime authority.
- `portal-domain` and `apps/portal` should not own runtime truth.
- `agent-protocol-domain` and `agent-protocol` should own transport/protocol shapes, not product logic.
- `agent-core` should own runtime composition/adapters; reusable engines should live in focused crates where practical.

## Acceptance

- Move candidates are listed before any source move.
- Adapter/frontage files are not counted as domain completion.
- Each plan report can be checked against the owner map.

## Failure conditions

- Moving source just to fix compile without preserving owner truth.
- Creating new broad frontage files for narrow-domain behavior.
- Treating portal screenshots as runtime ownership proof.
