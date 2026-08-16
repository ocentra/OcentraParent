<!-- agent-capsule -->

> Agent Capsule
> Plan: `cloudflare-control-plane-plan`
> Doc: `Cloudflare Control Plane Checklist Index`
> Kind: exact checklist router.
> Read when: a selected workpack references checklist rows.
> Stop rule: do not scan unrelated checklist rows.
> Proves: checklist routing only.
> Does not prove: implementation completion.
> Proof rule: a checkbox can be checked only after proof artifacts and focused command results exist.

<!-- /agent-capsule -->

# Cloudflare Control Plane Checklist Index

> **Live-code audit (2026-07-17):** [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) records current implementation, blockers, dependencies, and next unblocker. Rows remain proof-gated; this audit does not check unsupported work.

## Fill rules

- Leave a checkbox unchecked until proof exists.
- Every checked row must cite one or more proof artifacts from `PROOF_INDEX.md`.
- Every proof item must list exact commands run, pass/fail/blocker, and no-claim boundaries.
- Do not mark runtime-ready from scaffold/docs/placeholder tests.
- Do not mark payment handoff ready until WP12 consumes all required proof roots.

| Row | Owning workpack | Close when |
| --- | --- | --- |
| CF-00 | WP00 | Games parity map records keep/adapt/strip decisions and rejects game-only surfaces. |
| [ ] CF-01 | WP01 | Module tree and package scripts are present in source, but this checkout retains no proof bundle under `output/cloudflare-control-plane-plan-proof/01-cloudflare-module-scaffold/`. First reconcile `wrangler` with `@cloudflare/workers-types`, retain `03-package-dependency-graph.md` plus the scoped lint, unit, architecture, negative-case, and teardown evidence, then check this row. WP07 remains blocked until that clean graph is retained. |
| CF-02 | WP02 | Wrangler envs, bindings, vars, secret names, and dev/prod separation are documented/proven. |
| CF-03 | WP03 | Entry guard chain, env validation, request-size/origin behavior, kill switch, and scheduled hook shape are explicit. |
| CF-04 | WP04 | Route manifest, route groups, and domain contract ownership are explicit. |
| CF-05 | WP05 | Auth/admin/support/webhook states and adapter boundaries are explicit, with unsupported provider assumptions blocked. |
| CF-06 | WP06 | DO/D1/KV/R2/Queue ownership and coordination/storage responsibilities are explicit. |
| CF-07 | WP07 | Local dev, fixture, seed, teardown, and missing-runtime blockers are explicit. |
| CF-08 | WP08 | Test runner commands, required test files, exact assertion IDs, and blockers are mapped. |
| CF-09 | WP09 | Portal-to-worker smoke scope, redaction, and no-claim boundary are explicit. |
| CF-10 | WP10 | Security/property/fuzz/observability cases are parent-scoped, redacted, and fail-closed. |
| CF-11 | WP11 | Promotion, rollback, env separation, and secret custody are explicit. |
| CF-12 | WP12 | Payment handoff proof exists, blockers are explicit, and spec completeness is not treated as runtime readiness. |

## Broad blockers

- [ ] No private/admin/support route readiness without WP05 proof.
- [ ] No storage/binding readiness without WP06 proof.
- [ ] No test-pyramid readiness without WP08 proof.
- [ ] No production deploy readiness without WP11 proof.
- [ ] No payment runtime handoff without WP12 proof.
