# v0-8-enforcement-control-plan Instruction

## Verdict

`partial / false-green-risk`. WP01/WP02/WP03/WP07/WP09 look materially done; WP18 and proof router are false-green; WP12/WP19/WP20 are missing.

## Assign first

`v08-slice-01-proof-router-truth`:

- fix corrupted `PROOF_INDEX.md` and proof-router text;
- resolve missing `v0-8-integrity-alert-status-bridge` artifact reference;
- make WP18 status truthful;
- update proof script expectations without weakening required artifacts.

## Then

1. `v08-windows-browser-integrity-boundaries`.
2. `v08-app-game-service-bridge` after app-game readiness/preflight/host-capability proof is current.
3. portal Playwright proof for v0.8 state surfaces.
4. Rust inline test relocation for enforcement-counted surfaces.

## Coordinate with

- `app-game-plan` for readiness/preflight/host capability.
- `browser-plan` for managed browser proof.
- `policy-control-plane-plan` for policy dispatch.
- `logging-domain-parity` for tamper/integrity audit proof.

## Do not

- Do not claim broad app blocking or exact unmanaged URL control.
- Do not claim anti-tamper hardening beyond current proof.
- Do not let duplicate lanes edit v0.8 proof scripts/docs at the same time.
