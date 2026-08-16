<!-- agent-capsule -->

> Agent Capsule
> Doc: Billing And Subscription Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Billing And Subscription Expectations

Billing features should gate paid product value without breaking local child
safety irresponsibly. Billing and subscription systems are allowed Ocentra-hosted
services, but they are not child-activity data systems.

Billing is not a child-device safety engine. It must stay outside capture,
journal, local AI decisioning, policy evaluation, timers, and enforcement
adapters. Paid product features can be entitlement-gated, but the system must
describe how local safety behaves when billing checks are delayed, unavailable,
expired, or disputed.

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
- Download/update/license audit references that contain no child activity
  evidence.

Billing data must not include:

- Raw child activity evidence.
- Generated child activity reports.
- Browser URL history, screen evidence, app/game sessions, or network flow
  details.
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

Current endpoint proof: `billing-account-endpoint-contract-proof` defines
endpoint-domain route ids, API paths, headers, query params, and
contract-version labels for account status, plan/entitlement snapshot,
subscription status, device-limit decision, and account download/update/status
surfaces. The proof is route contract only; it does not add Stripe SDK code,
billing provider logic, an account backend, portal UI, updater runtime, or
child-activity custody.

Current entitlement proof: `billing-entitlement-contract-proof` defines
parent-domain contracts for plan entitlement rows, entitlement snapshots,
subscription status sync events, device-limit decisions, billing failure states,
parent-visible degraded/local-only status, retained evidence-export access, and
explicit non-claims for Stripe SDKs, provider backends, provider token custody,
child-activity custody, safety shutdown, and portal UI. The proof does not add
provider integration, account storage, entitlement signing/runtime delivery,
portal billing UI, or child-device entitlement consumption.

Current subscription/device-limit/failure proof:
`billing-subscription-device-limit-failure-proof` extends the parent-domain
contract with subscription status proof rows for trialing, active, past-due,
expired, grace, and unavailable states; device-limit decisions that deny new
activation at plan capacity while allowing trusted existing-device grace/manual
states; and failure states for provider unavailable, network unavailable, stale
snapshot, payment required, account mismatch, and validation failed. The proof
keeps evidence export retained, parent resolution visible, existing local safety
continuing, and child-activity custody excluded.

Current account runtime boundary proof:
`billing-account-runtime-boundary-proof` extends the parent-domain billing
contract with account runtime boundary rows for account status available,
backend unavailable, provider unavailable, and entitlement signing
manual-required states. The proof keeps Stripe/provider secrets absent from
app/source/docs, provider references backend-only, portal billing UI
unimplemented, child-device entitlement consumption unimplemented,
child-activity custody excluded, and parent-visible local-safety fallback
required when backend/provider state is unavailable.

Current public release/status proof: `production-release-public-status-proof`
adds parent-domain readiness rows for public account status and subscription
status surfaces as route-contract/manual readiness only. It keeps account
backend runtime, billing provider runtime, public website runtime, provider
secrets, child-device consumption, child-activity custody, production
publishing, and support backend upload unimplemented or unclaimed.

Current public runtime handoff proof:
`production-release-public-runtime-handoff-proof` adds parent-domain handoff rows
for public account status and subscription status plus backend adapter rows for
account backend and billing provider runtime. It keeps account backend runtime,
billing provider runtime, provider secrets, child-device consumption,
child-activity custody, public website runtime, production publishing,
signing/store proof, updater execution, and support backend upload
unimplemented or unclaimed.

Current support escalation boundary proof:
`support-bundle-redaction-proof` adds logging-domain support incident rows for
billing escalation manual-required state and account lookup manual-required
state. The proof keeps support bundles limited to redaction-safe billing-status
and account-status references, rejects provider secrets, does not contact a
billing provider, does not execute account lookup, and keeps backend upload,
remote support, and production SLA as manual-required or not-implemented states.

Current billing/support admin boundary proof:
`billing-support-admin-boundary-proof` adds parent-domain rows for support-case
triage, account-status review, billing escalation request, provider-contact
manual-required state, entitlement admin override manual-required state, and
refund/credit manual-required state. The proof keeps Stripe/provider SDKs,
provider secrets, provider contact execution, account backend admin runtime,
entitlement override runtime, refund/credit runtime, support backend upload,
portal admin UI, and child activity custody unimplemented or unsupported while
retaining evidence export and local safety behavior in parent-visible support
states.

Current billing/support admin status proof:
`billing-support-admin-status-proof` adds parent-visible support/admin status
rows for case triage, account review, billing escalation, provider-contact
manual-required, entitlement-override manual-required, refund-credit
manual-required, and resolution-update readiness. The proof requires redaction
audit and manual-proof refs, links the rows back to the billing support/admin
boundary and entitlement/failure/device-limit proofs, and keeps provider contact,
account lookup execution, entitlement override, refund/credit runtime, portal
admin UI, support backend upload, production billing support execution, and child
activity custody as explicit non-claims.

Current entitlement runtime proof:
`billing-entitlement-runtime-proof` adds parent-domain runtime/status
consumption rows for account entitlement snapshots, device-limit decisions, and
billing failure states. It proves active, stale, payment-required, and
provider-unavailable entitlement snapshots can be consumed by local status
runtime rows, over-limit new-device activation is blocked, degraded states carry
failure context, evidence export and existing local safety continue, and
Stripe/live provider execution, provider contact, refund/credit execution,
child activity custody, portal UI, and production billing support remain
unimplemented or unclaimed.

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
- Local-safety fallback and retained evidence-export behavior when billing is
  unavailable, stale, or payment-required. Current
  `billing-entitlement-contract-proof` covers this contract-only boundary.
- Account, entitlement, subscription, device-limit, download, update, and
  release-status endpoint route contracts. Current
  `billing-account-endpoint-contract-proof` covers this contract-only endpoint
  boundary.

## Acceptance

- Paid features check entitlements through typed contracts.
- Billing failures are visible.
- Local safety behavior degrades deliberately when billing cannot be checked.
- No billing secret is committed or exposed to the portal.
- Billing state changes are auditable.

## Validation Gates

- TypeScript schema tests prove valid/invalid plans, entitlement snapshots, subscription statuses, billing sync events, device-limit decisions, and failure states.
- `billing-entitlement-contract-proof` proves valid and invalid plan,
  entitlement snapshot, subscription sync, device-limit, failure, provider
  boundary, export-retention, and no-safety-shutdown states before provider or
  child-device runtime code exists.
- `billing-subscription-device-limit-failure-proof` proves degraded
  subscription rows require failure state, new-device activation cannot pass at
  the plan limit unless the device is already trusted, and billing failures
  cannot drop existing local safety continuation.
- `billing-account-runtime-boundary-proof` proves backend/provider unavailable
  account runtime rows require failure state, entitlement signing runtime gaps
  stay manual-required, and Stripe/provider secrets, portal UI, child-device
  consumption, and child-activity custody remain non-claims.
- `billing-entitlement-runtime-proof` proves account entitlement snapshot
  runtime rows consume active/stale/payment-required/provider-unavailable states,
  device-limit decisions block over-limit new-device activation, billing failure
  state consumption retains evidence export and local safety continuation, and
  provider execution/contact, refund/credit, child custody, portal UI, and
  production billing support remain non-claims.
- `support-bundle-redaction-proof` proves billing support escalation and account
  lookup remain support-safe manual-required states before provider/backend
  support workflows exist, and that support bundles do not contain billing
  provider secrets, child activity, raw URLs, screenshots, journals, SQLite
  snapshots, private paths, command lines, keystrokes, clipboard data, or
  message contents.
- `billing-support-admin-boundary-proof` proves support-case triage,
  account-status review, billing escalation, provider contact, entitlement admin
  override, and refund/credit support rows stay non-UI and non-provider, require
  manual state where production admin action would be needed, and reject
  provider contact, provider secrets, support upload, portal admin UI, and child
  activity custody.
- `billing-support-admin-status-proof` proves parent-visible billing
  support/admin status rows and resolution-update readiness stay source-contract
  or manual-required only, retain evidence export and redaction/manual proof
  references, and reject provider contact execution, account lookup execution,
  entitlement override runtime, refund/credit runtime, support backend upload,
  portal admin UI, and child activity custody.
- `production-support-account-sla-status-proof` proves account lookup request
  and result boundaries, billing provider contact status, remote support
  request/session status, and production SLA status remain source-contract,
  manual-required, or not-implemented only; it rejects account lookup execution,
  billing provider contact execution, provider secrets, support backend upload
  execution, remote support sessions, production SLA commitments, and child
  activity custody.
- `production-support-legal-provider-readiness-proof` proves billing provider
  contact and account lookup remain manual-required support boundaries inside a
  broader legal/provider readiness matrix; it rejects billing provider contact
  execution, account lookup execution, provider secret custody, payment provider
  tokens, support backend upload execution, remote support sessions, production
  SLA commitments, public runtime execution, and child activity custody.
- Endpoint-domain contract tests and `billing-account-endpoint-contract-proof`
  prove account, entitlement, subscription, device-limit, download, update, and
  release-status route boundaries before provider/backend code exists.
- Parent-domain contract tests and `production-release-public-status-proof`
  prove public account/subscription status surfaces stay route-contract or
  manual-required before account backend, provider runtime, public website, or
  production support execution exists.
- Parent-domain contract tests and
  `production-release-public-runtime-handoff-proof` prove public account and
  subscription status handoff rows plus account/backend provider adapter rows
  stay backend-required, manual-required, or not-implemented before account
  backend, billing provider runtime, public website runtime, production
  publishing, updater execution, support upload, or child custody exists.
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
