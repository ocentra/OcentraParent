# Policy Control Plane Plan State

Status: execution-grade architecture documented; implementation is in progress and proof remains open.

Current truth:

- This plan owns the cross-domain policy control contract: source of truth, lifecycle, schedule/time budget, conflict precedence, domain compiler boundaries, delivery/ack/audit, ask-parent overrides, and policy event model.
- Existing domain plans own runtime effects; this plan owns the parent policy control plane contract and proof route.
- Parent-facing UI is specified here and in the portal plan, but no plan should treat UI state as policy truth.
- Proof inventory and route gate docs now exist; they are required before any DONE or PR_READY claim.
- `policy-control-core` source/compiler/conflict/preview/request owner modules now compile through their real `policy_source` / `policy_authority` boundaries, and the crate-owned unit suite executes green in this checkout; `version_skew` still has two failing assertions in `tests/version-skew/policy_compiler.rs` that need the codex-b-owned fixture update.
- `policy-control-core` source registration now also has an explicit actor-authority gate for WP01, so wrong-household and revoked-parent writes fail before a parent policy source document can replace source truth.
- `policy-control-core` source lifecycle now also has an explicit WP01 supersede transition, so a source document cannot move to `superseded` without a strictly newer replacement policy version and a fresh audit reference proving the supersede event.
- `policy-control-core` source lifecycle now also owns a typed rollback reference plus rollback transition guard, so a source document cannot move to `rolledBack` unless it carries a prior-version reference and a fresh rollback audit reference.
- `policy-control-core` compiler lifecycle now also rejects `draft` and `preview` source documents, so domain compiler artifacts cannot be produced from pre-confirmation portal state and only confirmed-or-later source documents are treated as release-candidate inputs.
- `policy-control-core` source and compiler artifacts now also persist `audit_reference_ids`, `superseded_by_policy_version`, and `rollback_ref`, and the downstream child-policy, child-notification, child-runtime policy-control, and parent-runtime policy-control tests execute green against those persisted lifecycle refs in this checkout.
- `policy-control-core` delivery records now also preserve source-side compiler metadata separately from delivery-transition state, and matching unit/version-skew plus downstream handoff tests were added for that queue boundary, so queued policy delivery no longer silently drops source audit refs or supersede/rollback provenance before the first child-policy and child-runtime consumer seams.
- `policy-control-core` compiler artifacts now also own an explicit WP03 support matrix plus per-rule capability state, so supported/manual-required/unsupported ownership is first-class compiler data instead of only hard-coded domain-target status derivation.
- `policy-control-core` conflict and preview seams now also surface timezone-mismatched schedules as explicit blocking/manualRequired findings instead of silently treating them as non-overlapping, and conflict records preserve the source document version, audit refs, and rollback refs required by the workpack contract.
- `policy-domain` now also owns an explicit WP07 time-budget contract: schedules carry reset/carryover/grace/effective-window/clock-source/offline-recovery semantics, schedule boundaries can surface runtime budget state plus bonus-time expiry/offline-recovery state, and bonus-time approval requests now require that schedule budget context instead of raw minutes alone.
- `policy-domain` now also owns a shared WP03 compiler-contract module for compiled artifact ids, domains, capability states, support matrices, rule statuses, delivery targets, no-claim labels, source lifecycle refs, and rollback/supersede metadata, with unit tests written in `packages/policy-domain/tests/unit/policy-compiler.test.ts` instead of leaving TypeScript compiler seams to invent those shapes ad hoc across app-game, browser, and tracking domains.
- `app-game-domain` now also consumes the shared WP03 capability-state owner contract in its policy-target compiler seam, so app/game compile requests and preview fixtures use `PolicyCompilerCapabilityStateSchema` / `PolicyCompilerCapabilityState`, and manual-required or unsupported capability refs can no longer drift into `dry-run-ready` compiled output.
- `tracking-domain` now also consumes the shared WP03 compiled-artifact contract at the runtime-proof consumer boundary, so tracking compiler requests reject non-tracking artifacts, source-policy-version mismatches, and missing source-rule coverage before the local runtime-proof seam can treat a free-floating tracking rule as sufficient input.
- WP01 proof docs now exist under `docs/proof/policy-control-plane-plan/01-*.md`, but the workpack remains open until the codex-b-owned `version_skew` assertions are corrected and revalidated.

Open gaps:

- Implementation of the source document, authoring/preview, compilers, delivery/ack, overrides, event model, and route-gated proof packs.
- Focused architecture validation now passes on the `policy-control-core` source/compiler/conflict/preview/request/delivery seams after moving runtime strings to the `agent-protocol` owner constants, but the broader workpack implementation and proof closure remain open.
- Source-of-truth closure still needs broader lifecycle coverage beyond the new authority, active-after-ack, supersede, and rollback guards.
- WP03 still needs the remaining downstream TS adoption of the new explicit support-matrix / capability-state contract, with the next gaps now narrowed to the browser-domain compiler seams, broader parent-surface consumers, and any other consumer boundaries outside the owner package that still duplicate or bypass the shared compiler artifact contract.
- WP07 still needs Rust/source-compiler parity for the new time-budget reset/carryover, bonus-expiry, and offline-timer-recovery semantics beyond the new `policy-domain` owner contract and timezone-boundary conflict handling.
- WP01 still has the two failing `version_skew` assertions as the immediate blocker for proof closure, even though the architecture gate and unit suite now pass locally.
- The newest WP03 support-matrix, delivery-provenance, app-game downstream capability-state adoption, and tracking shared-artifact consumer additions are recorded here as code plus tests written only; this file does not claim focused cargo or npm execution for that incremental slice yet.
- Package-local `npm run type-check --workspace @ocentra-parent/policy-domain` and `npm run test --workspace @ocentra-parent/policy-domain` are currently blocked in this checkout before execution because `packages/policy-domain/tsconfig.json` extends a missing repo-root `tsconfig.base.json`.
- Focused `packages/app-game-domain` Vitest execution is also still blocked in this checkout because `@ocentra-parent/policy-domain/policy-compiler` cannot resolve without a built `packages/policy-domain/dist/policy-compiler.js`, and building that owner package is currently blocked by the same missing `tsconfig.base.json` plus pre-existing owner-package TypeScript errors.
- Closed proof artifacts for each workpack.
- Route sync with feature and plan indexes while work is executed.

## Execution boundary

- Use `WORKPACK_INDEX.md` and `TEST_PROOF_EXPECTATIONS.md` to choose work.
- Do not mark this plan complete from checklist deltas or architecture docs alone.
- Update this file when the proof or implementation state changes.
