# packages/child-runtime-domain

## Current Refresh Audit - 2026-06-19

- Status: the prior audit content in this file was effectively stale; it only contained the heading, so there was no preserved findings history to reconcile.
- Responsibility: this package owns child-runtime contract boundaries for Android, iOS, and shared child-domain runtime events, plus the read-model proofs that keep those boundaries honest.
- Dependencies: `@ocentra-parent/schema-domain`, `@ocentra-parent/capability-domain`, `@ocentra-parent/event-domain`, `@ocentra-parent/family-domain`, and `@ocentra-parent/setup-domain`.

### Violations

- There is no direct TS/JS re-export barrel in the inspected sources, but `packages/child-runtime-domain/package.json` uses a broad `./* -> ./dist/*.js` export shim that hides the actual source ownership surface behind a wildcard.
- `src/package-info.ts` is a one-line metadata shim, which is fine for identity, but it also means the package has almost no explicit owned entrypoint besides the wildcard export map.
- The package is heavily copy-pasted around the same schema/table pattern: literal maps, `Schema.Literal(...)`, `Schema.Struct(...)`, `Schema.filter(...)`, and `as const` truth tables repeat across the Android, iOS, and shared child-runtime files.
- The current tests prove parse behavior, but they do not exercise any real runtime logger/proof-chain instrumentation, so the proof story is schema-only and not chain-visible.

### Duplicated Shapes

- `src/child-runtime-gates.ts` repeats the same state vocabulary twice or three times: literal table, schema, and parsed constant object for each state family.
- `src/child-domain-runtime-events.ts` duplicates the event taxonomy in literal form, schema form, and runtime object form, then repeats the same domain-prefix coherence logic in a small family of helpers.
- The Android proof files repeat the same read-model pattern: build a base schema, add arrays of “required” rows, then validate with a dedicated honesty predicate.
- The iOS entitlement proof and mobile capability proof mirror the Android pattern almost exactly, just with different labels and boundary strings.
- The unit tests repeat the same structure per file: one positive parse, several negative parse mutations, and local helper builders that restate the same valid rows in slightly different forms.

### Barrel / Re-export / Shim Debt

- No `export *` or `pub use` style barrel is present in the inspected TypeScript source boundary.
- The debt is instead at package level: the wildcard export map in `package.json` acts like a distribution shim for the entire `dist` tree.
- If this package is meant to stay narrow and explicit, the wildcard export is the first place the surface is broadened beyond the source files themselves.

### Schema Drift

- `src/child-runtime-gates.ts` is the highest drift-risk file because the same states are represented as literals, schemas, helper predicates, and parsed constants; any future rename must be updated in multiple places.
- `src/child-domain-runtime-events.ts` has similar drift risk because event ownership is enforced by prefix and cross-domain exceptions rather than a single shared authority table.
- The Android/iOS proof files are mostly honest today, but each one has enough repeated row facts that a new capability or surface can easily be added in one file and forgotten in the test fixture or claim boundary list.
- The current refresh is honest as of 2026-06-19, but it is only honest relative to the inspected source snapshot; the old audit did not capture any of this state.

### Event / Log / Proof Misuse

- The domain and proof files encode the contract shape, but the unit tests do not emit structured logger milestones or proof-chain evidence.
- That means the package proves “schema accepted or rejected” rather than “chain entry, owner, event, consumer, and stop point” as required by the repo logging standard.
- The shared event envelope test is the closest thing to chain proof, but it still only asserts parse success and field equality; it does not validate any logged boundary evidence.

### Test / Proof Structure Issues

- The test suite is contract-heavy but not very DRY itself: each file builds its own valid model, its own row helpers, and its own negative mutations.
- Several test files use top-level helper calls that only register `it(...)` blocks, which makes the file harder to scan than a flat `describe` with grouped cases.
- The assertions are generally strong and exact, but the suite is still mostly “shape proof” rather than “boundary proof”; there is no logger setup, no proof artifact capture, and no chain milestone assertion.
- The shared timestamp and envelope tests are better than the larger proof fixtures because they check exact event type, ownership, and timestamp relationships, but they still stop short of proving a logged chain.

### DRY Score

- Score: 43/100.
- Reason 1: the package has real boundary ownership and mostly clear intent, so this is not random duplication.
- Reason 2: the same schema/table/honesty pattern is repeated across many files with only label changes, which is classic copy/paste drift pressure.
- Reason 3: the tests and proof helpers mirror the source pattern instead of sharing a small contract-builder layer, so every boundary change currently has many manual touch points.

### Fix Recommendation

- Extract a small internal contract-builder layer for repeated “literal table -> schema -> parsed const -> honesty predicate” shapes, starting with `src/child-runtime-gates.ts` and the Android/iOS proof files.
- Collapse the repeated test fixture builders into shared per-boundary factories so the valid model and the negative mutation helpers are declared once per family.
- Add structured logger-ready proof milestones to the tests that are intended to prove chain behavior, not just parse behavior.

### Decouple Recommendation

- Keep child-runtime-gates, child-domain-runtime-events, Android proof files, iOS proof files, and tests decoupled by contract family, not by generic utility dumping ground.
- If a shared helper is introduced, make it an explicit contract-builder module with ownership in this package, not a random “helpers” file.
- Preserve the manual-required / blocked / planned distinctions as first-class schema values; do not collapse them into looser generic status enums just to reduce duplication.

### Blockers

- The package is already modeling unimplemented or manual-required device, entitlement, and transport boundaries, so some duplication is intentional guardrail duplication.
- Real device, Apple entitlement, and external transport artifacts are not available in the inspected boundary, so the honest proof state must stay conservative.
- Any refactor that changes the wildcard export surface needs compatibility review because consumers may currently rely on `package.json` wildcard imports from the published dist tree.

### Exact Likely Paths

- `packages/child-runtime-domain/package.json`
- `packages/child-runtime-domain/src/package-info.ts`
- `packages/child-runtime-domain/src/child-runtime-gates.ts`
- `packages/child-runtime-domain/src/child-domain-runtime-events.ts`
- `packages/child-runtime-domain/src/mobile-child-agent-capability-proof.ts`
- `packages/child-runtime-domain/src/child-ios-entitlement-capability-proof.ts`
- `packages/child-runtime-domain/src/child-android-storage-protocol-proof.ts`
- `packages/child-runtime-domain/src/child-android-service-protocol-proof.ts`
- `packages/child-runtime-domain/src/child-android-privileged-capability-proof.ts`
- `packages/child-runtime-domain/src/child-android-permission-capability-proof.ts`
- `packages/child-runtime-domain/src/child-android-lifecycle-proof.ts`
- `packages/child-runtime-domain/src/child-android-device-proof-artifact-gate.ts`
- `packages/child-runtime-domain/tests/unit/*.test.ts`
- target_kind: TypeScript domain package
- owned_paths: packages/child-runtime-domain/package.json; packages/child-runtime-domain/src/package-info.ts; packages/child-runtime-domain/src/child-runtime-gates.ts; packages/child-runtime-domain/src/child-domain-runtime-events.ts; packages/child-runtime-domain/src/mobile-child-agent-capability-proof.ts; packages/child-runtime-domain/src/child-ios-entitlement-capability-proof.ts; packages/child-runtime-domain/src/child-android-storage-protocol-proof.ts; packages/child-runtime-domain/src/child-android-service-protocol-proof.ts; packages/child-runtime-domain/src/child-android-privileged-capability-proof.ts; packages/child-runtime-domain/src/child-android-permission-capability-proof.ts; packages/child-runtime-domain/src/child-android-lifecycle-proof.ts; packages/child-runtime-domain/src/child-android-device-proof-artifact-gate.ts
- declared_responsibility: Child Android, child iOS, and mobile child-agent runtime proof contracts.
- observed_responsibility: Owns child runtime gate schemas, event envelope and event-type contracts, branded child-domain ids, and Android/iOS/mobile proof read models.
- should_own: Shared child runtime gate schemas, event type literals, branded ids, proof read models, and parser-backed contract surfaces.
- should_not_own: Runtime execution, event bus wiring, platform behavior, or any consumer-specific orchestration logic.
- allowed_dependencies: @ocentra-parent/schema-domain; @ocentra-parent/setup-domain/readiness; @ocentra-parent/event-domain; @ocentra-parent/capability-domain; @ocentra-parent/family-domain.
- suspicious_dependencies: The package mixes several adjacent contract families, which makes it easy for proof scaffolds to drift independently instead of sharing a smaller core.
- expected_dependents: `crates/child-runtime`, policy/notification/enforcement crates, proof runners, and any app or service that consumes child runtime contracts.
- shared_contract_schema_usage: Strong Effect Schema usage throughout, but the contract surface is split across gate literals, event envelopes, mobile capability proofs, and Android/iOS proof read models instead of a compact shared helper layer.
- duplicate_or_near_duplicate_shapes: `child-runtime-gates.ts` and `child-domain-runtime-events.ts` both define literal consts, schemas, inferred types, and frozen value objects; the Android proof files repeat the same schema-version/surface/runtime-owner/proof-state/claim-boundary/read-model skeleton; `mobile-child-agent-capability-proof.ts` and `child-ios-entitlement-capability-proof.ts` are near-mirror catalogs.
- id_name_status_drift: Overlapping `ChildRuntime*`, `ChildDomain*`, Android, and iOS ids/states are easy to desynchronize if a string changes in one file but not its parallel read model or test.
- direct_import_vs_event_boundary: Correctly contract-only, but some files blend event-envelope shape, runtime gate checks, and proof catalog data in one place, which makes ownership broader than it needs to be.
- event_bus_usage: None directly; this package defines contracts and proof scaffolds, not bus behavior.
- logging_and_proof_chain_expectations: Mostly exempt as static contract surface, but its schemas define proof-chain expectations for downstream crates and should remain redaction-safe and stable.
- boundary_violations: No obvious runtime boundary breach, but the package is overcrowded with adjacent proof families that likely want a smaller common contract core.
- dry_common_core_candidates: Extract a shared proof scaffold helper for `schema version + surface + proof state + runtime owner + claim boundaries + read model + honesty filter`; reuse it across Android, iOS, and mobile-child-agent proof catalogs.
- dead_frontage_or_shims: `package-info.ts` is metadata frontage; several proof modules are also frontage-heavy wrappers around repeated read-model scaffolding.
- proposed_fix_packets: Split repeated proof scaffolds into helper modules, keep event-envelope contracts separate from proof catalogs, and reduce mirrored Android/iOS catalog duplication where the structure is identical.
- severity: medium
- confidence: medium-high
- evidence_paths: packages/child-runtime-domain/package.json; packages/child-runtime-domain/src/package-info.ts; packages/child-runtime-domain/src/child-runtime-gates.ts; packages/child-runtime-domain/src/child-domain-runtime-events.ts; packages/child-runtime-domain/src/mobile-child-agent-capability-proof.ts; packages/child-runtime-domain/src/child-ios-entitlement-capability-proof.ts; packages/child-runtime-domain/src/child-android-storage-protocol-proof.ts; packages/child-runtime-domain/src/child-android-service-protocol-proof.ts; packages/child-runtime-domain/src/child-android-privileged-capability-proof.ts; packages/child-runtime-domain/src/child-android-permission-capability-proof.ts; packages/child-runtime-domain/src/child-android-lifecycle-proof.ts; packages/child-runtime-domain/src/child-android-device-proof-artifact-gate.ts; packages/child-runtime-domain/tests/unit/*.test.ts

## Current Refresh Audit - 2026-06-19
- responsibility: This package owns child runtime gate schemas, child runtime event/domain contracts, and Android/iOS/mobile child-agent proof read models; it should stay contract-only and not drift into runtime execution, bus wiring, or consumer orchestration.
- deps: `@ocentra-parent/schema-domain/effect`, `@ocentra-parent/setup-domain/readiness`, `@ocentra-parent/event-domain/primitives`, `@ocentra-parent/event-domain/eventing`, `@ocentra-parent/capability-domain/capabilities`, and `@ocentra-parent/family-domain/reference-primitives`.
- violations: No hard runtime breach found, but the package is still overloaded with adjacent contract families in one surface, and `packages/child-runtime-domain/package.json:7-12` makes every leaf module public through `./*`, which is a broad contract shim rather than a narrow package boundary.
- duplicated shapes: `src/child-runtime-gates.ts:168-404` repeats the same literal/schema/frozen-value-object pattern for several gate families; `src/child-domain-runtime-events.ts:32-472` repeats the event taxonomy in literal, schema, map, and filter form; the Android, iOS, and mobile proof files repeat `schemaVersion + surfaces + claimBoundaries + honesty filter` scaffolds with only the vocabularies changed.
- barrel/reexport/shim debt: No TS/Rust re-export barrels were found in this package, which is good; the main shim debt is the wildcard export map in `package.json`, plus the metadata frontage in `src/package-info.ts:1` and its identity-only test in `tests/unit/package-info.test.ts:3-7`.
- schema drift: Any new child runtime state or event now has to be updated in multiple places, especially `src/child-domain-runtime-events.ts`, `src/child-runtime-gates.ts`, and the per-platform proof files; the `ChildDomainRuntimeEventTypeLiteral` and its schema/object mirror are the highest-risk drift point.
- event/log/proof misuse: The package does not log, which is correct, but its proof names often read like evidence claims while they are still static contracts; the main misuse risk is downstream code treating these schemas as runtime proof rather than as claim-boundary validators, especially in `src/mobile-child-agent-capability-proof.ts:157-468` and the Android/iOS read models.
- test/proof structure issues: The tests are useful negative/positive parser checks, but they mostly mirror the same fixture-driven shape per file instead of centralizing shared contract fixtures; `tests/unit/child-domain-runtime-events.test.ts:10-34` and `tests/unit/child-runtime-gates.test.ts:24-225` cover basic validation, but the broader suite still follows the same repeated pattern for each proof family.
- score: 6/10, because the package is strongly schema-typed and mostly honest, but the contract surface is too wide and too duplicated for long-term drift resistance.
- fix recommendation: Extract a small shared proof-scaffold helper for repeated read-model patterns, and centralize the event taxonomy plus gate-state tables so each new child runtime state only changes one ownership file per concern.
- decouple recommendation: Split the broad child-runtime contract core from the platform-specific proof catalogs, or at minimum split the shared taxonomy/gate core from the Android/iOS/mobile read models so the public package surface narrows and the duplicated honesty filters can be reused.
- blockers: No validation blocker for the audit itself; the refactor blocker is downstream compatibility because consumers can import every leaf module through `./*`, so the package surface cannot be narrowed casually without an import audit.
- exact likely paths: `packages/child-runtime-domain/package.json`; `packages/child-runtime-domain/src/package-info.ts`; `packages/child-runtime-domain/src/child-runtime-gates.ts`; `packages/child-runtime-domain/src/child-domain-runtime-events.ts`; `packages/child-runtime-domain/src/mobile-child-agent-capability-proof.ts`; `packages/child-runtime-domain/src/child-ios-entitlement-capability-proof.ts`; `packages/child-runtime-domain/src/child-android-storage-protocol-proof.ts`; `packages/child-runtime-domain/src/child-android-service-protocol-proof.ts`; `packages/child-runtime-domain/src/child-android-privileged-capability-proof.ts`; `packages/child-runtime-domain/src/child-android-permission-capability-proof.ts`; `packages/child-runtime-domain/src/child-android-lifecycle-proof.ts`; `packages/child-runtime-domain/src/child-android-device-proof-artifact-gate.ts`; `packages/child-runtime-domain/tests/unit/*.test.ts`.
