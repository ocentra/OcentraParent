<!-- agent-capsule -->

> Agent Capsule
> Doc: Assigned Topic Full Catalog -> Typed Policy Control Contracts
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Assigned Topic Full Catalog -> Typed Policy Control Contracts

Use this prompt for policy-control catalog contract slices. The coordinator should assign one topic per lane and keep the mail short:

```text
Fetch/rebase latest main first. Read docs/architecture/policy-control-catalog-worker-prompt.md. Take assigned topic <topic>. Do not touch C UI paths. Commit locally after validation, push when ready, and report PR_READY with counts and proof.
```

## Assignment

Repo root:

```text
E:\OcentraParent
```

Use repo-relative paths in code, tests, and reports. The full local path is:

```text
E:\OcentraParent\<repo-relative-path>
```

Assigned topic:

```text
<Topic: Browser | Apps | Games | Screen | Network | Tracking>
```

Side-panel category:

```text
<same as assigned topic>
```

Primary source docs:

```text
<topic capability/catalog guide>
<topic schema proposal>
```

Implement only the assigned topic unless the assignment explicitly expands scope.

## Topic Source Docs

### Browser

Primary docs:

```text
docs/browser-policy-settings-catalog.md
docs/browser-control-schema-proposal.md
```

Optional capability references:

```text
docs/managed-unmanaged-browser.md
docs/browser-control-coverage-matrix.md
```

### Apps

```text
docs/app-control-capability-guide.md
docs/app-control-schema-proposal.md
```

### Games

```text
docs/game-control-capability-guide.md
docs/game-control-schema-proposal.md
```

### Screen

```text
docs/screen-evidence-analysis-capability-guide.md
docs/screen-evidence-analysis-schema-proposal.md
```

### Network

```text
docs/network-control-capability-guide.md
docs/network-control-schema-proposal.md
```

### Tracking

```text
docs/device-location-tracking-capability-guide.md
docs/device-location-tracking-schema-proposal.md
```

## Global Reference Docs

Read these only as product guardrails and consistency references. Do not implement their whole scope in this slice.

```text
docs/product-roadmap.md
docs/full-platform-portal-ai-execution-plan.md
docs/feature-expectations.md
```

Use them to preserve these product truths:

- Portal/UI is an authoring, reporting, and preview surface.
- Portal must not execute capture, enforcement, AI safety decisions, tracking, network control, policy evaluation, or child-device runtime behavior.
- Child-device agent/local Rust runtime owns persistence, compile, local evaluation, evidence, timers, capture, enforcement handoff, rollback, and audit.
- Product claims must never exceed what the implementation and capability proof can honestly support.
- Ocentra-hosted services must not become the default store for child activity, screenshots, browser history, reports, journals, or parent rules.
- Validation must make lazy, fake, or over-claimed implementations fail.

## Main Goal

Convert the assigned topic's source docs into structured, typed, UI-presentable policy control contracts.

This is not a summary task.

This is not another proposal.

This is not UI implementation.

This is not runtime/enforcement wiring unless explicitly assigned.

The required transformation is:

```text
source docs
-> typed contracts/schema/data
-> tests proving count, wording, hierarchy, options, capability truth, and renderability
```

Capture every setting, control, option, answer, visibility rule, enabled rule, capability state, fallback, and meaningful source bullet from the assigned topic.

## How To Treat The Source Docs

The schema proposal is product-shaped guidance, not runtime source.

Do not copy proposal JSON directly into runtime code.

Translate the proposal into repo-valid Ocentra Parent contracts using:

- Effect Schema validation
- branded ids from schema brands
- decode helpers
- typed policy value shapes
- typed effective policy shapes
- typed authoring manifest/control shapes
- typed update/patch/replace/preview/ack/reject shapes where applicable
- focused tests

The capability guide/catalog is the truth boundary.

Use it to decide whether each control/effect is:

- observable
- enforceable
- local-only
- manual-required
- unavailable
- degraded
- future-gap
- proof-gated
- platform-limited
- permission-limited

Never imply exact knowledge or enforcement that the guide says cannot be proven.

## Required Contract Families

Where applicable for the assigned topic, implement or extend typed contracts for:

### 1. Authoring Manifest

Defines what Portal can render:

- sections
- questions
- controls
- accepted options
- helper text
- visibility conditions
- enabled conditions
- target scopes
- writesTo paths
- validation hints
- capability-state display hints

Portal must not invent questions outside the manifest.

### 2. Policy Value Document

Defines durable parent-authored intent:

- enabled state
- selected modes/options
- schedules
- limits
- rule lists
- target lists
- retention choices
- approval/override choices
- fallback choices

This is validated as a complete policy value.

### 3. Effective Policy Document

Defines deterministic compiled child-agent/runtime plan:

- flat enough for runtime use
- explicit fallback behavior
- explicit proof requirements
- explicit unsupported/manual/future states
- no hidden Portal-owned behavior

### 4. Update Protocol

Defines typed commands where applicable:

- get
- preview
- patch
- replace
- acknowledge
- reject
- rollback
- capability refresh
- topic-specific commands, if present in the proposal

### 5. Capability Registry

Defines runtime capability states that drive hide/disable/degrade behavior:

- available
- disabled
- unsupported
- permission-required
- permission-limited
- protected
- degraded
- manual-required
- future-gap
- unavailable

## Required UI Shape Support

The structured contracts must support the current Policy UI shape:

- left side-panel category tile
- target selector:
  - Family
  - Per Device
  - Per Child, if applicable
- optional target/device slots
- top lanes/tabs:
  - Rules
  - Schedule
  - Approvals
  - Enforcement
  - Audit
  - topic-specific lanes if needed
- section/subheading boxes
- subgroup/divider rows
- setting cards
- option grids
- Observe / Enforce / Audit-only effect chips
- compact vs many-option presentation
- disabled controls with clear reason
- capability state beside sensitive or strict controls

## Required Hierarchy

Preserve enough source hierarchy that C/UI can render without guessing.

Every setting must belong to this hierarchy:

```text
sidePanelCategory
-> policyLane/tab
-> catalogSection
-> subgroup/divider
-> setting/control
-> options/answers
```

Do not collapse sections or groups unless the source docs clearly require it.

## Required Setting Shape

The shape below is conceptual. Do not paste it as raw TypeScript interfaces with `string` fields.

Implementation must use repo-valid Effect Schema contracts:

- branded ids for settingId, optionId, sectionId, groupId, effectKey, writesTo path, etc.
- literal unions/schemas for controlType, uiCardType, effectStatus, runtimeOwner, targetScope, effectMode
- decode helpers for external/unknown input
- no manual `string & { ... }` brands
- no raw app/runtime `string` annotations

Conceptual setting/control shape:

```ts
{
  settingId: SettingId;
  sourceDocument: SourceDocumentPath;
  sourceHeadingPath: ReadonlyArray<string>;
  sourceSection: SectionId;
  sourceGroup?: GroupId;
  originalSourceText: string;
  uiQuestionText: string;
  helperText?: string;
  displayOrder: number;

  controlType:
    | "toggle"
    | "singleChoice"
    | "multiChoice"
    | "number"
    | "duration"
    | "schedule"
    | "ruleList"
    | "targetList"
    | "retention"
    | "actionList"
    | "readOnlyStatus";

  uiCardType:
    | "compactSingleChoice"
    | "manyOptionSingleChoice"
    | "normalMultiChoice"
    | "manyOptionMultiChoice"
    | "toggleCard"
    | "scheduleCard"
    | "ruleListCard"
    | "targetListCard"
    | "retentionCard"
    | "statusCard";

  layoutHints: {
    preferredColumnSpan?: number;
    collapsible?: boolean;
    searchableOptions?: boolean;
    optionGroupCount?: number;
    showAsMatrixWhenLarge?: boolean;
  };

  acceptedOptions: ReadonlyArray<{
    optionId: OptionId;
    label: string;
    originalSourceText: string;
    meaning?: string;
    defaultSelected?: boolean;
  }>;

  targetScopeOptions: ReadonlyArray<
    | "family"
    | "perChild"
    | "perDevice"
    | "perPlatform"
    | "perApp"
    | "perGame"
    | "perBrowser"
    | "perNetwork"
    | "perLocationRule"
  >;

  effectModeOptions: ReadonlyArray<
    | "off"
    | "observe"
    | "dryRun"
    | "warn"
    | "notify"
    | "ask"
    | "limit"
    | "block"
    | "enforce"
    | "auditOnly"
  >;

  effectKey: EffectKey;

  effectStatus:
    | "alreadyRepresented"
    | "needsWiring"
    | "manualRequired"
    | "unavailable"
    | "futureGap"
    | "degraded"
    | "permissionRequired"
    | "permissionLimited"
    | "proofRequired";

  runtimeOwner:
    | "portalOnly"
    | "parentDomain"
    | "agentProtocol"
    | "rustService"
    | "childAgent"
    | "osAdapter"
    | "manualProof"
    | "parentOwnedStorage"
    | "localAiRuntime";

  capabilityRequirement?: string;
  proofRequirement?: string;
  visibilityConditions?: unknown[];
  enabledConditions?: unknown[];
  validationRules?: unknown[];
  unsafeOrUnsupportedFallback?: string;
}
```

Use existing repository naming conventions if a compatible schema already exists. Do not create a parallel arbitrary shape when the repo already has a contract pattern.

## Many-Option Rule

If a topic has hundreds or thousands of options, do not create an endless flat settings array or huge button row.

Break large option sets into:

```text
policy lane
-> section
-> subgroup
-> card
-> grouped option set
```

Many-option cards must include metadata for:

- search
- filter
- grouping
- collapse/expand
- matrix rendering when useful
- count display
- selected count display

## Topic-Specific Truth Boundaries

Apply these boundaries from the source docs.

### Browser

Exact URL, active tab, page title, and browser download source require a managed browser boundary or another explicit browser integration.

Process/window/network evidence can detect browser-like activity or bypass behavior, but it must not be treated as exact URL/tab evidence.

### Apps

Native app controls depend on OS/package/process/window/usage/managed-device evidence.

Unknown apps must remain unknown. Do not silently promote an unknown app to a known app, risky app, blocked target, or game.

### Games

Launcher evidence is not automatically game evidence.

Browser games depend on browser evidence. Cloud games depend on the surface. Network-only evidence can suggest a service but usually cannot prove exact title or active play.

### Screen

Screen evidence is high-sensitivity.

The normal path is not "save screenshots." The normal path is parent-enabled capture, capability/permission check, local encrypted temporary queue, local OCR/vision analysis, schema-valid summary/evidence refs, then deletion of raw image/frame data.

Screen analysis evidence must not enforce by itself unless validated summaries, evidence refs, parent rules, and deterministic policy decisions make it eligible.

### Network

Network flow evidence is metadata, not decrypted content.

It does not prove page text, chat content, search terms, active browser tabs, full HTTPS URLs, or user intent.

Exact URL evidence belongs to managed browser or another explicit browser integration.

### Tracking

Location evidence must carry source, permission state, freshness, accuracy, custody, and fallback behavior.

Live tracking is not perfect continuous movement. Platforms can throttle, permissions can change, battery policy can pause work, and desktop devices may have weak location providers.

Location history must not silently upload to Ocentra-hosted storage by default.

## Implementation Requirements

Expected package area:

```text
packages/parent-domain
```

Implement or extend:

- typed schema files
- assigned topic catalog/control files
- chunked data files if large
- package exports
- focused contract tests
- decode helpers
- stable branded ids
- invalid-state rejection tests

Avoid:

- god files
- arbitrary JSON paths
- UI-owned arbitrary Q&A
- naked domain strings in app/runtime code
- unsupported runtime claims
- fake enforcement claims
- copying proposal JSON directly into runtime source
- implementing other topics by accident

Rust protocol parity should happen only after TypeScript contracts are explicit and test-backed, unless the assignment explicitly requires Rust work now.

## Tests Must Prove

Focused tests must prove:

- every assigned source setting/control was captured
- every accepted option was captured
- expected setting count matches the source docs
- expected option count matches the source docs
- source wording is preserved
- hierarchy is preserved:

```text
category -> lane/tab -> section -> group -> setting
```

- setting IDs are stable and unique
- option IDs are stable and unique within each setting
- control type exists for every setting
- UI card type exists for every setting
- many-option settings are searchable/grouped/collapsible where needed
- target scope metadata exists
- effect mode metadata exists
- effect key/status exists
- capability requirement/proof requirement exists where needed
- unsupported/manual/future/degraded behavior is honest
- Portal can render category -> lane -> section -> group -> card without inventing structure
- runtime/Rust/child-agent ownership is explicit
- invalid policy values are rejected
- proposal JSON was translated into repo-valid contracts, not copied as loose data

## Validation Commands

Run at minimum:

```bash
git diff --check origin/main...HEAD
npm run lanes:guard
npm run hub:guard
npm run build:contracts
npm run validate
```

Also run focused parent-domain tests for the assigned catalog.

If a command is blocked, report the exact blocker and what partial validation was completed.

## DONE / PR_READY Report

The final report must include:

- assigned topic
- branch
- commit
- pushed state
- PR URL if opened
- source docs consumed
- exact files touched
- number of catalog sections captured
- number of subgroups/dividers captured
- number of settings captured
- number of accepted options captured
- number of settings by UI card type
- number of settings by effectStatus
- number of capability states represented
- validation commands and results
- known gaps or risks
- any source bullets intentionally not captured, with exact reason
- what C/UI can render immediately
- what runtime/Rust/child-agent workers still need to wire
- any cross-topic dependency recorded but not duplicated
