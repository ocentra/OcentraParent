# V0.8 Enforcement Control Plan

This folder is the single working plan location for V0.8 enforcement,
product-control action states, adapter proof, integrity state, and parent-visible
control readiness.

- [V0.8 Enforcement Control 20-Step Plan](v0-8-enforcement-control-20-step-plan.md)
- [V0.8 Enforcement Control Test Blueprint](v0-8-enforcement-control-test-blueprint.md)

The rule remains:

```text
Portal requests. Agent validates. Adapter proves.
```

## Where We Are

- `origin/main` already has V0.8 enforcement contracts, audit, rollback,
  capability status, timer/recovery, approval/override audit references, browser
  intervention boundary states, and product-control action states.
- `codex-a` has a PR-ready branch for the product-control runtime vertical.
  Primary still owns branch review, PR creation, CI, merge timing, and post-merge
  sync.
- Current proof separates scoped owned-process control and app time-limit
  behavior from broad app blocking, network/domain blocking, exact URL blocking,
  notification delivery, and tamper/uninstall protection.
- Broad platform claims remain manual-required until adapter-specific proof
  exists.

## Where We Want To Be

Ocentra Parent needs a production-credible V0.8 enforcement control subsystem
that:

- accepts only typed parent policy or approval intents;
- validates policy, target, schedule, actor, device, route, evidence, and adapter
  capability inside the child-device agent;
- exposes one shared parent-visible action-state model for observe, warn,
  dry-run, time-limit, scoped process block, ask-parent, override, report-only,
  unavailable, degraded, and manual-required states;
- keeps managed browser, unmanaged browser, native app/game, and network/domain
  surfaces separate;
- journals every action, rejection, timer, rollback, approval, and unavailable
  state;
- gives portal, policy, activity, reports, and assistant surfaces the same
  service-backed truth;
- records honest proof levels instead of upgrading scaffold or report-only states
  into product-ready enforcement claims.

## Coverage Check Against Product Docs

This plan was grounded in:

- `docs/features/browser-web-control.md`
- `docs/features/app-game-control.md`
- `docs/features/network-domain-control.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/features/enforcement-integrity-tamper.md`
- `docs/expectations/enforcement.md`
- `docs/expectations/policy.md`
- `docs/expectations/browser-evidence.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/network-flow-evidence.md`
- `docs/expectations/tamper-uninstall-protection.md`

The repeated theme is not "add a blocker." The required product shape is:
evidence-backed policy decision, child-agent validation, adapter capability,
audited result, timer/rollback behavior, and honest unavailable/manual-required
states.

## Parallel Coordination Rules

- A owns this V0.8 enforcement/product-control program until primary merges or
  retargets it.
- Do not split future V0.8 work into tiny unconnected tasks. Use the workpacks
  below as the durable backlog and report which workpacks changed.
- Contract/domain changes happen before Rust protocol/service behavior.
- Portal changes consume service-backed action states only; the portal never
  evaluates policy, runs timers, or executes enforcement.
- Broad app, browser exact URL, network/domain, notification, and
  tamper/uninstall claims stay manual-required until the matching proof exists.
- Every `DONE` report must name workpacks, touched paths, validation, product-doc
  updates, proof artifacts, and known non-claims.

## Workpack Checklist

| Step | Workpack                                                                                       | Target State                                                                                      |
| ---- | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| 01   | [Contract boundary and Effect schemas](workpacks/01-contract-boundary-and-effect-schemas.md)   | Enforcement values crossing TS/Rust/service/portal are schema-backed before runtime use.          |
| 02   | [Policy decision evidence references](workpacks/02-policy-decision-evidence-references.md)     | Every action traces to parent rules, schedules, decisions, and evidence refs.                     |
| 03   | [Adapter capability matrix](workpacks/03-adapter-capability-matrix.md)                         | Each platform/surface says supported, unavailable, degraded, manual-required, or scaffold.        |
| 04   | [Owned-process time limit](workpacks/04-owned-process-time-limit.md)                           | Scoped process control remains narrow, auditable, and recoverable.                                |
| 05   | [App and game session handoff](workpacks/05-app-game-session-handoff.md)                       | App/game limits consume stored session evidence, not UI guesses.                                  |
| 06   | [Managed browser session control](workpacks/06-managed-browser-session-control.md)             | Managed browser control stays inside the managed profile/session boundary.                        |
| 07   | [Unmanaged browser fallback](workpacks/07-unmanaged-browser-fallback.md)                       | Unmanaged browser handling is process-only fallback, not exact URL proof.                         |
| 08   | [Network/domain report-only boundary](workpacks/08-network-domain-report-only-boundary.md)     | Network/domain blocking remains honest until real adapter proof exists.                           |
| 09   | [Timer recovery and rollback](workpacks/09-timer-recovery-and-rollback.md)                     | Temporary limits survive restart or degrade visibly with audit.                                   |
| 10   | [Parent approval and override](workpacks/10-parent-approval-override.md)                       | Requests, approvals, denials, expiry, and overrides are typed and audited.                        |
| 11   | [Audit and journal events](workpacks/11-audit-journal-events.md)                               | Action and failure history is persisted and queryable.                                            |
| 12   | [Child-facing status and reasons](workpacks/12-child-facing-status-and-reasons.md)             | Child-visible messages are reasoned, local, and not generated by portal code.                     |
| 13   | [Service read models and API](workpacks/13-service-read-models-and-api.md)                     | Parent surfaces receive one service-backed product-control read model.                            |
| 14   | [Portal control state consumption](workpacks/14-portal-control-state-consumption.md)           | Portal renders action states without pretending to enforce.                                       |
| 15   | [Integrity heartbeat and permission loss](workpacks/15-integrity-heartbeat-permission-loss.md) | Parent can see running, stale, degraded, permission-limited, stopped, or unsupported agent state. |
| 16   | [Tamper/uninstall non-claim design](workpacks/16-tamper-uninstall-non-claim-design.md)         | Anti-tamper remains design/proof-gated, non-stealth, and honestly labeled.                        |
| 17   | [Cross-platform unavailable states](workpacks/17-cross-platform-unavailable-states.md)         | Windows/macOS/Linux/Android/iOS claims are separated by real capability proof.                    |
| 18   | [Proof command and matrix](workpacks/18-proof-command-and-matrix.md)                           | A single proof command writes implemented/scaffold/manual-required states.                        |
| 19   | [Playwright and UI proof](workpacks/19-playwright-ui-proof.md)                                 | Parent-visible enforcement state is tested through the real service path.                         |
| 20   | [Rollout docs and CI/PR gate](workpacks/20-rollout-docs-ci-pr-gate.md)                         | Feature docs, checklist, README, PR body, and CI proof stay synchronized.                         |

## Handoff Rule

A should continue from the current PR-ready branch, pull/rebase latest `main`
after primary review or merge requests, and use this folder as the full V0.8
backlog. Primary should not replace this program with small one-off tasks unless
a workpack is explicitly blocked and needs a narrow fix.
