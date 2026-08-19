# 09 CDP Version And Target Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `09 CDP Version And Target Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

`poll_chromium_bridge` reads `/json/version` and `/json/list`, filters page
targets, maps observable page targets into activity events, marks active state
unknown, rejects malformed target payloads, bounds response size, and keeps the
adapter explicit about tab-list-only capability.

## Where We Want To Be

The CDP adapter is fixture-backed, malformed-payload-safe, redaction-safe, and
explicit about tab-list-only capability.

### Reviewed production-readiness boundary (2026-08-18)

The parser and target/capture authority code are real, but this workpack has
no production caller that retains the same managed launch, validates custody,
and routes a source-backed result into Screen. The next packet must poll from
the private authority retained by the WP07 service runtime, mint target
authority only from a trusted same-launch page candidate, and keep active-tab
state `Unknown` because target-list evidence is not focus proof. WP09 remains
open and is not PR-ready.

Required integration tests remain open: retained-launch bridge success and
owner mismatch; malformed, oversized, timeout, missing-target, target
disappearance/navigation, and process-replacement cases; same-launch target
authority; Screen handoff success/manual-required/failure; and a regression
that forbids promoting target-list evidence to a known active tab. Fixture
parser tests alone do not close this production seam.

## Scope

- `/json/version` parser.
- `/json/list` parser.
- Page target filtering.
- Internal/blank page handling.
- Missing URL and malformed payload rejection.
- Bridge-missing, timeout, invalid JSON, and adapter-error states.
- Redacted debugger endpoint handling.

## Touched Paths

- `crates/agent-core/src/browser_bridge_poll.rs`
- `crates/agent-core/src/browser_bridge_http.rs`
- `crates/agent-core/src/browser_bridge_tests.rs`
- `crates/agent-core/src/browser_bridge_cdp_adapter_tests.rs`
- `crates/agent-core/src/browser_bridge_poll_security_tests.rs`
- `crates/agent-core/src/browser_bridge_poll_test_support.rs`
- `crates/agent-core/src/browser_bridge_poll_tests.rs`
- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-protocol/src/constants/value.rs`

## Tests And Proof

- Fake HTTP server tests.
- CDP fixture parser tests.
- Oversized/malformed payload tests.
- `cargo test -p ocentra-parent-agent-core browser_bridge` covers non-loopback
  rejection, timeout, invalid JSON, non-object version payload, non-array target
  payload, missing target id/url, blank/internal target filtering, oversized
  response rejection, raw debugger URL redaction, and tab-list-only events.
- `cargo test -p ocentra-parent-agent-service browser_runtime` proves the
  service still compiles and reports browser runtime state with the expanded
  poll error surface.
- `cargo test -p ocentra-parent-agent-protocol browser_managed` proves protocol
  serialization stays compatible after the new CDP constants/reasons.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/09-cdp-version-and-target-adapter/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: `output/browser-plan-proof/09-cdp-version-and-target-adapter/03-runtime-evidence.json`.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `output/browser-plan-proof/09-cdp-version-and-target-adapter/06-ui-snapshots/ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: `output/browser-plan-proof/09-cdp-version-and-target-adapter/08-security-negative-proof.log`.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels in `output/browser-plan-proof/09-cdp-version-and-target-adapter/09-manual-platform-proof.md`.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

CDP target visibility does not prove active tab unless separate proof exists.
This workpack keeps capability status at tab-list-only and does not claim
policy action or intervention.
