# child-agent-runtime-distribution-plan Instruction

## Verdict

`partial`. Packaging/proof scaffolds are real, but canonical proof writer, test categories, tamper script path, runtime proof, and device proof are missing.

## Assign first

`child-dist-proof-root-materializer-and-test-category-normalization`:

- create/repair plan-owned proof materialization into `output/child-agent-runtime-distribution-plan-proof/<workpack>/`;
- move contract/proof-shape tests out of `tests/unit` into truthful `tests/contract` where applicable;
- fix tamper proof script path so it targets real owner tests.

## Then

1. `child-dist-windows-linux-real-package-proof`.
2. `child-dist-android-emulator-device-proof`.
3. `child-dist-tamper-uninstall-respawn-proof`.
4. `child-dist-setup-device-trust-handoff-proof` after setup/device-trust contracts are ready.

## Coordinate with

- `setup-install-provisioning-plan` for setup handoff.
- `device-trust-bootstrap-plan` for trust/uninstall/tamper semantics.
- `data-custody-storage-plan` for recovery/export substrate if referenced.

## Do not

- Do not count package script existence as package proof.
- Do not count `parent-domain` shims as child-runtime ownership.
- Do not claim Apple runtime proof from Windows host.
