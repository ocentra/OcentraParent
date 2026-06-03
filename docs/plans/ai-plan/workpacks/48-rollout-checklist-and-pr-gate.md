# 48 - Rollout Checklist And PR Gate

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
- [ ] `git diff --check` run.
- [ ] lane/hub guards run.
- [ ] `npm run validate` run or omission approved.

## Proof

- PR/DONE report includes branch, commit, touched files, validations, proof
  artifacts, UI screenshots, known gaps, non-claims, and PR body outline.
