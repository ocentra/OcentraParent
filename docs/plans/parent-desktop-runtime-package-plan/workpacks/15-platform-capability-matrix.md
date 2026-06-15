# 15 Platform Capability Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `15 Platform Capability Matrix`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

The release-support proof now carries a deterministic platform matrix in both the
parent-domain read model and the release helper proof JSON. Rows split parent
desktop, parent mobile, child desktop, child Android, child iOS, relay, signing,
store, and support states without upgrading preview or scaffolded support into
production claims.

## Where We Want To Be

Matrix rows stay synchronized across package, parent shell, child agent, signing,
store, relay, and support states, and any future product-claim change requires a
matching feature-doc/checklist update.

## Requirement Checklist

- [ ] Split implemented, scaffold, unavailable, degraded, and manual-required.
- [ ] Include parent desktop/mobile and child-agent rows separately.
- [ ] Include signing/store/relay rows.
- [ ] Generate or test deterministic output.
- [ ] Sync D-owned feature/expectation/workpack docs when rows change; product checklist wording stays tracked in workpack 19 until the codex-a lock clears.

## Acceptance And Proof

Platform claim matrix is validated by
`npm run test:parent-desktop-release-support-proof` and by
`node --test scripts/test/parent-desktop-release-support-proof.test.mjs`.
`docs/product-capability-checklist.md` is still locked by codex-a, so workpack
19 carries the desired checklist row language for primary reconciliation.

## Parallel Ownership Notes

This matrix should align with A and B proof matrices without duplicating them.
