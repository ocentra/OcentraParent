# Policy Control Plane Plan State

Status: execution-grade architecture documented; implementation is in progress and proof remains open.

Current truth:

- This plan owns the cross-domain policy control contract: source of truth, lifecycle, schedule/time budget, conflict precedence, domain compiler boundaries, delivery/ack/audit, ask-parent overrides, and policy event model.
- Existing domain plans own runtime effects; this plan owns the parent policy control plane contract and proof route.
- Parent-facing UI is specified here and in the portal plan, but no plan should treat UI state as policy truth.
- Proof inventory and route gate docs now exist; they are required before any DONE or PR_READY claim.
- `policy-control-core` source/compiler/conflict/preview/request owner modules now compile through their real `policy_source` / `policy_authority` boundaries, and the crate-owned unit plus version-skew suites execute green in this checkout.
- `policy-control-core` source registration now also has an explicit actor-authority gate for WP01, so wrong-household and revoked-parent writes fail before a parent policy source document can replace source truth.
- `policy-control-core` source compatibility assessment now also covers equal, migration-required, and unsupported schema/version combinations through the real `policy_source` unit bucket, and the slice-01 proof artifact now records that version-skew boundary in `docs/proof/policy-control-plane-plan/slice-01-source-of-truth.md`.
- `policy-control-core` source lifecycle now also has an explicit WP01 supersede transition, so a source document cannot move to `superseded` without a strictly newer replacement policy version and a fresh audit reference proving the supersede event.
- `policy-control-core` source lifecycle now also owns a typed rollback reference plus rollback transition guard, so a source document cannot move to `rolledBack` unless it carries a prior-version reference and a fresh rollback audit reference.
- `policy-control-core` compiler lifecycle now also rejects `draft` and `preview` source documents, so domain compiler artifacts cannot be produced from pre-confirmation portal state and only confirmed-or-later source documents are treated as release-candidate inputs.
- `policy-control-core` source and compiler artifacts now also persist `audit_reference_ids`, `superseded_by_policy_version`, and `rollback_ref`, and the downstream child-policy, child-notification, child-runtime policy-control, and parent-runtime policy-control tests execute green against those persisted lifecycle refs in this checkout.
- `policy-control-core` delivery records now also preserve source-side compiler metadata separately from delivery-transition state, and matching unit/version-skew plus downstream handoff tests were added for that queue boundary, so queued policy delivery no longer silently drops source audit refs or supersede/rollback provenance before the first child-policy and child-runtime consumer seams.
- `policy-control-core` compiler artifacts now also own an explicit WP03 support matrix plus per-rule capability state, so supported/manual-required/unsupported ownership is first-class compiler data instead of only hard-coded domain-target status derivation.
- `policy-control-core` conflict and preview seams now also surface timezone-mismatched schedules as explicit blocking/manualRequired findings instead of silently treating them as non-overlapping, and conflict records preserve the source document version, audit refs, and rollback refs required by the workpack contract.
- `policy-domain` now also owns an explicit WP07 time-budget contract: schedules carry reset/carryover/grace/effective-window/clock-source/offline-recovery semantics, schedule boundaries can surface runtime budget state plus bonus-time expiry/offline-recovery state, and bonus-time approval requests now require that schedule budget context instead of raw minutes alone.
- WP07 proof artifacts now exist under `docs/proof/policy-control-plane-plan/07-*.md` for timezone, DST, time-budget reset, conflict precedence, and offline timer recovery, and they are backed by focused Rust unit/version-skew validation plus the `policy-domain` package test run in this checkout.
- WP02 proof artifacts now exist under `docs/proof/policy-control-plane-plan/02-*.md` for authoring, conflict visibility, unsupported targets, no-fake-green preview, and assistant-draft preview-only behavior, and they are backed by the focused preview and policy-domain validation already executed in this checkout.
- `policy-domain` now also owns a shared WP03 compiler-contract module for compiled artifact ids, domains, capability states, support matrices, rule statuses, delivery targets, no-claim labels, source lifecycle refs, and rollback/supersede metadata, with unit tests written in `packages/policy-domain/tests/unit/policy-compiler.test.ts` instead of leaving TypeScript compiler seams to invent those shapes ad hoc across app-game, browser, and tracking domains.
- `app-game-domain` now also consumes the shared WP03 capability-state owner contract in its policy-target compiler seam, so app/game compile requests and preview fixtures use `PolicyCompilerCapabilityStateSchema` / `PolicyCompilerCapabilityState`, and manual-required or unsupported capability refs can no longer drift into `dry-run-ready` compiled output.
- `tracking-domain` now also consumes the shared WP03 compiled-artifact contract at the runtime-proof consumer boundary, so tracking compiler requests reject non-tracking artifacts, source-policy-version mismatches, and missing source-rule coverage before the local runtime-proof seam can treat a free-floating tracking rule as sufficient input.
- `browser-domain` now also emits canonical `PolicyCompilerCapabilityState` rows in the browser control coverage-matrix compiler seam, so the shared supported/manual-required/unsupported support vocabulary is now present in the browser compiler support matrix instead of only local browser coverage labels.
- `browser-domain` browser-game and social policy compiler seams now also emit canonical `PolicyCompilerCapabilityState` rows on compiled decision candidates, so the remaining browser-domain compiler surfaces now speak the shared supported/manual-required/unsupported vocabulary instead of only local compiler-mode wording.
- `screen-domain` now resolves the WP03 consumer seam through explicit `ai-domain`, `enforcement-domain`, and `notification-domain` exports and the touched screen and ai runtime code passes focused build/test/architecture validation without a full repo-wide `npm validate` run.
- WP03 proof artifacts now exist under `docs/proof/policy-control-plane-plan/03-*.md` and are backed by the focused Rust and TS validation already executed in this checkout.
- WP08 proof artifacts now exist under `docs/proof/policy-control-plane-plan/08-*.md`, and the policy-event family registry, idempotency, replay ordering, rollback linkage, and redaction slices are backed by focused Rust and TS validation in this checkout.

Open gaps:

- Implementation of the source document, authoring/preview, compilers, delivery/ack, overrides, event model, and route-gated proof packs.
- Focused architecture validation now passes on the `policy-control-core` source/compiler/conflict/preview/request/delivery seams after moving runtime strings to the `agent-protocol` owner constants, but the broader workpack implementation and proof closure remain open.
- Source-of-truth closure still needs broader lifecycle and custody coverage beyond the new compatibility report, authority, active-after-ack, supersede, and rollback guards.
- WP03 still needs the remaining downstream TS adoption of the new explicit support-matrix / capability-state contract, with the next gaps now narrowed to broader parent-surface consumers and any other consumer boundaries outside the owner package that still duplicate or bypass the shared compiler artifact contract.
- WP07 now has proof artifacts plus focused Rust and TS validation for the current schedule/time-budget/conflict slices; the remaining follow-on is broader downstream parent/runtime consumer coverage and any additional schedule-budget parity outside those proofed seams.
- The newest WP03 support-matrix, delivery-provenance, app-game downstream capability-state adoption, tracking shared-artifact consumer additions, and browser-domain compiler seam fixes are recorded here as code, tests, proof artifacts, and focused validation, while the broader downstream consumer additions remain open until their own focused reruns land.
- `packages/policy-domain` now type-checks, builds, and tests cleanly in this checkout after the repo-root `tsconfig.base.json` landed, so the shared WP03 owner contract is runnable again.
- WP08 proof closure is complete for the owner-contract slice, but the broader parent policy control-plane plan still has source, authoring/preview, compilers, delivery/ack, overrides, and route-gated proof work left open.
- `packages/app-game-domain/tests/unit/app-game-policy-target-compiler.test.ts` now passes directly, but the full `packages/app-game-domain` build still hits unrelated enforcement/production dependency and type-debt outside this plan.
- `packages/parent-domain/tests/unit/browser-policy-compiler.test.ts` now passes with the proof-chain skipped, so the parent-surface WP03 wrapper seam is validated without forcing the broken package build chain.
- Closed proof artifacts for each workpack.
- Route sync with feature and plan indexes while work is executed.

## Execution boundary

- Use `WORKPACK_INDEX.md` and `TEST_PROOF_EXPECTATIONS.md` to choose work.
- Do not mark this plan complete from checklist deltas or architecture docs alone.
- Update this file when the proof or implementation state changes.
