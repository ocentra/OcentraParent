# 01 - Source Index And Repo Reconciliation

## Target State

The AI plan indexes the real repo and reference sources before any worker starts
implementation.

## Where We Are

AI source truth is spread across `docs/expectations`, `docs/features`,
`docs/architecture`, `packages/parent-domain`, Rust service/core crates,
portal surfaces, proof scripts, and local TabAgent references.

## Checklist

- [x] Refresh `source-index.md` against current repo.
- [x] Refresh `tabagent-source-index.md` before reuse work.
- [x] Reconcile browser, screen, app/game, tracking, LAN, and activity plan
      links.
- [x] Mark stale docs or duplicate claims.
- [x] Name exact owner docs that need product status updates.

## Current Reconciliation

- `source-index.md` now reflects the concrete local AI, screen AI, household
  mesh, event bridge, provider route, child authority, memory/graph, remote
  boundary, and enforcement-consumption proof scripts present in the repo.
- `tabagent-source-index.md` was rechecked against `E:\Desktop\TabAgent`; every
  indexed candidate file is still present. TabAgent remains reference-only.
- Browser, screen, app/game, tracking, LAN, eventing, and activity plan links
  remain delegated to their owning plan directories. The AI source index does
  not duplicate their source truth.
- Stale claim corrected: household mesh is no longer purely planned for the
  screen-derived local route. Physical household LAN transport, live gossip,
  lease expiry/dead-letter runtime behavior, production model execution, portal
  mesh UI, and model cache corruption proof remain open.
- Product status owners: `docs/features/local-ai-safety-evaluator.md`,
  `docs/features/parent-assistant-actions.md`, and
  `docs/product-capability-checklist.md`. The shared product checklist remains
  the cross-feature status table and must be updated when its lane lock is
  available.

## Proof

- `git diff --check -- docs/plans/ai-plan`
- Source index includes docs, contracts, Rust, portal, tests, proof scripts, and
  reference boundaries.
