<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR readiness/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Screen AI Pipeline Proof Index

## Current audited truth

- No retained `output/screen-ai-pipeline-proof/` directory currently exists in this checkout.
- No `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` currently exists.
- Do not trust historical checked rows until proof is regenerated and retained.

## Proof root

```text
output/screen-ai-pipeline-proof/
```

## Directory routing

- Proof directories are scenario-based, not workpack-file-stem based.
- Follow the exact scenario IDs named by the assigned workpack or checklist row.
- Common examples in this plan include:

```text
output/screen-ai-pipeline-proof/service-foreground/
output/screen-ai-pipeline-proof/service-cadence/
output/screen-ai-pipeline-proof/service-analysis/
output/screen-ai-pipeline-proof/live-operator/
output/screen-ai-pipeline-proof/final-product-path/
```

## Artifact-shape warning

- Current plan docs use two artifact shapes:
  - scenario-local `proof-summary.json` artifacts named throughout the workpacks and `implementation-checklist.md`
  - the richer numbered scenario bundle defined in `pipeline-proof-matrix.md`
- Resolve the expected artifact shape in the assigned workpack before checking any row.

## Minimum retained evidence

```text
<exact artifact(s) named by the assigned workpack>
<exact command log for the run or blocker>
<negative-case proof or explicit blocker>
<no-claim boundary>
```

## Structured proof metadata

For new proof artifacts and new command-log entries, include structured metadata when available:

```text
plan: screen-ai-pipeline-plan
workpack: <workpack id and name>
owner: screen-ai-pipeline | screen-handoff | ai-handoff | schema-domain | policy-handoff | enforcement-handoff | custody-handoff | portal-handoff | browser-handoff | app-game-handoff | network-handoff | tracking-handoff | protocol-service | docs-only
scenario_id: <scenario id or n/a>
artifact_shape: proof-summary-json | numbered-bundle | command-log | manifest | screenshot | blocker | n/a
platform: windows | android | linux-wsl | macos-external | ios-external | n/a
source_trigger: browser-social-video | browser-education-video | browser-feed | browser-game | native-app | native-game | unknown-process | manual | cadence | disabled | live-operator | n/a
capture_state: not-tested | queued | captured | structured-skip | disabled-suppressed | protected-blocked | failed | n/a
capture_ref_state: present | missing | redacted | n/a
ai_context_state: not-tested | built | rejected | degraded | skipped-deterministic | n/a
model_route_state: not-tested | ocr | vlm | local-text | deterministic | unavailable | manual-required | n/a
ai_result_state: not-tested | schema-valid | invalid-rejected | degraded | unknown | n/a
policy_state: not-tested | eligible | decision-made | stricter-rule-wins | invalid-blocked | no-authority | n/a
action_state: not-tested | observe | allow | warn | ask-parent | time-limit | block-dry-run | unknown | manual-required | no-enforcement | n/a
journal_state: not-tested | written | read-model-projected | portal-rendered | missing | n/a
queue_state: not-tested | encrypted | pending | backpressured | drained | deleted | n/a
deletion_state: not-tested | success | ttl-deleted | failure-visible | raw-retention-rejected | n/a
retention_state: not-tested | no-raw-retention | opt-in | remote-upload-disabled | custody-required | n/a
live_operator_state: not-tested | manifest-required | captured | artifact-gate-only | not-rerun | n/a
portal_state: not-tested | screenshot | rendered | degraded-visible | not-claimed | n/a
architecture_gate_state: not-tested | green | red | blocked | n/a
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

## No-claim language

Do not claim:

```text
screen-AI pipeline ready
trigger-to-capture ready
capture-to-AI ready
AI-to-policy ready
policy action ready
enforcement ready
custody ready
live operator ready
PR_READY
```

unless retained proof under `output/screen-ai-pipeline-proof/` and, for slice closure, `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` prove the claim or carry the exact blocker.
