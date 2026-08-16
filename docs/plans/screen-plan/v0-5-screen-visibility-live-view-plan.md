# V0.5 Screen Visibility And Live View Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `V0.5 Screen Visibility And Live View Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Rule

Local screen summaries are the default screen product path.

Screenshots and live view are separate explicit modes.

## Modes

1. Local summary only.
2. Optional parent-visible screenshot after explicit opt-in.
3. Temporary screenshot evidence for alert only.
4. Parent-initiated live view on LAN only.
5. Parent-initiated live view through parent-approved relay.
6. Unavailable / not claimed.

## Extra Requirements For Screenshots Or Live View

- Explicit parent setting.
- Child/device disclosure.
- Platform proof.
- Transport/custody model.
- Retention setting.
- Viewer audit.
- Export/delete proof.
- Remote relay proof if away from LAN.
- No default Ocentra-hosted storage.

## Separation From Screen Evidence

Local evidence summaries:

- use encrypted temporary images;
- run local OCR/vision;
- store summaries and refs;
- delete raw images.

Screenshot retention/live view:

- exposes raw visual content;
- needs a separate consent/custody contract;
- needs transport and viewer audit;
- needs stricter retention and legal/privacy review;
- must not be enabled by local-summary opt-in.

## Required Product Decision

Before implementing live view, record whether Ocentra supports:

- no live view;
- LAN-only live view;
- relay-backed live view;
- screenshot alert snapshots only;
- raw screenshot history.

Each decision needs proof and a separate feature/checklist update.
