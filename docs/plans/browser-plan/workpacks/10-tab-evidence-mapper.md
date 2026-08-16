# 10 Tab Evidence Mapper

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `10 Tab Evidence Mapper`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

The current mapper can create browser tab observation events from CDP page
targets with unknown active state.

## Where We Want To Be

Raw bridge observations become schema-valid browser tab evidence with stable
ids, normalized URL/origin/domain, freshness, custody, and capability labels.

## Scope

- Evidence id generation.
- Source id and adapter id.
- Managed session id and profile id.
- Target/window/tab ids.
- URL, origin, domain, title.
- Observed/fresh/stale timestamps.
- Capability status and degraded reason.
- Custody/query visibility.

## Touched Paths

- `packages/activity-domain/src/browser-schemas.ts`
- `packages/activity-domain/tests/browser.test.ts`
- `packages/activity-domain/tests/browser-tab-evidence.test.ts`
- `crates/agent-core/src/browser_bridge_event.rs`
- `crates/agent-core/src/browser_bridge_fields.rs`
- `crates/agent-core/src/browser_bridge_ids.rs`
- `crates/agent-core/src/browser_bridge_poll.rs`
- `crates/agent-core/src/browser_bridge_poll_tests.rs`
- `crates/agent-core/src/browser_bridge_tests.rs`
- `crates/agent-core/src/activity_store_browser.rs`
- `crates/agent-core/src/activity_store_browser_tests.rs`
- `crates/agent-protocol/src/browser_read_model.rs`
- `crates/agent-protocol/src/constants/activity_store.rs`
- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-service/src/browser_evidence_payload.rs`

## Tests And Proof

- Contract tests for valid/invalid evidence.
- Mapper tests for URL normalization and malformed rows.
- Journal round-trip tests.
- Proof pack:
  `output/browser-plan-proof/10-tab-evidence-mapper/`.

## Implementation Notes

- Browser tab evidence rejects URL/origin/domain drift at the TypeScript
  contract boundary, including credential-bearing raw URL rows that bypass the
  mapper normalization step.
- The Rust bridge mapper derives a stable tab id from the CDP target id when
  the adapter does not provide `tabId`, preserves adapter-provided `tabId` and
  `windowId`, and rejects empty required target fields as invalid payloads.
- URL evidence is normalized by lowercasing scheme/host, removing URL user-info
  from emitted URL/origin/domain fields, retaining port in origin/URL, and
  keeping domain as the normalized host.
- Browser tab observations carry explicit degraded reason and query visibility
  through journal/read-model conversion and service payloads.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/10-tab-evidence-mapper/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service parity updated only after contracts exist; no portal UI change was made.
- [ ] Raw evidence artifacts captured where applicable: bridge/CDP payloads, managed session state, unmanaged process rows, journal entries, SQLite/read-model rows, policy decisions, and action results.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots are not applicable; `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof is not applicable; `09-manual-platform-proof.md` records why.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Evidence mapping does not upgrade active state beyond the input proof source.
