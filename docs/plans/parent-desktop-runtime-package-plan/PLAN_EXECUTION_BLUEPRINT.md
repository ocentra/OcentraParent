# Execution Blueprint

Status: reset.

## Execution objective

Turn the parent client distribution route from docs into artifact proof, update/rollback proof, and route-gate proof.

## Execution slices

1. Parent client scope and route boundary.
2. Parent web portal distribution.
3. Parent desktop shell/package.
4. Parent Android package.
5. Parent iOS package.
6. Parent local-service route bridge.
7. Signing/store/notarization matrix.
8. Update and rollback.
9. Launch smoke matrix.
10. Setup handoff contracts.
11. Proof, CI, and release gate.

## Required order

- Select one workpack, then collect code, tests, validation, proof, and route sync for that workpack only.
- Do not mix proof from sibling workpacks into the current proof folder.
- Do not mark a workpack PR-ready until the selected slice has at least one negative test and one rollback or teardown proof.
