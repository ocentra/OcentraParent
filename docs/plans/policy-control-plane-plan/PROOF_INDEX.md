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

Current audit state: this proof root is the only canonical proof location for this plan. It now contains the WP06 route bundle, the root manifest, universal guardrail files, and checked closeout bundles for WP01, WP07, and WP08. Keep WP02/WP03/WP04/WP05 open until their named closeout artifacts exist under this same root.

## Root manifest

```text
PLAN_PROOF_MANIFEST.md
```

The manifest records current file presence and workpack proof status only. It does not upgrade open workpacks or claim runtime completeness.

## Required universal proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

Workpack-specific proof files named in the workpack docs are the closeout artifacts for WP01-WP08. The universal files and manifest supplement them and do not replace them. `02-no-claim-boundary.md` is a universal guardrail file; it is not a WP02 closeout bundle.

## Command log format

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```
