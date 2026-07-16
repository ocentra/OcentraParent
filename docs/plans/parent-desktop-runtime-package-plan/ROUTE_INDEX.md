# Parent Client Runtime Distribution Route Index

Historical folder path: `docs/plans/parent-desktop-runtime-package-plan/`.
Canonical plan scope: `parent-client-runtime-distribution-plan`.

## Read route

1. Read [AGENTS.md](AGENTS.md).
2. Read [PLAN_STATE.md](PLAN_STATE.md).
3. Read [NEXT_ACTIONS.md](NEXT_ACTIONS.md).
4. Read [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
5. Read [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when owner/proof family is unclear.
6. Open exactly one selected workpack.
7. Use [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) for selected commands/proof.
8. Use [PROOF_INDEX.md](PROOF_INDEX.md) only for proof validation or readiness claims.

## Owns

Parent client distribution proof for:

```text
parent web portal distribution
parent desktop shell/package
parent Android package
parent iOS package
parent local-service route bridge
signing/store/notarization matrix
update/rollback
launch smoke
setup handoff contract
proof/CI/release gate
```

## Boundary split

```text
apps/portal owns parent web source/projection surface.
portal-domain owns public portal contracts/projections when selected.
scripts/dev owns dev launch proof anchors.
scripts/release owns selected build/package proof helpers.
setup-install-provisioning-plan owns setup journey, install readiness, and first-run state.
child-agent-runtime-distribution-plan owns child runtime/package distribution.
device-trust-bootstrap-plan owns trusted-device bootstrap and local sealed trust.
portal-ux-household-surfaces-plan owns generic household UX shell.
account/payment/policy/remote/data-custody plans own their own product behavior.
```

## Handoff rule

Open a sibling plan only when the selected workpack names the exact handoff, expected proof, owner path, and no-claim boundary.

## No-claim rule

Do not claim parent client readiness from scaffold, launch smoke, package metadata, preview output, web build, unsigned artifact, unpublished store state, or CI success alone. Setup completion and child runtime readiness require their owning plans.
