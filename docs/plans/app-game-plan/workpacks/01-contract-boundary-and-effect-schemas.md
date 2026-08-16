# 01 Contract Boundary And Effect Schemas

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `01 Contract Boundary And Effect Schemas`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: canonical app/game contract/schema boundary only after focused tests and proof exist.
> Does not prove: runtime source readiness, service readiness, policy readiness, adapter readiness, platform parity, PR readiness, or broad DONE.
> Proof rule: Before DONE, apply `workpacks/00-owner-boundary-proof-gate.md`, select tests in TEST_PROOF_EXPECTATIONS.md, and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Shared app/game identity, inventory, runtime, foreground, launcher, session, category, risk, approval, policy, authority, capability, action, AI digest, enforcement-result contracts, and route/action/read-model DTOs exist in the Rust-owned canonical schema layer before service, portal, policy, enforcement, notification, or adapter code consumes them.

## Scope

Current owner path:

```text
the owning Rust crate:
  canonical shared app/game contracts when shapes cross package, crate, app, or plan boundaries.
packages/schema-domain:
  generated-validation or edge-decoder surface only.
crates/agent-protocol / crates/agent-core / crates/app-game-core / crates/agent-service:
  canonical Rust contract/runtime/wire/service owners when selected.
```

Historical references to `packages/activity-domain`, `packages/parent-domain`,
`packages/agent-protocol-domain`, `packages/text-domain`, and
`packages/app-game-domain` are stale: those tracked owners are absent. Existing
workpack prose may be read for migration context, but current ownership is the
Rust-first map in `CODE_AUDIT.md` and the executable graph.

Required scope:

- Keep native app and native game product fields separate inside shared contracts.
- Encode stale, degraded, permission-limited, manual-required, unavailable, and not-claimed states.
- Keep source facts, classifier output, policy decisions, action readiness, and adapter result as separate contracts.
- Keep AI digest evidence-only; it cannot contain direct action authority.
- Keep policy/enforcement/notification/portal runtime behavior outside this contract workpack.

## Tests And Proof

Focused proof should include:

```bash
cargo test -p ocentra-schema
cargo lint-architecture crates/schema
npm run build --workspace @ocentra-parent/schema-domain
npm run type-check --workspace @ocentra-parent/schema-domain
cargo test -p ocentra-app-game-core app_game
cargo test -p ocentra-parent-agent-protocol app_game
```

If Rust/wire/service consumers are touched, add the focused commands from `TEST_PROOF_EXPECTATIONS.md`.

Negative proof required:

- Effect Schema accepts valid app and game rows.
- Display-name-only identity stays weak.
- Inventory cannot set running or foreground.
- Runtime cannot become foreground without foreground source proof.
- Launcher evidence cannot become known game without child-game proof.
- AI output cannot contain block/terminate/hide/suspend/shield authority.
- Manual-required/unavailable states cannot mark actions executed.
- Helper/projection package cannot re-own canonical shapes that belong in `crates/schema` or the owning Rust crate.

## Required Proof Root

```text
output/app-game-plan-proof/01-contract-boundary-and-effect-schemas/
```

Required files:

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## Done Signal

Rust-owned contracts exist for the selected shared app/game contract slice, TypeScript edge validation stays temporary, and Rust/service/portal/policy/enforcement changes are either absent or consume those contracts through explicit handoff boundaries.

Do not claim runtime source readiness, service readiness, policy readiness, adapter execution, platform parity, portal readiness, or PR_READY from this workpack alone.

Use the standard checklist in [workpacks README](README.md) plus the owner/import/proof gate in `workpacks/00-owner-boundary-proof-gate.md`.
