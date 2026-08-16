# 09 Timer Recovery And Rollback

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `09 Timer Recovery And Rollback`
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

Timer and recovery contracts exist. Product behavior needs consistent service
state, restart handling, rollback, and visible failure modes. The V0.8
browser/enforcement proof now covers create, restart-recovered, cancel,
recovery-needed, expired, rollback-completed, and rollback-unavailable states
with parent-visible next-check/failure state. Dedicated timer extend execution
remains manual-required rather than a claimed service command.

## Where We Want To Be

Temporary enforcement has a durable lifecycle: created, active, extended,
expired, cancelled, rollback requested, rollback completed, or recovery-needed.

## Requirement Checklist

- [x] Journal timer lifecycle events.
- [x] Recover active timers from durable state on restart.
- [x] Emit recovery-needed when state cannot be restored safely.
- [x] Show expiry and next-check state to parent surfaces.
- [x] Cover rollback success and rollback unavailable.

## Acceptance And Proof

Service tests cover timer create, expire, cancel, restart recover,
recovery-needed, and rollback outcomes. Current proof lives in
`npm run test --workspace @ocentra-parent/enforcement-domain -- v0-8-browser-enforcement-timer-recovery-proof`,
`node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs`,
`cargo test -p ocentra-parent-agent-core enforcement_timer_state`, and
`cargo test -p ocentra-parent-agent-service enforcement_timer`.
Supporting proof surfaces live in
`packages/enforcement-domain/tests/unit/v0-8-browser-enforcement-timer-recovery-proof.test.ts`,
`crates/agent-core/src/enforcement_timer_state_tests.rs`,
`crates/agent-service/src/enforcement_timer_tests.rs`, and
`scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs`.
Current artifacts live under
`test-results/v0-8-enforcement-timer-recovery-mvp/`,
`output/v0-8-enforcement-control-plan-proof/09-timer-recovery-and-rollback/`,
and
`docs/proof/v0-8-enforcement-control-plan/slice-02-timer-recovery-and-rollback.md`.

## Parallel Ownership Notes

Timers must stay in the child-device service path, never portal local state.
