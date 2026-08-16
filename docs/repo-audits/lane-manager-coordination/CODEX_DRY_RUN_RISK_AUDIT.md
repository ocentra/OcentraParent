# Codex Dry-Run Risk Audit

This file records the mental dry run of a Codex lane manager reading the repo-audit coordination folder and trying to execute it.

## Why this exists

The coordination docs are only useful if Codex cannot misread them into broad scans, expensive validation, wrong-layer refactors, or proof theater.

## Dry-run result

| Risk | How Codex could misunderstand | Fix required |
| --- | --- | --- |
| Phase 0 says `Run` | Codex could start source edits while doing structural inventory. | Make Phase 0 inventory/doc-only unless a workpack explicitly assigns a script. |
| Global structural work sounds broad | Codex could scan full repo and all plan docs. | Add read budget and path-bounded inventory rules. |
| Validation rule says full gate before handoff | Codex could run `npm run validate`, full `cargo test`, CodeQL, Playwright, or all proof scripts after small docs work. | Add validation budget ladder and require lane-manager approval for expensive gates. |
| `proof root` wording | Codex could commit `output/` or `test-results/`. | Restate generated artifacts are local/CI evidence unless a tracked proof doc is assigned. |
| Logger-ready rule | Codex could add logging to pure schema/constants or spam production runtime. | Keep exemptions and enablement ladder explicit. |
| Event-driven rule | Codex could wrap schema imports in events. | Keep direct imports allowed for schemas/constants/contracts/brands/parsers. |
| Per-thread instructions | Codex could treat first slice as permission to implement whole plan. | Require one assigned slice, exact paths, exact stop condition. |
| Path locks | Codex could assume broad package lock gives permission to edit all files under package. | Require exact files or subtrees, not broad package names, before source edits. |
| Test topology | Codex could move tests before inventory. | Phase 0 inventory first; moving tests is later per-thread source work. |
| DRY/common-core | Codex could extract common code before tests. | Require behavior tests before extraction. |
| Logging proof | Codex could assert logs exist but not inspect artifacts. | Proof must include run id/correlation id and artifact path. |
| UI proof | Codex could use screenshot-only Playwright. | Playwright proof must pair UI state with log/event/read-model artifact. |
| Eventing | Codex could create a plan-local event bus. | Reuse `ocentra-eventing` unless an explicit exception exists. |

## Immediate patches from dry run

1. Add a validation budget ladder.
2. Add Phase 0 read/write limits.
3. Add exact dispatch packet requirements.
4. Add expensive-command denylist unless explicitly approved.
5. Add generated-artifact handling rule.
6. Add source-edit stop conditions.
