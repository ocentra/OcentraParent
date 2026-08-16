# 02 Source Index And Doc Reconciliation

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `02 Source Index And Doc Reconciliation`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Source docs, pasted guidance, current repo state, and implementation plan route
to one shared app/game plan without duplicating app-only and game-only evidence
systems.

## Scope

- Maintain `source-index.md` and `pasted-content-coverage-audit.md`.
- Link source docs without overriding feature/expectation truth.
- Record bridge gaps when existing docs are locked.
- Keep generated inventories and proof outputs out of the plan folder.

## Tests And Proof

- Markdown links are sane for changed files.
- `git diff --check` passes.
- Coverage audit names every pasted source input used.
- Product docs are changed only when status, proof, acceptance contract, or gap
  changes.

## Done Signal

The plan can explain where app, game, browser-game, policy, enforcement,
platform, and UI work belongs without ambiguity.

Use the standard checklist in [workpacks README](README.md).
