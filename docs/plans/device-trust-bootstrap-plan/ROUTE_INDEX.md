# Route Index

## Current route

1. Read [AGENTS.md](AGENTS.md).
2. Read [PLAN_STATE.md](PLAN_STATE.md).
3. Read [NEXT_ACTIONS.md](NEXT_ACTIONS.md).
4. Read [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
5. Open exactly one workpack under `workpacks/`.
6. Read [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
7. Read [PROOF_INDEX.md](PROOF_INDEX.md) only for proof claims.

## Handoffs

- `setup-install-provisioning-plan` hands install and pairing flow into this plan for trust sealing and step-up.
- `account-identity-family-plan` hands account and household authority into this plan for trusted-device bootstrap.
- `data-custody-storage-plan` consumes the trust layer for encrypted storage and recovery artifacts.
- `remote-access-plan` consumes the trust layer for standing live access grants.
- `payment-subscription-plan` consumes the trust layer for device-bound entitlement unlock.
- `policy-control-plane-plan` consumes the trust layer for policy delivery and high-risk approval gating.
- `parent-client-runtime-distribution-plan` remains the owner of packaging and signing mechanics.

## Stop rule

Do not read sibling plans, source trees, or checkpoints unless the selected workpack names the handoff.
