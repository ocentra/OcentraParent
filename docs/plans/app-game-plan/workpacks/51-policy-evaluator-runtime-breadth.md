# WP51 - Policy Evaluator Runtime Breadth

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP51 - Policy Evaluator Runtime Breadth`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Build the Rust-owned runtime evaluator that turns stored app/game
time-budget policy, session, schedule, bonus-time, timer, and audit inputs into
typed runtime decisions.

This workpack proves the evaluator can derive dry-run time-limit, warn-only,
ask-parent, manual-required, and approved-bonus observe decisions without
creating a platform adapter or service persistence claim.

It does not add a service WebSocket command, portal policy authoring/status UI,
notification delivery, child request runtime, adapter execution, broad
blocking, or platform support.

## Implementation

- Add the evaluator boundary in `ocentra-app-game-core`.
- Reuse the existing app/game time-budget policy rules for target matching,
  counted/excluded session refs, duration source handling, budget math, bonus
  approval state, and schema validation.
- Keep exceeded-budget actions constrained to dry-run, warn-only, ask-parent,
  or manual-required outcomes.
- Keep adapter handoff disabled except for the existing dry-run/manual-required
  representation; no platform execution path is introduced.
- Preserve timer refs only for dry-run time-limit decisions.

## Current Status - Phase 1 Open

The 2026-08-15 code audit found preview/readiness projections but no current
evaluator spanning stored time-budget policy, sessions, schedules, bonus time,
timers, and audit refs. The former parent-domain implementation owner is
removed. WP19 now supplies the Rust compiler and WP49 supplies category/risk
routing, so this workpack is the next authorized Rust-owned Phase 1 slice.

Phase 1 requires the evaluator code and checked-in breadth/negative tests.
Phase 2 focused execution/Enforcer and Phase 3 retained proof remain later
gates.

## Expected Focused Validation

- `cargo clippy -p ocentra-app-game-core --all-targets -- -D warnings`
- `cargo test -p ocentra-app-game-core --test contract policy_evaluator_runtime`
- focused Enforcer routing for the exact Rust source and contract test files
- `npm run hub:guard`

Proof artifacts live in:

```text
output/app-game-plan-proof/51-policy-evaluator-runtime-breadth
```

## No-Claim Boundaries

- Runtime helper output is parent-domain proof, not a service command.
- No portal budget authoring, policy status UI, or child-facing notification
  runtime is added.
- Dry-run timer state does not execute a platform adapter.
- Ask-parent and manual-required outcomes do not dispatch adapters.
- Broad app/game blocking and platform support remain unproved.

## Product Doc Decision

`docs/product-capability-checklist.md` remains unchanged. WP51 moves the
time-budget evaluator toward deterministic Rust runtime decision construction,
but product status should not move until service
persistence/WebSocket evaluation, portal authoring/status UI, notification and
child request runtime, adapter execution, broad blocking, and platform proof are
complete.
