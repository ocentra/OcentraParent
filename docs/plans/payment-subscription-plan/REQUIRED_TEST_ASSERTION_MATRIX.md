# Required Test Assertion Matrix

Status: spec-complete / execution-pending.

Purpose: define the exact payment-plan assertions the next execution agent must
prove or explicitly block. `PROOF_AND_TEST_INVENTORY.md` owns the harness and
artifact map; this file owns the exact assertion scope.

## Global rules

- Every workpack assertion below is mandatory.
- Every assertion ID must end in one of two states:
  - proven by command output and proof artifacts; or
  - blocked with the exact runtime, provider, legal, or dependency reason.
- Placeholder code, placeholder proof files, or route docs alone do not satisfy
  these assertions.
- Spec completeness and runtime readiness are different states:
  - this matrix can be complete while Cloudflare handoff, provider setup, legal
    closure, or runtime code remain open;
  - payment runtime remains blocked until the required proofs exist.

## WP00 Cloudflare control-plane handoff

- `payment-route.cloudflare-plan-exists`: prove the shared
  `cloudflare-control-plane-plan` exists and remains the owner of
  `infra/cloudflare/`.
- `payment-route.cloudflare-module-spec-exists`: prove the shared module spec,
  auth model, route manifest, and storage model exist and are consumed here
  rather than redefined.
- `payment-route.cloudflare-auth-boundary-consumed`: prove payment consumes the
  shared auth states and keeps unresolved provider auth as an explicit blocker.
- `payment-route.cloudflare-route-manifest-consumed`: prove payment billing,
  webhook, and admin route groups come from the shared route manifest.
- `payment-route.cloudflare-test-shape-consumed`: prove payment consumes the
  shared Cloudflare test-runner shape and no parallel worker-test contract is
  invented here.
- `payment-route.cloudflare-portal-smoke-blocker-visible`: prove the shared
  portal-to-worker smoke state or exact blocker is visible to payment.
- `payment-route.payment-remains-blocked-without-handoff`: prove payment runtime
  slices remain blocked until the handoff proof is explicit.

## WP01 Product pricing entitlement

- `payment-pricing.free-starter-bundle`: prove the free starter bundle includes
  one parent portal and one child-device entitlement.
- `payment-pricing.base-one-parent-one-child`: prove the base product model is
  one parent plus one child device, not an open-ended household grant.
- `payment-pricing.paid-extra-child-device`: prove paid expansion is expressed
  as an extra child-device seat, not as provider-native truth.
- `payment-pricing.extra-parent-slot`: prove extra parent access is modeled
  separately from child-device seats.
- `payment-pricing.effective-child-device-limit`: prove the effective device
  limit is derived from app-owned ledger state, not inferred from checkout
  redirect success.
- `payment-pricing.over-limit-grace`: prove over-limit states enter a visible
  grace or restricted path instead of silently deleting access history.
- `payment-pricing.safety-critical-grace`: prove billing degradation preserves
  documented safety-critical behavior rather than shutting it off implicitly.
- `payment-pricing.rejected-game-economy-model`: prove game-only economy models,
  currencies, or marketplace semantics remain rejected.

## WP02 Checkout billing portal

- `payment-checkout.cloudflare-billing-api-required`: prove hosted checkout and
  portal creation require the shared Cloudflare billing boundary.
- `payment-checkout.hosted-checkout-created`: prove hosted checkout is the
  default browser-facing billing entrypoint.
- `payment-checkout.billing-portal-created`: prove hosted billing portal is the
  default self-service management path.
- `payment-checkout.auth-required`: prove checkout and portal creation reject
  callers without parent auth.
- `payment-checkout.household-role-required`: prove checkout and portal flows
  require the correct household role and do not accept arbitrary family members.
- `payment-checkout.invalid-product-rejected`: prove invalid or unknown billing
  products fail closed.
- `payment-checkout.redirect-allowlist`: prove success and cancel redirects are
  constrained to an allow-list.
- `payment-checkout.origin-csrf-negative`: prove origin and CSRF failures are
  rejected before session creation.
- `payment-checkout.bot-abuse-gate`: prove repeated abusive session creation
  attempts are rate-limited or abuse-gated.
- `payment-checkout.no-desktop-secrets`: prove desktop or browser surfaces never
  receive provider secrets.
- `payment-checkout.no-client-secret-exposure`: prove client-visible payloads do
  not contain secret session material or webhook credentials.
- `payment-checkout.return-success-not-entitlement`: prove a provider success
  redirect is not treated as entitlement truth without ledger confirmation.
- `payment-checkout.cancel-state`: prove cancel-return states remain explicit
  and do not masquerade as paid success.

## WP03 Subscription webhook lifecycle

- `payment-webhook.stripe-signature-valid`: prove valid Stripe signatures are a
  required ingress boundary.
- `payment-webhook.stripe-signature-invalid`: prove invalid Stripe signatures
  fail closed before lifecycle processing.
- `payment-webhook.razorpay-signature-valid`: prove Razorpay ingress requires
  the equivalent verified boundary.
- `payment-webhook.paypal-webhook-verified`: prove PayPal webhook verification
  is explicit and not hand-waved.
- `payment-webhook.duplicate-event-idempotent`: prove duplicate event IDs do not
  create duplicate grants or ledger transitions.
- `payment-webhook.replayed-event-rejected`: prove replay attempts are rejected
  or neutralized.
- `payment-webhook.out-of-order-event-safe`: prove out-of-order deliveries do
  not corrupt the app-owned lifecycle state.
- `payment-webhook.unknown-event-safe`: prove unknown event types fail safely.
- `payment-webhook.retry-no-double-grant`: prove retry paths cannot silently
  double-grant entitlement.
- `payment-webhook.dead-letter-manual-required`: prove dead-letter states remain
  visible and manual-required when automatic recovery ends.
- `payment-webhook.reconciliation-repairs-drift`: prove reconciliation exists to
  repair provider-versus-ledger drift.
- `payment-webhook.test-live-separated`: prove test and live traffic remain
  separated in lifecycle handling and proof.

## WP04 Entitlement delivery gates

- `payment-entitlement.billing-ledger-source`: prove billing ledger rows are the
  root source for paid access transitions.
- `payment-entitlement.referral-ledger-source`: prove referral credits and
  losses are captured in a separate app-owned ledger.
- `payment-entitlement.entitlement-ledger-source`: prove entitlement snapshots
  derive from app-owned entitlement state rather than provider payloads.
- `payment-entitlement.signed-snapshot-issued`: prove signed snapshots are
  derived artifacts, not the root of trust.
- `payment-entitlement.snapshot-signature-invalid-rejected`: prove invalid
  snapshot signatures fail closed.
- `payment-entitlement.local-device-trust-required`: prove sensitive
  entitlement consumption requires device-trust proof.
- `payment-entitlement.wrong-household-rejected`: prove wrong-household
  snapshots or requests are rejected.
- `payment-entitlement.wrong-device-rejected`: prove wrong-device snapshots or
  requests are rejected.
- `payment-entitlement.offline-stale-degraded`: prove stale or offline state
  degrades explicitly rather than silently disappearing.
- `payment-entitlement.grace-period`: prove grace behavior is explicit and tied
  back to ledger state.
- `payment-entitlement.cancel-revokes-paid-feature`: prove cancellations revoke
  paid features through ledger-driven transitions.
- `payment-entitlement.referral-loss-revokes-earned-feature`: prove referral
  loss recalculates earned capacity through the entitlement ledger.
- `payment-entitlement.safety-feature-not-silently-disabled`: prove billing
  degradation does not silently disable safety-critical behavior.

## WP05 Invoice tax refund dispute

- `payment-lifecycle.invoice-visible`: prove invoice visibility is explicit on
  the allowed parent or support surfaces.
- `payment-lifecycle.receipt-visible`: prove receipt visibility is explicit and
  separate from provider raw payloads.
- `payment-lifecycle.tax-mode-decision`: prove tax mode and manual-required tax
  decisions are explicit per region.
- `payment-lifecycle.refund-state`: prove full refund lifecycle states are
  explicit and auditable.
- `payment-lifecycle.partial-refund-state`: prove partial refunds do not get
  collapsed into full-refund semantics.
- `payment-lifecycle.refund-failed-state`: prove failed refunds remain explicit
  and auditable.
- `payment-lifecycle.dispute-opened`: prove disputes open a visible and
  auditable state.
- `payment-lifecycle.dispute-won`: prove dispute resolution can restore or
  preserve the correct entitlement path.
- `payment-lifecycle.dispute-lost`: prove lost disputes can revoke or restrict
  paid access through ledger transitions.
- `payment-lifecycle.chargeback-state`: prove chargeback states are distinct
  from ordinary refunds.
- `payment-lifecycle.failed-renewal-grace`: prove failed renewals enter a
  documented grace or restricted mode.
- `payment-lifecycle.cancel-immediate`: prove immediate cancellation semantics
  are explicit and do not rely on redirect success alone.
- `payment-lifecycle.cancel-period-end`: prove period-end cancellation remains a
  distinct lifecycle state.
- `payment-lifecycle.resume-after-past-due`: prove resumed access after past-due
  transitions is explicit and ledger-backed.
- `payment-lifecycle.support-admin-audited`: prove support/admin interventions
  on lifecycle state require audit traces.
- `payment-lifecycle.no-data-delete-on-refund`: prove refunds or disputes do not
  imply deletion of audit-critical records or child-safety history.

## WP06 Security privacy observability

- `payment-security.provider-metadata-allow-deny`: prove provider metadata has
  an explicit allow-list and deny-list.
- `payment-security.no-child-data-metadata`: prove child names, activity,
  screenshots, policy details, and private telemetry never enter provider
  metadata.
- `payment-security.secret-scan`: prove provider secrets, webhook secrets, and
  billing tokens are treated as server-only.
- `payment-security.webhook-smuggling-negative`: prove malformed or smuggled
  webhook envelopes fail closed.
- `payment-security.webhook-replay`: prove replay attempts remain visible and do
  not create double grants.
- `payment-security.rate-limit`: prove repeated checkout, portal, or webhook
  abuse paths are rate-limited.
- `payment-security.bot-abuse-gate`: prove abusive automated checkout or portal
  attempts are blocked or throttled.
- `payment-security.open-redirect-negative`: prove open redirect paths are
  rejected.
- `payment-security.redacted-logs`: prove logs, metrics, and analytics remain
  redacted by default.
- `payment-security.support-view-minimized`: prove support views only expose the
  minimum billing-safe fields.
- `payment-security.pci-hosted-checkout-boundary`: prove hosted checkout keeps
  card capture out of the product runtime boundary.
- `payment-security.referral-abuse-signals`: prove referral abuse detection
  remains explicit and auditable.
- `payment-security.admin-audit-required`: prove admin or support actions cannot
  bypass audit capture.

## WP07 Rollout proof and route gate

- `payment-route.plan-sync`: prove route docs, workpack docs, and the live queue
  remain synchronized.
- `payment-route.workpack-proof-manifest`: prove proof bundles live outside the
  plan folder and are referenced consistently.
- `payment-route.validation-log`: prove validation command families and blockers
  are recorded explicitly.
- `payment-route.negative-gate`: prove at least one negative case exists for the
  selected slice.
- `payment-route.rollback-path`: prove rollback or teardown expectations are
  explicit before route closure is claimed.

## WP08 Provider adapter portability

- `payment-provider.adapter-interface`: prove provider adapters implement one
  explicit normalization boundary.
- `payment-provider.stripe-adapter-contract`: prove the Stripe adapter stays
  behind that normalization boundary.
- `payment-provider.razorpay-adapter-contract`: prove Razorpay stays behind the
  same boundary and does not mutate product authority directly.
- `payment-provider.paypal-adapter-contract`: prove PayPal stays behind the same
  boundary.
- `payment-provider.apple-store-adapter-contract`: prove Apple billing is a
  channel adapter, not root truth.
- `payment-provider.google-play-adapter-contract`: prove Google billing is a
  channel adapter, not root truth.
- `payment-provider.manual-invoice-adapter-contract`: prove manual invoice
  billing stays explicit and auditable.
- `payment-provider.normalized-event-contract`: prove provider events normalize
  into one app-owned event contract.
- `payment-provider.provider-lock-escape`: prove the ledger can survive provider
  changes without rewriting product truth.
- `payment-provider.no-direct-product-provider-reads`: prove product surfaces do
  not read provider raw state directly to decide access.

## WP09 Regional payment rollout

- `payment-region.canada-us`: prove the default Stripe-led rollout assumptions
  for Canada and the US are explicit.
- `payment-region.india`: prove the Razorpay path and its setup assumptions are
  explicit.
- `payment-region.pakistan`: prove Pakistan remains supported only through an
  explicit manual-required or limited path if direct subscription support is not
  closed.
- `payment-region.china`: prove China wallet or manual-required assumptions are
  explicit and do not pretend Stripe parity.
- `payment-region.uae-dubai`: prove UAE or Dubai provider assumptions are
  explicit.
- `payment-region.eu-uk`: prove EU and UK rollout assumptions are explicit and
  tax-aware.
- `payment-region.southeast-asia`: prove Southeast Asia rollout assumptions and
  gaps are explicit.
- `payment-region.manual-enterprise`: prove manual or enterprise billing remains
  explicit where provider coverage is absent.
- `payment-region.local-methods`: prove local-method coverage is modeled per
  region rather than assumed globally.
- `payment-region.subscription-support`: prove each region explicitly states
  whether subscription support is available.
- `payment-region.manual-required-gaps`: prove unresolved regional gaps remain
  manual-required rather than silently marked done.

## WP10 Referral growth entitlement

- `payment-referral.invite-created`: prove referral invite creation is distinct
  from household invite creation.
- `payment-referral.invite-opened`: prove invite open state is tracked without
  granting credit prematurely.
- `payment-referral.signup-started`: prove signup start alone does not qualify a
  referral.
- `payment-referral.account-created`: prove account creation alone does not
  qualify a referral.
- `payment-referral.household-created`: prove household creation alone does not
  qualify a referral unless the qualification rules say so.
- `payment-referral.setup-activated`: prove setup activation is an explicit step
  in qualification.
- `payment-referral.qualified-credit-granted`: prove earned credits are granted
  only after qualification is complete.
- `payment-referral.active-referred-parent-required`: prove a referred parent
  must remain active or qualified for the credit to persist.
- `payment-referral.lost-referral-credit-removed`: prove lost qualification can
  remove earned expansion through ledger recalculation.
- `payment-referral.referral-grace`: prove credit loss or expiration enters an
  explicit grace or restricted path.
- `payment-referral.self-referral-rejected`: prove self-referrals fail closed.
- `payment-referral.same-household-rejected`: prove same-household referrals
  fail closed.
- `payment-referral.same-device-farm-rejected`: prove device-farm abuse signals
  are rejected or manual-reviewed.
- `payment-referral.same-payment-method-manual-review`: prove same-payment-method
  collisions trigger explicit review behavior.
- `payment-referral.fraud-review`: prove fraud review states are explicit and
  auditable.
- `payment-referral.entitlement-recalculated`: prove referral changes recalc
  entitlement through the app-owned ledger.
- `payment-referral.over-limit-grace-visible`: prove lost referral capacity and
  over-limit grace stay visible to the parent.
- `payment-referral.no-data-delete-on-lost-referral`: prove lost credits do not
  imply deletion of audit or safety-critical history.

## WP11 Parent website billing dashboard

- `payment-dashboard.parent-account-visible`: prove parent-authenticated users
  can see their own billing account state.
- `payment-dashboard.current-plan-visible`: prove the current plan and status
  are visible without implying provider raw truth is authoritative.
- `payment-dashboard.child-device-usage-visible`: prove seat usage is visible as
  billing-safe counts, not as child-private data.
- `payment-dashboard.referral-credit-visible`: prove referral credit state is
  visible when applicable.
- `payment-dashboard.paid-seat-visible`: prove paid seat expansion state is
  visible.
- `payment-dashboard.invoice-visible`: prove invoice visibility is explicit on
  the allowed parent surface.
- `payment-dashboard.change-plan-visible`: prove change-plan actions remain
  visible and routed through the hosted billing path.
- `payment-dashboard.cancel-visible`: prove cancellation state or action is
  visible without implying immediate revocation unless ledger state says so.
- `payment-dashboard.billing-portal-link`: prove the portal handoff is visible.
- `payment-dashboard.license-snapshot-visible`: prove the parent can see
  billing-safe snapshot or license state without raw device secrets.
- `payment-dashboard.wrong-household-denied`: prove wrong-household access is
  rejected.
- `payment-dashboard.no-child-private-data`: prove child-private data and
  support-only fields remain hidden.
- `payment-dashboard.targeted-parent-proof-file`: prove execution must create or
  explicitly block `packages/parent-domain/tests/unit/billing-entitlement-proof.test.ts`;
  billing-domain tests are not a substitute for parent-surface proof.

## WP12 Support admin billing ops

- `payment-admin.billing-account-search`: prove billing account search is an
  explicit admin or support capability.
- `payment-admin.invoice-search`: prove invoice search is explicit and
  redaction-safe.
- `payment-admin.refund-action-audited`: prove refund actions require audit
  capture.
- `payment-admin.dispute-state-visible`: prove dispute states remain visible to
  the right support or admin roles.
- `payment-admin.manual-invoice-state`: prove manual invoice states remain
  explicit and auditable.
- `payment-admin.referral-abuse-visible`: prove referral abuse review state is
  visible to the allowed operators only.
- `payment-admin.reconciliation-drift-visible`: prove reconciliation drift and
  retry/dead-letter outcomes remain visible to allowed operators.
- `payment-admin.webhook-failure-visible`: prove webhook failure states remain
  visible to support or admin operators.
- `payment-admin.admin-role-required`: prove privileged actions require admin
  authority.
- `payment-admin.support-role-limited`: prove support authority stays narrower
  than admin authority.
- `payment-admin.no-child-private-data`: prove support/admin billing surfaces do
  not expose child-private data.
- `payment-admin.audit-event-required`: prove every privileged action requires
  an audit event.
