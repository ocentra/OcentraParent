# Rollback Proof

Rollback execution was not implemented in WP23.

The contract requires rollback proof before any supported broad-blocking row can
claim adapter dispatch. Manual-required and unavailable rows carry
`rollback-required`; supported upgrade candidates are rejected unless
`rollback-proof` is attached and `rollbackState` is
`rollback-proof-attached`.

No unblock, unsuspend, unshield, allowlist restore, or system-app exception
execution is claimed.
