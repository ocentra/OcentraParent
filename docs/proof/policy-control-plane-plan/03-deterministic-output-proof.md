# WP03 Deterministic Output Proof

- plan: `policy-control-plane-plan`
- workpack: `03-domain-policy-compilers`
- proof id: `policy-compiler.deterministic-output`
- owner lane: `codex-a`
- date: `2026-06-15`

## Scope

This proof covers deterministic compiler output. The same source input must
produce the same support matrix and capability-state rows without mutating
runtime state or introducing hidden claim drift.

## Evidence

- `crates/policy-control-core/tests/unit/policy_compiler.rs`
- `crates/policy-control-core/tests/version-skew/policy_compiler.rs`
- `packages/policy-domain/src/policy-compiler.ts`
- `packages/policy-domain/tests/unit/policy-compiler.test.ts`
- `packages/app-game-domain/tests/unit/app-game-policy-target-compiler.test.ts`
- `packages/browser-domain/tests/unit/browser-control-coverage-matrix.test.ts`
- `packages/browser-domain/tests/unit/browser-game-policy-compiler.test.ts`
- `packages/browser-domain/tests/unit/social-policy-compiler.test.ts`

## Validation

- `cargo test -p ocentra-policy-control-core --test unit policy_compiler -- --test-threads=1`
  - pass
- `cargo test -p ocentra-policy-control-core --test version_skew policy_compiler -- --test-threads=1`
  - pass
- `npm run test --workspace @ocentra-parent/policy-domain`
  - pass
- `npm run test --workspace @ocentra-parent/app-game-domain -- tests/unit/app-game-policy-target-compiler.test.ts`
  - pass
- `npm run test --workspace @ocentra-parent/browser-domain -- tests/unit/browser-control-coverage-matrix.test.ts tests/unit/browser-game-policy-compiler.test.ts tests/unit/social-policy-compiler.test.ts`
  - pass

## Negative-case evidence

- Repeated compiles produce stable support-matrix and capability-state rows.
- Compiler artifacts do not mutate runtime.
- No claim leaks into enforcement from the compiler output alone.

## Teardown / rollback

- No runtime state was modified.
- No teardown or rollback was required beyond the local test runs.

## Remaining gaps

- Broader downstream consumer reruns are complete in this checkout.
- Route-gated proof closure for WP03 is complete.
