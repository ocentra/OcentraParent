# OcentraHub Event Ledger

Ocentra Parent live coordination state must not live in the product repository.

The product repository owns durable hub tooling, rules, schemas, lane declarations, and semantic snapshots. Live mailbox state is a separate operational system.

## Final Model

Use a separate `OcentraHub` project as an offline-first append-only event ledger:

```text
E:\OcentraHub\
  package.json
  src\
    cli\
    core\
    materialize\
    sync\
    transports\
      file.ts
      git.ts
      http.ts
  data\
    node.json
    hubs\
      ocentra-parent\
        events.ndjson
        peers.json
        index.sqlite
        materialized\
          lanes\
            codex-b\
              inbox.md
              status.md
              ownership.json
```

The same binary can be a CLI, local cache materializer, or peer server. No central server is required.

## Product Repo Boundary

`OcentraParent` keeps:

- `.hub/hub.config.json`
- `.hub/codex-rules.md`
- `.hub/lane-ledger.json`
- `docs/hub/*.md`
- hub shims and compatibility scripts

`OcentraParent` must not keep live coordination files:

- `.hub/state/**`
- lane inbox/status/ack files
- heartbeat logs
- watch logs
- mutable ownership files
- machine-local working state

## Canonical Truth

Canonical live hub truth is an append-only event log:

```text
events.ndjson
```

SQLite indexes and materialized lane files are caches. They can be deleted and rebuilt from events.

Example event:

```json
{
  "id": "01JZ8Z7V6H3J4CGZJ9JQ9M6Y1X",
  "hub": "ocentra-parent",
  "actor": "primary",
  "type": "message",
  "to": "codex-b",
  "ts": "2026-06-10T14:00:00Z",
  "body": "Retry PR545 sync after lock release."
}
```

## Conflict Model

Events are immutable. Most events do not conflict.

Ownership is modeled as claims and releases:

```json
{ "type": "ownership.claim", "path": "apps/portal/src/live-activity-state.ts", "actor": "codex-b" }
{ "type": "ownership.release", "path": "apps/portal/src/live-activity-state.ts", "actor": "codex-b" }
```

If two lanes claim the same path, both events stay in the log and the materializer reports an ownership conflict. The primary resolves with an explicit `ownership.resolve` event. No state is overwritten.

## Sync Model

Sync is transport-agnostic:

```text
hub sync --via file://E:/OcentraHub/data/hubs/ocentra-parent
hub sync --via http://pc-a:8787
hub sync --via git
hub sync --via usb-export
```

The protocol is:

1. Compare event IDs.
2. Fetch missing events.
3. Append unknown events.
4. Dedupe by event ID.
5. Rebuild materialized views.

## Legacy Compatibility

Until the new hub exists, workers may still read the legacy root:

```text
C:\Users\sujan\.codex\ocentra-parent-hub
```

That legacy root is operational state, not product repo state. It must not be copied into `.hub/state` or committed to `OcentraParent`.

## Migration Steps

1. Stop committing `.hub/state/**` in `OcentraParent`.
2. Keep the legacy global hub root as the temporary live transport.
3. Build `OcentraHub` with event append, materialize, status, inbox, ack, report, lock, unlock, heartbeat, and sync commands.
4. Point `OcentraParent` hub shims at `.hub/hub.config.json`.
5. Make all old `npm run hub:*` commands call the OcentraHub CLI.
6. Delete disposable local caches when materialized views can be rebuilt from events.

## Guardrail

Any added or modified file under `.hub/state/**` in `OcentraParent` is a product-repo boundary violation. Deletions are allowed for this migration only.
