# 01 - Source Index And Repo Reconciliation

## Target State

The AI plan indexes the real repo and reference sources before any worker starts
implementation.

## Where We Are

AI source truth is spread across `docs/expectations`, `docs/features`,
`docs/architecture`, `packages/parent-domain`, Rust service/core crates,
portal surfaces, proof scripts, and local TabAgent references.

## Checklist

- [ ] Refresh `source-index.md` against current repo.
- [ ] Refresh `tabagent-source-index.md` before reuse work.
- [ ] Reconcile browser, screen, app/game, tracking, LAN, and activity plan
      links.
- [ ] Mark stale docs or duplicate claims.
- [ ] Name exact owner docs that need product status updates.

## Proof

- `git diff --check -- docs/plans/ai-plan`
- Source index includes docs, contracts, Rust, portal, tests, proof scripts, and
  reference boundaries.
