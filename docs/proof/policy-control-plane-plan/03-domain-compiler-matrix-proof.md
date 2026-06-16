# WP03 Domain Compiler Matrix Proof

- plan: `policy-control-plane-plan`
- workpack: `03-domain-policy-compilers`
- proof id: `policy-compiler.contract-matrix`
- owner lane: `codex-a`
- date: `2026-06-15`

## Scope

This proof covers the shared compiler contract matrix: the control plane emits
typed, versioned, domain-targeted artifacts with explicit capability state
instead of letting each consumer invent its own target vocabulary.

## Evidence

- `crates/policy-control-core/src/policy_compiler.rs`
- `crates/policy-control-core/tests/unit/policy_compiler.rs`
- `crates/policy-control-core/tests/version-skew/policy_compiler.rs`
- `packages/policy-domain/src/policy-compiler.ts`
- `packages/policy-domain/tests/unit/policy-compiler.test.ts`
- `packages/app-game-domain/src/app-game-policy-target-compiler.ts`
- `packages/app-game-domain/tests/unit/app-game-policy-target-compiler.test.ts`
- `packages/browser-domain/src/browser-control-coverage-matrix.ts`
- `packages/browser-domain/tests/unit/browser-control-coverage-matrix.test.ts`
- `packages/browser-domain/src/browser-game-policy-compiler.ts`
- `packages/browser-domain/tests/unit/browser-game-policy-compiler.test.ts`
- `packages/browser-domain/src/social-policy-compiler.ts`
- `packages/browser-domain/tests/unit/social-policy-compiler.test.ts`
- `packages/tracking-domain/src/tracking-policy-compiler-runtime-proof.ts`
- `packages/tracking-domain/tests/contract/tracking-policy-compiler-runtime-proof.test.ts`

## Validation

- `cargo test -p ocentra-policy-control-core --test unit policy_compiler -- --test-threads=1`
  - pass
- `cargo test -p ocentra-policy-control-core --test version_skew policy_compiler -- --test-threads=1`
  - pass
- `cargo test -p ocentra-policy-control-core --test unit policy_delivery -- --test-threads=1`
  - pass
- `cargo test -p ocentra-policy-control-core --test version_skew policy_delivery -- --test-threads=1`
  - pass
- `npm run test --workspace @ocentra-parent/policy-domain`
  - pass
- `npm run test --workspace @ocentra-parent/app-game-domain -- tests/unit/app-game-policy-target-compiler.test.ts`
  - pass
- `npm run test --workspace @ocentra-parent/browser-domain -- tests/unit/browser-control-coverage-matrix.test.ts tests/unit/browser-game-policy-compiler.test.ts tests/unit/social-policy-compiler.test.ts`
  - pass
- `npm run test --workspace @ocentra-parent/tracking-domain -- tests/contract/tracking-policy-compiler-runtime-proof.test.ts`
  - pass

## Negative-case evidence

- Unsupported capability remains explicit rather than silently dropped.
- Manual-required capability remains explicit rather than being auto-upgraded
  to supported.
- Compiler outputs stay typed and versioned instead of becoming source truth.
- Consumer domains do not claim enforcement authority from the compiler
  artifact alone.

## Teardown / rollback

- No runtime state was modified.
- No teardown or rollback was required beyond the local test runs.

## Remaining gaps

- Broader downstream parent-surface consumer coverage remains open until the
  remaining focused reruns land.
- Route-gated proof closure for WP03 is complete.
