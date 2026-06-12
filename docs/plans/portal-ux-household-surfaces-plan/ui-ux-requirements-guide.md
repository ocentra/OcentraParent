# Portal UX Requirements Guide

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Requirements Guide`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This guide defines the C-lane UX requirement layer. It is not an implementation
claim.

## Main UI Rule

The parent must always know:

- which child/device the screen applies to;
- where the data came from;
- whether the child-device agent, LAN route, relay, cache, or storage path is
  authoritative;
- what is live, stale, degraded, unavailable, manual-required, or scaffold-only;
- what action is safe to take next.

## Required Product Modes

- First-run household setup.
- Device inventory and selected-device context.
- Policy and schedule authoring.
- Ask-parent approvals and bonus time.
- Activity/evidence diagnostics.
- Assistant action preview.
- Reports/notifications/custody.
- Audit and copy/debug.
- Degraded/offline/manual-required review.

## Layout Requirements

- Keep operational surfaces dense and scan-friendly.
- Avoid landing-page composition for app routes.
- Do not nest cards inside cards.
- Keep controls stable in size; hover, loading, and labels must not shift layout.
- Do not scale font size with viewport width.
- Use tables, timelines, segmented controls, tabs, toggles, sliders, and icon
  buttons where the workflow expects them.
- Text must not overlap, clip, or obscure adjacent content at desktop or mobile
  widths.

## Device State Requirements

Device rows/cards must show:

- child or household role;
- agent status;
- source: local, LAN, relay, cache, parent-owned storage, unavailable;
- evidence: agent-backed, passive LAN, router/infrastructure, manual label;
- assigned/confirmed/trusted/ignored/revoked/stale/offline state;
- controllable versus visible-only;
- last seen and next action.

## Policy Requirements

Policy UI must show:

- child/device scope;
- target type and confidence;
- schedule or time budget;
- action: allow, warn, limit, ask-parent, block, observe-only, dry-run;
- enforcement capability and proof level;
- last decision/result;
- conflict or unavailable reason.

## Activity Requirements

Activity UI must distinguish:

- exact managed browser evidence;
- unmanaged browser detection;
- app/game session evidence;
- network flow evidence;
- screen-analysis summary;
- AI/policy result;
- unknown, degraded, and manual-required states.

## Assistant Requirements

Assistant UI must not write directly. It should show cited explanation, proposed
action, preview, required parent confirmation, and child-agent validation/result
state.

## Review Requirements

C/user visual review is required before marking a route UX-ready. Automated proof
must include Playwright route coverage, console/page-error checks, and screenshots
for changed product-critical surfaces.
