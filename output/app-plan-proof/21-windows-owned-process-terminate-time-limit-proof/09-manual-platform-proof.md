# Native App Manual Platform Proof

Shared proof command:

```text
node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs
```

Observed proof on Windows:

- Dry-run: `would-enforce/dry-run-no-action`
- Stale mismatch: `enforcement-active-timer-state-mismatch`
- Recovery: `restart-recovered`
- Cancel: `cancelled`
- Expiry: `expired/process-terminated`

This does not prove broad installed-app blocking or package-level launch
control.
