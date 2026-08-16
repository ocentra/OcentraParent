<!-- agent-capsule -->

> Agent Capsule
> Doc: BLOCKED: Activity Surface Needs Tauri/Rust Contract Wiring
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# BLOCKED: Activity Surface Needs Tauri/Rust Contract Wiring

Paste this to the primary hub when C is ready to pull a main-backed implementation branch:

```text
BLOCKED/REQUEST: Please create a main-backed implementation slice for the parent portal Activity surface data contract and Tauri/Rust command wiring.

Context:
- C has a UI-only Activity surface in `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx`.
- The current UI intent seam is `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`.
- That seam is deliberately marked with `TODO(activity-surface-tauri)` and returns UI-check data only.
- This app is not meant to fetch this data from Vite. Vite is only the HMR/dev shell. Final wiring should go through the parent portal Tauri/Rust path.

Activity surface functions the UI now expects:
- `getDailyReport(request)`
- `getWeeklyReport(request)`
- `getMonthlyReport(request)`
- `saveActivityReport(report)`
- `listHistoricalReports(request)`
- `getScreenActivity(request)`
- `getAppUseActivity(request)`
- `getBrowserActivity(request)`
- `getGamesActivity(request)`
- `getNetworkActivity(request)`

Behavioral intent:
- Scope can be `family` or `device`.
- Family report generation fans out to all available child devices, requests report material, aggregates reachable responses, and records unavailable/offline sources.
- Per-device report generation requests only the selected device.
- Historical reports are stored as JSON first. The UI list expects file metadata: file name, date, range, summary, saved state, and a parsed report document.
- The viewer should display parsed user-facing report sections, not a raw JSON dump.
- Generate creates an unsaved draft report. Save persists the draft through the chosen Data storage target. Data-page storage selection can be stubbed until the Data surface is wired.
- Non-report Activity tabs need user-facing read models for Screen, App Use, Browser, Games, and Network. They should return structured view data scoped to family or selected device.

Requested implementation scope:
1. Add shared Effect Schema contracts under `packages/activity-domain` for:
   - activity surface target/scope
   - report frequency
   - report request
   - report list item
   - report document and report sections
   - activity tab view rows for screen/app/browser/games/network
2. Add portal/Tauri-facing command names and response contracts in the appropriate protocol/domain package. Avoid naked strings in app/runtime source.
3. Add Rust protocol parity in `crates/agent-protocol`.
4. Add Rust service/Tauri command adapter stubs that return real typed unavailable or local-read-model responses. Do not pretend Vite owns this data.
5. Add tests for the TypeScript contracts, Rust protocol shape, and the command adapter boundary.
6. After merge to `main`, C can pull and replace `activity-ui-intent.ts` with the real adapter without redesigning the Activity UI.

Validation expected:
- TypeScript contract tests for accepted and rejected report requests/responses.
- Rust protocol tests for serialized command/response parity.
- Focused portal smoke proving the Activity surface can call the adapter and render Reports plus Screen/App Use/Browser/Games/Network states.

Known UI temporary state:
- C currently keeps UI-check fake activity data visible for layout inspection.
- The UI intentionally hides raw TODO text from users while keeping TODO markers in source for the next wiring pass.
```
