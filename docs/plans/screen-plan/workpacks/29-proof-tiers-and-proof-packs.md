# 29 Proof Tiers And Proof Packs

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `29 Proof Tiers And Proof Packs`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-plan owns proof-tier definitions and screen proof-pack/report discipline.
other plans own their own proof roots and may be referenced only as handoff proof when selected.
```

## Target State

P0-P6 proof tier system, CI/fake/local/physical/authority proof artifact paths are defined and enforced in reports.

## Current State

Proof tier concept is recorded in platform deep dive. Artifact discipline is open.

## Required proof fields

The selected proof must name, at minimum:

```text
proof_tier_meaning_state
artifact_root_state
command_log_state
screenshot_state
platform_proof_state
physical_device_state
retention_proof_state
redaction_state
manual_required_state
report_template_state
claim_allowed_state
claim_blocked_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

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

## Failure conditions

- Do not claim higher-tier proof from lower-tier artifacts.
- Do not use screenshots without command/proof context as product proof.
- Do not omit non-claims from final reports.
- Do not use another plan's proof root as screen-plan closure unless a selected route accepts it as handoff proof.
