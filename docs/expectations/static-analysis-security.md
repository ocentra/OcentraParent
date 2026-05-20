# Static Analysis And Security Expectations

The repo should treat static security findings as blockers unless proven irrelevant.

## Current Required Gates

- Secret scan must pass.
- Dependency policy must pass.
- SBOM generation must pass when dependencies change.
- Rust clippy must pass with warnings denied.
- TypeScript lint and type-check must pass.
- Source-shape guard must pass.
- Test-double guard must pass.
- Rust string-boundary guard must pass.
- App string-literal guard must pass.

## Future CodeQL Expectation

- If CodeQL is added, new CodeQL alerts are merge blockers.
- A CodeQL alert may be dismissed only with a documented reason and a narrow code reference.
- Do not suppress CodeQL findings globally.
- Do not hide security findings behind generated code or broad ignore paths unless the generated boundary is documented and separately validated.

## Security Review Trigger

Treat these changes as security-sensitive:

- LAN exposure.
- Cloud relay.
- Pairing.
- Device identity.
- Installer/update logic.
- Secrets, signing, tokens, credentials, or provider webhooks.
- Enforcement or blocking behavior.
- Data export, sync, deletion, or retention.

## Done Signal

The feature has no unresolved security/static-analysis findings, and any accepted residual risk is documented with scope, reason, and follow-up owner.
