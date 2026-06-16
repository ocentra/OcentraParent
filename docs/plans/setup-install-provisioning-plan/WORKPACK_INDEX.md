<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Plan Workpack Index`
> Kind: workpack selector.
> Read when: after NEXT_ACTIONS.md.
> Stop rule: open exactly one selected workpack; do not read every workpack.
> Proves: workpack routing only.
> Does not prove: implementation completion, deployed site, installer readiness, pairing readiness, or PR readiness.
> Proof rule: update counts/status only after matching checklist rows and proof artifacts exist.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan Workpack Index

Use this index to select exactly one workpack.

| Status | Workpack | Boxes | Primary source docs | Proof root |
| --- | --- | ---: | --- | --- |
| open | [WP01 Family Web Info Site](workpacks/01-family-web-info-site.md) | 0/12 | `RESEARCH_AND_DECISIONS.md`, `docs/expectations/family-setup.md` | `output/setup-install-provisioning-plan-proof/01-family-web-info-site/` |
| open | [WP02 Registration Login Entry](workpacks/02-registration-login-entry.md) | 0/12 | `account-identity-family-plan/AGENTS.md`, `docs/expectations/family-setup.md` | `output/setup-install-provisioning-plan-proof/02-registration-login-entry/` |
| open | [WP03 Parent Install Journey](workpacks/03-parent-install-journey.md) | 0/13 | `docs/expectations/release-installer.md`, `parent-desktop-runtime-package-plan/AGENTS.md` | `output/setup-install-provisioning-plan-proof/03-parent-install-journey/` |
| open | [WP04 Child Install Permission Journey](workpacks/04-child-install-permission-journey.md) | 0/14 | `docs/expectations/platforms.md`, `child-agent-runtime-distribution-plan/AGENTS.md` | `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/` |
| open | [WP05 Pairing Readiness Recovery](workpacks/05-pairing-readiness-recovery.md) | 0/13 | `docs/expectations/lan-pairing.md`, `lan-plan/AGENTS.md` | `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/` |
| open | [WP07 First-Run Setup UI And State Machine](workpacks/07-first-run-setup-ui-and-state-machine.md) | 0/14 | `docs/expectations/family-setup.md`, `docs/expectations/portal.md` | `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/` |
| open | [WP06 Rollout Proof And Route Gate](workpacks/06-rollout-proof-and-route-gate.md) | 0/15 | all prior workpack proof roots | `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/` |

## Default execution order

```text
WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP07 -> WP06
```

## Dependency rules

```text
WP01 can run first and stays public-site/data-boundary only.
WP02 depends on account-identity provider/session handoff or records blocker.
WP03 depends on parent runtime distribution for package/signing/update claims.
WP04 depends on child runtime/package/platform owners for child artifact/permission claims.
WP05 depends on account identity, LAN, and device trust handoffs for true pairing/trust claims.
WP07 depends on WP01-WP05 state shapes or records UI blockers.
WP06 is last and consumes all previous proof roots.
```

## Do not select

Do not create a new workpack unless the existing seven cannot represent the slice.

Do not move account identity, runtime packaging, LAN protocol, device trust, data custody, payment, policy, or portal shell ownership into this plan.
