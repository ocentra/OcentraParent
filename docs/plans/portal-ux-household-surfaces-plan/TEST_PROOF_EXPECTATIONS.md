<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Portal UX Household Surfaces Test Proof Expectations

## Proof root

```text
output/portal-ux-household-surfaces-plan-proof/<workpack-file-stem>/
```

## Common commands

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal
npm run test:e2e --workspace @ocentra-parent/portal
npm run lint:architecture -- --files packages/portal-domain apps/portal docs/plans/portal-ux-household-surfaces-plan
```

Run through `npm run agent:run --` when collecting proof if the wrapper is available.

## Command ownership notes

- `apps/portal` owns rendered route composition and focused UI/e2e proof.
- `portal-domain` owns public portal route/panel/projection contracts.
- `schema-domain`, `agent-protocol-domain`, and domain packages own typed read-model contracts consumed by portal.
- Setup, account, device-trust, LAN, browser, app-game, network, screen, tracking, AI, payment, custody, notification, and enforcement scopes run only when the selected workpack names the handoff.

## Portal UX E2E meaning

Do not use one proof family to claim the whole portal path. For this plan, E2E has separate meanings:

```text
service-backed shell/navigation E2E: route/nav/shell -> service state -> loading/degraded/error labels.
first-run/profile E2E: setup/account read model -> household/profile presentation -> no setup completion claim.
device inventory E2E: device read model -> source/capability/stale state -> no device-trust claim.
selected-device context E2E: selected device -> projected panels -> no domain runtime claim.
policy authoring/preview E2E: policy read model -> preview/confirm UI -> no policy source/enforcement claim.
requests/approval E2E: request/approval state -> role-gated UI -> no approval without account/device-trust handoff.
activity diagnostics E2E: activity/evidence read model -> diagnostics surface -> no capture truth claim.
domain-surface projection E2E: browser/app/network/screen/tracking read model -> visible projection -> no domain runtime claim.
LAN state consumption E2E: LAN read model -> selected device/peer state -> no transport proof claim.
assistant preview E2E: assistant output -> cited explanation/typed action preview -> parent confirmation boundary.
reports/notifications/custody E2E: report/notification/custody read model -> visible labels -> no custody/export/send claim.
degraded/empty/error E2E: missing/stale/error/manual-required state -> user-visible status -> no fake green.
a11y/responsive/keyboard E2E: route under keyboard/mobile/screen-reader relevant states -> no product runtime claim.
no-fake-data E2E: fixture/runtime/service source labels -> schema decode and invalid payload handling.
screenshot proof E2E: Playwright route + console/page-error check + screenshots -> review artifact only.
mobile shell readiness E2E: narrow-width/parent mobile shell scaffold -> no parent package or child mobile claim.
manual user review E2E: route list, artifacts, commands, known gaps -> user visual decision required.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Product/runtime-safe logging:

```text
redact child private data, account/session secrets, raw screenshots unless artifact-scoped, assistant chat content unless fixture-scoped, support-private notes, and private URLs or payloads
log workpack, route, viewport, role, read-model source, source/custody label, fixture/runtime state, degraded/error/manual-required state, screenshot state, accessibility state, console/page-error state, artifact pointer, and no-claim boundary when safe
separate portal projection, source contract, runtime service, domain truth, policy truth, AI runtime, transport, custody, and enforcement states
never treat screenshots, fixtures, route presence, portal-local state, or happy-path UI logs as product readiness without selected proof root and no-claim boundary
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr and screenshots by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, route, viewport, role, exit code, result, artifact pointer, diagnostics summary, manual-required note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required negative states

```text
loading/empty/error/degraded visible
manual-required visible
fake data not shown as real
UI does not own domain truth
UI does not execute device work
source/custody labels visible
browser console warnings handled or documented
screenshot proof not used as service-backed validation
portal route existence not used as product readiness
happy-path UI tests not used as PR_READY
portal projection not used as domain runtime proof
```
