# 01 - Prerequisite Merge And Branch Gate

## Target State

Pipeline work starts only after screen and AI prerequisite implementations are
merged to `main` or explicitly approved as stacked heads.

## Checklist

- [ ] Record screen prerequisite branch/commit/PR.
- [ ] Record AI prerequisite branch/commit/PR.
- [ ] Confirm pipeline branch contains both implementations.
- [ ] Confirm no stale capture or AI proof assumptions.
- [ ] Run lane/hub guards before edits.

## Proof

- `01-prerequisite-commits.json`.
- Git status and branch base recorded.
- PR body names prerequisite commits.
