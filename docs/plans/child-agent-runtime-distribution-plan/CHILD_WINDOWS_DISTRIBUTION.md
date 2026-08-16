# Child Windows Distribution

Purpose: prove the child Windows package, service lifecycle, and respawn behavior.

## Validation anchors

- `npm run release:package:windows`

## Must prove

- package output exists for the child Windows artifact
- service lifecycle and restart behavior are explicit
- uninstall or disable paths are honest and testable

## Failure conditions

- package output is treated as proof of respawn
- uninstall resistance becomes hidden persistence
