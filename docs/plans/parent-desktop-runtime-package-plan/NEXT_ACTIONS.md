# Next Actions

## Scope and ownership

- Plan owner: `parent-client-runtime-distribution-plan` in the historical `parent-desktop-runtime-package-plan` path.
- Ownership domain: parent web portal distribution, parent desktop shell/package, parent Android package, parent iOS package, route bridge, signing/store matrix, update/rollback, and launch smoke.
- Scope boundary: parent client artifacts only. Child agent distribution, setup journey, account provider choice, pairing protocol internals, policy behavior, billing provider behavior, and child capture/enforcement adapters are out of scope.

## Decision routes and failure conditions

- If a package artifact or signing state is missing, keep the workpack open.
- If the setup handoff is being treated as package proof, block the row.
- If the mobile surface is scaffold-only, keep the row manual-required.
- If the route bridge is not explicitly defined, do not claim parent client readiness.

## Actioned completion tracker

- [ ] Confirm canonical scope and route bridge separation.
- [ ] Define the parent client artifact matrix.
- [ ] Define the parent web portal distribution contract.
- [ ] Define the parent desktop shell/package contract.
- [ ] Define Android and iOS distribution states and proof gaps.
- [ ] Define signing/store/notarization states per artifact.
- [ ] Define update/rollback and launch-smoke proof expectations.
- [ ] Define setup handoff inputs and outputs.
- [ ] Define the proof matrix and external artifact root.
