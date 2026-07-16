<!-- agent-capsule -->

> Agent Capsule
> Doc: Parent Assistant Chat Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Parent Assistant Chat Expectations

Parent assistant chat is a parent-facing workflow layer. It helps a parent ask
plain-language questions, start guided actions, review reports, and prepare rule
changes without hunting through every portal control. It must stay grounded in
typed local evidence, parent-owned sources, and explicit action previews.

This expectation extends [portal](portal.md), [AI](ai.md),
[data custody](data-custody.md), [evidence storage](evidence-storage.md),
[policy](policy.md), [contracts](contracts.md), and the V4
[parent-owned reports and optional assistant](roadmap-v4-parent-owned-reports-optional-assistant.md)
milestone.

## Product Outcome

Parent outcome:

- A parent can open MIA, ask what a child has been doing, request a report,
  understand a block/allow decision, or start a rule change from one chat
  surface.
- Quick actions make common parent tasks clickable: new chat, report, browser
  state, rules, AI setup, drives, and support/API.
- Quick action clicks appear in the chat as MIA-guided choices first. When the
  parent chooses an option or types, the resulting prompt is shown as parent
  input and passed to the assistant runtime.
- Follow-up suggestions stay inside the chat context and update from the latest
  assistant answer when a runtime is connected.
- Chat history is grouped by starter category so a parent can find previous
  report, rules, browser, AI setup, drives, or support conversations.
- Every answer that claims child activity, policy state, model state, custody,
  or support status cites typed source references or says the source is missing.

Child-device outcome:

- The child-device agent stays the source of truth for local evidence, policy,
  capture capability, local AI safety status, and enforcement eligibility.
- The child-device agent exposes typed summaries and typed action previews. It
  does not execute raw assistant text.
- Any child-device write, rule change, approval, support bundle, or diagnostic
  export is a schema-valid parent intent with explicit confirmation and audit.

## Boundary

The portal renders chat UI and sends typed parent intents. It does not run model
providers, read child evidence files directly, scan browser state, evaluate
policy, or execute enforcement.

The parent-side Rust app or local parent agent owns assistant runtime behavior:
thread persistence, provider selection, prompt assembly, evidence authorization,
streaming, action-intent validation, audit, and redaction.

Child-device agents own child-local data and effects: evidence summaries,
capability status, rule context, policy preview, policy application, local AI
status, support bundle preparation, and enforcement handoff.

Remote/API AI is optional and secondary. It may compile parent-authorized
reports or answer questions from approved source references, but it must not
replace local child-device safety evaluation or become default child-activity
custody.

## Contract Ownership

When implementation starts, define Rust-owned contracts and route snapshots
before runtime or UI code depends on assistant shapes. Effect Schema may remain
only for untrusted TypeScript edges or generated validation edges.

Preferred ownership boundary:

- Current TS scaffold names in `@ocentra-parent/ai-domain/parent-assistant`
  are migration surfaces, not product truth.
- Product assistant contracts belong in `crates/schema`,
  `crates/parent-runtime-core`, or the owning Rust assistant/AI runtime crate
  because they are parent, family, child-scope, evidence, action-intent, and
  provider-status product contracts.
- Add a Rust assistant runtime/domain crate later only if assistant behavior
  outgrows parent runtime ownership or needs a cross-product Rust boundary.
- Keep route ids, DOM ids, layout tokens, and static portal copy in generated
  bridge DTOs plus pure presentation helpers.
- Put parent UI actions and route snapshots in the Rust parent facade/bridge
  path. Parent/child WebSocket or HTTP transport contracts belong in Rust
  protocol/runtime crates, not TS package authority.
- Mirror Rust-crossing transport shapes in `crates/agent-protocol` only when
  the Rust-owned product contract is explicit and test-backed.
- Child-device feature requests must name the expected Rust contract, generated
  bridge/DTO shape when UI consumes it, platform scope, evidence source,
  failure state, and validation gate.

## Data Structures

Expected assistant contract families:

- `AssistantThread`: thread id, family id, parent actor id, child scope,
  starter category, title, status, source/custody mode, created time, updated
  time, archived time, and last run state.
- `AssistantThreadGroup`: category id, category label, thread ids, latest
  thread summary, unread/error state, and source availability.
- `AssistantMessage`: message id, thread id, role, created time, state, body
  blocks, source references, action options, run id, and redaction state.
- `AssistantMessageBlock`: plain text, cited text, choice group, action preview,
  warning, error, code/debug snippet, or source/custody disclosure.
- `AssistantQuickAction`: quick action id, display title, starter prompt
  template id, child data requirements, parent confirmation requirement, and
  allowed follow-up categories.
- `AssistantChoice`: label, prompt template id, resolved prompt preview,
  required context, and next action type.
- `AssistantPromptTemplate`: template id, version, category, allowed evidence
  scopes, provider capability requirements, retention policy, and audit label.
- `AssistantRun`: run id, thread id, provider, model reference, prompt template
  version, permitted source refs, streaming state, failure state, token/cost
  metadata where available, and retention state.
- `AssistantEvidenceRef`: child id, device id, evidence type, evidence id,
  time window, custody label, source freshness, redaction state, and human
  summary.
- `AssistantActionIntent`: intent id, category, target child/device, typed
  payload, source refs, risk level, preview requirement, confirmation state,
  and audit reason.
- `AssistantActionPreview`: intent id, expected changes, affected child/device,
  evidence refs, rule refs, failure risks, rollback notes, and confirmation
  requirements.
- `AssistantProviderStatus`: local/API/provider id, configured state,
  available state, capability flags, privacy mode, degraded reason, and last
  checked time.
- `AssistantComposerInput`: text, optional transcript text, input source,
  quick action context, thread id, parent actor id, and idempotency key.

Do not store raw microphone audio, raw screenshots, raw browser contents,
decrypted payloads, child chat contents, or raw evidence blobs in assistant
threads by default.

## Parent Portal Frontend

The chat route needs a separate assistant view, not the regular settings page
reused with different labels.

Required UI structure:

- Shared outer chrome and reusable frame art for side panel and main panel.
- Assistant side panel with Quick Action and History tabs.
- New Chat as the first Quick Action row.
- Title-only quick action rows: Report, Browser State, Rules, AI Setup, Drives,
  and Support/API.
- Main chat header centered to the page, with MIA badge and one side-panel
  toggle control.
- Collapsible/copyable message bubbles with sender labels in the bubble header.
- MIA-guided choice groups rendered inside assistant bubbles, not as unrelated
  side-panel content or bottom badges.
- Parent messages shown only after the parent chooses a prompt or types one.
- Follow-up questions inside the chat area, above the composer, with no more
  than four visible suggestions and responsive wrapping.
- Composer pinned to the bottom of the chat frame with text input, send, and
  voice-to-text affordance.
- Thin draggable visual divider between scrollable chat history and composer
  area.
- Thread history grouped by starter category and source availability.
- Empty, unavailable, connecting, streaming, failed, stale, and cited-answer
  states.

Until backend APIs exist, the UI may keep scaffold chat state in memory only.
Scaffold messages must not pretend to be live child evidence or live model
answers.

## Parent Rust App API

The parent Rust app or local parent agent should expose typed commands and
events to the portal.

Current migration-era command names:

- `agent.parent-assistant.thread.list`
- `agent.parent-assistant.thread.create`
- `agent.parent-assistant.thread.open`
- `agent.parent-assistant.thread.archive`
- `agent.parent-assistant.message.send`
- `agent.parent-assistant.run.cancel`
- `agent.parent-assistant.quick-action.start`
- `agent.parent-assistant.action.preview`
- `agent.parent-assistant.action.confirm`
- `agent.parent-assistant.provider.status.get`

Current migration-era event names:

- `agent.parent-assistant.thread.updated`
- `agent.parent-assistant.message.accepted`
- `agent.parent-assistant.run.started`
- `agent.parent-assistant.message.delta`
- `agent.parent-assistant.message.completed`
- `agent.parent-assistant.action.previewed`
- `agent.parent-assistant.action.confirmed`
- `agent.parent-assistant.provider.degraded`
- `agent.parent-assistant.error.reported`

Current scaffold behavior:

- `@ocentra-parent/agent-protocol-domain` exposes migration-era command and
  event names plus parent-assistant payload field constants; it must not remain
  product authority once Rust-owned bridge/protocol replacements are live.
- `crates/agent-protocol` mirrors the command/event names and serializable
  parent-assistant composer/action/scaffold status structs.
- `crates/agent-service` accepts the parent-assistant commands over the real
  Rust transport dispatcher and returns typed scaffold/degraded events with
  `assistantBackendState=scaffold-only` and
  `reason=parent-assistant-backend-not-connected`.
- The scaffold is intentionally not a model answer, not child activity, and not
  rule execution. It is a real transport proof and a backend TODO marker.

The parent agent must:

- Persist thread metadata, messages, action previews, and audit refs in a
  parent-owned local store or parent-owned storage connector.
- Resolve allowed source refs before prompt assembly.
- Refuse prompt assembly when requested evidence scope is unavailable,
  unauthorized, stale, or outside custody rules.
- Route local provider/API provider calls through explicit provider adapters.
- Stream assistant output as typed deltas and validate completed output before
  it becomes an answer.
- Convert any requested change into a typed `AssistantActionIntent`.
- Require preview and parent confirmation before child-device writes.
- Redact secrets, local private paths, raw evidence payloads, and unsupported
  child content from prompts, logs, thread export, and copy/debug output.
- Record prompt template version, model/provider version, source refs,
  retention state, and failure state.

## Child-Device Contract APIs

Each child-device platform slice needs its own feature request before
implementation. A feature request must name platform scope, data scope, trust
boundary, request/response shape, failure behavior, and validation.

Required child-device contract families:

- `ChildAssistantCapabilityStatusRequest` and
  `ChildAssistantCapabilityStatusResult`: report evidence adapters, local AI
  status, policy preview support, enforcement eligibility, support bundle
  support, stale/offline state, and platform limitations.
- `ChildEvidenceSummaryQuery` and `ChildEvidenceSummaryResult`: return typed
  summaries for a parent-approved time window, child, device, source type, and
  custody boundary. Return refs and summaries, not raw evidence blobs.
- `ChildReportWindowQuery` and `ChildReportWindowResult`: return daily, weekly,
  custom-window, and incident report source refs that the parent assistant can
  cite or pass to a report compiler.
- `ChildRuleContextQuery` and `ChildRuleContextResult`: return current rules,
  policy versions, matching rule refs, schedule state, override state, and
  explainable allow/block/ask-parent context.
- `ChildPolicyActionPreviewRequest` and
  `ChildPolicyActionPreviewResult`: preview a proposed rule or approval change
  from a typed parent intent without applying it.
- `ChildPolicyActionCommitRequest` and
  `ChildPolicyActionCommitResult`: apply only a confirmed, schema-valid action
  preview and emit audit evidence.
- `ChildSupportBundlePrepareRequest` and
  `ChildSupportBundlePrepareResult`: prepare diagnostics or support bundles
  with redaction, custody label, expiration, and parent confirmation.

Child agents must never accept raw assistant prose as command input. They accept
only schema-valid child contract requests from authenticated parent-control
paths.

## Quick Action Behavior

Quick actions are prompt starters, not navigation shortcuts.

Report:

- MIA asks whether the parent wants today, this week, a custom window, a blocked
  activity report, or a source/custody explanation.
- Resulting prompts query report windows and evidence summaries.

Browser State:

- MIA asks whether the parent wants supported browsers, unmanaged browser risk,
  recent browser evidence, or setup help.
- Resulting prompts query browser evidence and capability status.

Rules:

- MIA asks whether the parent wants to explain a rule, change a rule, allow
  something, block something, or review conflicts.
- Resulting prompts query rule context and may create action intents.

AI Setup:

- MIA asks whether the parent wants local AI status, provider setup, model
  readiness, privacy mode, or degraded states.
- Resulting prompts query provider status and local AI capability.

Drives:

- MIA asks whether the parent wants parent-owned storage status, export custody,
  sync health, or report source location.
- Resulting prompts query parent-owned storage connectors and custody state.

Support/API:

- MIA asks whether the parent wants diagnostics, route status, API/provider
  health, or a support bundle.
- Resulting prompts query support status and may create support-bundle intents.

## Voice Input

Voice-to-text is a parent-device input feature. It produces transcript text in
`AssistantComposerInput` and should be labeled as transcript input. Raw audio is
not stored by default. Child devices do not receive raw parent audio.

## Failure Behavior

- Child device offline: chat can use cached parent-owned thread history, but new
  child evidence questions return offline/stale state.
- Evidence missing: MIA must say the source is missing and avoid claiming
  activity understanding.
- Provider unavailable: MIA must show provider degraded/unavailable and keep
  local child safety unaffected.
- Action preview unavailable: rule/support changes cannot be confirmed.
- Invalid assistant output: reject the output, record a typed failure, and show
  a failed answer state.
- Unauthorized source: refuse prompt assembly and show custody/permission
  reason.
- Remote/API failure: degrade to local-only explanation, cached report source,
  unknown, or ask-parent. Do not disable child-device local evaluation.

## Validation Gates

- Rust serialization, round-trip, and generated-artifact drift tests for every
  assistant, parent-agent, and child-device product contract.
- TypeScript parser tests only for generated validation or untrusted edge
  decoders.
- Rust protocol parity tests for every Rust-crossing transport
  command/event/result.
- Real local transport tests for thread create, message send, streaming, cancel,
  action preview, action confirm, provider degraded, and child offline states.
- Stored-evidence integration tests that build assistant source refs from real
  journal/query-store read models.
- Policy preview tests proving raw assistant text cannot apply rules and cannot
  override stricter parent rules.
- Data-custody tests proving remote/API assistant requests include only
  parent-approved refs and do not retain child activity by default.
- Portal Playwright tests for new chat, quick action choices inside MIA bubbles,
  history tab, follow-ups, composer, responsive side-panel hide/show, copy,
  collapse, and degraded states.
- Browser console check for warnings/errors on the assistant route.

## Non-Goals

- Do not run child-safety AI in the web portal.
- Do not let assistant text directly change rules, approvals, support bundles,
  exports, timers, or enforcement.
- Do not upload child activity to Ocentra-hosted services by default.
- Do not store raw child evidence in chat threads by default.
- Do not claim live child activity when the child agent is offline, unsupported,
  stale, or unavailable.
- Do not create duplicate chat implementations in unrelated portal modules.
- Do not leave old assistant UI paths active after the dedicated chat route
  becomes the supported surface.

## Done Signal

The assistant chat is ready for backend implementation when the portal can
render Rust-owned snapshots for the chat shell and scaffold states, the parent
runtime contracts can create threads and process messages, child-device feature
requests define the evidence and action-preview APIs, and validation proves that
assistant output is cited, typed, custody-aware, and unable to bypass local
child-device safety decisions.
