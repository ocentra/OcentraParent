# Required Test Assertion Matrix

Status: spec-complete / implementation-present / proof-pending.

Purpose: define the exact Cloudflare test assertions the next execution agent
must implement. This is the source of truth for test scope. A future worker may
add coverage, but may not silently remove, merge away, or reinterpret the cases
listed here.

## Global rules

- Every listed test file is mandatory.
- Every assertion ID below must be either proven by real command output and
  proof artifacts, or recorded as blocked with the exact runtime or dependency
  reason.
- Placeholder files do not satisfy this matrix.
- One broad umbrella test does not replace multiple required files unless this
  document is updated first.
- Spec completeness and runtime readiness are different states:
  - this matrix can be complete while `infra/cloudflare/` already contains real
    runtime and test surfaces but proof artifacts remain incomplete;
  - payment remains blocked until runtime proof exists under WP12.

## Unit suite

### `tests/unit/route-manifest.test.ts`

- `UT-ROUTE-01`: `/health` exists in the manifest and is `public`.
- `UT-ROUTE-02`: `/public/pricing` exists in the manifest and is `public`.
- `UT-ROUTE-03`: `/auth/billing/status`, `/auth/billing/checkout`,
  `/auth/billing/portal`, `/auth/billing/invoices`,
  `/auth/billing/change-plan`, `/auth/billing/cancel`,
  `/auth/billing/referrals`, `/auth/billing/referral-invite`, and
  `/auth/billing/manual-invoice` each exist and are not `public`.
- `UT-ROUTE-04`: `/auth/billing/entitlement-snapshot` and
  `/auth/billing/license-check` each exist and require
  `trusted-parent-device-required` or an explicit manual-required blocker state.
- `UT-ROUTE-05`: `/webhooks/stripe`, `/webhooks/razorpay`,
  `/webhooks/paypal`, `/webhooks/apple`, and `/webhooks/google` each exist and
  require `provider-webhook-signature-required`.
- `UT-ROUTE-06`: `/admin/billing/accounts`, `/admin/billing/invoices`,
  `/admin/billing/refunds`, `/admin/billing/disputes`,
  `/admin/billing/referrals`, `/admin/billing/reconciliation`, and
  `/admin/billing/audit` each exist and require `admin-required` or
  `support-required`.
- `UT-ROUTE-07`: no `/auth/*`, `/admin/*`, or `/webhooks/*` route is marked
  `public`.
- `UT-ROUTE-08`: every manifest entry declares `path`, `method`, `auth state`,
  `handler key`, `request model`, `response model`, `audit event`, and `proof ID
  family`.
- `UT-ROUTE-09`: duplicate `method + path` manifest entries are rejected.
- `UT-ROUTE-10`: ad hoc billing routes outside the declared route groups are
  rejected until the route docs are updated first.

### `tests/unit/auth-boundary.test.ts`

- `UT-AUTH-01`: `public` routes do not require parent, support, admin, or
  provider verification.
- `UT-AUTH-02`: `parent-session-required` routes reject callers with no parent
  session.
- `UT-AUTH-03`: `trusted-parent-device-required` routes reject callers that
  have a parent session but no trusted-device proof.
- `UT-AUTH-04`: `admin-required` routes reject callers that only satisfy parent
  session requirements.
- `UT-AUTH-05`: `support-required` routes reject callers with neither support
  nor admin authority.
- `UT-AUTH-06`: `provider-webhook-signature-required` routes reject missing,
  malformed, invalid, or wrong-provider signatures.
- `UT-AUTH-07`: `internal-queue-only` routes reject non-queue and non-cron
  callers.
- `UT-AUTH-08`: stronger auth states never silently downgrade to weaker states
  by fallback or default behavior.
- `UT-AUTH-09`: admin and support rejections preserve audit-required markers and
  do not leak raw auth internals in the error path.

### `tests/unit/env-bindings.test.ts`

- `UT-ENV-01`: missing `ENVIRONMENT` fails validation.
- `UT-ENV-02`: missing `APP_ORIGIN` fails validation.
- `UT-ENV-03`: missing `CORS_ALLOWED_ORIGINS` fails validation.
- `UT-ENV-04`: missing `ENTITLEMENT_SIGNING_KEY_REF` fails validation.
- `UT-ENV-05`: required bindings are present for `BILLING_D1`, `BILLING_DO`,
  `REFERRAL_DO`, `ENTITLEMENT_SNAPSHOT_DO`, `BILLING_RECONCILIATION_QUEUE`,
  `BILLING_DEAD_LETTER_QUEUE`, `BILLING_RATE_LIMIT_KV`, and
  `BILLING_CONFIG_KV`.
- `UT-ENV-06`: optional `BILLING_AUDIT_R2` and `ANALYTICS` bindings remain
  optional and do not silently become required.
- `UT-ENV-07`: server-only secret names remain server-only and do not appear in
  any client-visible env shape.
- `UT-ENV-08`: unknown or misspelled binding keys fail closed instead of being
  ignored as valid runtime state.

### `tests/unit/request-limits.test.ts`

- `UT-LIMIT-01`: requests over the configured body-size limit fail with `413`
  before route dispatch.
- `UT-LIMIT-02`: requests at or under the configured limit reach the next guard
  or dispatch stage.
- `UT-LIMIT-03`: missing or malformed size metadata fails closed when the
  runtime cannot safely determine the body boundary.
- `UT-LIMIT-04`: oversized rejection responses remain redacted and do not echo
  request bodies, secrets, or stack traces.

### `tests/unit/kill-switch.test.ts`

- `UT-KILL-01`: state-changing authenticated billing routes are blocked when the
  kill switch is enabled.
- `UT-KILL-02`: state-changing admin or support routes are blocked when the
  kill switch is enabled.
- `UT-KILL-03`: provider webhook processing is blocked when the kill switch is
  enabled.
- `UT-KILL-04`: read-only health and public pricing routes remain available
  while the kill switch is enabled.
- `UT-KILL-05`: blocked responses remain explicit and support-safe without
  leaking internal implementation details.

### `tests/unit/redaction.test.ts`

- `UT-REDACT-01`: provider secret names and values are removed from logs and
  error payloads.
- `UT-REDACT-02`: webhook secrets and signing-key references, including
  `ENTITLEMENT_SIGNING_KEY_REF`, are removed from logs and error payloads.
- `UT-REDACT-03`: child-data markers and raw evidence references are removed
  from logs and error payloads.
- `UT-REDACT-04`: recovery-bundle markers, private local paths, and local
  device secret markers are removed from logs and error payloads.
- `UT-REDACT-05`: auth headers, cookies, session tokens, and provider
  credentials are removed from logs and error payloads.
- `UT-REDACT-06`: the remaining payload still keeps route or request identity
  needed for support-safe debugging.

## Integration suite

### `tests/integration/worker-health.test.ts`

- `IT-HEALTH-01`: `GET /health` succeeds through the worker boundary.
- `IT-HEALTH-02`: `GET /health` does not require auth.
- `IT-HEALTH-03`: the health payload does not disclose secret names, storage
  internals, or child-data fields.

### `tests/integration/pricing-public.test.ts`

- `IT-PRICE-01`: `GET /public/pricing` is reachable without private billing
  auth.
- `IT-PRICE-02`: the pricing response does not disclose provider secrets,
  signing refs, or admin-only data.
- `IT-PRICE-03`: pricing remains reachable even when private billing auth is
  absent.

### `tests/integration/billing-status-auth.test.ts`

- `IT-STATUS-01`: `/auth/billing/status` rejects callers without a parent
  session.
- `IT-STATUS-02`: `/auth/billing/status` allows a valid parent session to reach
  the intended route boundary.
- `IT-STATUS-03`: routes that require trusted-parent-device proof return an
  explicit trusted-device-required or manual-required outcome instead of a false
  success path.
- `IT-STATUS-04`: rejection and degraded responses remain redacted and
  client-safe.

### `tests/integration/webhook-signature-rejection.test.ts`

- `IT-WEBHOOK-01`: missing webhook signatures are rejected before processing.
- `IT-WEBHOOK-02`: invalid webhook signatures are rejected before processing.
- `IT-WEBHOOK-03`: malformed webhook payloads fail closed before business logic
  can run.
- `IT-WEBHOOK-04`: unsupported or mismatched provider routes fail closed.

### `tests/integration/admin-auth-rejection.test.ts`

- `IT-ADMIN-01`: admin routes reject callers with no auth.
- `IT-ADMIN-02`: admin routes reject callers that only satisfy
  `parent-session-required`.
- `IT-ADMIN-03`: support-required routes reject callers without support or admin
  authority.
- `IT-ADMIN-04`: admin or support rejection payloads remain redacted and carry
  audit-safe status only.

## E2E suite

### `tests/e2e/portal-to-worker-billing-status.test.ts`

- `E2E-PORTAL-01`: the first consumer smoke targets billing-status handoff only;
  it does not silently expand into checkout, admin, or provider flows.
- `E2E-PORTAL-02`: the portal-visible result is one of ready, degraded, stale,
  offline, or manual-required, not a raw worker error.
- `E2E-PORTAL-03`: the portal-visible payload remains redacted and contains no
  provider secrets, child data, or raw backend diagnostics.
- `E2E-PORTAL-04`: missing auth produces a controlled consumer-safe failure
  state.

## Contract suite

### `tests/contract/billing-api-contract.test.ts`

- `CT-CONTRACT-01`: every route in `ROUTE_MANIFEST_MODEL.md` maps to an
  explicit request and response model reference.
- `CT-CONTRACT-02`: billing status, checkout, portal, invoices, change-plan,
  cancel, referrals, referral-invite, entitlement snapshot, license check, and
  manual invoice routes keep Cloudflare as a transport boundary, not a provider
  SDK leak boundary.
- `CT-CONTRACT-03`: admin and support routes keep support-safe contract shapes
  and do not expose provider-secret fields or child-data custody.
- `CT-CONTRACT-04`: webhook contracts remain provider-input boundaries only and
  do not become portal or child-device payload shapes.
- `CT-CONTRACT-05`: manifest entries and contract docs agree on auth state and
  proof-family ownership.

## Security suite

### `tests/security/no-provider-secrets-in-client.test.ts`

- `SEC-SECRETS-01`: no provider secret name or value appears in public route
  payloads.
- `SEC-SECRETS-02`: no provider secret name or value appears in authenticated
  parent-visible payloads.
- `SEC-SECRETS-03`: no provider secret name, signing-key ref, or raw provider
  credential appears in admin or support client-visible payloads.
- `SEC-SECRETS-04`: no child-data marker, raw evidence reference, or support
  bundle secret marker appears in client-visible payloads.

### `tests/security/cors-origin-rejection.test.ts`

- `SEC-CORS-01`: requests from disallowed origins are rejected before protected
  work runs.
- `SEC-CORS-02`: requests from allowed origins reach the next guard.
- `SEC-CORS-03`: empty, wildcard, or malformed allow-list configuration fails
  closed or manual-required; it does not silently allow all origins.

### `tests/security/request-smuggling.test.ts`

- `SEC-SMUGGLE-01`: conflicting or duplicate body-length metadata is rejected.
- `SEC-SMUGGLE-02`: malformed request-line or header-normalization attempts are
  rejected.
- `SEC-SMUGGLE-03`: header injection or newline-smuggling attempts are rejected.
- `SEC-SMUGGLE-04`: malformed envelopes fail closed without leaking stack traces
  or secret-bearing diagnostics.

### `tests/security/redaction.test.ts`

- `SEC-REDACT-01`: security-facing error paths remove provider secrets, signing
  refs, child-data markers, recovery-bundle markers, and local device secret
  markers.
- `SEC-REDACT-02`: failure payloads preserve request correlation only through
  support-safe identifiers.
- `SEC-REDACT-03`: admin, support, webhook, and queue rejection paths remain
  redacted at the boundary level.

## Property suite

### `tests/property/route-auth-state.property.test.ts`

- `PROP-ROUTE-01`: every manifest entry has exactly one auth state.
- `PROP-ROUTE-02`: no `/auth/*`, `/admin/*`, or `/webhooks/*` route can be
  generated as `public`.
- `PROP-ROUTE-03`: `trusted-parent-device-required` implies
  `parent-session-required`.
- `PROP-ROUTE-04`: `internal-queue-only` routes are unreachable from public
  consumer route generation.
- `PROP-ROUTE-05`: route manifests cannot omit audit-event ownership for private
  or privileged routes.

### `tests/property/billing-idempotency.property.test.ts`

- `PROP-IDEMP-01`: duplicate webhook event IDs do not create duplicate ledger
  transitions.
- `PROP-IDEMP-02`: queue retry or reconciliation replay for the same event ID
  does not create divergent Durable Object or D1 state.
- `PROP-IDEMP-03`: out-of-order duplicate deliveries for the same idempotency
  key remain stable.
- `PROP-IDEMP-04`: conflicting event IDs or account bindings fail closed or
  manual-review; they do not create silent double-success behavior.

## Fuzz suite

### `tests/fuzz/provider-webhook-payload.fuzz.test.ts`

- `FUZZ-WEBHOOK-01`: truncated JSON, binary junk, or random payload bodies fail
  closed.
- `FUZZ-WEBHOOK-02`: oversize payloads fail before processing.
- `FUZZ-WEBHOOK-03`: missing or random provider headers fail closed.
- `FUZZ-WEBHOOK-04`: deeply nested or malformed provider payloads do not leak
  stack traces, secrets, or raw child-data markers.
- `FUZZ-WEBHOOK-05`: wrong-provider route and payload combinations fail closed.

## Observability coverage carried by the existing suites

- `OBS-01`: redacted structured logs preserve route identity, request
  correlation, and outcome class without raw payload disclosure.
  Proof source: `UT-REDACT-06`, `SEC-REDACT-02`.
- `OBS-02`: privileged rejections preserve audit-safe status only.
  Proof source: `UT-AUTH-09`, `IT-ADMIN-04`, `SEC-REDACT-03`.
- `OBS-03`: manual-required, degraded, stale, and offline states remain explicit
  in consumer-visible and proof-visible artifacts.
  Proof source: `IT-STATUS-03`, `E2E-PORTAL-02`, WP08 and WP10 proof artifacts.
