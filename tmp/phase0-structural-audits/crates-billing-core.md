# crates/billing-core
- target_kind: Rust crate
- owned_paths: crates/billing-core/Cargo.toml; crates/billing-core/src/lib.rs; crates/billing-core/src/billing_subscription.rs
- declared_responsibility: Billing and subscription provider lifecycle boundary, including webhook intake, lifecycle classification, entitlement update requirements, and child entitlement snapshot handling.
- observed_responsibility: A direct billing decision core that converts provider events into decisions and projection events without a surrounding event bus.
- should_own: Provider webhook classification, billing lifecycle transitions, entitlement write/no-write decisions, and child entitlement snapshot consumption decisions.
- should_not_own: Account/admin UI contracts, proof/read-model snapshots, or support workflows that belong in the TS billing domain package.
- allowed_dependencies: ocentra-eventing; serde.
- suspicious_dependencies: None obvious in manifest; the main risk is conceptual overlap with the much larger `billing-domain` package.
- expected_dependents: billing runtime adapters, provider webhook handlers, entitlement projection code, and downstream billing read-model consumers.
- shared_contract_schema_usage: Shares the same eventing contract/idempotency pattern as the rest of the repo and mirrors billing contract vocabulary found in the TS package.
- duplicate_or_near_duplicate_shapes: The provider/lifecycle/entitlement decision enums mirror TS billing-state vocabularies and may be drifting from the TS support-admin/read-model shapes.
- id_name_status_drift: The crate name is narrow but the module implements a full billing lifecycle, including child entitlement consumption projection.
- direct_import_vs_event_boundary: Uses direct domain events and projections only; no separate event bus or message routing layer is present.
- event_bus_usage: None.
- logging_and_proof_chain_expectations: Event constructors and decision helpers should remain deterministic and redaction-safe if logging is added later.
- boundary_violations: The crate owns both intake and projection decisions for adjacent billing flows, which risks overlapping with TS billing domain proof and account/support surfaces.
- dry_common_core_candidates: Shared billing lifecycle taxonomy between this crate and packages/billing-domain; a smaller common billing-state core could reduce duplication.
- dead_frontage_or_shims: `lib.rs` is just a one-module entrypoint, so there is no clear dead frontage.
- proposed_fix_packets: Keep the Rust crate as the canonical provider-lifecycle core and move any shared billing-state vocabulary into a tighter cross-language contract layer.
- severity: medium
- confidence: high
- evidence_paths: crates/billing-core/Cargo.toml; crates/billing-core/src/lib.rs; crates/billing-core/src/billing_subscription.rs; packages/billing-domain/src/billing-entitlement.ts; packages/billing-domain/src/billing-entitlement-runtime-proof.ts; packages/billing-domain/src/billing-support-admin-boundary.ts

## Current Refresh Audit - 2026-06-19

- Boundary summary: `crates/billing-core/src/lib.rs` is only a module entrypoint (`pub mod billing_subscription;`), so there is no Rust barrel/re-export debt in the crate root. The active boundary is `crates/billing-core/src/billing_subscription.rs`.
- Finding 1, medium: the Rust module still owns intake, lifecycle classification, entitlement transition projection, child entitlement snapshot consumption, and idempotency-key shaping in one place. That is coherent for a small core, but it is a broad responsibility slice at `crates/billing-core/src/billing_subscription.rs:33-683`.
- Finding 2, medium: the Rust lifecycle and proof vocabulary still mirrors the TS billing boundary instead of sharing a tighter contract layer. The main overlap is with `packages/billing-domain/src/billing-entitlement.ts:41-317`, `packages/billing-domain/src/billing-entitlement-runtime-proof.ts:79-324`, and `packages/billing-domain/src/billing-support-admin-boundary.ts:33-183`. The duplicated shapes show up in the Rust enums and event structs at `crates/billing-core/src/billing_subscription.rs:59-389`.
- Finding 3, low: no live logging/proof misuse is visible in this crate boundary. The Rust code is deterministic and currently has no runtime logging, but the test shape is still unit-only under `crates/billing-core/tests/unit/*.rs`, so cross-language proof drift is not pinned by a crate-local proof harness.
- DRY score: 6/10. The core is internally tidy, but the billing lifecycle taxonomy is still duplicated across Rust and TS boundaries.
- Fix recommendation: keep `crates/billing-core` as the canonical provider-lifecycle decision core, and extract a smaller shared billing-state contract for lifecycle/status/review/write-state vocabulary before the TS and Rust boundaries drift further.
- Decouple recommendation: only split webhook decisioning from child-entitlement snapshot consumption if those flows start evolving independently; the more immediate win is shared vocabulary extraction, not a file split.
- Blockers: I did not inspect the TS value atom files directly, so the exact duplication density there remains inferred from the boundary files above. Likely next paths are `packages/billing-domain/src/billing-entitlement-values.ts` and `packages/billing-domain/src/billing-support-admin-boundary-values.ts`.
