<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Portal UX Household Surfaces Test Proof Expectations

## Proof root

```text
output/portal-ux-household-surfaces-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal
npm run test:e2e --workspace @ocentra-parent/portal
npm run lint:architecture -- --files packages/portal-domain apps/portal docs/plans/portal-ux-household-surfaces-plan
```

## Required negative states

```text
loading/empty/error/degraded visible
manual-required visible
fake data not shown as real
UI does not own domain truth
UI does not execute device work
source/custody labels visible
browser console warnings handled or documented
```
