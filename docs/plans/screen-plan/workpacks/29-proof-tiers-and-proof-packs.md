# 29 Proof Tiers And Proof Packs

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `29 Proof Tiers And Proof Packs`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

P0-P6 proof tier system, CI/fake/local/physical/authority proof artifact paths are defined and enforced in reports.

## Current State

Proof tier concept is recorded in platform deep dive. Artifact discipline is open.

## Checklist

- [ ] Define proof tier meanings.
- [ ] Define artifact folder.
- [ ] Define required logs/screenshots.
- [ ] Define platform proof paths.
- [ ] Define retention proof artifacts.
- [ ] Define report template.

## Proof

- `output/screen-plan-proof/<workpack-id>/` artifacts for completed workpacks.
- Final DONE/PR-ready report links artifacts.
