# Child Windows Distribution

Purpose: prove the child Windows package, service lifecycle, and respawn behavior.

## Live source state

- Child binary, builder, MSI, and WinSW child values exist.
- Shipped startup has no current Device Trust source, authenticated product ingress, or external health endpoint.
- Some checked-in source/harness/workflow identities remain parent-era names.
- WP02 follows WP10 reviewed implementation; static service-manager declarations are not runtime reachability.

## Validation anchors

- `npm run release:package:windows`

## Must prove

- package output exists for the child Windows artifact
- service lifecycle and restart behavior are explicit
- uninstall or disable paths are honest and testable
- current-trust startup, authenticated ingress, and external health are reachable from the installed child package

## Failure conditions

- package output is treated as proof of respawn
- uninstall resistance becomes hidden persistence
