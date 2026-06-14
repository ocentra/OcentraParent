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

Pipeline PR-ready means the combined product path is proved with artifacts,
screenshots, validation logs, known gaps, and non-claims.

## Checklist

- [ ] Workpack checklists complete. Browser URL-trigger integration and
      browser/network/mobile/broad action adapters remain explicit non-complete
      gates.
- [ ] Proof artifacts written under `output/screen-ai-pipeline-proof`.
- [ ] UI screenshots captured.
- [ ] Feature docs/checklist updated for the current B proof stack; central
      product checklist is sequenced by hub ownership.
- [ ] Known gaps documented.
- [ ] Non-claims documented.
- [ ] Focused validations run.
- [ ] Full validation run or omission approved for the current pushed branch.

## Proof

- Final DONE/PR-ready report includes branch, commit, pushed state, touched
  paths, validation, proof artifacts, screenshots, known gaps, non-claims, and
  PR body outline.
- Current live-operator artifact gate branch
  `codex/screen-live-operator-artifact-gate` starts from `origin/main`
  `a6cc14d5` after PR326, acknowledges PR329 as fix-ready, and validates the
  existing live operator proof set with
  `node scripts/test/screen-ai-live-operator-artifact-gate.mjs`. The gate
  writes
  `output/screen-ai-pipeline-proof/live-operator-artifact-gate/proof-summary.json`
  and preserves the non-claims that it does not rerun the operator session or
  prove managed-browser trigger ownership.
