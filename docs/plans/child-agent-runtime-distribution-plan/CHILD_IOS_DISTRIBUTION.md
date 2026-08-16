# Child iOS Distribution

Purpose: prove the child iOS capability package and provisioning/manual-required state honestly.

## Validation anchors

- `npm run release:package:ios`

## Must prove

- package output exists for the child iOS artifact
- provisioning and distribution state are explicit
- background-service and respawn limits remain visible

## Failure conditions

- package output is treated as full service parity
- provisioning gaps are hidden behind generic mobile language
