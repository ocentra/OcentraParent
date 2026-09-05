# Policy Control Plane Plan State

Status: audit-open. Core TypeScript and Rust policy-control contracts exist and focused validation runs are real, but implementation/proof closure is not complete and PR coordination remains centralized with the coordinator.

## Current ownership interpretation

```text
crates/schema:
  Canonical shared policy shapes when contracts cross package, app, crate, or plan boundaries.

policy-domain:
  Legacy package anchor or TypeScript proof-consumer surface unless explicit public exports exist. Package identity is not source-truth proof by itself.

policy-control-core:
  Rust source, authority, compiler, conflict, delivery, event, preview, request, and source helper crate.

agent-protocol and agent-protocol-domain:
  Delivery/read-model/audit/assistant seams only when selected.

portal-domain/apps/portal:
  Rendered authoring, preview, conflict, approval, and audit UI surfaces only when selected. UI state is not policy truth.

eventing-plan:
  Reusable local event bus, idempotency, replay, journal, and request/response semantics only.

account-identity-family-plan:
  Actor, role, session, parent authority, and household authority owner.

device-trust-bootstrap-plan:
  Parent presence, step-up, and trusted-device gating owner.

data-custody-storage-plan:
  Policy export/delete/sync/retention custody owner.

v0-8-enforcement-control-plan:
  Enforcement action authority, execution, rollback, and adapter behavior owner.

domain plans:
  App/game, browser, network, tracking, screen, AI, and notification runtime effects after typed compiler handoff.
```

## WP07 code-and-test checkpoint — 2026-08-28

- Canonical `e565bd9dd` integrates the reviewed 18-file Rust packet for schedule,
  time-budget, conflict, policy-source, child-request, parent-approval,
  temporary-override, and replay validation, together with focused source in all
  five mapped test roots.
- The source now rejects impossible UTC calendar values, invalid temporal
  ordering, zero budgets/carryover, bonus-time/action mismatches, early expiry,
  and replay drift. Unsupported timezone ownership remains blocking/manual-
  required instead of being guessed.
- No test was executed in the code-first phase and no proof was regenerated.
  The prior routed proof bundle is historical evidence, not validation of this
  packet.
- WP07 remains runtime-open: there is no shipped trusted clock or timezone/DST
  owner, durable timer journal, restart/offline recovery composition, or
  production caller for the crate-private override-expiry transition. No
  contract-only result is promoted to active policy or enforcement authority.

## WP02 integration snapshot — 2026-08-05

- PR [#615](https://github.com/ocentra/OcentraParent/pull/615) is merged (`17739c4f10889f007691bf4b320e55f1e9d9f9f4a6`) as a reviewed partial WP02 slice. It projects the Rust-owned parent-attention states for conflict, manual-required, and unsupported previews; its branch added focused parent-runtime and portal tests plus `docs/proof/policy-control-plane-plan/02-conflict-visible-proof.md`. The workpack remains open because the authoring/confirmation surface, trusted write boundary, remaining proof, and acceptance rows are not complete.
- This evidence is visibility-only. It does not close WP02 or claim template/manual-rule authoring, preview-to-save confirmation, opaque confirmed-request relay, delivery, enforcement, child-device application, or the complete WP02 proof set.
- Keep WP02 open until those owner paths are merged and their targeted proof/validation reconciles with the workpack.

## Current truth

- This plan owns the cross-domain policy control contract: source of truth, lifecycle, schedule/time budget, conflict precedence, domain compiler boundaries, delivery/ack/audit, ask-parent overrides, and policy event model.
- Existing domain plans own runtime effects; this plan owns the parent policy control plane contract and proof route.
- Parent-facing UI is specified here and in the portal plan, but no plan should treat UI state as policy truth.
- Real Rust contract implementation exists in `crates/policy-control-core`, with policy-preview/delivery/audit/assistant seams in the runtime/protocol crates and a focused portal projection. `packages/policy-domain` is absent in this checkout and must not be cited as implementation evidence.
- Historical proof logs name focused validation commands, but the 2026-08-16
  production audit did not rerun them. Commands targeting the absent
  `@ocentra-parent/policy-domain`/`packages/policy-domain` and the empty
  `packages/agent-protocol-domain` package are stale and cannot be treated as
  current validation evidence. Focused Rust/portal tests will be selected and
  rerun only after the production code phase.
- Feature-owned parent authoring and assistant approval surfaces remain incomplete and cannot be claimed done from contract tests alone.
- WP02 production-code pass is drafted but unvalidated: Rust validates/stages the portal draft, projects exact household/child/profile/policy/source/actor context from the trusted preview row, builds and dispatches the typed confirmed-request command from the bounded handle, consumes only after accepted relay, and restores failed pending drafts; absent context fails closed for manual review, and tests/runtime validation remain deferred.
- WP03 is contract-drafted but production-open: deterministic Rust compilers and validators exist, but non-test callers do not register an identity-backed durable parent policy source, load an active version, invoke the Screen/AI compilers, or persist/deliver their artifacts. The production confirmation route records an audit event; it does not establish active policy source truth.
- WP07 has reviewed contract and mapped test source at canonical `e565bd9dd`, but no test has run and its trusted clock/timezone, durable timer/recovery, and production caller boundaries remain open.
- WP01 authority routing is now explicit: production source registration/query requires Cloudflare WP06's durable identity/household persistence (which consumes Account Identity WP08), Device Trust WP01's trusted-device context, and Device Trust WP03's parent step-up context. These are hard prerequisites, not completion claims; no caller-supplied authority or fixture/manual/debug custody path satisfies them.
- Policy WP03 is ordered after WP01 because its compiler library has no shipped authoritative-source caller; it cannot be promoted from deterministic contract code until WP01 supplies the durable active-source registration/query boundary.
- The canonical proof root for this plan is `docs/proof/policy-control-plane-plan/`, and the touched route docs in this slice now agree on that single root.
- `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md` records current file presence and route status. WP01/WP03 have contract proof only and remain production-open; WP04 has contract proof but remains runtime-blocked on owner-backed adapter identity/trace; WP02/WP05 also remain open.

## Current coupling risks

```text
- UI preview is not applied policy.
- Compiler output is not policy source truth.
- Compiler tests are not runtime domain effects.
- Event model proof is not delivery/ack proof.
- Assistant draft is not parent approval.
- Child request is not parent approval.
- Policy delivery proof is not enforcement authority.
- Single-domain ack is not global active policy.
- policy-domain package identity is not canonical policy source truth.
- Focused contract passes are not full plan completion while WP02/WP05 remain open.
- WP01's owner-backed authority prerequisites are not optional documentation: Account Identity WP08 is transitive through Cloudflare WP06, while Device Trust WP01/WP03 provide the trusted-device and step-up context for policy-changing source operations.
```

## Current proof interpretation

```text
docs/proof/policy-control-plane-plan/ is the canonical proof root.
PLAN_PROOF_MANIFEST.md records file presence and workpack proof status only.
WP01 and WP03 have contract bundles recorded, not production closeout; WP06, WP07, and WP08 retain their routed bundles.
WP04 has current contract, negative, receipt-validation, compatibility, audit, and parent-visible proof, but no trusted adapter authority, inspectable execution trace, or real enforcement side-effect proof; its enforcement outcome/rollback token is not a policy authority bridge, so it is dependency-blocked rather than complete.
WP02 and WP05 remain open until targeted authoring/preview and ask-parent/override proof bundles exist or explicit dependency blockers are carried.
Universal guardrail files supplement workpack closeouts; they do not replace them.
```

Open gaps:

Real dependency blockers:
- `portal-ux-household-surfaces-plan` still owns unfinished rendered policy authoring, conflict, approval, and audit surfaces required by WP02.
- WP05's rendered portal callback remains unconsumed. A Rust-owned typed decision staging/relay slice is drafted in `crates/parent-runtime-core/src/parent_ui_bridge/action_dispatch.rs`, `policy_preview.rs`, and `policy_preview/resolution.rs`; it projects `Modify` from trusted preview context, never accepts arbitrary caller-selected changes, and fails closed until account/identity supplies exact actor context. Notification-provider dispatch remains uncomposed.
- The WP05 resolution service now reports notification handoff as explicitly `Unclaimed` because no notification-provider dispatch is composed; the resolved policy request remains a typed policy result only and does not claim notification delivery.
- Device-trust, data-custody, and enforcement handoffs remain dependency-owned and are not proven complete here.
- WP04 requires a trusted domain- or enforcement-owned adapter that performs the real side effect, emits the required inspectable execution trace, and supplies non-forgeable execution authority; the current public policy surface intentionally exposes no production execution entry and cannot advance acknowledged, applied, or rolled-back state.

External platform constraints:
- Real iOS and macOS proof is not currently expected from this Windows host and should be tracked as an external-platform constraint when selected work requires it.

Avoidable local execution gaps:
- Workpack-specific closeout artifacts are still missing for WP02/WP05.
- The scoped architecture gate currently fails on existing `agent-protocol-domain` re-export debt.
- The `@ocentra-parent/portal` workspace `test` script is overbroad for policy-only validation and pulls unrelated LAN failures unless direct scoped `vitest` commands are used.

## Execution boundary

- Use `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md` when needed, and `TEST_PROOF_EXPECTATIONS.md` to choose work.
- Do not mark this plan complete from checklist deltas, architecture docs, proof manifest presence, or focused contract passes alone.
- Update this file whenever proof, dependency, or validation truth changes.
