# Workpack 05: Relay Security Abuse Controls

Goal: define relay safety, availability, and abuse controls.

Expected shape:

- Relay connections are authenticated, scoped, rate-limited, and observable.
- Partial outage, slow dependency, reconnect storm, and DoS are expected states.
- Diagnostics are redacted and do not store screen content unless explicitly authorized.

Expected proof:

- Rate-limit and brute-force proof.
- Retry storm/backpressure proof.
- Slow relay/partial outage proof.
- Redacted logs/metrics/alerts.
- Token scope/expiry/replay proof.
- Cross-household routing isolation proof.
- No raw screen/input payload retention in diagnostics unless explicitly enabled and governed by data custody.

Failure: remote relay that is treated as a trusted pipe without abuse and privacy proof.

## Decision Tree

| If relay work touches... | Required route                                                       |
| ------------------------ | -------------------------------------------------------------------- |
| Auth/session/token       | account-identity-family-plan plus this workpack                      |
| Screen frames/live view  | remote live view workpack plus data custody for retention boundaries |
| Remote input/control     | remote input authority workpack and enforcement/platform proof       |
| Logs/diagnostics         | data-custody-storage-plan and support/redaction proof                |
| Load/reconnect behavior  | this workpack plus observability proof                               |

## Execution Detail

Minimum context:

- `docs/expectations/cloud.md`
- `docs/expectations/static-analysis-security.md`
- `docs/plans/data-custody-storage-plan/AGENTS.md`

Threat surfaces:

- Unauthorized session creation.
- Token replay.
- Relay resource exhaustion.
- Slow dependency and reconnect storm.
- Cross-household routing bug.
- Frame/content retention leak.
- Support/admin misuse.
- Origin/header/host confusion.
- Open redirect or URL hijack in session links.
- Cache poisoning or stale grant reuse.

Expected controls:

- Authenticated session setup.
- Capability-scoped relay tokens.
- Rate limits and backpressure.
- Connection limits per household/device.
- Redacted diagnostics.
- Alerting on abuse and repeated failures.
- Explicit TTL/expiry and one-session/one-device scoping where required.
- Abuse lockout or progressive delay for repeated failures.
- Partial outage and degraded state surfaced to parent UI.

Expected tests/proof names:

- `relay.rate-limit`
- `relay.retry-storm-backpressure`
- `relay.cross-household-denied`
- `relay.token-replay-denied`
- `relay.redacted-diagnostics`
- `relay.origin-host-redirect-negative`
- `relay.slow-dependency-partial-outage`
- `relay.cache-stale-grant-denied`
- `relay.alerting-metrics-sanity`

Proof artifact expectations:

- Load/spike/partial outage notes when implemented.
- Logs/metrics/alert examples.
- No raw screen payload in relay diagnostics.
- Auth/session/token redaction evidence.
- Explicit remaining DoS/load limits if production-scale proof is not done.

## Failure Conditions

- Do not call relay production-ready without abuse, load, replay, and cross-household proof.
- Do not store screen frames, input stream, or child-private payload in relay diagnostics by default.
- Do not let support/admin bypass parent-visible session grants.
