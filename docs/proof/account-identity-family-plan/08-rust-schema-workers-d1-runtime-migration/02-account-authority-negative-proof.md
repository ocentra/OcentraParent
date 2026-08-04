# WP08 account authority negative proof

The focused Rust tests fail closed at the canonical boundary:

- Wrong household/device binding: `device_registration_rejects_wrong_household_child_binding`.
- Revoked and stale authority: household-authority and session-lifecycle suites
  cover revoked devices, leases, sessions, and stale remote grants.
- Invite/recovery misuse: setup-lifecycle covers revoked, expired, replayed,
  non-single-use, wrong-household, and wrong-target paths.
- Duplicate authority identifier: `parent_presence_verification_rejects_duplicate_issuance_without_overwriting_original_binding`.
- Malformed/noncanonical authority input:
  `parent_presence_verification_rejects_malformed_noncanonical_and_offset_timestamps`.
- Schema-incompatible generated edge: the Rust-to-TypeScript byte-equality
  contract test rejects checked-in edge drift.

The direct duplicate/malformed tests each passed once in the WP08 lane; the
authority/lifecycle suites passed as recorded in `16-validation-commands.md`.
No runtime persistence or provider-login claim is made.
