# Codex Hub Rules

`.hub/lane-ledger.json` and `docs/hub/lane-ledger.md` are the portable authority for durable Ocentra Parent lane declarations, PR state, validation, merge state, and cleanup safety.

Live mailbox, ack, heartbeat, ownership, and report traffic is operational transport. It must live outside the product repository in OcentraHub or, during migration only, in the legacy global hub root.

## Non-Negotiable

- Do not commit live hub transport state to `OcentraParent`.
- Do not treat `.hub/state/**` in the product repo as durable truth.
- During migration, the legacy live hub root is `C:\Users\<you>\.codex\ocentra-parent-hub`.
- The target live hub transport is OcentraHub: an external, append-only event ledger that materializes inbox/status/ownership views.
- Actual worktree checkouts may live outside the repo.
- Every `DONE`, `PR_READY`, merge, close, park, retarget, or cleanup-safe report must update the repo ledger files or explicitly state why no ledger change was needed.
- `localWorktreePath` is a convenience hint. Remote branch, PR number, merge commit, status, validation, and next action are the portable record.
- Never set `safeToDeleteWorktree` to `true` without a local audit of dirty files, unpushed commits, open PRs, and remote-only work.

## Required Report Shape

Each lane handoff should include:

- lane id;
- branch and remote branch;
- PR number or `none`;
- ledger status;
- last relevant commit;
- validation;
- known gaps or blockers;
- next action;
- whether `.hub/lane-ledger.json` and `docs/hub/lane-ledger.md` were updated.

## One-At-A-Time Integration

When PR sequencing is paused or ordered by the user, primary should pick one PR/lane, ask only that owner for needed rebase or fixes, integrate it after review and green CI, update the ledger, then move to the next. Do not send broad rebase/sync waves to unrelated waiting PR lanes.

## Two-PC Sync

- Do not rely on product-repo commits for hot hub sync.
- Sync live hub events through OcentraHub, or through the legacy global hub root until OcentraHub replaces it.
- Do not actively work the same lane from two PCs at once.
- Heartbeat files may be blanked or truncated; messages, reports, locks, ledgers, and machine state must be preserved.
