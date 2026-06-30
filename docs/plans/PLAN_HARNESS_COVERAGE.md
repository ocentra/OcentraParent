<!-- agent-capsule -->

> Agent Capsule
> Doc: `Plan Harness Coverage`
> Kind: coverage manifest for the plan/workpack harness hardening branch.
> Read when: reviewing whether this branch applied the planning/proof doctrine to the touched plan docs.
> Stop rule: this is a coverage manifest, not permission to edit all plans or source trees.
> Proves: scope of the docs/workpack harness correction only.
> Does not prove: implementation completion, source validation, test execution, product readiness, PR readiness, or completion of untouched plans.
> Proof rule: update this file whenever this branch changes plan/workpack coverage or discovers a blocker.

<!-- /agent-capsule -->

# Plan Harness Coverage

Branch: `codex/plan-harness-update`

This file makes the harness-coverage claim testable. The branch applied the same discipline it asks Codex agents to follow: route by plan, inspect actual workpack bodies, preserve proof history, correct stale owner paths, separate shared schema from feature-owner runtime, and write no-claim boundaries instead of broad readiness claims.

Current Rust-first authority supersedes this older harness wording where it
describes `schema-domain` as canonical product truth. Treat that wording as
migration/debt inventory. New shared product contracts, route snapshots,
actions, read models, and product schemas must route through `crates/schema` or
the owning Rust domain/runtime crate; TypeScript keeps only presentation,
generated DTO consumption, thin adapters, or temporary edge decoders.

## Coverage standard

A plan/workpack is considered harness-hardened only when the touched docs satisfy these conditions:

```text
1. Actual route files were inspected.
2. Actual workpack bodies were inspected for high-risk executable rows.
3. Stale owner paths were corrected where found.
4. Shared cross-boundary shapes route through `crates/schema` or the owning Rust domain/runtime crate; old `schema-domain` references are migration/debt only.
5. Feature-owner runtime imports are forbidden at the workpack level where risk exists.
6. Expected proof includes command/log artifact, negative case, owner path, and no-claim boundary.
7. Historical proof records are preserved but not treated as current branch validation.
8. Known tool blockers are recorded explicitly instead of hidden.
```

## Plans covered in this branch

```text
account-identity-family-plan
ai-plan
app-game-plan
app-plan
browser-plan
```

Plans not yet covered are intentionally outside this audit claim.

## Account identity coverage

Plan-level files were hardened and the following actual workpack bodies were read and patched:

```text
workpacks/01-auth-provider-decision.md
workpacks/02-identity-household-role-model.md
workpacks/04-invites-recovery-lifecycle.md
workpacks/05-device-ownership-authz.md
workpacks/06-security-proof-and-route-gate.md
workpacks/07-parent-account-family-setup-ui.md
```

`workpacks/03-session-token-lifecycle.md` could not be edited directly because the tool safety layer blocked the update. The current route is therefore:

```text
workpacks/03-session-token-lifecycle.md
plus workpacks/03-current-boundary-addendum.md
```

The workpack index now routes WP03 through that addendum.

## AI coverage

Plan-level files were hardened and the following actual workpack bodies were read and patched:

```text
workpacks/03-contract-boundary-and-effect-schemas.md
workpacks/07-ai-job-queue-contract.md
workpacks/32-family-ai-hub-and-remote-assistant-boundary.md
```

These represent the highest-risk AI owner-boundary families in the current queue: canonical contracts, queue/job contracts, and provider-mesh versus remote-assistant separation.

## App/game coverage

Plan-level files were hardened and the following actual workpack body was read and patched:

```text
workpacks/01-contract-boundary-and-effect-schemas.md
```

The branch also added `WORKPACK_FAMILIES.md` and `workpacks/00-owner-boundary-proof-gate.md` because the plan has a large generated workpack inventory. This is not a claim that every generated workpack body was individually patched.

## Native apps coverage

Plan-level files were hardened and the following actual workpack body was read and patched:

```text
workpacks/01-contract-boundary-and-effect-schemas.md
```

The branch also added `WORKPACK_FAMILIES.md` and `workpacks/00-owner-boundary-proof-gate.md` because the plan has many generated or ambiguous rows. This is not a claim that every generated workpack body was individually patched.

## Browser coverage

Plan-level files were hardened and the following actual workpack body was read and patched:

```text
workpacks/01-contract-boundary-and-effect-schemas.md
```

The branch also added `WORKPACK_FAMILIES.md` and `workpacks/00-owner-boundary-proof-gate.md` because the plan has very large reference/control inventories. This is not a claim that every reference inventory was individually patched.

## Known non-claims

```text
No source files were edited by this harness pass.
No tests were run by this harness pass.
No product readiness is claimed.
No PR_READY is claimed.
No untouched plan is covered by this audit.
No generated workpack inventory is claimed fully patched unless its body is listed above.
```

## Remaining follow-up rule

For all future plans and workpacks, do not call route hardening complete until the selected workpack body has been inspected. Use gates and family files as overlays only; they are not replacements for workpack-body correction.
