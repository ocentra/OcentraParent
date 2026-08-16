# WP165 App/Game Dashboard Platform Capability Limitation Rows

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP165 App/Game Dashboard Platform Capability Limitation Rows`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Surface the existing app/game platform-extension proof-pack readiness rows in the
main App/Game Sessions dashboard intent. The dashboard must keep native apps and
native games on one shared low-level evidence spine while making macOS, iOS,
Android, and Linux platform capability limitations parent-visible.

## Implementation

- Add an optional `activityAppGamePlatformExtensionReadModel` activity adapter
  result to the parent portal activity intent.
- Pass that read model to the app/game dashboard intent without importing a new
  parent-domain package export while another lane owns package export changes.
- Convert platform proof-pack readiness rows into parent-visible dashboard
  limitation rows, aggregate metrics, capability summaries, and evidence-drawer
  rows.
- Keep provider dispatch targets, raw platform diagnostics, raw private source
  rows, raw targets, adapter execution, broad blocking, child-device delivery,
  platform enforcement, provider delivery, and policy execution unclaimed.

## Proof

- Focused portal intent test covers populated app/game rows plus platform
  readiness rows for macOS, iOS, Android, and Linux.
- Test asserts:
  - `Platform gaps` metric is visible.
  - `Adapter executed` remains zero.
  - macOS and Android rows preserve manual-required/not-executed states.
  - evidence drawer contains platform proof rows with proof-ref counts.
  - private provider dispatch targets and raw platform diagnostics do not leak
    into the dashboard intent.

## Validation

- `cmd /c npm run test --workspace @ocentra-parent/portal -- --run tests/activity-ui-app-game-dashboard-intent.test.ts`
- `cmd /c npm run build --workspace @ocentra-parent/portal`
- `git diff --check`
- `node scripts/check-no-test-doubles.mjs`
- `node scripts/check-source-shape.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## Status

Done on `codex/app-game-control-product-completion`.

Central `docs/product-capability-checklist.md` remains intentionally untouched
because another lane owns product checklist churn. Runtime adapter execution and
real platform adapter proof remain open; this workpack is parent-visible
capability limitation UI proof only.
