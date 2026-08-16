<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Portal UX Household Surfaces Proof Index

## Deterministic proof root

```text
output/portal-ux-household-surfaces-plan-proof/<workpack-file-stem>/
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

If blocked:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## Structured proof metadata

For new proof artifacts and new command-log entries, include structured metadata when available:

```text
plan: portal-ux-household-surfaces-plan
workpack: <workpack id and name>
owner: apps-portal | portal-domain | schema-domain | agent-protocol-domain | policy-handoff | setup-handoff | account-handoff | device-trust-handoff | lan-handoff | browser-handoff | app-game-handoff | network-handoff | screen-handoff | tracking-handoff | ai-handoff | payment-handoff | custody-handoff | notification-handoff | enforcement-handoff | docs-only
route: <portal route/hash/path or n/a>
viewport: desktop | mobile | tablet | responsive | n/a
actor_role: parent-controller | observer | co-parent | unauthenticated | system | n/a
read_model_source: service | fixture | demo | generated | missing | n/a
fixture_state: not-used | explicit-fixture | leaked-as-real | n/a
runtime_state: not-tested | service-backed | degraded | unavailable | stale | error | n/a
schema_decode_state: not-tested | parsed | rejected-invalid | missing-payload | n/a
portal_local_replacement_state: not-used | blocked | present | n/a
source_label_state: visible | missing | not-applicable
custody_label_state: visible | missing | not-applicable
manual_required_state: visible | hidden | not-applicable
degraded_state: visible | hidden | not-applicable
screenshot_state: not-tested | captured | missing | review-only | n/a
console_state: not-tested | clean | warnings-documented | errors-blocked | n/a
accessibility_state: not-tested | keyboard-proved | responsive-proved | blocked | n/a
action_authority_state: not-claimed | parent-confirmation-required | denied-role | handoff-required | n/a
no_claim: <what this result does not prove>
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <stdout/stderr artifact pointer, screenshot path, proof file, test result path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
```

The command log is a compact index, not a raw terminal transcript. Store screenshots, traces, console logs, test reports, or failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Required proof themes

```text
typed contract/read-model source
empty/loading/degraded/error state
source/custody labels
responsive proof when layout changes
Playwright proof when behavior changes
no fake readiness
manual-required states
console/page-error state
no-claim boundary
```

## No-claim language

Do not claim:

```text
product runtime ready
domain source truth ready
policy ready
enforcement ready
AI action ready
transport ready
custody ready
parent mobile package ready
PR_READY
```

unless the selected workpack proof root proves the claim or carries the exact blocker.
