<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# V0.8 Enforcement Control Proof Index

## Deterministic proof root

```text
output/v0-8-enforcement-control-plan-proof/<workpack-file-stem>/
```

## Narrative proof root

```text
docs/proof/v0-8-enforcement-control-plan/
```

When a workpack closes, keep using the existing `slice-*.md` note convention in
this directory and update the selected workpack plus `PLAN_STATE.md` if closure
status changes.

## Composed proof command

```text
node scripts/test/v0-8-enforcement-control-plan-proof.mjs
```

Outputs:

```text
test-results/v0-8-enforcement-control-plan-proof/proof.json
output/v0-8-enforcement-control-plan-proof/18-proof-command-and-matrix/
docs/proof/v0-8-enforcement-control-plan/slice-03-proof-command-and-matrix.md
```

## Closed slice outputs

- `02-policy-decision-evidence-references`
  - `test-results/v0-8-enforcement-policy-dispatch-proof/`
  - `output/v0-8-enforcement-control-plan-proof/02-policy-decision-evidence-references/`
  - `docs/proof/v0-8-enforcement-control-plan/slice-06-policy-decision-evidence-references.md`
- `01-contract-boundary-and-effect-schemas`
  - `output/v0-8-enforcement-control-plan-proof/01-contract-boundary-and-effect-schemas/`
  - `docs/proof/v0-8-enforcement-control-plan/slice-04-contract-boundary-and-effect-schemas.md`
- `03-adapter-capability-matrix`
  - `test-results/v0-8-supported-adapter-runtime-proof/`
  - `test-results/v0-8-cross-platform-enforcement-capability-proof/`
  - `test-results/v0-8-broad-os-adapter-runtime-proof/`
  - `output/v0-8-enforcement-control-plan-proof/03-adapter-capability-matrix/`
  - `docs/proof/v0-8-enforcement-control-plan/slice-05-adapter-capability-matrix.md`
- `07-unmanaged-browser-fallback`
  - `test-results/windows-managed-unmanaged-browser-enforcement-proof/`
  - `output/v0-8-enforcement-control-plan-proof/07-unmanaged-browser-fallback/`
  - `docs/proof/v0-8-enforcement-control-plan/slice-01-unmanaged-browser-fallback.md`
- `09-timer-recovery-and-rollback`
  - `test-results/v0-8-enforcement-timer-recovery-mvp/`
  - `output/v0-8-enforcement-control-plan-proof/09-timer-recovery-and-rollback/`
  - `docs/proof/v0-8-enforcement-control-plan/slice-02-timer-recovery-and-rollback.md`

## Open workpack proof contract

- Every still-open workpack uses the deterministic proof root shown above.
- Every proof root must contain the universal files below before DONE/PR_READY.
- A docs-only route cleanup or a focused contract pass does not close an open
  enforcement workpack by itself.
- Use the selected workpack and `TEST_PROOF_EXPECTATIONS.md` to decide which
  focused commands belong in `16-validation-commands.log`.

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
policy decision ref proof
account/device authority proof
platform capability proof
execution result / no-op / mismatch / unavailable proof
rollback / recovery / manual override proof
audit / redaction proof
parent-visible and child-visible state proof
manual-required gaps
```

## Hard no-claim rule

No AI result, portal click, screen result, browser observation, app/game
session, network/domain observation, or tracking signal can become enforcement
proof unless the selected workpack also proves policy decision refs, authority,
adapter capability, execution state, rollback/recovery, audit, and visible
state.
