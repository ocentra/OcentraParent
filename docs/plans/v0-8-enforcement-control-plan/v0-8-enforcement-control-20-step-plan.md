# V0.8 Enforcement Control 20-Step Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control 20-Step Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This plan turns the V0.8 enforcement and product-control work into one concrete
program. It keeps enforcement separate from evidence capture, policy authoring,
LAN discovery, AI model execution, and portal layout.

This is a plan document only. It does not claim broad app blocking,
network/domain blocking, exact URL blocking, notification delivery, mobile
parity, or tamper/uninstall protection until proof artifacts exist.

Companion requirement doc:
[V0.8 Enforcement Control Test Blueprint](v0-8-enforcement-control-test-blueprint.md).

## Product Boundary

- Owning feature: Enforcement, integrity, and tamper.
- Secondary feature overlap: Browser and web control, app and game control,
  network and domain control, and policy/schedules/approvals.
- Main expectations: enforcement, policy, browser evidence, app/game evidence,
  network flow evidence, and tamper/uninstall protection.
- Product goal: execute or honestly decline parent-authored control decisions
  through the child-device agent with audit, timer, capability, rollback, and
  parent-visible status.
- Non-goals: portal-side enforcement, stealth, privilege escalation, decrypted
  packet inspection, AI-direct enforcement, exact unmanaged URL blocking, billing
  logic in adapters, and broad platform claims without proof.

## 20-Step Plan

1. Establish the enforcement contract boundary.
   Define or extend TypeScript domain/protocol contracts for intents, actions,
   results, capability status, audit events, timers, approvals, overrides,
   integrity state, and product-control read models before runtime code consumes
   them.

2. Ground actions in policy decisions and evidence references.
   Every enforceable action must reference a parent rule, policy decision,
   target, schedule or time budget, evidence refs, and optional AI result refs.
   Missing evidence should produce unknown, ask-parent, observe-only, or
   unavailable state, not guessed enforcement.

3. Build the adapter capability matrix.
   Split platform, adapter kind, surface, action, permission state, dependency
   state, degraded reason, and proof level. The matrix is the source for parent
   UI claims and proof JSON.

4. Keep owned-process time-limit control narrow.
   Scoped process termination or time-limit behavior must require explicit pid
   and process identity checks, record already-exited and mismatch outcomes, and
   leave broad app blocking as manual-required.

5. Wire app/game session handoff.
   App/game control consumes stored app/game session summaries and evidence ids.
   It must distinguish known app, known game, launcher, unknown process,
   background process, foreground session, and report-only states.

6. Wire managed browser session control.
   Managed browser control applies only to an Ocentra-launched managed profile or
   session with a validated bridge/session id. Exact URL action remains
   manual-required until the managed active-tab adapter proves it.

7. Wire unmanaged browser fallback.
   Unmanaged browser handling is process-only detection, warning, possible-bypass
   status, or scoped terminate where proved. It must never claim exact URL, title,
   tab, history, or page content.

8. Preserve the network/domain report-only boundary.
   Network/domain observation can inform policy preview and manual-required
   states. Blocking stays unavailable/manual-required until a real OS adapter
   proves domain or network action without decrypted-content claims.

9. Add timer, recovery, and rollback behavior.
   Temporary limits need create, extend, expire, cancel, recover, and rollback
   events. Restart must restore active state from journal/query data or emit a
   recovery-needed status.

10. Add ask-parent approvals and overrides.
    Child requests, parent approvals, denials, bonus time, expiry, and overrides
    must be typed, audited, and validated on the child-device agent before action.

11. Persist action, rejection, and failure audit events.
    Every allow, warn, block, time-limit, ask-parent, rollback, unavailable,
    invalid, stale, wrong-device, or dry-run state needs a durable event with
    references.

12. Add child-facing status and reason codes.
    The child device should expose stable local reason text references such as
    time limit reached, ask parent, blocked by parent rule, adapter unavailable,
    or allowed by schedule.

13. Add service read models and API responses.
    Parent surfaces should read one normalized product-control state, including
    target, action, result, capability, proof level, source evidence, route, and
    next action.

14. Wire portal consumption without portal authority.
    Portal controls author typed intents and render returned state. They do not
    evaluate policy, create timers, run OS commands, or mark success locally.

15. Add integrity heartbeat and permission-loss states.
    Parent-visible health should distinguish running, stale, offline, stopped,
    removed, permission-denied, outdated, unsupported, and degraded where platform
    proof permits.

16. Keep tamper/uninstall work design-gated.
    Tamper/uninstall protection needs explicit product/security design before any
    hardening behavior. Heartbeat alone is not anti-tamper proof.

17. Add cross-platform unavailable/scaffold states.
    Windows, macOS, Linux, Android, iOS, and web must each state what is
    implemented, scaffold-only, unavailable, degraded, or manual-required for
    every control surface.

18. Add proof command and proof matrix output.
    A composed proof command should write JSON for implemented, scaffold,
    unavailable, degraded, report-only, and manual-required states across V0.8
    product-control surfaces.

19. Add Playwright and browser-visible proof.
    UI proof should click parent controls against the real Rust service, confirm
    service-returned state, and fail on browser console/page errors for touched
    routes.

20. Close with rollout docs, CI, and PR review.
    Feature docs, checklist rows, module READMEs, proof outputs, validation, PR
    body, and merge notes must all agree on what changed and what remains not
    claimed.

## Implementation Order

1. Review and merge the current PR-ready product-control runtime branch after
   local diff review and acceptable validation.
2. Rebase any follow-up V0.8 work on latest `main`.
3. Finish service read models and proof command coverage before visual-only
   portal claims.
4. Add portal rendering only for states returned by the service.
5. Add integrity/permission states after contract and protocol parity.
6. Defer tamper/uninstall hardening until the security/product design is written.

## Validation Expectations

- Contract tests for valid and invalid intents, actions, results, capability
  statuses, timers, approvals, overrides, and audit events.
- Rust parity tests for every Rust-crossing field and enum.
- Service tests for dry-run, allow, warn, time-limit, scoped block, ask-parent,
  unavailable, mismatch, restart recovery, expiry, rollback, and rejection.
- Proof script that writes implemented, report-only, scaffold, unavailable,
  degraded, and manual-required states.
- Portal Playwright proof only after service-backed states exist.
- Manual host/device proof before broad privileged platform claims.

## Open Product Questions

- Which managed browser exact active-tab action is the first acceptable proof
  target?
- Which network/domain adapter is acceptable for a first real block claim?
- What parent-facing child request UI is required before ask-parent can move
  from audit-reference proof to product flow?
- What security review must precede any tamper or uninstall protection behavior?
