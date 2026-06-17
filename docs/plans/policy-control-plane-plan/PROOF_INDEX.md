<!-- agent-capsule -->

> Agent Capsule
> Plan: `policy-control-plane-plan`
> Doc: `Policy Control Plane Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Policy Control Plane Proof Index

## Proof root

```text
docs/proof/policy-control-plane-plan/
```

Current audit state: this proof root is the canonical location referenced by the workpacks and now contains the WP06 route bundle plus universal guardrail files. Keep WP01/WP02/WP03/WP04/WP05/WP07/WP08 open until their named closeout artifacts exist.

## Required universal proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

Workpack-specific proof files named in the workpack docs are the closeout artifacts for WP01-WP08; the universal files supplement them and do not replace them.

## Command log format

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```
