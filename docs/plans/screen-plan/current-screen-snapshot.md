# Current Screen Snapshot

## Current Product State

Screen evidence analysis has real foundation but is not product-complete. The current architecture and expectation docs already define the correct privacy boundary:

- opt-in;
- local-first;
- temporary;
- encrypted while queued;
- summarized locally;
- deleted under visible retention rules;
- policy consumes summaries/evidence refs, not raw screenshots.

## Existing Foundation

| Area                            | Existing Evidence                                                                                                                                    | Status              |
| ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------- |
| Feature docs                    | `docs/features/screen-evidence-analysis.md`, `docs/features/screen-visibility-live-view.md`                                                          | Existing.           |
| Expectation docs                | `docs/expectations/screen-evidence.md`                                                                                                               | Existing.           |
| Architecture                    | `docs/architecture/local-screen-evidence-analysis-queue.md`                                                                                          | Existing.           |
| Schema/capability/settings docs | `docs/screen-evidence-analysis-schema-proposal.md`, `docs/screen-evidence-analysis-capability-guide.md`, `docs/screen-control-settings-inventory.md` | Existing inputs.    |
| TS contracts                    | `packages/activity-domain/src/screen-evidence*.ts`                                                                                                   | Partial foundation. |
| Rust protocol                   | `crates/agent-protocol/src/screen_evidence.rs`                                                                                                       | Partial foundation. |
| Queue                           | `crates/agent-core/src/screen_evidence_queue.rs`                                                                                                     | Partial foundation. |
| SQLite/journal summary store    | `crates/agent-core/src/activity_store_screen_evidence.rs`                                                                                            | Partial foundation. |
| Portal/read-model plumbing      | `crates/agent-service/src/activity_surface_*`, `packages/portal-domain`, `apps/portal/src/live-activity-state.ts`                                    | Partial foundation. |

## Known Gaps

- Parent opt-in UI.
- Capability/status UI.
- Real platform capture adapters.
- Capture cadence proof.
- OCR/vision runtime quality proof.
- Encrypted queue proof.
- Deletion proof.
- Confidence threshold policy.
- Parent explanation UX.
- Raw screenshot retention mode decision.
- Live view mode decision.
- Platform proof.
- Playwright proof.
- Privacy/legal review.

## Product Boundary

Local screen evidence analysis is the default screen path:

```text
parent opt-in
  -> child capture scheduler
  -> encrypted temporary queue
  -> local OCR/vision worker
  -> schema-valid result
  -> journal/SQLite
  -> portal/policy handoff
  -> temporary image deletion
```

Raw screenshot retention and live view are separate explicit modes. They are not silently included in local screen evidence.
