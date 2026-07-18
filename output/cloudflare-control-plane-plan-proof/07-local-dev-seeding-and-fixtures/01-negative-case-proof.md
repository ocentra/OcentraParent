# WP07 Negative Case Proof

plan: cloudflare-control-plane-plan
workpack: WP07 local dev seeding and fixtures
owner: local-dev
environment: local
run_id: cloudflare-wp07-20260718-a259534c2
correlation_id: cloudflare-wp07-local-dev-correlation
result: pass

## Missing command environment

The focused integration test runs the workflow with an empty `PATH`. The entrypoint preflight remains ready because it uses the current Node executable and checked-in sidecar, while all four command-backed seed families become `blocked`. The aggregate seed state is asserted as `blocked`, and every blocked family must carry a non-empty `missing-runtime-dependency` diagnostic. This prevents all-blocked seed commands from satisfying the runnable assertion.

## Missing generated sidecar

The test supplies an exact missing generated-contract path and asserts the fail-closed `missing-runtime-dependency` blocker, including the canonical repo-relative path. The default sidecar lookup is also exercised after changing the process working directory, proving module-relative rather than caller-CWD resolution.

## Import versus runtime boot

The standalone report can reach `preflightStatus = ready` only after the root command, generated sidecar, and Worker import checks pass. It still reports `runtimeBootStatus = unproven` because it does not start Wrangler or issue a health request. A separate 10/10 real-runtime integration suite carries the bounded local boot evidence without changing the standalone report.

## Redaction

The persisted proof rows contain only categorical status, counts, owner/boundary fields, and no-claim metadata. The focused test asserts that representative token, webhook-secret, and child-name sentinel values are absent from the serialized NDJSON rows.

no_claim: These negative cases do not prove production deployment, provider custody, account/device authority, payment handoff, or final production seed data.
