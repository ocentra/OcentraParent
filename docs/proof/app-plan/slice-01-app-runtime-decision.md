# App-plan WP01 local validation proof

plan: `app-plan`
workpack: `01-contract-boundary-and-effect-schemas`
owner: `packages/schema-domain` and `crates/app-core`
proof_tier: contract/schema and Rust runtime-decision boundary
status: complete for the contract slice; broad app-plan/product integration remains open

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
| PR #643 CI run `31366692141` | passed; all required checks and package previews green |
| Normal main merge | passed; merge commit `47a2ac717` |

## Negative coverage

The schema-domain contract suite rejects display-name-only identity, malformed
aggregate/decision identifiers, inventory rows claiming foreground evidence,
AI/manual-required rows publishing policy, and known/unknown foreground states
that violate the required handoff boundary. Rust invariant and contract tests
cover the corresponding runtime matrix and event envelope.

## No-claim boundary

This proves the app-only contract and Rust runtime-decision boundary through
focused local validation plus fresh CI and normal main merge. It does not prove
installed-app inventory, process/foreground capture on a real host,
portal/service integration, policy enforcement, or platform parity.
