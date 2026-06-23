# Workpack 03 - Parent Desktop Shell Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `03-parent-desktop-shell-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the desktop shell/package boundary, including local-service bridge and launch smoke, without claiming product readiness.

## Ownership boundary

```text
scripts/dev owns dev:desktop and dev:desktop:lan launch anchors.
scripts/release owns selected desktop package artifact helpers.
apps/portal owns embedded parent web surface when selected.
local service/agent-service owners provide route bridge proof only through typed handoff.
```

## Must prove

- `dev:desktop` and `dev:desktop:lan` are honest launch anchors
- the shell reaches service state or degrades honestly
- signing/update/rollback remain explicit artifact claims
- launch smoke does not imply child runtime authority

## Required proof fields

The selected proof must name, at minimum:

```text
artifact_kind
shell_kind
platform
launch_command
launch_state
service_bridge_state
degraded_state
stale_state
artifact_path
artifact_hash_state
signing_state
update_state
rollback_state
manual_required_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Failure conditions

- launch smoke becomes product readiness
- stale local-service state is treated as healthy
- desktop proof is used to claim mobile parity
- desktop launch proof is used to claim signing, update, rollback, or setup completion
- desktop shell proof is used to claim child-agent runtime authority
