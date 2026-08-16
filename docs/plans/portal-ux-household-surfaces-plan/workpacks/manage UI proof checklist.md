<!-- agent-capsule -->

> Agent Capsule
> Doc: Manage UI Proof Checklist
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Manage UI Proof Checklist

Status: implementation/proof companion for the `codex/parent-portal-manage-ia` Manage UI pass.

## Expectation Documents Read

- `docs/expectations/policy.md`
- `docs/expectations/browser-evidence.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/screen-evidence.md`
- `docs/expectations/network-flow-evidence.md`
- `docs/expectations/enforcement.md`
- `docs/expectations/data-custody.md`
- `docs/policy Ui fix.md`
- `docs/data and AI Ui plan.md`
- `docs/portal and account Ui fix.md`

## Shared Pattern Required

- Devices and Activity established the shared frame rhythm: header, top target/grid surface, divider, reflective tabs, straight-bottom body panel, then details.
- Policy, Data, and AI need the same Family / Per Device top target treatment when child-device state is involved.
- Policy side-panel area choices are Browser, Apps, Games, Screen, and Network.
- Policy body tabs are Rules, Schedule, Approvals, Enforcement, and Audit.
- Policy content must show parent-meaningful settings and states, not decorative informational cards.
- The portal must author/read typed intent/read-model surfaces. It must not evaluate policy, enforce, scan the OS, run timers, or invent child evidence.

## Implemented In This Pass

- Policy, Data, and AI now use the same Activity-style `DeviceChoiceGrid` target band.
- Policy target band uses Family / Per Device labels with plan-seat fake UI-check devices only (`D001`, `D002`, `D003`).
- Per Device mode disables Policy controls until a device is selected.
- Policy keeps Browser/Apps/Games/Screen/Network as side-panel area routes.
- Policy body tabs now remain Rules, Schedule, Approvals, Enforcement, and Audit.
- Policy Rules/Schedule/Approvals/Enforcement/Audit each render a pair of SVG segmented controls using `ScopeToggle`.
- Policy setting rows are area-specific:
  - Browser: managed browser boundary, exact URL evidence, unmanaged browser bypass, download requests, schedule windows, dry-run/enforcement, and audit refs.
  - Apps: installed/running inventory, unknown apps, school allowlists, new app asks, foreground budgets, process enforcement, and inventory/session audit.
  - Games: launcher/process/catalog identity, daily/weekly budgets, unknown games, blocked game asks, terminate/block capability, and session audit.
  - Screen: explicit parent opt-in, local-only summaries, cadence/triggers, raw image deletion, one-time capture/live-view asks, and summary/deletion audit.
  - Network: process/domain/IP/protocol metadata, VPN/proxy indicators, school/bedtime windows, endpoint approvals, DNS/domain/process enforcement, and flow-summary audit.
- Data and AI inherit the shared top target selector; their body content remains the planned custody/AI setup first pass until the deeper typed read models land.

## Explicit UI-Check Fake Data

- The visible devices are UI-check fixtures from the existing `ocentraActivityUiCheckFakeDeviceCount` path.
- Fixture device labels are generic ids (`D001`, `D002`, `D003`), not child names.
- Policy examples use generic terms and `.invalid` domain policy language where examples are needed.
- No fake screenshots, browser bodies, credentials, personal app paths, real domains, packet payloads, or hardcoded child names are introduced.

## Still Not Wired

- No production policy read-model contracts are added in this UI pass.
- No Rust/Tauri policy evaluator, timer, capture, enforcement, storage, AI, or connector behavior is added here.
- No policy save/preview intent is fired yet; controls are UI-state proof only.
- Data and AI still need their own deeper per-tab read-model rendering after the Policy surface is accepted.
- The fake UI-check devices must be removed or disabled once real discovery/portal-seat read models are wired.

## Proof Evidence - 2026-05-27

- `npm --workspace @ocentra-parent/portal-domain run test`: passed, 11 tests.
- `npm --workspace @ocentra-parent/portal run type-check`: passed.
- `npm --workspace @ocentra-parent/portal-domain run build`: passed; refreshed ignored `dist` contracts for the live dev portal.
- `npm --workspace @ocentra-parent/portal run test:e2e`: passed, 1 browser E2E.
- `npm run validate`: passed full root gate, including schema/source guards, Turbo lint/type-check/test, Rust validation, WebSocket local/LAN smoke, portal local smoke, and Playwright E2E.
- `git diff --check`: passed.
- `npm run lanes:guard`: passed.
- `npm run hub:guard`: passed.

Live route proof screenshots were captured from `http://127.0.0.1:4478`:

- `C:/Users/sujan/AppData/Local/Temp/ocentra-parent-manage-proof/policy-browser.png`
- `C:/Users/sujan/AppData/Local/Temp/ocentra-parent-manage-proof/policy-apps.png`
- `C:/Users/sujan/AppData/Local/Temp/ocentra-parent-manage-proof/policy-games.png`
- `C:/Users/sujan/AppData/Local/Temp/ocentra-parent-manage-proof/policy-screen.png`
- `C:/Users/sujan/AppData/Local/Temp/ocentra-parent-manage-proof/policy-network.png`
- `C:/Users/sujan/AppData/Local/Temp/ocentra-parent-manage-proof/data.png`
- `C:/Users/sujan/AppData/Local/Temp/ocentra-parent-manage-proof/ai.png`

Live text checks confirmed each screenshot route contained its expected planned anchors:

- Browser: `Policy / Browser`, `Rules`, `Managed boundary`, `Decision ladder`.
- Apps: `Policy / Apps`, `Rules`, `Inventory`, `Unknown apps`.
- Games: `Policy / Games`, `Rules`, `Game identity`, `Budgets`.
- Screen: `Policy / Screen`, `Rules`, `Explicit parent opt-in`, `Delete raw image`.
- Network: `Policy / Network`, `Rules`, `Network metadata`, `VPN / proxy`.
- Data: `Data`, `Family`, `Per Device`.
- AI: `AI`, `Family`, `Per Device`.
