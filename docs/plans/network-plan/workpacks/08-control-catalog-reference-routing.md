# WP08 Control Catalog Reference Routing

Scope: route the large network control capability guide, schema proposal, and settings inventory without forcing agents to read them by default.

Source rows: moved root source docs under `workpacks/`.

Read next:

- `network-control-capability-guide.md` only when capability surface or parent wording is assigned.
- `network-control-schema-proposal.md` only when schema shape or contract fields are assigned.
- `network-control-settings-inventory.md` only when an exact setting/provider/control family is assigned.
- `../source-index.md` for source ownership lookup.

## Ownership boundary

```text
WP08 owns reference routing and token control only.
Network control catalog docs are source/reference material, not implementation proof.
Selected controls must route into WP01-WP07 or the owning sibling plan before source work begins.
Policy, UI, billing, data custody, enforcement, setup, device trust, and portal decisions remain in their owning plans.
```

## Expected outcome

- Large reference docs are treated as source material, not implementation proof.
- Any extracted control becomes a bounded work item under WP01-WP07 or the owning enforcement/policy/portal plan.
- Settings inventory rows are never copied wholesale into context; agents open only the exact needed section/search hit.

## Required proof fields

The selected proof must name, at minimum:

```text
source_file
search_term_or_section
selected_control_family
owning_plan
selected_workpack
rejected_out_of_scope_controls
runtime_claim_state
implementation_claim_state
no_default_read_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Expected tests/proof

- `network.control-catalog.route-note`
- `network.control-catalog.no-default-read`
- `network.control-catalog.claim-boundary`
- Proof records exact source file, search term/section, selected plan/workpack, and rejected out-of-scope controls.

## Failure conditions

- Do not treat the 300k+ settings inventory as the plan.
- Do not create broad implementation claims from catalog existence.
- Do not route policy, UI, billing, or data custody decisions into network just because a setting touches network behavior.
- Do not use catalog rows to claim runtime, platform, parent UX, billing, custody, or enforcement readiness.
- Do not copy whole catalog sections into an agent prompt when an exact search hit or section is sufficient.

## Current slice note

- The 2026-06-17 `network-foundation-shim-cleanup` slice uses WP08 only to keep the control-catalog surface honest while the parent-domain shim cleanup is in progress.
- The exact remaining decision is whether `@ocentra-parent/parent-domain` continues to publish `./network-control-catalog`, or whether that path must be retired before the remaining control-catalog shim files are deleted.

## Live validation — 2026-08-09

- Focused contract test: `node --test tests/network-plan/contract/control-catalog-reference-routing.test.mjs` — 1 passed, 0 failed.
- Scoped architecture validation and `npm run hub:guard` passed.
- Durable validation manifest: `docs/proof/network-plan/slice-08-control-catalog-routing.md`.
- This is `validation`, not `done`: the catalog remains reference material and does not prove network runtime, policy, enforcement, portal, platform, or production behavior.
