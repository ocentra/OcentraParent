# Rollback Proof

WP31 does not execute platform adapters, alter OS settings, block apps, kill
processes, hide packages, suspend packages, shield apps, or apply policy.

Rollback requirement for this workpack is therefore not applicable at runtime.
Future adapter work must attach rollback, cleanup, unblock, unsuspend, unshield,
and safe-failure proof before any platform action can move out of
manual-required state.
