# 30 Test Suite Playwright Rollout PR Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `30 Test Suite Playwright Rollout PR Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
WP30 aggregates screen-plan proof roots only.
Screen-AI, AI, policy, enforcement, custody, portal, remote-access, browser, app-game, network, tracking, agent-service, and protocol owners remain separate unless their handoff proof is explicitly accepted.
Whole-plan readiness requires retained proof roots, command logs, screenshots where required, known gaps, and non-claim preservation.
```

## Target State

Unit, integration, contract, security, E2E, UI, performance, manual tests and merge-blocking gates are complete or explicitly recorded as non-claims before any full screen-plan completion claim.

## Current State

The screen plan now has contract, service, eventing, Playwright, live-operator, real-trigger, cadence, capture-analysis-policy, portal, deletion/custody, child disclosure, and final stacked product-path proof artifacts. The closure audit now verifies the current readiness artifacts for macOS, Linux, Android, iOS, live view, OCR, and VLM while separately recording the product-readiness gates that remain false. This prevents the stacked product-path artifact from being misread as full screen-plan completion.
`scripts/test/screen-plan-external-gates-proof.mjs` now enumerates the remaining real external evidence gates and keeps the screen plan in non-claim state until digest-backed live-device/live-host artifacts are attached through the manual evidence manifest. It rejects fixture/static/raw-private artifacts, so this intake does not replace the missing macOS, Linux, physical Android, iOS, live view prompt, physical parity, hosted relay, or privacy/legal proof.

The rollout gate remains partial because macOS live capture proof, Linux root/Wayland portal proof, physical Android parity, iOS ReplayKit proof, optional live-view transport/platform prompt proof, service-persisted retention controls, broad OCR/VLM model-quality evaluation, authenticated-account social proof, and production startup wiring are still non-claims.

## Required rollout fields

The selected rollout proof must name, at minimum:

```text
rollout_gate_id
accepted_screen_proof_roots
missing_screen_proof_roots
open_workpack_state
checklist_workpack_mismatch_state
closure_audit_state
external_gate_state
contract_test_state
service_test_state
security_negative_state
platform_manual_state
playwright_state
e2e_service_state
real_trigger_state
operator_live_state
cadence_state
capture_analysis_policy_state
deletion_custody_state
child_disclosure_state
feature_doc_state
known_gaps
non_claims
screen_ai_handoff_state
platform_model_live_view_non_claims
privacy_legal_state
final_handoff_state
claims_allowed
claims_blocked
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Add contract/unit tests for completed screen contracts.
- [ ] Add Rust core/service proof tests for completed screen service paths.
- [ ] Add security negative tests for completed raw-retention, raw-output, and authority boundaries.
- [ ] Add platform manual proof for currently claimed platform capture paths.
- [ ] Add Playwright UI tests.
- [ ] Add E2E proof against real service.
- [ ] Add real browser-use trigger proof.
- [ ] Add real app-use trigger proof.
- [ ] Add timed cadence capture proof.
- [ ] Add operator live URL/app proof before product-complete claim.
- [ ] Add capture plus analysis plus policy dry-run proof.
- [ ] Update feature docs/checklist for completed proof gates.
- [ ] Record known gaps.
- [ ] Add external proof-gate intake that rejects fixture/static/raw-private artifacts.
- [ ] Run a screen-plan closure audit that proves current readiness artifacts are present while platform/model/live-view product gates remain explicit non-claims.
- [ ] Run final screen-plan closure validation after partial platform/model workpacks are completed or deliberately accepted as non-claims.
- [ ] Write final whole-slice handoff only after screen, AI, and pipeline plans are complete on the unified B branch.

## Proof

- Validation command logs.
- Screenshot artifacts.
- Real trigger proof artifacts.
- Operator live proof artifacts.
- Cadence proof artifacts.
- Closure audit artifact: `output/screen-plan-proof/screen-plan-closure-audit/proof-summary.json`.
- External gate intake artifact: `output/screen-plan-proof/external-gates/proof-summary.json`.
- Feature/checklist updates.
- Final branch handoff with scope, proof, and gaps after the full B scope is complete.

## Failure conditions

- Do not claim whole-plan completion from stacked product-path proof while platform/model/live-view product gates remain false.
- Do not accept fixture/static/raw-private artifacts for external proof gates.
- Do not claim screen-AI, AI, policy, enforcement, custody, portal, or remote-access completion from screen-plan rollout proof.
- Do not omit known gaps or non-claims from final handoff.
