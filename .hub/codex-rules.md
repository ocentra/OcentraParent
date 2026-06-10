# Codex Hub Rules

`.hub/lane-ledger.json` and `docs/hub/lane-ledger.md` are the portable authority for Ocentra Parent lane state.

## Non-Negotiable

- Do not treat `C:\Users\<you>\.codex` as truth for Ocentra Parent coordination state.
- Operational Codex state defaults to `.hub/state/worktree-lanes.json` and `.hub/state/ocentra-parent-hub`.
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

- Run `npm run hub:state:sync` before making coordination decisions on any PC.
- Run `npm run hub:state:sync -- -Commit` after semantic hub state changes.
- Do not actively work the same lane from two PCs at once.
- Heartbeat files may be blanked or truncated; messages, reports, locks, ledgers, and machine state must be preserved.
