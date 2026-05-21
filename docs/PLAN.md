# Final Pre-AI Proof Gate And CI Pass

## Summary

Create the real-evidence expectation, wire it into the roadmap/proof docs, add a machine-checkable proof gate, then run the final local validation and GitHub CI. After CI is green, pause coding and move to real-machine testing on downstairs PC, upstairs PC, and Mac.

## Key Changes

- Add `docs/expectations/real-evidence-proof.md` defining the standard:
  - Rust app/service must launch.
  - Parent portal must send real requests over the real local transport.
  - Rust must reply from real OS/runtime capture or real persisted state created by app code.
  - Parent UI must render the real returned result.
  - CI proves mechanics; real machines prove privileged OS/device capabilities.
  - Fake DB rows, mocked sockets, hardcoded replies, portal-only state, MSW/Nock/Sinon/`vi.mock`/`vi.fn`, and similar fake-green paths do not count.
- Update roadmap/platform expectation docs so every completed runtime claim is tied to one of:
  - CI mechanical proof
  - real local Windows proof
  - real macOS/Linux proof
  - real Android/iOS proof
  - scaffold-only / manual-required / not-yet-proven.
- Add a proof matrix checked by `npm run test:pre-ai-proof`:
  - each completed milestone maps to proof command/job
  - each platform has an honest coverage status
  - privileged features require manual proof entries before being called fully working.
- Expand/confirm CI jobs for:
  - full validation
  - real Rust service launch
  - real parent portal to Rust E2E
  - SQLite/journal persistence through real code paths
  - package-preview smokes on Windows, Linux, macOS, Android, and iOS where available.
- Do not start new AI/enforcement work in this slice.

## Execution Order

1. Create a short-lived branch from current `main`.
2. Add the real-evidence expectation doc and proof matrix.
3. Add the pre-AI proof check script/package command.
4. Update CI workflow so the proof check and platform checkpoint run as part of the final gate.
5. Run locally on Windows:
   - `cmd /c npm run lanes:guard`
   - `cmd /c npm run hub:guard`
   - `cmd /c npm run validate`
   - `cmd /c npm run test:pre-ai-proof`
6. Commit, push, open PR with detailed scope, touched docs/scripts/workflows, validation, known gaps, and roadmap slice.
7. Watch GitHub CI.
8. Fix any CI failures on the same branch until green.
9. Merge only after green CI.
10. Pull latest `main` locally and confirm clean state.
11. Stop coding and begin real-machine proof pass.

## Real-Machine Test Pass After CI

- Windows downstairs PC:
  - launch installed/app dev build
  - verify process/window, browser bridge, network evidence, screen evidence state, SQLite/journal, portal buttons, and local policy preview.
- Windows upstairs PC:
  - pull latest `main`
  - run same app/service/portal smoke
  - verify LAN-related behavior where available.
- Mac:
  - pull latest `main`
  - run package/service/portal smoke
  - verify macOS permission states are honest before permission grants and real after grants where supported.
- Android/iOS:
  - verify current package/simulator/device launch scope honestly
  - do not claim Family Controls, DeviceActivity, UsageStats, or device-owner proof until provisioned real-device tests exist.

## Worker Coordination

- Primary owns coordination, PR, CI watching, merge, and final report.
- Workers stay parked unless a focused failure needs a branch:
  - codex-a: portal/Playwright proof failures
  - codex-b: CI/workflow/package-preview failures
  - codex-c: Rust/service/protocol proof failures.
- Worker handoffs must include detailed scope, touched files, validation commands/results, known gaps, commit state, and roadmap slice.

## Assumptions

- “Final CI” means the new pre-AI proof gate plus existing validation/package-preview CI.
- CI is required before manual up/down PC/Mac testing, but CI does not replace real privileged OS/device proof.
- Any capability that cannot be exercised in hosted CI must show honest state in CI and have a manual proof checklist before being called complete.
