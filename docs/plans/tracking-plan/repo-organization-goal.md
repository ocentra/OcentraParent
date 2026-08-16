# Repo Organization Goal

> **Historical checkpoint:** this goal records the retired
> `codex/tracking-plan-full-continuation-a` organization pass. Current work is
> routed by `PLAN_STATE.md`, `CODE_AUDIT.md`, and the engineering graph; do not
> create or resume the branch named below from this document.

This file defines the organization pass for the tracking continuation branch.
It is a goal contract, not a completion claim.

## Task

Organize Ocentra Parent into canonical, DRY, domain-owned code, contract,
test, and documentation boundaries, starting with tracking and using the same
pattern for LAN, network, app/game, browser, AI, screen, reporting, policy,
enforcement, and future lanes.

## Current Branch Rule

- Branch: `codex/tracking-plan-full-continuation-a`.
- Use one branch for this pass.
- Do not create micro-branches.
- Do not open a PR until the user explicitly approves.
- Pull/rebase latest main only at clean checkpoints or when required to avoid
  conflict drift.
- Preserve existing tracking work during sync and conflict resolution.

## Scope

In scope:

- Tracking organization first.
- Repo-wide organization inventory where it affects tracking boundaries.
- Canonical contract placement.
- Move-only or low-risk wiring changes.
- Test relocation where crate/package public APIs support it.
- Documentation updates that make ownership, gaps, and validation clear.

Out of scope for this organization pass:

- New tracking product behavior unless required to preserve existing tests.
- New AI implementation.
- New provider mesh, AI prompt tuning, model quality tests, or temperature
  tests inside tracking.
- PR creation or merge.
- Large rewrites that are not needed for ownership, imports, exports, or
  validation.

## Architecture Target

Canonical truth lives once:

- TypeScript contracts live in `packages/*-domain`.
- TypeScript runtime validation uses Effect Schema.
- Branded values use Effect Schema brands and decode helpers.
- UI consumes domain contracts; UI does not define canonical events, route ids,
  command names, policy ids, status names, or protocol shapes.
- Rust wire/protocol shapes live in `crates/agent-protocol`.
- Rust domain/runtime logic lives in `crates/agent-core`.
- Rust transport/service orchestration lives in `crates/agent-service`.
- Shared parent/child contracts live in shared packages or crates, not in both
  apps.
- Core/common layers must not import feature-specific layers.
- Dependency cycles are forbidden.

## DRY Target

Remove or mark for removal any same-shape-different-name duplication across:

- schema payloads;
- event ids and event names;
- route ids and API paths;
- WebSocket command names;
- policy ids and status names;
- child/profile/device identifiers;
- read-model payloads;
- proof helper command wrappers;
- Rust protocol structs duplicated outside `agent-protocol`;
- UI-local copies of domain contracts;
- test-local copies of production contracts.

Prefer canonical exports and narrow adapters over parallel definitions.

## Tracking-Specific Boundary

Tracking owns:

- observe location, status, geofence, expected-place, nearby-place, and device
  evidence;
- classify tracking-side conditions;
- decide when AI analysis is needed;
- publish typed AI request/work events;
- accept validated AI result events from the AI lane contract;
- feed accepted AI results into policy as evidence only;
- prevent AI from directly creating live tracking, notification, escalation,
  enforcement, audit authority, or policy authority.

Tracking does not own:

- AI provider selection;
- AI provider mesh;
- AI work lease/claim internals;
- prompt tuning;
- model quality regression;
- hallucination quality evaluation;
- temperature sensitivity.

Tracking may test AI boundary safety. Tracking must not test AI model behavior.

## Test Organization Target

Existing tests should move where safe instead of being rewritten.

Each crate/domain should have an obvious test home when applicable:

- unit;
- integration;
- e2e;
- invariant;
- property-based;
- mutation;
- differential;
- contract;
- consumer-driven;
- security;
- authN/authZ;
- replay, ordering, race, idempotency;
- migration, rollback, schema drift;
- fuzzing;
- load, spike, soak;
- chaos, slow dependency, retry storm;
- clock skew and expiry boundary;
- observability, logging, metrics, tracing, alerting;
- canary and rollback validation;
- human misuse and double-submit cases.

Only create folders that match repo conventions and near-term test ownership.
Do not add fake coverage or placeholder green tests.

## Process

1. Inventory branch state, main drift, locks, rules, tracking docs, `docs/todos`,
   packages, crates, tests, and duplicate contract shapes.
2. Sync main at a clean checkpoint if needed.
3. Map tracking TS contracts, Rust protocol structs, Rust runtime modules,
   service boundaries, UI consumers, scripts, docs, and tests to canonical homes.
4. Apply minimal organization changes.
5. Run a DRY/canonicalization pass for obvious duplicate truth.
6. Run focused validation for touched packages/crates.
7. Run required local gates.
8. Commit and push one meaningful validated chunk.
9. Do not report PR-ready unless the user explicitly asks for PR readiness.

## Required Local Gates

Use this order before any PR-ready-style claim:

```powershell
npm run hub:guard
npm run lint:schema-boundaries
npm run codeql:local:changed -- --reuse-db
npm run validate
```

For tracking Rust or portal changes, add focused checks before `validate`, for
example:

```powershell
npm run lint --workspace @ocentra-parent/portal
cargo test -p ocentra-parent-agent-core tracking
cargo test -p ocentra-parent-agent-service tracking
```

Record exact pass/fail results. Fix failures before handoff.

## Failure Conditions

This pass fails if it:

- creates duplicate schema, event, route, command, policy, or protocol truth;
- moves code into a dependency cycle;
- leaves UI as the canonical owner of domain contracts;
- leaves Rust protocol shapes duplicated outside `agent-protocol`;
- claims AI implementation or AI quality coverage inside tracking;
- rewrites behavior unnecessarily;
- commits generated proof churn as product source truth;
- reports done without validation;
- opens a PR without explicit user approval.

## Expected Handoff

The handoff must include:

- branch and commit;
- pushed state;
- files/packages/crates touched;
- what moved versus what changed behavior;
- duplicate truth removed or explicitly deferred;
- focused validation results;
- full validation result;
- known gaps;
- next safe work chunk.
