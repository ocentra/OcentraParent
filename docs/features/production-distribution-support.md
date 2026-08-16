<!-- agent-capsule -->

> Agent Capsule
> Doc: Production Distribution And Support
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Production Distribution And Support

## Parent Outcome

Parents can install, update, uninstall, subscribe, get support, understand
privacy/data handling, and trust that platform-specific releases are real, not
scaffold artifacts.

## Ocentra Requirement

Production distribution is part of the product. A CI package preview is not a
shipping claim. Signed installers, app stores, update channels, support docs,
privacy/legal docs, billing, and release proof must be explicit.

## Roadmap And Expectations

- Roadmap: V7 subscription and monetization, V8 production hardening.
- Expectations: [release installer](../expectations/release-installer.md),
  [billing](../expectations/billing.md),
  [platform deliverables](../expectations/platform-deliverables.md),
  [static analysis/security](../expectations/static-analysis-security.md),
  [documentation](../expectations/documentation.md).
- Modules: `crates/agent-updater`, `platforms/android`, `platforms/ios`,
  release scripts, root README, public website/account surface.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
production distribution, remote parent access, billing/subscription, and mobile
coverage.

All mature competitors ship through app stores, installers, support surfaces,
billing, privacy documents, and update channels. Ocentra must not look like a
dev-only repo when it claims consumer readiness.

## Current Ocentra State

- Windows MSI/updater scaffolding exists.
- Cross-platform package previews exist as CI mechanics.
- Parent mobile Android/iOS scaffold package previews now exist separately from
  child-agent Android/iOS previews. They remain CI/mechanical proof, not
  signing, store, real-device, controller-authority, or child-agent parity
  proof.
- Parent desktop Tauri package proof exposes built-portal frontend state,
  Rust-service backend kind, package service-manager launch ownership, service
  health endpoint, runtime readiness, fixed port/process ownership,
  connect-or-degrade behavior, route/source/custody labels, support redaction,
  preview/manual-required release states, platform-matrix split rows, and
  blank-window guard state without treating Vite as a packaged backend.
- Windows package lifecycle proof installs/starts the service wrapper and probes
  the Rust service health endpoint; this remains CI/mechanical proof, not signed
  release proof.
- Parent desktop release-support proof now records update/rollback posture,
  signing/store claim boundaries, package-runtime evidence, support diagnostic
  redaction fields, CI artifact expectations, production-support incident
  handoff metadata, parent consent, disclosed support-bundle data classes,
  support-safe diagnostic references, and manual platform proof requirements.
  The typed proof requires built portal dist, Rust-service boundary, package
  service-manager launch ownership, fixed loopback process ownership,
  connect-or-degrade behavior, signed-channel update posture, and explicit
  not-signing/not-production/non-upload non-claims.
- `v8-updater-rollback-runbook-proof` adds explicit updater rollback and
  release-support runbook status rows for scaffold, unsigned-preview,
  signature-required, and production channels. It proves rollback execution is
  still unavailable, rollback failure status remains manual-required, the
  support runbook is preview-only until published, and signed production update
  channel proof is required before any rollback execution claim.
- `support-bundle-redaction-proof` adds logging-domain production-support
  incident rows for parent consent, support bundle readiness, manual review,
  backend-upload manual-required state, billing escalation manual-required
  state, and account lookup manual-required state. It proves support-safe
  release/package/runtime, service, route, capability, degraded-state,
  redaction, manual-proof, incident, billing-status, and account-status
  references while rejecting tokens, child activity, raw URLs, screenshots,
  journals, SQLite snapshots, private paths, command lines, keystrokes,
  clipboard data, message contents, provider secrets, backend upload execution,
  billing provider contact, account lookup execution, remote support sessions,
  and production SLA claims.
- `support-incident-workflow-proof` adds logging-domain production support
  incident workflow rows for parent consent gating, privacy/legal disclosure
  before export, redaction and custody audit review, backend-upload
  manual-required state, billing-escalation manual-required state, and
  account-lookup manual-required state. It proves support-safe incident
  workflow metadata while rejecting tokens, child activity, raw URLs,
  screenshots, journals, SQLite snapshots, private paths, command lines,
  keystrokes, clipboard data, message contents, provider secrets, backend upload
  execution, billing provider contact, account lookup execution, remote support
  sessions, production SLA claims, and Ocentra-hosted child activity custody.
- `production-support-backend-upload-status-proof` adds logging-domain support
  backend upload status rows for parent-initiated and parent-consented queued,
  running, succeeded, failed, manual-required, backend-unavailable, and
  provider-unavailable states. It proves redaction and audit refs, retry-queued
  and retry-exhausted rows, parent/operator abandon refs, manual proof
  requirements, package/runtime refs, and support-safe status payloads while
  rejecting raw child activity custody, provider secrets, remote support
  transcripts, real backend upload execution, account lookup execution, billing
  provider execution, and default Ocentra-hosted family data.
- `production-support-backend-upload-execution-runtime-proof` adds
  logging-domain support backend upload execution/runtime boundary rows for
  parent-consented request recording, redaction preflight readiness,
  dispatch-manual-required, backend-unavailable, provider-unavailable,
  retry-scheduled, and operator-abandoned states. It links to the prior support
  upload status proof with status refs and proves redaction/audit/runtime refs,
  retry/abandon refs, and manual proof requirements while rejecting child
  activity custody, provider secrets, remote support transcripts, real backend
  upload execution, account lookup execution, billing provider contact
  execution, remote support sessions, production SLA, and default
  Ocentra-hosted family data.
- `production-support-backend-upload-custody-audit-proof` adds logging-domain
  support backend upload custody/audit rows for custody boundary recording,
  retention manual-required state, delete request recording, deletion
  manual-required state, and support-safe audit export readiness. It links to
  the prior support upload status and execution/runtime proofs with status and
  runtime refs, proves redaction/custody/retention/delete/manual refs, and keeps
  real backend upload execution, backend payload retention, backend payload
  deletion, raw child activity custody, provider secrets, account lookup,
  billing provider contact, remote support sessions, production SLA, and default
  Ocentra-hosted family data unclaimed.
- `production-support-case-resolution-status-proof` adds logging-domain support
  case resolution/status rows for parent-consented case opened, triage-ready,
  parent-update-ready, escalation manual-required, response manual-required,
  closure-ready, and SLA manual-required states. It links to incident workflow,
  support upload status/execution, and publication workflow refs, proves
  support-safe parent-visible case lifecycle metadata, and keeps real support
  backend upload execution, provider contact, account lookup, billing provider
  contact, remote support sessions, production SLA execution, raw child activity
  custody, provider secrets, remote support transcripts, and default
  Ocentra-hosted family data unclaimed.
- `production-incident-support-status-proof` adds parent-domain production
  incident/support status rows for support incident intake, parent consent,
  privacy/legal disclosure, data export request, delete request, incident
  publication, and case resolution handoff. It proves only source-contract and
  support-safe status metadata while preserving public publication, legal
  execution, support backend upload execution, account lookup, billing provider
  contact, remote support sessions, production SLA, provider secrets, and child
  activity custody as explicit non-claims.
- `production-support-process-runtime-status-proof` adds parent-domain support
  process runtime status rows for support process requested, parent consent
  authorized, privacy/legal queued, redaction review running, backend-upload
  failed, case resolution succeeded, manual-required support process, incident
  runtime requested, incident runtime authorized, incident runtime running,
  incident runtime evidence-ready, and incident runtime manual-required states.
  It proves deterministic support-safe status metadata while preserving real
  incident runtime execution, real backend upload execution, public runtime
  execution, provider execution, production SLA, remote support sessions,
  provider secrets, child activity custody, and default Ocentra-hosted family
  data as explicit non-claims.
- `production-support-account-sla-status-proof` adds parent-domain production
  support account/SLA status rows for account lookup request/result boundaries,
  billing provider contact status, remote support request/session boundaries,
  and production SLA status. It proves only support-safe status metadata and
  manual requirements while keeping real account lookup execution, billing
  provider contact, remote support sessions, production SLA commitments, support
  backend upload execution, `family.ocentra.ca` runtime, provider secrets, and
  child activity custody unclaimed.
- `production-support-legal-provider-readiness-proof` adds parent-domain
  production support legal/provider readiness rows for privacy/legal review,
  data export/delete runtime, provider-secret custody, billing provider contact,
  remote-support legal/session boundaries, and production SLA legal boundary
  status. It proves only source-contract and support-safe metadata while keeping
  real legal disclosure execution, data export/delete runtime, provider secret
  custody, billing provider contact execution, account lookup execution, remote
  support sessions, production SLA commitments, support backend upload
  execution, public runtime execution, and child activity custody unclaimed.
- `production-support-provider-secret-custody-status-proof` adds logging-domain
  provider-secret custody status rows for custody-boundary recorded,
  provider-secret absent, backend secret store manual-required, rotation
  manual-required, revocation manual-required, and support-safe audit export
  readiness. It links legal/provider readiness, billing support, redaction,
  custody audit, rotation, revocation, manual proof, and audit export refs while
  keeping real provider-secret custody, backend secret store execution,
  rotation execution, revocation execution, support backend upload execution,
  account lookup execution, billing provider contact, remote support sessions,
  production SLA, default Ocentra-hosted family data, and child activity custody
  unclaimed.
- `provider-secret-execution-readiness-proof` adds logging-domain
  provider-secret execution readiness rows for execution boundary, backend
  secret-store preflight, rotation preflight, revocation preflight, operator
  approval, manual execution, and support-safe audit export states. It links
  provider-secret custody status, backend secret-store preflight,
  rotation/revocation preflight, operator approval, manual proof, and audit refs
  while keeping backend secret store execution, rotation execution, revocation
  execution, provider-secret delivery, support backend upload execution, account
  lookup, billing provider contact, remote support sessions, production SLA,
  default Ocentra-hosted family data, and child activity custody unclaimed.
- `production-support-provider-secret-rotation-revocation-status-proof` adds
  logging-domain provider-secret rotation and revocation status rows for
  rotation requested, rotation preflight-ready, rotation manual-required,
  revocation requested, revocation preflight-ready, revocation manual-required,
  and audit-export-ready states. It links provider-secret custody status,
  provider-secret execution readiness, backend secret-store preflight, operator
  approval, manual proof, and audit refs while keeping backend secret store
  execution, rotation execution, revocation execution, provider-secret delivery,
  support backend upload execution, account lookup, billing provider contact,
  remote support sessions, production SLA, default Ocentra-hosted family data,
  and child activity custody unclaimed.
- `production-support-backend-provider-runtime-readiness-proof` adds a
  logging-domain readiness boundary that composes support backend upload
  execution runtime, upload custody/audit, provider-secret execution readiness,
  account/SLA, privacy/legal, and case-resolution proof refs. It proves only
  support-safe status metadata for upload runtime linkage, provider-secret
  preflight, billing provider, account lookup, legal disclosure, remote support,
  SLA, and audit export rows while keeping real support backend upload
  execution, provider-secret delivery/custody execution, account lookup,
  billing provider contact, legal disclosure execution, remote support sessions,
  production SLA, default Ocentra-hosted family data, and child activity custody
  unclaimed.
- `production-support-data-export-delete-lifecycle-proof` adds parent-domain
  and logging-domain export/delete runtime lifecycle rows for requested,
  authorized, queued, running, succeeded, failed, and manual-required export and
  delete states. It proves only redaction-safe parent-authorized local runtime
  status metadata and parent-owned local output/delete refs while preserving
  real backend upload execution, public runtime execution, provider execution,
  production SLA, remote support sessions, default Ocentra-hosted family data,
  and child activity custody as explicit non-claims.
- `production-support-delete-executor-proof` adds logging-domain delete
  executor readiness/status rows for local export output, support backend
  payload, status backend payload, public runtime payload, and legal disclosure
  payload boundaries. It proves only source-backed delete-request,
  authorization, redaction/audit, custody, source-proof, and manual-proof refs
  while preserving real delete execution, durable queue execution, payload
  deletion, provider execution, public runtime execution, legal execution,
  backend upload execution, production SLA, default Ocentra-hosted family data,
  and child activity custody as explicit non-claims.
- `production-support-publication-workflow-proof` adds parent-domain source
  contract rows for public privacy policy publication, privacy/legal disclosure
  execution, support runbook publication, support incident status publication,
  support backend upload publication handoff, and public support contact
  publication. It proves the publication workflow remains source-contract
  ready/manual-required while rejecting real public runtime, support backend
  upload execution, account lookup execution, billing provider contact,
  production SLA, legal disclosure execution, remote support sessions, and child
  activity custody.
- `production-support-publication-runtime-readiness-proof` adds source-backed
  readiness rows for the same support/publication runtime handoffs without
  duplicating the publication freshness source-contract proof. It proves the
  current public runtime, support runbook publication runner, incident status
  publication runner, support upload publication runtime, privacy/legal
  publication runtime, and public support contact runtime remain
  source-contract/manual-required/backend-required or legal-review-required,
  while rejecting real public runtime execution, publication runner execution,
  support backend upload execution, account lookup execution, billing provider
  contact, production SLA, legal disclosure execution, remote support sessions,
  and child activity custody.
- `production-support-public-surface-export-closure-proof` closes the package
  export and documentation surface for existing public release status,
  public-status freshness, public-docs freshness, support-publication runtime
  readiness, support-publication status freshness, and public support contact
  status contracts. It proves those source-contract modules are importable
  through `@ocentra-parent/parent-domain` while preserving real public runtime,
  publication runner, status backend, support backend upload, account lookup,
  billing provider contact, legal disclosure, remote support, production SLA,
  provider-secret custody, and child activity custody as non-claims.
- `production-support-publication-execution-status-proof` adds parent-domain
  status rows for support runbook, incident status, public support contact,
  support backend upload, privacy/legal, and account/billing publication
  execution targets across requested, queued, running, succeeded, failed, and
  manual-required lifecycle labels. It proves only the status contract and
  manual proof boundary while rejecting public runtime execution, publication
  runner execution, status backend execution, support backend upload execution,
  account lookup, billing provider contact, production SLA, legal disclosure
  execution, remote support sessions, provider-secret custody, and child
  activity custody.
- `production-support-status-backend-public-runtime-followthrough-proof` adds
  parent-domain follow-through rows for support status public runtime, support
  runbook status backend, incident status backend, public support contact status
  backend, support upload status backend, and account/billing status backend
  targets across requested, queued, running, succeeded, failed, and
  manual-required labels. It proves deterministic handoff/status metadata while
  preserving real public runtime execution, status backend execution, support
  backend upload execution, account lookup, billing provider contact, legal
  disclosure execution, remote support sessions, production SLA,
  provider-secret custody, public runtime payload custody, and child activity
  custody as explicit non-claims.
- `production-support-status-backend-execution-queue-proof` adds parent-domain
  status backend execution queue rows for support runbook, incident, public
  support contact, support upload, privacy/legal, and account/billing targets
  across requested, authorized, queued, running, succeeded, failed,
  manual-required, and backend-unavailable labels. It proves support-safe queue,
  retry, and audit references while preserving real status backend execution,
  public runtime execution, provider execution, support backend upload
  execution, account lookup, billing provider contact, legal disclosure
  execution, remote support sessions, production SLA, provider-secret custody,
  status backend payload custody, and child activity custody as explicit
  non-claims.
- `production-support-status-backend-queue-audit-persistence-proof` adds
  parent-domain status backend queue audit/persistence rows for support runbook,
  incident, public support contact, support upload, privacy/legal, and
  account/billing targets across requested, authorized, queued,
  retry-scheduled, audit-ready, failed, manual-required, and
  backend-unavailable labels. It proves support-safe queue, retry, audit, and
  manual proof references while preserving real status backend execution,
  durable queue storage, retry worker execution, audit persistence, public
  runtime execution, provider execution, support backend upload execution,
  account lookup, billing provider contact, legal disclosure execution, remote
  support sessions, production SLA, provider-secret custody, status backend
  payload custody, and child activity custody as explicit non-claims.
- `production-support-status-backend-dead-letter-proof` adds parent-domain
  status backend dead-letter/manual-triage rows for support runbook, incident,
  public support contact, support upload, privacy/legal, and account/billing
  targets across requested, authorized, dead-lettered, triage-ready,
  retry-blocked, failed, manual-required, and backend-unavailable labels. It
  proves support-safe queue, dead-letter, retry, audit, and manual proof
  references while preserving real status backend execution, durable queue
  storage, retry worker execution, audit persistence, dead-letter payload
  custody, public runtime execution, provider execution, support backend upload
  execution, account lookup, billing provider contact, legal disclosure
  execution, remote support sessions, production SLA, provider-secret custody,
  and child activity custody as explicit non-claims.
- `production-support-status-backend-runtime-execution-proof` adds
  parent-domain status backend runtime execution rows for support runbook,
  incident, public support contact, support upload, privacy/legal, and
  account/billing targets across requested, authorized, queued, running,
  runtime-evidence-ready, audit-ready, failed, manual-required, and
  backend-unavailable labels. It proves support-safe queue, retry, audit,
  dead-letter, runtime evidence, and manual proof references while preserving
  real status backend execution, durable queue storage, retry worker execution,
  audit persistence, dead-letter payload custody, public runtime execution,
  provider execution, support backend upload execution, account lookup, billing
  provider contact, legal disclosure execution, remote support sessions,
  production SLA, provider-secret custody, status backend payload custody, and
  child activity custody as explicit non-claims.
- `production-support-status-backend-payload-custody-proof` adds
  logging-domain status backend payload custody rows for custody boundary,
  retention manual-required, delete request, deletion manual-required,
  audit-export-ready, and backend-unavailable states. It proves only
  support-safe status target, queue, audit, redaction, custody, retention,
  delete, and manual proof references while preserving real status backend
  execution, durable status backend payload storage, payload deletion, retry
  worker execution, audit persistence, public runtime execution, provider
  execution, support backend upload execution, account lookup, billing provider
  contact, legal disclosure execution, remote support sessions, production SLA,
  provider-secret custody, default Ocentra-hosted family data, and child
  activity custody as explicit non-claims.
- `production-support-status-backend-redaction-manifest-proof` extends the
  logging-domain support bundle redaction rows with status backend redaction
  manifest readiness and manual-required states. It proves support-safe status
  target refs, queue refs, audit refs, redaction manifest refs, and manual proof
  refs while preserving real status backend execution, status backend payload
  custody, durable payload storage, payload deletion, retry worker execution,
  audit persistence execution, public runtime execution, support backend upload
  execution, provider execution, account lookup, billing provider contact,
  legal disclosure execution, remote support sessions, production SLA,
  provider-secret custody, and child activity custody as explicit non-claims.
- `production-support-status-backend-runtime-closure-proof` adds a parent-domain
  closure contract that composes the status backend runtime execution,
  queue/audit persistence, dead-letter, payload-custody, redaction-manifest,
  and public-runtime follow-through proof refs. It proves only support-safe
  status labels and source refs while preserving real status backend execution,
  durable queue storage, retry-worker execution, audit persistence, dead-letter
  payload custody, status backend payload custody, redaction manifest execution,
  public runtime execution, provider execution, support backend upload
  execution, account lookup, billing provider contact, legal disclosure
  execution, remote support sessions, production SLA, provider-secret custody,
  and child activity custody as explicit non-claims.
- `production-support-status-backend-durable-queue-runtime-proof` adds a
  parent-domain durable queue runtime boundary/readiness contract for status
  backend queue storage, retry-worker, audit-persistence, dead-letter, runtime
  execution, and runtime closure refs. It proves only support-safe boundary refs
  while preserving real status backend execution, durable queue storage,
  retry-worker execution, audit persistence, dead-letter payload custody, public
  runtime execution, provider execution, support backend upload execution,
  account lookup, billing provider contact, legal disclosure execution, remote
  support sessions, production SLA, provider-secret custody, and child activity
  custody as explicit non-claims.
- `production-support-status-backend-execution-continuation-proof` adds a
  parent-domain execution continuation boundary/readiness contract for the next
  status-backend execution gap. It composes durable queue runtime, runtime
  closure, logging-domain payload-custody, and redaction-manifest refs while
  preserving real status backend execution, durable queue storage, retry-worker
  execution, audit persistence, dead-letter payload custody, status-backend
  payload custody, redaction-manifest execution, public runtime execution,
  provider execution, support backend upload execution, account lookup, billing
  provider contact, legal disclosure execution, remote support sessions,
  production SLA, provider-secret custody, default hosted family data, and child
  activity custody as explicit non-claims.
- `production-support-proof-status-matrix-closure-proof` adds a parent-domain
  proof/status matrix closure row set after PR534. It reconciles existing
  status-backend runtime, public runtime/publication, privacy/legal disclosure,
  provider-secret, export/delete lifecycle, and release-installer support proof
  refs without duplicating the underlying proofs. It preserves real public
  runtime, status backend execution, signing/store proof, updater execution,
  support backend upload execution, account/billing provider execution, legal
  disclosure execution, production SLA, provider-secret custody, and child
  activity custody as explicit non-claims.
- `production-support-privacy-legal-disclosure-status-proof` adds
  logging-domain privacy/legal disclosure status rows for disclosure requested,
  parent-authorized, legal-review queued, legal-review running,
  parent-notification-ready, publication-ready, failed, and manual-required
  states. It proves support-safe status metadata, parent consent refs, privacy
  policy refs, legal review refs, publication/support runbook refs, audit refs,
  failure refs, and manual proof requirements while rejecting legal disclosure
  execution, public runtime execution, support backend upload execution, account
  lookup, billing provider contact, remote support sessions, production SLA,
  provider secrets, remote support transcripts, raw child activity custody, and
  raw support bundle payloads.
- `public-support-contact-status-proof` adds parent-domain public support
  contact/status boundary rows for public support contact, support status page
  contact, support runbook contact, incident status contact, backend-upload
  support contact, and billing-support contact. It now carries explicit
  status-boundary references for each contact surface and proves only
  source-contract readiness, status-boundary handoff metadata, and manual
  requirements while keeping public runtime execution,
  support backend upload execution, account lookup execution, billing provider
  contact, remote support sessions, production SLA, legal disclosure execution,
  provider secrets, and child activity custody unclaimed.
- `mobile-child-agent-capability-proof` adds package/runtime hook evidence for
  Android debug APK/checksum, Android package-local status, Android device
  install/manual Play signing, iOS Xcode target, iOS simulator status, iOS
  signing, and iOS TestFlight/device proof without promoting any store or
  signing claim.
- Parent mobile route-status/service-bridge proof now records Android parent
  mobile, iOS parent mobile, Android child agent, and iOS child agent as
  separate claim boundaries. It proves typed local-service, LAN-service,
  cloud-relay, parent-cache, parent-owned-storage, mobile-package, observer
  read-only, controller-takeover manual-required, LAN AI provider degraded or
  unavailable, phone-local-model disabled, and package/signing/store
  manual-required states without promoting mobile controller authority or child
  mobile parity.
- `billing-account-endpoint-contract-proof` defines endpoint-domain route ids,
  API paths, headers, query params, and contract-version labels for account
  status, plan/entitlement snapshot, subscription status, device-limit decision,
  and account download/update/status surfaces. It is route contract proof only:
  no Stripe SDK, billing provider backend, account backend, portal UI, updater
  runtime, or child-activity custody is implemented.
- `production-release-public-status-proof` defines parent-domain public
  release/download/account/status readiness rows for `family.ocentra.ca`,
  public download, release status, update status, account status, subscription
  status, and support status. It keeps the public website runtime, account
  backend, billing provider runtime, production publishing, signing/store proof,
  updater execution, support backend upload, and child-activity custody
  unimplemented or manual-required.
- `production-release-public-runtime-handoff-proof` adds parent-domain
  handoff rows for public download, release status, update status, account
  status, subscription status, and support status plus adapter rows for the
  public website runtime, download/status backend, release publishing pipeline,
  updater status runtime, account backend, billing provider runtime, and support
  backend upload. It proves only the runtime handoff/status contract boundary:
  public runtime, account backend, billing provider runtime, production
  publishing, signing/store proof, updater execution, support upload, real
  device/store evidence, and child-activity custody remain non-claims.
- `production-release-public-docs-status-proof` adds parent-domain document
  status rows for the public privacy policy, retention policy, export/delete
  process, support runbook, incident status disclosure, and legal disclosure.
  It proves only source-contract readiness plus manual publication requirements:
  public website publication, support backend upload, account lookup execution,
  billing provider contact, remote support sessions, production SLA, legal
  disclosure execution, and child-activity custody remain non-claims.
- `production-release-public-surface-publication-proof` composes the existing
  public release/status, runtime handoff, and public docs status contracts into
  a deterministic `family.ocentra.ca` publication/readiness proof. It proves the
  public website/download/account/status publication rows remain manual,
  backend-required, promotion-required, or not implemented; it does not claim
  public runtime execution, account backend runtime, billing provider runtime,
  signing/store proof, updater execution, support upload, production SLA, legal
  execution, or child-activity custody.
- `production-release-public-status-freshness-proof` adds parent-domain
  freshness/status rows for public download, release status, update status,
  account status, subscription status, and support status. It proves only the
  source-contract freshness policy boundary and keeps public runtime execution,
  account backend runtime, billing provider runtime, production publishing,
  signing/store proof, updater execution, support backend upload, production
  SLA, and child-activity custody unclaimed.
- `production-release-public-docs-freshness-proof` adds parent-domain
  freshness rows for privacy policy, retention policy, export/delete process,
  support runbook, incident status disclosure, and legal disclosure. It proves
  only the source-contract freshness policy boundary and keeps public
  publication execution, legal disclosure execution, support backend upload,
  account lookup execution, billing provider contact, remote support sessions,
  production SLA, and child-activity custody unclaimed.
  account backend runtime, billing provider runtime, production publishing,
  signing/store proof, updater execution, support backend upload, production
  SLA, and child-activity custody unclaimed.
- `production-support-publication-status-freshness-proof` adds parent-domain
  freshness rows for support runbook publication, incident status publication,
  public support contact publication, support backend upload publication,
  privacy/legal publication, and account/billing support publication. It proves
  only the source-contract freshness policy boundary and keeps public runtime,
  support publication execution, support backend upload execution, account
  lookup execution, billing provider contact, production SLA, legal disclosure
  execution, remote support sessions, and child-activity custody unclaimed.
- `billing-entitlement-contract-proof` defines parent-domain contracts for
  plan entitlement rows, entitlement snapshots, subscription status sync events,
  device-limit decisions, parent-visible failure states, local-safety fallback,
  evidence-export retention, and explicit billing non-claims. It is contract
  proof only: no Stripe SDK, billing provider backend, account backend,
  entitlement signing runtime, portal UI, child-device consumption, or
  child-activity custody is implemented.
- `billing-subscription-device-limit-failure-proof` extends that contract proof
  with subscription status proof rows, over-limit device activation denial,
  trusted existing-device grace/manual states, all current billing failure kinds,
  retained evidence-export access, parent-visible resolution labels, and
  existing-local-safety continuation. It remains contract proof only and does
  not add provider, backend, portal, or child-device runtime code.
- `billing-account-runtime-boundary-proof` adds parent-domain account runtime
  boundary rows for account status available, backend unavailable, provider
  unavailable, and entitlement signing manual-required states. It proves no
  Stripe SDK or provider secrets in app/source, no portal billing UI, no
  child-device entitlement consumption, no child-activity custody, and visible
  local-safety fallback when the backend/provider path is unavailable.
- `billing-support-admin-boundary-proof` adds parent-domain billing support
  admin rows for support-case triage, account-status review, billing escalation
  request, provider-contact manual-required state, entitlement admin override
  manual-required state, and refund/credit manual-required state. It proves the
  non-UI/non-provider support boundary keeps Stripe/provider code and secrets
  absent, keeps provider contact, account backend admin runtime, entitlement
  override runtime, refund/credit runtime, support backend upload, and portal
  admin UI unimplemented, and excludes child activity custody while retaining
  evidence export and local safety behavior.
- `billing-support-admin-status-proof` adds parent-domain billing support/admin
  status rows for parent-visible case triage, account review, escalation,
  provider-contact manual-required, entitlement-override manual-required,
  refund-credit manual-required, and resolution-update readiness states. It links
  the rows back to the billing support/admin boundary, entitlement,
  device-limit, and failure-state proofs while preserving non-claims for real
  provider contact, account lookup execution, entitlement override,
  refund/credit runtime, portal admin UI, support backend upload, production
  billing support execution, and child activity custody.
- `billing-entitlement-runtime-proof` adds parent-domain runtime/status
  consumption rows for account entitlement snapshots, device-limit decisions,
  and billing failure states. It proves local status consumption of active,
  stale, payment-required, and provider-unavailable entitlement snapshots,
  blocks over-limit new-device activation, carries parent-visible failure state
  into runtime rows, retains evidence export/local safety, and keeps Stripe/live
  provider execution, provider contact, refund/credit execution, child activity
  custody, portal UI, and production billing support unclaimed.
- Billing/support/public website/store distribution are planned or incomplete.

## Current Gap

Need production signing, release channels, app store paths, Play/TestFlight,
support docs, privacy/legal docs, billing entitlement flows, update rollback,
public download/account/status surfaces, production support workflows, and real
signed installer/update-channel/store evidence beyond CI-mechanical
package/runtime proof.

## Checklist

- [ ] Windows signing and installer release proof.
- [ ] Parent desktop package runtime proof tied to installer/update smoke.
      Current `parent-desktop-release-support-proof` covers typed package
      runtime evidence and CI/manual-required release posture; signed
      installer/update smoke remains a production gap.
- [ ] Updater channel, rollback, and failure status.
      Current `v8-updater-rollback-runbook-proof` records updater channel,
      rollback, failure-status, and manual-required rows for scaffold,
      unsigned-preview, signature-required, and production update channels.
      Production rollback execution, signed update channel evidence, rollback
      failure smoke, published support runbook, and support escalation execution
      remain manual-required.
- [ ] macOS signing/notarization if shipped.
- [ ] Android Play signing/store proof if shipped. Current
      `mobile-child-agent-capability-proof` row keeps Play signing planned and
      device install manual-required.
- [ ] Android parent mobile route-status/service bridge proof. Current
      `parent-mobile-service-bridge-proof` and
      `parent-mobile-controller-observer-handoff-proof` keep Android parent
      mobile observer/request-first, controller authority manual-required,
      parent cache stale, parent-owned storage offline, and child-agent parity
      unclaimed.
- [ ] iOS TestFlight/App Store proof if shipped. Current
      `mobile-child-agent-capability-proof` row keeps signing-required,
      TestFlight/device proof manual-required, and App Store planned.
- [ ] iOS parent mobile route-status/service bridge proof. Current
      `parent-mobile-service-bridge-proof` and
      `parent-mobile-controller-observer-handoff-proof` keep iOS parent mobile
      controller-candidate behavior manual-required, LAN/provider routing
      unavailable, parent cache stale, parent-owned storage offline, and
      entitlement/TestFlight/device proof unclaimed.
- [ ] Billing/subscription/account flow. Current
      `billing-account-endpoint-contract-proof` covers endpoint contracts for
      account status, entitlement snapshot, subscription status, device-limit
      decision, and download/update/status surfaces only; billing provider
      integration, account backend, entitlement runtime, and UI remain
      incomplete. Current `billing-entitlement-contract-proof` covers typed plan,
      entitlement snapshot, subscription sync, device-limit decision, failure
      behavior, evidence-export retention, and no-safety-shutdown contract proof
      only; provider integration, backend storage/signing, runtime delivery,
      portal UI, and child-device consumption remain incomplete.
      `billing-subscription-device-limit-failure-proof` adds subscription
      status proof rows, over-limit device-denial rules, existing-device
      grace/manual paths, and all current billing failure kinds while keeping
      provider/runtime work unimplemented.
      `billing-account-runtime-boundary-proof` adds account status,
      backend/provider unavailable, and entitlement-signing manual-required
      runtime-boundary proof without adding Stripe/provider secrets, portal UI,
      child-device consumption, or child-activity custody.
      `billing-support-admin-boundary-proof` adds support/admin boundary rows
      for billing escalation, provider-contact manual-required, account review,
      entitlement override manual-required, and refund/credit manual-required
      states without adding provider contact, account backend admin runtime,
      portal admin UI, support upload, or child-activity custody.
      `billing-support-admin-status-proof` adds parent-visible billing
      support/admin status and resolution-update proof rows while preserving
      provider contact, account lookup execution, entitlement override,
      refund/credit runtime, portal admin UI, support backend upload, and
      child-activity custody as explicit non-claims.
      `billing-entitlement-runtime-proof` adds runtime/status consumption proof
      for account entitlement snapshots, device-limit decisions, and billing
      failure states while keeping live provider execution, provider contact,
      refund/credit execution, child custody, portal UI, and production billing
      support unclaimed.
- [ ] Privacy, retention, export/delete, and support docs. Current
      `production-release-public-docs-status-proof` records privacy policy,
      retention policy, export/delete process, support runbook, incident status
      disclosure, and legal disclosure rows as source-contract-ready with
      manual publication required. Public website publication, support backend
      upload, account lookup execution, billing provider contact, remote
      support sessions, production SLA, legal disclosure execution, and
      child-activity custody remain unimplemented or unclaimed.
- [ ] Support bundle redaction proof scaffold and incident process. Current
      release-support proof requires parent consent, incident/status metadata,
      visible support-bundle data-class disclosure, support-safe diagnostic
      references, manual-required production support states, and blocks tokens,
      child activity, raw URLs, screenshots, journals, SQLite snapshots, private
      paths, commands, keystrokes, clipboard data, and message contents from
      support output. Current `support-bundle-redaction-proof` moves that
      support-bundle boundary into logging-domain contracts and adds explicit
      backend-upload, billing-escalation, account-lookup, remote-support, and
      production-SLA manual/not-implemented rows. Real support backend upload,
      billing/account escalation, public account lookup, remote support, and
      production SLA remain unimplemented/manual-required.
- [ ] Production support incident privacy/legal workflow proof. Current
      `support-incident-workflow-proof` covers parent consent gating,
      privacy/legal disclosure before export, redaction and custody audit refs,
      support-safe incident workflow state, and backend-upload/billing/account
      manual-required states. Real support backend upload, account lookup,
      billing provider contact, remote support sessions, production SLA, public
      privacy policy publication, and Ocentra-hosted child activity custody
      remain unimplemented/unclaimed.
- [ ] Production support backend upload status boundary proof. Current
      `production-support-backend-upload-status-proof` covers parent-initiated
      and parent-consented support upload queued, running, succeeded, failed,
      manual-required, backend-unavailable, and provider-unavailable status
      rows. It proves redaction/audit refs, retry-queued and retry-exhausted
      behavior, abandon refs, manual proof requirements, and package/runtime
      refs while keeping raw child activity custody, provider secrets, remote
      support transcripts, real backend upload execution, account lookup
      execution, billing provider execution, default Ocentra-hosted family data,
      and production SLA unclaimed.
- [ ] Production support backend upload execution/runtime proof. Current
      `production-support-backend-upload-execution-runtime-proof` covers
      parent-consented support upload request recording, redaction preflight
      readiness, dispatch-manual-required, backend-unavailable,
      provider-unavailable, retry-scheduled, and operator-abandoned runtime
      boundary rows. It proves status refs, runtime refs, redaction/audit refs,
      retry/abandon refs, and manual proof requirements while keeping child
      activity custody, provider secrets, remote support transcripts, real
      backend upload execution, account lookup execution, billing provider
      contact execution, remote support sessions, production SLA, and default
      Ocentra-hosted family data unclaimed.
- [ ] Production support backend upload custody/audit proof. Current
      `production-support-backend-upload-custody-audit-proof` covers custody
      boundary recording, retention manual-required state, delete request
      recording, deletion manual-required state, and support-safe audit export
      readiness for support backend upload. It proves status/runtime refs,
      redaction refs, custody refs, retention/delete refs, and manual proof
      requirements while keeping real backend upload execution, backend payload
      retention, backend payload deletion, raw child activity custody, provider
      secrets, account lookup execution, billing provider contact execution,
      remote support sessions, production SLA, and default Ocentra-hosted
      family data unclaimed.
- [ ] Production support case resolution/status proof. Current
      `production-support-case-resolution-status-proof` covers case opened,
      triage-ready, parent-update-ready, escalation manual-required, response
      manual-required, closure-ready, and SLA manual-required states with
      incident, upload status/execution, publication, response, escalation,
      closure, SLA, and manual proof refs. Real backend upload execution,
      provider contact, account lookup, billing provider contact, remote
      support sessions, production SLA execution, raw child activity custody,
      provider secrets, remote transcripts, and hosted family data remain
      unclaimed.
- [ ] Production incident/support status proof. Current
      `production-incident-support-status-proof` covers support incident intake,
      parent consent, privacy/legal disclosure, data export request, delete
      request, incident publication, and case resolution handoff status rows.
      It links support workflow, custody/audit, case resolution, publication,
      public docs, and data-custody expectations while keeping public
      publication, legal execution, backend upload execution, account lookup,
      billing provider contact, remote support sessions, production SLA,
      provider secrets, and child activity custody unclaimed.
- [ ] Production support process runtime status proof. Current
      `production-support-process-runtime-status-proof` covers requested,
      authorized, queued, running, failed, succeeded, and manual-required
      support process runtime status rows plus incident runtime requested,
      authorized, running, evidence-ready, and manual-required rows with
      support workflow, incident status, status-backend runtime execution,
      backend upload status/runtime, case resolution, publication runtime,
      custody, and documentation refs. Real incident runtime execution, backend
      upload execution, public runtime execution, provider execution,
      production SLA, remote support sessions, provider secrets, child activity
      custody, and default Ocentra-hosted family data remain unclaimed.
- [ ] Production support account/SLA status proof. Current
      `production-support-account-sla-status-proof` covers account lookup
      request/result status, billing provider contact status, remote support
      request/session status, and production SLA status rows. It links incident
      support, billing support/admin, public support contact, case resolution,
      release-installer, billing, and data-custody refs while keeping account
      lookup execution, billing provider contact, remote support sessions,
      production SLA, support backend upload execution, `family.ocentra.ca`
      runtime, provider secrets, and child activity custody unclaimed.
- [ ] Production support legal/provider readiness proof. Current
      `production-support-legal-provider-readiness-proof` covers privacy/legal
      review, data export/delete runtime, provider-secret custody, billing
      provider contact, remote-support legal/session boundary, and production
      SLA legal boundary rows. It links incident support, account/SLA support,
      publication freshness, release-installer, billing, data-custody, and
      documentation refs while keeping legal disclosure execution, data
      export/delete runtime, provider secret custody, billing provider contact
      execution, account lookup execution, remote support sessions, production
      SLA, support backend upload execution, public runtime execution, and child
      activity custody unclaimed.
- [ ] Production support provider-secret custody status proof. Current
      `production-support-provider-secret-custody-status-proof` covers
      custody-boundary recorded, provider-secret absent, backend secret store
      manual-required, rotation manual-required, revocation manual-required, and
      support-safe audit export readiness rows. It proves legal/provider,
      billing support, redaction, custody audit, rotation, revocation, manual
      proof, and audit export refs while keeping real provider-secret custody,
      backend secret store execution, rotation execution, revocation execution,
      support backend upload execution, account lookup execution, billing
      provider contact, remote support sessions, production SLA, default
      Ocentra-hosted family data, and child activity custody unclaimed.
- [ ] Production support provider-secret execution readiness proof. Current
      `provider-secret-execution-readiness-proof` covers provider-secret
      execution boundary, backend secret-store preflight, rotation preflight,
      revocation preflight, operator approval, manual execution, and
      support-safe audit export rows with custody/preflight/operator/manual/audit
      refs. It remains deterministic logging contract proof only: backend secret
      store execution, provider-secret rotation execution, provider-secret
      revocation execution, provider-secret delivery, support backend upload
      execution, account lookup execution, billing provider contact, remote
      support sessions, production SLA, default Ocentra-hosted family data, and
      child activity custody remain unimplemented or unclaimed.
- [ ] Production support data export/delete runtime lifecycle proof. Current
      `production-support-data-export-delete-lifecycle-proof` covers
      parent-authorized export and delete requested, authorized, queued,
      running, succeeded, failed, and manual-required lifecycle rows in
      parent-domain and logging-domain contracts. It proves support-safe local
      status metadata, local output/delete refs, redaction/audit refs, and
      manual proof requirements while keeping real backend upload execution,
      public runtime execution, provider execution, production SLA, remote
      support sessions, default Ocentra-hosted family data, and child activity
      custody unclaimed.
- [ ] Production support delete executor proof. Current
      `production-support-delete-executor-proof` covers delete executor
      readiness/status rows for local export output, support backend payload,
      status backend payload, public runtime payload, and legal disclosure
      payload targets with delete-request, authorization, redaction/audit,
      custody, source-proof, and manual-proof refs. It remains deterministic
      logging contract proof only: real delete execution, durable queues,
      payload deletion execution, provider execution, public runtime, legal
      execution, backend upload execution, production SLA, default
      Ocentra-hosted family data, and child activity custody remain
      unimplemented or unclaimed.
- [ ] Production support public publication workflow proof. Current
      `production-support-publication-workflow-proof` covers public privacy
      policy publication, privacy/legal disclosure execution, support runbook
      publication, support incident status publication, support backend upload
      publication handoff, and public support contact publication as
      source-contract/manual-required rows. Real public runtime, support backend
      upload execution, account lookup execution, billing provider contact,
      production SLA, legal disclosure execution, remote support sessions, and
      child activity custody remain unimplemented or unclaimed.
- [ ] Production support publication runtime readiness proof. Current
      `production-support-publication-runtime-readiness-proof` covers public
      runtime, support runbook publication runner, incident status publication
      runner, support upload publication runtime, privacy/legal publication
      runtime, and public support contact runtime readiness as source-backed
      manual-required/backend-required/legal-review-required rows. It keeps real
      public runtime execution, publication runner execution, support backend
      upload execution, account lookup execution, billing provider contact,
      production SLA, legal disclosure execution, remote support sessions, and
      child activity custody unimplemented or unclaimed.
- [ ] Production support public surface export closure proof. Current
      `production-support-public-surface-export-closure-proof` verifies package
      exports for public release status, public status freshness, public docs
      freshness, support-publication runtime readiness, support-publication
      status freshness, and public support contact status modules. It remains a
      package surface/proof closure only: real public runtime execution,
      publication runner execution, status backend execution, support backend
      upload execution, account lookup, billing provider contact, legal
      disclosure execution, remote support sessions, production SLA,
      provider-secret custody, and child activity custody remain unimplemented
      or unclaimed.
- [ ] Production support publication execution status proof. Current
      `production-support-publication-execution-status-proof` covers support
      runbook, incident status, public support contact, support backend upload,
      privacy/legal, and account/billing publication execution status labels for
      requested, queued, running, succeeded, failed, and manual-required rows.
      It is status-contract proof only: real public runtime execution,
      publication runner execution, status backend execution, support backend
      upload execution, account lookup, billing provider contact, legal
      disclosure execution, remote support sessions, production SLA,
      provider-secret custody, and child activity custody remain unimplemented
      or unclaimed.
- [ ] Production support status backend/public runtime follow-through proof.
      Current
      `production-support-status-backend-public-runtime-followthrough-proof`
      covers support status public runtime, support runbook status backend,
      incident status backend, public support contact status backend, support
      upload status backend, and account/billing status backend follow-through
      labels for requested, queued, running, succeeded, failed, and
      manual-required rows. It remains deterministic contract/status proof only:
      real public runtime execution, status backend execution, support backend
      upload execution, account lookup, billing provider contact, legal
      disclosure execution, remote support sessions, production SLA,
      provider-secret custody, public runtime payload custody, and child
      activity custody remain unimplemented or unclaimed.
- [ ] Production support status backend execution queue proof. Current
      `production-support-status-backend-execution-queue-proof` covers support
      runbook, incident, public support contact, support upload, privacy/legal,
      and account/billing status backend queue labels for requested,
      authorized, queued, running, succeeded, failed, manual-required, and
      backend-unavailable rows. It remains deterministic contract/status proof
      only: real status backend execution, durable queue storage, retry worker
      execution, audit persistence, public runtime execution, provider
      execution, support backend upload execution, account lookup, billing
      provider contact, legal disclosure execution, remote support sessions,
      production SLA, provider-secret custody, status backend payload custody,
      and child activity custody remain unimplemented or unclaimed.
- [ ] Production support status backend queue audit/persistence proof. Current
      `production-support-status-backend-queue-audit-persistence-proof` covers
      support runbook, incident, public support contact, support upload,
      privacy/legal, and account/billing status backend queue audit persistence
      labels for requested, authorized, queued, retry-scheduled, audit-ready,
      failed, manual-required, and backend-unavailable rows. It remains
      deterministic contract/status proof only: real status backend execution,
      durable queue storage, retry worker execution, audit persistence, public
      runtime execution, provider execution, support backend upload execution,
      account lookup, billing provider contact, legal disclosure execution,
      remote support sessions, production SLA, provider-secret custody, status
      backend payload custody, and child activity custody remain unimplemented
      or unclaimed.
- [ ] Production support status backend dead-letter proof. Current
      `production-support-status-backend-dead-letter-proof` covers support
      runbook, incident, public support contact, support upload, privacy/legal,
      and account/billing status backend dead-letter/manual-triage labels for
      requested, authorized, dead-lettered, triage-ready, retry-blocked, failed,
      manual-required, and backend-unavailable rows. It remains deterministic
      contract/status proof only: real status backend execution, durable queue
      storage, retry worker execution, audit persistence, dead-letter payload
      custody, public runtime execution, provider execution, support backend
      upload execution, account lookup, billing provider contact, legal
      disclosure execution, remote support sessions, production SLA,
      provider-secret custody, and child activity custody remain unimplemented
      or unclaimed.
- [ ] Production support status backend runtime execution proof. Current
      `production-support-status-backend-runtime-execution-proof` covers
      support runbook, incident, public support contact, support upload,
      privacy/legal, and account/billing status backend runtime execution
      labels for requested, authorized, queued, running,
      runtime-evidence-ready, audit-ready, failed, manual-required, and
      backend-unavailable rows. It remains deterministic contract/status proof
      only: real status backend execution, durable queue storage, retry worker
      execution, audit persistence, dead-letter payload custody, public runtime
      execution, provider execution, support backend upload execution, account
      lookup, billing provider contact, legal disclosure execution, remote
      support sessions, production SLA, provider-secret custody, status backend
      payload custody, and child activity custody remain unimplemented or
      unclaimed.
- [ ] Production support status backend payload custody proof. Current
      `production-support-status-backend-payload-custody-proof` covers status
      backend payload custody boundary, retention manual-required, delete
      request, deletion manual-required, audit-export-ready, and
      backend-unavailable states with support-safe status target, queue, audit,
      redaction, custody, retention, delete, and manual proof refs. It remains
      deterministic logging contract proof only: real status backend execution,
      durable status backend payload storage, payload deletion execution, retry
      worker execution, audit persistence, public runtime execution, provider
      execution, support backend upload execution, account lookup, billing
      provider contact, legal disclosure execution, remote support sessions,
      production SLA, provider-secret custody, default Ocentra-hosted family
      data, and child activity custody remain unimplemented or unclaimed.
- [ ] Production support status backend redaction manifest proof. Current
      `production-support-status-backend-redaction-manifest-proof` covers
      status backend redaction manifest readiness and manual-required rows with
      support-safe status target, queue, audit, redaction manifest, and manual
      proof refs. It remains deterministic logging contract proof only: real
      status backend execution, status backend payload custody, durable payload
      storage, payload deletion, retry worker execution, audit persistence
      execution, public runtime execution, support backend upload execution,
      provider execution, account lookup, billing provider contact, legal
      disclosure execution, remote support sessions, production SLA,
      provider-secret custody, and child activity custody remain unimplemented
      or unclaimed.
- [ ] Production support status backend runtime closure proof. Current
      `production-support-status-backend-runtime-closure-proof` composes the
      status backend runtime execution, queue/audit persistence, dead-letter,
      payload-custody, redaction-manifest, and public-runtime follow-through
      proof refs into one support-safe closure read model. It remains
      deterministic parent-domain contract proof only: real status backend
      execution, durable queue storage, retry-worker execution, audit
      persistence, dead-letter payload custody, status backend payload custody,
      redaction manifest execution, public runtime execution, provider
      execution, support backend upload execution, account lookup, billing
      provider contact, legal disclosure execution, remote support sessions,
      production SLA, provider-secret custody, and child activity custody
      remain unimplemented or unclaimed.
- [ ] Production support status backend durable queue runtime proof. Current
      `production-support-status-backend-durable-queue-runtime-proof` covers
      durable queue storage, retry-worker, audit-persistence, dead-letter,
      runtime execution, and runtime closure refs for support runbook, incident,
      public support contact, support upload, privacy/legal, and
      account/billing status backend targets. It remains deterministic
      contract/read-model proof only: real status backend execution, durable
      queue storage, retry-worker execution, audit persistence, dead-letter
      payload custody, public runtime execution, provider execution, support
      backend upload execution, account lookup, billing provider contact, legal
      disclosure execution, remote support sessions, production SLA,
      provider-secret custody, and child activity custody remain unimplemented
      or unclaimed.
- [ ] Production support status backend execution continuation proof. Current
      `production-support-status-backend-execution-continuation-proof` covers
      execution preflight, runtime-worker-required, durable-storage-required,
      payload-custody-required, redaction-manifest-required, manual-required,
      and backend-unavailable rows for support runbook, incident, public
      support contact, support upload, privacy/legal, and account/billing
      status backend targets. It remains deterministic contract/read-model
      proof only: real status backend execution, durable queue storage,
      retry-worker execution, audit persistence, dead-letter payload custody,
      status-backend payload custody, redaction-manifest execution, public
      runtime execution, provider execution, support backend upload execution,
      account lookup, billing provider contact, legal disclosure execution,
      remote support sessions, production SLA, provider-secret custody, default
      Ocentra-hosted family data, and child activity custody remain
      unimplemented or unclaimed.
- [ ] Production support proof/status matrix closure proof. Current
      `production-support-proof-status-matrix-closure-proof` reconciles the
      status-backend runtime, public runtime/publication, privacy/legal,
      provider-secret, export/delete, and release-installer support proof refs
      into one source-backed status pack after PR534. It remains deterministic
      contract/read-model proof only: real public runtime, real status backend
      execution, signing/store proof, updater execution, support backend upload
      execution, account/billing provider execution, legal disclosure
      execution, production SLA, provider-secret custody, and child activity
      custody remain unimplemented, manual-required, or unclaimed.
- [ ] Production support privacy/legal disclosure status proof. Current
      `production-support-privacy-legal-disclosure-status-proof` covers
      privacy/legal disclosure requested, parent-authorized, legal-review
      queued, legal-review running, parent-notification-ready,
      publication-ready, failed, and manual-required rows with support-safe
      status refs, legal review refs, audit refs, failure refs, and manual proof
      requirements. It is status-contract proof only: real legal disclosure
      execution, public runtime execution, support backend upload execution,
      account lookup, billing provider contact, remote support sessions,
      production SLA, provider secrets, remote support transcripts, raw child
      activity custody, and raw support bundle payloads remain unimplemented or
      unclaimed.
- [ ] Production support provider-secret rotation/revocation status proof.
      Current
      `production-support-provider-secret-rotation-revocation-status-proof`
      covers rotation requested, rotation preflight-ready, rotation
      manual-required, revocation requested, revocation preflight-ready,
      revocation manual-required, and audit-export-ready rows with custody
      status, execution readiness, backend secret-store preflight, operator
      approval, manual proof, and audit refs. It remains deterministic logging
      contract proof only: real backend secret store execution, rotation
      execution, revocation execution, provider-secret delivery, support backend
      upload execution, account lookup, billing provider contact, remote support
      sessions, production SLA, default Ocentra-hosted family data, and child
      activity custody remain unimplemented or unclaimed.
- [ ] Production support backend provider runtime readiness proof. Current
      `production-support-backend-provider-runtime-readiness-proof` composes
      support backend upload execution runtime, upload custody/audit,
      provider-secret execution readiness, account/SLA, privacy/legal, and case
      resolution proof refs into support-safe upload/provider readiness rows. It
      remains deterministic logging contract proof only: real support backend
      upload execution, provider-secret delivery/custody execution, account
      lookup, billing provider contact, legal disclosure execution, remote
      support sessions, production SLA, default Ocentra-hosted family data, and
      child activity custody remain unimplemented or unclaimed.
- [ ] Public support contact/status boundary proof. Current
      `public-support-contact-status-proof` covers public support contact,
      support status page contact, support runbook contact, incident status
      contact, backend-upload support contact, and billing-support contact as
      source-contract/manual-required rows with explicit status-boundary
      references for each contact surface. Public runtime execution, support
      backend upload execution, account lookup execution, billing provider
      contact, remote support sessions, production SLA, legal disclosure
      execution, provider secrets, and child activity custody remain
      unimplemented or unclaimed.
- [ ] Public website/download/account/status surfaces. Current
      `production-release-public-status-proof` records public download, release
      status, update status, account status, subscription status, and support
      status surfaces as route-contract/manual readiness only. The
      `family.ocentra.ca` runtime, account backend, billing provider runtime,
      production publishing, signing/store proof, updater execution, support
      backend upload, and child-activity custody remain unimplemented or
      manual-required. Current
      `production-release-public-runtime-handoff-proof` adds runtime handoff and
      adapter status rows for those surfaces while keeping public runtime,
      account/backend/provider execution, publishing/signing/store proof,
      updater execution, support upload, real-device/store evidence, and child
      activity custody unimplemented or manual-required. Current
      `production-release-public-surface-publication-proof` composes the public
      status, runtime handoff, and docs status rows into a
      `family.ocentra.ca` publication/readiness proof while keeping public
      runtime execution, account backend runtime, billing provider runtime,
      signing/store proof, updater execution, support upload, production SLA,
      legal execution, and child-activity custody unclaimed. Current
      `production-release-public-status-freshness-proof` adds a source-contract
      freshness policy boundary for the same public download/account/status
      surfaces while keeping public runtime execution, account backend runtime,
      billing provider runtime, production publishing, signing/store proof,
      updater execution, support backend upload, production SLA, and
      child-activity custody unclaimed.
      Current `production-release-public-docs-freshness-proof` adds the same
      freshness boundary for public privacy, retention, export/delete, support
      runbook, incident disclosure, and legal disclosure document rows while
      keeping publication/legal execution, support upload, account lookup,
      billing provider contact, remote support, production SLA, and
      child-activity custody unclaimed.
      Current `production-support-publication-status-freshness-proof` adds a
      support-publication freshness boundary for support runbook, incident
      status, public support contact, support upload publication, privacy/legal,
      and account/billing support publication rows while keeping real public
      runtime, support publication execution, support upload execution, account
      lookup execution, billing provider contact, production SLA, legal
      execution, remote support sessions, and child-activity custody unclaimed.

## Next AI Instructions

Do not promote scaffold package previews to product support. Every release claim
must name platform, signing/store state, smoke proof, support path, and known
limitations. The product checklist now records this boundary: parent desktop
package runtime release-support proof is preview/mechanical evidence only, not
signing, store, notarization, Play, TestFlight, production updater rollback, or
child mobile parity proof.
