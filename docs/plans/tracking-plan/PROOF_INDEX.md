<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Tracking Plan Proof Index

## Proof root rule

Prefer proof roots explicitly named inside the selected workpack.

If the selected workpack lacks a proof root, derive:

```text
output/tracking-plan-proof/<workpack-file-stem>/
```

## Required universal proof files for newly closed workpacks

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## Command log format

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```

## Central schema proof fields

Every new proof row should identify:

```text
owner: tracking-plan | schema-domain | tracking-domain | tracking-core | protocol | eventing | handoff | docs-only
schema_owner: schema-domain | protocol | eventing | evidence-boundary | tracking-private | n/a
schema_public_state: canonical | mirror | private-helper | invalid-cross-boundary | n/a
artifact_shape: proof-summary-json | proof-pack | command-log | screenshot | manifest | blocker | n/a
proof_tier: static | contract | unit | local-runtime | service | operator | physical-or-external | n/a
claim_state: allowed | blocked | manual-required | false-green-reopened | n/a
no_claim: <what this result does not prove>
```

A public contract shape is invalid if its only canonical owner is `tracking-domain` or `tracking-core`. Promote cross-boundary shapes to `schema-domain` or an approved neutral protocol/event/evidence boundary.

## No-claim language

Do not claim product-ready tracking, physical-device proof, background platform behavior, notification delivery, adapter dispatch, AI authority, policy authority, custody execution, production-worker readiness, or portal runtime readiness unless the selected workpack explicitly proves it.
