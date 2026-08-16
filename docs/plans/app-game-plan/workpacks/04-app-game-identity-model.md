# 04 App/Game Identity Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `04 App/Game Identity Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Multiple evidence fields can identify one app or game without display-name-only
matching.

## Scope

- Model platform, product kind, package id, bundle id, AppUserModelId, desktop
  entry id, application token ref, executable path ref, publisher/signature ref,
  file hash ref, launcher app id, launcher manifest id, store id, parent label,
  and display label.
- Separate launcher identity from child game identity.
- Represent weak, candidate, deterministic, parent-labeled, and AI-assisted
  identity confidence.

## Tests And Proof

- Same display name with different hash does not merge.
- Same launcher with no child game proof stays launcher-only.
- Store package and process can merge only through deterministic identifiers.
- Parent label changes display, not raw identity.

## Done Signal

App and game rows can merge through layered identity evidence while unknown and
candidate rows remain honest.

Use the standard checklist in [workpacks README](README.md).
