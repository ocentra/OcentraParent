# WP04 Owned-Process Time Limit

## Local proof before commit

Run `npm run test:enforcement-owned-process-time-limit-proof` before committing.
It records the ignored local artifact at
`test-results/v0-8-owned-process-time-limit-proof/proof.json`.

The proof runs a real owned Windows PowerShell child process, terminates it
through the production adapter, and checks the typed terminated/audit-safe
result. It also runs the app time-limit and persisted timer lifecycle tests.

## Independent CI rerun

`CI Rust Agent Core` reruns the exact proof runner in its
`Proof: Windows Owned Process Adapter` job on `windows-latest` after push.
The existing Ubuntu core and service jobs continue to cover cross-platform
contract and persistence behavior.

## Non-claims

- Broad installed-app blocking remains manual-required.
- Process termination has no relaunch rollback; its result is explicitly
  `rollback-not-required` rather than a false reversible-control claim.
- Browser, network, screen, AI, mobile, and non-Windows enforcement are not
  covered by this proof.
