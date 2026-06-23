# WP03 Contract Boundary And Effect Schemas

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP03 Contract Boundary And Effect Schemas`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Create schema-backed tracking contracts before runtime code consumes location, status, geofence, alert, retention, AI, or acknowledgement data.

## Central schema boundary

```text
schema-domain owns canonical cross-boundary tracking schemas.
tracking-domain may define private helpers/projections/proof adapters.
tracking-core may mirror canonical contracts for runtime use.
```

Any schema used as a public contract, event payload, protocol shape, read-model DTO, portal input, policy input, notification input, custody/export shape, or proof metadata must live in `schema-domain` or an approved neutral boundary.

## Source Inputs

- `docs/expectations/location-geofence.md`
- `docs/device-location-tracking-schema-proposal.md`
- `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`
- `docs/plans/tracking-plan/v0-5-location-tracking-full-scope-plan.md`

## Target State

Effect Schema contracts exist for every tracking contract family, with branded ids and decode helpers. No Zod, no manual string brands, and no raw app/runtime string literals.

## Required proof fields

```text
canonical_schema_owner_state
private_helper_state
rust_mirror_state
public_contract_state
invalid_state_rejection
raw_string_state
manual_brand_state
cross_boundary_state
no_runtime_claim
no_product_ready_claim
no_claim
```

## Tests And Proof

Proof root: `output/tracking-plan-proof/03-contract-boundary-and-effect-schemas/`

- `00-source-snapshot.md`
- `01-contract-proof.log`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Add canonical contracts before runtime consumers.
- [ ] Add parser/brand tests and invalid-state tests.
- [ ] Mirror Rust only after canonical contracts are explicit and tested.
- [ ] Keep external input as `unknown` until parsed.
- [ ] Update docs/checklist rows if proof status changes.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope` under the proof root below. Runtime, platform, provider, and UI behavior is not claimed beyond the proof state recorded in `proof-summary.json` and the implementation checklist.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [ ] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [ ] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [ ] Proof artifacts under `output/tracking-plan-proof/03-contract-boundary-and-effect-schemas/`.
- [ ] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
