# account-identity-family-plan Instruction

## Verdict

`partial`. WP01 is real; WP02-WP05 are partial; WP06/WP07 remain open. Do not mark done.

## Assign first

`account-wp02-05-proof-reconciliation`:

- finish WP02-WP05 proof reconciliation against `packages/family-domain`, `packages/setup-domain`, `crates/family-identity-core`, and `crates/provisioning-core`;
- write exact proof files or blocker notes for missing proof rows;
- keep Cloudflare/setup/custody/device-trust dependencies explicit in WP06.

## Then

- `account-wp07-real-family-ui-proof`: prove account/household/device/recovery UI states, not setup-only projection.
- `account-wp06-security-route-gate`: aggregate only after WP02-WP05/WP07 and sibling blockers are current.

## Coordinate with

- `cloudflare-control-plane-plan` for auth/cloud runtime proof.
- `setup-install-provisioning-plan` for first-run route/setup producer surfaces.
- `device-trust-bootstrap-plan` for trusted-device authority.
- `data-custody-storage-plan` for export/delete/recovery substrate.

## Do not

- Do not claim provider/session/household authority from setup UI alone.
- Do not close WP06 before sibling route blockers are explicit.
- Do not write account authority into portal or parent-domain frontage packages.
