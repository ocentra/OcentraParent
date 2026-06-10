# Ocentra Ledger Worktree Coordination

Ocentra Parent no longer keeps live hub state in this product repo. The product
repo owns code, docs, scripts, and the pinned Ledger submodule only.

Live coordination state belongs in Ocentra Ledger:

```text
tools/ocentra-ledger
```

That path is a git submodule pointing at `ocentra/OcentraParentHub`. The actual
event streams, identity files, runtime PID files, peer aliases, and generated
views are external to this checkout.

## State Root

Set `LEDGER_ROOT` when a machine needs an explicit state location:

```powershell
$env:LEDGER_ROOT="E:\OcentraLedger\ocentra-parent"
npm run ledger:ensure
```

Without `LEDGER_ROOT`, Ledger uses:

```text
~/.ocentra/ledger/ocentra-parent
```

The state root is disposable/rebuildable except for append-only event streams
and identity. Do not commit it to this repo.

## Lanes

Stable lane identities are explicit:

- `primary`: the user's main checkout and coordinator.
- `codex-a`: reusable Codex worktree lane.
- `codex-b`: reusable Codex worktree lane.
- `codex-c`: reusable Codex worktree lane.
- `codex-d` and `E-*`: additional reusable lanes when assigned.

Set `LEDGER_LANE` or `OCENTRA_PARENT_LEDGER_LANE` in worker shells when the
checkout path does not make the lane obvious. The compatibility wrappers infer
common `codex-a`/`codex-b`/`E-A` path names, but explicit lane identity wins.

## Commands

Install/build the pinned Ledger submodule:

```powershell
npm run ledger:install
```

If the submodule is missing after a fresh pull, the Ledger wrapper initializes
`tools/ocentra-ledger` automatically on the first command.

Start the local browser/API daemon:

```powershell
npm run ledger:ensure
```

Then open:

```text
http://127.0.0.1:8787/
```

Check the current state:

```powershell
npm run ledger:root
npm run ledger:doctor
npm run ledger:workers
npm run ledger:tasks
```

Send work to a lane:

```powershell
npm run hub:message -- --lane codex-b --subject "V1 slice" --body "Do the assigned work and report DONE with validation."
```

Read and acknowledge mail:

```powershell
npm run hub:inbox
npm run hub:ack
```

Claim and release edit paths:

```powershell
npm run hub:lock -- --paths "packages/activity-domain" --reason "activity status slice"
npm run hub:unlock -- --paths "packages/activity-domain"
```

Report work state:

```powershell
npm run hub:report -- --summary "STARTED activity status slice"
npm run hub:heartbeat -- --state alive --note "minute wake"
```

The old `hub:*` and `lanes:*` names are compatibility aliases that call Ledger.
They do not write `.hub` files.

## Guard

The pre-commit hook calls:

```powershell
npm run ledger:guard
```

The guard checks unread Ledger messages, ownership conflicts, and changed files
outside active claims. `primary` may coordinate without claims; worker lanes
should claim intended paths before editing.

Use the bypass environment variables only for deliberate emergency commits:

```powershell
$env:OCENTRA_PARENT_SKIP_LANE_GUARD="1"
$env:OCENTRA_PARENT_SKIP_HUB_GUARD="1"
```

## Submodule Policy

Do not gitignore `tools/ocentra-ledger`. The parent repo tracks only the
submodule pointer and `.gitmodules`; Ledger code changes are committed in
`E:\OcentraParentHub` and then the parent repo pointer is advanced.

Generated Ledger files belong under `LEDGER_ROOT`, not under this product repo.
