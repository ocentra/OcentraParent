# Workpack 04: Entitlement Delivery Gates

Goal: define how paid status reaches account, household, device, and feature gates.

Expected shape:

- Entitlement is linked to account/household authority.
- Device and feature gates read app-owned entitlement state.
- Cancellation, refund, dispute, chargeback, or payment failure changes entitlement through explicit states.
- Offline child devices get stale/degraded entitlement state, not silent false success.

Expected proof:

- Household/device/role entitlement matrix.
- Stale/offline entitlement proof.
- Revoke and grace-period proof.
- Audit and support diagnostics proof.

Failure: each product surface checking Stripe directly or inventing its own paid-status truth.

## Execution Detail

Minimum context:

- `docs/plans/account-identity-family-plan/workpacks/02-identity-household-role-model.md`
- `docs/plans/data-custody-storage-plan/AGENTS.md`
- `docs/features/production-distribution-support.md`
- `docs/features/remote-lan-mobile-platforms.md`

Research required:

- Inspect existing Parent packages for feature gates or entitlement-like concepts.
- Decide with Sujan what payment can and cannot disable for child safety.
- Decide whether child devices receive entitlement snapshots and how stale snapshots behave.

Required model:

- Billing customer.
- Household entitlement.
- Plan tier.
- Device/child limits.
- Feature gates.
- Offline/stale entitlement.
- Revocation and grace.

Expected tests/proof names:

- `entitlement.household-scope`
- `entitlement.device-limit`
- `entitlement.offline-stale-degraded`
- `entitlement.cancel-revokes-paid-feature`
- `entitlement.safety-feature-not-silently-disabled`

Proof artifact expectations:

- Entitlement state machine.
- Cross-family and wrong-role rejection.
- Offline child behavior.
- Audit event expectations.
