# Next Actions

## Scope and ownership

- Plan owner: `policy-control-plane-plan/AGENTS.md` with explicit handoff to policy/app/screen/network lanes.
- Ownership boundary: policy contract source-of-truth, cross-plan decision lattice, and domain-to-runtime handoff conditions.
- Scope boundary: avoid runtime implementation until policy compiler boundaries and conflict resolution are defined.

## Decision routes and failure conditions

- Decision path:
  - If cross-plan handoff contract is not mapped -> hold implementation lanes and keep research lane open.
  - If conflict resolution (manual-required/override/rollback) is unresolved -> block parent-visible rollout.
  - If runtime authority boundaries are unclear -> route to domain contract follow-up before code changes.
- Failure modes:
  - Policy conflict loops without deterministic resolution.
  - Missing compiler handoff contract for parent-visible decisions.
  - Undefined precedence/rollback behavior across plan intersections.

## Actioned completion tracker

- [ ] Define the policy source of truth and version model.
- [ ] Define nontechnical parent authoring and preview expectations.
- [ ] Define domain compiler handoff contracts.
- [ ] Define delivery/ack/conflict/audit lifecycle.
- [ ] Define ask-parent, bonus time, overrides, and assistant action gates.

## Proof and proof path

- Required proof links: policy source-of-truth rows, compiler handoff rows, and TEST_PROOF_EXPECTATIONS.md for policy boundaries.

## State

- Current state remains first-pass architecture + workpack gate; no completion claim until decision tree and execution handoffs are closed.
