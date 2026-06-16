<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `Parent Client Runtime Distribution Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: package/runtime readiness without matching artifacts.

<!-- /agent-capsule -->

# Parent Client Runtime Distribution Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

If a required package/test path does not exist yet, write a blocker artifact and leave the checklist row open.

## Common command families

```bash
npm run build --workspace @ocentra-parent/portal
npm run test --workspace @ocentra-parent/portal
npm run test:e2e --workspace @ocentra-parent/portal
npm run test:parent-mobile-shell-runtime-proof
npm run test:parent-mobile-package-source-artifact-proof
npm run test:parent-mobile-service-bridge
npm run test:parent-mobile-controller-observer-handoff
npm run test:parent-desktop-release-support-proof
npm run lint:architecture -- --files apps/portal packages/portal-domain scripts/release docs/plans/parent-desktop-runtime-package-plan
```

## Expected proof focus by workpack

| Workpack | Expected proof focus |
| --- | --- |
| WP01 | canonical parent-client scope and setup handoff boundary |
| WP02 | parent web build, route, auth/cache/env separation, no child-agent execution claim |
| WP03 | desktop shell/package, local service bridge, launch smoke, no product-readiness overclaim |
| WP04 | parent Android package/build/install state and manual-required/store blockers |
| WP05 | parent iOS package/build/install state and manual-required/store blockers |
| WP06 | parent client route bridge and local-service boundary without setup-complete claim |
| WP07 | signing/store/notarization matrix by artifact/platform |
| WP08 | update channel, rollback path, checksum, SBOM proof |
| WP09 | launch smoke matrix by artifact/platform and manual-required gaps |
| WP10 | setup handoff request/response contract only |
| WP11 | proof/CI/release gate and product-status wording |

## Required negative states

```text
web build is not production account portal readiness
launch smoke is not desktop product readiness
mobile scaffold is not mobile platform support
installer/package artifact is not setup complete
route bridge is not child-agent execution authority
unsigned/unnotarized/unpublished artifacts remain manual-required
update channel without rollback/checksum/SBOM proof is blocked
```

## Proof storage

Proof artifacts live under:

```text
output/parent-client-runtime-distribution-plan-proof/<workpack-id>/
```

## Failure conditions

- Do not mark DONE or PR_READY until code, tests, validation, and proof are complete for the selected slice.
- Do not store proof inventories inside this plan folder.
- Do not claim child agent runtime distribution from parent client packaging work.
