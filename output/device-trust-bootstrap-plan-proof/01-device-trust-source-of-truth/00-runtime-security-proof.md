# WP01 Parent-Presence Runtime Security Proof

plan: device-trust-bootstrap-plan
workpack: WP01 Device Trust Source Of Truth
owner: family-identity-core
trust_subject: parent-device
step_up_state: accepted-once / replay-rejected
replay_state: accepted-once / replay-rejected
platform_note: Windows-host focused filesystem and real-process proof
manual_required_note: cross-platform filesystem substitution parity remains unproven

## Proven scope

- The caller supplies an absolute SQLite path whose parent already exists. The repository does not create arbitrary parent directories.
- Challenge refs and nonce refs are durable unique identities. `BEGIN IMMEDIATE` serializes issuance/consumption, and database uniqueness is the final race guard.
- The public receipt is a stored 256-bit value from the operating-system CSPRNG. The transactional integer sequence remains private and is never returned or logged.
- Two synchronized child processes contend for one challenge: exactly one consumes it, the other observes replay rejection, and a fresh process after restart remains rejected.
- Two synchronized child processes issue different challenge refs with the same nonce: exactly one issues, the other receives duplicate-nonce rejection, and restart issuance remains rejected.
- Corrupt SQLite input is rejected without recreation or data replacement. Relative paths, missing parents, read-only database files, and host-supported final/ancestor symbolic substitutions fail closed.
- Challenge, assertion, receipt, and trust-bootstrap operational `Debug` output stays redacted.

## Validation

```text
command: cargo test -p ocentra-family-identity-core --test unit trust_bootstrap
result: pass (22 focused tests)

command: cargo test -p ocentra-family-identity-core
result: pass (12 contract tests, 76 unit tests, library and doc-test targets green)

command: cargo clippy -p ocentra-family-identity-core --tests -- -D warnings
result: pass

command: npm run lint:architecture -- --files <focused Rust source/test/docs/proof paths>
result: pass for focused Rust source/test paths; final docs/proof-inclusive rerun recorded before commit
```

## No-claim boundary

This artifact does not prove backup, export, restore, encrypted recovery, platform key sealing, passkey/WebAuthn ceremony, phone QR approval, entitlement unlock, full revocation integration, child tamper/uninstall behavior, or equivalent symbolic-substitution guarantees on every supported OS. WP01 and the overall plan remain partial.
