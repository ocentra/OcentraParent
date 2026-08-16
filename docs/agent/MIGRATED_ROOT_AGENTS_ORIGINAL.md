<!-- agent-capsule -->

> Agent Capsule
> Doc: Ocentra Parent Agent Guide
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Ocentra Parent Agent Guide (Historical Snapshot)

> Warning: this file is preserved migration context only. It does not define
> current ownership, contract, or test authority. Read
> `.ocentra-ai/rules/ocentra-parent-rules.mdc` and
> `docs/agent/RUST_FIRST_PARENT_ARCHITECTURE.md` for current direction.
>
> Current direction is not negotiable here: TypeScript is pure
> UI/presentation-only surface, while Rust owns contracts, logic, runtime
> truth, read models, and proof truth. Historical TS-first package ownership in
> this file is migration archaeology only and must not be revived.

This repo follows Ocentra-style scaffold discipline in the historical snapshot.
Use the Rust-first docs for current ownership and validation rules.

## Ocentra AI Rule Map

Before coding, read `.ocentra-ai/rules/ocentra-parent-rules.mdc`. It routes work to granular rule files for tests, domain boundaries, protocol/WebSocket, Rust service, portal, logging/redaction, localhost security, source shape, and validation.

## Product Feature Doc Protocol

Before starting product, feature, roadmap, policy, UI, AI, platform, enforcement, remote, or reporting work, use the focused product-doc path. Do not bulk-read every roadmap, checkpoint, and expectation file.

Minimum reading path:

1. Read `docs/feature-list.md` and identify the single feature doc that owns the work.
2. Read only that `docs/features/*.md` file, plus any second feature doc if the task clearly crosses a second feature boundary.
3. Read the expectation files linked by the feature doc that match the files you will touch.
4. Read the relevant milestone section in `docs/product-roadmap.md` only when the task changes milestone scope, status, order, or completion claims.
5. Read the relevant rows in `docs/product-capability-checklist.md` before and after the work when the task changes feature status, proof, or gaps.
6. Read the README for each touched app/package/crate/platform area before editing that area.

Context hygiene:

- Do not load all files under `docs/expectations`, `docs/features`, or `docs/checkpoints`.
- Do not open historical checkpoint files unless the feature doc, checklist, roadmap, or hub assignment names them as current proof.
- Do not use old checkpoint wording to override the current `docs/feature-list.md`, `docs/product-capability-checklist.md`, or `docs/product-roadmap.md`.
- If no feature doc owns the task, create or update the missing feature doc before making broad implementation claims.

Before reporting `DONE`, update the documentation layer that changed:

- update the owning `docs/features/*.md` checklist/current-state/gap when implementation changes status or proof;
- update `docs/product-capability-checklist.md` when a feature moves status, gains proof, or gains a new gap;
- update `docs/product-roadmap.md` when milestone order, scope, or completion changes;
- update expectation docs when the acceptance contract changes;
- update the touched module README when ownership, flow, or module gaps change;
- update `README.md` only when user-facing positioning changes;
- update `docs/competitor-capability-map.md` only when competitor parity, rejection, or tracking changes.

Every worker `DONE` or PR-ready report must say which feature doc and checklist row were updated, or explicitly say that no product-doc update was needed and why.

Before editing or committing, run `npm run lanes:status`, `npm run lanes:guard`, `npm run hub:status`, and `npm run hub:guard` from the checkout you are using. Live lane, inbox, ownership, heartbeat, and task state belongs in Ocentra Ledger, not in this product repo. The parent repo tracks only code, docs, scripts, and the pinned `tools/ocentra-ledger` submodule pointer. Set `LEDGER_ROOT` when a machine needs an explicit state location; otherwise Ledger uses its default external state root. The pre-commit hook runs the Ledger guard automatically.

No Codex lane work is complete unless the worker records the semantic state in Ledger, or explicitly reports why no Ledger update was needed. Use `npm run hub:status`, `npm run ledger:workers`, `npm run ledger:tasks`, and `npm run hub:lane-ledger:audit` on any PC that has Ocentra Parent worktrees before declaring a worktree safe to delete, parked, merged, or lost.

When using Codex from more than one PC, use a shared `LEDGER_ROOT` or configured Ledger sync transport before coordination or lane work. Only actual worktree folders and external Ledger state may live outside the repo. Do not let two PCs actively edit the same lane at the same time; sync Ledger, claim paths, and report state before continuing.

When starting in a worker lane, run `npm run hub:inbox` and acknowledge the latest hub instruction with `npm run hub:ack` before committing. Before starting or resuming assigned work, report `STARTED` back to the hub so the primary coordinator has a timestamped handoff. Before editing files, claim your intended ownership with `npm run hub:lock -- --paths "path/or/package,other/path" --reason "short scope"`. Report progress back to the hub with `npm run hub:report -- --summary "short status" --details "validation, blockers, touched files"`. When work is done, verify it, run the lint/tests requested in the hub mail, make a local commit on the worker branch, push that branch when ready for review, and report `DONE` with exact validation, commit state, touched packages/files, known gaps/risks, and detailed scope of what changed. If the user or primary asks the worker to prepare or create a PR, the worker may open the PR and include the same detailed scope in the PR body. Workers must not merge PRs or push directly to `main` unless the user explicitly asks for that exact action. Keep `hub:report` semantic: `STARTED`, meaningful progress, `BLOCKED`, and `DONE`. Use `npm run hub:heartbeat -- --state idle --note "waiting for instruction"` for per-minute liveness or idle notes instead of overwriting work state. Keep hub reports short unless the hub mail specifically asks for detail.

Multiple Codex chats may work inside the same worker lane. The Ledger hook records a thread/session wake identity from the hook `session_id`, but it does not make the lane exclusive. The write gate is exact-file claim ownership, not a lane-wide session lock. A thread may answer questions and inspect status while another thread is also active on the same lane, provided each thread claims only the files it is actively writing.

When a worker lane should receive follow-up work from the primary hub without another manual prompt, prefer a targeted one-shot Codex wakeup over an always-on minute automation. The sender writes Ledger mail, creates or resumes one recipient wakeup automation, and the recipient pauses that wakeup after it reads/acks the mail. Do not delete stable lane wakeup automations during normal ack flow; the saved automation is the reusable lane-to-thread target record and should stay as a readable markdown-style role prompt. Disposable temporary wakeups may delete themselves only when they were explicitly created as temporary and do not carry a lane's canonical thread target. Use `npm run hub:notify -- --lane <lane> --exit-code` as the cheap non-LLM prefilter for any automation bridge; it exits non-zero only when that lane has unread inbox mail, and for `primary` also when worker handoff reports start with `PR_READY`, `DONE`, or `BLOCKED`. `hub:notify` by itself is only a detector and does not wake a sleeping Codex thread. After it emits a wake request, the active sender Codex thread must use Codex automation tooling to create, resume, or update the intended recipient's one-shot thread wakeup. When updating an existing recipient automation, preserve that automation's original `targetThreadId`; never let the sender thread become the target. If the target thread id is unknown, stop and report that limitation in Ledger instead of guessing. If Codex automation tooling is unavailable in that sender thread, report that limitation in Ledger and ask the user/primary to wake the recipient manually. Idle liveness should be a Ledger heartbeat event, not a Codex chat report. If there is no unread mail and no active assignment, the worker should stay quiet in chat. See `docs/architecture/worktree-lanes.md` for the wakeup sequence.

For event-style lane coordination, use this handshake instead of broad polling. The sender writes the real instruction in Ledger mail, then creates a disposable temporary wakeup for the recipient thread or activates the recipient's stable paused wakeup without changing its target. The recipient reads the newest relevant mail, acknowledges it in Ledger, does only the requested work, and sends any required `ACK`, `STARTED`, `DONE`, `BLOCKED`, or `PR_READY` response back through Ledger. If the sender needs to be woken for that response, the recipient creates a disposable temporary wakeup for the sender thread, or activates the sender's stable paused wakeup without changing its target. After each side processes a disposable wakeup, that temporary automation should be deleted. Stable lane wakeups must be paused again, not deleted. Never use automation prompt text as the assignment source of truth; it is only a wake bridge. The Ledger message/report is the assignment and acknowledgement record.

When the primary hub should notice worker reports without manual polling, use targeted primary wakeups created by the worker after `DONE`, `BLOCKED`, or `PR_READY`. Keep a slow primary safety-net automation only as a fallback until targeted wakeups are proven on every active PC. To inspect worker liveness separately from semantic reports, run `npm run hub:heartbeats` or `npm run ledger:workers`.

Repo-local Codex hooks live in `.codex/hooks.json` and route through `npm run --silent hub:hook` to the Ledger compatibility wrapper. They inject Ledger context on session start and user prompts, record the active Codex `session_id` for the current lane, remind dirty worker lanes to lock files after tool use, and continue worker turns at stop time when unread Ledger messages or unguarded dirty paths still need attention. If the primary or a worker chat gets too long, start a new chat in the same worktree; the hook will identify the lane, show already acknowledged Ledger messages and latest reports, and prevent repeating completed inbox setup. If the Codex Hooks settings page asks for trust review, review and enable the project hooks before relying on automatic Ledger context.

The primary coordinator should read this guide, `.ocentra-ai/rules/ocentra-parent-rules.mdc`, `docs/architecture/worktree-lanes.md`, `docs/architecture/primary-coordinator-reminder.md`, and `docs/product-roadmap.md` before assigning or integrating roadmap work. On every coordination pass, check hub status, lane status, primary/worktree Git status, open PRs/checks, and GitHub Actions state when relevant. Tell workers to pull or rebase latest `main` before starting assigned work. Review worker `DONE` reports by inspecting the branch diff and validation before asking for fixes or creating a PR. Create a PR only after local validation is acceptable and the branch is pushed; the PR body must include detailed scope covering what changed, touched packages/files, validation, known gaps/risks, and the roadmap slice completed. Merge only after PR CI is green and the reviewed diff is acceptable, then pull latest `main`, update roadmap/lane/hub state, and tell active workers to rebase or pull latest `main` before continuing. Post-merge hub reports must include the same detailed scope plus PR/merge state. Workers resolve conflicts on their own branches after fetching/rebasing latest `main`; primary resolves only conflicts it owns during integration and must keep the worker informed.

When writing or changing tests, also read `.ocentra-ai/rules/ocentra-parent-test-rules.mdc`. Test doubles are forbidden; tests must use real contracts, parsers, services, transports, or UI paths.

When changing multiple layers, use `.ocentra-ai/skills/ocentra-parent-rule-router/SKILL.md` as the lookup workflow instead of loading every rule file at once.

Local dev ports default to fixed values: Rust agent on `127.0.0.1:4477`, Vite portal on `127.0.0.1:4478`. Worker lanes that need visible demos can set `OCENTRA_PARENT_AGENT_PORT` and `OCENTRA_PARENT_PORTAL_PORT` before `npm run dev`, `npm run dev:agent`, `npm run dev:portal`, or `npm run dev:lan`; for example codex-b uses agent `4677` and portal `4678`. LAN dev uses the same selected ports with explicit `npm run dev:lan` binding and origin allowlists. Use managed scripts; they reclaim only stale Ocentra Parent processes and must not take over Ocentra Games editor ports.

## Historical Boundaries Snapshot (Superseded)

The bullets in this section are preserved only as migration context from the
older TS-first period. They are not current authority. For active work, use
`.ocentra-ai/rules/ocentra-parent-rules.mdc`,
`docs/agent/TASK_ROUTER.md`, and
`docs/agent/RUST_FIRST_PARENT_ARCHITECTURE.md`.

If any bullet below suggests TS-owned contracts, TS-owned runtime truth, or
TS-first business logic, treat that as superseded history. The live rule is:
TS renders; Rust owns product truth.

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

## Historical Package Responsibilities Snapshot (Superseded)

This table is preserved only to explain the former TS-first package split. Do
not use it to assign current contract, runtime, or proof ownership. Do not use
it to justify reviving TS-first package authority.

| Package                                 | Historically owned (superseded)                                               |
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

The root gate runs release version alignment, local CodeQL checks for changed files, schema-boundary checks, Turbo lint/type-check/test tasks, Rust format, Rust clippy, Rust workspace checks/tests, integration smoke, local portal smoke, and Playwright UI coverage against the real Rust service. CI also runs dependency policy, SBOM generation, and package install/launch smoke checks.

Local CodeQL is part of `npm run validate` so new security/query findings are caught before they reach GitHub code scanning. Use `npm run codeql:local:changed` for the normal changed-file gate, `npm run codeql:local` to inspect all current JavaScript/TypeScript and workflow findings locally, and `npm run codeql:local:all` only when Rust CodeQL is specifically needed. The local runner uses all available cores by default through `CODEQL_THREADS=0`; set `CODEQL_RAM_MB` when a machine needs a memory cap.

The pre-commit hook is intentionally lighter than the root gate. It runs lane/hub guards plus fast local source validation, but it does not run package lint/type-check tasks, TypeScript/Rust unit suites, real-service smoke tests, portal Playwright E2E, production build, or package previews on every local commit. Use `npm run test:local`, `npm run precommit:full`, `npm run validate`, `npm run ci:local`, or focused scripts such as `npm run test:e2e` when those heavier checks are needed before PR-ready handoff or integration.

IMPORTANT: Pre-commit passing is not enough for `DONE`, `PR_READY`, PR refreshes, or any hub handoff that tells another lane or the primary coordinator to expect CI to pass. During normal development, fast commits may rely on pre-commit plus focused checks for the changed files. Before reporting `DONE`, `PR_READY`, refreshing an existing PR branch, asking for a PR, or claiming CI readiness, run the heavier gate for every touched workspace/package/crate:

- run lane/hub guards;
- run schema/source boundary checks;
- run focused tests for the touched behavior;
- run `lint`, `type-check`, and `build` for every touched TypeScript workspace;
- run `cargo check` and focused Rust tests for every touched Rust crate;
- run the relevant E2E/proof command when the change touches portal, protocol, runtime, or proof behavior.

Every `DONE` or PR-ready hub report must list the exact heavier commands that were run. If a heavier command is skipped, the report must say why and mark the remaining risk explicitly instead of implying CI readiness.

If CI fails in a platform-specific or service-backed path, do not keep pushing guesses. First extract the exact CI payload, log shape, trace, or service diagnostic that differs from local behavior, then add or update a local regression using that exact shape. Only push another fix after the local regression proves the boundary that failed in CI. If local reproduction is genuinely impossible, add durable diagnostics that prove where the data is present and where it disappears before changing product code.

When a failure exposes duplicate naming, routing, ids, fields, events, or UI surface truth, fix the contract boundary instead of patching one caller. Examples include route splits such as `activity` versus `network-activity`, or paired feature ids such as eventing runtime versus eventing UI. The owning `packages/*-domain` contract must define the canonical constants, branded values, schemas, route predicates, or field mappings; app/runtime code should consume those exports; and a guard or test should prevent reintroducing local ad hoc literals or comparisons.

`main` is a CI and package-preview branch. It must not publish GitHub Releases. Production installer publishing belongs to the `production` branch workflow and only runs when the aligned version tag is missing. Package-preview jobs should stay honest about platform scope: build and smoke-check real Windows/Linux/macOS/mobile artifacts, but do not claim signing, stores, device-owner policy, or iOS Family Controls until those credentials and entitlements are actually wired.

ESLint includes local Ocentra Parent rules. Editors with ESLint enabled should report app string literals, raw app `string` annotations, manual brands, and naked domain string aliases before validation runs.

`scripts/check-source-shape.mjs` enforces source file/function/class/export budgets. File-size warnings begin at 250-line advisory bands; function/class/export/type warnings remain near the configured hard limit. Treat warnings as a request to split ownership before adding more behavior.

`scripts/check-no-test-doubles.mjs` rejects fake-green testing patterns in app, package, and crate source. Build real seams and test real boundaries instead of replacing behavior.

## Testing Standard

Current test authority is stricter than this historical snapshot: tests belong
in organized test folders, and inline, placeholder, fake, mocked, or otherwise
non-provable coverage does not count as completion evidence.

New tests belong in visible test folders/groups/crates. Historical inline or
source-owned tests are migration debt only and must not be used as fresh
closure evidence.

- Tests specify behavior.
- Weak tests are not useful.
- Prefer flat test files with one top-level module description.
- Add tests for every contract, parser, path helper, Rust protocol conversion, and local transport loop once those exist.
