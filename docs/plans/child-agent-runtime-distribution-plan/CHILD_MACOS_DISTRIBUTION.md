# Child macOS Distribution

Purpose: prove the child macOS package, launchd lifecycle, and notarization behavior.

## Live source state

- Builder output and launchd values target the child binary and package paths.
- The checked-in plist source remains parent-labelled; signing/notarization and fail-closed lifecycle source are incomplete.
- Shipped startup has no current Device Trust source, authenticated product ingress, or external health endpoint.
- WP03 follows WP10 reviewed implementation.

## Validation anchors

- `npm run release:package:macos`

## Must prove

- package output exists for the child macOS artifact
- launchd or equivalent service lifecycle is explicit
- notarization and install state are not hidden
- canonical child identity and trusted startup/health are proven through the installed package

## Failure conditions

- package output is treated as notarization proof
- launchd lifecycle is implied without a real service boundary
