# V0.8 Enforcement Adapters Expectations

This is the milestone-specific expectation file for V0.8 in `docs/product-roadmap.md`.

Supporting expectation files: [enforcement](enforcement.md), [policy](policy.md), [evidence storage](evidence-storage.md), [platforms](platforms.md), [platform deliverables](platform-deliverables.md), and [static analysis and security](static-analysis-security.md).

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
