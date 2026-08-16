# WP29 - Rust Protocol Evidence Identity Parity

## Scope

Cross-record shared app/game WP29 for the native app plan. The native app slice
uses the shared `AppGame*` evidence spine and does not create a second
app-only protocol truth.

Covered shared shapes:

- `AppGameEvidenceClaim`
- `AppGameAiDigestReference`
- `AppGameAiClassificationDigest`
- `AppGameIdentity`
- `AppGameIdentityMergeProof`

## Implementation Boundary

This workpack proves Rust protocol parity only. It does not add native app live
source crawling, runtime identity merge behavior, service persistence, portal
identity UI, app policy runtime evaluation, child request UI, install approval,
platform authority upgrades, or broad app blocking.

## Required Proof

- Shared implementation and tests in `crates/agent-protocol`.
- Cross-recorded proof output under
  `output/app-plan-proof/29-rust-protocol-evidence-identity-parity/`.
- Native app snapshot/checklist docs that preserve the remaining live runtime
  and product UI gaps.

## AI Worker Checklist

- [ ] Native app plan read and reconciled with shared app/game plan.
- [ ] No duplicate native-app-only protocol shape was created.
- [ ] Rust protocol parity exists for the shared evidence/identity shapes.
- [ ] App-specific runtime identity merge, storage, portal, policy, and
      platform gaps remain explicitly documented.
- [ ] Product checklist was not edited; no product status moved.
