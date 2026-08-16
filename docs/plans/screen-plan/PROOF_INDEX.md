<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Screen Plan Proof Index

## Deterministic proof root

```text
output/screen-plan-proof/<workpack-file-stem>/
```

Historical/current snapshot artifacts may use named subdirectories under `output/screen-plan-proof/`. The selected workpack must name the accepted artifact path before any row is checked.

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
plan: screen-plan
workpack: <workpack id and name>
owner: screen-plan | screen-domain | screen-ai-handoff | ai-handoff | policy-handoff | enforcement-handoff | custody-handoff | portal-handoff | remote-access-handoff | browser-handoff | app-game-handoff | network-handoff | tracking-handoff | protocol-service | docs-only
artifact_shape: proof-summary-json | proof-pack | command-log | screenshot | manifest | blocker | n/a
proof_tier: p0-static | p1-contract | p2-unit | p3-local-runtime | p4-service | p5-operator | p6-physical-or-external | n/a
platform: windows | macos | linux-x11 | linux-wayland | android-emulator | android-physical | ios | web | n/a
capture_scope: selected-window | active-window | display | managed-browser | browser-cdp | app-window | protected-surface | live-view | n/a
trigger_type: manual | parent-opt-in | managed-browser | native-foreground | timed-cadence | service-started | disabled | n/a
permission_state: not-tested | granted | missing-visible | unsupported-visible | manual-required | n/a
capability_state: not-tested | disabled | unsupported | permission-required | degraded | ready | blocked | n/a
protected_surface_state: not-tested | allowed | blocked | redacted | degraded | n/a
queue_state: not-tested | encrypted | pending | backpressured | drained | deleted | expired | delete-failed | n/a
image_custody_state: not-tested | local-only | encrypted | deleted | retained-opt-in | raw-remote-denied | n/a
retention_state: not-tested | no-raw-retention | opt-in | rejected-unsafe | delete-after-ttl | delete-failed-visible | n/a
analysis_state: not-tested | summary-only | ocr | vlm | router | result-schema | validator | no-ai-claim | n/a
policy_state: not-tested | evidence-ref-only | dry-run | no-authority | handoff-required | n/a
enforcement_state: not-tested | guarded | no-execution | handoff-required | n/a
portal_state: not-tested | visible | screenshot | degraded-visible | not-claimed | n/a
live_view_state: not-tested | disabled | preflight-only | loopback | relay-cache-harness | worker-gated | product-blocked | n/a
remote_boundary_state: not-tested | raw-upload-denied | redacted-summary-only | parent-approved | custody-required | n/a
privacy_legal_state: not-tested | required | approved | blocker | n/a
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <stdout/stderr artifact pointer, proof file, test result path, screenshot path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
manual_required_note: <manual-required gap or n/a>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store command output, screenshots, proof summaries, traces, redacted diagnostics, or failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Required proof themes

```text
capture source proof
permission/degraded state
local deletion/retention proof
custody labels
redacted output proof
portal visible state proof
manual-required states
protected-surface behavior
platform-specific no-claim boundary
AI/policy/enforcement handoff boundary
live-view and remote-boundary no-claims
```

## No-claim language

Do not claim:

```text
whole screen plan ready
screen-AI pipeline ready
AI/model quality ready
policy authority ready
enforcement ready
raw retention ready
product live-view ready
remote access ready
raw remote upload ready
privacy/legal approved
PR_READY
```

unless the selected workpack proof root proves the claim or carries the exact blocker.
