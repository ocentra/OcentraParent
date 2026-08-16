<!-- agent-capsule -->

> Agent Capsule
> Doc: Data Custody And Local-First Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Data Custody And Local-First Expectations

Ocentra Parent is a local/LAN-first product. The child-device agent and the
parent's own devices are the normal data custody boundary. Ocentra-hosted
services must not become the default store for child activity, raw evidence,
screen analysis, browser history, journals, reports, or parent rules.

## Parent Outcome

- A parent can understand exactly where family data lives: child device, parent
  device, parent-owned cloud storage, third-party notification provider, or
  Ocentra-hosted service.
- A parent can use Ocentra Parent at home over local/LAN without sending child
  activity to Ocentra.
- A parent can optionally configure their own storage provider, such as Google
  Drive, OneDrive, iCloud Drive, Dropbox, a NAS, or a local folder, for backup,
  remote report access, and cross-device continuity.
- A parent away from home receives minimal notifications and can open a parent
  app/portal that reads from local cache, a reachable child agent, or a
  parent-approved external storage source.

## Product Custody Rule

Ocentra-hosted infrastructure may handle:

- Public website, downloads, release metadata, and update manifests.
- Account, subscription, billing, entitlement, and license checks.
- Device registration and route metadata needed for pairing or relay.
- Minimal notification routing metadata and provider delivery state.
- Stateless or short-lived report compilation from a parent-authorized storage
  source.
- Support diagnostics only when the parent intentionally exports or shares them.

Ocentra-hosted infrastructure must not store by default:

- Raw encrypted journals.
- SQLite evidence databases.
- Screenshots, screen-analysis images, or raw visual evidence.
- Browser URL history, page titles, page contents, chat contents, keystrokes, or
  decrypted network payloads.
- Long-term child activity reports or generated summaries.
- Parent rules and approval history as the source of truth.
- Parent-owned external-storage contents or long-lived provider tokens unless a
  later explicit encrypted token-vault feature is designed and approved.

## Storage Locations

| Data class                         | Default location                     | Optional location                        | Ocentra-hosted default |
| ---------------------------------- | ------------------------------------ | ---------------------------------------- | ---------------------- |
| Raw evidence journal               | Child device                         | Parent-owned encrypted backup/export     | No                     |
| SQLite query store                 | Child device                         | Rebuilt from parent-owned export         | No                     |
| Parent rules and approvals         | Child/parent local devices           | Parent-owned sync target                 | No                     |
| Local AI and policy decisions      | Child device                         | Parent-owned reports/export              | No                     |
| Screen-analysis temporary images   | Child device encrypted temp queue    | None by default                          | No                     |
| Generated reports                  | Parent device local cache            | Parent-owned cloud storage if configured | No                     |
| Subscription and entitlement state | Ocentra billing/control-plane system | Signed local entitlement snapshot        | Yes, non-activity only |
| Notification delivery metadata     | Provider/Ocentra route boundary      | Parent device notification history/cache | Minimal only           |

## Remote Parent Access

Remote access must be designed as one of these explicit paths:

1. Parent app reads a local cache on the parent device.
2. Parent app reads encrypted reports or sync bundles from parent-owned storage.
3. Parent app uses an authenticated relay to send typed intents to a reachable
   child-device agent, with the child agent still owning execution.
4. Parent invokes an Ocentra-hosted stateless compiler that reads a
   parent-authorized source and returns a report without retaining source data or
   generated output beyond short operational TTLs.

Remote access must not silently turn Ocentra into a family-data warehouse.

## Security And Transparency

- Every cross-boundary data movement needs a visible parent setting, data-class
  list, destination, retention behavior, and audit event.
- Parent-owned storage connectors must use least-privilege scopes and show which
  provider/account/folder is connected.
- Child safety decisions continue locally when Ocentra services, storage
  providers, notification providers, or billing checks are unavailable.
- Ocentra logs must minimize child details and prefer ids, status, reason codes,
  and delivery state over activity content.
- Stateless report compilation status may reference request ids, source
  connector/cursor refs, status refs, temporary artifact TTLs, deletion
  confirmation, and audit refs, but it must not retain source child evidence or
  generated reports in Ocentra-hosted systems by default.
- Local export/delete runtime status may reference parent-authorized job ids,
  local output refs, checksums, delete request refs, queue refs, and audit refs,
  but it must preserve source local evidence for safety, delete only the
  exported parent-owned output when requested, and avoid Ocentra-hosted copies by
  default.
- Any support bundle must make included data classes obvious before export.
- Support incident handoff must be parent-initiated and parent-approved before
  export; Ocentra must not silently upload or retain child activity as support
  data.
- Support bundles may disclose release/package/service status metadata, support
  redaction summaries, and manual proof references, but must exclude child
  activity, raw URLs, screenshots, journals, SQLite stores, private paths,
  commands, keystrokes, clipboard content, and message contents.
- `production-release-public-docs-status-proof` may disclose only public policy
  text, retention/export/delete process summaries, support runbook status,
  incident/legal disclosure status, redaction summaries, contact-channel
  status, and manual proof references; it must not include child evidence,
  support bundle payloads, account lookup results, billing provider contact
  records, remote support session transcripts, provider secrets, or parent
  rules as public documentation data.
- `production-release-public-surface-publication-proof` may summarize
  `family.ocentra.ca` publication/readiness rows for public release, download,
  update, account, subscription, support, privacy, retention, export/delete,
  incident, and legal surfaces, but it must not include child evidence, raw
  support bundle payloads, provider secrets, account lookup results, billing
  provider contact records, remote support transcripts, or parent rules.
- `production-support-backend-upload-status-proof` may summarize
  parent-consented support upload status rows, redaction refs, audit refs, retry
  refs, abandon refs, failure refs, manual proof refs, and release/package
  runtime refs, but it must not include raw child activity, provider secrets,
  remote support transcripts, account lookup results, billing provider contact
  records, default Ocentra-hosted family data, or raw support bundle payloads.
- `production-support-backend-upload-execution-runtime-proof` may summarize
  parent-consented support upload execution/runtime boundary rows, redaction
  preflight refs, status refs, runtime refs, audit refs, retry refs, abandon
  refs, failure refs, and manual proof refs, but it must not include raw child
  activity, provider secrets, remote support transcripts, account lookup
  results, billing provider contact records, remote support session transcripts,
  production SLA claims, default Ocentra-hosted family data, or raw support
  bundle payloads.
- `production-support-backend-upload-custody-audit-proof` may summarize
  parent-consented support upload custody/audit rows, status refs, runtime refs,
  redaction refs, custody refs, retention refs, delete refs, audit export refs,
  and manual proof refs, but it must not include raw child activity, provider
  secrets, remote support transcripts, account lookup results, billing provider
  contact records, backend-retained support payloads, backend-deleted support
  payload proof claims, remote support session transcripts, production SLA
  claims, default Ocentra-hosted family data, or raw support bundle payloads.
- `production-support-case-resolution-status-proof` may summarize
  parent-consented support case lifecycle/status rows, incident refs, redaction
  refs, audit refs, publication refs, backend upload status/execution refs,
  escalation refs, response refs, closure refs, SLA refs, and manual proof refs,
  but it must not include raw child activity, provider secrets, remote support
  transcripts, account lookup results, billing provider contact records, real
  support backend upload execution, remote support session transcripts,
  production SLA claims, default Ocentra-hosted family data, or raw support
  bundle payloads.
- `production-incident-support-status-proof` may summarize parent-consented
  support incident intake, parent consent, privacy/legal disclosure,
  export/delete request status, incident publication status, and case resolution
  handoff rows with custody refs, status refs, and manual proof refs, but it
  must not include raw child activity, provider secrets, account lookup results,
  billing provider contact records, real support backend upload execution,
  legal execution, remote support session transcripts, production SLA claims,
  default Ocentra-hosted family data, or raw support bundle payloads.
- `production-support-account-sla-status-proof` may summarize account lookup
  request/result status, billing provider contact status, remote support
  request/session status, production SLA status, redaction refs, public status
  refs, support runbook refs, and manual proof refs, but it must not include raw
  child activity, account lookup results, billing provider contact records,
  provider secrets, support backend payloads, payment provider tokens, remote
  support session transcripts, production SLA commitments, default
  Ocentra-hosted family data, or raw support bundle payloads.
- `production-support-legal-provider-readiness-proof` may summarize
  privacy/legal review, data export/delete runtime readiness, provider-secret
  custody boundary, billing provider contact readiness, remote-support
  legal/session boundary, and production SLA legal-boundary rows with source
  refs and manual proof refs, but it must not include raw child activity,
  account lookup results, billing provider contact records, provider secrets,
  payment provider tokens, support backend payloads, remote support session
  transcripts, production SLA commitments, default Ocentra-hosted family data,
  parent rules, or raw support bundle payloads.
- `production-support-provider-secret-custody-status-proof` may summarize
  provider-secret custody-boundary recorded, provider-secret absent, backend
  secret store manual-required, rotation manual-required, revocation
  manual-required, and support-safe audit export readiness rows with
  legal/provider, billing support, redaction, custody audit, rotation,
  revocation, manual proof, and audit export refs, but it must not include raw
  child activity, provider secrets, payment provider tokens, raw support bundle
  payloads, account lookup results, billing provider contact records, support
  backend payloads, remote support session transcripts, production SLA claims,
  default Ocentra-hosted family data, or any claim that Ocentra-hosted services
  executed provider-secret custody, backend secret storage, rotation, or
  revocation.
- `provider-secret-execution-readiness-proof` may summarize provider-secret
  execution boundary, backend secret-store preflight, rotation preflight,
  revocation preflight, operator approval, manual execution, and support-safe
  audit export rows with custody status refs, preflight refs, operator approval
  refs, manual proof refs, and audit refs, but it must not include raw child
  activity, provider secrets, payment provider tokens, raw support bundle
  payloads, account lookup results, billing provider contact records, support
  backend payloads, remote support session transcripts, production SLA claims,
  default Ocentra-hosted family data, or any claim that Ocentra-hosted services
  executed backend secret storage, rotation, revocation, or provider-secret
  delivery.
- `production-support-provider-secret-rotation-revocation-status-proof` may
  summarize provider-secret rotation and revocation requested, preflight-ready,
  manual-required, and audit-export-ready status rows with custody status,
  execution readiness, backend secret-store preflight, operator approval,
  manual proof, and audit refs, but it must not include raw child activity,
  provider secrets, payment provider tokens, raw support bundle payloads,
  account lookup results, billing provider contact records, support backend
  payloads, remote support session transcripts, production SLA claims, default
  Ocentra-hosted family data, or any claim that Ocentra-hosted services executed
  backend secret storage, rotation, revocation, or provider-secret delivery.
- `production-support-data-export-delete-lifecycle-proof` may summarize
  parent-authorized local export/delete requested, authorized, queued, running,
  succeeded, failed, and manual-required lifecycle status rows with local queue,
  runtime, output, delete, redaction/audit, custody, and manual proof refs, but
  it must not include raw child activity, raw support bundle payloads, provider
  secrets, backend upload payloads, public runtime payloads, remote support
  transcripts, production SLA commitments, default Ocentra-hosted family data,
  or any claim that Ocentra-hosted services executed export/delete custody.
- `production-support-delete-executor-proof` may summarize delete executor
  readiness/status rows with delete-request, authorization, redaction/audit,
  custody, source-proof, and manual-proof refs for local export output, support
  backend payload, status backend payload, public runtime payload, and legal
  disclosure payload targets, but it must not include raw child activity, raw
  support bundle payloads, provider secrets, backend upload payloads, status
  backend payloads, public runtime payloads, legal execution payloads, remote
  support transcripts, production SLA commitments, durable queue payloads,
  payload deletion execution, default Ocentra-hosted family data, or any claim
  that Ocentra-hosted services executed export/delete custody.
- `production-support-process-runtime-status-proof` may summarize support
  process requested, parent-consent authorized, privacy/legal queued, redaction
  review running, backend-upload failed, case-resolution succeeded, and
  manual-required runtime status rows plus incident runtime requested,
  authorized, running, evidence-ready, and manual-required rows with
  support-safe status, runtime evidence, audit, and manual proof refs, but it
  must not include raw child activity, raw support bundles, provider secrets,
  account lookup results, billing contact records, backend upload payloads,
  public runtime payloads, incident runtime payloads, remote support
  transcripts, production SLA commitments, or default Ocentra-hosted family
  data.
- `production-support-status-backend-public-runtime-followthrough-proof` may
  summarize status backend/public runtime follow-through rows with support-safe
  status labels, backend handoff refs, public runtime handoff refs, and manual
  proof refs, but it must not include raw child activity, raw support bundles,
  provider secrets, account lookup results, billing contact records, backend
  upload payloads, public runtime payloads, remote support transcripts,
  production SLA commitments, or default Ocentra-hosted family data.
- `production-support-status-backend-execution-queue-proof` may summarize
  status backend execution queue rows with support-safe status labels, queue
  refs, retry policy refs, audit refs, and manual proof refs, but it must not
  include raw child activity, raw support bundles, provider secrets, account
  lookup results, billing contact records, backend upload payloads, status
  backend execution payloads, public runtime payloads, remote support
  transcripts, production SLA commitments, or default Ocentra-hosted family
  data.
- `production-support-status-backend-queue-audit-persistence-proof` may
  summarize status backend queue audit persistence rows with support-safe status
  labels, queue refs, retry policy refs, audit refs, and manual proof refs, but
  it must not include raw child activity, raw support bundles, provider secrets,
  account lookup results, billing contact records, backend upload payloads,
  status backend execution payloads, durable queue payloads, retry worker
  payloads, audit persistence payloads, public runtime payloads, remote support
  transcripts, production SLA commitments, or default Ocentra-hosted family
  data.
- `production-support-status-backend-dead-letter-proof` may summarize status
  backend dead-letter rows with support-safe status labels, queue refs,
  dead-letter refs, retry policy refs, audit refs, and manual proof refs, but
  it must not include raw child activity, raw support bundles, provider secrets,
  account lookup results, billing contact records, backend upload payloads,
  status backend execution payloads, durable queue payloads, retry worker
  payloads, audit persistence payloads, dead-letter payloads, public runtime
  payloads, remote support transcripts, production SLA commitments, or default
  Ocentra-hosted family data.
- `production-support-status-backend-runtime-execution-proof` may summarize
  status backend runtime execution rows with support-safe status labels, queue
  refs, dead-letter refs, retry policy refs, audit refs, runtime evidence refs,
  and manual proof refs, but it must not include raw child activity, raw support
  bundles, provider secrets, account lookup results, billing contact records,
  backend upload payloads, status backend execution payloads, durable queue
  payloads, retry worker payloads, audit persistence payloads, dead-letter
  payloads, status backend payloads, public runtime payloads, remote support
  transcripts, production SLA commitments, or default Ocentra-hosted family
  data.
- `production-support-status-backend-payload-custody-proof` may summarize
  status backend payload custody rows with support-safe status target refs,
  queue refs, audit refs, redaction refs, custody refs, retention refs, delete
  refs, and manual proof refs, but it must not include raw child activity, raw
  support bundles, provider secrets, account lookup results, billing contact
  records, backend upload payloads, status backend payloads, public runtime
  payloads, remote support transcripts, production SLA commitments, durable
  status backend payload storage, payload deletion execution, retry worker
  execution, audit persistence execution, or default Ocentra-hosted family data.
- `production-support-status-backend-redaction-manifest-proof` may summarize
  status backend redaction manifest readiness with support-safe status target,
  queue, audit, redaction manifest, and manual proof refs, but it must not
  include raw child activity, raw support bundles, provider secrets, account
  lookup results, billing contact records, backend upload payloads, status
  backend payloads, public runtime payloads, remote support transcripts,
  production SLA commitments, durable status backend payload storage, payload
  deletion execution, retry worker execution, audit persistence execution, or
  default Ocentra-hosted family data.
- `production-support-status-backend-runtime-closure-proof` may summarize
  source-backed closure refs for status backend runtime execution,
  queue/audit persistence, dead-letter, payload custody, redaction manifest,
  and public-runtime follow-through rows, but it must not include raw child
  activity, raw support bundles, provider secrets, account lookup results,
  billing contact records, backend upload payloads, status backend payloads,
  public runtime payloads, legal execution payloads, remote support transcripts,
  production SLA commitments, durable queue payloads, retry worker payloads,
  audit persistence payloads, dead-letter payloads, durable status backend
  payload storage, payload deletion execution, retry worker execution, audit
  persistence execution, or default Ocentra-hosted family data.
- `production-support-status-backend-durable-queue-runtime-proof` may summarize
  source-backed durable queue runtime boundary refs for queue storage,
  retry-worker, audit-persistence, dead-letter, runtime execution, and runtime
  closure rows, but it must not include raw child activity, raw support bundles,
  provider secrets, account lookup results, billing contact records, backend
  upload payloads, status backend payloads, public runtime payloads,
  provider-secret payloads, legal execution payloads, remote support
  transcripts, production SLA commitments, durable queue payloads, retry worker
  payloads, audit persistence payloads, dead-letter payloads, durable queue
  storage execution, retry worker execution, audit persistence execution, or
  default Ocentra-hosted family data.
- `production-support-status-backend-execution-continuation-proof` may
  summarize source-backed execution continuation refs for durable queue runtime,
  runtime closure, payload custody, redaction manifest, and manual proof rows,
  but it must not include raw child activity, raw support bundles, provider
  secrets, account lookup results, billing contact records, backend upload
  payloads, status backend payloads, public runtime payloads, provider-secret
  payloads, legal execution payloads, remote support transcripts, production
  SLA commitments, durable queue payloads, retry worker payloads, audit
  persistence payloads, dead-letter payloads, durable queue storage execution,
  retry worker execution, audit persistence execution, redaction manifest
  execution, status backend payload custody, default Ocentra-hosted family data,
  or child activity custody.
- `production-support-privacy-legal-disclosure-status-proof` may summarize
  parent-authorized privacy/legal disclosure requested, legal-review
  queued/running, parent-notification-ready, publication-ready, failed, and
  manual-required status rows with privacy policy refs, legal review refs,
  publication refs, support runbook refs, audit refs, failure refs, and manual
  proof refs, but it must not include raw child activity, raw support bundles,
  provider secrets, account lookup results, billing contact records, backend
  upload payloads, public runtime payloads, remote support transcripts, legal
  execution payloads, production SLA commitments, or default Ocentra-hosted
  family data.
- Any future feature that stores family activity in Ocentra infrastructure
  requires a new explicit product, privacy, security, retention, and deletion
  design before implementation.

## Done Signal

A feature that crosses a device, LAN, cloud, provider, or Ocentra-hosted boundary
states what data moves, who owns the destination, how long it is retained, how it
is deleted, what happens offline, and how tests prove Ocentra does not become the
default child-activity store.
