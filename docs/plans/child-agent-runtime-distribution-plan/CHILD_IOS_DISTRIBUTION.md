# Child iOS Distribution

Purpose: prove the child iOS capability package and provisioning/manual-required state honestly.

## Live source state

- Rust capability/limit contracts and their contract tests exist.
- Reviewed source at `c71becbcfd4f07eb98a118f10dbf261320f6b54e` gives the Xcode project, target, scheme, app/product name, bundle id, release inputs, and simulator artifact the canonical child identity.
- Smoke/workflow expectations, Apple signing/provisioning, physical-device launch, TestFlight/App Store ownership, expected tests, and retained proof remain open, so WP06 is not complete.

## Validation anchors

- `npm run release:package:ios`

## Must prove

- package output exists for the child iOS artifact
- provisioning and distribution state are explicit
- background-service and respawn limits remain visible
- the actual built artifact uses the canonical child project/product/scheme/bundle/release identity

## Failure conditions

- package output is treated as full service parity
- provisioning gaps are hidden behind generic mobile language
