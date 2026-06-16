<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App Game Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# App Game Plan Proof Index

## Deterministic proof root

```text
output/app-game-plan-proof/<workpack-file-stem>/
```

## Required universal proof files

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

## Required proof themes

```text
owned package/crate/route boundary
source/custody labels
unsupported/manual-required states
negative cases
portal proof when UI changes
service/protocol proof when runtime changes
no historical checked-row overclaim
```

## No-claim language

Do not claim broad app/game product readiness from a single workpack, historical checklist row, generated read model, or docs-only update.
