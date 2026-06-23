# Setup Install Provisioning Route Index

Use [AGENTS.md](AGENTS.md) first, then [PLAN_STATE.md](PLAN_STATE.md), [NEXT_ACTIONS.md](NEXT_ACTIONS.md), and [WORKPACK_INDEX.md](WORKPACK_INDEX.md). Use [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when the selected workpack owner/proof family is unclear.

## Owns

```text
public family setup entry route and data boundary
registration/login handoff state labels
parent install journey state labels
child install/permission journey state labels
pairing/readiness/recovery journey labels
first-run setup state machine and readiness cards
manual-required and degraded setup states
setup proof roots and rollout aggregation
safe product-status wording
```

## Boundary split

```text
account-identity-family-plan owns auth/session/household/role/invite/recovery authority.
parent-desktop-runtime-package-plan owns parent package/sign/update/rollback/distribution proof.
child-agent-runtime-distribution-plan owns child package/runtime distribution proof.
device-trust-bootstrap-plan owns trust, key sealing, parent presence, and step-up proof.
lan-plan owns LAN discovery, signed hello, pairing protocol, and physical LAN proof.
data-custody-storage-plan owns storage/export/delete/sync/custody guarantees.
policy-control-plane-plan owns policy baseline and policy readiness semantics.
payment-subscription-plan owns subscription/entitlement readiness.
portal-ux-household-surfaces-plan owns broader rendered portal shell and household UX.
```

## Proof-root rule

Use only the selected workpack proof root from `PROOF_INDEX.md`. WP06 may aggregate earlier proof roots, but it cannot upgrade sibling blockers into readiness.

## No-claim rule

Do not claim deployed public site, registration implementation, parent installer readiness, child installer readiness, trusted pairing, first-run setup readiness, platform support readiness, or production onboarding readiness unless the selected proof root proves the claim and WP06 safely aggregates it.
