<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `ROUTE_INDEX.md`
> Kind: local route map.
> Read when: When deciding whether this plan owns the task or an adjacent plan owns the handoff.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Route changes require PLAN_INDEX and FEATURE_ROUTE_INDEX updates.

<!-- /agent-capsule -->

# Data Custody Storage Plan Route Index

## Owns

- data custody guarantees, encrypted storage, evidence retention, export, sync, deletion/tombstones, no-stolen-data boundaries, cloud/relay custody, and query/report source truth.

## Does Not Own

- Adjacent implementation completion in: eventing-plan, portal-ux-household-surfaces-plan, parent-desktop-runtime-package-plan
- Broad source rewrites without selected workpack proof.
- Release or production claims outside this plan's evidence.

## Handoff Rule

Open an adjacent plan only after the selected workpack records the exact handoff reason and expected proof.
