# Parent Desktop Runtime Package Plan � HID Execution Blueprint

## Execution objective

Make desktop runtime package claims auditable: launch, local service link, origin security, packaging, and release artifacts.

## Slice 01 � Checklist and Snapshot Baseline

### Acceptance

- Define concrete implementation checklist/snapshot and keep aligned with workpacks.

### Tests

- `desktop.packaging.checklist-sync`

### Proof

- `docs/proof/parent-desktop-runtime-package-plan/slice-01-checklist-snapshot.md`

## Slice 02 � Tauri/Service Contracts

### Acceptance

- Local service invocation and token/header/origin checks are contract-defined.

### Tests

- `desktop.connection.authz`
- `desktop.origin.header-security`

### Proof

- `docs/proof/parent-desktop-runtime-package-plan/slice-02-tauri-service.md`

## Slice 03 � LAN Route and Update/Recovery

### Acceptance

- LAN controller path and rollback/restart behavior are proven.

### Tests

- `desktop.route.state.machine`
- `desktop.update.rollback`

### Proof

- `docs/proof/parent-desktop-runtime-package-plan/slice-03-lan-recovery.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/parent-desktop-runtime-package-plan/workpacks/01-tauri-shell-contract-boundary.md
- Slice 02: docs/plans/parent-desktop-runtime-package-plan/workpacks/02-local-service-connection-command.md
- Slice 03: docs/plans/parent-desktop-runtime-package-plan/workpacks/03-lan-route-and-controller-state.md

## PR-ready gate

- No packaging/release check mark without launch artifact, smoke output, and origin/header proof logs.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: bootstrap/install contract checks
- Integration: service smoke + runtime handoff
- E2E: install/rollback/recover cycle
- Security: origin/header policy and signing checks
- Non-functional: startup latency and smoke reproducibility

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
