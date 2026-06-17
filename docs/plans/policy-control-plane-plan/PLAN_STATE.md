# Policy Control Plane Plan State

Status: audit-open. Core TypeScript and Rust policy-control contracts exist and focused validation runs are real, but implementation/proof closure is not complete and PR coordination remains centralized with the coordinator.

Current truth:

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
- The plan-local proof route was inconsistent between `docs/proof/policy-control-plane-plan/` and `output/policy-control-plane-plan-proof/`; the canonical `docs/proof/policy-control-plane-plan/` root now contains checked closeout bundles for WP01, WP07, WP08, plus the WP06 route bundle, while WP02/WP03/WP04/WP05 remain open.

Open gaps:

Real dependency blockers:
- `portal-ux-household-surfaces-plan` still owns unfinished rendered policy authoring, conflict, approval, and audit surfaces required by WP02.
- The parent-assistant and portal chat surfaces still need parent confirmation, child-agent validation, and portal chat/audit integration required by WP05.
- Device-trust, data-custody, and enforcement handoffs remain dependency-owned and are not proven complete here.

External platform constraints:
- Real iOS and macOS proof is not currently expected from this Windows host and should be tracked as an external-platform constraint when selected work requires it.

Avoidable local execution gaps:
- The plan docs previously marked workpacks and closure as complete without present proof artifacts.
- Workpack-specific closeout artifacts are still missing for WP02/WP03/WP04/WP05.
- The current checkout still has deleted stale `03-*.md` proof artifacts and no refreshed WP03 replacement bundle.
- The scoped architecture gate currently fails on existing `agent-protocol-domain` re-export debt.
- The `@ocentra-parent/portal` workspace `test` script is overbroad for policy-only validation and pulls unrelated LAN failures unless direct scoped `vitest` commands are used.

## Execution boundary

- Use `WORKPACK_INDEX.md` and `TEST_PROOF_EXPECTATIONS.md` to choose work.
- Do not mark this plan complete from checklist deltas, architecture docs, or focused contract passes alone.
- Update this file whenever proof, dependency, or validation truth changes.
