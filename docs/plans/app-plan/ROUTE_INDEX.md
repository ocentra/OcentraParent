<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `ROUTE_INDEX.md`
> Kind: local route map.
> Read when: When deciding whether this plan owns the task or an adjacent plan owns the handoff.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Route changes require PLAN_INDEX and FEATURE_ROUTE_INDEX updates.

<!-- /agent-capsule -->

# Native Apps Plan Route Index

## Owns

- App-only narrowing and reconciliation for native app identity, installed inventory, process/runtime, foreground app evidence, app-only policy targets, app catalog/settings, and legacy app-plan rows.
- App-specific proof expectations where the selected workpack is not better owned by the shared `app-game-plan` evidence spine.
- App-only route hygiene and no-claim boundaries for app-specific product docs.

## Shared boundary with app-game-plan

- `app-game-plan` owns the shared native app/game evidence spine, combined app/game runtime/read-model chains, native game slices, most generated source-gated timer/read-model handoff rows, and broad app/game control proof gates.
- `app-plan` narrows app-only meaning, app-only reconciliation, and app-specific proof routing.
- If a selected app-plan workpack touches shared inventory/runtime/foreground/session/journal/service/portal/policy/timer/enforcement chains, first classify the owner path in `WORKPACK_FAMILIES.md` and record whether this plan owns the app-only slice or whether `app-game-plan` owns the shared implementation.

## Does Not Own

- Adjacent implementation completion in: app-game-plan, v0-8-enforcement-control-plan, portal-ux-household-surfaces-plan.
- Broad source rewrites without selected workpack proof.
- Shared native app/game spine completion without app-game-plan proof.
- Release or production claims outside this plan's evidence.

## Handoff Rule

Open an adjacent plan only after the selected workpack records the exact handoff reason, owner path, expected proof, and no-claim boundary.
