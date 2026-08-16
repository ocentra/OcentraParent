# WP31 - Journal/SQLite Authority Classifier Storage

## Scope

Cross-record shared app/game WP31 for the native app plan. Native app work uses
the shared `AppGame*` journal and read-model projection path and does not create
a second app-only storage truth.

Covered shared rows:

- `AppGameEvidenceClaim`
- `AppGameIdentity`
- `AppGameControlApprovalAuthority`
- `AppGameControlActionResult`
- `AppGamePlatformAuthorityMatrix`
- `AppGameAiClassifierResult`

## Implementation Boundary

This workpack proves staged Rust journal/SQLite storage and projection only. It
does not add native app live source crawling, live classifier execution, service
event exposure, policy evaluator consumption, portal authority/classifier UI,
child request UI, install approval, platform authority upgrades, or broad app
blocking.

## Required Proof

- Shared implementation and tests in `crates/agent-core`.
- Shared protocol rows in `crates/agent-protocol`.
- Cross-recorded proof output under
  `output/app-plan-proof/31-journal-sqlite-authority-classifier-storage/`.
- Native app snapshot/checklist docs that preserve the remaining live runtime,
  classifier, authority, platform, policy, and product UI gaps.

## AI Worker Checklist

- [ ] Native app plan read and reconciled with shared app/game plan.
- [ ] No duplicate native-app-only storage or protocol shape was created.
- [ ] Shared app/game evidence, identity, authority/action-result, platform
      authority, and classifier result rows can be journaled, replayed, and
      projected through the existing `ActivityStore`.
- [ ] App-specific live classifier, service-event, portal, policy, and platform
      gaps remain explicitly documented.
- [ ] Product checklist was not edited; no product status moved.
