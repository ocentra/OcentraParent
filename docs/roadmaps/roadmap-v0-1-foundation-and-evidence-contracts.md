<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.1 Foundation And Evidence Contracts Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.1 Foundation And Evidence Contracts Expectations

This is the milestone-specific expectation file for V0.1 in `docs/product-roadmap.md`.

Supporting expectation files: [feature request](../expectations/feature-request.md), [universal done](../expectations/universal-done.md), [code quality](../expectations/code-quality.md), [static analysis and security](../expectations/static-analysis-security.md), [contracts](../expectations/contracts.md), and [release installer](../expectations/release-installer.md).

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
