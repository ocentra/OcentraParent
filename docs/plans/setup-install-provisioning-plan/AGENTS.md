<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: first file inside this plan after a global route selects it.
> Stop rule: choose one route/workpack; do not continue into sibling plans unless the selected workpack names a handoff.
> Proves: local routing and ownership only.
> Does not prove: implementation completion, deployed website, signed installers, account readiness, pairing, or PR readiness.
> Proof rule: route changes must keep PLAN_STATE, WORKPACK_INDEX, TEST_PROOF_EXPECTATIONS, PROOF_INDEX, CHECKLIST_INDEX, PLAN_INDEX, and FEATURE_ROUTE_INDEX aligned when those routes are touched.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan Agent Route

## Mission

This plan owns the **first-run customer journey** from public family entry to parent-visible setup readiness.

It does not own the internals of identity, package signing, LAN protocol, device trust, data custody, payment, or portal shell implementation. It defines the product journey, state machine, user-facing readiness labels, proof manifest, and handoff boundaries that make those systems usable.

## Core model

```text
public family site
  -> account/login handoff
  -> household setup handoff
  -> parent bootstrap/install handoff
  -> child bootstrap/install handoff
  -> pairing/readiness handoff
  -> first-run UI state machine
  -> rollout proof gate
```

No single step means setup is complete.

## Ownership, Import, And Boundary Contract

This plan owns setup/onboarding journey contracts, route-state matrices, public/private setup boundaries, first-run readiness labels, proof aggregation, and user-visible handoff states. It does not own account/session implementation, runtime package build/signing/update, LAN pairing internals, device-trust bootstrap, custody execution, policy authority, payment entitlement, or broad portal shell implementation.

Module roles:

```text
setup-install-provisioning-plan: setup journey state machine, setup route labels, proof roots, manual-required gates, and rollout aggregation.
setup-domain: setup/install/pairing/onboarding/provisioning contract boundary; current package export is package-info only, so internal source/tests prove owned slices but do not define a public API unless exports are added.
family-domain: household/family helper contracts consumed by setup proofs when selected.
portal-domain and apps/portal: selected first-run route projection and rendered proof for this plan's setup surfaces only.
account-identity-family-plan: account provider, login/session/token, household authority, invite/recovery, roles, and membership truth.
parent-desktop-runtime-package-plan: parent client package, signing, update, rollback, distribution, and runtime delivery truth.
child-agent-runtime-distribution-plan: child package artifacts, platform previews, runtime distribution proof, and child delivery gates.
device-trust-bootstrap-plan: trusted-device bootstrap, parent presence, key sealing, step-up, and trust approval proof.
lan-plan: LAN discovery, signed hello, local pairing protocol, transport, and physical LAN proof.
data-custody-storage-plan: storage/export/delete/sync/custody guarantees and parent-owned storage behavior.
policy-control-plane-plan: policy baseline authority, schedule/default semantics, approval, and policy delivery truth.
payment-subscription-plan: subscription, entitlement, checkout, and billing truth.
portal-ux-household-surfaces-plan: broader portal shell, household surfaces, and visual/UX readiness beyond selected setup routes.
```

Direct imports are allowed only for explicit public or selected helper surfaces:

```text
setup-domain selected internal contract/test surfaces while package exports remain package-info only
schema-domain canonical setup/readiness/status/platform/capability shapes when shared across owners
family-domain public family/household helper contracts when selected
portal-domain public setup projection contracts when selected
agent-protocol or service read models only when the selected workpack names them
neutral evidence/logging helpers that do not own sibling behavior
```

Forbidden direct imports and claims:

```text
account/session internals implemented or owned here
installer/signing/update/rollback implementation owned here
child runtime/package/platform permission implementation owned here
LAN signed hello or device-trust internals owned here
custody/export/delete side effects owned here
policy baseline or payment entitlement owned here
setup-domain internal tests upgraded into public API readiness without exports
website-only proof upgraded into onboarding readiness
download button upgraded into signed installer readiness
child installed upgraded into permissioned/paired/trusted/policy-ready
pairing discovered upgraded into trusted device
first-run UI mock upgraded into setup complete
public site collecting child activity data
```

If setup needs account, package, LAN, device trust, custody, policy, payment, portal UX, or runtime behavior, it must use typed handoffs, retained proof roots, explicit blockers, and no-claim boundaries. Do not solve sibling behavior by importing another feature owner's runtime internals.

## Scope

This plan owns:

```text
family.ocentra.ca public information and download entry boundary
registration/login entry handoff into account identity
parent bootstrap/install journey state labels
child bootstrap/install/permission journey state labels
pairing readiness and recovery state model
first-run setup UI state machine and readiness cards
proof manifest and route/index rollout gate
```

Out of scope:

```text
auth provider/session implementation -> account-identity-family-plan
installer package build/signing/notarization/update/rollback -> parent-client-runtime-distribution-plan / child-agent-runtime-distribution-plan
LAN signed hello/pairing protocol internals -> lan-plan
device trust/key sealing/QR approval -> device-trust-bootstrap-plan
portal component implementation beyond selected setup surfaces -> portal-ux-household-surfaces-plan
child activity storage/export/delete -> data-custody-storage-plan
policy baseline semantics -> policy-control-plane-plan
payment/subscription entitlement -> payment-subscription-plan
```

## Required read order

1. `PLAN_STATE.md`
2. `NEXT_ACTIONS.md`
3. `WORKPACK_INDEX.md`
4. `WORKPACK_FAMILIES.md` only when owner/proof family is unclear
5. one selected workpack only
6. `CHECKLIST_INDEX.md` only for that workpack rows
7. `TEST_PROOF_EXPECTATIONS.md` only for that workpack command/proof set
8. `PROOF_INDEX.md` only when writing or validating proof artifacts
9. `PLAN_EXECUTION_BLUEPRINT.md` only when execution order or DONE/PR_READY criteria are unclear
10. `RESEARCH_AND_DECISIONS.md` only when web/deploy/install/platform assumptions are touched

Do not read all workpacks. Do not read sibling plans by default.

## Decision tree

| If the task is about... | Open |
| --- | --- |
| Public family website, content/data boundary, download entry | `workpacks/01-family-web-info-site.md` |
| Register/login entry, account handoff, household start | `workpacks/02-registration-login-entry.md` |
| Parent app install, package selection, update-channel handoff | `workpacks/03-parent-install-journey.md` |
| Child agent install, permissions, platform readiness | `workpacks/04-child-install-permission-journey.md` |
| Pairing, first-run readiness, degraded/recovery states | `workpacks/05-pairing-readiness-recovery.md` |
| First-run UI and state machine | `workpacks/07-first-run-setup-ui-and-state-machine.md` |
| Launch gate, proof manifest, route/index sync | `workpacks/06-rollout-proof-and-route-gate.md` |

## Non-negotiable journey rules

- Public pages must not collect child activity data.
- Registration/login is a handoff to account identity, not website-owned auth logic.
- Parent bootstrap code and child pairing/bootstrap code are separate.
- Parent app installed is not child protected.
- Child agent installed is not permissioned, paired, trusted, policy-ready, or enforcement-ready.
- Pairing discovered is not pairing trusted until parent confirmation and authority proof exist.
- Platform detection is advisory; parent can choose manually.
- Every platform state must be one of: unsupported, planned, preview-only, manual-required, ready-for-test, or production-ready.
- Claims about signed installers, notarization, store delivery, update/rollback, or device-owner capability require owning plan proof.
- Setup complete requires visible readiness state for account, parent app, child agent, pairing, permissions, data custody, and policy baseline.

## Local work loop

1. Select exactly one workpack.
2. Fill the pre-edit note from the workpack.
3. Change only owned docs/source paths named by that workpack.
4. Add/update the required tests or record the exact missing test location.
5. Run focused commands from `TEST_PROOF_EXPECTATIONS.md` through `npm run agent:run --` when possible.
6. Write proof artifacts to the selected proof root from `PROOF_INDEX.md`.
7. Update `CHECKLIST_INDEX.md`, the selected workpack completion section, and `PLAN_STATE.md` only for proven rows.
8. Report no-claim boundaries.

## Failure conditions

Do not claim DONE or PR_READY if any of these are true:

- public website privacy/data boundary is unclear;
- account/login implementation was added here instead of account-identity;
- installer artifact/signing/update claims lack package-plan proof;
- parent bootstrap and child bootstrap are collapsed;
- child install is represented as protected/trusted without permissions/pairing/readiness proof;
- missing/degraded/manual-required states are hidden;
- child activity data is routed through public web/account pages;
- proof artifacts or command logs are missing;
- policy/eventing plan files are edited while active lanes own them.
