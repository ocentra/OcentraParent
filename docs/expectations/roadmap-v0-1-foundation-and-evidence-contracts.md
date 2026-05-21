# V0.1 Foundation And Evidence Contracts Expectations

This is the milestone-specific expectation file for V0.1 in `docs/product-roadmap.md`.

Supporting expectation files: [feature request](feature-request.md), [universal done](universal-done.md), [code quality](code-quality.md), [static analysis and security](static-analysis-security.md), [contracts](contracts.md), and [release installer](release-installer.md).

## Outcome

- The repo has contract-first TypeScript and Rust boundaries before product behavior expands.
- Local development, validation, CI, hooks, dependency policy, SBOM, installer scaffolding, and update scaffolding exist.
- Activity, journal, and query contracts are explicit and test-backed.

## Acceptance

- Full validation passes locally and in CI.
- TypeScript and Rust contract parity is tested for shared shapes.
- README and agent docs explain product intent, local dev, and validation.
- Future feature work has an obvious contract/package/crate boundary.

## Validation

- Run `npm run validate`.
- Confirm package-preview and CI workflows remain honest about scaffolded versus production-ready claims.
