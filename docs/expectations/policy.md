<!-- agent-capsule -->

> Agent Capsule
> Doc: Policy Feature Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Policy Feature Expectations

Policy features define what parents want the local child-device AI evaluator to allow, limit, warn about, block, or send back for parent approval.

Parent surfaces author policy rules and expectations. The child-device agent owns policy validation, conflict resolution, local AI integration, timers, dry-run previews, and enforcement handoff.

Ocentra does not hard-code the household's value judgments. It provides typed
targets, evidence categories, preview behavior, and safe execution boundaries;
parents decide which categories, apps, sites, schedules, and actions apply to
their child.

## Roadmap Scope

V0.6 defines parent, child, device, rule, schedule, permission-request, local AI decision, and policy decision contracts.

V0.7 runs the evaluator against captured evidence, parent rules, and local AI output. It must support dry-run before enforcement.

V5 turns rule management into a parent-facing product with family setup, child profiles, schedules, reports, and audit history. V5 improves authoring and sync; it does not move evaluation into the portal.

## Parent Outcome

- Parent can configure common rules for apps, sites, domains, categories, schedules, time budgets, permission requests, and overrides without editing files.
- Parent can choose whether a category is allowed, warned, time-limited,
  parent-review, or blocked for a child profile and schedule.
- Parent can preview what a rule would do before enabling enforcement.
- Parent can see which evidence, parent rule, local AI result, schedule, and conflict-resolution reason produced a decision.
- Parent can approve, deny, or time-box parent-review requests through typed approvals that the child-device agent validates locally.

## Child-Device Outcome

- The child-device agent evaluates typed rules against typed local evidence.
- The evaluator produces deterministic decisions even when local AI output is ambiguous.
- The evaluator records policy decision events with evidence references, rule references, and local AI references when AI contributed.
- The evaluator can run in dry-run mode and explain what would happen without enforcing it.

## Platform Scope

- Windows is first for policy evaluation tied to capture and enforcement.
- Other child-device platforms must reuse the same contracts but claim support only after platform-specific validation exists.
- Web is a rule-authoring, preview, and explanation surface. It does not evaluate policy, run timers, execute scripts, or enforce rules.

## Data Scope

Policy input may include:

- Parent account, family, child profile, and device references.
- App, process, window, URL, domain, category, video, channel, and recent activity evidence when those capture contracts exist.
- App/game session summaries, running time, foreground time, category candidates,
  and evidence-backed unknown states when app/game evidence exists.
- Network flow summaries, destination/category candidates, VPN/proxy/tunnel
  indicators, bandwidth/count summaries, and evidence-backed unknown states when
  network flow evidence exists.
- Local screen-analysis summaries, visible category candidates, risk signals,
  confidence, and deletion state when screen evidence exists.
- Parent-authored rules, schedules, overrides, grace periods, and permission-request state.
- Local AI safety result references and confidence/degraded state when AI contributes.
- Browser AI policy evaluator handoff refs, including validated AI result,
  memory/cache, knowledge graph, parent rule, schedule, and evaluator mode refs
  when browser URL/video AI contributes.
- Time budget state, active timers, and previous policy decisions.

Policy input must not include:

- Billing provider state inside the evaluator.
- Portal-only UI state as a decision source.
- Untyped model text or unvalidated API AI output.
- Derived memory or graph claims unless they carry evidence references.
- Portal UI state, AI output, memory, or graph refs as direct policy authority
  without deterministic evaluator reason codes and audit refs.

## Contract Boundary

Policy contracts should live in the owning domain packages before runtime behavior consumes them. Expected contract families are:

- `FamilyPolicySet`: schema version, family reference, child profile references, device references, rule list, schedule list, and policy version.
- `PolicyRule`: rule id, target type, target reference, action, schedule reference, priority, reason code, created-by reference, effective window, and enabled state.
- `PolicyTarget`: app/process, domain/site, category, video/channel, activity type, device, child profile, or future platform-specific target.
- `PolicySchedule` and `TimeBudget`: local time zone, recurrence, active windows, budget duration, reset behavior, grace period, and expiry.
- `PermissionRequest`: evidence reference, requested target/action, child-device context, parent response state, expiry, and audit references.
- `PolicyDecision`: action, reason codes, evidence references, rule references, AI result reference when used, timer/expiry fields, conflict-resolution explanation, dry-run flag, and enforcement handoff state.
- `PolicyAuditEvent`: previous policy version, new policy version, actor reference, source surface, timestamp, and validation result.

Parent-authored contracts and child-device evaluator contracts must stay separated. Parent surfaces send intents and rule sets; child-device agents validate and execute local decisions.

## Deterministic Decisioning

The evaluator must define and test conflict resolution before enforcement is possible:

- Explicit parent block or time-limit rules beat ambiguous local AI allow output.
- Explicit parent allow can permit a known safe target unless a stricter safety or legal policy says otherwise.
- Category labels by themselves do not enforce anything; action requires a
  matching parent-authored rule.
- Ask-parent decisions are used when rule intent is clear but parent approval is required.
- Unknown is used when evidence, rule, or AI state is insufficient for a safe decision.
- Time windows and active timers must be evaluated from the child-device clock and recorded with the decision.
- Dry-run returns the same decision shape as enforcement mode, with enforcement disabled and explanation marked as preview.

## Failure Behavior

- Invalid rules fail schema validation and are not activated.
- Rule-set version mismatch returns a typed rejection.
- Missing evidence returns unknown, parent-review, or no-op according to explicit parent rules; it must not invent content claims.
- App/game policy previews must preserve source freshness readiness. Stale,
  missing, manual-required, unavailable, and not-claimed app/game source rows
  stay manual-required before policy preview output can be accepted.
- Local AI unavailable falls back to deterministic rule behavior, unknown, parent-review, or warn.
- Timer state loss must be recoverable from journaled decision and timer events where the feature has enabled timers.
- Portal or cloud unavailability does not stop local evaluation of already validated local rules.
- Billing failures do not silently disable critical local safety behavior.

## Expected Deliverables

- Parent/family/child/device contracts where needed.
- Policy rule contracts.
- Parent rule-authoring contracts separated from child-device evaluator contracts.
- Schedule/time-window contracts.
- Category/app/site/domain target contracts.
- Game/app session target contracts for app, process, launcher, game title,
  category, running-time budget, and foreground-time budget.
- Network flow target contracts for process, domain, IP, protocol, destination
  category, VPN/proxy/tunnel indicator, bandwidth/count budget, and
  unusual-traffic digest.
- Screen-derived category/risk target contracts for visible activity categories,
  confidence thresholds, and local-only evidence requirements.
- Permission request contracts.
- Decision reason codes.
- Local AI decision input and output contracts when the policy is context-heavy.
- Dry-run evaluator before enforcement.

## Acceptance

- Invalid rules fail schema validation.
- Portal-authored rules are inert configuration until the child-device agent validates and evaluates them.
- Conflicting rules have deterministic resolution.
- Policy decisions reference evidence.
- Policy decisions reference the local AI output when AI contributed.
- Browser policy decision bundles expose reason codes, audit refs, fallback
  visibility for unknown outcomes, and adapter proof before active browser block
  decisions are valid.
- Browser post-analysis action plans distinguish what happened after review
  from what could have happened in real time. They require adapter proof for
  delivered warning, stop, approval, or future-block actions and expiry for
  remembered decisions.
- Browser-game risk/benefit signal sets must remain evidence-backed policy
  input candidates. They may classify bounded risk and benefit rows from
  evidence, metadata, analysis, and parent-rule refs, but they must not consume
  raw game payloads, chat content, page body, raw model text, account/purchase
  execution, cloud-frame analysis, native game control, final policy decisions,
  runtime gate execution, or enforcement.
- Browser-game account/signup/purchase gate requests and decisions must be
  evidence-backed candidate states only. They may reference managed route,
  title, AI-analysis, parent-rule, policy-version, and action-candidate refs,
  but they must not capture raw URLs, titles, account identifiers, credentials,
  form submissions, payments, downloads, UI delivery, child notifications, final
  policy decisions, runtime gate execution, native game control, cloud-frame
  analysis, or enforcement.
- Browser-game cloud-gaming gate requests and decisions must distinguish
  platform/session evidence from exact streamed-game claims. They may reference
  managed route, signal, platform title/rating metadata, policy, parent
  approval, schedule, and mobile capability refs, but they must not capture raw
  stream frames, analyze cloud-streamed frames, claim per-game cloud title
  certainty, control native games or launchers, read game chat, handle
  account/purchase flows, deliver UI/notifications, make final policy
  decisions, execute runtime gates, or enforce actions.
- Browser-game unblocked-site detection outputs must remain candidate policy
  inputs or bypass evidence only. They may reference managed route, portal
  index, iframe, search-intent, unmanaged-process, and parent-policy refs, but
  they must not store raw URLs, raw page body, raw search queries, iframe
  content, exact unmanaged URLs, native game control claims, cloud-frame
  analysis, account/purchase flows, UI delivery, final policy decisions,
  runtime gates, or enforcement.
- Browser-game portal pattern libraries must remain policy-input metadata only.
  They may describe reviewed or manual-required portal families through
  evidence refs and fingerprints, but they must not store raw domains, URLs,
  page titles, page bodies, perform runtime detection, run AI classification,
  claim cloud-gaming ownership, make final policy decisions, or enforce
  actions.
- Browser-game cloud-gaming pattern libraries must remain policy-input metadata
  only. They may describe reviewed or manual-required cloud platform, cloud PC,
  mobile portal, browser-embedded cloud-game, and native launcher prompt
  patterns through evidence refs and fingerprints, but they must not store raw
  cloud domains, URLs, titles, stream frames, perform runtime detection, inspect
  cloud-streamed frames, claim per-game cloud-title certainty, control native
  launchers or games, make final policy decisions, or enforce actions.
- Browser-game educational classifier results must remain candidate policy
  inputs only. They may use evidence refs for school URLs, teacher/parent
  allowlists, page/subject metadata, AI classification refs, past parent
  approval, homework context, school platforms, and platform self-labels, but
  they must not treat platform labels as authority, consume raw page/game/model
  payloads, execute account or purchase flows, make final policy decisions,
  execute runtime gates, render UI, control native games, inspect cloud frames,
  or enforce actions.
- Browser-game AI analysis results must remain candidate policy inputs only.
  They may use typed evidence refs, task refs, custody labels, model runtime
  refs, prompt template versions, summary refs, benefit/risk signals, and
  uncertainty codes, but they must not persist raw URLs, page body, game
  payloads, screen frames, model text, execute account or purchase flows,
  inspect cloud frames, control native games, render UI, make final policy
  decisions, execute runtime gates, or enforce actions.
- Browser-game UGC/multiplayer/chat risk assessments must remain candidate
  policy inputs only. They may use evidence refs for UGC pages, experience
  pages, lobbies, profile/friends/message routes, launch prompts, web-to-app
  launch surfaces, approved experience refs, chat-control capability refs,
  purchase-control capability refs, and public risk context, but they must not
  read chat content, store profile/account/experience identifiers, execute
  launches or purchases, control native games, make final policy decisions,
  execute runtime gates, render UI, or enforce actions.
- Browser-game memory/cache entries must remain bounded policy-input refs only.
  Fresh hits can be reused only with bounded TTL, required subject keys,
  decision refs, and evidence refs. Stale, miss, and manual-required rows must
  not drive policy input. Cache entries must not store raw URLs, raw platform
  game IDs, raw cloud game titles, raw game payloads, raw model text, runtime
  cache state, AI cache state, UI delivery, native game control,
  cloud-frame analysis, final policy decisions, or enforcement.
- Browser-game policy compiler candidates must consume parent-owned
  browser-game evidence, analysis, mobile capability, rule, and schedule refs
  only. Candidate outputs can propose allow, warn, parent-review, block,
  time-limit, manual-review, or unknown outcomes, but they must not claim final
  policy authority, runtime gate execution, UI delivery, native game control,
  cloud-frame analysis, raw game payload storage, or enforcement.
- Browser-game managed hold/block adapter plans must remain policy-candidate
  delivery plans only. They may link policy candidates, child UX refs, managed
  intervention adapter proof refs, and audit refs for hold, approval, block, and
  warn paths, but they must not store raw URL/page/game payloads, reuse child
  cookies or sessions, mutate browsers, render child pages, deliver
  notifications, apply time limits, make final policy decisions, inspect
  cloud-streamed frames, control native games, or enforce actions.
- Browser-game journal/SQLite read-model refs must remain evidence visibility
  inputs only. They may reference journal replay, SQLite rows, app-game session
  reports, adapter audit rows, and proof refs, but they must not store raw
  URL/page/game/title/account/purchase data, claim cloud-title certainty, mutate
  browsers, render UI, make final policy decisions, or enforce actions.
- Browser-game platform/route contracts must remain policy-input metadata only.
  They may describe platform kinds, route surface kinds, source kinds, custody
  labels, pattern refs, evidence refs, confidence, candidate/manual-required
  status, and managed-browser requirements, but they must not store raw domains,
  URLs, paths, page bodies, perform live URL parsing, claim runtime detection,
  run AI classification, make final policy decisions, control native games,
  inspect cloud frames, or enforce actions.
- Browser-game URL shape parser results must remain redacted policy-input
  metadata only. They may classify protocol/host/path-depth shape, route surface
  kind, route hints, query/fragment presence, reason codes, confidence, and
  route-shape fingerprints, but they must not store raw URLs, domains, paths,
  queries, fragments, navigate browsers, claim runtime detection, run AI
  classification, make final policy decisions, inspect cloud frames, control
  native games, or enforce actions.
- Browser-game runtime signal detector rows must remain shape-only policy-input
  metadata. They may classify signal kinds, source kinds, fingerprints, evidence
  refs, confidence, status, reason codes, and managed-browser proof
  requirements, but they must not store raw DOM, canvas, stream, audio, or
  gamepad input, instrument browsers, execute runtime detection, run AI
  classification, make final policy decisions, inspect cloud frames, control
  native games, or enforce actions.
- Browser-game metadata extractor rows must remain redacted policy-input
  metadata only. They may classify field kinds, source kinds, fingerprints,
  evidence refs, confidence, status, and reason codes for metadata shape, but
  they must not store raw titles, descriptions, page bodies, images, structured
  data, scrape runtime DOM, call platform APIs, run AI classification, make final
  policy decisions, inspect cloud frames, control native games, or enforce
  actions.
- Browser-game hidden analysis profile safety rows must remain policy-input
  safety metadata only. They may describe isolated profile kinds, profile
  fingerprints, loader proof refs, evidence refs, confidence, status, reason
  codes, retention bounds, and safety flags, but they must not reuse child
  cookies or sessions, share child storage, store or capture raw URL/page/game
  payloads or frames, instrument browsers, control hidden native surfaces, run AI
  classification, make final policy decisions, render UI, inspect cloud frames,
  control native games, or enforce actions.
- Browser-game child checking/block UX snapshots are presentation state only.
  They must use schema-known child text-token refs, reject raw child copy and
  rendered UI claims, and must not become policy authority, runtime browser
  blocking, native game control, cloud-frame analysis, or enforcement.
- Browser-game parent dashboard UX snapshots are presentation state only. They
  may organize evidence and policy-candidate refs for parent review, but they
  must not render portal UI, fetch runtime data, deliver notifications, make
  final policy decisions, inspect cloud-streamed frames, control native games,
  or enforce actions.
- Child-facing checking/warning UX snapshots are presentation state only. They
  must use schema-known calm copy tokens, reject raw or shaming/surveillance
  copy claims, and must not become policy authority, direct enforcement, or a
  visual-render claim without the matching adapter and UI proof.
- Parent explanation/audit UX bundles are also presentation state. They must
  cite policy decision reason codes and audit refs, reveal degraded/manual
  fallback states, and must not let portal explanation state become policy
  authority or enforcement authority.
- Decision events are journaled.
- Parent-facing explanation is stable and testable.
- Dry-run mode can explain what would happen without enforcing it.
- Explicit parent rules override ambiguous AI output.
- Evidence categories are labels for parent rules, not hidden product-level
  blocks.

## Validation Gates

- TypeScript schema tests prove valid/invalid policy sets, rules, schedules, targets, permission requests, decisions, and audit events.
- Rust parity tests cover every Rust-crossing policy shape and exact enum values.
- Evaluator tests cover allow, warn, block, time-limit, parent-review, unknown, disabled rule, expired rule, active schedule, inactive schedule, and conflicting-policy cases.
- Integration tests use real stored evidence and real policy contracts, not mocks or fake provider output.
- Dry-run tests prove preview and enforcement modes return consistent decisions with different enforcement handoff state.
- Portal tests, when UI exists, prove rule authoring sends typed intents and does not run evaluation in the browser.
- Source-gated policy preview read-model tests prove source-manual-required
  rows remain visible without preview decision refs, compiler-manual-required
  rows stay distinct, and service/runtime/UI/adapter claims remain false.

## Non-Goals

- Do not enforce policy until the evaluator is trusted.
- Do not evaluate or enforce policy in the portal/browser.
- Do not make untyped or untraceable AI the source of a policy decision.
- Do not mix billing entitlements into policy logic.
- Do not make cloud availability required for local evaluation of already validated local rules.
- Do not create broad catch-all rules that hide missing target contracts.

## Done Signal

Given real or contract-valid activity evidence, parent rules authored through typed contracts, and local AI output where needed, the child-device evaluator returns a deterministic typed decision with reason codes and evidence references. Tests cover allow, limit, block, parent-review, unknown, and conflict behavior.
