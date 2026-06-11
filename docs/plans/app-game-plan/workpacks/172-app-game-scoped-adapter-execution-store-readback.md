# WP172 App/Game Scoped Adapter Execution Store Readback

## Scope

Connect the WP171 scoped adapter execution evidence bridge to real persisted
service evidence by reading the latest `activity.enforcement.audit-recorded`
row from `ActivityStore` when
`agent.activity.app-game.adapter-dispatch-result.read-model.get` runs.

This is limited to the single scoped Windows owned-process app/game timer row.
The read model must not upgrade broad installed-app blocking, platform
enforcement outside that scoped boundary, provider delivery, child-device
delivery, raw private rows/targets, or private diagnostics.

## Implementation

- Add a narrow `ActivityStore` query for the latest enforcement audit
  `fields_json` row.
- Feed that query into the existing app/game adapter dispatch result read-model
  command before it renders execution evidence.
- Keep the existing side-effect-free read-model builder available for tests and
  callers that provide explicit evidence.
- Preserve parent-safe portal rendering through the existing portal-domain
  panel.

## Validation

- `node scripts/test/app-game-scoped-adapter-execution-store-readback-proof.mjs`
- `cargo test -p ocentra-parent-agent-core activity_store_enforcement_audit`
- `cargo test -p ocentra-parent-agent-service app_game_adapter_dispatch_result`
- `git diff --check`
- `npm run lanes:guard`
- `npm run hub:guard`

## Non-Claims

- Broad installed-app blocking execution is not implemented.
- macOS, Linux, Android, and iOS enforcement remain manual-required,
  unavailable, unsupported, or not claimed according to existing proof rows.
- Provider delivery, provider receipt ingestion, and child-device runtime
  delivery are not implemented by this workpack.
- Raw private source rows, raw target values, and private diagnostics remain out
  of parent-facing payloads.
