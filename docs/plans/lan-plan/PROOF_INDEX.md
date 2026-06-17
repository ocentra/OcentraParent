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

## Current Authoritative Proof Roots

`Slice A` proof root:

```text
output/lan-plan-proof/00-plan-model-reconciliation/
```

Files present for `Slice A`:

- `00-source-snapshot.md`
- `01-lan-domain-validation.log`
- `02-plan-truth-sync.md`
- `03-missing-proof-inventory.md`

`B1` proof root:

```text
output/lan-plan-proof/01-lan-b1-proof-regeneration/
```

Files present for `B1`:

- `01-lan-source-matrix-plan-completion-proof.json`
- `02-lan-signed-discovery-relay-spine-proof.json`
- `03-production-discovery-household-proof.json`
- `04-household-lan-proof-readiness.json`

`B1` proves local mechanical LAN-domain proof only. It does not claim portal
screenshots, service/runtime-backed proof, physical household readiness, real
signed child hello/heartbeat artifacts, or cloud relay implementation.

`B2` proof root:

```text
output/lan-plan-proof/02-lan-b2-test-truth-repair/
```

Files present for `B2`:

- `00-b2-test-truth-note.md`

`B2` proves LAN test-category truth only. It does not claim new LAN runtime
behavior, new integration/e2e/security coverage, or closure of protocol,
service, portal, or physical proof gaps.

If a proof script emits `test-results/.../proof.json`, the workpack proof pack must reference that file and must not imply the artifact exists until it has been regenerated on this branch/worktree.

## Proof Paths Explicitly Not Claimed By Current Slices

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
