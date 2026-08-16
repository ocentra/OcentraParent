# setup-install-provisioning-plan Event Architecture Instruction

## Owns

- setup journey contracts, registration entry, setup state machine, first-run setup projection, setup/install readiness handoff.

## Must not own

- account/session authority;
- device-trust transition rules;
- child package/runtime proof;
- parent package/distribution proof;
- custody or policy runtime truth.

## Required chain

```text
setup command or first-run route action
-> setup owner validates journey/state machine
-> setup readiness event/read model records state
-> parent/child/package/trust/custody consumers use typed readiness output
```

## Logging/proof

Log journey step, readiness state, blocker owner, redaction state, first-run route result, and manual-required platform state.

## Tests

Setup-domain owns journey/state-machine unit and contract tests. Portal first-run UI is portal/app proof. Parent/child distribution proof belongs to distribution plans.

## First architecture slice

Repair WP06 truth-sync, then WP03 export-surface repair. Final aggregate waits account, device trust, custody, policy, LAN, and distribution proof.
