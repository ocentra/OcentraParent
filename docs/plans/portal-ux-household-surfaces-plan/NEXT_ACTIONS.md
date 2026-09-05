# Portal UX Household Surfaces Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
4. Open that workpack and exact checklist rows only.
5. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by unchecked boxes

- [05 Policy Authoring Control Center](workpacks/05-policy-authoring-control-center.md): 5 open of 5 boxes; current exact next gap is preview-envelope bridge plus parent-confirmation action wiring.
- [06 Schedules, Time Budgets, And Templates](workpacks/06-schedules-time-budgets-and-templates.md): 5 open of 5 boxes.
- [07 Parent Requests And Approvals](workpacks/07-parent-requests-and-approvals.md): 5 open of 5 boxes.
- [08 Activity Evidence And Diagnostics](workpacks/08-activity-evidence-and-diagnostics.md): 5 open of 5 boxes.
- [09 Browser, App, And Network Surfaces](workpacks/09-browser-app-and-network-surfaces.md): 5 open of 5 boxes.
- [11 Assistant Action Preview Flow](workpacks/11-assistant-action-preview-flow.md): 5 open of 5 boxes.
- [12 Reports, Notifications, And Custody](workpacks/12-reports-notifications-and-custody.md): 5 open of 5 boxes.
- [13 Degraded, Empty, Stale, And Error States](workpacks/13-degraded-empty-stale-and-error-states.md): 5 open of 5 boxes.
- [14 Audit History And Copy/Debug](workpacks/14-audit-history-and-copy-debug.md): 5 open of 5 boxes.
- [15 Accessibility, Responsive, And Keyboard UX](workpacks/15-accessibility-responsive-keyboard-ux.md): 5 open of 5 boxes.
- [17 Playwright Screenshot Proof](workpacks/17-playwright-screenshot-proof.md): 5 open of 5 boxes.
- [18 Parent Mobile Shell Readiness](workpacks/18-parent-mobile-shell-readiness.md): 5 open of 6 boxes.
- [19 Product Docs And Checklist Sync](workpacks/19-product-docs-and-checklist-sync.md): 5 open of 5 boxes.
- [20 Manual User Review Gate](workpacks/20-manual-user-review-gate.md): 5 open of 5 boxes.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Classify selected owner/proof family through `WORKPACK_FAMILIES.md` when unclear.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
- [ ] Keep portal projection claims separate from sibling domain/runtime readiness.

## Production-code audit boundary (2026-08-16)

The shipped entrypoint and typed host bridge are present. The audit found no
missing portal-only production slice with an independently owned backend
authority:

- WP01-WP04 and WP10 have real bridge/read-model consumption, but account,
  setup, device-trust, and physical LAN authority remain sibling-owned.
- WP05 already has the narrow real portal staging/cancel/confirm path and Rust
  authoring boundary. Do not duplicate it or claim policy mutation, rollback,
  delivery, or enforcement from the panel.
- WP06-WP09 and WP11-WP15 are mixed route/read-model seams with missing domain
  providers, durable custody, action authority, or degraded-state ownership.
- WP16 is closed for the bounded no-fake-data projection and retained proof;
  installer, updater, rollback, signing, store, transport, and authenticated
  remote-session execution remain sibling-owner non-claims.
- WP17-WP20 are proof, mobile scaffold, documentation, and manual-review work,
  not portal production-code gaps.

No fake store, fixture, static success, generic JSON bridge, or proof adapter
was added. The next legal code change must come with a named sibling service
read model/action owner and a shipped portal caller; otherwise preserve the
typed unavailable/manual-required states.
