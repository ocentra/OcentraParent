# Account Identity Family Plan � HID Execution Blueprint

## Execution objective

Make identity + household authority explicit and enforceable in implementation with replay-safe auth and explicit family boundaries.

## Slice 01 � Provider Decision and Custody Contract

### Acceptance

- ADR names chosen auth provider model and custody ownership.
- Family/household identity authority mapped to one source of truth.

### Tests

- `account-identity.auth-provider.decisions`
- `account-identity.contract.schema-negative`

### Proof required

- `docs/proof/account-identity-family-plan/slice-01-provider-adr.md`

## Slice 02 � Session and Token Lifecycle

### Acceptance

- Login, refresh, expiry, revocation, replay and stolen token handling are proven by tests and logs.

### Tests

- `account-identity.auth-session.replay-idempotency`
- `account-identity.recovery.rate-limit`

### Proof required

- `docs/proof/account-identity-family-plan/slice-02-token-lifecycle.md`

## Slice 03 � Family Role/Device Authorization Boundaries

### Acceptance

- Parent, child, co-parent, support, and admin roles reject cross-family and stale actions.

### Tests

- `account-identity.authz.role-boundary`
- `account-identity.observability.audit`

### Proof required

- `docs/proof/account-identity-family-plan/slice-03-authz-boundary.md`

## Slice 04 � Recovery and Abuse Hardening

### Acceptance

- Recovery and invite flows reject enumeration, reuse, and lockout-bypass behavior.

### Tests

- `account-identity.recovery.rate-limit`
- `account-identity.authz.role-boundary`

### Proof required

- `docs/proof/account-identity-family-plan/slice-04-recovery-abuse.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/account-identity-family-plan/workpacks/01-auth-provider-decision.md
- Slice 02: docs/plans/account-identity-family-plan/workpacks/02-identity-household-role-model.md
- Slice 03: docs/plans/account-identity-family-plan/workpacks/03-session-token-lifecycle.md
- Slice 04: docs/plans/account-identity-family-plan/workpacks/04-invites-recovery-lifecycle.md

## PR-ready gate

- No `PLAN_STATE` checkbox may be marked complete without all four slices and corresponding proof logs.
- Include failed case list and explicit manual-required limitations.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: auth provider/session contract decoders
- Integration: authN/authZ and token replay flows
- E2E: registration/account/handoff lifecycle
- Security: role separation, replay-idempotency, rate-limit abuse
- Non-functional: observability + rollback proof

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
