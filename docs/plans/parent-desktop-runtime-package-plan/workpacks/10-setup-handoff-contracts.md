# Workpack 10 - Setup Handoff Contracts

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `10-setup-handoff-contracts`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the install-state handoff between setup and parent client distribution.

## Must prove

- the request/response contract is explicit
- the contract names the real parent client state it needs
- setup continues to own the journey, not the package mechanics
- compatibility is preserved for the historical folder path

## Failure conditions

- setup becomes a package proof shortcut
- route bridge and setup journey are merged
- child runtime or policy claims leak into the handoff contract
