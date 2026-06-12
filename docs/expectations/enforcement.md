<!-- agent-capsule -->

> Agent Capsule
> Doc: Enforcement Feature Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Enforcement Feature Expectations

Enforcement features change device behavior and therefore need a higher bar.

Only the child-device agent performs enforcement. Parent surfaces may send typed rules, approvals, overrides, or requests, but the agent validates the request and executes through a platform adapter.

Enforcement exists to carry out parent-authored household rules. Ocentra provides
the adapter, validation, audit trail, rollback behavior, and clear status; it
does not silently decide which lawful household activities should be blocked.

## Roadmap Scope

V0.8 starts enforcement after local evidence, policy decisions, and AI decision contracts are trusted. Enforcement must begin with simple, scoped local decisions and must produce auditable evidence.

V5 parent policy product may make enforcement easier to configure, but it still does not move enforcement into the portal.

Billing in V7 may gate paid product value, but billing must stay outside core safety enforcement logic.

## Parent Outcome

- Parent can tell what was blocked, warned, time-limited, allowed, or sent for approval.
- Parent can see the policy decision, local AI reference when applicable, evidence references, adapter result, timer state, and rollback/unavailable state.
- Parent can override or approve through typed intents that are validated by the child-device agent.
- Parent can distinguish "would enforce in dry-run" from "actually enforced".
- Parent can see which parent-authored rule caused an enforcement action.

## Child-Device Outcome

- The child-device agent executes only schema-valid policy decisions.
- Enforcement is scoped to configured targets and time windows.
- Every action, failure, expiry, rollback, and parent override writes an auditable event.
- The service remains uninstallable, debuggable, and honest about unavailable capabilities.

## Platform Scope

- Windows is first for enforcement adapters.
- Windows enforcement may start with one narrow mode, such as process block/terminate, domain/network block, temporary block, or timeout, depending on the preceding policy slice.
- Managed-browser-only mode may terminate or block browser-like processes that
  are outside the Ocentra-managed browser boundary.
- Game/app time-limit enforcement may terminate or block a native app/game
  process after a typed policy decision references stored app/game session
  evidence.
- Network enforcement may block or terminate only after a typed policy decision
  references stored network flow evidence or a network digest. It must not depend
  on AI-invented packet or content claims.
- Screen-derived enforcement may act only after a typed policy decision
  references a schema-valid local screen-analysis summary. It must not act on raw
  AI text or retained screenshots.
- macOS, Linux, Android, and iOS require platform-specific adapter contracts and proof before claiming enforcement.
- Web is not an enforcement platform. Web can show status and author intents only.

## Data Scope

Enforcement input may include:

- Typed policy decision id and version.
- Evidence references and local AI result references used by the policy decision.
- Target reference, adapter kind, enforcement mode, timer/expiry, and rollback token.
- Parent approval or override reference where applicable.
- Platform capability status.
- For unmanaged browser enforcement: detected process id, process name,
  executable path/signature/hash where available, managed-browser session id if
  relevant, and possible-bypass reason.
- For game/app enforcement: app/game session id, process id, process name,
  executable path/signature/hash where available, running time, foreground time,
  parent rule reference, and permission-request state when relevant.
- For network enforcement: flow summary id, process reference, destination
  reference, protocol, domain/IP attribution status, VPN/proxy/tunnel indicator
  where available, and parent rule reference.
- For screen-derived enforcement: screen-analysis id, source evidence refs,
  category/risk signal, confidence, image deletion state, and parent rule
  reference.

Enforcement output must include:

- Enforcement action event.
- Adapter result: succeeded, failed, unavailable, expired, rolled back, superseded, or no-op.
- Reason code and user-visible explanation reference.
- Evidence, policy, AI, and parent-action references.
- Timer event for temporary blocks and time budgets.
- For unmanaged browser enforcement: whether the process was terminated,
  blocked, already exited, unavailable, or left running because policy is in
  observe-only mode.
- For game/app enforcement: child-facing result text reference such as stopped by
  parent policy, ask parent for permission, or time limit reached.
- For network enforcement: whether the flow/process/domain was blocked,
  terminated, unavailable, already ended, or left running because policy is in
  observe-only mode.
- For screen-derived enforcement: whether action was taken, skipped for low
  confidence, degraded to ask-parent, or left in observe-only mode.
- Recovery state when the adapter could not enforce or had to rollback.

## Trust Boundary

- Parent portal actions are intents, not authority by themselves.
- The child-device agent validates rule version, device identity, child profile, target, schedule, policy decision, and adapter capability before enforcing.
- Local AI output cannot directly call an enforcement adapter.
- Enforcement adapters cannot invent policy decisions.
- Adapter code stays platform-specific behind a shared interface.

## Contract Boundary

Expected contract families are:

- `EnforcementIntent`: source, policy decision reference, target reference, requested action, parent approval reference when needed, and idempotency key.
- `EnforcementAction`: adapter kind, platform, target, action, timer/expiry, reason code, policy decision reference, AI result reference, evidence references, and dry-run flag.
- `EnforcementResult`: status, adapter result code, started/completed time, rollback token, unavailable reason, and next check time when retryable.
- `EnforcementAuditEvent`: action reference, result reference, policy version, evidence references, parent override reference, actor/source, and journal sequence.
- `EnforcementCapabilityStatus`: platform, adapter kind, permission state, installed dependency state, degraded reason, last checked time, and supported actions.
- `EnforcementTimerEvent`: timer created, extended, expired, cancelled, rollback requested, rollback completed, or unavailable.

Rust protocol parity is required for every shape the service sends, receives, journals, or exposes to the portal.

## Dry-Run Before Enforcement

Before an adapter can change device behavior:

- The policy evaluator must produce the same typed decision shape in dry-run.
- The adapter must expose capability status.
- The portal must label preview state as dry-run.
- Tests must prove no adapter action executes in dry-run.
- The journal must record preview decisions separately from actual enforcement events.

## Failure Behavior

- Invalid or stale policy decision: reject and journal the rejection.
- Adapter unavailable: return unavailable, keep local evidence capture running, and show parent-visible status.
- Partial adapter success: journal partial result and rollback plan where possible.
- Timer failure: journal failure and degrade to explicit unavailable or ask-parent state.
- Parent approval unavailable: keep local decision pending or degrade according to policy expiry; do not silently allow or block unless a deterministic rule says so.
- Service restart: restore active timers and current enforcement state from journal/query store or emit recovery-needed status.
- Billing unavailable: continue critical local safety behavior according to documented grace rules; do not silently disable enforcement.

## Expected Deliverables

- Adapter boundary per platform.
- Agent-side authorization for parent-originated requests.
- Explicit policy decision input.
- Local AI output reference when the decision came from the AI safety evaluator.
- Enforcement action event.
- Reason code.
- Evidence reference.
- Timer/expiry behavior for temporary blocks and time limits.
- Manual override or safe rollback path.
- Clear status when enforcement capability is unavailable.

## Acceptance

- Enforcement acts only after a typed policy decision.
- Portal-originated actions are treated as intents until validated by the child-device agent.
- Every action is journaled.
- Parent can see what happened and why.
- Failure to enforce is reported.
- Time-limited blocks expire or unblock through a typed timer path.
- Enforcement tests cover allowed, blocked, timeout, ask-parent, unavailable, expiry, and rollback paths where feasible.

## Validation Gates

- Contract tests prove valid/invalid enforcement intents, actions, results, audit events, capability statuses, and timer events.
- Rust parity tests prove exact platform adapter payload field names and enum values.
- Adapter tests exercise real platform boundaries where feasible; unsupported platform tests must prove honest unavailable status.
- Integration tests use real policy decisions and stored evidence references.
- Timer tests cover create, expire, restart recovery, cancel, and rollback paths.
- Portal E2E tests, when UI exists, prove it sends typed intents and never runs enforcement itself.
- Dev builds remain uninstallable and debuggable.

## Non-Goals

- Do not add stealth behavior.
- Do not add anti-tamper behavior.
- Do not add privilege escalation.
- Do not execute enforcement, timers, rollback, or scripts from the parent portal.
- Do not claim persistence-hardening without explicit product/security design.
- Do not block without an auditable policy decision.
- Do not block from a category label alone without a matching parent-authored
  rule.
- Do not put Stripe or billing provider logic inside enforcement adapters.

## Done Signal

The feature can enforce one clearly scoped typed decision, including a local-AI-derived block or timer decision, report success/failure through typed events, and leave an auditable journal trail.
