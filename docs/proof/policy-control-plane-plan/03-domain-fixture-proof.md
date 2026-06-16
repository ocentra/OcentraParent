# WP03 Domain Fixture Proof

- plan: `policy-control-plane-plan`
- workpack: `03-domain-policy-compilers`
- proof id: `policy-compiler.domain-fixture`
- owner lane: `codex-a`
- date: `2026-06-15`

## Scope

This proof covers the real compiler fixtures used by the package and crate
tests. The fixture data must preserve domain-specific compiler inputs without
inventing plan-only shapes or hidden fixtures.

## Evidence

- `crates/policy-control-core/tests/unit/policy_compiler.rs`
- `crates/policy-control-core/tests/unit/policy_delivery.rs`
- `packages/policy-domain/tests/unit/policy-compiler.test.ts`
- `packages/app-game-domain/tests/unit/app-game-policy-target-compiler.test.ts`
- `packages/browser-domain/tests/unit/browser-control-coverage-matrix.test.ts`
- `packages/browser-domain/tests/unit/browser-game-policy-compiler.test.ts`
- `packages/browser-domain/tests/unit/social-policy-compiler.test.ts`
- `packages/tracking-domain/tests/contract/tracking-policy-compiler-runtime-proof.test.ts`

## Validation

- `cargo test -p ocentra-policy-control-core --test unit policy_compiler -- --test-threads=1`
  - pass
- `cargo test -p ocentra-policy-control-core --test unit policy_delivery -- --test-threads=1`
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

- Fixtures do not mutate runtime or source truth.
- Unsupported and manual-required states remain visible in the fixture-driven
  compiler outputs.
- Dry-run-ready output cannot bypass the compiler seam.

## Teardown / rollback

- No runtime state was modified.
- No teardown or rollback was required beyond the local test runs.

## Remaining gaps

- Broader downstream consumer additions are complete in this checkout.
- Route-gated proof closure for WP03 is complete.
