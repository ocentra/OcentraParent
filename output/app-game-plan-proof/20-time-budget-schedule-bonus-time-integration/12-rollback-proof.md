# Rollback Proof

No runtime rollback path changed in WP20.

The new contract prevents enforcement overclaiming by keeping exceeded-budget
outcomes in dry-run, ask-parent, or manual-required actions. Adapter dispatch,
process termination, package blocking, rollback execution, and cleanup proof
remain deferred to runtime/enforcement workpacks.
