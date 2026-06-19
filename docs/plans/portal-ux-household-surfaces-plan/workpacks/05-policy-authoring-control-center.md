# 05 Policy Authoring Control Center

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `05 Policy Authoring Control Center`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Policy preview and control states exist in pieces. Complete nontechnical policy
authoring remains incomplete.

Current live seam findings from the 2026-06-18 restart audit:

- `apps/portal/src/ParentPortalRoute.tsx` already mounts `PolicyPreviewRoutePanel` on `RuleManagement`, `Schedules`, `Approvals`, and `Enforcement`.
- `apps/portal/src/ParentPortalRoute.tsx` now resolves `shellStatus.parentAccessState` and passes that typed access state into `PolicyPreviewRoutePanel`, so the policy route can distinguish active-controller, observer-only, unauthenticated, and proof-missing parent roles without inventing portal-local policy authority.
- `packages/portal-domain/src/policy-preview-panel.ts` already owns typed preview-only, manual-required, conflict, source-lifecycle, assistant-confirmation, and no-enforcement-claim intent shaping.
- `packages/portal-domain/src/policy-preview-panel.ts` now also renders typed `Parent access` and `Write authority` summary details plus an `Approval authority` card, including explicit observer-only, unauthenticated, proof-missing, and parent-confirmation-required wording.
- `packages/portal-domain/src/policy-preview-workspace.ts` now shapes those preview intents into typed `Preview`, `Approval`, `Lifecycle`, and `Boundary` rows for the main policy workspace.
- `packages/portal-domain/src/parent-portal-nav.ts` now gives `RuleManagement`, `Schedules`, `Approvals`, and `Enforcement` real manage-nav entries, so those hash routes resolve into the parent-manage surface instead of falling back into the guide lane.
- `packages/portal-domain/src/policy-preview-panel.ts` now keeps delivery and acknowledgement claims separate from active enforcement by rendering lifecycle text such as `Delivered is reported, but active enforcement is separate.`
- `packages/agent-protocol-domain/src/policy-preview-read-model.ts`, `crates/agent-protocol/src/policy_preview.rs`, and `crates/agent-service/src/policy_preview_payload.rs` now carry typed nullable approval/audit preview metadata (`policyApprovalId`, `policyOverrideId`, replay lineage, reviewer identity, reviewed-at, and audit reference) plus `replay-rejected` request status through the preview read-model seam.
- `packages/portal-domain/src/policy-preview-panel.ts` now surfaces those approval/audit preview details in the `Approval authority` card when present, and it explicitly reports replay-rejected approval attempts without inventing a new override.
- `crates/agent-service/src/websocket/policy_request_confirm.rs` now persists supported assistant-preview confirmations (`app`, `site`, `category`, and `device`) into the activity store through a real `ActivityStore` write path, and `crates/agent-core/src/activity_store_policy_preview.rs` plus `crates/agent-core/src/activity_store_policy_preview_targets.rs` now project those stored lifecycle fields and explicit target kinds back through the preview read model.
- That producer/writer bridge stays intentionally bounded: `child-profile` and `resource` confirmation targets still return confirmed service validation while keeping the activity-store mutation, upstream-writer, and read-model-projection claim states unclaimed, because the current preview target contract does not yet support those target kinds honestly.
- `apps/portal/tests/policy-preview-live-activity-state.test.ts`, `apps/portal/tests/policy-preview-route-panel.test.ts`, and `packages/portal-domain/tests/unit/policy-preview-panel.test.ts` already cover parser, observer/controller visibility, and the delivery-versus-enforcement boundary behavior.
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx` now renders a typed preview banner inside the main policy workspace from `activityState.policyPreviewReadModel`.
- Focused validation for the currently landed WP05 slices is already green: touched-file TS architecture lint over `packages/agent-protocol-domain/src/defaults.ts`, `packages/agent-protocol-domain/src/policy-preview-read-model.ts`, `packages/agent-protocol-domain/src/policy-request-confirm-command.ts`, and `packages/agent-protocol-domain/tests/unit/policy-preview-contracts.test.ts`; `npx tsc -p packages/agent-protocol-domain/tsconfig.json --noEmit`; `npm run test --workspace @ocentra-parent/agent-protocol-domain -- tests/unit/policy-preview-contracts.test.ts tests/unit/policy-request-confirm-command.test.ts`; Rust architecture lint over `crates/agent-protocol/src/policy.rs`, `crates/agent-protocol/src/policy_preview.rs`, `crates/agent-core/src/activity_store_policy_preview.rs`, `crates/agent-core/src/activity_store_policy_preview_targets.rs`, `crates/agent-core/src/activity_store_policy_preview_tests.rs`, and `crates/agent-service/src/websocket/policy_request_confirm.rs`; `cargo test -p ocentra-parent-agent-protocol policy_request_assistant_preview_confirm -- --nocapture`; `cargo test -p ocentra-parent-agent-core policy_preview_read_model -- --nocapture`; and `cargo test -p ocentra-parent-agent-service policy_request_assistant_preview_confirm -- --nocapture`.
- A typed parent-confirmation/write command seam now exists across `agent-protocol-domain`, `agent-protocol`, and `agent-service`, but the current portal route still does not expose a parent-triggered confirm action and the portal-visible preview seam still does not carry the full typed request envelope required to construct that command honestly. `packages/agent-protocol-domain/src/policy-request-confirm-command.ts` requires request identifiers, timestamps, source-document fields, audit refs, and related request metadata that the current `packages/agent-protocol-domain/src/policy-preview-read-model.ts` parser and `crates/agent-service/src/policy_preview_payload.rs` projection do not yet surface.
- The remaining open WP05 gap is no longer just UI visibility or missing producer truth. It is now sequenced as: project the typed request envelope into the portal-visible preview seam first or explicitly prove another typed source, then expose the typed parent-confirmation action from the portal surface, extend or explicitly defer unsupported `child-profile` / `resource` preview targets, and prove the observer/co-parent/controller authZ matrix plus rollback execution beyond the currently landed controller-versus-observer visibility slice.

## Where We Want To Be

Parents can scan, create, preview, and understand rules by child, target,
schedule, action, proof level, and last result.

## Decision Tree

| If the assignment touches...        | Read next                                       | Required proof                              |
| ----------------------------------- | ----------------------------------------------- | ------------------------------------------- |
| Policy source of truth or compiler  | `../../policy-control-plane-plan/AGENTS.md`     | typed policy/version/conflict proof         |
| Enforcement-ready state             | `../../v0-8-enforcement-control-plan/AGENTS.md` | adapter authority and rollback proof        |
| App/browser/network/tracking target | owning domain plan AGENTS                       | target compiler/read-model proof            |
| Parent UI authoring                 | this workpack and exact route/source            | create/preview/confirm UI proof             |
| Assistant-proposed policy           | WP11 assistant action preview flow              | typed preview and parent confirmation proof |

## Required Policy States

- Draft intent: not saved, not delivered, no enforcement claim.
- Preview: typed dry-run result with target, schedule, child/device scope, conflict, and proof tier.
- Pending approval: parent confirmation required or co-parent approval required.
- Delivered: child/service acknowledged receipt, not necessarily enforced.
- Active: policy is within schedule and domain adapter has authority.
- Blocked/manual-required: platform permission, adapter authority, account role, conflict, or stale route prevents action.
- Rollback/recovery: previous state and audit ref are visible.

## Requirement Checklist

- [ ] Use typed intents for rule changes.
- [ ] Show dry-run/observe/enforcement-eligible states.
- [ ] Show conflict and unavailable reasons.
- [ ] Keep policy evaluation out of the portal.
- [ ] Test create/preview UI paths where backed by service state.
- [ ] Require parent confirmation before writes.
- [ ] Show delivery/ack/audit status separately from active enforcement.
- [ ] Prove authZ matrix for observer, co-parent, and controller roles.

Current slice-scoped truth on this branch/worktree:

- The preview/manage route packet is real and validated.
- The authZ/lifecycle visibility packet is real and validated.
- The typed approval/audit preview metadata packet is real and validated, and the current store-backed producer now persists supported assistant-preview confirmations (`app`, `site`, `category`, `device`) with typed `policySourceStatus`, `policyRequestStatus`, reviewer, and audit fields through the activity-store preview projection.
- Unsupported `child-profile` and `resource` confirmation targets are still intentionally unclaimed at the store/writer/projection boundary.
- Parent-facing portal action wiring, rollback execution, and downstream delivery/enforcement truth are still open.
- Co-parent-specific authZ proof is still open.
- The next exact executable slice on current source is the preview-envelope bridge that carries the typed request fields into the portal-visible preview seam, because the service/store producer bridge is real for the supported preview target kinds but the current read model still omits the request envelope the portal would need to emit a real confirmation command honestly.

## Acceptance And Proof

UI actions produce typed request/preview state and render service response.

Expected proof names:

- `portal.policy.draft-preview-confirm`
- `portal.policy.conflict-unavailable`
- `portal.policy.delivery-ack-audit`
- `portal.policy.authz-role-matrix`
- `portal.policy.no-evaluation-in-portal-negative`
- `portal.policy.rollback-visibility`

Proof must include screenshots/DOM snapshots, typed intent/preview fixture or live response, denied-role case, and audit/proof refs.

Current 2026-06-18 checkpoint-safe proof on this branch/worktree is narrower than full WP05 completion:

- It proves preview/read-model rendering and manage-route entry.
- It proves controller-versus-observer visibility wording in the route panel and preview intent.
- It proves `delivered` and `acknowledged` lifecycle text stay separate from active enforcement claims.
- It proves the preview seam can now carry typed approval/audit/replay metadata without widening into enforcement claims.
- It proves a live activity-store producer and preview projection for supported assistant-preview confirmation targets (`app`, `site`, `category`, `device`) while leaving unsupported `child-profile` / `resource` rows unclaimed instead of synthetic.
- It does not yet prove a portal-exposed parent confirmation action, co-parent approval flow, or rollback execution.

## Failure Conditions

- Do not let portal UI compile, evaluate, or enforce policy by itself.
- Do not equate delivered policy with active enforcement.
- Do not allow AI/assistant output to write policy without typed preview and parent confirmation.

## Parallel Ownership Notes

A owns enforcement action truth. C owns the authoring workflow and visual model.
