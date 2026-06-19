# packages/evidence-domain
- target_kind: TypeScript package
- owned_paths: `packages/evidence-domain/**`
- declared_responsibility: Shared evidence, subject, observer, and activity envelope contracts used by feature domains.
- observed_responsibility: Owns a compact evidence contract set: branded primitives, activity/event kinds, evidence custody evaluation, and a generic activity event envelope.
- should_own: Evidence identifiers, custody decision vocabularies, evidence reference envelopes, and the activity event contract surface.
- should_not_own: LAN-specific meaning, runtime-specific event emission, or any owner that should live in a feature package.
- allowed_dependencies: `@ocentra-parent/schema-domain`.
- suspicious_dependencies: No suspicious third-party dependency, but the package lacks a local README, so the declared responsibility is only visible through `package.json` and source shape.
- expected_dependents: Feature packages that need shared evidence envelopes, custody decisions, and activity event contracts.
- shared_contract_schema_usage: Everything here is schema-first and brand-first, with Effect Schema used for all runtime validation.
- duplicate_or_near_duplicate_shapes: `primitives.ts` repeats the same branded-id-plus-decoder pattern many times, `kinds.ts` repeats literal/object pairs, and `custody.ts` / `contracts.ts` both use small struct decision/envelope shapes that could share a tiny base helper if the package grows.
- id_name_status_drift: The package’s TS custody vocabulary is `local-only`, `family-shared`, and `exportable`, which does not line up with the Rust evidence crate’s custody scope names, so the cross-language contract is not obviously canonical yet.
- direct_import_vs_event_boundary: Direct imports stay within the package’s own primitive/kind files, which is fine for a pure contract package; there is no event bus or runtime boundary here.
- event_bus_usage: None.
- logging_and_proof_chain_expectations: This is a pure contract package, so logger-ready runtime expectations are not the main concern.
- boundary_violations: No severe boundary violation is visible in the package itself; the main risk is cross-language semantic drift rather than bad local ownership.
- dry_common_core_candidates: A shared helper for branded primitive definitions and a canonical custody vocabulary aligned with the Rust side.
- dead_frontage_or_shims: `package-info.ts` is the only explicit package-frontage file, and it is benign.
- proposed_fix_packets: Add a local README or owning doc for the package contract story, decide the canonical custody vocabulary across TS and Rust, and keep the package focused on shared evidence envelopes rather than feature-specific meaning.
- severity: medium
- confidence: medium-high
- evidence_paths: `packages/evidence-domain/package.json`, `packages/evidence-domain/src/primitives.ts`, `packages/evidence-domain/src/kinds.ts`, `packages/evidence-domain/src/contracts.ts`, `packages/evidence-domain/src/custody.ts`, `packages/evidence-domain/src/package-info.ts`

## Current Refresh Audit - 2026-06-19

- responsibility: Shared evidence-domain contracts only. This package should stay on branded primitives, canonical activity kinds, custody decisions, and the generic activity envelope. It should not acquire feature semantics or runtime side effects.
- deps: Clean and minimal. `@ocentra-parent/schema-domain` is the only declared dependency; no suspicious third-party edge is visible in the package itself.
- violations: No severe local boundary violation is visible. The main structural risk is semantic drift, not a broken import boundary.
- duplicated_shapes: `src/primitives.ts` repeats the same branded-string/decoder pattern for many ids and timestamps. `src/kinds.ts` repeats literal-plus-parsed-constant tables for observer, event, subject, and evidence kinds. `src/contracts.ts` and `src/custody.ts` both build small struct/union envelopes that could share a tiny helper if the package grows.
- barrel_reexport_shim_debt: No TS re-export barrel is present in the source package. `package.json` exports are direct package frontage, not a shim problem. `src/package-info.ts` remains the only frontage-style file and is benign.
- schema_drift: The package-level custody vocabulary is still `local-only` / `family-shared` / `exportable`, which does not obviously line up with the Rust-side evidence vocabulary. `ActivityEventSchema` is also fixed to schema version `1`, so any cross-language contract evolution will need a coordinated version story.
- event_log_proof_misuse: None in the local package runtime sense. This package is contract-only, so logger/proof-chain concerns are mostly downstream. The only misuse risk here is treating the package tests as proof of broader runtime behavior.
- test_proof_structure_issues: Tests are unit-only and focus on parse/acceptance cases. `evaluateEvidenceCustodyReference()` defines `ScopeMismatch` in its decision vocabulary but never returns it, so the decision shape and implementation are already drifting. There is no negative test that exercises scope mismatch or canonical vocabulary divergence.
- score: 72/100
- fix_recommendation: Normalize the custody decision vocabulary against the Rust evidence side, then collapse the repeated branded schema/constant patterns behind small local helpers without changing the public contract surface.
- decouple_recommendation: Keep the package strictly contract-shaped. If more feature meaning is needed, move it into the owning feature package instead of growing this package into a mixed evidence-plus-policy layer.
- blockers: No blocking source change is required for this refresh. The only blocker is cross-language truth: the canonical custody vocabulary is still not settled between TS and Rust, so any final cleanup depends on that decision.
- exact_likely_paths: `packages/evidence-domain/src/primitives.ts`, `packages/evidence-domain/src/kinds.ts`, `packages/evidence-domain/src/contracts.ts`, `packages/evidence-domain/src/custody.ts`, `packages/evidence-domain/tests/unit/custody.test.ts`, `packages/evidence-domain/tests/unit/contracts.test.ts`, `packages/evidence-domain/package.json`

## Current Refresh Audit - 2026-06-19

- responsibility: Pure shared evidence-domain contract surface. Keep it limited to branded primitives, canonical kind vocabularies, custody decision structs, and the generic activity envelope.
- dependencies: Only `@ocentra-parent/schema-domain` is declared. That dependency shape is appropriately small and does not suggest hidden runtime coupling.
- violations: No direct package boundary violation is visible in the inspected files. The main issue is contract drift and repetition, not an obvious local ownership breach.
- duplicated_shapes: `src/primitives.ts` repeats the same branded-string schema/decoder pattern for every identifier. `src/kinds.ts` repeats literal tables plus parsed constants for observer, event, subject, and evidence kinds. `src/contracts.ts` and `src/custody.ts` both build small struct/union envelopes with the same schema-first style.
- barrel_reexport_shim_debt: No TypeScript barrel or re-export shim is present in this package. `package.json` uses direct subpath exports, which is honest frontage rather than shim debt. `src/package-info.ts` is a thin package-info module and is not a problem by itself.
- schema_drift: `EvidenceCustodyScope` is still `local-only` / `family-shared` / `exportable`, while `EvidenceReferenceDecision` includes `ScopeMismatch` that the implementation never returns. That is a local contract drift signal even before comparing to any other language boundary. `ActivitySchemaVersion` is hard-coded to `1`, so cross-package evolution needs an explicit versioning path.
- event_log_proof_misuse: None locally. This package is contract-only, so proof/log concerns belong downstream. The misuse risk is treating package unit tests as proof of broader event or runtime behavior.
- test_proof_structure_issues: The likely tests here are narrow unit checks around schema parsing and custody evaluation. The current implementation of `evaluateEvidenceCustodyReference()` only returns accepted or missing, so `ScopeMismatch` is dead vocabulary unless a test or behavior path is added. There is also no visible test coverage for vocabulary alignment or version drift.
- dry_score_0_100: 70. Repetition is high enough to create maintenance drag, but the package is still structurally compact and has no barrel soup, no runtime side effects, and no obvious ownership breach.
- dry_score_reasons: The main score reducer is duplicated schema/table construction and a decision enum that outgrows its implementation. The main score preservers are the small package size, direct exports, and absence of runtime/logging complexity.
- fix_recommendation: Collapse the repeated branded primitive and literal-table patterns behind a tiny local helper layer, then make custody decision behavior and vocabulary match exactly.
- decouple_recommendation: Keep this package strictly contract-shaped. If policy or feature meaning needs to grow, move that logic into the owning feature package instead of expanding evidence-domain into a mixed contract-plus-policy layer.
- blockers: No code blocker is required for the audit itself. The unresolved blocker for cleanup is choosing the canonical custody vocabulary and decision set that should govern both the TypeScript package and the corresponding Rust-side contract.
- exact_likely_paths: `packages/evidence-domain/src/primitives.ts`, `packages/evidence-domain/src/kinds.ts`, `packages/evidence-domain/src/custody.ts`, `packages/evidence-domain/src/contracts.ts`, `packages/evidence-domain/src/package-info.ts`, `packages/evidence-domain/package.json`
