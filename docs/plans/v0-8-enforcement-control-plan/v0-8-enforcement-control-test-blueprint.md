# V0.8 Enforcement Control Test Blueprint

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Test Blueprint`
> Kind: test blueprint reference; read only when local expectations route here.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This is the companion requirement blueprint for the
[V0.8 Enforcement Control 20-Step Plan](v0-8-enforcement-control-20-step-plan.md).
The plan defines what to build. This blueprint defines the tests, fixtures,
proof gates, and quality bars required while building it.

## Scope

Included:

- Typed parent enforcement intents.
- Child-agent validation.
- Policy/evidence/action references.
- Adapter capability status.
- Scoped owned-process control and app time-limit proof.
- Managed browser session state and unmanaged browser fallback state.
- Network/domain report-only and manual-required states.
- Timer, rollback, restart recovery, approval, override, and audit events.
- Parent-visible service read models and portal consumption.
- Integrity heartbeat and permission-loss states.

Excluded until separately proved:

- Broad app blocking.
- Exact unmanaged browser URL control.
- Decrypted network inspection.
- Production notification delivery.
- Mobile enforcement parity.
- Stealth or persistence-hardening anti-tamper behavior.

## Core Test Principle

### ENF-TEST-001: No enforcement without policy decision

Requirement: Every action that changes device behavior must reference a typed
policy decision and evidence refs.

Proof: Contract and service tests reject actions with missing, malformed, stale,
wrong-device, or wrong-version policy decisions.

Acceptance: Portal clicks can create intents only; the child-device agent
decides whether an action is valid.

### ENF-TEST-002: Adapter capability is visible

Requirement: Every surface/action exposes capability status before claiming
support.

Proof: Matrix tests cover supported, unavailable, degraded, permission-required,
dependency-missing, scaffold, and manual-required states.

Acceptance: Parent UI can show why a control is unavailable instead of hiding or
faking success.

### ENF-TEST-003: Dry-run and enforcement are separate

Requirement: Dry-run preview must return the same decision shape but cannot call
an adapter.

Proof: Service tests assert no adapter action runs in dry-run and audit events
mark preview state.

Acceptance: Parent can preview without changing child-device behavior.

### ENF-TEST-004: Scoped process actions verify identity

Requirement: Owned-process control requires explicit process id and matching
identity.

Proof: Tests cover pid/name mismatch, already-exited process, unsupported host,
manual-required broad target, and successful scoped target where available.

Acceptance: Broad app blocking is not accidentally claimed by a narrow process
proof.

### ENF-TEST-005: Browser surfaces stay separated

Requirement: Managed session control, unmanaged process fallback, and exact URL
manual-required states must remain separate.

Proof: Contract/service tests reject exact URL action from unmanaged process
evidence and reject stale managed sessions.

Acceptance: Parent UI cannot imply exact page control from process detection.

### ENF-TEST-006: Network/domain remains honest

Requirement: Network/domain blocking must stay report-only/manual-required until
real adapter proof exists.

Proof: Matrix and proof tests fail if network/domain states are upgraded without
adapter evidence and documentation updates.

Acceptance: Network flow evidence can support policy preview without pretending
to block encrypted traffic or page content.

### ENF-TEST-007: Timers recover or degrade visibly

Requirement: Temporary limits must survive service restart or emit
recovery-needed/unavailable state.

Proof: Service tests cover create, extend, expire, cancel, rollback, restart
recovery, and missing-state recovery-needed.

Acceptance: Parent and child can see whether a time limit is active, expired, or
requires attention.

### ENF-TEST-008: Ask-parent is auditable

Requirement: Request, approval, denial, expiry, bonus time, and override must be
typed and journaled.

Proof: Tests cover approval success, expired approval, wrong child/device,
duplicate approval, denial, and override audit.

Acceptance: Parent actions cannot silently mutate policy or enforcement state.

### ENF-TEST-009: Integrity state is not anti-tamper proof

Requirement: Heartbeat, stale, stopped, removed, permission-loss, and outdated
states must be visible without claiming stealth or persistence hardening.

Proof: Contract/service tests cover each status and proof output labels
anti-tamper as design/proof-gated unless real platform proof exists.

Acceptance: The product is honest about agent health and removal limitations.

### ENF-TEST-010: Portal consumes real service state

Requirement: Portal UI may render only service-returned action states.

Proof: Playwright tests run against the real Rust service and assert control
clicks produce typed service output, not browser-local success.

Acceptance: UI proof supports the product path rather than a visual-only demo.

## Required Fixture Families

- Policy/action fixtures: valid allow, valid warn, valid time-limit, valid
  ask-parent, stale policy version, wrong-device, missing evidence, malformed
  target, and dry-run action.
- Capability fixtures: supported Windows process action, report-only browser
  exact URL, unmanaged process fallback, network/domain manual-required,
  platform unavailable, permission-required, and dependency missing.
- Timer fixtures: active timer, expired timer, cancelled timer, rollback token,
  restart recovered, and recovery-needed.
- Audit fixtures: action accepted, action rejected, adapter failed, rollback
  completed, approval granted, approval denied, approval expired, and override.
- Portal fixtures: observe-only, dry-run, time-limit active, ask-parent pending,
  adapter unavailable, manual-required, degraded, and proof-required state.

## Acceptance And Proof

- CI runs contract, Rust parity, service, proof-script, source-boundary, and
  no-test-double checks for changed paths.
- Portal changes include Playwright coverage against the real service path.
- Proof JSON distinguishes implemented, report-only, scaffold, unavailable,
  degraded, and manual-required states.
- Product docs and checklist rows are updated when any state moves.
- Manual proof records are required for privileged OS, browser, network, mobile,
  notification, or tamper claims that CI cannot exercise.

## Non-Claim Gate

The proof gate must fail or require explicit review if any change presents broad
app blocking, exact unmanaged URL blocking, network/domain blocking,
notification delivery, mobile parity, or tamper/uninstall protection as done
without named proof artifacts.
