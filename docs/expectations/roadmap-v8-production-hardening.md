# V8 Production Hardening Expectations

This is the milestone-specific expectation file for V8 in `docs/product-roadmap.md`.

Supporting expectation files: [release installer](release-installer.md), [sync and export](sync-export.md), [static analysis and security](static-analysis-security.md), [documentation](documentation.md), and [code quality](code-quality.md).

## Outcome

- The product is reliable, secure, supportable, maintainable, and honest about platform capability.
- Install, update, rollback, uninstall, backup/export, privacy, retention, signing, crash reporting, and support paths are proven.
- Security, legal/compliance, threat model, and abuse-resistance reviews are explicit.

## Acceptance

- Production release claims match actual signing, packaging, installer, store, and entitlement state.
- Parents can export or delete family data according to documented custody and retention behavior.
- Source shape, tests, docs, and validation remain maintainable under production scale.

## Validation

- Run `npm run validate`.
- Include package install/update/uninstall smoke, signing/notarization/store evidence where applicable, threat model review, and final CI green on `main`.
