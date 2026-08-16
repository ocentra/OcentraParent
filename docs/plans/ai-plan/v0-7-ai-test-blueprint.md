# V0.7 AI Test Blueprint

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `V0.7 AI Test Blueprint`
> Kind: test blueprint reference; read only when local expectations route here.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Unit Test Targets

Expected folders:

- `tests/ai/unit/local_ai_input_contract.test.ts`
- `tests/ai/unit/local_ai_result_contract.test.ts`
- `tests/ai/unit/runtime_status_contract.test.ts`
- `tests/ai/unit/provider_capability_contract.test.ts`
- `tests/ai/unit/ai_job_queue_contract.test.ts`
- `tests/ai/unit/provider_route_contract.test.ts`
- `tests/ai/unit/ai_work_item_contract.test.ts`
- `tests/ai/unit/ai_work_claim_contract.test.ts`
- `tests/ai/unit/ai_work_lease_contract.test.ts`
- `tests/ai/unit/ai_work_result_contract.test.ts`
- `tests/ai/unit/ai_result_validation_contract.test.ts`
- `tests/ai/unit/ai_provider_advertisement_contract.test.ts`
- `tests/ai/unit/ai_provider_capability_contract.test.ts`
- `tests/ai/unit/ai_provider_mobile_dormant_policy.test.ts`
- `tests/ai/unit/mesh_transport_message_contract.test.ts`
- `tests/ai/unit/prompt_template_version.test.ts`
- `tests/ai/unit/memory_reference_contract.test.ts`
- `tests/ai/unit/graph_reference_contract.test.ts`
- `tests/ai/unit/evidence_context_builder_contract.test.ts`

## Integration Test Targets

- `tests/ai/integration/context_builder_from_sqlite.test.ts`
- `tests/ai/integration/provider_status_unavailable.test.ts`
- `tests/ai/integration/provider_route_selection.test.ts`
- `tests/ai/integration/ai_output_parser.test.ts`
- `tests/ai/integration/invalid_output_rejection.test.ts`
- `tests/ai/integration/policy_dry_run_integration.test.ts`
- `tests/ai/integration/ai_result_journal_sqlite.test.ts`
- `tests/ai/integration/memory_reference_source_guard.test.ts`
- `tests/ai/integration/graph_reference_source_guard.test.ts`
- `tests/ai/integration/tabagent_reuse_adapter_boundary.test.ts`
- `tests/ai/integration/child_agent_queues_ai_work_from_screen_summary.test.ts`
- `tests/ai/integration/trusted_desktop_provider_claims_once.test.ts`
- `tests/ai/integration/competing_provider_claim_rejected.test.ts`
- `tests/ai/integration/lease_expiry_requeues_job.test.ts`
- `tests/ai/integration/provider_result_accepted_then_policy_requested.test.ts`
- `tests/ai/integration/invalid_provider_result_rejected.test.ts`
- `tests/ai/integration/mobile_provider_dormant_when_desktop_available.test.ts`
- `tests/ai/integration/no_raw_screenshot_transfer_by_default.test.ts`

## Security Test Targets

- `tests/ai/security/ai_no_direct_os_scan.test.ts`
- `tests/ai/security/ai_no_direct_enforcement.test.ts`
- `tests/ai/security/remote_ai_disabled_by_default.test.ts`
- `tests/ai/security/raw_screenshot_api_guard.test.ts`
- `tests/ai/security/memory_without_source_rejected.test.ts`
- `tests/ai/security/graph_without_source_rejected.test.ts`
- `tests/ai/security/prompt_minimization.test.ts`
- `tests/ai/security/custody_label_guard.test.ts`
- `tests/ai/security/provider_cannot_publish_policy.test.ts`
- `tests/ai/security/provider_cannot_publish_enforcement.test.ts`
- `tests/ai/security/remote_peer_cannot_direct_publish_local_bus.test.ts`
- `tests/ai/security/wrong_provider_result_rejected.test.ts`
- `tests/ai/security/expired_lease_result_rejected.test.ts`
- `tests/ai/security/raw_screen_payload_rejected_by_default.test.ts`

## E2E Test Targets

- `tests/ai/e2e/browser_url_ai_dry_run.test.ts`
- `tests/ai/e2e/unknown_app_ai_dry_run.test.ts`
- `tests/ai/e2e/location_ai_alert_support.test.ts`
- `tests/ai/e2e/screen_ocr_ai_summary.test.ts`
- `tests/ai/e2e/policy_conflict_ai_cannot_override.test.ts`
- `tests/ai/e2e/ai_unavailable_safe_degrade.test.ts`
- `tests/ai/e2e/real_browser_capture_ai_analysis.test.ts`
- `tests/ai/e2e/real_app_capture_ai_analysis.test.ts`
- `tests/ai/e2e/timed_cadence_capture_ai_analysis.test.ts`

## Playwright Proof Targets

- AI runtime status.
- AI provider routing.
- AI degraded state.
- AI decision explanation.
- AI memory evidence.
- AI activity/job history.
- Remote assistant boundary.
- Real browser-use capture artifact analyzed and visible.
- Real app-use capture artifact analyzed and visible.
- Timed cadence capture sequence analyzed without queue flood.

## Real Analysis Proof Targets

The real proof requirements are defined in
[Real AI Analysis And Pipeline Proof Matrix](real-ai-analysis-and-pipeline-proof-matrix.md).
At minimum, the AI pass must prove analysis for:

- YouTube ordinary video or controlled equivalent.
- YouTube or Vimeo education video or controlled equivalent.
- Vimeo ordinary video or controlled equivalent.
- Facebook/social surface or controlled equivalent.
- Browser game/cloud-game surface or controlled equivalent.
- Native app foreground capture.
- Native game or controlled game-window capture.
- Bypass-tool fixture/app.
- Shopping fixture/page.
- School/productivity fixture/page/app.
- Unknown activity degraded safely.
- Timed cadence capture sequence with repeated analysis.
- Disabled capture produces no AI analysis.

The final trigger-to-capture-to-analysis-to-policy/action proof belongs to
`docs/plans/screen-ai-pipeline-plan` after screen and AI prerequisite branches
are merged or explicitly stacked.

## Critical Assertions

- AI input missing evidence refs is rejected.
- AI result missing parent-rule refs is rejected.
- Confidence outside `0..1` is rejected.
- AI result cannot directly enforce.
- AI cannot scan OS/process/browser/network/screen directly.
- Remote/API AI is disabled by default.
- Invalid output yields degraded/unknown.
- Timeout yields degraded/unknown.
- Low confidence maps to unknown, warn, or ask-parent depending policy.
- AI cannot override a stricter parent rule.
- Memory/graph without source evidence cannot drive a decision.
- Policy consumes only schema-valid AI result.
- Journal stores AI result with refs.
- Portal explanation cites evidence and rules.
- Real capture proof is linked when AI analyzes a screen-derived result.
- AI work is event-driven; no direct capture-to-worker coupling exists.
- Household Mesh Bridge is the only cross-device event transport path.
- Incoming LAN messages validate before local republish.
- Remote peers cannot directly publish into another runtime's bus.
- AI provider cannot publish policy decision.
- AI provider cannot publish enforcement command.
- Child agent validates provider result before policy sees it.
- Duplicate work `dedupeKey` does not execute twice.
- Expired lease cannot submit accepted result.
- Wrong provider cannot complete another provider's lease.
- Mobile provider remains dormant unless fallback policy allows it.
