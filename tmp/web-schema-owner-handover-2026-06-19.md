# Web Handover: Canonical Schema Owner Completion

Date: `2026-06-19`
Branch: `codex/tracking-plan-full-continuation-a`
Workspace: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`

## Objective

Finish the repo-wide move to:

- exactly `1` canonical TypeScript schema owner package
- exactly `1` canonical Rust schema owner crate
- every other crate/package/app/tool/infra target importing shared schema from those owners only
- no stray schema ownership outside those canonical owners for shared cross-target shapes
- no TS/JS barrel or re-export frontage files
- no Rust `pub use` frontage or re-export layers
- no compatibility shims or alias hops for schema primitives
- TS and Rust schema shape drift becoming explicit and testable

## Canonical owner model

### TypeScript

Working canonical TS schema owner:

- `packages/schema-domain`

Expected outcome:

- all shared TS schema primitives live under `packages/schema-domain`
- other packages/apps/tools/infra import from `@ocentra-parent/schema-domain/...`
- `packages/family-domain` and other domain packages stop owning shared schema primitives

### Rust

Working canonical Rust schema owner for this handover:

- `crates/agent-protocol`

This is a working assumption, not a fully completed local migration.

Expected outcome:

- shared Rust protocol / wire / schema DTOs converge under `crates/agent-protocol`
- consumer crates import from the canonical owner directly
- `pub use` frontage is removed

If a specific Rust type clearly does not belong in `crates/agent-protocol`, leave a note rather than inventing a second canonical Rust schema owner.

## Repo denominator

Use `65` for repo-wide planning:

- `31` crates
- `28` packages
- `3` apps
- `2` tools
- `1` infra target

## Current local truth on this branch

### What is already done locally

For the exact legacy TS import path:

- old path: `@ocentra-parent/family-domain/reference-primitives`
- new path: `@ocentra-parent/schema-domain/family-reference-primitives`

Local branch truth now:

- repo source hits for the old path under `packages`, `apps`, `tools`, `infra`, and `crates`: `0`
- repo-wide remaining hits for that exact old path: `3`
- those `3` remaining hits are docs only, not source:
  - `tmp/phase0-structural-audits/packages-child-runtime-domain.md`
  - `tmp/phase0-structural-audits/packages-network-domain.md`
  - `tmp/phase0-structural-audits/packages-remote-access-domain.md`

The heaviest concrete local rewrite already completed in this branch was:

- `packages/app-game-domain`
- legacy `family-domain/reference-primitives` imports drained to zero in both `src` and `tests`

### What is not done yet

This branch does **not** yet prove the full repo-wide target architecture.

Still open:

- broader TS schema ownership audit across all `65` targets
- movement of shared schema definitions into `packages/schema-domain` where they still live elsewhere
- removal of remaining TS re-export / frontage debt
- Rust single-owner consolidation into the working canonical owner
- removal of Rust `pub use` frontage debt
- TS/Rust drift enforcement pass

### Validation truth so far

What is already proven locally:

- the exact old TS import path above is drained from source
- packet-level architecture lint passed repeatedly during the local migration
- direct, self-contained unit tests passed for many touched slices

What is still validation-blocked:

- several timer-service and proof-chain tests in `packages/app-game-domain/tests/unit`
- failures are due to missing `packages/test-results/.../*.json` fixture inputs
- those failures are `ENOENT` fixture reads, not evidence that the import migration itself is still wrong

### Important Windows validation note

Whole-module architecture lint on large directories can hit:

- `spawnSync ... ENAMETOOLONG`

So after any large rewrite, validation should stay packetized:

- per-file or per-small-batch `npm run lint:architecture -- --files ...`
- not one giant directory-wide command on Windows

## What web ChatGPT should do

Web ChatGPT can help as a write / move / rewrite worker.
Assume it may **not** have Node, npm, cargo, or local test execution.

That is acceptable.

Its job is:

1. edit / move / delete files to match the target architecture
2. update imports to the canonical owners
3. remove legacy exports / frontage / re-export surfaces
4. leave the repo in a coherent textual state
5. return a file-by-file change summary

My job after that:

1. inspect the returned diff against this table
2. apply or pull those edits
3. run local validation and tests packet by packet

## Rules for web ChatGPT

- Work only from this branch snapshot.
- Do not invent a second TS schema owner.
- Do not invent a second Rust schema owner unless the existing repo structure makes the current assumption impossible.
- Prefer direct imports from canonical owners.
- Remove TS/JS re-export barrels:
  - `export * from "..."`
  - `export { X } from "..."`
  - `export type { X } from "..."`
- Remove Rust re-exports:
  - `pub use ...`
  - `pub(crate) use ...`
- Remove shim-only alias files and one-hop frontages.
- Keep behavior stable.
- Do not widen into feature implementation.
- If a move is too risky without running code, prefer:
  - create destination under canonical owner
  - repoint consumers
  - leave a clear note for later removal of the legacy owner file

## Priority work order

### Priority 1: canonical TS owner

- `packages/schema-domain`
- expand it so it owns all shared TS schema primitives now scattered elsewhere

### Priority 2: legacy TS owner surfaces to drain

- `packages/family-domain`
- `packages/parent-domain`
- any other package still acting as a schema frontage

### Priority 3: largest TS consumer / local schema clusters

- `packages/app-game-domain`
- `packages/browser-domain`
- `packages/ai-domain`
- `packages/screen-domain`
- `packages/tracking-domain`
- `packages/production-domain`
- `packages/agent-protocol-domain`

### Priority 4: canonical Rust owner

- `crates/agent-protocol`

### Priority 5: Rust re-export debt

- `crates/agent-core`
- `crates/agent-service`
- `crates/ocentra-network-evidence`
- `crates/screen-ai-core`
- `crates/storage-custody-core`
- `crates/tracking-core`

## Status legend

- `canonical-ts-owner-candidate`
  - should become or remain the single TS schema owner
- `proposed-rust-owner-candidate`
  - working canonical Rust owner for this handover
- `legacy-ts-owner-surface-to-drain`
  - likely still exposes schema that should move out
- `consumer-on-new-ts-owner`
  - already imports from `schema-domain`; still audit for stray local shared schema ownership
- `architecture-debt-present`
  - measured re-export / frontage signal exists; needs explicit cleanup
- `audit-needed`
  - no strong grep signal from this pass; still must be inspected for ownership correctness

## Signal columns

- `old_ts_owner_hits`
  - files still matching `@ocentra-parent/family-domain/reference-primitives`
- `new_ts_owner_hits`
  - files matching `@ocentra-parent/schema-domain`
- `ts_reexport_signal_files`
  - file-count signal for forbidden TS re-export patterns
- `rust_pub_use_signal_files`
  - file-count signal for Rust `pub use` patterns

These are baseline signals, not final architecture verdicts.

## Focused action notes

### `packages/schema-domain`

- destination TS owner
- add shared schema primitives here
- export directly from owner, not via other domains

### `packages/family-domain`

- legacy TS surface to drain
- remove remaining shared schema ownership
- keep only family-domain logic that truly belongs here

### `packages/parent-domain`

- extremely high TS re-export signal
- likely major frontage / re-export cleanup target
- consumers should prefer owning modules directly or canonical owners

### `packages/app-game-domain`

- local old-path drain is already complete
- next step is broader shared-schema extraction only if schemas here are cross-target rather than domain-local

### `crates/agent-protocol`

- working Rust canonical owner
- absorb shared Rust schema / protocol / DTO surfaces
- remove `pub use` frontage after direct imports are updated

### `tools/no-reexports`

- signal is likely influenced by detection examples / messages
- audit manually before changing it

## 65-target baseline table

| group | name | status | old_ts_owner_hits | new_ts_owner_hits | ts_reexport_signal_files | rust_pub_use_signal_files | root |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| crate | agent-core | architecture-debt-present | 0 | 0 | 0 | 5 | crates/agent-core |
| crate | agent-protocol | proposed-rust-owner-candidate | 0 | 0 | 0 | 10 | crates/agent-protocol |
| crate | agent-service | architecture-debt-present | 0 | 0 | 0 | 3 | crates/agent-service |
| crate | agent-updater | audit-needed | 0 | 0 | 0 | 0 | crates/agent-updater |
| crate | app-core | audit-needed | 0 | 0 | 0 | 0 | crates/app-core |
| crate | app-game-core | audit-needed | 0 | 0 | 0 | 0 | crates/app-game-core |
| crate | billing-core | audit-needed | 0 | 0 | 0 | 0 | crates/billing-core |
| crate | browser-core | audit-needed | 0 | 0 | 0 | 0 | crates/browser-core |
| crate | child-ai-core | audit-needed | 0 | 0 | 0 | 0 | crates/child-ai-core |
| crate | child-enforcement-core | audit-needed | 0 | 0 | 0 | 0 | crates/child-enforcement-core |
| crate | child-notification-core | audit-needed | 0 | 0 | 0 | 0 | crates/child-notification-core |
| crate | child-policy-core | audit-needed | 0 | 0 | 0 | 0 | crates/child-policy-core |
| crate | child-runtime | audit-needed | 0 | 0 | 0 | 0 | crates/child-runtime |
| crate | entitlement-core | audit-needed | 0 | 0 | 0 | 0 | crates/entitlement-core |
| crate | family-identity-core | audit-needed | 0 | 0 | 0 | 0 | crates/family-identity-core |
| crate | lan-core | audit-needed | 0 | 0 | 0 | 0 | crates/lan-core |
| crate | logging-core | audit-needed | 0 | 0 | 0 | 0 | crates/logging-core |
| crate | network-core | audit-needed | 0 | 0 | 0 | 0 | crates/network-core |
| crate | ocentra-eventing | audit-needed | 0 | 0 | 0 | 0 | crates/ocentra-eventing |
| crate | ocentra-evidence | audit-needed | 0 | 0 | 0 | 0 | crates/ocentra-evidence |
| crate | ocentra-network-evidence | architecture-debt-present | 0 | 0 | 0 | 6 | crates/ocentra-network-evidence |
| crate | parent-runtime-core | audit-needed | 0 | 0 | 0 | 0 | crates/parent-runtime-core |
| crate | policy-control-core | audit-needed | 0 | 0 | 0 | 0 | crates/policy-control-core |
| crate | provisioning-core | audit-needed | 0 | 0 | 0 | 0 | crates/provisioning-core |
| crate | remote-access-core | audit-needed | 0 | 0 | 0 | 0 | crates/remote-access-core |
| crate | screen-ai-core | architecture-debt-present | 0 | 0 | 0 | 1 | crates/screen-ai-core |
| crate | screen-capture-adapter | audit-needed | 0 | 0 | 0 | 0 | crates/screen-capture-adapter |
| crate | screen-core | audit-needed | 0 | 0 | 0 | 0 | crates/screen-core |
| crate | screen-live-view-core | audit-needed | 0 | 0 | 0 | 0 | crates/screen-live-view-core |
| crate | storage-custody-core | architecture-debt-present | 0 | 0 | 0 | 1 | crates/storage-custody-core |
| crate | tracking-core | architecture-debt-present | 0 | 0 | 0 | 1 | crates/tracking-core |
| package | activity-domain | consumer-on-new-ts-owner | 0 | 8 | 0 | 0 | packages/activity-domain |
| package | agent-protocol-domain | consumer-on-new-ts-owner | 0 | 54 | 0 | 0 | packages/agent-protocol-domain |
| package | ai-domain | consumer-on-new-ts-owner | 0 | 54 | 14 | 0 | packages/ai-domain |
| package | app-game-domain | consumer-on-new-ts-owner | 0 | 233 | 1 | 0 | packages/app-game-domain |
| package | billing-domain | consumer-on-new-ts-owner | 0 | 25 | 0 | 0 | packages/billing-domain |
| package | browser-domain | consumer-on-new-ts-owner | 0 | 165 | 3 | 0 | packages/browser-domain |
| package | capability-domain | consumer-on-new-ts-owner | 0 | 2 | 0 | 0 | packages/capability-domain |
| package | child-runtime-domain | consumer-on-new-ts-owner | 0 | 12 | 0 | 0 | packages/child-runtime-domain |
| package | data-custody-domain | consumer-on-new-ts-owner | 0 | 4 | 0 | 0 | packages/data-custody-domain |
| package | endpoint-domain | consumer-on-new-ts-owner | 0 | 2 | 0 | 0 | packages/endpoint-domain |
| package | enforcement-domain | consumer-on-new-ts-owner | 0 | 31 | 0 | 0 | packages/enforcement-domain |
| package | event-domain | consumer-on-new-ts-owner | 0 | 2 | 0 | 0 | packages/event-domain |
| package | evidence-domain | consumer-on-new-ts-owner | 0 | 5 | 0 | 0 | packages/evidence-domain |
| package | family-domain | legacy-ts-owner-surface-to-drain | 0 | 7 | 0 | 0 | packages/family-domain |
| package | lan-domain | consumer-on-new-ts-owner | 0 | 23 | 0 | 0 | packages/lan-domain |
| package | logging-domain | consumer-on-new-ts-owner | 0 | 25 | 0 | 0 | packages/logging-domain |
| package | network-domain | consumer-on-new-ts-owner | 0 | 5 | 0 | 0 | packages/network-domain |
| package | notification-domain | consumer-on-new-ts-owner | 0 | 8 | 0 | 0 | packages/notification-domain |
| package | parent-domain | consumer-on-new-ts-owner | 0 | 8 | 665 | 0 | packages/parent-domain |
| package | policy-domain | consumer-on-new-ts-owner | 0 | 8 | 0 | 0 | packages/policy-domain |
| package | portal-domain | consumer-on-new-ts-owner | 0 | 9 | 1 | 0 | packages/portal-domain |
| package | production-domain | consumer-on-new-ts-owner | 0 | 75 | 0 | 0 | packages/production-domain |
| package | remote-access-domain | consumer-on-new-ts-owner | 0 | 3 | 0 | 0 | packages/remote-access-domain |
| package | schema-domain | canonical-ts-owner-candidate | 0 | 4 | 0 | 0 | packages/schema-domain |
| package | screen-domain | consumer-on-new-ts-owner | 0 | 49 | 5 | 0 | packages/screen-domain |
| package | setup-domain | consumer-on-new-ts-owner | 0 | 11 | 0 | 0 | packages/setup-domain |
| package | text-domain | consumer-on-new-ts-owner | 0 | 2 | 0 | 0 | packages/text-domain |
| package | tracking-domain | consumer-on-new-ts-owner | 0 | 86 | 0 | 0 | packages/tracking-domain |
| app | local-api | consumer-on-new-ts-owner | 0 | 2 | 0 | 0 | apps/local-api |
| app | parent-desktop | audit-needed | 0 | 0 | 0 | 0 | apps/parent-desktop |
| app | portal | consumer-on-new-ts-owner | 0 | 1 | 0 | 0 | apps/portal |
| tool | no-reexports | architecture-debt-present | 0 | 0 | 0 | 1 | tools/no-reexports |
| tool | ocentra-ledger | audit-needed | 0 | 0 | 0 | 0 | tools/ocentra-ledger |
| infra | cloudflare | audit-needed | 0 | 0 | 0 | 0 | infra/cloudflare |

## Expected web ChatGPT output

After editing, web ChatGPT should return:

1. exact files changed
2. exact files moved
3. exact files deleted
4. which rows from the table changed status
5. any places where it refused to move schema because canonical ownership was still ambiguous
6. any places where it found new TS or Rust re-export debt not captured by this baseline

## Local Codex follow-up after web pass

After the web pass comes back, local Codex should:

1. review diff against this handover file
2. apply / pull changes into this branch
3. run packetized architecture lint
4. run direct self-contained tests first
5. separately handle fixture-driven timer-service proof tests

## Bottom line

Current branch state:

- exact legacy TS owner import path drain is complete in source
- broader single-owner TS/Rust schema architecture is not complete
- this file is the execution spec for finishing the remaining repo-wide consolidation work from a remote write-only model
