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

- native app identity, installed inventory, process/runtime, foreground app evidence, app-only policy targets, app catalog/settings, and legacy app-plan reconciliation.

## Does Not Own

- Adjacent implementation completion in: app-game-plan, v0-8-enforcement-control-plan, portal-ux-household-surfaces-plan
- Broad source rewrites without selected workpack proof.
- Release or production claims outside this plan's evidence.

## Handoff Rule

Open an adjacent plan only after the selected workpack records the exact handoff reason and expected proof.
