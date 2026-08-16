# 08 Bridge Custody And Security

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `08 Bridge Custody And Security`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

CDP polling rejects non-loopback endpoints, but full custody proof must also tie
port, profile, process, session id, and endpoint refs together.

## Where We Want To Be

The agent consumes only Ocentra-launched loopback browser bridges for the
current managed session and never leaks raw debugger URLs.

## Scope

- Loopback-only bridge.
- Reserved current-session port.
- Managed profile match.
- Process id match.
- Browser family/channel match.
- Stale session rejection.
- Default profile bridge rejection.
- Raw `webSocketDebuggerUrl` redaction.

## Touched Paths

- `crates/agent-core/src/browser_bridge_*.rs`
- `crates/agent-core/src/browser_managed_session.rs`
- `crates/agent-service/src/browser_runtime*.rs`
- `crates/agent-protocol/src/constants/*.rs`

Portal UI was not changed for this workpack; bridge custody is enforced before
portal-facing browser observations are emitted.

## Tests And Proof

- Security tests for wrong port, non-loopback, wrong profile, wrong process,
  stale session, malformed target, and debugger URL leakage.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/08-bridge-custody-and-security/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: `output/browser-plan-proof/08-bridge-custody-and-security/03-runtime-evidence.json`.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `output/browser-plan-proof/08-bridge-custody-and-security/06-ui-snapshots/ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: `output/browser-plan-proof/08-bridge-custody-and-security/08-security-negative-proof.log`.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels in `output/browser-plan-proof/08-bridge-custody-and-security/09-manual-platform-proof.md`.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Bridge custody does not prove browser policy enforcement. Real installed
Chrome/Edge bridge proof, exact URL intervention, extension/native-host/router
claims, and AI direct enforcement remain manual-required or out of scope for
this workpack.
