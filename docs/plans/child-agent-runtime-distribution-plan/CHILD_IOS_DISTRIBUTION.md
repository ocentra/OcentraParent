# Child iOS Distribution

Purpose: prove the child iOS capability package and provisioning/manual-required state honestly.

## Live source state

- Rust capability/limit contracts and their contract tests exist.
- Reviewed source at `c71becbcfd4f07eb98a118f10dbf261320f6b54e` gives the Xcode project, target, scheme, app/product name, bundle id, release inputs, and simulator artifact the canonical child identity.
- Real Node/XCTest and proof-runner behavior source exists. Enforcer run `ocentra-parent.child-ios-entitlement-capability-proof-20260902102636-1c1154d5` wrote the exact canonical retained proof; its Windows XCTest outcome is explicitly platform-unavailable/manual-required, not passed.
- The bounded WP06 capability-only source/test/proof contract is complete. macOS/Xcode execution, Apple signing/provisioning, physical-device launch, TestFlight/App Store ownership, and aggregate child release readiness remain external/manual-required or open.

## Validation anchors

- `npm run test:child-ios-entitlement-capability-proof`
- `npm run release:package:ios`

## Must prove

- package output exists for the child iOS artifact
- provisioning and distribution state are explicit
- background-service and respawn limits remain visible
- the actual built artifact uses the canonical child project/product/scheme/bundle/release identity
- a host without macOS/Xcode records the XCTest row as blocked/manual-required instead of passing it

## Failure conditions

- package output is treated as full service parity
- provisioning gaps are hidden behind generic mobile language
