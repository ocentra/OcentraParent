# 30 Test Suite Playwright Rollout PR Gate

## Target State

Unit, integration, contract, security, E2E, UI, performance, manual tests and
merge-blocking gates are complete or explicitly recorded as non-claims before
any full screen-plan completion claim.

## Current State

The screen plan now has contract, service, eventing, Playwright, live-operator,
real-trigger, cadence, capture-analysis-policy, portal, deletion/custody, child
disclosure, and final stacked product-path proof artifacts. The closure audit
now verifies the current readiness artifacts for macOS, Linux, Android, iOS,
live view, OCR, and VLM while separately recording the product-readiness gates
that remain false. This prevents the stacked product-path artifact from being
misread as full screen-plan completion.

The rollout gate remains partial because macOS live capture proof, Linux
root/Wayland portal proof, physical Android parity, iOS ReplayKit proof,
optional live-view transport/platform prompt proof, service-persisted retention
controls, broad OCR/VLM model-quality evaluation, authenticated-account social
proof, and production startup wiring are still non-claims.

## Checklist

- [x] Add contract/unit tests for completed screen contracts.
- [x] Add Rust core/service proof tests for completed screen service paths.
- [x] Add security negative tests for completed raw-retention, raw-output, and
      authority boundaries.
- [x] Add platform manual proof for currently claimed platform capture paths.
- [x] Add Playwright UI tests.
- [x] Add E2E proof against real service.
- [x] Add real browser-use trigger proof.
- [x] Add real app-use trigger proof.
- [x] Add timed cadence capture proof.
- [x] Add operator live URL/app proof before product-complete claim.
- [x] Add capture plus analysis plus policy dry-run proof.
- [x] Update feature docs/checklist for completed proof gates.
- [x] Record known gaps.
- [x] Run a screen-plan closure audit that proves current readiness artifacts
      are present while platform/model/live-view product gates remain explicit
      non-claims.
- [ ] Run final screen-plan closure validation after partial platform/model
      workpacks are completed or deliberately accepted as non-claims.
- [ ] Write final whole-slice handoff only after screen, AI, and pipeline plans
      are complete on the unified B branch.

## Proof

- Validation command logs.
- Screenshot artifacts.
- Real trigger proof artifacts.
- Operator live proof artifacts.
- Cadence proof artifacts.
- Closure audit artifact:
  `output/screen-plan-proof/screen-plan-closure-audit/proof-summary.json`.
- Feature/checklist updates.
- Final branch handoff with scope, proof, and gaps after the full B scope is
  complete.
