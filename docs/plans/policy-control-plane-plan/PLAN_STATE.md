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

## Current truth

- This plan owns the cross-domain policy control contract: source of truth, lifecycle, schedule/time budget, conflict precedence, domain compiler boundaries, delivery/ack/audit, ask-parent overrides, and policy event model.
- Existing domain plans own runtime effects; this plan owns the parent policy control plane contract and proof route.
- Parent-facing UI is specified here and in the portal plan, but no plan should treat UI state as policy truth.
- Verified implementation exists in `packages/policy-domain`, `crates/policy-control-core`, the policy-preview/delivery/audit/assistant seams in `packages/agent-protocol-domain`, and the focused portal policy-preview surface.
- Verified focused validation in this checkout includes:
  - `npm run test --workspace @ocentra-parent/policy-domain`
  - `cargo test -p ocentra-policy-control-core`
  - `cargo test -p ocentra-parent-agent-protocol policy`
  - `npm run test --workspace @ocentra-parent/agent-protocol-domain -- tests/unit/policy-preview-contracts.test.ts tests/unit/policy-control-delivery-read-model.test.ts tests/unit/policy-control-audit-redaction.test.ts tests/unit/parent-assistant-adapter.test.ts`
  - `cd apps/portal && npx vitest run tests/policy-preview-route-panel.test.ts tests/policy-preview-live-activity-state.test.ts`
- Verified owner-slice architecture validation in this checkout now also includes:
  - `npm run lint:architecture -- --files packages/policy-domain`
  - `cargo lint-architecture crates/policy-control-core`
- The shared architecture gate for the selected validation slice is not green because `packages/agent-protocol-domain` still contains banned re-exports.
- Feature-owned parent authoring and assistant approval surfaces remain incomplete and cannot be claimed done from contract tests alone.
- The canonical proof root for this plan is `docs/proof/policy-control-plane-plan/`, and the touched route docs in this slice now agree on that single root.
- `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md` records current file presence and route status: checked closeout bundles for WP01, WP03, WP06, WP07, and WP08; WP04 has a checked policy contract but remains runtime-blocked; open gaps remain for WP02/WP05.

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
```

## Current proof interpretation

```text
docs/proof/policy-control-plane-plan/ is the canonical proof root.
PLAN_PROOF_MANIFEST.md records file presence and workpack proof status only.
WP01, WP03, WP06, WP07, and WP08 have closeout bundles recorded in current route docs.
WP04 has current contract, negative, receipt-validation, compatibility, audit, and parent-visible proof, but no trusted adapter authority, inspectable execution trace, or real enforcement side-effect proof; it is dependency-blocked rather than complete.
WP02 and WP05 remain open until targeted authoring/preview and ask-parent/override proof bundles exist or explicit dependency blockers are carried.
Universal guardrail files supplement workpack closeouts; they do not replace them.
```

Open gaps:

Real dependency blockers:
- `portal-ux-household-surfaces-plan` still owns unfinished rendered policy authoring, conflict, approval, and audit surfaces required by WP02.
- The parent-assistant and portal chat surfaces still need parent confirmation, child-agent validation, and portal chat/audit integration required by WP05.
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
