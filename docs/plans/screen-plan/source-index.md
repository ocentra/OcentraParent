# Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Source Index`
> Kind: source ownership index; read only when source ownership is unclear.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not inspect broad source from here; use only the named package/crate path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This index records the source files and product docs that own screen evidence work. Use it before implementation so a worker does not invent a second truth.

## Product Sources

| Source                                                      | Ownership                                                                                                        |
| ----------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `docs/feature-list.md`                                      | Lists screen evidence analysis and screen visibility/live view as separate feature areas.                        |
| `docs/features/screen-evidence-analysis.md`                 | Owns local screenshot evidence, OCR/vision summaries, deletion, confidence, and parent explanation gaps.         |
| `docs/features/screen-visibility-live-view.md`              | Owns optional screenshot/live-view product decision and proof requirements.                                      |
| `docs/expectations/screen-evidence.md`                      | Owns privacy, custody, data, retention, and done criteria.                                                       |
| `docs/architecture/local-screen-evidence-analysis-queue.md` | Owns the parent opt-in to capture scheduler to encrypted queue to local OCR/vision to SQLite/portal/policy flow. |
| `docs/screen-evidence-analysis-schema-proposal.md`          | Schema design input that must be reconciled against current contracts.                                           |
| `docs/screen-evidence-analysis-capability-guide.md`         | Capability guide input for platform/status surfaces.                                                             |
| `docs/screen-control-settings-inventory.md`                 | Screen settings inventory input.                                                                                 |

## Current Code Foundation

| Source                                                               | Current Role                                      |
| -------------------------------------------------------------------- | ------------------------------------------------- |
| `packages/activity-domain/src/screen-evidence*.ts`                   | Screen evidence TypeScript contracts.             |
| `packages/activity-domain/tests/screen-evidence.test.ts`             | Current contract tests.                           |
| `crates/agent-protocol/src/screen_evidence.rs`                       | Rust protocol shapes.                             |
| `crates/agent-protocol/src/screen_evidence_tests.rs`                 | Rust protocol tests.                              |
| `crates/agent-core/src/screen_evidence_queue.rs`                     | Encrypted temporary queue foundation.             |
| `crates/agent-core/src/screen_evidence_queue_tests.rs`               | Queue tests.                                      |
| `crates/agent-core/src/activity_store_screen_evidence.rs`            | Screen summary SQLite/journal storage foundation. |
| `crates/agent-core/src/activity_store_screen_evidence_tests.rs`      | Store tests.                                      |
| `crates/agent-service/src/activity_surface_store.rs`                 | Service activity-surface integration.             |
| `crates/agent-service/src/activity_surface_read_models.rs`           | Read-model projection.                            |
| `crates/agent-service/src/activity_surface_read_model_states.rs`     | Read-model states.                                |
| `crates/agent-service/src/activity_surface_adapter.rs`               | Portal adapter boundary.                          |
| `crates/agent-service/src/websocket.rs`                              | Service command/event routing.                    |
| `packages/portal-domain/src/routes.ts`                               | Screen-analysis and screen-policy route IDs.      |
| `packages/portal-domain/src/commands.ts`                             | Activity screen read-model command direction.     |
| `packages/portal-domain/src/parent-portal-manage-data.ts`            | Portal route/nav metadata.                        |
| `apps/portal/src/live-activity-state.ts`                             | Portal live activity aggregation.                 |
| `apps/portal/tests/live-activity-surface-adapter.test.ts`            | Portal adapter proof direction.                   |
| `apps/portal/tests/activity-ui-intent.test.ts`                       | Portal UI intent proof direction.                 |
| `packages/parent-domain/src/screen-control-*`                        | Policy/control catalog inputs.                    |
| `packages/parent-domain/tests/screen-control-policy-catalog.test.ts` | Policy control catalog tests.                     |

## External Platform References To Verify During Workpacks

| Platform   | Primary Source                                          |
| ---------- | ------------------------------------------------------- |
| Windows    | Microsoft Learn: Windows Graphics Capture               |
| macOS      | Apple Developer: ScreenCaptureKit                       |
| Linux      | Desktop portal, PipeWire, X11, compositor documentation |
| Android    | Android Developers: MediaProjection                     |
| iOS/iPadOS | Apple Developer: ReplayKit                              |

Workers must verify current official docs before claiming platform-specific implementation behavior.

## Screen Intelligence Research References

| Area                         | Source To Verify                                                                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Managed browser screenshot   | Chrome DevTools Protocol `Page.captureScreenshot` and `Page.startScreencast`.                                                                                      |
| OCR baseline                 | Tesseract project/docs, Apache-2.0 license, Windows installer state, and packaging proof.                                                                          |
| OCR preferred path           | PaddleOCR 3.0 / PP-OCR evaluation and packaging proof at `output/screen-plan-proof/35-ocr-paddleocr-ppocr-evaluation/proof-summary.json`.                          |
| Small/household-provider VLM | Guided local VLM readiness proof at `output/screen-plan-proof/36-small-vlm-guided-classifier-evaluation/proof-summary.json`; candidate model quality remains open. |
| Remote/API boundary          | Product privacy docs and parent-approved redacted-summary-only contract.                                                                                           |

Do not claim a model is selected until packaging, runtime, privacy, and quality proof exist.

## Cross-Slice Consumers

Screen summaries may be consumed by:

- browser policy;
- app policy;
- native game policy;
- browser game policy;
- social/video policy;
- bypass-tool policy;
- unknown activity/manual-required policy;
- tracking/check-in context when explicitly linked.

Screen evidence remains a shared local visual evidence layer. It is not owned only by browser-plan.

## Not Product Proof

- Browser automation screenshots.
- Portal screenshots alone.
- Temporary developer screenshots.
- Remote desktop experiments.

Those are useful UI proof artifacts, but they do not prove child-device capture, local OCR/vision, custody, deletion, or policy behavior.
