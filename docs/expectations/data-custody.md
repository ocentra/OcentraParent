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
- Any future feature that stores family activity in Ocentra infrastructure
  requires a new explicit product, privacy, security, retention, and deletion
  design before implementation.

## Done Signal

A feature that crosses a device, LAN, cloud, provider, or Ocentra-hosted boundary
states what data moves, who owns the destination, how long it is retained, how it
is deleted, what happens offline, and how tests prove Ocentra does not become the
default child-activity store.
