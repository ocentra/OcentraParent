<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Eventing Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Eventing Plan Proof Index

## Proof root

```text
output/eventing-plan-proof/<workpack-file-stem>/
```

## Expected route-closure proof bundle

```text
docs/proof/eventing-plan/slice-01-envelope-version.md
docs/proof/eventing-plan/slice-02-ordering-replay.md
docs/proof/eventing-plan/slice-03-consumer-boundary.md
output/eventing-plan-proof/rollout-proof/proof-summary.json
test-results/eventing-rollout-proof/proof.json
output/eventing-plan-proof/rollout-proof/pr-done-report.md
output/eventing-plan-proof/rollout-proof/command-logs/
```

If any path above is missing, keep WP12 open and record the blocker in
`PLAN_STATE.md` and `NEXT_ACTIONS.md`. Historical doc references do not prove
route closure by themselves.

## Current WP11 local proof roots

```text
output/eventing-plan-proof/63-type-safety-source-gate/proof-summary.json
test-results/eventing-type-safety-source-gate-proof/proof.json
output/eventing-plan-proof/66-76-source-safety/proof-summary.json
output/eventing-plan-proof/67-lock-await/proof-summary.json
output/eventing-plan-proof/68-fixture-parity/proof-summary.json
```

## Command log format

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```
