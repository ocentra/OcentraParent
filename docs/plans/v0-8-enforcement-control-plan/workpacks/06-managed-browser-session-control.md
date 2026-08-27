# 06 Managed Browser Session Control

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `06 Managed Browser Session Control`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md),
[folder README](../README.md),
[browser-web-control feature](../../features/browser-web-control.md), and
[enforcement expectation](../../expectations/enforcement.md).

## Purpose

Define the managed-browser action boundary so only an Ocentra-managed
profile/session with a validated bridge/session id can become an enforcement
target.

## Central schema boundary

```text
schema-domain owns public managed-session, bridge-status, action-result, and reason schemas when they cross package/crate/protocol boundaries.
browser-plan owns browser profile/session/URL evidence and unmanaged-browser fact surfaces.
policy-control-plane-plan owns upstream policy authority.
v0-8-enforcement-control-plan owns managed-session action states, audit, and no-claim boundaries.
```

## Source Inputs

- `../v0-8-enforcement-control-20-step-plan.md`
- `../v0-8-enforcement-control-test-blueprint.md`
- `../../features/browser-web-control.md`
- `../../expectations/enforcement.md`

## Target State

Managed browser actions apply only to an Ocentra-managed profile/session with a
validated bridge and service-owned session id.

## Required proof fields

```text
canonical_schema_owner_state
managed_profile_state
bridge_session_state
stale_bridge_state
unmanaged_state
exact_url_claim_state
adapter_capability_state
audit_state
manual_required_state
no_decrypted_content_claim
no_exact_url_claim
no_unmanaged_control_claim
no_claim
```

## Tests And Proof

Proof root: `output/v0-8-enforcement-control-plan-proof/06-managed-browser-session-control/`

Focused validation should record:

- `cargo test -p ocentra-parent-agent-core browser_managed_session`
- `cargo test -p ocentra-parent-agent-core enforcement`
- `cargo test -p ocentra-parent-agent-service enforcement`
- selected browser managed-session/service tests when source changes
- `npm run lint:architecture -- --files <touched Rust source/test directories>`

These commands prove the Rust boundary and no-effect behavior; they do not
prove browser action execution. A future real adapter must add focused
execution, receipt/audit, rollback, and parent-visible-state coverage.

## AI Worker Checklist

- [ ] Reject unmanaged or stale bridge/session ids.
- [ ] Separate managed session intervention from exact URL action.
- [ ] Show unsupported, degraded, bridge-unavailable, and manual-required states.
- [ ] Avoid page body, form, cookie, token, or decrypted content claims.
- [ ] Add browser evidence refs to action/audit output.

## Where We Are

Live audit on 2026-07-23 found a real Rust-owned managed-profile/launch and
bridge-status boundary in `agent-core` and `agent-service`, including rejection
of default/unowned profiles and unreserved bridge ports. That boundary is not
yet an enforcement adapter: policy targets `site`, `video`, and `channel` are
classified as `ManagedBrowserControl` with a manual-required capability, and
the service execution switch implements only owned-process termination. A
managed-browser adapter request therefore cannot produce a browser action,
receipt, rollback, or audit execution trace.

The old TypeScript command was stale: neither `packages/enforcement-domain` nor
`packages/agent-protocol-domain` exists in the current Rust-first repository.
Do not create them to satisfy the former command. The execution owner is
`crates/agent-core` plus `crates/agent-service`, with `crates/agent-protocol`
for the typed boundary.

Several proof/read-model surfaces label the managed-browser boundary as an
implemented real service. Those labels are not completion evidence while the
execution switch still rejects that adapter kind. They must be corrected or
backed by a real adapter; this workpack remains open.

## Managed Runtime Test-Truth Correction — 2026-08-27

PR #709 withdrew the managed-ready, running, bridge-disconnected, and connected
agent-service cases that constructed private `BrowserManagedProfileStoreEntry`
or `BrowserManagedLaunch` owner authority. Retained agent-core and
agent-service roots still cover fail-closed missing/profile-missing/error
states, unmanaged observation, payload and empty-inventory behavior, and
direct inventory/policy models. They do not prove an owner-issued managed
profile, launch, bridge session, or action target.

The production profile store remains
`ProtectedCustodyAdapterUnavailable`. Keep
`crates/agent-service/tests/integration/browser_managed_runtime.rs` and
`crates/agent-core/tests/integration/browser_bridge_managed_launch.rs`
missing/open until a protected owner adapter and launch authority provide the
real integration boundary. Public authority constructors, fixture authority,
and fake harnesses are not acceptable substitutes. This workpack remains open
and manual-required.

## Negative Cases

- stale or missing bridge/session ids must stay rejected or manual-required
- unmanaged browser process detection must not become managed control
- exact URL, page body, or token/cookie claims must stay out of scope
- bridge unavailable or degraded transport must not be shown as success
- browser evidence alone must not claim adapter execution

## Manual-Required Gaps

- Exact-URL control remains separate and unproved.
- Unmanaged browser behavior remains fallback/reporting only unless another
  slice proves more.
- Non-Windows or mobile browser parity remains unclaimed.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch recorded.
- [ ] Validation commands and results recorded in `16-validation-commands.log`.
- [ ] Proof artifacts under `output/v0-8-enforcement-control-plan-proof/06-managed-browser-session-control/`.
- [ ] Known gaps/manual-required states listed here and in the proof note.
