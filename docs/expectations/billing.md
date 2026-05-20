# Billing And Subscription Expectations

Billing features should gate paid product value without breaking local child safety irresponsibly.

Billing is not a child-device safety engine. It must stay outside capture, journal, local AI decisioning, policy evaluation, timers, and enforcement adapters. Paid product features can be entitlement-gated, but the system must describe how local safety behaves when billing checks are delayed, unavailable, expired, or disputed.

## Roadmap Scope

V5 parent policy product may need entitlement-aware UI for family setup, device limits, reports, and paid policy management.

V7 introduces subscription and monetization: plans, trials, Stripe billing, device limits, subscription status sync, and admin/support flows.

Billing must not be introduced into V0.6 through V0.8 local AI, policy, or enforcement internals.

## Parent Outcome

- Parent can understand plan, trial, device-limit, renewal, grace, and entitlement state.
- Parent can see which paid features are available and why a paid feature is locked.
- Parent can keep critical local safety behavior in a documented degraded mode if billing status cannot be checked.
- Parent can resolve billing issues without losing access to local evidence export and safety-critical audit history.

## Child-Device Outcome

- Child-device capture, evidence storage, local safety decisioning, and currently active critical safety behavior do not silently stop because Stripe or cloud billing is unavailable.
- The child-device agent receives only typed entitlement state or local grace state; it does not call Stripe directly.
- Device-limit and paid-feature decisions are auditable.

## Platform Scope

- Web/mobile parent surfaces may host billing UI and account management.
- Cloud/control-plane services may synchronize subscription and entitlement state when those services exist.
- Child-device agents consume signed or schema-valid entitlement snapshots only after the billing contract exists.
- Offline/local-first behavior must be explicit for Windows first and then proven per platform.

## Data Scope

Billing data may include:

- Plan id, subscription status, trial state, grace state, renewal status, cancellation state, device limits, feature entitlements, family/account reference, and billing audit events.
- Stripe customer/subscription/payment references stored behind a backend boundary.

Billing data must not include:

- Raw child activity evidence.
- Local AI prompts or model outputs.
- Enforcement adapter internals.
- Secrets in portal source, child-device source, logs, docs examples, or committed config.

## Contract Boundary

Expected contract families are:

- `Plan`: plan id, display text token, feature entitlement references, device limit, retention/export allowance, price reference, and active state.
- `EntitlementSnapshot`: family reference, plan reference, feature flags, device limits, generated time, expiry, source, and signature or validation metadata when applicable.
- `SubscriptionStatus`: trialing, active, past due, cancelled, expired, grace, unknown, or unavailable.
- `BillingSyncEvent`: previous status, next status, source, actor, timestamp, and provider reference.
- `DeviceLimitDecision`: requested device reference, entitlement snapshot reference, allowed/denied state, reason code, and audit reference.
- `BillingFailureState`: provider unavailable, network unavailable, stale snapshot, payment required, account mismatch, or validation failed.

Stripe-specific API shapes stay behind the billing backend boundary. Runtime safety modules receive only Ocentra-owned contracts.

## Entitlement Boundaries

Entitlements may gate:

- Number of child devices.
- Cloud relay, remote access, and multi-device sync once those features exist.
- Advanced reports, parent assistant, long-window summaries, and exports.
- Non-critical convenience features.

Entitlements must not silently disable:

- Local evidence capture that is already installed and configured.
- Local journal integrity and audit history.
- Existing local policy decisions needed for child safety during a billing outage.
- Parent visibility into why billing state changed.

Any feature that can be disabled for billing must define its degraded local behavior and parent-facing status before implementation.

## Failure Behavior

- Stripe unavailable: keep last known entitlement snapshot until expiry/grace rules decide next state.
- Cloud unavailable: child-device agent continues local safety behavior from local validated state.
- Snapshot expired: enter a documented grace, restricted, or local-only mode; do not silently erase safety behavior.
- Payment past due: surface parent-visible status and retain local evidence/audit access required for safety and support.
- Device limit exceeded: deny new paid-device activation through typed decision and leave existing local safety behavior explicit.
- Billing data mismatch: reject the billing update and journal/report validation failure.

## Expected Deliverables

- Plan contract.
- Entitlement contract.
- Stripe boundary.
- Billing status sync.
- Trial state.
- Device limit policy.
- Grace/failure behavior.

## Acceptance

- Paid features check entitlements through typed contracts.
- Billing failures are visible.
- Local safety behavior degrades deliberately when billing cannot be checked.
- No billing secret is committed or exposed to the portal.
- Billing state changes are auditable.

## Validation Gates

- TypeScript schema tests prove valid/invalid plans, entitlement snapshots, subscription statuses, billing sync events, device-limit decisions, and failure states.
- Backend tests prove Stripe references are isolated behind the billing boundary before provider code exists.
- Child-device tests prove safety modules consume typed entitlement snapshots only and do not import billing provider code.
- Failure tests cover unavailable provider, stale snapshot, expired trial, grace mode, payment required, and device-limit denial.
- Secret scanning and repo security gates prove no Stripe or billing secrets are committed.
- Portal tests, when billing UI exists, prove locked/available states are derived from typed entitlement contracts.

## Non-Goals

- Do not put Stripe logic inside capture, journal, local AI, policy, timer, or enforcement modules.
- Do not make payment failures silently disable critical local safety behavior.
- Do not add billing provider code before plan and entitlement contracts exist.
- Do not send child activity evidence to a billing provider.
- Do not represent scaffolded billing as production subscription support.

## Done Signal

A paid product capability is gated by typed entitlements, failure behavior is explicit, and billing concerns stay outside core child-device evidence and enforcement modules.
