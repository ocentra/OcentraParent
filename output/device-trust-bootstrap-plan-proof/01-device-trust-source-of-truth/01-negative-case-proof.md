## Negative cases

- login alone does not create trust: `evaluate_provisioning_readiness` still blocks until the trust-facing states are satisfied
- copied binaries or install copies do not create trust: the crate keys off typed readiness state, not filesystem presence
- revoked or stale pairing state does not pass through as ready: the readiness tests keep replayed, stale, wrong-household, wrong-device, and anonymous-device paths blocked
- parent authority remains required for accepted-but-untrusted pairing: pending device trust still maps to an explicit trust blocker
- permission revocation is surfaced as a blocker, not as a hidden retry state
- policy baseline missing or stale remains blocked
- data custody sync pending remains degraded, not fake-ready
- child install not installed and reinstall-required remain blocked
- child service not started remains blocked
- network offline, LAN unavailable, and direct-entry-required remain explicit recovery states
- recovery states stay typed, including wrong-account, lost-parent-device, permission-loss, and stale-code paths

## Coverage source

- `crates/provisioning-core/tests/unit/readiness.rs`
- `crates/provisioning-core/tests/unit/readiness_flow.rs`

## No-surrogate note

- These checks are real Rust unit coverage over the provisioning slice.
- They do not claim whole-plan device trust, platform ceremony proof, or sibling-plan readiness.
