# WP30 - Rust Protocol Authority Classifier Parity

## Scope

Cross-record shared app/game WP30 for the native app plan. The native app slice
uses the shared `AppGame*` authority and classifier protocol spine and does not
create a second app-only protocol truth.

Covered shared shapes:

- `AppGameControlApprovalAuthority`
- `AppGameControlApprovalRequest`
- `AppGameControlApprovalDecision`
- `AppGameControlActionResult`
- `AppGamePlatformAuthorityRow`
- `AppGamePlatformAuthorityMatrix`
- `AppGameAiClassifierResult`

## Implementation Boundary

This workpack proves Rust protocol parity only. It does not add native app live
source crawling, service persistence, live classifier execution, policy
evaluator consumption, portal authority/classifier UI, child request UI,
install approval, platform authority upgrades, or broad app blocking.

## Required Proof

- Shared implementation and tests in `crates/agent-protocol`.
- Cross-recorded proof output under
  `output/app-plan-proof/30-rust-protocol-authority-classifier-parity/`.
- Native app snapshot/checklist docs that preserve the remaining live runtime,
  classifier, authority, platform, and product UI gaps.

## AI Worker Checklist

- [ ] Native app plan read and reconciled with shared app/game plan.
- [ ] No duplicate native-app-only protocol shape was created.
- [ ] Rust protocol parity exists for shared authority/action-result,
      platform-authority, and classifier-boundary shapes.
- [ ] App-specific live classifier, storage, portal, policy, and platform gaps
      remain explicitly documented.
- [ ] Product checklist was not edited; no product status moved.
