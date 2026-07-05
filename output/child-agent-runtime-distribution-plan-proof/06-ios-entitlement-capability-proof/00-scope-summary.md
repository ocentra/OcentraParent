plan: child-agent-runtime-distribution-plan
workpack: 06-child-ios-agent-capability-package
owner: crates/schema
platform: ios
artifact_kind: capability-proof-contract
package_path: test-results/child-ios-entitlement-capability-proof/proof.json
checksum_ref: n/a
sbom_ref: n/a
signing_state: manual-required
install_state: manual-required
runtime_state: manual-required
respawn_state: unsupported
uninstall_state: not-applicable
device_owner_state: not-applicable
managed_profile_or_supervision_state: manual-required
setup_trust_handoff_ref: n/a
run_id: n/a
command_id: child-ios-entitlement-capability-proof
correlation_id: wp06-rust-owned-ios-capability-proof

# WP06 Scope Summary

Current WP06 execution state is `complete`.

What is real:

- The canonical shared WP06 contract/read-model owner is Rust-first in `crates/schema/src/child_ios_entitlement_capability_proof.rs`.
- The Rust owner emits the checked-in generated TS edge at `packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts`.
- The remaining `schema-domain` surface is a thin adapter only at `packages/schema-domain/src/child-ios-entitlement-capability-proof.ts`.
- Real tests live in:
  - `crates/schema/tests/contract/child_ios_entitlement_capability_proof.rs`
  - `packages/schema-domain/tests/proof/child-ios-entitlement-capability-proof.test.ts`
- The focused proof runner is real at `scripts/test/child-ios-entitlement-capability-proof.mjs` and writes `test-results/child-ios-entitlement-capability-proof/proof.json`.
- The checked-in generated TS contract now matches the Rust-exported content again, so `generated_child_ios_entitlement_contracts_stay_checked_in` passes in the focused Rust contract test.
- The iOS status scaffold keeps capability-only and no-daemon truth explicit:
  - `service-mode=capability-only`
  - `launch-availability=manual-required`
  - `recovery=not-implemented`
  - `daemon=not-claimed`
  - `child-agent-parity=not-claimed`

Validation closure:

- `cargo test -p ocentra-schema --test contract child_ios_entitlement_capability` passes.
- `cmd /c npm run build --workspace @ocentra-parent/schema-domain` passes.
- `cmd /c npm exec --workspace @ocentra-parent/schema-domain -- vitest run tests/proof/child-ios-entitlement-capability-proof.test.ts` passes.
- `cmd /c npm run test:child-ios-entitlement-capability-proof` passes.
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts packages/schema-domain/src/child-ios-entitlement-capability-proof.ts packages/schema-domain/tests/proof/child-ios-entitlement-capability-proof.test.ts scripts/test/child-ios-entitlement-capability-proof.mjs` passes.
- `cargo lint-architecture crates/schema/src/lib.rs crates/schema/src/child_ios_entitlement_capability_proof.rs crates/schema/src/child_ios_entitlement_capability_proof_ts.rs crates/schema/tests/contract.rs crates/schema/tests/contract/child_ios_entitlement_capability_proof.rs` passes.
- `cmd /c npm run type-check --workspace @ocentra-parent/schema-domain` now passes in the current checkout, clearing the last focused WP06 blocker.

Proof root:

- `output/child-agent-runtime-distribution-plan-proof/06-ios-entitlement-capability-proof/`

Legacy note:

- The older `output/child-agent-runtime-distribution-plan-proof/06-child-ios-agent-capability-package/` path is historical only and must not be used for current WP06 status.
