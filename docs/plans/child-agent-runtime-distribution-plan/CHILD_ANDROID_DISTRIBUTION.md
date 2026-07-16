# Child Android Distribution

Purpose: prove the child Android package, install state, and device-owner or managed-profile gaps honestly.

## Validation anchors

- `npm run release:package:android`
- `npm run test:child-android-protocol-package-lifecycle-proof`

## Must prove

- package output exists for the child Android artifact
- install state is explicit
- device-owner / managed-profile capability is explicit
- tamper or uninstall limitations are not hidden

## Failure conditions

- package output is treated as device-owner proof
- install proof is used to claim parent client readiness
