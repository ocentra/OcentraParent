# 11 Cross-Platform Authority Matrix

## Target State

Windows, macOS, Linux, Android, and iOS/iPadOS capabilities are represented by
authority tier, setup state, proof state, and parent-visible limitation.

## Scope

- Observe-only, user-approved-helper, accessibility-assisted, managed-profile,
  device-owner, MDM-enrolled, supervised-device, system-extension,
  root/admin-service, kiosk/single-app, manual-required, and not-claimed states.
- Platform action matrix for inventory, runtime, foreground, warn, ask,
  time-limit, terminate, hide/suspend/shield, block launch, and allowlist.

## Tests And Proof

- Observe-only cannot block.
- Manual-required cannot execute.
- Android hide/suspend requires Device Owner/Profile Owner proof.
- iOS shield requires FamilyControls/ManagedSettings proof.
- macOS hard block requires MDM/Endpoint/System Extension proof.
- Linux block names mechanism and distro/session proof.

## Done Signal

Every app/game action shows what platform authority exists and what proof is
needed to move up.

Use the standard checklist in [workpacks README](README.md).
