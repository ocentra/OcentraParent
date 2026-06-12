<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: TEST_PROOF_EXPECTATIONS
> Kind: plan-local test/proof matrix.
> Read when: After one workpack is selected.
> Stop rule: Use this to choose proof; do not run broad validation by habit.

<!-- /agent-capsule -->

# Test and Proof Expectations

These are expected proof intents, not implementation recipes.

| Risk surface                 | Expected proof                                                                                                                                |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Public website data boundary | Static/content review proving no child activity collection, no hidden tracking claim, consent wording, and route-level registration boundary. |
| Registration/login handoff   | Auth route proof, account-state proof, failed/expired/replayed invite proof, and cross-family rejection proof.                                |
| Parent install journey       | Platform matrix, signed/unsigned/manual-required state, download integrity artifact, update-channel handoff.                                  |
| Child install journey        | Permission matrix, unsupported-platform state, child disclosure state, rollback/uninstall/tamper handoff.                                     |
| Pairing readiness            | Pairing success, stale code, revoked code, wrong household, offline child, permission missing, and recovery proof.                            |
| UI readiness                 | Screenshots or rendered artifacts for empty/error/stale/degraded/success states.                                                              |
| Observability                | Redacted logs/metrics/traces for setup progress, failure reason, and recovery path.                                                           |
| PR gate                      | Workpack updates, route/index sync, proof paths, skipped-risk notes, and remaining gaps.                                                      |

## Where tests should live

- Route setup/install test intent into setup-install package/domain package tests until those boundaries are instantiated.
- Maintain end-to-end proof artifacts for public-site, onboarding, pairing, and installer lifecycle in plan-linked proof folders.
- Prefer platform and real integration checks for device install/pairing over fixture-only stories.

## Expected test/proof inventory

- `setup.public-site.data-boundary`: static content and site behavior prove no child telemetry collection beyond disclosed boundaries.
- `setup.account-handoff.authn-authz`: account creation and pairing handoff reject reused/expired/replayed invite claims.
- `setup.installer.platform-matrix`: install, manual-required, tamper/update-channel, and uninstall/rollback behaviors are covered by OS matrix.
- `setup.permission.device-pairing`: permissions, unsupported platform, and offline readiness are explicit negatives.
- `setup.observability.recovery`: retry/backoff, error budget, alerts, and support diagnostics are captured with redaction.

## Failure conditions

- No setup/install DONE/PR_READY if public-site promises are not tied to explicit boundary tests.
- No setup/install DONE/PR_READY if account handoff and install/pairing lifecycle claims rely on positive-path tests only.
- No setup/install DONE/PR_READY unless recovery and unsupported-platform states are documented and evidenced.
