# Rollback Proof

WP30 does not execute adapters and therefore has no runtime rollback path.

Rollback proof is represented only as protocol data in platform authority rows:

- proof references can include `rollback-proof`;
- proof-needed lists can require rollback proof before a future row moves out
  of manual-required or not-claimed state.
