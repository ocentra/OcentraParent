# 20 Time Budget, Schedule, And Bonus-Time Integration

## Target State

App and game schedules/budgets consume session summaries and preserve approval,
bonus-time, schedule, and audit refs.

## Scope

- App daily/weekly limits.
- Game budgets.
- Schedule windows.
- Bonus time.
- Ask-parent extension.
- Timer state and recovery refs.

## Tests And Proof

- Time budget uses stored session summaries.
- Foreground-only and running-time modes stay distinct.
- Bonus time extends decision with audit refs.
- Dry-run exposes expected action without executing.
- Restart recovery preserves active timer state where implemented.

## Done Signal

Time/budget decisions can be explained from evidence, schedule, parent action,
and capability state.

Use the standard checklist in [workpacks README](README.md).
