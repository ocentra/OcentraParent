# Source Surface Status Matrix

Purpose: track which shared Cloudflare surfaces exist now versus which remain scaffold-only or manual-required.

| Slice | Source path | Exists now? | Current implementation state | Next required action | Manual-required / no-claim state |
| --- | --- | --- | --- | --- | --- |
| 01 | `infra/cloudflare/package.json` | Yes | scaffold-only | Replace placeholder scripts with real runnable dependencies and logs. | No runtime claim until commands have proof. |
| 02 | `infra/cloudflare/wrangler.toml` | Yes | placeholder bindings present | Replace placeholder IDs and names with real dev resources. | No deploy claim. |
| 02 | `infra/cloudflare/wrangler.production.toml` | Yes | placeholder bindings present | Replace placeholder IDs and names with real production resources and promotion proof. | No production claim. |
| 02 | `infra/cloudflare/.dev.vars.example` | Yes | placeholder env present | Replace dummy values only in local private vars, never in repo. | No secret claim. |
| 03 | `infra/cloudflare/src/index.ts` | Yes | safe stub / no-claim runtime | Replace manual-required responses with real handlers without weakening fail-fast guards. | No runtime success claim. |
| 04 | `infra/cloudflare/src/routes.ts` | Yes | route manifest scaffold present | Bind manifest entries to domain-owned request/response contracts. | No consumer contract proof yet. |
| 05 | `infra/cloudflare/src/auth/verifier.ts` | Yes | adapter placeholder present | Wire the real account/session provider after ownership decisions land. | `account-auth-adapter-manual-required` until chosen. |
| 06 | `infra/cloudflare/src/env.ts` | Yes | binding interface scaffold present | Add real validation and binding proofs. | No binding validation claim. |
| 07 | `infra/cloudflare/scripts/test-runner.ts` | Yes | exact blocker runner present | Replace blocker output with real suite execution and logs. | No test-runner proof yet. |
| 07 | `infra/cloudflare/scripts/seed-local.ts` | Yes | exact blocker seed placeholder present | Replace blockers with real fixture and teardown flows. | No seed proof yet. |
| 08 | `infra/cloudflare/tests/` | Yes | placeholder test family files present | Replace placeholder files with real assertions and artifacts. | No test pass claim. |
| 08 | `infra/cloudflare/docs/TESTING.md` | Yes | module-local pointer doc present | Keep module docs synchronized with runnable suites and blockers. | No runtime proof claim. |
| 12 | `docs/proof/cloudflare-control-plane-plan/wp12-payment-plan-handoff-gate/payment-handoff-proof.md` | No | missing | Add accepted handoff proof once slices 00-11 are explicit. | Payment remains blocked. |
