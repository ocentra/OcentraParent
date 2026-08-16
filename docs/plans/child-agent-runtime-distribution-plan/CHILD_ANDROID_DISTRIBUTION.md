# Child Android Distribution

Purpose: define the child Android package and keep install, runtime, transport, and device-owner or managed-profile gaps honest.

## Production code boundary

The Android package now uses the `ca.ocentra.child.agent` identity and launches
the child-owned composition foreground service. The service owns an app-private
composition directory and typed manual-required health state while deliberately
retaining the existing parent-package Android capability adapters behind the
child shell. It does not embed the Rust child-runtime crate or provide
LAN/WebSocket transport because the Android native bridge is not implemented.

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
