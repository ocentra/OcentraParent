# SOCIAL-16 Contract Proof

The Android native app matrix requires rows for:

- package visibility;
- UsageStats foreground evidence;
- accessibility route hints;
- VPN/domain hints;
- device-owner app control;
- managed-profile config.

The accepted states keep social native app support app-level, permission-gated,
manual-required, unavailable, or not implemented. The matrix can cite existing
parent-domain proof refs but cannot turn them into native route or enforcement
proof.

The focused Vitest suite accepts an honest six-row matrix, accepts package
visibility as manual-required when no Android device proof exists, and rejects
missing surfaces, native route/content/runtime claims, and unsupported
capability upgrades for accessibility and device-owner app control.

`scripts/test/social-android-native-app-host-proof.mjs` records the real host
state for Android proof. The current run finds adb installed and no attached
device/emulator, so it writes a manual-device-proof-required matrix and does not
query package visibility, screenshots, UI trees, logcat, raw installed packages,
native routes, content, account identity, runtime adapter behavior, UI delivery,
or enforcement.
