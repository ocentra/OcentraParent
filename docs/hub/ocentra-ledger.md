<!-- agent-capsule -->

> Agent Capsule
> Doc: Ocentra Ledger Integration
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Ocentra Ledger Integration

Ocentra Ledger is the live coordination layer for Ocentra Parent. It replaces
repo-local `.hub` state with an external append-only event ledger.

The product repo remains source-only. It tracks:

- the `tools/ocentra-ledger` git submodule pointer;
- compatibility wrappers in `scripts/dev`;
- docs and CI/hooks that call Ledger.

It does not track inboxes, status files, worker heartbeats, locks, or generated
views.

## Install Model

Clone with submodules or initialize them after clone:

```powershell
git submodule update --init --recursive
npm install
npm run ledger:install
```

The parent wrapper also auto-initializes `tools/ocentra-ledger` on the first
Ledger command if the submodule folder is missing, so a synced worktree can run
`npm run hub:inbox` or `npm run ledger:ensure` directly.

The submodule points at `ocentra/OcentraParentHub`. If that repository is
private, each developer machine and CI runner needs read access. If it must work
without GitHub auth, the hub repo must be public or mirrored through another
accessible package/source channel.

## Runtime

Start the same Ledger binary as local API, browser dashboard, and peer server:

```powershell
npm run ledger:ensure
```

Then open:

```text
http://127.0.0.1:8787/
```

Use `LEDGER_HOST`, `LEDGER_PORT`, and peer aliases for LAN/tunnel use. The
default is localhost because one fixed local daemon per machine is enough for
normal worktree coordination.

## State Root

Ledger state is selected by `LEDGER_ROOT`:

```powershell
$env:LEDGER_ROOT="E:\OcentraLedger\ocentra-parent"
npm run ledger:ensure
```

If `LEDGER_ROOT` is not set, Ledger uses:

```text
~/.ocentra/ledger/ocentra-parent
```

The ledger root contains stable node identity, append-only NDJSON streams,
archives, peer aliases, runtime PID files, and generated views. These files are
not product source.

## Commands

Native Ledger commands:

```powershell
npm run ledger:root
npm run ledger:ensure
npm run ledger:dashboard
npm run ledger:doctor
npm run ledger:guard
npm run ledger:inbox -- codex-b
npm run ledger:workers
npm run ledger:free
npm run ledger:tasks
npm run ledger:message -- codex-b "Review the next slice."
npm run ledger:sync -- --peer ocentrahub
```

Compatibility aliases:

```powershell
npm run hub:message -- --lane codex-b --subject "Task" --body "Do the work."
npm run hub:inbox
npm run hub:ack
npm run hub:heartbeat -- --state alive --note "minute wake"
npm run hub:lock -- --paths "packages/activity-domain" --reason "assigned slice"
npm run hub:guard
npm run lanes:status
```

The aliases call Ledger. They do not write `.hub` files.

## Retention

Canonical truth is append-only events. Materialized inbox/status/ownership
views are generated and disposable. When event streams grow too large, compact
with Ledger retention commands after the event stream has enough preserved
history for audit/debugging.
