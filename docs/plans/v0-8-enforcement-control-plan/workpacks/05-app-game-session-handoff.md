# 05 App And Game Session Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `05 App And Game Session Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md),
[folder README](../README.md),
[app-game-control feature](../../features/app-game-control.md), and
[enforcement expectation](../../expectations/enforcement.md).

## Purpose

Consume stored app/game session evidence into enforcement decisions without
letting portal timers, AI guesses, or stale process observations pretend to be
session truth.

## Central schema boundary

```text
schema-domain owns public session-ref, action-ref, reason, and result schemas when they cross package/crate/protocol boundaries.
app-game-plan owns session summaries, launcher/app/game identity, and duration facts.
policy-control-plane-plan owns upstream policy authority and time-budget intent.
v0-8-enforcement-control-plan owns the action-state handoff, audit, and no-claim boundary.
```

## Source Inputs

- `../v0-8-enforcement-control-20-step-plan.md`
- `../v0-8-enforcement-control-test-blueprint.md`
- `../../features/app-game-control.md`
- `../../expectations/enforcement.md`

## Target State

Time budgets and app/game actions are grounded in real session summaries,
foreground/running time, target identity, and confidence state.

## Required proof fields

```text
canonical_schema_owner_state
policy_decision_ref_state
session_summary_ref_state
target_identity_state
launcher_game_app_kind_state
foreground_background_state
duration_state
ask_parent_state
evidence_freshness_state
parent_visible_state
no_ui_timer_authority
no_ai_authority_claim
no_claim
```

## Tests And Proof

Proof root: `output/v0-8-enforcement-control-plan-proof/05-app-game-session-handoff/`

Focused validation should record:

- `cargo test -p ocentra-parent-agent-protocol --test contract app_game_timer_parent_surface`
- `cargo test -p ocentra-parent-agent-core enforcement`
- selected app/game session proof/read-model tests
- selected portal tests only when this slice updates visible session-backed state

The former `@ocentra-parent/enforcement-domain` workspace command is obsolete:
that package is not present in the current workspace. The Rust protocol,
core, and service tests above are the current owners for this handoff.

## AI Worker Checklist

- [ ] Require app/game session summary refs for app/game time-limit decisions.
- [ ] Distinguish launcher, game, app, unknown, foreground, and background.
- [ ] Preserve unknown or ask-parent state when identity is weak.
- [ ] Record running/foreground duration used by policy.
- [ ] Show parent-facing rule/action/result details.

## Where We Are

App/game session evidence and read models exist in progress form. Enforcement
must consume stored session evidence rather than portal timers or AI guesses.

## Negative Cases

- missing or stale session summaries must block dispatch-ready state
- weak target identity must stay unknown, ask-parent, or manual-required
- portal timer values must not override stored session duration truth
- AI classification must not upgrade a weak session into action authority
- wrong-device or wrong-session refs must reject rather than execute

## Manual-Required Gaps

- Broad app blocking and non-session-based action paths remain manual-required.
- Mobile parity and platform-specific app/game adapter behavior remain unproved.
- Notification delivery and surface polish remain separate follow-on slices.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch recorded.
- [ ] Validation commands and results recorded in `16-validation-commands.log`.
- [ ] Proof artifacts under `output/v0-8-enforcement-control-plan-proof/05-app-game-session-handoff/`.
- [ ] Known gaps/manual-required states listed here and in the proof note.
