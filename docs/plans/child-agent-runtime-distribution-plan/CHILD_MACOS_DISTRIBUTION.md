# Child macOS Distribution

Purpose: prove the child macOS package, launchd lifecycle, and notarization behavior.

## Validation anchors

- `npm run release:package:macos`

## Must prove

- package output exists for the child macOS artifact
- launchd or equivalent service lifecycle is explicit
- notarization and install state are not hidden

## Failure conditions

- package output is treated as notarization proof
- launchd lifecycle is implied without a real service boundary
