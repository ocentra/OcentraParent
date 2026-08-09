# App-plan WP01 local validation proof

plan: `app-plan`
workpack: `01-contract-boundary-and-effect-schemas`
owner: `packages/schema-domain` and `crates/app-core`
proof_tier: contract/schema and Rust runtime-decision boundary
status: locally-validated; CI/main merge still pending

## Source and custody

- Canonical TypeScript edge decoder: `packages/schema-domain/src/app-runtime-decision.ts`
- Rust-owned contract fixtures: `crates/app-core/tests/contract/fixtures/`
- Rust runtime owner: `crates/app-core/src/runtime_decision.rs`
- Raw command output: `output/app-plan-proof/01-contract-boundary-and-effect-schemas/`

## Commands and results

| Command | Result |
| --- | --- |
| `npm run build --workspace @ocentra-parent/schema-domain` | passed |
| `npm run test --workspace @ocentra-parent/schema-domain -- app-runtime-decision.test.ts` | 11 passed |
| `npm run type-check --workspace @ocentra-parent/schema-domain` | passed |
| `cargo test -p ocentra-app-core app -- --nocapture` | 8 focused tests passed |
| `npm run lint:architecture -- --files <selected app contract and runtime files>` | passed |
| `npm run hub:guard` | passed; no findings or merge risks |

## Negative coverage

The schema-domain contract suite rejects display-name-only identity, malformed
aggregate/decision identifiers, inventory rows claiming foreground evidence,
AI/manual-required rows publishing policy, and known/unknown foreground states
that violate the required handoff boundary. Rust invariant and contract tests
cover the corresponding runtime matrix and event envelope.

## No-claim boundary

This proves the app-only contract and Rust runtime-decision boundary locally.
It does not prove installed-app inventory, process/foreground capture on a real
host, portal/service integration, policy enforcement, platform parity, fresh
CI, PR review, or merge to `main`.
