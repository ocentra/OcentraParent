# 23 Broad Blocking Proof Gates

## Target State

Broad app/game blocking, launch prevention, allowlists, hide/suspend/shield, and
strict mode remain manual-required until platform-specific proof exists.

## Scope

- Windows AppLocker/App Control.
- macOS Endpoint Security/MDM/System Extension.
- Linux cgroup/systemd/AppArmor/SELinux/package restrictions.
- Android Device Owner/Profile Owner/DPC.
- iOS FamilyControls/ManagedSettings/supervised/MDM.
- Rollback and safe system-app allow proof.

## Tests And Proof

- Manual-required action cannot call adapter.
- Unsupported/unavailable action cannot call adapter.
- Platform proof names setup, authority tier, rollback, and audit state.
- UI exposes manual-required reason.

## Done Signal

No broad blocking claim exists without platform proof and rollback evidence.

Use the standard checklist in [workpacks README](README.md).
