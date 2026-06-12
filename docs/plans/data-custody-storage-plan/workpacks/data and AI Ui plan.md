<!-- agent-capsule -->

> Agent Capsule
> Doc: Data And AI UI Plan
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Data And AI UI Plan

Status: first-pass Manage UI wiring is implemented on `codex/parent-portal-manage-ia`; keep this note as the product/UI reference for the next backend wiring pass.

## Goal

Data and AI should follow the same manage-page discipline as Devices, Activity, and Policy:

- one primary side-panel item per major product area
- shared frame/header/body rhythm
- explicit Family / Per device targeting where child-device state is involved
- no hardcoded child names, no fake production state, and no Vite-owned claims
- typed read models and typed parent intents instead of UI-only settings

Data is mostly a custody, export, retention, and parent-owned storage surface. AI is more involved because it spans child-device local safety AI, parent-portal assistant/report AI, local hardware fit, model catalog, memory/graph state, and optional external provider credits.

The portal still does not execute capture, data export, model inference, child-device safety evaluation, policy evaluation, or enforcement. It renders typed state and sends typed intents to the Rust/Tauri/local/LAN/backend boundaries.

## Current Findings

- `packages/portal-domain/src/parent-portal-nav.ts` currently exposes Data as three side-panel routes: Drives, Export, Audit.
- The same file exposes AI as three side-panel routes: AI Setup, API Keys, Memory Set.
- `packages/portal-domain/tests/contracts.test.ts` owns the route contract; collapsed pages should remove stale aliases instead of carrying compatibility paths.
- `docs/expectations/data-custody.md` says Ocentra-hosted services must not store raw child activity, reports, browser history, screenshots, parent rules, or parent-owned storage contents by default.
- `docs/expectations/sync-export.md` says export/sync is explicit parent action or a preconfigured parent policy, with declared data classes, destination, format, retention, and audit.
- `docs/expectations/evidence-storage.md` says encrypted journal is source of truth and SQLite is a rebuildable query store. Portal reads typed service read models, not local files directly.
- `docs/expectations/ai.md` says local child-device AI is the safety path. Remote/API AI is optional, parent-authorized, and cannot replace child-device local safety evaluation.
- `docs/expectations/parent-assistant-chat.md` says the parent assistant is a workflow layer. It can help explain, report, and prepare actions, but it must cite sources and preview actions instead of silently changing safety behavior.
- `docs/architecture/local-ai-provider-runtime-boundary.md` says current local AI runtime status is status-only: provider unavailable/unconfigured is valid state, model execution is not enabled, and remote providers stay out of the child-device safety path.
- `packages/parent-domain/src/capabilities.ts` has platform capability claims, but it does not yet expose a device hardware profile rich enough for model-fit decisions.
- `docs/architecture/local-ai-and-tabagent-reuse.md` points to TabAgent as a reference for model lifecycle, model cache, execution-provider checks, and memory/graph ideas. The reusable lesson is runtime management shape, not TabAgent UI or extension product behavior.
- TabAgent's model-management references include pull/load/unload/delete, model-state, generation, stop-generation, inference settings, prompt templates, cache progress, manifest state, and provider availability. Ocentra Parent should model those as typed child-device/runtime read models and intents.
- Current Ocentra Parent already has a llama.cpp/llama-cli path in Rust, including local AI runtime status, local AI chat generation, llama.cpp runtime cache, model cache state, download status, generation state, timeout/max-token settings, and acceleration controls such as GPU layers, device, split mode, tensor split, main GPU, fit, op offload, and CPU MoE.

## IA Decision

Recommended side-panel cleanup:

- Collapse Data subitems into one main `Data` manage item.
- Collapse AI subitems into one main `AI` manage item.
- Keep old routes as aliases:
  - Data aliases: `#/drive-connections`, `#/export-retention`, `#/audit-history`.
  - AI aliases: `#/ai-runtime`, `#/api-providers`, `#/memory-settings`.
- Route aliases should select the matching internal tab instead of rendering separate old pages.

Recommended shared page behavior:

- Data uses a Family / Per device target selector when exporting or inspecting child evidence.
- Data storage connector setup is family/portal-level, not per-device.
- AI uses a three-option target selector where needed: Family, Per Device, Portal.
- AI Family means default local AI posture and model/provider policy for the family.
- AI Per Device means child-device runtime, hardware fit, model cache, and capability status.
- AI Portal means parent assistant, report compiler, external provider keys/credits, and parent-device local runtime.
- AI setup is a parent-portal control surface for child-device local AI. The parent portal sends setup/configuration intents and reads progress/status back. The child device owns model cache, runtime load, safety evaluation, and generation execution.

## Shared Surface Pattern

Data and AI should reuse the established manage surface:

- Header: icon, title, info badge, optional right action.
- Top region: target selector plus compact status area.
- Device selector: only when the selected target requires a specific child device.
- Divider: same width/thickness/spacing as Devices and Activity.
- Body tabs: reflective/beveled tabs, square bottom corners on the body panel.
- Body content: real cards/tables/forms that map to product contracts, not decorative badge clusters.

Use the existing multi-choice SVG toggle for compact decisions such as:

- Family, Per Device, Portal
- Off, Manual, Scheduled, Always
- Local Only, Parent Drive, Ask Each Time
- Daily, Weekly, Monthly
- Default, Override
- CPU, GPU, NPU, Unavailable
- Local, External, Disabled

If a control needs more than a compact segmented choice, design a new SVG/control instead of forcing a badge grid.

## Data Product Truth

Data may show:

- parent-owned storage connections
- local folder/export destination state
- Google Drive, OneDrive, iCloud Drive, Dropbox, or NAS connector state when implemented
- exportable data classes
- retention policy
- sync policy
- last export/sync status
- import/rebuild status
- audit events for export, sync, deletion, retention, and support bundle preparation
- stale, unavailable, revoked, permission-required, conflict, and validation-failed states

Data must not show:

- raw screenshots
- raw browser history or page text
- decrypted network payloads
- child chat/form/keystroke content
- parent-owned drive contents unless the parent explicitly grants and requests it
- Ocentra-hosted child activity storage as a normal destination

The parent chooses the destination and data classes. The UI should explain custody and consequences, then send the typed intent. It should not silently choose Google Drive, local folder, cloud, raw evidence, or report format.

## Data Page Tabs

Recommended tabs:

- Storage
- Export
- Retention
- Import
- Audit

### Storage

Purpose: connect and inspect parent-owned storage destinations.

Show:

- destination type: local folder, Google Drive, OneDrive, iCloud Drive, Dropbox, NAS, unavailable
- connection status: connected, not connected, revoked, stale, permission-required, unavailable
- account/folder label
- last successful sync/export
- permission scope summary
- revoke, reconnect, test, and choose-folder actions

Design notes:

- Use cards for providers only when they are repeated provider choices.
- Do not show child data in provider cards.
- Google Drive can be first-class in UI fixtures because the user mentioned it, but it remains parent-owned storage and not Ocentra-hosted storage.
- Connector errors should be visible, not collapsed into generic failure text.

### Export

Purpose: export family or per-device data in a declared format.

Top target behavior:

- Family: export an aggregate across selected devices/data classes.
- Per Device: require a selected child device before enabling export.

Data classes:

- generated reports
- parent rules and schedules
- approvals and ask-parent decisions
- device registry and pairing audit
- notification history
- policy/enforcement audit events
- encrypted journal backup
- SQLite query-store export or rebuild package
- support bundle

Formats:

- encrypted machine-readable backup
- intentionally human-readable parent report
- encrypted support bundle
- audit log export

Actions:

- generate/export now
- schedule export
- save to destination
- download local copy
- verify export

### Retention

Purpose: define how long local and parent-owned exported data lives.

Show:

- local evidence retention
- generated report retention
- remote parent-drive retention
- support bundle retention
- delete-after-export behavior
- imported backup retention
- next cleanup time
- last cleanup outcome

Rules:

- local source remains intact when export fails
- deleting an export does not silently delete source evidence unless a separate parent-confirmed local retention action exists
- import validates schema and records conflicts deterministically

### Import

Purpose: restore or inspect parent-owned backup/export packages.

Show:

- selected package metadata
- schema version
- data classes inside package
- encrypted/not encrypted status
- source provider
- conflict state
- validation result
- dry-run import summary before applying

### Audit

Purpose: show custody-changing events.

Show events for:

- provider connected/revoked
- export generated
- export saved
- export failed
- retention applied
- import validated/applied
- support bundle prepared
- support bundle shared
- delete requested/completed/failed

## Data Read Models And Intents

These names are pre-contract labels for implementation capture. Promote each row into domain contracts before the first production data/AI implementation pass.

Read models:

- `getDataCustodyReadModel`
- `getStorageConnectorReadModel`
- `getExportableDataClassesReadModel`
- `getRetentionPolicyReadModel`
- `getDataAuditReadModel`
- `getImportValidationReadModel`

Parent intents:

- `connectStorageProviderIntent`
- `revokeStorageProviderIntent`
- `testStorageProviderIntent`
- `chooseExportDestinationIntent`
- `generateDataExportIntent`
- `scheduleDataExportIntent`
- `saveExportToDestinationIntent`
- `setRetentionPolicyIntent`
- `validateImportPackageIntent`
- `applyImportPackageIntent`
- `prepareSupportBundleIntent`

All intents need:

- scope: Family or device target
- data classes
- destination
- format
- retention/deletion behavior
- parent actor reference
- audit reason
- dry-run flag where destructive

## Data UI-Check Fake Data

Use explicit fake-data switches only:

- `DATA_UI_CHECK_FAKE_DATA_ENABLED`
- `DATA_UI_CHECK_STORAGE_CONNECTORS`
- `DATA_UI_CHECK_EXPORT_HISTORY`
- `DATA_UI_CHECK_AUDIT_EVENTS`

Fixture examples:

- local folder: ready
- Google Drive: not connected
- OneDrive: revoked
- NAS: unavailable
- last weekly family report: generated but not saved
- encrypted backup: validation-ready fixture

Rules:

- no personal names
- no real email addresses except `.invalid`
- no real child activity content
- no browser URLs except `.invalid`
- no real drive tokens or folder ids

## AI Product Truth

AI has three separate meanings and the UI must keep them apart.

Child-device local AI:

- runs on the child device
- consumes typed local evidence plus parent rules
- returns typed safety results and reason codes
- is local-only for child safety
- is required before AI-backed blocking/asking/timers can be trusted

Parent-portal assistant/report AI:

- helps the parent understand, query, summarize, and prepare actions
- may use local parent runtime or optional external provider
- must cite sources and custody
- must preview actions before changes
- must not become the child safety engine

External/API AI:

- optional
- parent-authorized
- useful for assistant/report compilation only
- may use purchased credits or BYOK later
- cannot receive child activity by default
- cannot be the fallback for child safety decisions

## AI Page Tabs

Recommended tabs:

- Runtime
- Hardware
- Models
- Inference
- Templates
- Providers
- Memory
- Assistant
- Activity
- Audit

### Runtime

Purpose: show honest AI readiness for the selected target.

Target behavior:

- Family: default AI posture and child-device requirements.
- Per Device: selected device local runtime status.
- Portal: parent portal assistant/report runtime status.

Show:

- provider id
- model id/reference
- load state
- readiness state
- execution state
- privacy mode
- adapter boundary
- provider source
- capability flags
- resource class
- degraded/unavailable reason
- last checked time

Current baseline:

- status-only is valid
- unavailable/unconfigured is valid
- execution disabled is valid
- no model output is valid

Do not hide unavailable state. That is useful product truth.

### Hardware

Purpose: explain whether the selected device can run local AI.

This is also a Devices Info gap. Devices should eventually show the same hardware/capability profile when a device is clicked.

Required hardware profile fields:

- platform and OS version
- architecture
- CPU name/class
- CPU core count
- total RAM
- available RAM
- GPU name/vendor
- total VRAM where available
- available VRAM where available
- NPU/accelerator presence where available
- battery/power state for mobile/laptop
- thermal/performance mode if available
- disk free space for model cache
- supported local runtime backends
- supported quantization/runtime formats
- capture/policy/enforcement capability summary
- last hardware probe time
- permission-limited/unavailable reason

Mobile notes:

- Android and iOS must not claim desktop-level local model support until proven.
- Mobile may support small local classifiers or tiny SLMs but can still be RAM, battery, thermal, or entitlement constrained.
- UI should show `not checked`, `too small`, `fits tiny model`, `fits small model`, `fits configured model`, `permission-limited`, `unsupported`, and `stale` as typed states.

### Models

Purpose: help the parent pick or inspect local model candidates without hardcoding a fake top list into app source.

Model catalog principles:

- no source-code hardcoded "top 10"
- catalog records are data, not UI literals
- each candidate has source, license, size, tasks, memory estimate, local runtime support, and last verified timestamp
- Hugging Face candidate lists should be fetched/imported through a reviewed catalog path, not embedded in the portal
- Ocentra-tested recommendations can be a curated catalog after validation
- UI-check fixtures can use clearly fake generic candidates until real catalog integration exists
- llama.cpp/GGUF is the first product path. ONNX, WebGPU, Transformers.js, and extension/browser runtimes are TabAgent references only, not first-pass Ocentra Parent UI promises.

Candidate row fields:

- model display name
- model source/ref
- task category: safety classifier, summarizer, assistant, vision/screen, multimodal, embedding
- parameter class
- quantization/profile
- download size
- RAM estimate
- VRAM estimate
- CPU-only fit
- GPU/NPU fit
- mobile fit
- license
- local-only capable
- offline capable
- tested platforms
- known limitations
- recommended target: portal assistant, child safety, report summary, screen analysis, not recommended
- cache state: not cached, download disabled, download in progress, cache ready, degraded, corrupted, storage error
- manifest integrity: unavailable, unchecked, verified, failed, corrupted
- runtime compatibility: llama.cpp supported, unsupported platform, missing runtime, missing artifact, model id invalid

Model actions:

- refresh catalog
- check fit
- download/install local model to the target child device or portal device
- remove local model
- set family default
- set per-device override
- test with safe built-in diagnostic prompt
- show why unavailable
- load model
- unload model
- repair/revalidate cache
- cancel download when supported
- retry failed download

Important:

- A "recent Hugging Face top models" area should be a live/catalog-derived panel, not a fixed list in the app.
- If a model list is shown during UI checks, label it as UI-check fixture data and remove/replace it before production wiring.
- Hugging Face checking belongs behind a reviewed catalog/cache path. The UI should request catalog refresh/import and render typed results; it should not scrape Hugging Face from Vite.

### Inference

Purpose: manage safe, typed inference settings for the selected target without exposing every low-level flag as normal parent UI.

Target behavior:

- Family: default inference profile for child safety/report tasks.
- Per Device: device override, disabled until a device is selected.
- Portal: parent assistant/report profile.

Show simple profiles first:

- Safe Default
- Faster
- More Careful
- Low Memory
- Advanced

Show advanced fields only behind an Advanced panel:

- max output tokens
- timeout
- temperature
- top p
- top k
- repetition penalty
- context length if supported
- CPU/GPU/NPU resource preference
- llama.cpp GPU layers
- llama.cpp device
- split mode
- tensor split
- main GPU
- fit and fit target
- op offload
- CPU MoE / CPU MoE layers

Rules:

- Child-safety defaults should stay deterministic and conservative.
- The UI should validate settings against target hardware before sending them.
- Settings should be versioned and scoped: family default, per-device override, or portal assistant.
- Invalid settings should produce typed config-invalid state, not silently fall back.
- Vite should not own inference settings. It should render the read model and send typed intents.

Suggested setting read models:

- `getAiInferenceProfileReadModel`
- `getAiInferenceOverrideReadModel`
- `getAiRuntimeCompatibilityReadModel`

Suggested parent intents:

- `setFamilyAiInferenceProfileIntent`
- `setDeviceAiInferenceOverrideIntent`
- `setPortalAssistantInferenceProfileIntent`
- `validateAiInferenceSettingsIntent`
- `resetAiInferenceOverrideIntent`

### Templates

Purpose: manage prompt/template versions used by child safety, report generation, screen analysis, and parent assistant.

Template categories:

- child safety decision
- screen analysis summary
- app/game digest
- browser/domain summary
- network digest
- daily/weekly/monthly report compilation
- parent assistant answer
- JSON/typed-output repair
- diagnostic test prompt

Show:

- active template version
- task category
- target scope
- source: bundled, parent-custom, Ocentra-updated, unavailable
- output schema expected
- last validation result
- last failed reason
- rollback target

Rules:

- Prompt templates are product contracts, not casual text boxes.
- A parent may choose/customize where the product permits, but child-safety templates must remain schema-bound and validated.
- Template edits should create draft/preview state first.
- Every AI result should carry the prompt/template version used.
- Raw child private content should not appear inside template preview fixtures.

### Providers

Purpose: configure provider boundaries for local and optional external AI.

Show:

- local provider status
- local model cache state
- external provider connection state
- Ocentra external AI credit state
- BYOK state if supported later
- quota/credit warnings
- privacy/custody boundary
- allowed use: assistant/report only, child safety disabled, or local child safety

External provider notes:

- API provider keys and Ocentra external AI credits belong to account/billing infrastructure too.
- AI page can show provider readiness and credit state, but purchasing credits likely routes to Account/Plan.
- Remote/API AI must not be presented as the normal child safety runtime.

### Memory

Purpose: inspect and control local memory/graph indexes.

Show:

- index state: disabled, building, ready, stale, corrupted, unavailable
- source scope: child local evidence, parent device cache, parent-owned export, unavailable
- evidence reference coverage
- last build time
- last failed reason
- storage size
- retention
- delete/rebuild/export actions

Rules:

- memory/graph references must cite source evidence, parent rules, or parent actions
- memory cannot create policy truth by itself
- graph truth must not drive blocking without current source evidence and parent rule context

### Assistant

Purpose: parent-facing assistant/report workflow.

Show:

- assistant runtime: local portal, external provider, unavailable
- provider privacy boundary
- source/custody selector
- available quick actions: report, browser state, rules, AI setup, drives, support/API
- recent assistant sessions
- action previews
- cited source list
- unsupported/missing routes

Rules:

- assistant can explain and prepare typed actions
- assistant cannot silently apply rule, export, provider, or storage changes
- assistant cannot replace child-device local safety AI

### Activity

Purpose: show AI runtime and decision activity without mixing it into setup controls.

This also creates a follow-up gap for the Activity page. Activity currently covers Reports, Screen, App Use, Browser, Games, and Network. It should also have an AI activity view because parents and developers need to see what local AI did, when it was unavailable, and which model/template was used.

Show:

- AI jobs: queued, running, complete, failed, timed out, cancelled, unavailable
- job class: child safety, screen analysis, report compilation, parent assistant, diagnostic test, memory rebuild
- target: family, device, or portal
- model id/reference
- provider id
- runtime reference
- template version
- inference profile version
- started/completed time
- duration
- token counts where available
- unavailable/degraded reason
- evidence refs or report refs, not raw private prompts
- scheduler state: child-safety priority, parent-assistant queued, provider busy, provider overloaded
- stop/halt/cancel outcome where supported

Rules:

- Activity AI is read-model only.
- It should not expose raw prompts containing child private data.
- It should not imply AI was used for enforcement unless the typed decision/audit says so.
- It should make unavailable states visible: model missing, runtime missing, execution disabled, cache corrupted, prompt too large, timeout, invalid output, provider overloaded.

### Audit

Purpose: show AI-relevant state changes and sensitive actions.

Show:

- provider configured/revoked
- model catalog refreshed
- model installed/removed
- default model changed
- per-device override changed
- model load/unload requested
- model load failed
- inference profile changed
- prompt/template version changed
- runtime probe performed
- memory rebuilt/deleted/exported
- assistant request sent
- external provider request authorized
- external provider request denied
- local AI unavailable during preview

## AI Read Models And Intents

These names are pre-contract labels for implementation capture. Promote each row into domain contracts before the first production data/AI implementation pass.

Read models:

- `getAiRuntimeReadModel`
- `getAiHardwareProfileReadModel`
- `getAiModelCatalogReadModel`
- `getAiModelFitReadModel`
- `getAiModelCacheReadModel`
- `getAiModelLoadReadModel`
- `getAiInferenceProfileReadModel`
- `getAiPromptTemplateReadModel`
- `getAiJobActivityReadModel`
- `getAiProviderReadModel`
- `getAiMemoryGraphReadModel`
- `getAiAssistantReadModel`
- `getAiAuditReadModel`

Parent intents:

- `runAiRuntimeProbeIntent`
- `runAiHardwareCheckIntent`
- `refreshAiModelCatalogIntent`
- `checkAiModelFitIntent`
- `installLocalAiModelIntent`
- `removeLocalAiModelIntent`
- `loadLocalAiModelIntent`
- `unloadLocalAiModelIntent`
- `cancelLocalAiModelDownloadIntent`
- `repairLocalAiModelCacheIntent`
- `setAiInferenceProfileIntent`
- `validateAiInferenceSettingsIntent`
- `setAiPromptTemplateIntent`
- `rollbackAiPromptTemplateIntent`
- `stopAiGenerationIntent`
- `setFamilyAiDefaultIntent`
- `setDeviceAiOverrideIntent`
- `configureExternalAiProviderIntent`
- `revokeExternalAiProviderIntent`
- `purchaseExternalAiCreditsIntent`
- `rebuildAiMemoryGraphIntent`
- `deleteAiMemoryGraphIntent`
- `startParentAssistantSessionIntent`

All intents need:

- target: Family, device, or portal
- custody mode
- provider/model reference
- model cache reference where applicable
- prompt/template version where applicable
- inference profile/version where applicable
- parent actor reference
- audit reason
- explicit external-provider consent where remote/API AI is involved
- dry-run flag where the action can affect local model/cache/memory state

## TabAgent-Informed Runtime Management

Use TabAgent as a reference for operational states, but keep Ocentra Parent simpler:

- llama.cpp/llama-cli is the first runtime path.
- model cache is separate from encrypted evidence storage.
- runtime setup happens on the target device role, not in the React portal.
- child safety jobs have priority over parent assistant/report jobs.
- one physical device should not load duplicate local models for child safety and parent assistant unless a future advanced mode explicitly supports it.

Required UI status states:

- runtime unavailable
- runtime binary missing
- runtime binary installed
- model source unconfigured
- model not cached
- model download disabled
- model download in progress
- model download complete
- model download failed
- model cache ready
- model cache degraded
- model cache corrupted
- model load requested
- model loading
- model loaded
- model load failed
- execution disabled
- execution dry-run-ready
- execution running
- generation complete
- generation failed
- generation timed out
- provider overloaded
- provider busy with child-safety job
- parent assistant/report queued

Required UI feedback:

- download progress
- cache verification/integrity state
- current loaded model
- requested model vs active model
- load/unload progress or last state
- generation/test progress where streaming is unavailable
- stop/halt result
- typed unavailable/degraded reason
- current runtime resource class

Do not copy TabAgent broad provider matrix into first-pass UI. ONNX, WebGPU, Transformers.js, and browser-extension model execution remain reference material unless we later decide they are needed.

## Device Info Hardware Gap

Devices should add a real Info tab section for machine capability, because AI cannot make a responsible model-fit UI without it.

The same read model should feed:

- Devices > Info
- AI > Hardware
- AI > Models fit badges
- Policy capability state
- Activity unavailable/degraded explanations
- Account/Plan device support warnings if a platform cannot run required features

Device Info should eventually show:

- OS/platform
- CPU
- CPU cores
- RAM
- GPU
- VRAM
- NPU/accelerator
- disk/cache capacity
- battery/thermal state
- local AI runtime capability
- screen/browser/app/network capture capability
- policy/enforcement capability
- LAN reachability and stale/offline state

Do not fake "local device" or hardcode device labels. If hardware has not been reported by the real scan/service, show empty, unknown, not checked, stale, or unavailable.

## AI UI-Check Fake Data

Use explicit fake-data switches only:

- `AI_UI_CHECK_FAKE_DATA_ENABLED`
- `AI_UI_CHECK_HARDWARE_PROFILES`
- `AI_UI_CHECK_MODEL_CATALOG`
- `AI_UI_CHECK_PROVIDER_STATUS`
- `AI_UI_CHECK_MEMORY_GRAPH`
- `AI_UI_CHECK_ASSISTANT_SESSIONS`

Fixture examples:

- device D001: Windows laptop, 16 GB RAM, integrated GPU, CPU-only small model fit
- device D002: Android phone, 6 GB RAM, tiny model only, battery caution
- portal device: desktop with discrete GPU, assistant model fit
- local provider: status-only/unconfigured
- model catalog: fake generic candidates, clearly labeled UI-check data
- external provider: not configured
- credit state: fixture only, not billing truth

Rules:

- no hardcoded child names
- no real model downloads
- no real API keys
- no real child prompts
- no real screenshots or browser content
- no fake provider success unless the fixture name says UI-check fake

## Cross-Surface Coordination

Data coordinates with:

- Activity Reports: generated reports can be exported/saved.
- Policy: parent rules and approvals can be exported/backed up.
- Account: plan/entitlement may gate device count or external AI credits, not local custody.
- Portal Support: support bundles are a data export type with stricter redaction.

AI coordinates with:

- Devices: hardware profile and runtime capability.
- Activity: read models for screen/app/browser/game/network views.
- Activity: add an AI activity tab/read model for local AI jobs, safety decisions, unavailable states, prompt/template versions, model/runtime references, and scheduler outcomes.
- Policy: local AI safety result references and dry-run preview.
- Data: AI memory/graph export/delete/rebuild and assistant/report source custody.
- Account: external AI credits and BYOK/provider account state.

## Open Decisions

- Should Data and AI fully collapse in side panel now, or only route-alias internally first?
- Which storage provider is first: local folder, Google Drive, or both?
- Which data classes are exportable in the first UI slice?
- Which export format is first: encrypted backup or human-readable report?
- Is scheduled export in MVP, or generate/save only?
- Where should external AI credit purchase live: Account Plan only, AI Providers only, or both with deep links?
- Which local runtime backend is first for Windows?
- Which catalog source owns model recommendations: Hugging Face import, Ocentra curated catalog, or local-only curated JSON?
- What hardware-fit thresholds count as tiny, small, configured, and unsupported?
- How much mobile local AI should be promised before Android/iOS proof exists?
- Should model downloads live on child device, parent portal device, or both?

## Non-Goals For This Slice

- no UI implementation
- no route contract changes
- no Google Drive OAuth implementation
- no Stripe/credit implementation
- no Cloudflare backend implementation
- no model downloads
- no model execution
- no remote AI calls
- no child-device safety behavior change
- no hardcoded Hugging Face top list in app/runtime code
- no fake data except explicit UI-check fixtures
