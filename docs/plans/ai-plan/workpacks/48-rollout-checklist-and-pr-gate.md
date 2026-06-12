# 48 - Rollout Checklist And PR Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `48 - Rollout Checklist And PR Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

No AI slice reports complete without contracts, tests, proof artifacts, docs,
known gaps, non-claims, and UI screenshots when UI changed.

## Where We Are

Existing AI proof scripts are useful, but AI needs a stronger plan-level DONE
gate because it is a core safety subsystem.

## Checklist

- [ ] Workpack checklist filled.
- [ ] Source docs updated.
- [ ] Product checklist updated or explicit no-update reason.
- [ ] Contract tests run.
- [ ] Rust parity/service tests run where touched.
- [ ] Proof scripts run.
- [ ] UI screenshots captured where UI changed.
- [ ] Real browser-use capture analysis proof exists when screen-derived AI is in scope.
- [ ] Real app-use capture analysis proof exists when screen-derived AI is in scope.
- [ ] Timed cadence capture analysis proof exists when screen-derived AI is in scope.
- [ ] Final capture plus analysis plus policy/action proof is either out of scope for this AI PR or completed under `docs/plans/screen-ai-pipeline-plan`.
- [ ] Security negative tests run where boundary touched.
- [ ] Event topology manifest includes AI mesh events.
- [ ] No AI mesh event is orphaned unless explicitly accepted.
- [ ] Mesh bridge proof exists if cross-device behavior is touched.
- [ ] AI work lifecycle replay proof exists.
- [ ] Claim/lease/idempotency proof exists.
- [ ] Cross-device provider rejection proof exists.
- [ ] Child policy authority proof exists.
- [ ] Mobile dormant/fallback proof exists if mobile behavior touched.
- [ ] Raw screenshot transfer negative proof exists for screen-derived jobs.
- [ ] Remote/API child-safety rejection still passes.
- [ ] `git diff --check` run.
- [ ] lane/hub guards run.
- [ ] `npm run validate` run or omission approved.

## Proof

- PR/DONE report includes branch, commit, touched files, validations, proof
  artifacts, UI screenshots, known gaps, non-claims, and PR body outline.
