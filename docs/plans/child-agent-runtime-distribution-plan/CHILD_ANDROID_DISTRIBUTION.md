# Child Android Distribution

Purpose: define the child Android package and keep install, runtime, transport, and device-owner or managed-profile gaps honest.

## Production code boundary

The Android package uses the `ca.ocentra.child.agent` identity, launches the
child-owned composition foreground service, owns app-private durable custody,
and embeds a real Rust/JNI bridge to `ocentra-child-runtime`. The bridge starts
the service without a current Device Trust source, so readiness remains
fail-closed/manual-required. Binder health is local and transport is explicitly
`NOT_IMPLEMENTED`; device-owner/managed-profile and removal integration remain
absent. WP05 follows WP10 reviewed implementation.

## Validation anchors

- `npm run release:package:android`
- `npm run test:child-android-protocol-package-lifecycle-proof`

## Must prove

- package output exists for the child Android artifact
- install state is explicit
- device-owner / managed-profile capability is explicit
- tamper or uninstall limitations are not hidden
- missing/stale/revoked trust remains non-ready and authenticated ingress is not inferred from JNI/Binder composition

## Failure conditions

- package output is treated as device-owner proof
- install proof is used to claim parent client readiness
