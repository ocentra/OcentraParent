# Research and UI Guidance

Purpose: keep the child distribution UI honest while platform gaps remain open.

## Research targets

- child service lifecycle and package launch behavior
- Windows, macOS, Linux, Android, and iOS signing / distribution docs
- respawn and restart-survival behavior per platform
- uninstall / revocation / tamper paths
- device-owner, managed-profile, supervision, and provisioning limits

## UI guidance

- show the child artifact, not a generic "mobile" or "desktop" bucket
- show manual-required and capability-only states explicitly
- show respawn, uninstall, and provisioning limits in the same place as the package state
- keep parent client distribution visually separate from child distribution
