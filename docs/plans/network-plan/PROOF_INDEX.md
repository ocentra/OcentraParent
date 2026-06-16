<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Network Plan Proof Index

## Deterministic proof root

```text
output/network-plan-proof/<workpack-file-stem>/
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
source/custody labels
unknown-attribution state
adapter/platform limits
no private network-content claim without explicit product/legal proof
portal visibility proof when UI changes
service/protocol proof when runtime changes
manual-required states
```
