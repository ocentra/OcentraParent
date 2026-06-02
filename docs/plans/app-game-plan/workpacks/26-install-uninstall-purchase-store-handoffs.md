# 26 Install, Uninstall, Purchase, And Store Handoffs

## Target State

Install, uninstall, purchase, and store signals are routed as app/game evidence
and approval handoffs without pretending the full install/purchase product is
complete.

## Scope

- New app/game detected.
- Installer/updater process.
- Store package install signal.
- Game purchase capability or purchase event where available.
- Uninstall/tamper handoff.
- App-install/purchase approval feature handoff.

## Tests And Proof

- Store/purchase signal is context, not automatic decision.
- Install approval refs include evidence.
- Uninstall/tamper routes to owning feature docs.
- Manual-required states are visible where platform support is missing.

## Done Signal

App/game plan can hand off install, uninstall, purchase, and store work without
silently claiming another feature.

Use the standard checklist in [workpacks README](README.md).
