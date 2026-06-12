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

Failure: no DONE when only the happy path is shown, when registration is treated as marketing contact capture, or when setup is claimed without account, install, permission, pairing, and readiness evidence.
