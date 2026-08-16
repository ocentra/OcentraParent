<!-- agent-capsule -->

> Agent Capsule
> Doc: Rust-First Parent Architecture
> Kind: agent flow documentation; read when root routing, a current user prompt, or a worker packet references Rust-first parent architecture.
> Stop rule: Use this as the architecture authority, then return to the selected route/workpack. Do not broaden into LAN plan ownership from this document.
> Proves: the intended architecture direction and ownership rules.
> Does not prove: implementation completion, product readiness, PR readiness, or deletion safety.

<!-- /agent-capsule -->

# Rust-First Parent Architecture

This document records the current architecture direction. It supersedes stale
TS-first wording that may still exist in older plan docs and rules.

## Target Flow

Product path:

```text
TSX UI -> HostBridge -> Rust parent app facade -> Rust event bus/domain -> Rust read models -> HostBridge -> TSX UI
```

Dev web path:

```text
TSX UI -> DevWebHostBridge -> local dev transport
```

Web/Vite is a development target for Codex and HMR. Product targets are Tauri
desktop plus Android and iOS shells. Remove WebSocket from the product
`TSX UI <-> parent Rust` path only. Do not remove or weaken Rust-owned
parent/child LAN/WAN transport.

## Ownership

Rust owns:

- schema truth, contracts, DTOs, actions, route snapshots, read models,
  projections, runtime behavior, and proof truth;
- business logic for policy, activity, tracking, network, browser,
  enforcement, logging, AI, evidence, custody, family, setup, billing,
  production, and notification paths;
- product proof logs and runtime event authority;
- mobile bridge shapes and parent/child transport contracts.

TypeScript owns only presentation and generated edge code:

- TSX, CSS, assets, and visual layout;
- generated bridge DTO/type imports;
- thin HostBridge and DevWebHostBridge adapters;
- minimal local visual state;
- pure presentation text and display helpers when they do not encode product
  state.

Generated bridge DTOs and thin TS adapters are allowed.

Effect Schema may remain at untrusted TS edges and generated validation edges.
It must not be treated as business truth.

## TS Surface Disposition

This is the live package-by-package disposition table for the Rust-first
cutover. It is the sync source for what stays, what shrinks to generated/thin
adapter only, and what loses TS business ownership.

| Surface | End-state disposition | Allowed remaining TS role |
| --- | --- | --- |
| `apps/portal` | Stay | Pure UI/presentation only. |
| `apps/parent-desktop` | Stay | Host shell only; no business ownership. |
| `apps/local-api` | Generated/thin or retire | Dev-only transport/adapter surface; no product UI dependency. |
| `packages/activity-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/agent-protocol-domain` | Generated/thin only | Temporary generated/thin protocol adapter; never canonical. |
| `packages/ai-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/app-game-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/billing-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/browser-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/capability-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/child-runtime-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/data-custody-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/endpoint-domain` | Generated/thin only if still needed | Thin transport/edge adapter only; no product logic. |
| `packages/enforcement-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/event-domain` | Stay narrow only | UI interaction/subscription glue only; no business ownership. |
| `packages/evidence-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/family-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/lan-domain` | Remove as TS business owner | Transitional shim only; LAN runtime ownership stays in Rust and in the LAN thread. |
| `packages/logging-domain` | Stay narrow only | Dev/proof/UI-edge helper only; Rust owns product logs. |
| `packages/network-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/notification-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/parent-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/policy-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/portal-domain` | Stay narrow only | Pure presentation helpers only; no product contracts, logic, or snapshots. |
| `packages/production-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/remote-access-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/schema-domain` | Generated/thin only | Temporary generated validation or edge decoder only; never canonical. |
| `packages/screen-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/setup-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |
| `packages/test-results` | Stay | Test/dev artifact only. |
| `packages/text-domain` | Stay narrow only | Pure presentation text/helpers only. |
| `packages/tracking-domain` | Remove as business owner | Transitional shim only until Rust replacement is live. |

Interpretation:

- `Stay` means the surface remains in the repo but only inside the narrow role
  stated above.
- `Generated/thin only` means the surface may survive temporarily, but only as a
  generated DTO, validation edge, or thin adapter.
- `Remove as business owner` means TS must lose product logic ownership. The
  files/package may exist temporarily during migration, but only until the Rust
  replacement is live, bridge-exposed, UI-consumed, and validated.

No TS package is deleted merely because the target says `remove`. Deletion or
hard collapse is allowed only after Rust owns the replacement, generated
bridge/API output exists where needed, UI consumes it, focused tests are green,
imports are gone, and the main lane approves the deletion condition.

## Packet Types

- Schema packet: move one contract family into `crates/schema`, generate TS
  bridge DTOs, add Rust serialization/round-trip/drift tests.
- Runtime packet: wire one route/action through `crates/parent-runtime-core`
  into Rust snapshots/results.
- Domain packet: move one TS domain's business logic into the relevant Rust
  crate and leave TS as presentation only.
- UI packet: update one route/panel to consume Rust snapshots/actions only.
- Validation packet: write focused drift/guard tests for one completed slice.
- Delete packet: remove one TS business package/file only after the deletion
  condition is proven.

Subagents may own one bounded packet at a time. They must not make broad
architecture decisions independently.

## LAN Boundary

Do not take LAN plan workpacks from this architecture thread. LAN execution
stays in the LAN thread. Architecture packets may touch generic bridge/schema
surfaces only when the slice is not LAN runtime/debugging and the claim is
explicit.

## Rust Test Taxonomy

Each Rust crate must use visible, crate-level test categories that match its
actual risk. Possible categories include `unit`, `integration`, `contract`,
`security`, `property`, `fuzz`, `replay`, `concurrency`, `migration`,
`compatibility`, `observability`, `performance`, `chaos`, `ai`, and `e2e`.

Not every crate needs every folder. Empty placeholder folders and `.gitkeep`
test optics do not count as evidence. Tests must live in visible crate-level
folders/groups; inline or source-owned tests do not count as plan closure or
product proof.

## Test Organization Rule

Tests must be organized by the real risk surface of the crate/package. Use only
the groups that honestly apply to that surface.

Possible groups include:

- `unit`, `integration`, `contract`, `consumer`, `e2e`
- `invariant`, `property`, `fuzz`, `differential`, `mutation`
- `security`, `authn`, `authz`, `replay`, `privilege-escalation`
- `concurrency`, `race`, `idempotency`, `ordering`
- `migration`, `rollback`, `compatibility`, `schema-drift`, `version-skew`
- `chaos`, `slow-dependency`, `partial-outage`, `retry-storm`
- `performance`, `load`, `spike`, `soak`, `memory`, `fd`, `connection`
- `observability`, `logging`, `metrics`, `tracing`, `alerting`
- `ai`, `prompt-injection`, `hallucination`, `output-invariant`, `safety`

Rules:

- Tests live in real organized test folders, groups, and crates that match the
  actual risk surface.
- Do not create empty named test folders or `.gitkeep` optics to imply
  coverage.
- Do not rely on fake-green, mock-heavy proof, or happy-path-only coverage.
- Do not keep inline or source-owned tests.
- TypeScript tests must stay aligned with TS's reduced role: UI, generated
  edges, thin adapters, and presentation helpers. Product-path logic proof must
  move to Rust-owned tests as ownership moves.

## Status Report Shape

Every packet report must include:

- exact slice owned;
- files changed;
- Rust owner created or confirmed;
- generated bridge/API changes, if any;
- TS consumers removed or updated;
- focused tests run;
- scoped architecture gates run;
- remaining blocker, if any.
