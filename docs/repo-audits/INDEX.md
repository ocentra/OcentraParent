# Repo Audits Index

This folder holds repo-level truth audits used for coordination across plan threads.

| Audit | Purpose | Status |
| --- | --- | --- |
| [2026-06-17 Structural Truth Audit](2026-06-17-structural-truth-audit.md) | Baseline structural audit for test topology, CI coverage, architecture gates, ownership drift, and DRY/common-core risks before reviewing per-plan thread reports. | Active baseline |

## Current active audit

Use `2026-06-17-structural-truth-audit.md` when reviewing each pasted plan-thread self-assessment.

Each report should be checked against:

- actual source owners;
- real executable tests;
- empty scaffold folders;
- inline source tests;
- tracked proof generator commands;
- local or CI run evidence;
- architecture gate scope;
- ownership and DRY/common-core risks;
- dependency blockers.
