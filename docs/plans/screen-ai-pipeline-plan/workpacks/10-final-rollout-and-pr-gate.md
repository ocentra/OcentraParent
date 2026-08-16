# 10 - Final Rollout And PR Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `10 - Final Rollout And PR Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Pipeline PR-ready means the combined product path is proved with retained artifacts, screenshots, validation logs, known gaps, and non-claims.

## Ownership boundary

```text
WP10 aggregates screen-ai-pipeline retained proof roots only.
Screen, AI, policy, enforcement, custody, portal, browser, app-game, network, tracking, agent-service, and protocol owners remain separate unless their handoff proof is explicitly accepted.
A readiness claim requires retained proof roots, command logs, known gaps, non-claims, screenshots where required, and proof-manifest alignment.
```

## Required rollout fields

The selected rollout proof must name, at minimum:

```text
rollout_gate_id
accepted_proof_roots
missing_proof_roots
carried_blockers
proof_root_state
plan_manifest_state
artifact_shape_state
architecture_gate_state
trigger_capture_state
capture_ai_state
ai_policy_state
policy_dry_run_state
journal_read_model_portal_state
custody_delete_state
live_operator_state
live_operator_artifact_gate_state
performance_backpressure_state
portal_screenshot_state
managed_browser_trigger_gap
adapter_gap_state
validation_state
known_gaps
claims_allowed
claims_blocked
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Workpack checklists complete. Browser URL-trigger integration and browser/network/mobile/broad adapter paths remain explicit non-complete gates.
- [ ] Proof artifacts written under `output/screen-ai-pipeline-proof`.
- [ ] UI screenshots captured.
- [ ] Feature docs/checklist updated for the current B proof stack; central product checklist is sequenced by hub ownership.
- [ ] Known gaps documented.
- [ ] Non-claims documented.
- [ ] Focused validations run.
- [ ] Full validation run or omission approved for the current pushed branch.

## Proof

- Final DONE/PR-ready report includes branch, commit, pushed state, touched paths, validation, proof artifacts, screenshots, known gaps, non-claims, and PR body outline.
- Current live-operator artifact gate branch `codex/screen-live-operator-artifact-gate` starts from `origin/main` `a6cc14d5` after PR326, acknowledges PR329 as fix-ready, and validates the existing live operator proof set with `node scripts/test/screen-ai-live-operator-artifact-gate.mjs`. The gate writes `output/screen-ai-pipeline-proof/live-operator-artifact-gate/proof-summary.json` and preserves the non-claims that it does not rerun the operator session or prove managed-browser trigger ownership.

## Failure conditions

- Do not claim PR_READY while `output/screen-ai-pipeline-proof/` is absent.
- Do not claim slice closure while `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` is absent.
- Do not claim product readiness from mock-only, source-only, happy-path-only, or screenshot-only proof.
- Do not claim enforcement readiness from dry-run proof.
- Do not claim managed-browser trigger ownership from live-operator proof.
- Do not claim live rerun from artifact-gate proof.
- Do not claim custody readiness without retained deletion/retention evidence.
