# setup-install-provisioning-plan Instruction

## Verdict

`partial`. WP01/WP02/WP04/WP07 have owned green slices; WP03/WP05 partial; WP06 false-green/stale.

## Assign first

`setup-wp06-truth-sync`:

- reconcile WP06 proof pack with current WP07 state;
- make route/proof/checklist state honest;
- remove stale done wording where aggregate still says blocked.

## Then

1. `setup-wp03-export-surface-repair`: fix package export mismatch and rerun focused validation.
2. Refresh WP03/WP06 proof roots.
3. Resolve WP05 redaction ownership.
4. Consume sibling proof only after those owners publish current artifacts.

## Coordinate with

- `account-identity-family-plan` for account/session/household truth.
- `device-trust-bootstrap-plan` for pairing/trust handoff.
- parent/child distribution plans for install/package boundaries.
- `data-custody-storage-plan` and `policy-control-plane-plan` for custody/policy baseline.

## Do not

- Do not claim registration/login/session ownership from setup-domain.
- Do not claim installer/package proof; distribution plans own that.
- Do not close WP06 while its aggregate proof still contradicts status docs.
