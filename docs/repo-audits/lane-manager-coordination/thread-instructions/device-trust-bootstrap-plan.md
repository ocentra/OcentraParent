# device-trust-bootstrap-plan Instruction

## Verdict

`partial`. WP01 trust-state cleanup is coherent; plan closure still needs setup handoff, step-up/QR, recovery, key sealing, and platform proof.

## Assign first

`device-trust-step-up-qr-approval`:

- build step-up and QR approval semantics on the cleaned trust-state core;
- consume setup-domain producer contract boundaries;
- add real tests/proof for replay/stale/wrong-family/wrong-device paths.

## Defer

`local-key-sealing` until `data-custody-storage-plan` publishes recovery persistence/encryption substrate.

## Then

- `device-trust-recovery-state-validation` after custody substrate.
- `device-trust-bootstrap-add-device-handoff` with setup/install.
- platform proof rows with Windows/Android/Linux where feasible and Apple external.

## Coordinate with

- `data-custody-storage-plan` for recovery/key substrate.
- `setup-install-provisioning-plan` for bootstrap/add-device producer contract.
- `child-agent-runtime-distribution-plan` for tamper/uninstall trust semantics.

## Do not

- Do not start key sealing before custody substrate is explicit.
- Do not claim end-to-end device trust from WP01 vocabulary cleanup alone.
- Do not move trust authority into payment or portal surfaces.
