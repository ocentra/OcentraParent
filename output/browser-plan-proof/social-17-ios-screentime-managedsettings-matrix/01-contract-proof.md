# SOCIAL-17 Contract Proof

The iOS Screen Time/ManagedSettings matrix requires rows for:

- FamilyControls authorization;
- application-token selection;
- web-domain-token selection;
- DeviceActivity monitor state;
- ManagedSettings application shield state;
- ManagedSettings web-domain shield state.

The accepted states keep iOS social native app support entitlement-required,
token-selection-required, or manual-device-proof-required until Apple approval
and physical device artifacts exist. The matrix can cite existing parent-domain
iOS entitlement proof refs but cannot turn them into raw app identity, native
route, UI, connector, runtime adapter, or enforcement proof.

The focused Vitest suite accepts an honest six-row matrix and rejects missing
surfaces, entitlement/content/identity/runtime claims, and unsupported capability
upgrades for FamilyControls authorization, token selection, and ManagedSettings
shield state.

`scripts/test/social-ios-screen-time-host-proof.mjs` now parses the built
matrix contract and writes a real host proof. On this Windows worker host it
records `isDarwinHost=false`, `appleToolingAvailable=false`,
`attachedDeviceCount=0`, and `resultState=host-tooling-unavailable`, so SOCIAL-17
stays partial/manual-required rather than claiming Apple entitlement, token
selection, DeviceActivity, ManagedSettings, UI, or enforcement behavior.
