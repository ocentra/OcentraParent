# Rollback Proof

Rollback is code-only for this slice.

Safe rollback path:

1. Revert the branch commit that adds app-game service read-model projection
   fields and mappers.
2. Rebuild contracts so generated package output returns to the previous
   activity-surface shape.
3. Re-run the focused protocol, core, service, and TypeScript contract tests.

No runtime adapter action, OS policy, enforcement timer, block list, shield,
unsuspend, unblock, uninstall prevention, or cleanup command is required because
this workpack does not execute platform actions.
