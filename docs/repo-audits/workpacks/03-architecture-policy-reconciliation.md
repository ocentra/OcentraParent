# WP03 Architecture Policy Reconciliation

## Objective

Resolve architecture-gate truth before accepting repo-wide or plan-level architecture-clean claims.

## Scope

Inspect:

- `AGENTS.md`
- `.ocentra-ai/rules/*`
- `scripts/check-architecture-policy.mjs`
- `scripts/check-no-reexports.mjs`
- `tools/no-reexports/**`
- representative crate/package roots

## Questions to answer

| Question | Required answer |
| --- | --- |
| Is no-reexport policy global today? | Yes, staged, or exception-based. |
| Does full repo pass? | Exact command and result, or explicit known blockers. |
| What is allowed during migration? | Changed-scope only, package-scope, or full cleanup. |
| What must a plan report say? | Exact architecture gate scope and remaining debt. |

## Known starting risks

- Rust `pub use` is banned by rule/tool but appears in many crate roots.
- TypeScript export-forwarding/barrels exist or recently existed in shared package roots.
- Scoped architecture passes can be misreported as repo-wide clean.

## Explicit answers

| Question | Answer |
| --- | --- |
| Is no-reexport policy global today? | Yes as a repo rule. `AGENTS.md` bans TypeScript/JavaScript re-exports and Rust public re-exports repo-wide, and `.ocentra-ai/rules/ocentra-parent-rules.mdc` routes architecture and source-boundary work through the same policy stack. |
| Does full repo pass? | Not proven in this slice. The repo-wide commands would be `npm run lint:architecture:all` and `cargo lint-architecture --all`, but they were not run under the V0/V1 budget. Known blockers are already visible in representative Rust crate roots (`agent-core`, `agent-protocol`, `app-core`, `app-game-core`, `tracking-core`, `storage-custody-core`, `family-identity-core`, `lan-core`, `agent-service`, `child-runtime`, `ocentra-network-evidence`, `remote-access-core`, `screen-core`, `screen-ai-core`, `screen-live-view-core`, `browser-core`, `child-ai-core`, `child-enforcement-core`, `child-notification-core`) and in WP07 legacy-shim rows for shared TypeScript frontage surfaces. |
| What is allowed during migration? | Scoped enforcement is allowed for validation, not for repo-wide closure claims. Workers may use changed-file, path, package, or diff scope (`npm run lint:architecture -- --files ...`, `npm run lint:architecture -- --base <base> --head <head>`, `cargo lint-architecture <path>`, `cargo lint-architecture --base <base> --head <head>`), but they must not add new barrels/re-exports, must state the exact scope they ran, and must name remaining debt. No explicit exception registry exists in the current repo-audit docs; the temporary migration stance is `"existing debt may remain visible while owners remove it, but new debt is forbidden."` Removal trigger: the owner threads finish the shared-surface/root cleanup and the coordinator can replace scoped wording with a real repo-wide clean run. |
| What must a plan report say? | The exact architecture-gate command, the exact scope, and the remaining debt. Acceptable wording is scoped only, for example "`npm run lint:architecture -- --files crates/agent-core/src/lib.rs` passed" or "`cargo lint-architecture crates/agent-protocol/src/lib.rs` passed". Unacceptable wording is any repo-wide clean claim unless `npm run lint:architecture:all` and `cargo lint-architecture --all` were both run and passed. |

## Output table

| Surface | Violation type | Scope | Owner | Fix strategy | Blocker? |
| --- | --- | --- | --- | --- | --- |
| `AGENTS.md` plus `.ocentra-ai/rules/ocentra-parent-rules.mdc` | Global policy exists but live repo state does not conform | repo-wide rule layer | repo maintainers plus lane manager enforcement | Keep the ban global, keep reports scoped until the debt is removed, and do not invent informal exceptions in plan threads | yes |
| `scripts/check-architecture-policy.mjs` | Architecture suite is intentionally scope-aware (`--files`, `--base/--head`, `--all`) | repo tooling entrypoint | tooling owners | Preserve scoped validation for touched work, but require every report to state that scoped passes do not imply repo-wide clean | yes |
| `scripts/check-no-reexports.mjs` plus `tools/no-reexports/src/main.rs` | Enforcement exists for JS/TS and Rust, but it is a validator, not proof that existing repo debt is gone | repo tooling entrypoint and Rust lint tool | tooling owners | Use the existing scope modes for migration; reserve `--all` for explicit debt sweeps and final repo-wide closure claims | yes |
| `crates/agent-core/src/lib.rs` | Rust public re-export debt (`pub use ...`) across a broad runtime frontage root | crate root | `agent-core` owner thread | Remove root re-exports or reduce the root to explicit modules/import paths after ownership cleanup; do not treat scoped passes here as repo-wide success | yes |
| `crates/agent-protocol/src/lib.rs` | Rust public re-export debt (`pub use ...`) across a broad transport/product surface root | crate root | `agent-protocol` owner thread | Remove crate-root re-exports after transport/schema ownership is narrowed; keep protocol/product split explicit in reports | yes |
| `crates/app-core/src/lib.rs`, `crates/app-game-core/src/lib.rs`, `crates/tracking-core/src/lib.rs`, `crates/storage-custody-core/src/lib.rs`, `crates/family-identity-core/src/lib.rs`, `crates/lan-core/src/lib.rs`, `crates/agent-service/src/lib.rs`, `crates/child-runtime/src/lib.rs`, `crates/ocentra-network-evidence/src/lib.rs`, `crates/remote-access-core/src/lib.rs`, `crates/screen-core/src/lib.rs`, `crates/screen-ai-core/src/lib.rs`, `crates/screen-live-view-core/src/lib.rs`, `crates/browser-core/src/lib.rs`, `crates/child-ai-core/src/lib.rs`, `crates/child-enforcement-core/src/lib.rs`, `crates/child-notification-core/src/lib.rs` | Additional Rust public re-export debt visible in representative roots | crate roots | per-crate owner threads | Treat as staged debt inventory, clean by owner thread, and keep every architecture claim tied to the exact root or package scope that was validated | yes |
| `docs/repo-audits/workpacks/07-orphaned-legacy-surface-inventory.md` rows for `packages/parent-domain/src/local-ai-runtime.ts`, `packages/portal-domain/src/contracts.ts`, `packages/agent-protocol-domain/src/primitives.ts`, and `packages/parent-domain/src/app-game-*.ts` / `tracking-*.ts` | TypeScript export-forwarding / barrel-like shim debt in shared frontage surfaces | shared package frontage; evidence is provisional in this slice because no `packages/*/src/index.ts` representative roots were present to inspect directly | `parent-domain`, `portal-domain`, and `agent-protocol-domain` owner threads | Treat the shared-frontage re-export rows as migration debt from WP07, forbid new forwarders, and convert/remove them under the later ownership-drift cleanup rather than hiding them behind scoped "clean" wording | yes |

## Acceptance

- Current policy state is explicit.
- Repo-wide versus scoped clean is impossible to confuse.
- Any temporary exception has owner, reason, and removal trigger.

## Failure conditions

- Claiming architecture clean from changed-file-only validation.
- Adding new barrels/re-exports as adapters.
- Hiding export debt under broad package roots.
