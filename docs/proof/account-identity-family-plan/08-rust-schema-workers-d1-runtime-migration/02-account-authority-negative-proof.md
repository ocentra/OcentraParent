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

The record-derived handoff adds direct contract negatives:

- parent, child, and device records from different households reject as
  `external-household` before authorization;
- a device record not bound to the supplied child profile rejects as
  `child-profile-not-bound`;
- a revoked device record rejects as `device-not-trusted`;
- stale session state rejects a remote-control action; and
- a controller lease for another parent member is ignored and the action
  rejects as `controller-lease-required`.

The PR #622 authority-safety repair adds exact handoff negatives:

- the target device must be listed in the identifier-only child target's
  `device_ids`; a matching child id alone is not a bound child-device proof;
- parent actions derive trust from a separate parent-controller proof, so a
  trusted child target cannot authorize a revoked or stale controller;
- an active lease expires at the injected canonical observed time and rejects
  as `controller-lease-expired` once its expiry is reached; and
- a lease must bind to the same parent member and parent-controller device,
  never the child action target.
