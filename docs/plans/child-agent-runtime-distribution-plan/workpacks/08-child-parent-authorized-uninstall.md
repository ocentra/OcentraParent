# Workpack 08 - Child Parent Authorized Uninstall

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `08-child-parent-authorized-uninstall`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define parent-authorized uninstall, revocation, and removal proof for the child agent.

## Owns

- parent-authorized uninstall flow
- revocation and removal state
- no-child-self-authorize removal rule
- uninstall cleanup and audit trail

## Must prove

- the child cannot self-authorize trust removal
- parent authorization is required where the platform allows uninstall control
- revocation leaves an auditable removal trail
- teardown proof shows child authority ends cleanly

## Failure conditions

- stealth persistence is treated as success
- child self-uninstall authority is implied
- revoked trust remains active
- removal proof is kept only in the plan folder
