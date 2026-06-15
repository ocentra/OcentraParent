# Next Actions

## Scope and ownership

- Plan owner: `child-agent-runtime-distribution-plan`.
- Ownership domain: child Windows, macOS, Linux, Android, and iOS package distribution, respawn, tamper/uninstall, signing/device-owner matrix, and setup-device-trust handoff.
- Scope boundary: child runtime artifacts only. Parent client distribution, setup journey ownership, account identity, policy behavior, and billing behavior are out of scope.

## Decision routes and failure conditions

- If a package artifact or signing state is missing, keep the workpack open.
- If parent client distribution is being claimed here, block the row.
- If the platform cannot support respawn or device-owner behavior, keep the row manual-required.
- If setup-device-trust handoff is not explicit, do not claim package readiness.

## Actioned completion tracker

- [ ] Confirm canonical child scope and parent/child separation.
- [ ] Define the child artifact matrix.
- [ ] Define the Windows, macOS, Linux, Android, and iOS distribution contracts.
- [ ] Define signing, store, and device-owner states per artifact.
- [ ] Define managed respawn and uninstall/tamper proof expectations.
- [ ] Define setup-device-trust handoff inputs and outputs.
- [ ] Define the proof matrix and external artifact root.
