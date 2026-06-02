# 13 Desktop Launch Smoke

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

The desktop shell needs focused launch proof to support package claims.

## Where We Want To Be

Launch smoke proves the shell starts, handles service available/unavailable
states, and does not claim backend authority.

## Requirement Checklist

- [ ] Build or launch desktop shell where feasible.
- [ ] Check service available state.
- [ ] Check service unavailable/degraded state.
- [ ] Record commands and environment.
- [ ] Avoid interactive-only proof as the sole artifact.

## Acceptance And Proof

Focused smoke tests and reports show package launch behavior clearly.

## Parallel Ownership Notes

If GUI launch is not possible in CI, record local/manual proof requirements.
