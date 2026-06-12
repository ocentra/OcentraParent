<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Enforcement Adapters Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.8 Enforcement Adapters Expectations

This is the milestone-specific expectation file for V0.8 in `docs/product-roadmap.md`.

Supporting expectation files: [enforcement](../expectations/enforcement.md), [policy](../expectations/policy.md), [evidence storage](../expectations/evidence-storage.md), [platforms](../expectations/platforms.md), [platform deliverables](../expectations/platform-deliverables.md), and [static analysis and security](../expectations/static-analysis-security.md).

## Outcome

- Enforcement adapters act only from typed, auditable policy decisions after local evidence and policy evaluation are trusted.
- Windows starts with narrow, reversible modes such as process block/terminate, network/domain block, managed-browser-only handling, timeout, or ask-parent.
- Parent-authored rules remain the authority for household actions.
- macOS, Linux, Android, and iOS enforcement are separate platform claims that
  require real OS API, permission, store, and rollback proof before shipping.

## Acceptance

- Enforcement events record policy decision id, evidence refs, adapter result, rollback/unavailable state, and parent override/approval refs when applicable.
- Category labels or AI text alone never cause blocking without a matching parent-authored rule.
- Dev builds remain uninstallable, debuggable, and honest about missing hardening.
- Unsupported enforcement modes return unavailable/degraded/manual-required adapter results and
  never pretend that policy was applied.
- The service distinguishes the narrow proven owned-process and app time-limit
  paths from app block, network/domain block, and managed-browser control paths
  that still require privileged OS, browser-management, or manual Windows proof.
- Parent cancel/override, restart recovery, expiry, unavailable, audit, and
  storage proof must flow through Rust service commands, not portal-local state.

## Validation

- Run `npm run validate`.
- Include adapter integration tests, rollback/unavailable tests, policy handoff tests, and security review evidence.
- Run `node scripts/test/v0-8-production-enforcement-hardening.mjs` for the
  service-path proof that app block, domain block, and managed-browser block
  return typed unavailable/manual-required states instead of broad blocking
  claims.
