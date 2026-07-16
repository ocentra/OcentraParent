# Workpack 07 - Child Managed Service Respawn

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `07-child-managed-service-respawn`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define managed respawn and restart-survival behavior for the supported child platforms.

Status: complete with proof under `output/child-agent-runtime-distribution-plan-proof/07-child-managed-service-respawn/`.

Proof summary:

- Windows WinSW, macOS launchd, and Linux systemd rows now prove managed respawn for kill, reboot, and service-manager restart paths.
- Deliberate stop remains explicit/manual-required on supported desktop rows; teardown and stop-path evidence is recorded instead of being hidden behind generic respawn language.
- Android remains manual-required until real device lifecycle artifacts exist.
- iOS remains unsupported for managed-service respawn and does not reuse desktop or Android proof.

## Owns

- restart survival across supported platforms
- recovery after kill, stop, reboot, or service-manager restart
- respawn truth by platform
- explicit unsupported or manual-required states

## Must prove

- managed respawn is only claimed where the platform can prove it
- restart survival is explicit and tested
- failure or unsupported states are visible, not hidden
- teardown or stop-path proof exists for each supported platform slice

## Failure conditions

- respawn is generalized across platforms without proof
- unsupported platforms are shown as supported
- manual-required states are hidden
- the slice reuses parent-client proof instead of child proof
