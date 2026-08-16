# account-identity-family-plan Event Architecture Instruction

## Owns

- account, household, role, session, invite, recovery, and device-authority contracts;
- authz decision vocabulary consumed by other plans;
- no-claim boundary for external auth provider versus household authority.

## Must not own

- Cloudflare runtime/storage implementation;
- setup UI runtime;
- LAN transport;
- custody persistence implementation;
- payment/provider runtime.

## Required chain

```text
identity/session/setup command
-> family-domain validates authority/session/invite/recovery state
-> setup-domain consumes family contract for readiness only
-> service/orchestrator records accepted/rejected authority event
-> dependent plans consume read model or typed authority result
```

## Logging/proof

Log authority decision, session freshness, trust state, recovery transition, and no-claim provider boundary. Proof must show provider authn does not imply household/device authority.

## Tests

- `family-domain`: unit/contract for authority/session/invite/recovery.
- `setup-domain`: consumer readiness tests only.
- Portal/account UI proof is app/portal-level and must consume family/setup read models.

## First architecture slice

Finish WP02-WP05 proof reconciliation. Then WP07 must prove real account/household/device/recovery UI states. WP06 aggregates only after sibling blockers are explicit.
