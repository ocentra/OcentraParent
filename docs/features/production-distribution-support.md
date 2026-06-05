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
- `production-support-publication-workflow-proof` adds parent-domain source
  contract rows for public privacy policy publication, privacy/legal disclosure
  execution, support runbook publication, support incident status publication,
  support backend upload publication handoff, and public support contact
  publication. It proves the publication workflow remains source-contract
  ready/manual-required while rejecting real public runtime, support backend
  upload execution, account lookup execution, billing provider contact,
  production SLA, legal disclosure execution, remote support sessions, and child
  activity custody.
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
- `public-status-surface-readiness-proof` adds a focused parent-domain
  readiness matrix for `family.ocentra.ca`, public download, release/update
  status, account/subscription status, and support status surfaces. It links the
  existing public status, runtime handoff, docs status, billing endpoint,
  entitlement runtime, and support case proof rows while keeping real public
  runtime, account backend runtime, billing provider runtime, production
  publishing, signing/store proof, updater execution, support backend upload,
  production SLA, legal execution, remote support sessions, and child activity
  custody unclaimed.
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
- [ ] Production support public publication workflow proof. Current
      `production-support-publication-workflow-proof` covers public privacy
      policy publication, privacy/legal disclosure execution, support runbook
      publication, support incident status publication, support backend upload
      publication handoff, and public support contact publication as
      source-contract/manual-required rows. Real public runtime, support backend
      upload execution, account lookup execution, billing provider contact,
      production SLA, legal disclosure execution, remote support sessions, and
      child activity custody remain unimplemented or unclaimed.
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
      legal execution, and child-activity custody unclaimed.
      `public-status-surface-readiness-proof` adds a focused readiness matrix
      for the same public website/download/account/status surfaces and links
      them to public status, runtime handoff, public docs, billing endpoint,
      entitlement runtime, and support case evidence while keeping the same
      runtime/backend/provider/support/SLA/legal/custody non-claims.

## Next AI Instructions

Do not promote scaffold package previews to product support. Every release claim
must name platform, signing/store state, smoke proof, support path, and known
limitations. The product checklist now records this boundary: parent desktop
package runtime release-support proof is preview/mechanical evidence only, not
signing, store, notarization, Play, TestFlight, production updater rollback, or
child mobile parity proof.
