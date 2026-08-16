# Device Trust Bootstrap Route Index

## Current route

1. Read [AGENTS.md](AGENTS.md).
2. Read [PLAN_STATE.md](PLAN_STATE.md).
3. Read [NEXT_ACTIONS.md](NEXT_ACTIONS.md).
4. Read [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
5. Read [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when owner/proof family is unclear.
6. Open exactly one workpack under `workpacks/`.
7. Read [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
8. Read [PROOF_INDEX.md](PROOF_INDEX.md) only for proof claims.

## Owns

- Device-trust bootstrap rules, proof gates, local sealed trust state, parent step-up handoff, QR approval boundary, device-bound entitlement consumption, recovery reset/re-pair trust state, child tamper/uninstall authorization boundary, and cross-plan trust handoffs.
- No-claim boundaries between account login, setup journey, LAN pairing, package install, license/entitlement, data custody, remote access, policy delivery, and actual device trust.

## Boundary split

```text
account-identity-family-plan owns identity, account/session, household, role, invite, membership, and account-provider decisions.
setup-install-provisioning-plan owns setup journey, installer entrypoints, pairing UI, and setup-side handoff into trust bootstrap.
LAN/lan-domain owns LAN transport, pairing transport, selected-device registry, and packet-local proof consumers; LAN pairing is not trust root.
parent-client-runtime-distribution-plan and child-agent-runtime-distribution-plan own package build, signing, install, update, rollback, and uninstall mechanics.
data-custody-storage-plan owns encrypted storage, export/import/restore, recovery artifact custody, and retention/delete policy after trust exists.
payment-subscription-plan owns subscription entitlement policy and billing state; license state is not product unlock proof.
remote-access-plan owns standing live access grants after trust exists.
policy-control-plane-plan owns policy authoring and delivery after trust exists.
portal plans own projection/UI only.
```

## Handoffs

- `setup-install-provisioning-plan` hands install and pairing flow into this plan for trust sealing and step-up.
- `account-identity-family-plan` hands account and household authority into this plan for trusted-device bootstrap.
- `data-custody-storage-plan` consumes the trust layer for encrypted storage and recovery artifacts.
- `remote-access-plan` consumes the trust layer for standing live access grants.
- `payment-subscription-plan` consumes the trust layer for device-bound entitlement unlock, but keeps entitlement policy ownership.
- `policy-control-plane-plan` consumes the trust layer for policy delivery and high-risk approval gating.
- `parent-client-runtime-distribution-plan` and `child-agent-runtime-distribution-plan` remain the owners of packaging, signing, and install mechanics.

## Stop rule

Do not read sibling plans, source trees, or checkpoints unless the selected workpack names the handoff.

## No-claim rule

Do not claim device trust from login/session, setup completion, LAN pairing, package install, license state, entitlement snapshot presence, document tests, route tests, mock proof, or copied config. Trust claims require the selected proof root plus explicit no-claim boundary.
