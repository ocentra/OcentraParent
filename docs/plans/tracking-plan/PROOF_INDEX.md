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

## No-claim language

Do not claim product-ready tracking, physical-device proof, background platform behavior, notification delivery, or adapter dispatch unless the selected workpack explicitly proves it.
