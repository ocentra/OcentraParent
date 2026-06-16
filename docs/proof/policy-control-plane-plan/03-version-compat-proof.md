# WP03 Version Compat Proof

- plan: `policy-control-plane-plan`
- workpack: `03-domain-policy-compilers`
- proof id: `policy-compiler.version-compat`
- owner lane: `codex-a`
- date: `2026-06-15`

## Scope

This proof covers version compatibility across the compiler boundary. The
compiler and queued delivery records must preserve source policy version
metadata and reject incompatible version skew explicitly.

## Evidence

- `crates/policy-control-core/src/policy_compiler.rs`
- `crates/policy-control-core/tests/unit/policy_compiler.rs`
- `crates/policy-control-core/tests/version-skew/policy_compiler.rs`
- `crates/policy-control-core/src/policy_delivery.rs`
- `crates/policy-control-core/tests/unit/policy_delivery.rs`
- `crates/policy-control-core/tests/version-skew/policy_delivery.rs`
- `packages/policy-domain/src/policy-compiler.ts`
- `packages/policy-domain/tests/unit/policy-compiler.test.ts`

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

## Negative-case evidence

- Newer schema stays unsupported rather than pretending to be compatible.
- Older schema with a lower minimum policy version still requires migration.
- Queued delivery preserves source policy version and rollback provenance
  instead of erasing it.

## Teardown / rollback

- No runtime state was modified.
- No teardown or rollback was required beyond the local test runs.

## Remaining gaps

- Broader downstream consumer reruns are complete in this checkout.
- Route-gated proof closure for WP03 is complete.
