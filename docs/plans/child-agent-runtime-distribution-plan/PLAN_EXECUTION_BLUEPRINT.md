# Execution Blueprint

Status: reset.

## Execution objective

Turn the child agent distribution route from docs into platform-specific package proof, respawn proof, tamper/uninstall proof, and setup-device-trust handoff proof.

## Execution slices

1. Child scope and route boundary.
2. Child Windows service package.
3. Child macOS service package.
4. Child Linux service package.
5. Child Android agent package.
6. Child iOS capability package.
7. Managed service respawn.
8. Parent-authorized uninstall.
9. Signing/store/device-owner matrix.
10. Setup-device-trust handoff.
11. Proof, CI, and release gate.

## Required order

- Select one workpack, then collect code, tests, validation, proof, and route sync for that workpack only.
- Do not mix proof from sibling workpacks into the current proof folder.
- Do not mark a workpack PR-ready until the selected slice has at least one negative test and one teardown or uninstall proof.
