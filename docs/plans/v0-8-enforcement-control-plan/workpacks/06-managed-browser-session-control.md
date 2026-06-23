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

- `npm run test --workspace @ocentra-parent/enforcement-domain -- enforcement`
- selected browser managed-session proof or service tests
- `npm run test --workspace @ocentra-parent/portal -- enforcement` when a parent-visible browser state changes
- selected architecture gate for touched browser/enforcement/portal surfaces

## AI Worker Checklist

- [ ] Reject unmanaged or stale bridge/session ids.
- [ ] Separate managed session intervention from exact URL action.
- [ ] Show unsupported, degraded, bridge-unavailable, and manual-required states.
- [ ] Avoid page body, form, cookie, token, or decrypted content claims.
- [ ] Add browser evidence refs to action/audit output.

## Where We Are

Managed browser status and manual-required exact URL states are represented, but
managed browser enforcement is not product-complete.

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
