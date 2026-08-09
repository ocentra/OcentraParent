# Ocentra engineering graph

The engineering graph is the repository-owned control plane over the existing
plans, workpacks, checklists, tests, proof, ADRs, and agent instructions. It
does not replace those artifacts or duplicate their detailed intent.

## Authority boundaries

- `docs/engineering-graph/graph.json` owns dependency, readiness, execution
  state, and completion-contract relationships.
- Plan and workpack Markdown owns scope, acceptance expectations, and detailed
  instructions.
- Tests are technical validation evidence; proof is process evidence required
  by the selected workpack; ADRs own architectural decisions.
- `AGENTS.md` and the routed agent documents own execution behavior and safety.
- The user remains the authority for unresolved product or scope decisions.
- `code-map.json` owns reviewed plan-to-runtime ownership roots. `graph:code`
  scans those roots live and reports implementation/test topology; file counts
  never promote a plan or workpack to accepted.

## Commands

```powershell
npm run graph:bootstrap             # preview the import
npm run graph:bootstrap -- --write # rebuild graph.json from docs/plans
npm run graph:validate
npm run graph:code
npm run graph:code -- PLAN-policy-control-plane-plan
npm run graph:report
npm run graph:report -- --json
npm run graph:report -- PLAN-policy-control-plane-plan
npm run graph:status
npm run graph:ready
npm run graph:parallel
npm run graph:next
npm run graph:blocked
npm run graph:inspect WP-policy-control-plane-plan-05-ask-parent-overrides
npm run graph:deps WP-policy-control-plane-plan-05-ask-parent-overrides
npm run graph:dependents WP-policy-control-plane-plan-04-delivery-ack-audit
npm run graph:why WP-policy-control-plane-plan-05-ask-parent-overrides
```

Use a plan or goal ID as a scope for `status`, `ready`, `parallel`, and
`blocked`. `parallel` is the deterministic set of independent workpacks whose
derived state is READY; when it is empty, the graph is not authorizing new
work.

`graph:code` accepts a plan ID or plan slug and answers the code-first question:
which reviewed runtime roots exist, how many implementation files are present,
and how many test files are present. It is intentionally a topology audit, not
a test runner or a completion certificate; focused test results, proof, CI,
checklists, and merge state remain separate gates.

`graph:report` is the canonical “where are we?” query. It joins every selected
plan's derived workpack states/counts and completion-contract path gaps with its
live reviewed-root implementation/test topology. The JSON form is intended for
agents and dashboards. Workpack rows deliberately say
`codeTestTopology: plan-reviewed-roots` because this repository does not yet
have reviewed workpack-to-file ownership maps; the report never infers those
maps from filenames or Markdown prose.

## Import policy

The bootstrap imports every plan directory and every workpack row that can be
parsed from its `WORKPACK_INDEX.md`. It supports both linked rows and the
existing numeric-ID table format when a matching `workpacks/<id>-*.md` file is
present. It records ambiguous or unknown imports in
`graph.json.migration.ambiguities`; it does not invent hard dependencies from
prose. Reviewed dependency edges live in `overrides.json` and must carry
evidence. A reviewed `stateOverrides` entry may record a current validation
slice (never an unverified `done` claim) and must point to its proof manifest
and command evidence. A reviewed `proofOverrides` entry may point a completed
workpack at a durable plan-level manifest when that manifest explicitly covers
the workpack; a generic proof directory is not sufficient by itself.

## Completion

`done` is not a free-form checklist label. A workpack's completion contract
points to implementation, test, proof, and checklist artifacts. Validation
rejects duplicate IDs, missing references, dependency cycles, invalid states,
contradictory readiness, and `done` nodes whose contract is incomplete. When a
plan declares an `output/` or other generated proof root, that path is an
expected artifact and must exist too; a stale imported `done` row is demoted to
`validation` during bootstrap and `graph why` reports the exact missing path.
Proof root conventions are imported from each plan's
`TEST_PROOF_EXPECTATIONS.md`; durable `docs/proof/<plan>` manifests are
accepted when the plan explicitly retains them, but they do not silently
replace a missing generated artifact.

## Adding a workpack

1. Add the workpack to the owning plan's `WORKPACK_INDEX.md`.
2. Keep detailed scope, expected tests, proof, and ADR requirements in the
   existing routed documents.
3. Add only reviewed hard dependencies to `overrides.json` with evidence.
4. Run `npm run graph:bootstrap -- --write` and `npm run graph:validate`.
5. Query `graph inspect` before assigning the workpack.

The graph is intentionally conservative: an ambiguous imported workpack stays
`planned` until its dependency/readiness context is reviewed.

See `DOGFOOD.md` for the first live status/ready/blocked/inspect/why query and
the resulting parallel-ready set.
