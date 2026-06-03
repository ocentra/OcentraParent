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
  ask-parent, or blocked for a child profile and schedule.
- Parent can preview what a rule would do before enabling enforcement.
- Parent can see which evidence, parent rule, local AI result, schedule, and conflict-resolution reason produced a decision.
- Parent can approve, deny, or time-box ask-parent requests through typed approvals that the child-device agent validates locally.

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
- Missing evidence returns unknown, ask-parent, or no-op according to explicit parent rules; it must not invent content claims.
- Local AI unavailable falls back to deterministic rule behavior, unknown, ask-parent, or warn.
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
- Evaluator tests cover allow, warn, block, time-limit, ask-parent, unknown, disabled rule, expired rule, active schedule, inactive schedule, and conflicting-policy cases.
- Integration tests use real stored evidence and real policy contracts, not mocks or fake provider output.
- Dry-run tests prove preview and enforcement modes return consistent decisions with different enforcement handoff state.
- Portal tests, when UI exists, prove rule authoring sends typed intents and does not run evaluation in the browser.

## Non-Goals

- Do not enforce policy until the evaluator is trusted.
- Do not evaluate or enforce policy in the portal/browser.
- Do not make untyped or untraceable AI the source of a policy decision.
- Do not mix billing entitlements into policy logic.
- Do not make cloud availability required for local evaluation of already validated local rules.
- Do not create broad catch-all rules that hide missing target contracts.

## Done Signal

Given real or contract-valid activity evidence, parent rules authored through typed contracts, and local AI output where needed, the child-device evaluator returns a deterministic typed decision with reason codes and evidence references. Tests cover allow, limit, block, ask-parent, unknown, and conflict behavior.
