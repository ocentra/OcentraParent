# 11 Support Diagnostics And Redaction

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `11 Support Diagnostics And Redaction`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Support diagnostics are needed for package/manual testing but must avoid private
child data leakage.

## Where We Want To Be

Support output includes version, commit, platform, package state, route state,
service state, and non-private diagnostic ids with redaction.

## Requirement Checklist

- [ ] Define allowed support fields.
- [ ] Redact secrets, tokens, raw journals, SQLite data, and child activity.
- [ ] Include package/service/source state.
- [ ] Test redaction.
- [ ] Document support use.

## Acceptance And Proof

Support output is useful for troubleshooting and safe to share in reports.

Current proof: support diagnostics require version, commit, platform, package,
service, route, capability, and degraded-state fields. The proof rejects
unredacted sensitive values and the release helper strips forbidden diagnostic
fields from support output.

## Parallel Ownership Notes

Reports/export features are separate from support diagnostics.
