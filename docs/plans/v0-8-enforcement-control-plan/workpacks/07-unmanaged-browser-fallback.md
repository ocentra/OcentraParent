# 07 Unmanaged Browser Fallback

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `07 Unmanaged Browser Fallback`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Unmanaged browser states can be shown as possible bypass and process-only
fallback, not exact page proof. The V0.8 browser/enforcement timer recovery
proof adds schema-backed unmanaged fallback rows for process identity required,
report-only, warn-child, parent-review, terminate-process, relaunch-managed
manual-required, degraded, and unavailable states while rejecting exact
URL/tab/title/content claim upgrades.

## Where We Want To Be

Unmanaged browser handling is honest: detect, warn, report possible bypass, or
perform scoped process action where proved.

## Requirement Checklist

- [x] Never attach exact URL, title, tab, or content to unmanaged process state.
- [x] Require explicit process identity for process fallback.
- [x] Label possible bypass and manual-required exact URL gaps.
- [x] Keep managed/unmanaged states separate in UI and proof JSON.
- [x] Add tests for unsupported and degraded states.

## Acceptance And Proof

Unmanaged browser tests fail if exact URL action is inferred from process or
network metadata. Current proof runs:
`npm run test --workspace @ocentra-parent/enforcement-domain -- v0-8-browser-enforcement-timer-recovery-proof`
and `node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`.
Supporting proof surfaces live in
`packages/enforcement-domain/tests/unit/v0-8-browser-enforcement-timer-recovery-proof.test.ts`
and `scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`.
Current artifacts live under
`test-results/windows-managed-unmanaged-browser-enforcement-proof/`,
`output/v0-8-enforcement-control-plan-proof/07-unmanaged-browser-fallback/`,
and
`docs/proof/v0-8-enforcement-control-plan/slice-01-unmanaged-browser-fallback.md`.
Latest evidence rows include process-identity-required rejection, report-only,
warn-child, parent-review, terminate-process, relaunch-managed manual-required,
degraded, unavailable, and exact URL/tab/title/content not-claimed states.

## Parallel Ownership Notes

This workpack protects product truth. It should be reviewed anytime browser
copy, policy targets, or proof labels change.
