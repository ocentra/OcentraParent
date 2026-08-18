# Child iOS Distribution

Purpose: prove the child iOS capability package and provisioning/manual-required state honestly.

## Live source state

- Rust capability/limit contracts and their contract tests exist.
- The actual Xcode project, target, scheme, app/product name, bundle id, release artifacts, smoke defaults, and CI paths remain parent-labelled.
- WP06 is an independent first source correction; it is not complete until the canonical child identity exists end to end.

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
