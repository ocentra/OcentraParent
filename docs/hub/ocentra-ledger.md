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

# Ocentra Enforcer Coordination Integration

Ocentra Enforcer is the live coordination layer for this repo. It replaces
repo-local `.hub` state and the old Parent-owned ledger submodule with an
external append-only event ledger owned by the Enforcer install.

The product repo remains source-only. It tracks:

- `ocentra-enforcer.config.json`;
- thin npm aliases that call `scripts/enforcer/run-ocentra-enforcer.mjs`;
- docs and CI/hooks that call Enforcer coordination.

It does not track inboxes, status files, worker heartbeats, locks, or generated
views.

## Install Model

Install or clone Enforcer once per machine, then run the repo aliases:

```powershell
npm install
npm run ledger:root
npm run ledger:ensure
```

`scripts/enforcer/run-ocentra-enforcer.mjs` resolves Enforcer from
`OCENTRA_ENFORCER_HOME`, `node_modules/ocentra-enforcer`, a sibling
`ocentra-enforcer` checkout, or `E:\ocentra-enforcer`. The old
`tools/ocentra-ledger` submodule is not a source of truth.

## Runtime

Start the Enforcer coordination daemon/API/dashboard:

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

Coordination state is selected by `LEDGER_ROOT`:

```powershell
$env:LEDGER_ROOT="E:\ocentra-enforcer\.ledger\ocentra-parent"
npm run ledger:ensure
```

If `LEDGER_ROOT` is not set, the Parent wrapper uses:

```text
E:\ocentra-enforcer\.ledger\ocentra-parent
```

The ledger root contains stable node identity, append-only NDJSON streams,
archives, peer aliases, runtime PID files, and generated views. These files are
not product source.

## Commands

Enforcer-backed coordination commands:

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

Hub/lane aliases:

```powershell
npm run hub:message -- --lane codex-b --subject "Task" --body "Do the work."
npm run hub:inbox
npm run hub:ack
npm run hub:heartbeat -- --state alive --note "minute wake"
npm run hub:lock -- --paths "packages/activity-domain" --reason "assigned slice"
npm run hub:guard
npm run lanes:status
```

The aliases call Enforcer coordination. They do not write `.hub` files and do
not call Parent-owned ledger implementation code.

## Retention

Canonical truth is append-only events. Materialized inbox/status/ownership
views are generated and disposable. When event streams grow too large, compact
with Ledger retention commands after the event stream has enough preserved
history for audit/debugging.
