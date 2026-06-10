# Ocentra Ledger Integration

Ocentra Ledger is the replacement-track hub event ledger. It lives in the separate `ocentra/OcentraParentHub` repo and is installed here as a pinned development dependency.

The product repo remains source-only. Ledger state does not belong in this checkout.

## Install Model

`package.json` pins `@ocentra/parent-hub` to a specific `OcentraParentHub` commit. A new machine should be able to run:

```powershell
npm install
npm run ledger:ensure
```

Then open:

```txt
http://127.0.0.1:8787/
```

## Private Repo Requirement

`ocentra/OcentraParentHub` can stay private, but every machine or CI runner installing this repo must have GitHub read access to that private repo.

If install fails while fetching `OcentraParentHub`, authenticate GitHub for this machine first. For a no-auth install path, the hub package must be public or distributed through another authenticated package channel.

## State Root

Ledger state is selected by `LEDGER_ROOT`:

```powershell
$env:LEDGER_ROOT="E:\OcentraLedger\ocentra-parent"
npm run ledger:ensure
```

If `LEDGER_ROOT` is not set, Ledger uses:

```txt
~/.ocentra/ledger/ocentra-parent
```

The ledger root contains node identity, append-only NDJSON streams, archives, peer aliases, runtime PID files, and generated views. These files are not product source.

## Commands

```powershell
npm run ledger:root
npm run ledger:ensure
npm run ledger:dashboard
npm run ledger:doctor
npm run ledger:inbox -- codex-b
npm run ledger:workers
npm run ledger:free
npm run ledger:tasks
npm run ledger:message -- codex-b "Review the next slice."
npm run ledger:sync -- --peer ocentrahub
```

The old `hub:*` scripts remain in place during migration. Do not replace current product-repo hub callers until parity is proven against the Ledger materialized views.
