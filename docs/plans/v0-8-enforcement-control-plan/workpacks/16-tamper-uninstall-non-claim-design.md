# 16 Tamper/Uninstall Non-Claim Design

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `16 Tamper/Uninstall Non-Claim Design`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md),
[folder README](../README.md),
[enforcement-integrity-tamper feature](../../features/enforcement-integrity-tamper.md), and
[enforcement expectation](../../expectations/enforcement.md).

## Purpose

Define the explicit allowed, forbidden, and manual-required tamper/uninstall
posture so the repo cannot accidentally imply stealth or anti-tamper protection
before design and proof gates exist.

## Central schema boundary

```text
schema-domain owns public uninstall/tamper-visible state and reason schemas when they cross package/crate/protocol boundaries.
child-agent-runtime-distribution-plan and device-trust-bootstrap-plan own real install, trust, and future hardening mechanisms when selected.
security review owns approval before stealth, privilege, persistence, or removal-resistance work.
v0-8-enforcement-control-plan owns the non-claim language, parent-visible states, and rollout gate for this boundary.
```

## Source Inputs

- `../v0-8-enforcement-control-20-step-plan.md`
- `../v0-8-enforcement-control-test-blueprint.md`
- `../../features/enforcement-integrity-tamper.md`
- `../../expectations/enforcement.md`

## Target State

The product has an explicit design for install health, permission loss, removal,
alerts, support/admin removal, and platform-specific proof before hardening.

## Required proof fields

```text
canonical_schema_owner_state
allowed_behavior_state
forbidden_behavior_state
removed_state
stopped_state
permission_denied_state
unsupported_state
support_admin_removal_state
security_review_gate_state
manual_required_state
no_tamper_claim
no_stealth_claim
no_claim
```

## Tests And Proof

Proof root: `output/v0-8-enforcement-control-plan-proof/16-tamper-uninstall-non-claim-design/`

Focused validation should record:

- route/doc updates for allowed/forbidden/manual-required states
- selected service/read-model validation only when this slice changes visible state
- selected architecture gate for touched integrity/enforcement/docs surfaces
- blockers when real platform/security review proof is not available yet

## AI Worker Checklist

- [ ] Document allowed and forbidden behavior.
- [ ] Define parent-visible removed, stopped, permission-denied, and unsupported states.
- [ ] Keep removal/support paths documented.
- [ ] Require security review before hardening.
- [ ] Keep proof output manual-required until real platform artifacts exist.

## Where We Are

Tamper/uninstall protection is not product-proved. The roadmap tracks it, but
no stealth or persistence behavior is approved.

## Negative Cases

- stealth, privilege, persistence, or uninstall-resistance claims must stay forbidden
- missing security review must block hardening claims
- removed or permission-denied states must remain visible rather than hidden
- support/admin removal paths must not be conflated with tamper proof
- docs or UI text must not imply anti-tamper readiness without platform artifacts

## Manual-Required Gaps

- Real anti-tamper or uninstall-hardening behavior remains manual-required until
  design, security review, and platform proof exist.
- Platform-specific service-control or device-owner behavior remains separate.
- Alerts/notifications about removal remain downstream and do not prove
  tamper resistance.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch recorded.
- [ ] Validation commands and results recorded in `16-validation-commands.log`.
- [ ] Proof artifacts under `output/v0-8-enforcement-control-plan-proof/16-tamper-uninstall-non-claim-design/`.
- [ ] Known gaps/manual-required states listed here and in the proof note.
