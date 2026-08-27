# WP02 Policy Decision Evidence References

- checkedAt: `2026-06-17T02:50:22Z`
- branch: `codex/tracking-plan-full-continuation-a`
- commit: `1f192e52b931d3b2b8080f3e9479d37a94172958`
- result: `pass`

## Commands

- `node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs`
- `npm run lint:architecture -- --files packages/enforcement-domain/src/enforcement-policy-dispatch.ts packages/enforcement-domain/tests/unit/enforcement-policy-dispatch.test.ts packages/agent-protocol-domain/tests/unit/enforcement-policy-dispatch-adapter.test.ts crates/agent-protocol/src/constants/v08_enforcement_policy_dispatch.rs crates/agent-core/src/enforcement_policy_dispatch.rs crates/agent-core/src/enforcement_policy_dispatch_tests.rs crates/agent-service/src/enforcement_policy_dispatch_read_model.rs crates/agent-service/src/enforcement_policy_dispatch_read_model_tests.rs scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs docs/plans/v0-8-enforcement-control-plan/workpacks/02-policy-decision-evidence-references.md docs/plans/v0-8-enforcement-control-plan/WORKPACK_INDEX.md docs/plans/v0-8-enforcement-control-plan/PLAN_STATE.md docs/plans/v0-8-enforcement-control-plan/NEXT_ACTIONS.md docs/plans/v0-8-enforcement-control-plan/PROOF_INDEX.md docs/proof/v0-8-enforcement-control-plan/slice-06-policy-decision-evidence-references.md`

## Owning surfaces

- `packages/enforcement-domain/src/enforcement-policy-dispatch.ts`
- `packages/enforcement-domain/tests/unit/enforcement-policy-dispatch.test.ts`
- `packages/agent-protocol-domain/src/enforcement-policy-dispatch-adapter.ts`
- `packages/agent-protocol-domain/tests/unit/enforcement-policy-dispatch-adapter.test.ts`
- `crates/agent-protocol/src/constants/v08_enforcement_policy_dispatch.rs`
- `crates/agent-core/src/enforcement_policy_dispatch.rs`
- `crates/agent-core/src/enforcement_policy_dispatch_tests.rs`
- `crates/agent-service/src/enforcement_policy_dispatch_read_model.rs`
- `crates/agent-service/src/enforcement_policy_dispatch_read_model_tests.rs`
- `scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs`

## Covered proof

- the proof harness now runs against the actual `enforcement-domain` owner path instead of the broken `parent-domain` re-export shim and unrelated parent-domain workspace build
- action-capable rows require evidence references before they can remain dispatch-ready
- ask-parent stays explicit as a dry-run-only row with pending approval rather than being upgraded into adapter execution
- stale-policy-version, missing-source, wrong-device, missing-policy-decision, malformed-policy-decision, and missing-evidence negatives are all covered by the schema-backed TypeScript and Rust-core validation path
- the service-backed read model preserves parent-visible reason codes and source states across dispatch-ready, dry-run-only, report-only, manual-required, rejected, and scaffold rows
- proof JSON now reports 8 dispatch rows: 2 dispatch-ready, 1 dry-run-only, 1 report-only, 1 manual-required, and 3 rejected with ready/stale/missing/unavailable source separation
- the named feature docs already described the typed policy/evidence/ask-parent/no-claim boundaries this slice proves, so no feature-doc or product-checklist text change was required for this closure

## Remaining gaps

- broad installed-app blocking remains unproved/manual-required
- host network or domain blocking remains manual-required rather than dispatch-ready
- managed exact-URL enforcement, unmanaged browser exact evidence, notification delivery, tamper hardening, and mobile enforcement parity remain unclaimed
- later service API, portal consumption, and rollout/CI workpacks remain open even though the dispatch evidence chain is now proved

## Current Service-Command Boundary Status — 2026-08-27

The historical direct dispatcher tests were withdrawn in PR #709 because
crate-private dispatcher calls are not authenticated `/dev_ws` evidence. The
retained protocol, core, and read-model tests still cover typed validation and
projection, but authenticated
`run_agent_service` -> `ParentLocalBridgeAdmission` -> `/dev_ws` ->
handshake/revalidation -> `command_entry` service-command coverage is absent
and **manual-required**. This does not invalidate the direct contract proof
above; it prevents upgrading it to authenticated service-route proof.
