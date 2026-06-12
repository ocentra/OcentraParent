# Portal UX Household Surfaces Plan � HID Execution Blueprint

## Execution objective

Make household portal behaviors service-backed, state-complete, and safe with explicit error, degraded, and manual paths.

## Slice 01 � Service-Backed Shell and Routes

### Acceptance

- All household screens use service data + explicit fallback handling.

### Tests

- `portal.action.double-submit-replay`
- `portal.authz.visible-state-matrix`

### Proof

- `docs/proof/portal-ux-household-surfaces-plan/slice-01-service-shell.md`

## Slice 02 � Setup and Profile State Machine

### Acceptance

- Household setup/profile transitions are explicit and fail-safe.

### Tests

- `portal.state-machine.integrity`

### Proof

- `docs/proof/portal-ux-household-surfaces-plan/slice-02-setup-state-machine.md`

## Slice 03 � Policy/UI and Logging Proof

### Acceptance

- UI surfaces cannot bypass policy authority; logs trace action decisions.

### Tests

- `portal.logging.trace-proof`
- `portal.no-fake-data`

### Proof

- `docs/proof/portal-ux-household-surfaces-plan/slice-03-policy-logging.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/portal-ux-household-surfaces-plan/workpacks/01-service-backed-shell-and-navigation.md
- Slice 02: docs/plans/portal-ux-household-surfaces-plan/workpacks/02-household-first-run-and-profiles.md
- Slice 03: docs/plans/portal-ux-household-surfaces-plan/workpacks/03-device-inventory-and-source-states.md

## PR-ready gate

- No UI claim until screenshots for empty/error/degraded/permissions states are stored.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: portal contract schema checks
- Integration: data flow from service to household UI
- E2E: visible state/error transitions
- Security: authZ leaks and unauthorized state transitions
- Non-functional: render and accessibility checks under failure

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
