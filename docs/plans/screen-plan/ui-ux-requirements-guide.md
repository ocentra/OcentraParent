# UI/UX Requirements Guide

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `UI/UX Requirements Guide`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Screen evidence UI must help a parent understand what happened without exposing raw screenshots by default.

## Primary Surfaces

| Surface                 | Purpose                                                                |
| ----------------------- | ---------------------------------------------------------------------- |
| Device capability panel | Shows whether the selected device can capture/analyze screen evidence. |
| Policy screen page      | Controls opt-in, schedule, confidence, retention, and policy behavior. |
| Activity screen tab     | Shows screen evidence summaries and recent state.                      |
| Audit/explanation view  | Explains why a policy action happened.                                 |
| Data/export page        | Shows retention, deletion, and custody proof.                          |
| Optional live-view page | Separate surface only if live view is approved.                        |

## Required States

- Disabled by parent.
- Unsupported platform.
- Permission missing.
- Enabled and idle.
- Capture queued.
- Processing.
- Processed summary.
- Failed capture.
- Failed analysis.
- Raw image deleted.
- Low-confidence summary.
- Policy used summary evidence.
- Screenshot retention unavailable/disabled.
- Live view unavailable/disabled.

## Parent-Facing Questions

The UI should clearly answer:

- Is screen evidence on?
- Which child/device/schedule does it apply to?
- Is this device able to capture?
- Is local AI/OCR available?
- What was seen in summary form?
- How confident is the summary?
- Was the raw image deleted?
- Did a policy action use this evidence?
- What should the parent do next?

## Layout Requirements

- Do not overload info panels with every raw field.
- Group fields into Status, Evidence, Custody, Policy, and Diagnostics.
- Keep raw diagnostics behind an expandable section.
- Put primary action controls near the status they affect.
- Use clear disabled states when capture, analysis, update, or capability actions do not apply.
- Use visual hierarchy to separate live state from historical summaries.
- Use screenshots only as optional future/live-view scope, not as default summary cards.

## Activity Summary Card

Each summary card should include:

- time;
- device;
- source app/window when available;
- summary;
- category labels;
- confidence;
- redaction state;
- policy result if any;
- custody/deletion state;
- audit details link.

## Policy Screen Requirements

Policy authoring should cover:

- on/off/paused/emergency allow;
- capture cadence or trigger mode;
- capture scope;
- confidence threshold;
- low-confidence behavior;
- protected surface behavior;
- retention TTL;
- policy action mode: observe, warn, ask parent, limit, block.

## Diagnostics Requirements

Diagnostics should show:

- queue size;
- oldest queued job age;
- last capture error;
- local model status;
- permission status;
- last deletion status;
- last summary ref.

Diagnostics should not show raw image paths or secrets in parent-facing copy.

## Screenshot Proof Requirements

Every UI change must include screenshots for desktop and narrow/mobile widths when the surface is parent-facing.

Required screenshot names should include:

- route;
- state;
- viewport;
- run ID or date.
