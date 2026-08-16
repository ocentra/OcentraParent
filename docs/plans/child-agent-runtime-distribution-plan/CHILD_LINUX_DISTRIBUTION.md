# Child Linux Distribution

Purpose: prove the child Linux package, service-manager lifecycle, and package proof.

## Validation anchors

- `npm run release:package:linux`

## Must prove

- package output exists for the child Linux artifact
- service-manager lifecycle is explicit
- uninstall and restart behavior are testable

## Failure conditions

- package output is treated as service-manager proof
- restart behavior is implied without a real negative case
