# WP51 - Policy Evaluator Runtime Breadth

## Scope

Build the parent-domain runtime helper that turns stored app/game
time-budget policy, session, schedule, bonus-time, timer, and audit inputs into
schema-validated runtime decisions.

This workpack proves the evaluator can derive dry-run time-limit, warn-only,
ask-parent, manual-required, and approved-bonus observe decisions without
creating a platform adapter or service persistence claim.

It does not add a service WebSocket command, portal policy authoring/status UI,
notification delivery, child request runtime, adapter execution, broad
blocking, or platform support.

## Implementation

- Add `app-game-time-budget-policy-runtime.ts` in `packages/parent-domain`.
- Reuse the existing app/game time-budget policy rules for target matching,
  counted/excluded session refs, duration source handling, budget math, bonus
  approval state, and schema validation.
- Keep exceeded-budget actions constrained to dry-run, warn-only, ask-parent,
  or manual-required outcomes.
- Keep adapter handoff disabled except for the existing dry-run/manual-required
  representation; no platform execution path is introduced.
- Preserve timer refs only for dry-run time-limit decisions.

## Proof

- `cmd /c npm run build --workspace @ocentra-parent/parent-domain`
- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-time-budget-policy-runtime`
- `node scripts/test/app-game-policy-evaluator-runtime-proof.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

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

`docs/product-capability-checklist.md` is intentionally unchanged because
primary owns central checklist edits during the merge wave. WP51 moves the
time-budget evaluator from contract shape toward deterministic parent-domain
runtime decision construction, but product status should not move until service
persistence/WebSocket evaluation, portal authoring/status UI, notification and
child request runtime, adapter execution, broad blocking, and platform proof are
complete.
