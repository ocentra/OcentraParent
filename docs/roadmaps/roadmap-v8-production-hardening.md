<!-- agent-capsule -->

> Agent Capsule
> Doc: V8 Production Hardening Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V8 Production Hardening Expectations

This is the milestone-specific expectation file for V8 in `docs/product-roadmap.md`.

Supporting expectation files: [release installer](../expectations/release-installer.md), [sync and export](../expectations/sync-export.md), [static analysis and security](../expectations/static-analysis-security.md), [documentation](../expectations/documentation.md), and [code quality](../expectations/code-quality.md).

## Outcome

- The product is reliable, secure, supportable, maintainable, and honest about platform capability.
- Install, update, rollback, uninstall, backup/export, privacy, retention, signing, crash reporting, and support paths are proven.
- Security, legal/compliance, threat model, and abuse-resistance reviews are explicit.

## Acceptance

- Production release claims match actual signing, packaging, installer, store, and entitlement state.
- Updater rollback and rollback-failure claims name the update channel and
  remain manual-required until signed-channel execution and rollback smoke proof
  exist.
- Release-support runbook claims distinguish preview-only draft readiness from
  published production support execution.
- Parents can export or delete family data according to documented custody and retention behavior.
- Source shape, tests, docs, and validation remain maintainable under production scale.

## Validation

- Run `npm run validate`.
- Include package install/update/uninstall smoke, signing/notarization/store evidence where applicable, threat model review, and final CI green on `main`.
