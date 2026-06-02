# 02 Source Index And Doc Reconciliation

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
