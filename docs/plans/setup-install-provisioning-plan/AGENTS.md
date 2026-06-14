<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: AGENTS
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after a global route selects it.
> Stop rule: Choose one route/workpack; do not continue into sibling plans unless a workpack names a handoff.
> Proves: local routing and ownership only.
> Does not prove: implementation completion, deployed website, signed installers, account readiness, pairing, or PR readiness.
> Proof rule: Route changes must keep PLAN_STATE, WORKPACK_INDEX, TEST_PROOF_EXPECTATIONS, PLAN_INDEX, and FEATURE_ROUTE_INDEX aligned.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan Agent Route

Task: plan and verify the first-run journey from public family site to paired household.
Context: parents start at `family.ocentra.ca`; the site is informational by default and must not collect child activity data. The intended flow is invite/code entry, account and household authority, parent bootstrap install, child bootstrap install, pairing, and readiness status.
Scope: public web entry, install journey, provisioning state machine, role/device readiness, recovery, and handoff proof.
Out of scope: package build mechanics, auth provider implementation, LAN protocol internals, portal component details, data sync internals, and enforcement adapters.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; this plan owns the setup graph and must not absorb account, package, policy, or LAN implementation details.
- Work one setup workpack at a time and the exact proof/checklist rows; keep handoffs explicit when ownership crosses plans.
- Every completion claim must include journey boundary states (account, parent install, child install, pairing, permissions, recovery), proof artifact path, and failure conditions.
- Stop condition: no DONE/PR_READY without evidence of degraded recovery states, explicit handoff boundaries to owned adjacent plans, and separate parent/bootstrap and child/bootstrap flows.

## Research Gate

This plan is execution-grade architecture and UI guidance. Before implementation, DONE, or PR_READY, the assigned agent must inspect existing repo code/docs for the touched slice, map what already exists versus missing, and record unresolved product/architecture choices with Sujan. Do not treat this plan as product-complete until the matching proof artifacts exist.

## Read Order

1. [PLAN_STATE.md](PLAN_STATE.md)
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md)
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md)
4. One assigned workpack only
5. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md)
6. [PROOF_INDEX.md](PROOF_INDEX.md) only for proof claims

## Decision Tree

| If the task is about...                                       | Open                                               |
| ------------------------------------------------------------- | -------------------------------------------------- |
| Public family website, content/data boundary, download entry  | `workpacks/01-family-web-info-site.md`             |
| Register/login entry, account handoff, household start        | `workpacks/02-registration-login-entry.md`         |
| Parent app install, package selection, update channel handoff | `workpacks/03-parent-install-journey.md`           |
| Child agent install, permissions, platform readiness          | `workpacks/04-child-install-permission-journey.md` |
| Pairing, first-run readiness, degraded/recovery states        | `workpacks/05-pairing-readiness-recovery.md`       |
| First-run UI and state machine                                | `workpacks/07-first-run-setup-ui-and-state-machine.md` |
| Launch gate, proof manifest, route/index sync                 | `workpacks/06-rollout-proof-and-route-gate.md`     |

## Ownership Boundaries

- `account-identity-family-plan` owns identity, session, account recovery, household membership, roles, invites, and auth provider decision.
- `device-trust-bootstrap-plan` owns trusted-device bootstrap, local sealed trust keys, QR approval, and recovery/reset.
- `parent-client-runtime-distribution-plan` owns package build, signing, update, rollback, and installer artifact proof.
- `lan-plan` owns LAN discovery, peer trust, pairing protocol, signed hello, and local transport proof.
- `portal-ux-household-surfaces-plan` owns rendered parent/child UI once setup state reaches portal surfaces.
- `data-custody-storage-plan` owns custody labels, export/import, cloud sync, and storage privacy guarantees.

## Failure Conditions

- Do not claim setup complete without a parent-visible readiness state for account, parent install, child install, pairing, permissions, and degraded recovery.
- Do not collect child activity data from the public information site.
- Do not claim install support without platform-specific proof or an explicit manual-required state.
- Do not bury account/login decisions in this plan; route them to account identity.
- Do not collapse parent bootstrap and child pairing into one download claim.
