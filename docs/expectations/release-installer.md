<!-- agent-capsule -->

> Agent Capsule
> Doc: Release And Installer Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Release And Installer Expectations

Release features are product features because parents need install/update paths that work.

## Expected Deliverables

- Version policy.
- Installer artifact.
- Install smoke.
- Uninstall smoke.
- Update manifest where applicable.
- Signature verification where applicable.
- Clear production/manual release boundary.
- Release-support proof covering update/rollback posture, signing/store claim
  boundaries, package-runtime evidence, support diagnostic redaction, CI
  artifacts, support incident handoff metadata, parent consent, safe
  support-bundle manifest disclosure, and manual proof requirements.
- V8 release/support readiness gate that summarizes package-preview artifacts,
  support-safe diagnostic/runbook readiness, updater rollback execution state,
  signing/store proof state, production publishing state, and manual platform
  gaps without promoting preview artifacts to production release evidence.
- V8 updater rollback and release-support runbook status proof that names
  scaffold, unsigned-preview, signature-required, and production update channels
  separately, records rollback execution as unavailable, records rollback
  failure handling as manual-required, and keeps the production support runbook
  manual-required until it is published and exercised.
- V8 public release/download/account/status readiness proof that names public
  download, release status, update status, account status, subscription status,
  and support status separately while keeping `family.ocentra.ca` runtime,
  production publishing, signing/store proof, updater execution, support backend
  upload, account backend, billing provider runtime, and child-activity custody
  unclaimed.
- V8 public release/download/account/status runtime handoff proof that links
  those public surfaces to route/status/backend adapter rows and keeps public
  runtime, account backend, billing provider runtime, production publishing,
  signing/store proof, updater execution, support backend upload, real
  device/store evidence, and child-activity custody unclaimed.
- V8 public privacy/legal/support-docs status proof that names privacy policy,
  retention policy, export/delete process, support runbook, incident status
  disclosure, and legal disclosure separately while keeping public publication,
  support backend upload, account lookup execution, billing provider contact,
  remote support, production SLA, legal disclosure execution, and
  child-activity custody unclaimed.
- V8 public surface publication/status proof that composes the
  `family.ocentra.ca` public status, runtime handoff, and public docs status
  rows while keeping public runtime execution, account backend runtime, billing
  provider runtime, production publishing, signing/store proof, updater
  execution, support upload, production SLA, legal execution, and child-activity
  custody unclaimed.
- `production-release-public-docs-freshness-proof` may prove privacy policy,
  retention policy, export/delete process, support runbook, incident status
  disclosure, and legal disclosure freshness rows while keeping public
  publication execution, legal disclosure execution, support backend upload,
  account lookup execution, billing provider contact, remote support sessions,
  production SLA, and child-activity custody unclaimed.
- V8 public support contact/status proof that names public support contact,
  support status page contact, support runbook contact, incident status
  contact, backend-upload support contact, and billing-support contact while
  keeping public runtime execution, support backend upload execution, account
  lookup execution, billing provider contact, remote support sessions,
  production SLA, legal disclosure execution, provider secrets, and
  child-activity custody unclaimed.
- V8 production support backend upload status proof that records
  parent-initiated and parent-consented queued, running, succeeded, failed,
  manual-required, backend-unavailable, and provider-unavailable rows, with
  redaction/audit refs, retry/abandon refs, manual proof requirements, and
  package/runtime refs while keeping real backend execution and child-activity
  custody unclaimed.
- V8 production support backend upload execution/runtime boundary proof that
  records parent-consented request recording, redaction preflight readiness,
  dispatch manual-required, backend/provider unavailable, retry-scheduled, and
  operator-abandoned rows with status refs, runtime refs, redaction/audit refs,
  retry/abandon refs, and manual proof requirements while keeping real backend
  execution, child-activity custody, provider secrets, account lookup, billing
  provider contact, remote support sessions, production SLA, and default
  Ocentra-hosted family data unclaimed.
- V8 production support backend upload custody/audit proof that records custody
  boundary, retention manual-required, delete request, deletion
  manual-required, and support-safe audit export readiness rows with status
  refs, runtime refs, custody refs, retention/delete refs, and manual proof
  requirements while keeping real backend execution, backend payload retention,
  backend payload deletion, child-activity custody, provider secrets, account
  lookup, billing provider contact, remote support sessions, production SLA, and
  default Ocentra-hosted family data unclaimed.
- V8 production support case resolution/status proof that records case opened,
  triage-ready, parent-update-ready, escalation manual-required, response
  manual-required, closure-ready, and SLA manual-required rows with incident,
  upload status/execution, publication, response, escalation, closure, SLA, and
  manual proof refs while keeping real backend execution, provider contact,
  account lookup, billing provider contact, remote support sessions, production
  SLA execution, child-activity custody, provider secrets, remote transcripts,
  and default Ocentra-hosted family data unclaimed.
- V8 production incident/support status proof that records support incident
  intake, parent consent, privacy/legal disclosure, data export request, delete
  request, incident publication, and case resolution handoff rows while keeping
  public publication, legal execution, support backend upload execution, account
  lookup, billing provider contact, remote support sessions, production SLA,
  provider secrets, and child-activity custody unclaimed.
- V8 production support publication runtime readiness proof that records public
  runtime, support runbook publication runner, incident status publication
  runner, support upload publication runtime, privacy/legal publication runtime,
  and public support contact runtime readiness rows while keeping real public
  runtime execution, publication runner execution, support backend upload
  execution, account lookup, billing provider contact, remote support sessions,
  production SLA, legal disclosure execution, provider secrets, and
  child-activity custody unclaimed.
- V8 production support publication execution status proof that records support
  runbook, incident status, public support contact, support backend upload,
  privacy/legal, and account/billing publication execution status labels across
  requested, queued, running, succeeded, failed, and manual-required rows while
  keeping real public runtime execution, publication runner execution, status
  backend execution, support backend upload execution, account lookup, billing
  provider contact, remote support sessions, production SLA, legal disclosure
  execution, provider secrets, and child-activity custody unclaimed.
- V8 production support privacy/legal disclosure status proof that records
  disclosure requested, parent-authorized, legal-review queued,
  legal-review running, parent-notification-ready, publication-ready, failed,
  and manual-required rows while keeping legal disclosure execution, public
  runtime execution, support backend upload execution, account lookup, billing
  provider contact, remote support sessions, production SLA, provider secrets,
  remote support transcripts, and child-activity custody unclaimed.
- V8 production support legal/provider readiness proof that records
  privacy/legal review, data export/delete runtime, provider-secret custody,
  billing provider contact, remote-support legal/session boundary, and
  production SLA legal boundary rows while keeping real legal execution, export
  or delete runtime, provider secret custody, billing provider contact
  execution, account lookup execution, remote support sessions, production SLA
  commitments, support backend upload execution, public runtime execution, and
  child-activity custody unclaimed.
- V8 production support provider-secret custody status proof that records
  custody-boundary recorded, provider-secret absent, backend secret store
  manual-required, rotation manual-required, revocation manual-required, and
  support-safe audit export readiness rows while keeping real provider-secret
  custody, backend secret store execution, rotation execution, revocation
  execution, support backend upload execution, account lookup execution, billing
  provider contact, remote support sessions, production SLA, default
  Ocentra-hosted family data, and child-activity custody unclaimed.
- V8 production support status backend payload custody proof that records
  status payload custody-boundary, retention manual-required, delete request,
  deletion manual-required, audit-export-ready, and backend-unavailable rows
  while keeping real status backend execution, durable status backend payload
  storage, payload deletion execution, retry worker execution, audit
  persistence, public runtime execution, provider execution, support backend
  upload execution, account lookup execution, billing provider contact, remote
  support sessions, production SLA, default Ocentra-hosted family data, and
  child-activity custody unclaimed.

## Acceptance

- `main` builds previews and does not publish production releases.
- `production` publishes only by explicit promotion.
- Feature branches may be pushed regularly but do not publish product releases.
- Final PRs into `main` are CI integration events, not release events.
- Product releases can intentionally batch multiple completed milestones.
- Installer paths are documented.
- Support diagnostics keep only support-safe fields and redact tokens, child
  activity, raw URLs, screenshots, journals, SQLite snapshots, private paths,
  command lines, keystrokes, clipboard data, and message contents.
- Support incident handoff requires explicit parent consent and visible
  support-bundle data-class disclosure before export.
- Support bundle manifests may include release version, commit, platform,
  package/runtime, service, route, capability, degraded state, redaction
  summary, manual proof reference, and incident status metadata only.
- Support incident handoff must exclude tokens, child activity, raw URLs,
  screenshots, journals, SQLite snapshots, private paths, commands, keystrokes,
  clipboard data, and message contents.
- Support diagnostic references must point to support-safe proof JSON,
  workflow, redaction summary, manual runbook, or status-row references; they
  must not point to private user paths, command logs, raw URLs, screenshots,
  journals, SQLite stores, clipboard data, or message contents.
- Current `support-bundle-redaction-proof` records logging-domain incident rows
  for parent consent, ready/manual review, backend-upload manual-required,
  billing-escalation manual-required, and account-lookup manual-required states.
  It limits support bundles to release/package/runtime, service, route,
  capability, degraded-state, redaction, manual-proof, incident, billing-status,
  and account-status references and rejects tokens, child activity, raw URLs,
  screenshots, journals, SQLite snapshots, private paths, command lines,
  keystrokes, clipboard data, message contents, provider secrets, executed
  backend upload, billing provider contact, account lookup, remote support, and
  production SLA claims.
- Production support backend upload, account lookup, billing escalation, remote
  support, and production SLA remain explicit manual-required or
  not-implemented states until real support workflows exist.
- Update paths reject unsigned or incorrectly signed manifests once signing is enabled.
- Package claims match real artifacts.
- Parent desktop package-runtime proof names built portal dist, the Rust service
  boundary, package service-manager ownership, fixed loopback process ownership,
  connect-or-degrade behavior, blank-window guard state, and signed-channel
  update posture without treating Vite or preview CI artifacts as production
  release evidence.
- Parent desktop release/support readiness proof records the Windows, Linux,
  macOS, Android, and iOS package-preview artifact names but keeps production
  publishing, signing, store upload, and updater rollback execution
  manual-required or promotion-required until real credentials, promotion, and
  manual platform proof exist.
- Updater rollback proof must reject rollback-available claims unless a signed
  production update channel, rollback execution smoke, rollback failure-status
  smoke, and published support runbook evidence are present.
- `production-release-public-status-proof` may prove public-surface readiness
  rows and support-safe data-class boundaries, but it must not claim the public
  website runtime, account backend, billing provider runtime, production
  publishing, signing/store proof, updater execution, support backend upload,
  or child-activity custody.
- `production-release-public-runtime-handoff-proof` may prove route/status and
  backend adapter handoff rows for the public website/download/account/status
  surfaces, but it must not claim a live public runtime, account backend,
  billing provider runtime, production publishing, signing/store proof, updater
  execution, support backend upload, real device/store proof, or child-activity
  custody.
- `production-release-public-docs-status-proof` may prove source-contract and
  manual-publication status rows for public privacy, retention, export/delete,
  support, incident, and legal docs, but it must not claim public website
  publication, support backend upload, account lookup execution, billing
  provider contact, remote support sessions, production SLA, legal disclosure
  execution, or child-activity custody.
- `production-release-public-surface-publication-proof` may prove a composed
  `family.ocentra.ca` publication/readiness summary across public status,
  runtime handoff, and public docs rows, but it must not claim live public
  runtime execution, account backend runtime, billing provider runtime,
  signing/store proof, updater execution, support upload, production SLA, legal
  execution, or child-activity custody.
- `public-support-contact-status-proof` may prove public support contact/status
  source-contract rows across publication, runtime handoff, docs, incident,
  backend upload, and billing-support boundaries, but it must not claim public
  runtime execution, support backend upload execution, account lookup
  execution, billing provider contact, remote support sessions, production SLA,
  legal disclosure execution, provider secrets, or child-activity custody.
- `production-support-backend-upload-status-proof` may prove support upload
  status/read-model rows for queued, running, succeeded, failed,
  manual-required, backend-unavailable, and provider-unavailable states, but it
  must not claim raw child activity custody, provider secrets, remote support
  transcripts, real support backend upload execution, account lookup execution,
  billing provider execution, default Ocentra-hosted family data, or production
  SLA.
- `production-support-backend-upload-execution-runtime-proof` may prove
  execution/runtime boundary rows for request recording, redaction preflight,
  manual dispatch, unavailable, retry, and abandon states, but it must not claim
  raw child activity custody, provider secrets, remote support transcripts, real
  support backend upload execution, account lookup execution, billing provider
  contact execution, remote support session execution, default Ocentra-hosted
  family data, or production SLA.
- `production-support-backend-upload-custody-audit-proof` may prove
  custody/audit boundary rows for custody recording, retention manual-required,
  delete request, deletion manual-required, and support-safe audit export
  readiness, but it must not claim raw child activity custody, provider secrets,
  remote support transcripts, real support backend upload execution, backend
  payload retention, backend payload deletion, account lookup execution, billing
  provider contact execution, remote support session execution, default
  Ocentra-hosted family data, or production SLA.
- `production-support-case-resolution-status-proof` may prove support-safe
  parent-visible case resolution/status rows for opened, triage, update,
  escalation, response, closure, and SLA manual-required states, but it must not
  claim real support backend upload execution, provider contact, account lookup,
  billing provider contact, remote support session execution, production SLA
  execution, raw child activity custody, provider secrets, remote support
  transcripts, default Ocentra-hosted family data, or raw support bundle
  payloads.
- `production-incident-support-status-proof` may prove production
  incident/support status rows for parent consent, privacy/legal disclosure,
  export/delete request status, incident publication status, and case resolution
  handoff, but it must not claim real public publication, legal execution,
  support backend upload execution, account lookup execution, billing provider
  contact, remote support session execution, production SLA, raw child activity
  custody, provider secrets, default Ocentra-hosted family data, or raw support
  bundle payloads.
- `production-support-publication-runtime-readiness-proof` may prove
  source-backed runtime readiness rows for public runtime, support runbook
  publication runner, incident status publication runner, support upload
  publication runtime, privacy/legal publication runtime, and public support
  contact runtime handoffs, but it must not claim real public runtime execution,
  publication runner execution, support backend upload execution, account lookup
  execution, billing provider contact, remote support session execution,
  production SLA, legal disclosure execution, raw child activity custody,
  provider secrets, default Ocentra-hosted family data, or raw support bundle
  payloads.
- `production-support-publication-execution-status-proof` may prove
  source-contract status labels for support/publication execution targets across
  requested, queued, running, succeeded, failed, and manual-required states, but
  it must not claim real public runtime execution, publication runner execution,
  status backend execution, support backend upload execution, account lookup
  execution, billing provider contact execution, remote support session
  execution, production SLA, legal disclosure execution, raw child activity
  custody, provider secrets, default Ocentra-hosted family data, or raw support
  bundle payloads.
- `production-support-status-backend-public-runtime-followthrough-proof` may
  prove status backend/public runtime follow-through labels for support status,
  runbook, incident, contact, upload, account, and billing support targets
  across requested, queued, running, succeeded, failed, and manual-required
  states, but it must not claim real public runtime execution, status backend
  execution, support backend upload execution, account lookup execution, billing
  provider contact execution, remote support session execution, production SLA,
  legal disclosure execution, raw child activity custody, provider secrets,
  public runtime payload custody, default Ocentra-hosted family data, or raw
  support bundle payloads.
- `production-support-status-backend-execution-queue-proof` may prove
  support-safe status backend execution queue labels for support runbook,
  incident, contact, upload, privacy/legal, account, and billing support targets
  across requested, authorized, queued, running, succeeded, failed,
  manual-required, and backend-unavailable states, but it must not claim real
  status backend execution, durable queue execution, retry worker execution,
  audit persistence, public runtime execution, provider execution, support
  backend upload execution, account lookup execution, billing provider contact
  execution, remote support session execution, production SLA, legal disclosure
  execution, raw child activity custody, provider secrets, status backend
  payload custody, default Ocentra-hosted family data, or raw support bundle
  payloads.
- `production-support-status-backend-queue-audit-persistence-proof` may prove
  support-safe queue audit persistence labels for support runbook, incident,
  contact, upload, privacy/legal, account, and billing support targets across
  requested, authorized, queued, retry-scheduled, audit-ready, failed,
  manual-required, and backend-unavailable states, but it must not claim real
  status backend execution, durable queue storage, retry worker execution, audit
  persistence, public runtime execution, provider execution, support backend
  upload execution, account lookup execution, billing provider contact
  execution, remote support session execution, production SLA, legal disclosure
  execution, raw child activity custody, provider secrets, status backend
  payload custody, default Ocentra-hosted family data, or raw support bundle
  payloads.
- `production-support-status-backend-dead-letter-proof` may prove support-safe
  status backend dead-letter/manual-triage labels for support runbook,
  incident, contact, upload, privacy/legal, account, and billing support targets
  across requested, authorized, dead-lettered, triage-ready, retry-blocked,
  failed, manual-required, and backend-unavailable states, but it must not
  claim real status backend execution, durable queue storage, retry worker
  execution, audit persistence, dead-letter payload custody, public runtime
  execution, provider execution, support backend upload execution, account
  lookup execution, billing provider contact execution, remote support session
  execution, production SLA, legal disclosure execution, raw child activity
  custody, provider secrets, default Ocentra-hosted family data, or raw support
  bundle payloads.
- `production-support-status-backend-runtime-execution-proof` may prove
  support-safe status backend runtime execution labels for support runbook,
  incident, contact, upload, privacy/legal, account, and billing support targets
  across requested, authorized, queued, running, runtime-evidence-ready,
  audit-ready, failed, manual-required, and backend-unavailable states, but it
  must not claim real status backend execution, durable queue storage, retry
  worker execution, audit persistence, dead-letter payload custody, public
  runtime execution, provider execution, support backend upload execution,
  account lookup execution, billing provider contact execution, remote support
  session execution, production SLA, legal disclosure execution, raw child
  activity custody, provider secrets, status backend payload custody, default
  Ocentra-hosted family data, or raw support bundle payloads.
- `production-support-status-backend-payload-custody-proof` may prove
  support-safe status backend payload custody labels for custody boundary,
  retention manual-required, delete request, deletion manual-required,
  audit-export-ready, and backend-unavailable states, but it must not claim
  real status backend execution, durable status backend payload storage,
  payload deletion execution, retry worker execution, audit persistence, public
  runtime execution, provider execution, support backend upload execution,
  account lookup execution, billing provider contact execution, remote support
  session execution, production SLA, legal disclosure execution, raw child
  activity custody, provider secrets, default Ocentra-hosted family data, or raw
  support bundle payloads.
- `production-support-status-backend-redaction-manifest-proof` may prove
  support-safe status backend redaction manifest labels for redaction-ready and
  manual-required states, but it must not claim real status backend execution,
  status backend payload custody, durable payload storage, payload deletion,
  retry worker execution, audit persistence execution, public runtime
  execution, provider execution, support backend upload execution, account
  lookup execution, billing provider contact execution, remote support session
  execution, production SLA, legal disclosure execution, raw child activity
  custody, provider secrets, default Ocentra-hosted family data, or raw support
  bundle payloads.
- `production-support-privacy-legal-disclosure-status-proof` may prove
  support-safe privacy/legal disclosure status labels for requested,
  parent-authorized, legal-review queued/running, parent-notification-ready,
  publication-ready, failed, and manual-required states, but it must not claim
  legal disclosure execution, public runtime execution, support backend upload
  execution, account lookup execution, billing provider contact execution,
  remote support session execution, production SLA, raw child activity custody,
  provider secrets, remote support transcripts, default Ocentra-hosted family
  data, or raw support bundle payloads.
- `production-support-account-sla-status-proof` may prove account lookup,
  billing provider contact, remote support request/session, and production SLA
  status rows as manual-required or not-implemented support boundaries, but it
  must not claim account lookup execution, billing provider contact execution,
  remote support sessions, production SLA commitments, support backend upload
  execution, `family.ocentra.ca` runtime, provider secrets, or child-activity
  custody.
- `production-support-legal-provider-readiness-proof` may prove
  privacy/legal-review, data export/delete runtime, provider-secret custody,
  billing-provider contact, remote-support legal/session boundary, and
  production SLA legal-boundary rows as source-contract, manual-required, or
  not-implemented support boundaries, but it must not claim legal disclosure
  execution, export/delete runtime execution, provider secret custody, billing
  provider contact execution, account lookup execution, remote support sessions,
  production SLA commitments, support backend upload execution, public runtime
  execution, or child-activity custody.
- `production-support-provider-secret-custody-status-proof` may prove
  provider-secret custody-boundary recorded, provider-secret absent, backend
  secret store manual-required, rotation manual-required, revocation
  manual-required, and support-safe audit export readiness rows, but it must not
  claim provider-secret custody, backend secret store execution, rotation
  execution, revocation execution, support backend upload execution, account
  lookup execution, billing provider contact execution, remote support sessions,
  production SLA commitments, default Ocentra-hosted family data, or
  child-activity custody.
- Parent-facing install flow is understandable for non-technical users.
- `mobile-child-agent-capability-proof` package/runtime hooks may reference
  Android debug APK/checksum and iOS simulator/Xcode artifacts, but Play
  signing, TestFlight, App Store, physical-device install, and mobile child
  parity stay manual-required or planned until those release artifacts exist.

## Non-Goals

- Do not claim store distribution, notarization, or full signing before credentials and workflows exist.
- Do not publish production releases from `main`.
- Do not treat every milestone merge as a product release.
- Do not bypass update signature checks in production code.

## Done Signal

The target platform has a real install/update/uninstall path, CI or local packaging proves the artifact, and docs describe the current branch, PR, and production release boundary honestly.
