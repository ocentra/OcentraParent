# Real AI Analysis And Pipeline Proof Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `Real AI Analysis And Pipeline Proof Matrix`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This is a merge-blocking AI-analysis proof matrix. AI cannot be called done only
because contracts, mocked fixtures, or parser tests are green. AI PR-ready
requires real capture artifacts or real stored evidence, real local analysis,
schema validation, and visible parent explanation proof.

The final combined trigger-to-capture-to-analysis-to-policy/action proof is
owned by `docs/plans/screen-ai-pipeline-plan` after the screen and AI
prerequisite branches are merged or explicitly stacked.

## Proof Layers

| Layer                         | Purpose                                                                                                                                        | Required before done                                                        |
| ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| Controlled AI fixture proof   | Local deterministic pages/apps/images with known expected labels                                                                               | Yes                                                                         |
| Real capture analysis proof   | Analyze real screen captures produced by screen triggers                                                                                       | Yes                                                                         |
| Household provider mesh proof | Child agent owns evidence/work, a trusted provider claims one job under lease, returns a typed result, and child agent validates before policy | Required before household mesh execution claims                             |
| Live operator proof           | User/worker opens real URLs/apps and records AI output                                                                                         | Required before product-complete claim                                      |
| Pipeline action proof         | Capture, analysis, policy decision, and action/dry-run output happen in one run                                                                | Required in `screen-ai-pipeline-plan` before product-complete/action claims |

## Required Analysis Scenarios

Each scenario must write:

```text
output/ai-plan-proof/real-analysis/<scenario-id>/
  00-scenario.md
  01-source-evidence.json
  02-capture-proof-ref.json
  03-ai-context.json
  04-provider-route.json
  05-model-runtime-status.json
  06-ai-result.json
  07-policy-decision.json
  08-journal-read-model-proof.json
  09-parent-explanation.json
  10-ui-snapshot.png
```

When a household provider mesh route is used, the scenario must also write:

```text
  11-ai-work-item.json
  12-provider-discovery.json
  13-provider-selection.json
  14-claim-lease-proof.json
  15-provider-execution-result.json
  16-result-validation.json
  17-event-chain-proof.json
  18-policy-authority-proof.json
  19-custody-proof.json
```

| Scenario id                            | Input                                                               | Expected AI evidence                                                                                    |
| -------------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `youtube-ordinary-video`               | Managed browser or active-window capture of ordinary YouTube video  | `video` plus platform/source refs; content risk unknown unless metadata/OCR/VLM supports stronger claim |
| `youtube-education-video`              | Educational YouTube video or controlled fixture                     | `education` and `video` evidence with confidence and source refs                                        |
| `vimeo-video`                          | Vimeo video or controlled fixture                                   | `video` evidence, platform/source refs, unknown risk unless supported                                   |
| `facebook-social-feed`                 | Facebook accessible page or controlled social-feed fixture          | `social`, `feed` or `account/sign-in` evidence based on visible/typed proof                             |
| `browser-game`                         | Browser game or controlled game fixture                             | `game`, `browser-game`, optional multiplayer/purchase/login signals if visible                          |
| `native-game`                          | Native game or controlled game-window fixture                       | `game` evidence linked to app/game session refs and screen summary                                      |
| `bypass-tool`                          | VPN/proxy/private-browser/bypass fixture or real allowed test app   | `bypass-tool` candidate with confidence and evidence refs                                               |
| `shopping`                             | Shopping page fixture or real page                                  | `shopping` evidence with source refs                                                                    |
| `school-productivity`                  | School/productivity fixture or real page/app                        | `school` or `productivity` evidence with source refs                                                    |
| `unknown-activity`                     | Unknown app/page with weak evidence                                 | `unknown` or `manual-required`, not invented certainty                                                  |
| `adult-or-violence-controlled-fixture` | Controlled local fixture only unless legal/manual proof is approved | risk label only from fixture/safe evidence; no public unsafe browsing required                          |
| `protected-or-permission-required`     | Protected surface or permission-required state                      | no AI content claim; degraded/protected state only                                                      |

## Analysis Acceptance

Every AI analysis proof must show:

- context was built from typed evidence, not direct OS/browser/screen scanning by
  the model;
- local provider route selected the cheapest safe lane first;
- model/runtime state is recorded;
- output schema validation passed or degraded explicitly;
- confidence and unknown/degraded reasons are present;
- evidence refs and parent-rule refs are present;
- memory/graph refs, if used, cite source evidence;
- AI result did not directly enforce;
- parent explanation cites evidence and rules;
- raw screen image deletion proof remains linked when screen capture was used.

Household provider mesh acceptance additionally requires:

- AI work is created from typed evidence, not direct scanning;
- provider is selected by capability, custody, resource state, and parent
  policy;
- claim is granted exactly once;
- competing claims are rejected;
- expired leases cannot complete accepted results;
- results are rejected if provider, claim, evidence, or custody mismatches;
- child agent validates results before policy;
- providers cannot publish policy or enforcement;
- raw screenshots are not sent by default.

## Final Pipeline Hand-Off

The final pipeline pass must run the whole chain:

```text
real trigger
  -> real capture or structured skip
  -> encrypted queue
  -> OCR/VLM/text/deterministic or household provider route
  -> claim/lease/result-validation when mesh route is used
  -> schema-valid child-accepted AI result
  -> deterministic parent policy
  -> action or dry-run action
  -> journal/read model
  -> portal explanation
  -> deletion proof
```

Required final pipeline scenarios live in
`docs/plans/screen-ai-pipeline-plan/pipeline-proof-matrix.md`. The AI pass must
produce analysis artifacts that the pipeline pass can reuse.

Recommended pipeline scenario mapping:

| Pipeline id                 | Policy setup                                             | Required proof                                                                        |
| --------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `observe-social-video`      | Social/video set to observe                              | Capture/analyze/journal/explain; no enforcement                                       |
| `warn-social-feed`          | Social/feed set to warn                                  | Policy emits warn decision and parent-visible explanation                             |
| `ask-parent-unknown-game`   | Unknown game/app set to ask parent                       | Ask-parent decision created with source refs                                          |
| `time-limit-browser-game`   | Browser game has short time budget                       | Time-limit decision/dry-run includes expiry/timer refs                                |
| `block-bypass-tool`         | Bypass tool set to block in dry-run or real adapter mode | Policy decision proves block candidate; real enforcement only if adapter proof exists |
| `allow-school-productivity` | School/productivity allowed                              | Policy allows and explanation cites why                                               |
| `low-confidence-degrade`    | Ambiguous evidence                                       | AI result degrades to unknown/manual-required and policy follows safe fallback        |
| `disabled-no-capture-no-ai` | Capture disabled                                         | No capture, no AI analysis, no policy action from screen                              |
| `cadence-repeated-analysis` | Timed cadence every few seconds in test                  | Multiple captures produce bounded repeated analysis without queue flood               |

## Done Gate

AI analysis work is not done until:

- at least one browser-use capture is analyzed by AI;
- at least one app-use capture is analyzed by AI;
- at least one timed cadence sequence is analyzed without queue flood;
- at least one low-confidence case degrades safely;
- every proof writes artifacts under `output/ai-plan-proof`;
- UI screenshots show parent-visible result, source refs, and degraded states.

Product-complete screen AI work is not done until the separate
`screen-ai-pipeline-plan` also proves capture plus analysis plus policy/action.
