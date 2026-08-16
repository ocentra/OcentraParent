<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan`
> Kind: short plan entry point.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# Network Plan

This is the short, token-efficient entry point for `network-plan`. The original full
README content is preserved at [README_FULL_ORIGINAL.md](README_FULL_ORIGINAL.md) and is not default context.

## Default agent path

1. Read [AGENTS.md](AGENTS.md).
2. Read [PLAN_STATE.md](PLAN_STATE.md).
3. Read [NEXT_ACTIONS.md](NEXT_ACTIONS.md) when starting/resuming.
4. Read [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
5. Open only the assigned workpack.
6. Use [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) to locate exact checklist sections.
7. Use [PROOF_INDEX.md](PROOF_INDEX.md) only when proof artifacts are needed.

## Current scope

This folder is the single working plan location for child-device network evidence, domain observation, DNS and flow classification, process/app/browser correlation, network-triggered cross-slice evidence cascade, network policy handoff, DNS/firewall/VPN/WFP/NetworkExtension intervention paths, proof artifacts, and parent UI.

## Do not default-read

- `implementation-checklist.md` unless the route names exact rows/sections.
- `source-index.md` unless source ownership is unclear.
- all files under `workpacks/`.
- sibling plan folders.
- historical proof/checkpoint docs not named by the workpack.
