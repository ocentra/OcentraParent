# Execution Blueprint

Status: reset.

## Execution objective

Turn the Cloudflare control-plane route from a games-derived concept into a Parent-owned module scaffold, tests/proof plan, and payment handoff gate.

## Slice gates

| Slice | Primary docs/workpacks | First-touch source surface | Entry criteria | Exit criteria | Proof pointer | Rollback / teardown |
| --- | --- | --- | --- | --- | --- | --- |
| 00. Games parity extraction | `AGENTS.md`, `PLAN_STATE.md`, `GAMES_INFRA_PARITY_MAP.md`, WP00 | `GAMES_INFRA_PARITY_MAP.md` | Named games files inspected | Keep/adapt/strip table is explicit and consumer-safe | `docs/proof/cloudflare-control-plane-plan/wp00-games-infra-parity-extraction/parity-extraction-proof.md` | Restore any copied scope to `strip` or `manual-required` |
| 01. Module scaffold | WP01 | `infra/cloudflare/package.json` | Plan route is selected | Module tree exists and is marked scaffold-only where needed | `docs/proof/cloudflare-control-plane-plan/wp01-cloudflare-module-scaffold/module-scaffold-proof.md` | Remove unsupported files or mark them manual-required |
| 02. Wrangler env and bindings | WP02 | `infra/cloudflare/wrangler.toml`; `infra/cloudflare/wrangler.production.toml`; `infra/cloudflare/.dev.vars.example` | Module scaffold exists | Bindings, env names, and secret custody are explicit | `docs/proof/cloudflare-control-plane-plan/wp02-wrangler-env-bindings/wrangler-binding-proof.md` | Revert unsupported binding claims to manual-required |
| 03. Worker entrypoint runtime guards | WP03 | `infra/cloudflare/src/index.ts` | Wrangler/env model exists | Fail-fast guard chain and scheduled hook shape are explicit | `docs/proof/cloudflare-control-plane-plan/wp03-worker-entrypoint-runtime-guards/entrypoint-guard-proof.md` | Remove any implied runtime success without test proof |
| 04. Route manifest and contracts | WP04 | `infra/cloudflare/src/routes.ts` | Guard chain exists | Route groups, auth states, and domain-owned contracts agree | `docs/proof/cloudflare-control-plane-plan/wp04-route-manifest-and-domain-contracts/route-manifest-proof.md` | Revert raw route additions or consumer-only contracts |
| 05. Auth, admin, support, webhook trust | WP05 | `infra/cloudflare/src/auth/verifier.ts`; `AUTH_BOUNDARY_MODEL.md` | Route groups exist | Auth states, adapter interface, and protected route rules are explicit | `docs/proof/cloudflare-control-plane-plan/wp05-auth-admin-support-boundary/auth-boundary-proof.md` | Remove unsupported auth-provider assumptions |
| 06. Storage and coordination | WP06 | `infra/cloudflare/src/env.ts`; `STORAGE_BINDING_MODEL.md` | Auth boundary exists | DO/D1/KV/R2/Queue ownership is explicit | `docs/proof/cloudflare-control-plane-plan/wp06-storage-do-d1-kv-r2-queue-bindings/storage-binding-proof.md` | Revert child-data or unsupported storage claims |
| 07. Local dev and seeding | WP07 | `LOCAL_DEV_AND_SEEDING_MODEL.md` | Bindings are named | Local start, seed, fixture, and teardown path is explicit | `docs/proof/cloudflare-control-plane-plan/wp07-local-dev-seeding-and-fixtures/local-dev-proof.md` | Revert any unproven seed/runtime claims |
| 08. Test runner and test pyramid | WP08 | `infra/cloudflare/scripts/test-runner.ts`; `TESTING_STRATEGY.md`; `REQUIRED_TEST_ASSERTION_MATRIX.md` | Local dev path exists | Test families, exact assertions, commands, and blocker states are explicit | `docs/proof/cloudflare-control-plane-plan/wp08-testing-runner-and-test-pyramid/test-pyramid-proof.md` | Revert any fake "implemented" test claims |
| 09. Portal-to-worker smoke | WP09 | `docs/proof/cloudflare-control-plane-plan/wp09-portal-to-worker-e2e-smoke/` | Test runner shape exists | First consumer smoke scope is explicit and redaction-safe | `docs/proof/cloudflare-control-plane-plan/wp09-portal-to-worker-e2e-smoke/portal-smoke-proof.md` | Remove any UI/runtime success claim without proof |
| 10. Security, property, fuzz, observability | WP10 | `SECURITY_PRIVACY_OBSERVABILITY.md`; `TESTING_STRATEGY.md`; `REQUIRED_TEST_ASSERTION_MATRIX.md` | Test pyramid exists | Security/property/fuzz coverage and carried observability assertions are explicit and scoped to Parent | `docs/proof/cloudflare-control-plane-plan/wp10-security-fuzz-property-observability/security-baseline-proof.md` | Revert inherited game tooling that Parent does not need |
| 11. Deployment and promotion | WP11 | `infra/cloudflare/wrangler.production.toml`; `DEPLOYMENT_MODEL.md` | Scaffold and bindings exist | Promotion, rollback, and secret custody are explicit | `docs/proof/cloudflare-control-plane-plan/wp11-deployment-and-environment-promotion/deployment-model-proof.md` | Revert any production claim without env separation |
| 12. Payment handoff gate | WP12 | `docs/proof/cloudflare-control-plane-plan/wp12-payment-plan-handoff-gate/payment-handoff-proof.md` | Slices 00-11 have explicit outputs or blockers | Payment assumptions, missing runtime blockers, and no-claim boundaries are explicit | `docs/proof/cloudflare-control-plane-plan/wp12-payment-plan-handoff-gate/payment-handoff-proof.md` | Restore payment to blocked state if the handoff becomes stale |

## Required order

1. Select exactly one slice/workpack.
2. Read the slice docs and proof rows only.
3. Write or update the smallest scope for that slice.
4. Record exact validation or exact blocker for that slice.
5. Capture negative-case and rollback expectations.
6. Update the proof pointer, blueprint, and next-actions queue.
7. Do not unblock payment until slice 12 proof exists.

## Stop rules

- Do not let payment or portal docs become the source of truth for shared Cloudflare scaffolding.
- Do not claim runtime coverage from placeholder scripts, docs, or file trees.
- Do not confuse spec completeness with runtime readiness.
- Do not move to the payment handoff gate until the module, auth, bindings, and test pyramid each have explicit state.
