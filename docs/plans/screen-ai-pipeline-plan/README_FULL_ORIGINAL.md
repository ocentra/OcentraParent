# Screen AI Pipeline Plan

This folder is the required second-stage integration plan for the complete
screen-capture plus AI-analysis plus policy/action path.

It exists because screen capture and AI analysis can and should land as separate
workstreams, but the product is not complete until a final pipeline pass proves
the combined runtime behavior.

## Stage Order

```text
Stage 1A: Screen capture pass
  -> proves real triggers create real capture/queue/deletion proof
  -> can PR independently

Stage 1B: AI analysis pass
  -> proves real stored/captured evidence can be analyzed and explained
  -> can PR independently

Stage 2: Screen AI pipeline pass
  -> starts only after screen and AI prerequisites are merged or explicitly stacked
  -> proves trigger -> capture event -> AI analysis event -> policy event
     -> action/dry-run/audit/read-model/deletion events -> portal
```

## Prerequisites

The pipeline pass may start only when one of these is true:

- screen capture prerequisite PR is merged to `main`;
- AI analysis prerequisite PR is merged to `main`;
- or the primary/user explicitly approves a stacked branch that includes both
  prerequisite heads.

Before the pipeline pass reports done, it must be based on a branch that contains
both prerequisite implementations.

## Non-Negotiable Product Gate

Screen capture alone is not enough.

AI analysis alone is not enough.

The product-complete claim needs proof that real activity can flow through the
whole path:

```text
browser/app/timed trigger
  -> real capture or structured-skip proof
  -> encrypted queue
  -> typed screen evidence/custody event on `crates/ocentra-eventing`
  -> child-owned AI work item
  -> same-device OCR/VLM/text/deterministic consumer or Household Mesh Bridge provider route
  -> claim/lease/result-validation when a household provider is used
  -> child-accepted schema-valid AI result or degraded-state event
  -> deterministic parent policy consumer
  -> action or dry-run action event
  -> audit/journal/read-model/deletion consumers
  -> parent portal explanation
  -> raw image deletion proof
```

The final pipeline must not reintroduce a direct call chain where the screen
runtime invokes AI, AI invokes policy, and policy invokes action in one coupled
module. Screen, AI, policy, action, audit, read-model, and deletion stages must
be typed consumers layered on the reusable Rust event bus. Cross-process parent
to child or peer-to-peer delivery still uses typed service, WebSocket, LAN,
relay, or journal/replay boundaries before publishing into each local bus.

When a household AI provider is used, the screen pipeline must stay
child-agent-owned:

```text
screen trigger
  -> child-owned encrypted screen queue
  -> screen summary/redacted crop payload
  -> ai.work.queued
  -> household provider claim/lease
  -> provider OCR/VLM/text result
  -> child result validation
  -> policy dry-run/action handoff
  -> audit/read model
  -> deletion/custody proof
```

Screen AI pipeline is not product-complete for household mesh execution until
provider claim/lease/result-validation proof and no-raw-screen-transfer proof
both pass.

## Source Plans

- [Screen Plan](../screen-plan/README.md)
- [AI Plan](../ai-plan/README.md)
- [Household AI Provider Mesh Plan](../ai-plan/household-ai-provider-mesh-plan.md)
- [Screen Real Capture Proof Matrix](../screen-plan/v0-5-real-capture-proof-matrix.md)
- [AI Real Analysis Proof Matrix](../ai-plan/real-ai-analysis-and-pipeline-proof-matrix.md)
- [Proof Tiers](proof-tiers.md)

## Plan Files

- [Pipeline Proof Matrix](pipeline-proof-matrix.md)
- [Implementation Checklist](implementation-checklist.md)

## Workpacks

| Step | Workpack                                                                                               | Target State                                                                                      |
| ---- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------- |
| 01   | [Prerequisite merge and branch gate](workpacks/01-prerequisite-merge-and-branch-gate.md)               | Pipeline work starts from a branch that contains both screen and AI prerequisite implementations. |
| 02   | [Real trigger to capture gate](workpacks/02-real-trigger-to-capture-gate.md)                           | Browser/app/timed triggers produce real capture or structured-skip proof.                         |
| 03   | [Capture to AI analysis gate](workpacks/03-capture-to-ai-analysis-gate.md)                             | Captured evidence routes into OCR/VLM/text/deterministic analysis.                                |
| 04   | [AI result to policy gate](workpacks/04-ai-result-to-policy-gate.md)                                   | Schema-valid AI result event feeds deterministic parent policy only.                              |
| 05   | [Policy action dry-run gate](workpacks/05-policy-action-dry-run-gate.md)                               | Allow/warn/ask/time-limit/block/unknown action events are proved in dry-run or real adapter mode. |
| 06   | [Journal read model and portal gate](workpacks/06-journal-read-model-and-portal-gate.md)               | Journal/read model and parent portal show the full chain with evidence refs.                      |
| 07   | [Deletion retention and custody gate](workpacks/07-deletion-retention-and-custody-gate.md)             | Raw image deletion, custody labels, and no-default-remote behavior are proved.                    |
| 08   | [Live operator proof gate](workpacks/08-live-operator-proof-gate.md)                                   | Real user-selected URLs/apps are exercised and recorded before product-complete claim.            |
| 09   | [Performance cadence and backpressure gate](workpacks/09-performance-cadence-and-backpressure-gate.md) | Timed cadence, queue backpressure, and repeated AI analysis stay bounded.                         |
| 10   | [Final rollout and PR gate](workpacks/10-final-rollout-and-pr-gate.md)                                 | Final report includes artifacts, screenshots, validation, gaps, and non-claims.                   |

## Done Signal

The pipeline pass is done only when:

- screen prerequisite proof exists;
- AI prerequisite proof exists;
- combined pipeline proof exists for browser, app, and cadence triggers;
- combined pipeline proof uses `crates/ocentra-eventing` for the screen -> AI ->
  policy -> action/read-model/deletion handoff rather than direct coupling;
- policy action or dry-run proof exists;
- parent portal screenshots show the full chain;
- raw images are deleted or explicit opt-in retention is proved;
- remote/cloud screenshot upload remains disabled by default;
- known gaps and non-claims are documented.
