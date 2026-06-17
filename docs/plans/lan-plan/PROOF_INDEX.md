# LAN Plan Proof Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Proof Index`
> Kind: canonical proof routing.
> Read when: a LAN slice needs exact proof paths.
> Stop rule: claim only files that exist on disk.
> Proves: proof-root routing and current artifact status only.
> Does not prove: implementation completion by itself.
> Proof rule: if a file is absent, mark the claim open/manual-required/not yet regenerated.

<!-- /agent-capsule -->

## Current Authoritative Proof Root

`Slice A` proof root:

```text
output/lan-plan-proof/00-plan-model-reconciliation/
```

Files present for `Slice A`:

- `00-source-snapshot.md`
- `01-lan-domain-validation.log`
- `02-plan-truth-sync.md`
- `03-missing-proof-inventory.md`

## Next Proof Roots

`B1` is not started yet. When it starts, regenerated workpack proof must point at:

```text
output/lan-plan-proof/<workpack-id>-<short-slug>/
```

If a proof script emits `test-results/.../proof.json`, the workpack proof pack must reference that file and must not imply the artifact exists until it has been regenerated on this branch/worktree.

## Proof Paths Explicitly Not Claimed By Slice A

These previously cited paths are absent on disk as of 2026-06-17 and are not current proof:

- `test-results/v0-9-lan-source-matrix-plan-completion/proof.json`
- `output/playwright/lan-source-matrix-plan-completion/devices-lan-source-matrix.png`
- `output/playwright/lan-source-matrix-plan-completion/activity-network-source-matrix.png`
- `output/playwright/lan-source-matrix-plan-completion/policy-network-target-binding.png`
- `output/playwright/lan-source-matrix-plan-completion/browser-proof.json`
- `output/lan-plan-proof/15-household-device-store/devices-identity-routing-proof.md`
- `output/lan-plan-proof/15-household-device-store/06-ui-snapshots/devices-identity-persisted.png`
- `output/lan-plan-proof/15-household-device-store/06-ui-snapshots/devices-update-gated.png`
- `docs/proof/lan-plan/PLAN_PROOF_MANIFEST.md`

## Proof Routing Rules

- Do not cite absent files as proof.
- Do not use `docs/proof/lan-plan/` as the active proof root for current LAN work.
- Use manual-required or open status whenever a physical/device/network artifact has not been regenerated yet.
