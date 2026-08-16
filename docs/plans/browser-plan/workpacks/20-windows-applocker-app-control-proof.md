# 20 Windows AppLocker And App Control Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `20 Windows AppLocker And App Control Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

Docs correctly say unmanaged browser prevention needs OS application control,
not URL/tab inference. Current proof includes scoped process termination
guardrails, not production AppLocker/App Control deployment.

## Where We Want To Be

Windows can represent app-control readiness, audit-only, enforced, unavailable,
manual-required, and failed states before claiming unmanaged browser prevention.

## Scope

- AppLocker/App Control readiness state representation.
- Audit-only state representation.
- Enforced state representation without prevention claim upgrade.
- Policy creation/update/manual/failure status representation.
- Rule target identity by publisher/path/hash/package.
- Admin/permission requirement.
- Rollback/failure/audit event representation.
- Parent-visible manual setup and unavailable state.

## Touched Paths

- `packages/parent-domain/src/v0-8-browser-domain-adapter-proof.ts`
- `packages/parent-domain/tests/v0-8-browser-domain-adapter-proof.test.ts`
- `crates/agent-protocol/src/constants/v08_browser_domain_adapter_proof.rs`
- `crates/agent-protocol/src/enforcement_browser_domain_adapter_proof.rs`
- `crates/agent-protocol/src/enforcement_browser_domain_adapter_proof_tests.rs`
- `crates/agent-service/src/enforcement_browser_domain_adapter_app_control_proof_states.rs`
- `crates/agent-service/src/enforcement_browser_domain_adapter_proof_read_model.rs`
- `crates/agent-service/src/enforcement_browser_domain_adapter_proof_read_model_tests.rs`
- `crates/agent-service/src/main.rs`
- `scripts/test/v0-8-browser-domain-adapter-proof.mjs`
- `docs/plans/browser-plan/implementation-checklist.md`
- `docs/features/browser-web-control.md`
- `docs/expectations/browser-evidence.md`

## Tests And Proof

- Model tests for capability states.
- Rust protocol and service read-model parity tests.
- Proof harness evidence at
  `test-results/v0-8-browser-domain-adapter-proof/proof.json`.
- Real/manual Windows AppLocker/WDAC policy artifacts remain required before any
  claim upgrade.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/20-windows-applocker-app-control-proof/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: this workpack adds read-model/proof-state representation, not live AppLocker/WDAC event capture.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no unmanaged exact URL claim, no AppLocker/WDAC launch prevention claim, no policy creation/update/rollback claim, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

No production AppLocker/App Control claim until real Windows device artifacts
exist for edition/provider readiness, administrator permission, policy
create/update, audit-only and enforced modes, refresh, rollback, failure events,
and publisher/path/hash/package target identity.
