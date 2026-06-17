<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `Device Trust Bootstrap Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan Proof Index

## Proof root

```text
output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/
```

## Required universal proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
03-platform-proof-status.md
16-validation-commands.log
17-blockers.md
```

## Command log format

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
blocker-class: real dependency blocker | external platform constraint | avoidable local execution gap | n/a
notes: <short note>
```

## Platform proof status

`03-platform-proof-status.md` should say which of these were expected for the
touched slice and which were not:

- Windows proof expected / not relevant
- Android proof expected / not relevant
- Linux proof expected / not relevant
- iOS proof external-platform-constraint / not relevant
- macOS proof external-platform-constraint / not relevant

Do not mark iOS or macOS as a local blocker from this Windows host when the
missing proof is only an external-platform constraint.

## Blocker file

`17-blockers.md` should separate:

1. real dependency blockers
2. external platform constraints
3. avoidable local execution gaps

If there are no blockers in one category, say so explicitly.

## No fake-green proof rule

- A proof folder with only document assertions does not close a runtime claim.
- If a workpack is still docs-only, the proof must say so directly.
- If mocks were used, the proof must call them out and explain why the remaining real-behavior gap is acceptable for that slice.

## Legacy note

Older `docs/proof/device-trust-bootstrap-plan/*` references are legacy pointers. New work should use `output/device-trust-bootstrap-plan-proof/`.
