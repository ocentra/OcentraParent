# WP03 Unsupported Manual Required Proof

- plan: `policy-control-plane-plan`
- workpack: `03-domain-policy-compilers`
- proof id: `policy-compiler.unsupported-manual-required`
- owner lane: `codex-a`
- date: `2026-06-15`

## Scope

This proof covers the explicit unsupported and manual-required capability
states. They must remain first-class outcomes instead of being auto-upgraded,
silently dropped, or collapsed into a fake green state.

## Evidence

- `crates/policy-control-core/src/policy_compiler.rs`
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

- Unsupported capability remains explicit rather than silently dropped.
- Manual-required capability remains explicit rather than auto-promoted to
  supported.
- Consumer seams do not flip to `dry-run-ready` when the compiler says the
  target is unsupported.

## Teardown / rollback

- No runtime state was modified.
- No teardown or rollback was required beyond the local test runs.

## Remaining gaps

- Broader downstream parent-surface consumer coverage remains open.
- Route-gated proof closure for WP03 is complete.
