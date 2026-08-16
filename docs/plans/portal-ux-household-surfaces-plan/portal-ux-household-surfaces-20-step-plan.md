# Portal UX Household Surfaces 20-Step Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces 20-Step Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This plan turns C-lane portal work into a coherent parent product program. It
keeps UI/UX ownership separate from child-agent runtime ownership.

This is a plan document only. It does not claim finished LAN, enforcement,
remote relay, notification delivery, mobile child-agent parity, or production
packaging unless those service/package artifacts exist.

Companion docs:
[Portal UX Household Surfaces Test Blueprint](portal-ux-household-surfaces-test-blueprint.md)
and [Portal UX Requirements Guide](ui-ux-requirements-guide.md).

## Product Boundary

- Owning feature: Parent portal shell and family setup/device roles.
- Secondary feature overlap: Remote/LAN/mobile platforms, policy/schedules,
  evidence store/query, reports/notifications/sync, and parent assistant.
- Main expectations: portal, family setup, LAN pairing, policy, real evidence
  proof.
- Product goal: make real child-agent and household state understandable and
  usable by a parent.
- Non-goals: backend authority, LAN scanning, OS enforcement, local AI execution,
  timers, package runtime, and hosted child-activity custody.

## 20-Step Plan

1. Establish a service-backed portal shell.
   Navigation, route ids, DOM ids, display tokens, and route structure should
   align with domain packages. The shell should show connection, source, and
   selected-device state without relying on portal-only success.

2. Build household first-run state.
   Show household setup progress, child profiles, parent role, invite/recovery
   state, and what remains incomplete before rules or enforcement are enabled.

3. Build device inventory as the product center.
   Devices should show role, source, evidence, trust, assignment, confirmation,
   stale/offline state, route state, and whether the row is controllable.

4. Add selected-device context everywhere.
   Policy, activity, AI, reports, and account surfaces should make it obvious
   which child/device/route is selected and whether the data is live, stale,
   cached, LAN, relay, parent-owned, or unavailable.

5. Build the policy control center.
   Policy UI should make parent rules scannable by child, device, target,
   schedule, mode, action, proof state, and last result. It authors typed intents
   and previews before action.

6. Build schedules, budgets, and templates.
   Schedules should support school, bedtime, free time, app/game limit, website
   limit, bonus time, and exception states as a parent workflow, not raw form
   fields.

7. Build ask-parent and approval surfaces.
   Pending requests, approvals, denials, bonus time, expiry, and override history
   should be visible as actionable state with child/device context.

8. Build activity evidence and diagnostics.
   Activity views should cite source evidence, freshness, custody, confidence,
   unknown state, and degraded state without overclaiming page content or AI
   certainty.

9. Separate browser, app/game, network, and screen surfaces.
   Exact browser URL evidence, unmanaged browser detection, app/game session
   evidence, network flow summaries, and screen-analysis summaries must have
   different labels and constraints.

10. Consume LAN pairing/discovery state.
    LAN B-lane read models should render discovered, assigned, confirmed,
    ignored, stale, offline, router/infrastructure, and agent-connected states
    without C inventing LAN data.

11. Build assistant action previews.
    Assistant responses should cite evidence, show proposed rule/report/action
    previews, require parent confirmation, and show child-agent validation
    outcome when implemented.

12. Build reports, notifications, and custody surfaces.
    Reports and notifications should show evidence source, delivery status,
    minimal payload status, quiet hours/escalation where available, retention,
    export, and delete state.

13. Build degraded, empty, stale, and error states.
    Empty routes should guide the next valid action. Stale, offline, service
    unavailable, permission-limited, unsupported, and manual-required states
    should not resemble successful data.

14. Build audit history and copy/debug.
    Recent actions, decisions, service events, and debug copy should be concise,
    redacted, and useful for support or handoff.

15. Add accessibility, responsive, and keyboard behavior.
    Core flows should be usable at desktop and mobile widths, with stable
    dimensions, clear focus, no overlap, and no text clipping.

16. Add a no-fake-data adapter boundary.
    UI fixtures may exist for design/testing, but runtime product routes must
    validate service payloads and label fixture/demo state explicitly.

17. Add Playwright screenshot proof.
    Browser proof should run real service flows, capture relevant route
    screenshots, check console/page errors, and include desktop/mobile coverage.

18. Prepare parent mobile shell readiness.
    The portal design should reuse source-state, selected-device, and action
    patterns for future parent mobile while not claiming child mobile support.

19. Keep product docs and checklist synchronized.
    Feature docs and checklist rows should reflect UX status, runtime
    dependency, proof level, and remaining gaps.

20. End with a manual user review gate.
    C/user visual review is a formal gate for look, feel, layout, interaction,
    and ergonomics before claiming UX readiness.

## Implementation Order

1. Stabilize shell, route context, and source/selected-device patterns.
2. Finish devices and household setup as the product center.
3. Build policy and approval flows on service-backed states.
4. Add activity/evidence diagnostics and assistant/report previews.
5. Add degraded/error states and copy/debug.
6. Run Playwright/browser proof and manual visual review.

## Validation Expectations

- Type/lint/contract checks for touched portal/domain paths.
- Playwright proof against the real Rust service path.
- Browser console and page error checks on touched routes.
- Desktop and mobile width checks for changed layouts.
- Product docs/checklist updates when UX status or product claims change.

## Open Product Questions

- Which first-run setup screen is the real first viewport for manual testing?
- Which policy builder shape should become the canonical parent workflow?
- Which C visual decisions should become reusable components before more routes
  are wired?
- Which route needs the first user manual review gate before more wiring?
