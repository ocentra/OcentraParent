# Ocentra engineering graph

The engineering graph is the repository-owned control plane over the existing
plans, workpacks, checklists, tests, proof, ADRs, and agent instructions. It
does not replace those artifacts or duplicate their detailed intent.

## Authority boundaries

- `docs/engineering-graph/graph.json` owns dependency, readiness, execution
  state, and completion-contract relationships.
- Implementation-only authorization is a separate derived view. It never
  changes normal `READY`, validation, `DONE`, or completion state.
- Plan and workpack Markdown owns scope, acceptance expectations, and detailed
  instructions.
- Tests are technical validation evidence; proof is process evidence required
  by the selected workpack; ADRs own architectural decisions.
- `AGENTS.md` and the routed agent documents own execution behavior and safety.
- The user remains the authority for unresolved product or scope decisions.
- `code-map.json` owns reviewed plan-to-runtime ownership roots. It may also
  contain explicit `workpacks` entries for slices whose expected code/test
  shape and exact roots have been reviewed. `graph:code` and `graph:report`
  scan those roots live; file counts and expectation matches never promote a
  plan or workpack to accepted.

## Commands

```powershell
npm run graph:bootstrap             # preview the import
npm run graph:bootstrap -- --write # rebuild graph.json from docs/plans
npm run graph:validate
npm run graph:code
npm run graph:code -- PLAN-policy-control-plane-plan
npm run graph:report
npm run --silent graph:report -- --json
npm run graph:report -- PLAN-policy-control-plane-plan
npm run graph:matrix
npm run graph:matrix -- PLAN-policy-control-plane-plan --state blocked
npm run --silent graph:matrix -- --json
npm run graph:status
npm run graph:ready
npm run graph:parallel
npm run graph:next
npm run graph:next -- --phase implementation
npm run graph:blocked
npm run graph:inspect WP-policy-control-plane-plan-05-ask-parent-overrides
npm run graph:deps WP-policy-control-plane-plan-05-ask-parent-overrides
npm run graph:dependents WP-policy-control-plane-plan-04-delivery-ack-audit
npm run graph:why WP-policy-control-plane-plan-05-ask-parent-overrides
npm run graph:why WP-policy-control-plane-plan-05-ask-parent-overrides -- --phase implementation
```

`graph:validate` rebuilds the graph in memory from the current plan indexes and
fails if the checked-in `graph.json` has drifted. Run the bootstrap write command
after adding, removing, or renaming a workpack.

Use a plan or goal ID as a scope for `status`, `ready`, `parallel`, and
`blocked`. `parallel` is the deterministic set of independent workpacks whose
derived state is READY; when it is empty, the graph is not authorizing new
work.

`graph:code` accepts a plan ID or plan slug and answers the code-first question:
which reviewed runtime roots exist, how many implementation files are present,
and how many test files are present. It is intentionally a topology audit, not
a test runner or a completion certificate; focused test results, proof, CI,
checklists, and merge state remain separate gates.

Each reviewed workpack entry may declare `codeExpectation` as
`code-and-tests`, `tests-only`, or `no-code-required`. The default is
`code-and-tests`. A `no-code-required` entry must still be reviewed against the
workpack contract, but may use an empty `roots` array; this is how planning,
boundary-decision, and routing-only workpacks stop appearing as unknown code
ownership. `tests-only` requires at least one reviewed root and is satisfied
only when those roots contain tests and no implementation files. The report
always includes both the expected and observed topology so a mismatch remains
visible.

A reviewed runtime workpack whose source does not exist yet must use
`code-and-tests` with the exact missing production and test paths in `roots`.
It may also declare `plannedImplementationRoots` as a non-empty, production-only
subset of those roots and `expectedTestRoots` as a non-empty, test-classified
subset. These reviewed paths must be normalized repository-relative code-file
paths with supported language extensions. Documentation, proof/output,
generated, vendor, fixture/example/support-only, ignored, directory, symlink,
and outside-repository paths are rejected. A present path must be a regular file
of the declared production/test class. Bootstrap then replaces the default
workpack-Markdown implementation reference with missing expected production
paths and records the expected tests. This records ownership without creating
placeholder code and prevents planning documents or path-shaped directories
from satisfying executable completion.

The later test-source wave must add focused graph-tool regressions proving
that planned implementation roots reject unnormalized, unknown, duplicate,
non-code, directory, symlink, outside-root, documentation, proof/output,
generated, vendor, test, fixture, example, and ignored paths; that expected
test roots reject non-test-classified or non-regular paths; and that a
directory, symlink, or non-production file at an expected implementation path
cannot clear the completion gap.

`graph:report` is the canonical “where are we?” query. It joins every selected
plan's derived workpack states/counts and completion-contract path gaps with its
live reviewed-root implementation/test topology. The JSON form is intended for
agents and dashboards. A mapped workpack reports exact implementation/test
paths under `reviewed-workpack-roots`; every other row reports
`unknown-workpack-ownership` and inherits no plan-wide count. The report never
infers ownership from filenames or Markdown prose.

`graph:matrix` is the operator view for a plan-by-plan review. It prints a
summary row for every plan and a workpack row for every imported workpack with
derived state, reviewed code/test topology (or explicit unknown ownership),
completion-gap count, dependency blockers, and downstream unlocks. Use
`--state` to focus a handoff (for example `--state validation`) or `--json`
for automation. The matrix is a view over the graph; it is not a second
source of truth.

Use npm's `--silent` form for JSON output so npm does not prepend its lifecycle
banner to the machine-readable payload. The equivalent direct invocation is
`node scripts/engineering-graph.mjs report --json`.
The CLI accepts `--root <repo>` for an explicit repository root; npm commands
default to the repository containing the script.

`graph:next` first prints graph-authorized READY work. If no READY work exists,
it prints the unblocked active/validation queue and says explicitly that this
queue is not permission to start new work. That distinction prevents a
validation backlog from being mistaken for scheduler authorization.

`graph:next -- --phase implementation` is an explicitly narrower source-edit
queue. A row may appear there while its normal state remains `blocked`, but only
when its workpack dependency review and code ownership are reviewed and every
dependency meets its implementation threshold. Missing phase metadata defaults
to the normal requirement that the dependency is `DONE`. The command does not
authorize tests, proof, PR readiness, service activation, merge, or completion.
It also does not bypass the normal task route, exact-file claim, or Enforcer
guards.
Use `graph:why <id> -- --phase implementation` for exact ownership, lifecycle,
dependency, and reviewed-implementation blockers.

## Import policy

The bootstrap imports every plan directory and every workpack row that can be
parsed from its `WORKPACK_INDEX.md`. It supports both linked rows and the
existing numeric-ID table format when a matching `workpacks/<id>-*.md` file is
present. It records ambiguous or unknown imports in
`graph.json.migration.ambiguities`; it does not invent hard dependencies from
prose. Reviewed dependency edges live in `overrides.json` and must carry
evidence. A `workpackReviews` entry is the per-workpack migration gate for
dependency readiness: it must name an exact workpack node, provide an explicit
`hardDependencies` array (including `[]`), existing plan/workpack evidence, and
a reason. Its dependency IDs must exactly equal the existing valid reviewed
`depends_on` edges for that workpack. A valid review only sets
`dependencyConfidence=reviewed` and clears that workpack's `needsReview`; it
does not mark code, tests, proof, or completion done. Invalid or incomplete
override records fail graph bootstrap with a precise field/path error; they are
never silently ignored or converted into an ambiguous review item.

A reviewed `depends_on` edge may opt into
`implementationGate: "reviewed-implementation"`. That exception applies only
to the implementation-phase query. The dependency still must be `DONE` for
normal `READY`, validation, proof, PR readiness, and completion. Do not apply
the gate in bulk: the edge reason and evidence must explain why downstream
source can be written safely before predecessor tests and proof. Authority,
activation, custody, and release dependencies stay completion-gated unless the
owning workpack is decomposed into a genuinely safe source-only packet.

A reviewed `stateOverrides` entry may record a current validation
slice (never an unverified `done` claim) and must point to its proof manifest
and command evidence. A reviewed `proofOverrides` entry may point a completed
workpack at a durable plan-level manifest when that manifest explicitly covers
the workpack; a generic proof directory is not sufficient by itself. If the
workpack's test expectations declare a generated `output/` proof root that is
intentionally not checked in, the override must also set
`satisfiesExpected: true` and carry an existing evidence manifest. The graph
then accepts only the explicit durable proof references; it does not silently
ignore a missing output path.

The bootstrap also scans every Markdown file physically present under each
`workpacks/` directory. Files not linked from that plan's
`WORKPACK_INDEX.md` are recorded in
`graph.json.migration.unindexedWorkpackArtifacts` and added to the review
items. They are not promoted to workpack nodes because they may be README,
proposal, legacy, or support material. This keeps the import conservative
without hiding files from the operator.

## Completion

`done` is not a free-form checklist label. A workpack's completion contract
points to implementation, test, proof, and checklist artifacts. Validation
rejects duplicate IDs, missing references, dependency cycles, invalid states,
contradictory readiness, and `done` nodes whose contract is incomplete. When a
plan declares an `output/` or other generated proof root, that path is an
expected artifact and normally must exist; a stale imported `done` row is
demoted to `validation` during bootstrap and `graph why` reports the exact
missing path. An explicit evidence-backed durable-proof override is the only
exception for a workpack that names a checked-in proof bundle as its retained
artifact. Proof root conventions are imported from each plan's
`TEST_PROOF_EXPECTATIONS.md`; durable `docs/proof/<plan>` manifests are
accepted when the plan explicitly retains them, but they do not silently
replace a missing generated artifact.

Plan and workpack Markdown is context, not implementation/test execution proof.
The graph therefore does not accept a `done` state from paths alone. A reviewed
`completionEvidenceOverrides` entry must provide concrete implementation, test,
proof, and checklist evidence paths for the workpack; each path is checked for
existence and recorded as reviewed. This keeps a source inventory or a checklist
row from silently becoming a completion claim.

Completion evidence may be recorded one requirement at a time. A partial entry
containing only `implementation` must still provide a non-empty reason, existing
review evidence, and exact existing non-planning repository-relative regular
source files with an allowed executable extension. Test paths, documentation,
directories, symlink escapes, and traversal paths are rejected. It can satisfy a
reviewed-implementation dependency gate, but the predecessor remains incomplete
until every required test, proof, checklist, and ADR requirement is reviewed.
Code-map topology or file presence alone never satisfies this gate.

All report, matrix, `graph:next`, and implementation-phase queries regenerate
the graph from current plan/workpack sources and reject a checked-in graph that
differs from that result. `--phase` accepts only `implementation` on phase-aware
queries; unsupported or missing values fail instead of being ignored.

## Adding a workpack

1. Add the workpack to the owning plan's `WORKPACK_INDEX.md`.
2. Keep detailed scope, expected tests, proof, and ADR requirements in the
   existing routed documents.
3. Add only reviewed hard dependencies to `overrides.json` with evidence. Add
   `implementationGate: "reviewed-implementation"` only to an individually
   reviewed edge whose downstream source is safe before predecessor completion.
4. Add a `workpackReviews` entry only after reviewing the exact workpack's
   dependency context; use an explicit empty `hardDependencies` array when the
   next source-code slice has no hard code-writing prerequisite. Do not use this
   entry to suppress unrelated plan/workpack ambiguities.
5. If the expected code/test shape is known, add a
   `code-map.json.workpacks` entry with the workpack ID, `codeExpectation`, and
   reviewed file/directory roots. Use `no-code-required` with empty roots only
   after the workpack contract is reviewed. Leave it unmapped when ownership or
   expected topology is uncertain. For reviewed runtime source that is planned
   but absent, use `code-and-tests`, include the exact missing production and
   test roots, declare the production-only subset in
   `plannedImplementationRoots`, and declare the test-classified subset in
   `expectedTestRoots`.
6. Run `npm run graph:bootstrap -- --write` and `npm run graph:validate`.
7. Query `graph:inspect <workpack-id>` before assigning the workpack.
8. For a code-first pass, query `graph:next -- --phase implementation`; do not
   reinterpret that result as normal READY or completion authority.

The graph is intentionally conservative: an ambiguous imported workpack stays
`planned` until its dependency/readiness context is reviewed.

See `DOGFOOD.md` for the first live status/ready/blocked/inspect/why query and
the resulting parallel-ready set.
