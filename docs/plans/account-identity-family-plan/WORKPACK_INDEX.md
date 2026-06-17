<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Workpack Index`
> Kind: workpack selector.
> Read when: after NEXT_ACTIONS.md.
> Stop rule: open exactly one selected workpack; do not read every workpack.
> Proves: workpack routing only.
> Does not prove: implementation completion, provider choice, auth security, or PR readiness.
> Proof rule: update counts/status only after matching checklist rows and proof artifacts exist.

<!-- /agent-capsule -->

# Account Identity Family Plan Workpack Index

Use this index to select exactly one workpack.

| Status | Workpack | Boxes | Primary source docs | Proof root |
| --- | --- | ---: | --- | --- |
| partial | [WP01 Auth Provider Decision](workpacks/01-auth-provider-decision.md) | 10/10 | `RESEARCH_AND_DECISIONS.md`, `docs/expectations/cloud.md` | `output/account-identity-family-plan-proof/01-auth-provider-decision/` |
| complete | [WP02 Identity Household Role Model](workpacks/02-identity-household-role-model.md) | 13/13 | `docs/features/family-setup-device-roles.md`, `docs/expectations/family-setup.md` | `output/account-identity-family-plan-proof/02-identity-household-role-model/` |
| complete | [WP03 Session Token Lifecycle](workpacks/03-session-token-lifecycle.md) | 14/14 | `RESEARCH_AND_DECISIONS.md`, `packages/family-domain/src/session-lifecycle.ts` | `output/account-identity-family-plan-proof/03-session-token-lifecycle/` |
| complete | [WP04 Invites Recovery Lifecycle](workpacks/04-invites-recovery-lifecycle.md) | 13/13 | `docs/expectations/family-setup.md`, `docs/expectations/data-custody.md` | `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/` |
| complete | [WP05 Device Ownership AuthZ](workpacks/05-device-ownership-authz.md) | 13/13 | `docs/features/family-setup-device-roles.md`, `docs/expectations/platforms.md` | `output/account-identity-family-plan-proof/05-device-ownership-authz/` |
| complete | [WP07 Parent Account Family Setup UI](workpacks/07-parent-account-family-setup-ui.md) | 13/13 | `docs/expectations/portal.md`, `docs/expectations/family-setup.md` | `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/` |
| complete | [WP06 Security Proof And Route Gate](workpacks/06-security-proof-and-route-gate.md) | 16/16 | all prior workpack proof roots | `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/` |

## Default execution order

```text
WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP07 -> WP06
```

## Dependency rules

```text
WP01 blocks runtime provider/session implementation.
WP02 blocks most authorization, UI, policy, payment, and remote-access handoffs.
WP03 blocks secure-login/session claims.
WP04 may run after WP02 but must not implement data-custody side effects itself.
WP05 depends on WP02/WP03 authority and session freshness models.
WP07 depends on WP02 and enough WP03/WP04 state to render honest setup states.
WP06 must be last and consumes all previous proof roots.
```

## Do not select

Do not create new workpacks unless the existing seven cannot represent the implementation slice.

Do not split proof-only rows into tiny workpacks unless WP06 explicitly needs a proof-gate follow-up.

Do not move provider/account/family authority into setup, payment, policy, remote, LAN, device-trust, or data-custody plans.
