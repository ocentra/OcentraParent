# 02 Household First-Run And Profiles

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `02 Household First-Run And Profiles`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

First-run setup is not product-complete. Parents still need an understandable
path for household, child profile, device, and role state.

## Where We Want To Be

A nontechnical parent can see setup progress, what is missing, and which states
are live, unavailable, or manual-required.

## Decision Tree

| If the assignment touches... | Read next                                                                                    | Required proof                    |
| ---------------------------- | -------------------------------------------------------------------------------------------- | --------------------------------- |
| Account/household/roles      | `../../account-identity-family-plan/AGENTS.md`                                               | role/authZ proof                  |
| Pairing/device setup         | `../../setup-install-provisioning-plan/AGENTS.md` and `../../lan-plan/AGENTS.md` as assigned | pairing/setup proof               |
| Child profile UI             | this workpack and owning profile/read-model source                                           | empty/partial/configured UI proof |
| Parent client first launch   | `../../parent-desktop-runtime-package-plan/AGENTS.md`                                        | launch/setup handoff proof        |

## Required First-Run States

- No account/session.
- Account exists but no household.
- Household exists but no child profile.
- Child profile exists but no device.
- Device discovered but not paired.
- Device paired but service unavailable.
- Parent is observer/co-parent/controller with different allowed actions.
- Recovery needed: invite expired, pairing expired, permission missing, device offline, or stale proof.

## Requirement Checklist

- [x] Show household and child profile state.
- [x] Show parent role and observer/co-parent distinction.
- [x] Show setup incomplete and recovery-needed states.
- [x] Keep child-device authority separate from account membership.
- [x] Add tests for empty and partially configured households.
- [x] Show next action for each first-run state without pretending setup is complete.
- [x] Prove invite/pairing/session expiry handling.
- [x] Preserve child safety copy without exposing private child data.

## Acceptance And Proof

The route renders setup state from contracts/read models or labels fixture state
explicitly.

Expected proof names:

- `portal.first-run.no-account`
- `portal.first-run.no-household`
- `portal.first-run.no-child-profile`
- `portal.first-run.unpaired-device`
- `portal.first-run.service-unavailable`
- `portal.first-run.role-authz-matrix`
- `portal.first-run.recovery-expired-negative`

Proof must include screenshots/DOM snapshots for each selected state, source fixture/live route, role used, and unavailable/manual-required notes.

Current checkpoint truth on this branch/worktree (2026-06-18):

- Focused proof is now recorded under `output/portal-ux-household-surfaces-plan-proof/02-household-first-run-and-profiles/`.
- The current packet proves the Start route projects the typed first-run setup state machine and the required first-run states on current source, including no account, no household, no child profile, unpaired device, service-unavailable/manual-required, and recovery-expired-negative visibility.
- The current packet also proves the role/authZ matrix and authority split: signed-in-without-household, co-parent invite, observer read-only scope, support-admin separation, revoked child state, stale parent state, wrong-account/recovery-required, and direct-entry-required all remain explicit instead of collapsing into a fake ready claim.
- The route keeps account membership, household authority, child profile identity, child device trust, session freshness, and sibling-plan handoffs explicit; it does not promote them into a product-complete onboarding claim.
- This checklist row is now closed on this branch/worktree because the typed `portal-domain` intent model, rendered Start route, focused unit/route tests, and the targeted Playwright proof all rerun green.

## Failure Conditions

- Do not treat account membership as device control authority.
- Do not skip empty/partial setup states.
- Do not claim pairing/install success without setup-install/LAN proof.

## Parallel Ownership Notes

Backend family setup contracts or service changes must be owned outside C unless
explicitly assigned.
