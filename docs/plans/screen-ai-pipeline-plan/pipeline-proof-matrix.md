# Pipeline Proof Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Pipeline Proof Matrix`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

The pipeline proof is the final integration proof after separate screen and AI
passes. It is not allowed to replace the separate screen or AI proof packs.

## Required Artifact Root

```text
output/screen-ai-pipeline-proof/
```

Each scenario writes:

```text
output/screen-ai-pipeline-proof/<scenario-id>/
  00-scenario.md
  01-prerequisite-commits.json
  02-trigger-input.json
  03-capture-proof.json
  04-queue-proof.json
  05-ai-context.json
  06-ai-route-and-runtime.json
  07-ai-result.json
  08-policy-decision.json
  09-action-or-dry-run-proof.json
  10-journal-read-model-proof.json
  11-portal-screenshot.png
  12-deletion-proof.json
  13-validation-log.txt
```

When a household provider mesh route is used, the same scenario must also
write:

```text
  14-ai-work-item.json
  15-provider-discovery.json
  16-provider-selection.json
  17-claim-lease-proof.json
  18-provider-execution-result.json
  19-result-validation.json
  20-event-chain-proof.json
  21-policy-authority-proof.json
  22-custody-proof.json
```

## Required Scenarios

| Scenario                           | Input                                                                | Required result                                                                               |
| ---------------------------------- | -------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `browser-social-video-observe`     | Managed browser opens social/video page or controlled fixture        | capture/structured proof, AI video/social evidence, observe policy result, portal explanation |
| `browser-education-video-allow`    | Managed browser opens education video page or controlled fixture     | AI education/video evidence, allow policy result, source refs                                 |
| `browser-social-feed-warn`         | Managed browser opens social feed/sign-in fixture or accessible page | AI social/feed evidence, warn policy result                                                   |
| `browser-game-time-limit`          | Managed browser opens browser game/cloud game fixture                | AI browser-game evidence, time-limit policy dry-run with timer/expiry refs                    |
| `native-app-productivity-allow`    | Real app or fixture becomes foreground                               | active-window capture, productivity/app evidence, allow policy result                         |
| `native-game-ask-parent`           | Native game or controlled game-window fixture becomes foreground     | game evidence, ask-parent policy result                                                       |
| `bypass-tool-block-dry-run`        | VPN/proxy/private-browser/bypass fixture or approved test app        | bypass-tool evidence, block policy dry-run unless real adapter proof exists                   |
| `unknown-activity-manual-required` | Unknown app/page fixture                                             | low-confidence/unknown AI result, manual-required or ask-parent policy                        |
| `timed-cadence-repeated-analysis`  | Parent sets short cadence such as 2 seconds                          | at least three captures, repeated analysis, queue remains bounded                             |
| `disabled-no-capture-no-ai`        | Parent disables screen analysis                                      | no capture, no AI analysis, no screen-derived policy action                                   |
| `protected-surface-skip`           | Protected/permission-required surface                                | protected/degraded state, no content AI claim                                                 |

## Stage Separation

Separate PR-ready requirements:

- Screen PR-ready: real trigger/capture/queue/deletion proof exists, but AI
  result can be skipped/degraded.
- AI PR-ready: real stored/captured evidence can be analyzed and policy-ready
  AI result can be produced, but it can use imported capture artifacts or
  controlled capture refs.
- Pipeline PR-ready: same branch contains both screen and AI implementations and
  proves the live combined path.
- Mesh pipeline PR-ready: the same path additionally proves provider
  discovery, claim/lease, result validation, child-agent-only policy authority,
  event topology, and no raw screenshot transfer by default.

## Merge-Blocking Failures

```text
pipeline starts without screen prerequisite proof
pipeline starts without AI prerequisite proof
capture succeeds but AI analysis is not invoked
AI analysis succeeds but policy does not consume result
household provider route bypasses child-agent result validation
provider can publish policy or enforcement events
duplicate provider claims execute the same job twice
expired lease result is accepted
raw screenshot transfers to household provider by default
policy decision exists but portal/read model cannot explain it
timed cadence floods queue or skips deletion proof
disabled screen analysis still creates capture or AI jobs
raw screenshot persists without explicit retention opt-in
remote/API receives raw screenshot by default
AI direct-enforces without deterministic policy decision
```
