# Workpack 10 - Setup Device Trust Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `10-setup-device-trust-handoff`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the setup-device-trust request/response contract that hands off into child distribution.

## Owns

- setup-device-trust request and response shape
- typed handoff from setup into child install state
- separation from parent bootstrap and parent-client package proof
- explicit route sync with setup-install-provisioning-plan

## Must prove

- the setup handoff is a typed contract, not a loose UI transition
- the request/response names the real setup state and the target package
- parent bootstrap and child pairing codes are not conflated
- the handoff proof points to the external artifact path

## Failure conditions

- setup success is claimed from package metadata alone
- parent bootstrap and child pairing codes are merged into one concept
- route sync with setup-install-provisioning-plan is missing
- proof is stored inside the plan folder
