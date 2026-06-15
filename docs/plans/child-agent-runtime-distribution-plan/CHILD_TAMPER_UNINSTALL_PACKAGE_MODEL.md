# Child Tamper and Uninstall Package Model

Purpose: keep tamper resistance, uninstall behavior, and respawn claims explicit per platform.

## Required states

- installable / uninstallable / manual-required
- respawn available / unavailable / manual-required
- tamper detected / tamper resistant / tamper unsupported
- device-owner or managed-profile available / unavailable / manual-required

## Rules

- Windows, macOS, and Linux may use platform service managers for respawn claims.
- Android claims must stay explicit about device-owner and managed-profile support.
- iOS claims must stay explicit about provisioning and background-service limits.
- Uninstall resistance is not stealth persistence.
