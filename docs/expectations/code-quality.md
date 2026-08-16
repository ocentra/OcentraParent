<!-- agent-capsule -->

> Agent Capsule
> Doc: Code Quality Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Code Quality Expectations

Code quality is a product feature in this repo. Future agents should not rely on taste, memory, or convention alone. The repo should force quality through contracts, lint rules, tests, source-shape checks, and CI.

## Required

- Keep modules small and owned by one reason to change.
- Split files before they become catch-all modules.
- Prefer shared domain packages over app-local constants.
- Prefer Rust protocol constants over inline service/core strings.
- Prefer explicit typed adapters over generic bags of data.
- Keep async request handlers responsive.
- Move blocking OS, filesystem, database, or provider calls behind deliberate blocking boundaries.
- Avoid global mutable state unless the ownership and lock scope are clear.
- Prefer deterministic state transitions over clever parallelism.
- Avoid broad refactors unrelated to the feature.

## Forbidden

- God files.
- God classes.
- Fake green tests.
- Mocked service behavior.
- New Zod usage.
- Manual string brands.
- Raw domain-bearing strings.
- UI-only demos that bypass the real agent or real contracts.
- Product claims that are not backed by implementation.

## Done Signal

The feature passes source-shape, string-boundary, type, lint, and test-double guards without exemptions. The implementation should be easy to inspect by file ownership: contracts, adapters, service handlers, UI, and tests are not mixed into one large module.
