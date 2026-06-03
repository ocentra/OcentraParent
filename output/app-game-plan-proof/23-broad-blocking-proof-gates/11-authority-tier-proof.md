# Authority Tier Proof

The focused contract records authority/setup proof gates only. It does not
upgrade platform support.

- Windows block launch: `manual-required`, setup `manual-required`, proof needs
  AppLocker or App Control plus setup, authority tier, rollback, audit, and
  safe system-app allowlist proof.
- Windows AppLocker audit-only: `manual-required`; audit-only evidence is not
  enforce proof.
- macOS block launch: `manual-required`, setup `system-extension-required`,
  proof needs MDM/profile, Endpoint Security, or System Extension plus rollback
  and audit proof.
- Linux block launch: `unavailable`, setup `admin-or-root-required`, proof
  needs mechanism, distro, session, rollback, and audit proof.
- Android suspend/hide: `manual-required`, setup `device-owner-required`, proof
  needs Device Owner or Profile Owner plus rollback and audit proof.
- iOS shield: `manual-required`, setup `supervision-required`, proof needs
  FamilyControls and ManagedSettings proof.
- iOS process kill: `not-claimed`; process scanning/killing is not an iOS app
  control claim.
