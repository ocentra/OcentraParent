# Workpack Index

| Workpack | Purpose | Status |
| --- | --- | --- |
| [00-games-infra-parity-extraction](workpacks/00-games-infra-parity-extraction.md) | Extract the games module pattern and reduce it for Parent. | Planned |
| [01-cloudflare-module-scaffold](workpacks/01-cloudflare-module-scaffold.md) | Create the repo-local module tree and no-claim scaffold. | Planned |
| [02-wrangler-env-bindings](workpacks/02-wrangler-env-bindings.md) | Define Wrangler envs, bindings, vars, and secret custody. | Planned |
| [03-worker-entrypoint-runtime-guards](workpacks/03-worker-entrypoint-runtime-guards.md) | Freeze env validation, CORS, request-size, kill-switch, and scheduled hook behavior. | Planned |
| [04-route-manifest-and-domain-contracts](workpacks/04-route-manifest-and-domain-contracts.md) | Define route groups and contract ownership. | Planned |
| [05-auth-admin-support-boundary](workpacks/05-auth-admin-support-boundary.md) | Define auth states and private/admin/webhook trust gates. | Planned |
| [06-storage-do-d1-kv-r2-queue-bindings](workpacks/06-storage-do-d1-kv-r2-queue-bindings.md) | Freeze storage and coordination ownership. | Planned |
| [07-local-dev-seeding-and-fixtures](workpacks/07-local-dev-seeding-and-fixtures.md) | Define local dev, fixtures, and seed flows. | Planned |
| [08-testing-runner-and-test-pyramid](workpacks/08-testing-runner-and-test-pyramid.md) | Define the Cloudflare test runner shape and required test families. | Spec-complete / implementation-open |
| [09-portal-to-worker-e2e-smoke](workpacks/09-portal-to-worker-e2e-smoke.md) | Define first consumer smoke and portal handoff proof. | Planned |
| [10-security-fuzz-property-observability](workpacks/10-security-fuzz-property-observability.md) | Reduce games security tooling to the Parent baseline. | Spec-complete / implementation-open |
| [11-deployment-and-environment-promotion](workpacks/11-deployment-and-environment-promotion.md) | Define environment promotion, deploy, and rollback flow. | Planned |
| [12-payment-plan-handoff-gate](workpacks/12-payment-plan-handoff-gate.md) | Gate payment on real Cloudflare control-plane assumptions. | Planned |

Each workpack must name its proof path under `docs/proof/cloudflare-control-plane-plan/` and the exact blocker if runtime work is not ready.
