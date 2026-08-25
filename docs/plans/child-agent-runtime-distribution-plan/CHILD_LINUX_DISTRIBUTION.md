# Child Linux Distribution

Purpose: prove the child Linux package, service-manager lifecycle, and package proof.

## Live source state

- Builder output, `.deb` values, binary paths, and systemd contents target the child identity.
- The checked-in unit source and smoke/workflow inputs retain parent labels; maintainer hooks tolerate service-manager failures.
- Shipped startup has no current Device Trust source, authenticated product ingress, or external health endpoint.
- WP04 follows WP10 reviewed implementation.

## Validation anchors

- `npm run release:package:linux`

## Must prove

- package output exists for the child Linux artifact
- service-manager lifecycle is explicit
- uninstall and restart behavior are testable
- lifecycle failures, trusted startup, and external health are observable rather than inferred

## Failure conditions

- package output is treated as service-manager proof
- restart behavior is implied without a real negative case
