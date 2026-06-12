<!-- agent-capsule -->

> Agent Capsule
> Doc: Static Analysis And Security Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

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
- Parent-owned storage connectors.
- Stateless report compilation.
- Pairing.
- Device identity.
- Installer/update logic.
- Secrets, signing, tokens, credentials, or provider webhooks.
- Enforcement or blocking behavior.
- Data export, sync, deletion, or retention.
- Any feature that could move child activity data into Ocentra-hosted systems.

## Done Signal

The feature has no unresolved security/static-analysis findings, and any accepted residual risk is documented with scope, reason, and follow-up owner.
