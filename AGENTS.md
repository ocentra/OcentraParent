# Ocentra Parent Agent Guide

This repo follows Ocentra-style scaffold discipline. Keep changes narrow, contract-first, and validated.

## Ocentra AI Rule Map

Before coding, read `.ocentra-ai/rules/ocentra-parent-rules.mdc`. It routes work to granular rule files for tests, domain boundaries, protocol/WebSocket, Rust service, portal, logging/redaction, localhost security, source shape, and validation.

Before editing or committing, run `npm run lanes:status`, `npm run lanes:guard`, `npm run hub:status`, and `npm run hub:guard` from the checkout you are using. Each active worktree lane must be claimed in `C:\Users\sujan\.codex\ocentra-parent-worktrees.json` with the lane owner, thread label, branch, task, and next action. Cross-chat instructions, reports, and file locks live under `C:\Users\sujan\.codex\ocentra-parent-hub`. The pre-commit hook runs both guards automatically.

When starting in a worker lane, run `npm run hub:inbox` and acknowledge the latest hub instruction with `npm run hub:ack` before committing. Before starting or resuming assigned work, report `STARTED` back to the hub so the primary coordinator has a timestamped handoff. Before editing files, claim your intended ownership with `npm run hub:lock -- --paths "path/or/package,other/path" --reason "short scope"`. Report progress back to the hub with `npm run hub:report -- --summary "short status" --details "validation, blockers, touched files"`. When work is done, verify it, run the lint/tests requested in the hub mail, make a local commit only when instructed, and report `DONE` with exact validation, commit state, touched packages/files, known gaps/risks, and detailed scope of what changed. If asked to prepare or create a PR, include the same detailed scope in the PR body. Keep `hub:report` semantic: `STARTED`, meaningful progress, `BLOCKED`, and `DONE`. Use `npm run hub:heartbeat -- --state idle --note "waiting for instruction"` for per-minute liveness or idle notes instead of overwriting work state. Keep hub reports short unless the hub mail specifically asks for detail.

When a worker lane should receive follow-up work from the primary hub without another manual prompt, leave `npm run hub:watch -- --interval-ms 5000` running in that worker checkout. Use `--ack` only when the worker intentionally accepts displayed messages as read. Per-minute worker heartbeat automations are standing mailbox checks; do not delete, pause, or replace them just because there is no unread mail or active assignment. Those automations should write liveness with `npm run hub:heartbeat`; they should not use `npm run hub:report` for routine idle/waiting checks.

When the primary hub should notice worker reports without manual polling, leave `npm run hub:watch -- --reports --interval-ms 5000` running in the primary checkout. To inspect worker liveness separately from semantic reports, run `npm run hub:heartbeats` or read `C:\Users\sujan\.codex\ocentra-parent-hub\worker-heartbeats.ndjson`.

Repo-local Codex hooks live in `.codex/hooks.json` and route through `npm run --silent hub:hook` to `scripts/dev/codex-hub-hook.mjs`. They inject hub context on session start and user prompts, record the active Codex `session_id` for the current lane, remind dirty worker lanes to lock files after tool use, and continue worker turns at stop time when unread hub messages or unguarded dirty paths still need attention. If the primary or a worker chat gets too long, start a new chat in the same worktree; the hook will identify the lane, show already acknowledged hub messages and latest reports, and prevent repeating completed inbox setup. If the Codex Hooks settings page asks for trust review, review and enable the project hooks before relying on automatic hub context.

The primary coordinator should read this guide, `.ocentra-ai/rules/ocentra-parent-rules.mdc`, `docs/architecture/worktree-lanes.md`, `docs/architecture/primary-coordinator-reminder.md`, and `docs/product-roadmap.md` before assigning or integrating roadmap work. On every coordination pass, check hub status, lane status, primary/worktree Git status, open PRs/checks, and GitHub Actions state when relevant. Tell workers to pull or rebase latest `main` before starting assigned work. Review worker `DONE` reports by inspecting the branch diff and validation before asking for fixes or creating a PR. Create a PR only after local validation is acceptable and the branch is pushed; the PR body must include detailed scope covering what changed, touched packages/files, validation, known gaps/risks, and the roadmap slice completed. Merge only after PR CI is green and the reviewed diff is acceptable, then pull latest `main`, update roadmap/lane/hub state, and tell active workers to rebase or pull latest `main` before continuing. Post-merge hub reports must include the same detailed scope plus PR/merge state. Workers resolve conflicts on their own branches after fetching/rebasing latest `main`; primary resolves only conflicts it owns during integration and must keep the worker informed.

When writing or changing tests, also read `.ocentra-ai/rules/ocentra-parent-test-rules.mdc`. Test doubles are forbidden; tests must use real contracts, parsers, services, transports, or UI paths.

When changing multiple layers, use `.ocentra-ai/skills/ocentra-parent-rule-router/SKILL.md` as the lookup workflow instead of loading every rule file at once.

Local dev ports default to fixed values: Rust agent on `127.0.0.1:4477`, Vite portal on `127.0.0.1:4478`. Worker lanes that need visible demos can set `OCENTRA_PARENT_AGENT_PORT` and `OCENTRA_PARENT_PORTAL_PORT` before `npm run dev`, `npm run dev:agent`, `npm run dev:portal`, or `npm run dev:lan`; for example codex-b uses agent `4677` and portal `4678`. LAN dev uses the same selected ports with explicit `npm run dev:lan` binding and origin allowlists. Use managed scripts; they reclaim only stale Ocentra Parent processes and must not take over Ocentra Games editor ports.

## Non-Negotiable Boundaries

- Do not put shared API paths, route ids, event names, log shapes, policy ids, or device identifiers directly in app or crate code.
- Add shared TypeScript contracts under `packages/*-domain`.
- Add Rust-facing protocol shapes under `crates/agent-protocol` only after the TypeScript contract is explicit and test-backed.
- Use Effect Schema for TypeScript runtime validation.
- Do not add Zod.
- Do not create manual `string & { readonly __brand: ... }` aliases.
- Branded strings must come from Effect Schema brands and decode helpers.
- App/runtime source must not contain inline string literals. Text, ids, routes, fields, commands, and events live in domain packages.
- App/runtime TypeScript source must not annotate values as raw `string`; use a branded domain type or keep external input as `unknown` until parsed.
- Rust service/core source must not contain inline string literals. Runtime strings live in `crates/agent-protocol` constants.
- Do not create god files or god classes. Source shape validation warns on file-size advisory bands and near function/class/export/type limits, then fails past the hard limit.
- Do not use mocks, fakes, stubs, spies, MSW, Nock, Sinon, `vi.mock`, `vi.fn`, or equivalent test doubles. Tests must exercise real domain contracts, parsers, services, and local transports.
- Every source workspace and Rust crate needs tests from the beginning.
- Rust service code should stay async and use Tokio's multithreaded runtime unless a specific boundary requires otherwise.
- Do not add core recorder, blocking, AI, notification delivery, or product portal UI code during scaffold-only tasks.
- Dev portal screens are allowed only when they prove local protocol and runtime visibility.

## Package Responsibilities

| Package                                 | Owns                                                                          |
| --------------------------------------- | ----------------------------------------------------------------------------- |
| `@ocentra-parent/schema-domain`         | Shared Effect Schema wrappers and decode helpers.                             |
| `@ocentra-parent/endpoint-domain`       | API path, route id, header, query, and endpoint brands.                       |
| `@ocentra-parent/agent-protocol-domain` | WebSocket command/event contracts shared by portal and Rust.                  |
| `@ocentra-parent/text-domain`           | Schema-backed display text tokens.                                            |
| `@ocentra-parent/portal-domain`         | Portal routes, DOM constants, and dev command button contracts.               |
| `@ocentra-parent/parent-domain`         | Parent/family/device product contracts when implementation starts.            |
| `@ocentra-parent/activity-domain`       | Device activity event schemas and query contracts when implementation starts. |
| `@ocentra-parent/logging-domain`        | Operational app/service logging contracts shared by TypeScript and Rust.      |

## Validation

Run:

```powershell
npm run validate
```

The root gate runs release version alignment, schema-boundary checks, Turbo lint/type-check/test tasks, Rust format, Rust clippy, Rust workspace checks/tests, integration smoke, local portal smoke, and Playwright UI coverage against the real Rust service. CI also runs dependency policy, SBOM generation, and package install/launch smoke checks.

`main` is a CI and package-preview branch. It must not publish GitHub Releases. Production installer publishing belongs to the `production` branch workflow and only runs when the aligned version tag is missing. Package-preview jobs should stay honest about platform scope: build and smoke-check real Windows/Linux/macOS/mobile artifacts, but do not claim signing, stores, device-owner policy, or iOS Family Controls until those credentials and entitlements are actually wired.

ESLint includes local Ocentra Parent rules. Editors with ESLint enabled should report app string literals, raw app `string` annotations, manual brands, and naked domain string aliases before validation runs.

`scripts/check-source-shape.mjs` enforces source file/function/class/export budgets. File-size warnings begin at 250-line advisory bands; function/class/export/type warnings remain near the configured hard limit. Treat warnings as a request to split ownership before adding more behavior.

`scripts/check-no-test-doubles.mjs` rejects fake-green testing patterns in app, package, and crate source. Build real seams and test real boundaries instead of replacing behavior.

## Testing Standard

- Tests specify behavior.
- Weak tests are not useful.
- Prefer flat test files with one top-level module description.
- Add tests for every contract, parser, path helper, Rust protocol conversion, and local transport loop once those exist.
