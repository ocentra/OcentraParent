<!-- agent-capsule -->

> Agent Capsule
> Doc: App Control Settings Inventory
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# App Control Settings Inventory

Generated from `BaselineAppControlFullCatalog`.
Total settings: 346

Use this as the raw review list for deciding parent-facing grouping, proof gaps, and policy UX.
This is a generated inventory of current typed catalog data, not product-complete implementation proof.

## Source Documents

- docs/app-control-capability-guide.md
- docs/app-control-schema-proposal.md

## Section: app-management

### app-management

#### app-management-defaults

1.  Enable app management?

- settingId: `app.enabled`
- policyLane: `rules`; sectionId: `app-management`; groupId: `app-management-defaults`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Enable app management?
- acceptedOptions: Enabled | Disabled
- helperText: Native app controls stay disabled until a parent enables this policy document.

2.  What should happen to app activity?

- settingId: `app.defaultPosture`
- policyLane: `rules`; sectionId: `app-management`; groupId: `app-management-defaults`
- cardKind: `single-choice-many`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Blocking is product-true only when the target platform adapter proves it.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: What should happen to app activity?
- acceptedOptions: Allow | Observe | Warn | Ask | Limit | Block
- helperText: Blocking is product-true only when the target platform adapter proves it.

3.  How should app management run on this device?

- settingId: `app.managementMode`
- policyLane: `rules`; sectionId: `app-management`; groupId: `app-management-defaults`
- cardKind: `single-choice-compact`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Portal authoring alone must not claim runtime enforcement.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: How should app management run on this device?
- acceptedOptions: Local Child Agent | Lan Live | Authoring Only | Unavailable
- helperText: Portal authoring alone must not claim runtime enforcement.

## Section: inventory

### inventory

#### inventory-sources

4.  How should installed app inventory be used?

- settingId: `inventory.mode`
- policyLane: `evidence`; sectionId: `inventory`; groupId: `inventory-sources`
- cardKind: `single-choice-compact`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `permission-limited`
- proofRequirement: Inventory is not proof of current use.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: How should installed app inventory be used?
- acceptedOptions: Disabled | Reports Only | Use For Matching And Reports | Required For Strict Rules
- helperText: Inventory is not proof of current use.

5.  Which app inventory sources are allowed?

- settingId: `inventory.sources`
- policyLane: `evidence`; sectionId: `inventory`; groupId: `inventory-sources`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: Portable, privacy-hidden, and mobile-tokenized apps may be absent.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Which app inventory sources are allowed?
- acceptedOptions: Os Installed Apps | Desktop Shortcuts | Store Packages | Package Manager | Managed Device Apps | Screen Time Tokens | Executable Metadata | Parent Catalog
- helperText: Portable, privacy-hidden, and mobile-tokenized apps may be absent.

#### inventory-identity

6.  Which identity fields may app rules use?

- settingId: `inventory.identityFields`
- policyLane: `evidence`; sectionId: `inventory`; groupId: `inventory-identity`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: No single identity field proves an app on every platform.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Which identity fields may app rules use?
- acceptedOptions: Package Id | Bundle Id | App User Model Id | Desktop Entry Id | Application Token | Executable Path | Publisher Signature | File Hash | Display Name | Parent Label
- helperText: No single identity field proves an app on every platform.

7.  What should happen when an app cannot be identified?

- settingId: `inventory.unknownHandling`
- policyLane: `rules`; sectionId: `inventory`; groupId: `inventory-identity`
- cardKind: `single-choice-many`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unknown apps must remain labeled unknown.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: What should happen when an app cannot be identified?
- acceptedOptions: Allow | Observe | Warn | Ask | Count Under Unknown Budget | Block If Supported
- helperText: Unknown apps must remain labeled unknown.

## Section: runtime-evidence

### runtime-evidence

#### runtime-sources

8.  Which runtime evidence sources may be used?

- settingId: `evidence.runtimeSources`
- policyLane: `evidence`; sectionId: `runtime-evidence`; groupId: `runtime-sources`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Foreground evidence and mobile usage visibility are platform-permission dependent.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Which runtime evidence sources may be used?
- acceptedOptions: Process Snapshot | Process Start Exit | Foreground Window | Usage Stats | Device Activity | Managed Device State | Accessibility Approved State | App Session Summary
- helperText: Foreground evidence and mobile usage visibility are platform-permission dependent.

#### runtime-proof

9.  What proof is enough for app rules?

- settingId: `evidence.requiredProof`
- policyLane: `evidence`; sectionId: `runtime-evidence`; groupId: `runtime-proof`
- cardKind: `single-choice-many`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Inventory-only proof cannot justify strict runtime action.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: What proof is enough for app rules?
- acceptedOptions: Inventory Only | Process Running | Foreground Window | Fresh App Session | Platform Usage Event | Managed Device State
- helperText: Inventory-only proof cannot justify strict runtime action.

10. What if app proof is unavailable?

- settingId: `evidence.whenProofUnavailable`
- policyLane: `evidence`; sectionId: `runtime-evidence`; groupId: `runtime-proof`
- cardKind: `single-choice-many`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unavailable proof is a parent-visible state, not a hidden allow or block.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: What if app proof is unavailable?
- acceptedOptions: Allow | Observe | Warn | Ask | Block Until Ready | Mark Unavailable
- helperText: Unavailable proof is a parent-visible state, not a hidden allow or block.

#### duration-proof

11. Which duration should time budgets count?

- settingId: `evidence.durationMode`
- policyLane: `schedule`; sectionId: `runtime-evidence`; groupId: `duration-proof`
- cardKind: `single-choice-compact`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Portal refresh cadence must not count as child activity.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Which duration should time budgets count?
- acceptedOptions: Running Time | Foreground Time | Platform Usage Time | Manual Review Only
- helperText: Portal refresh cadence must not count as child activity.

#### data-minimization

12. What must app controls never collect?

- settingId: `evidence.neverCollect`
- policyLane: `audit`; sectionId: `runtime-evidence`; groupId: `data-minimization`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: App evidence does not prove content, keystrokes, screenshots, or chat text.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: What must app controls never collect?
- acceptedOptions: Screen Contents | Screenshots | Keystrokes | Chat Content | Voice Content | App Internal Documents | Launcher Credentials | Decrypted Network Payload | Raw Command Line With Secrets
- helperText: App evidence does not prove content, keystrokes, screenshots, or chat text.

## Section: app-rules

### app-rules

#### rule-targets

13. What app targets should rules match?

- settingId: `rules.allowedTargetTypes`
- policyLane: `rules`; sectionId: `app-rules`; groupId: `rule-targets`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Unknown, helper, wrapper, and renamed apps require honest confidence state.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: What app targets should rules match?
- acceptedOptions: App Ref | App Category | Package Id | Bundle Id | Application Token | Executable Identity | Publisher Signature | Unknown App | Managed App State | App Session | Capability State
- helperText: Unknown, helper, wrapper, and renamed apps require honest confidence state.

14. How strong must an app match be before strict action?

- settingId: `rules.matchConfidenceRequired`
- policyLane: `rules`; sectionId: `app-rules`; groupId: `rule-targets`
- cardKind: `single-choice-compact`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Strict action requires deterministic or approved app identity proof.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: How strong must an app match be before strict action?
- acceptedOptions: Any Candidate | Catalog Confidence High | Deterministic Or Parent Approved | Managed Device Proof
- helperText: Strict action requires deterministic or approved app identity proof.

#### unknown-apps

15. Default rule for unknown apps?

- settingId: `rules.defaultUnknownRule`
- policyLane: `rules`; sectionId: `app-rules`; groupId: `unknown-apps`
- cardKind: `single-choice-many`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Unknown remains unknown until adapter evidence maps it confidently.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Default rule for unknown apps?
- acceptedOptions: Allow | Observe | Warn | Ask First Run | Limit | Block If Supported
- helperText: Unknown remains unknown until adapter evidence maps it confidently.

## Section: budgets

### budgets

#### time-budgets

16. Enable app time budgets?

- settingId: `budgets.enabled`
- policyLane: `schedule`; sectionId: `budgets`; groupId: `time-budgets`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Budgets need child-agent timer state, not portal-rendered time.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Enable app time budgets?
- acceptedOptions: Enabled | Disabled
- helperText: Budgets need child-agent timer state, not portal-rendered time.

17. Default daily app time limit in minutes?

- settingId: `budgets.defaultDailyMinutes`
- policyLane: `schedule`; sectionId: `budgets`; groupId: `time-budgets`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Timer recovery and audit are child-agent responsibilities.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Default daily app time limit in minutes?
- acceptedOptions: none
- helperText: Timer recovery and audit are child-agent responsibilities.

18. What happens when app time runs out?

- settingId: `budgets.whenExhausted`
- policyLane: `schedule`; sectionId: `budgets`; groupId: `time-budgets`
- cardKind: `single-choice-many`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Terminate, shield, or block after budget requires platform proof.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: What happens when app time runs out?
- acceptedOptions: Observe | Warn | Ask | Terminate If Supported | Shield If Supported | Block If Supported
- helperText: Terminate, shield, or block after budget requires platform proof.

## Section: enforcement

### enforcement

#### strict-actions

19. Which app enforcement actions may run?

- settingId: `enforcement.allowedActions`
- policyLane: `enforcement`; sectionId: `enforcement`; groupId: `strict-actions`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Broad app blocking remains manual-required until a real adapter proves it.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Which app enforcement actions may run?
- acceptedOptions: Warn | Ask Parent | Owned Process Terminate | Target Process Terminate | Block Launch | Shield App | Suspend Package | Hide Package | Time Limit | Managed Install | Managed Uninstall
- helperText: Broad app blocking remains manual-required until a real adapter proves it.

20. What if a strict app action is unsupported?

- settingId: `enforcement.strictActionFallback`
- policyLane: `enforcement`; sectionId: `enforcement`; groupId: `strict-actions`
- cardKind: `single-choice-many`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unsupported strict actions must surface as unavailable or parent-visible fallback.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: What if a strict app action is unsupported?
- acceptedOptions: Allow And Report Unavailable | Observe And Report Unavailable | Warn And Report Unavailable | Parent Request Report Unavailable | Block Until Ready
- helperText: Unsupported strict actions must surface as unavailable or parent-visible fallback.

21. How long should the child get before strict action applies?

- settingId: `enforcement.graceSeconds`
- policyLane: `enforcement`; sectionId: `enforcement`; groupId: `strict-actions`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Grace timers need local runtime and audit state.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: How long should the child get before strict action applies?
- acceptedOptions: none
- helperText: Grace timers need local runtime and audit state.

22. Require rollback state for strict actions?

- settingId: `enforcement.requireRollbackPlan`
- policyLane: `enforcement`; sectionId: `enforcement`; groupId: `strict-actions`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `rust-service`; capabilityState: `available`
- proofRequirement: Strict actions need rollback or explicit unavailable outcome.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Require rollback state for strict actions?
- acceptedOptions: Enabled | Disabled
- helperText: Strict actions need rollback or explicit unavailable outcome.

## Section: app-lifecycle

### app-lifecycle

#### managed-lifecycle

23. How should app install and uninstall controls be handled?

- settingId: `lifecycle.mode`
- policyLane: `setup`; sectionId: `app-lifecycle`; groupId: `managed-lifecycle`
- cardKind: `single-choice-compact`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Personal app install or removal often is not available.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: How should app install and uninstall controls be handled?
- acceptedOptions: Disabled | Report Managed State | Managed Apps Only | Device Owner Or Mdm Only
- helperText: Personal app install or removal often is not available.

24. Which managed app lifecycle operations are allowed?

- settingId: `lifecycle.allowedOperations`
- policyLane: `setup`; sectionId: `app-lifecycle`; groupId: `managed-lifecycle`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Install, uninstall, hide, and suspend depend on platform custody and policy.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Which managed app lifecycle operations are allowed?
- acceptedOptions: Install Managed App | Uninstall Managed App | Hide Managed App | Suspend Managed App | Remove User Installed App If Platform Approved | Prevent Uninstall If Platform Approved
- helperText: Install, uninstall, hide, and suspend depend on platform custody and policy.

## Section: approvals

### approvals

#### approval-events

25. Which app events require parent approval?

- settingId: `approvals.requiredFor`
- policyLane: `approvals`; sectionId: `approvals`; groupId: `approval-events`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Approval state is policy data; child-agent still owns local action results.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Which app events require parent approval?
- acceptedOptions: Unknown App | New App | Blocked App | Time Extension | Managed Install | Managed Uninstall | Strict Action Unavailable | Category Override
- helperText: Approval state is policy data; child-agent still owns local action results.

26. What happens if the parent does not answer?

- settingId: `approvals.unansweredDefault`
- policyLane: `approvals`; sectionId: `approvals`; groupId: `approval-events`
- cardKind: `single-choice-compact`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unanswered approvals must be deterministic and auditable.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: What happens if the parent does not answer?
- acceptedOptions: Allow | Deny | Keep Pending | Use Rule Fallback
- helperText: Unanswered approvals must be deterministic and auditable.

## Section: reports

### reports

#### report-fields

27. Which app report fields should be visible?

- settingId: `reports.visibleFields`
- policyLane: `reports`; sectionId: `reports`; groupId: `report-fields`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Reports must distinguish raw evidence from redacted rollups.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: Which app report fields should be visible?
- acceptedOptions: Installed Apps | Running Now | Foreground Now | Session Rollups | Unknown Apps | Category Rollups | Time Budget | Policy Decisions | Enforcement Results | Approval Events | Managed Lifecycle Events | Source Capability
- helperText: Reports must distinguish raw evidence from redacted rollups.

#### retention

28. How long should raw app observations be kept?

- settingId: `retention.rawObservation`
- policyLane: `audit`; sectionId: `reports`; groupId: `retention`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Raw app observations should be short-lived and redacted where possible.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: How long should raw app observations be kept?
- acceptedOptions: none
- helperText: Raw app observations should be short-lived and redacted where possible.

29. How long should app rollups be kept?

- settingId: `retention.rollups`
- policyLane: `audit`; sectionId: `reports`; groupId: `retention`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Rollups must preserve custody and evidence-reference boundaries.
- sourceDocument: docs/app-control-schema-proposal.md
- sourceLine: 0; sourceText: How long should app rollups be kept?
- acceptedOptions: none
- helperText: Rollups must preserve custody and evidence-reference boundaries.

## Section: app-guide-core-terms

### app-guide-core-terms

#### app-guide-core-terms-native-app

30. Represent windows Win32 desktop app?

- settingId: `app-guide-core-terms-native-app-001`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-native-app`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 28; sourceText: Windows Win32 desktop app.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

31. Represent windows packaged app or Microsoft Store app?

- settingId: `app-guide-core-terms-native-app-002`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-native-app`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 29; sourceText: Windows packaged app or Microsoft Store app.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

32. Represent macOS app bundle?

- settingId: `app-guide-core-terms-native-app-003`
- policyLane: `rules`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-native-app`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 30; sourceText: macOS app bundle.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

33. Represent linux desktop app, package app, Flatpak, Snap, AppImage, or command-backed desktop entry?

- settingId: `app-guide-core-terms-native-app-004`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-native-app`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 31; sourceText: Linux desktop app, package app, Flatpak, Snap, AppImage, or command-backed desktop entry.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

34. Represent android package?

- settingId: `app-guide-core-terms-native-app-005`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-native-app`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 33; sourceText: Android package.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

35. Represent iOS or iPadOS application selected through Apple-approved controls?

- settingId: `app-guide-core-terms-native-app-006`
- policyLane: `approvals`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-native-app`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 34; sourceText: iOS or iPadOS application selected through Apple-approved controls.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

#### app-guide-core-terms-managed-app

36. Represent app installed by a supervised or managed device flow?

- settingId: `app-guide-core-terms-managed-app-001`
- policyLane: `setup`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-managed-app`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 47; sourceText: App installed by a supervised or managed device flow.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

37. Represent app allowlisted or denylisted by an OS application-control policy?

- settingId: `app-guide-core-terms-managed-app-002`
- policyLane: `approvals`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-managed-app`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 48; sourceText: App allowlisted or denylisted by an OS application-control policy.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

38. Represent app represented by an opaque mobile platform token selected by a guardian?

- settingId: `app-guide-core-terms-managed-app-003`
- policyLane: `rules`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-managed-app`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 49; sourceText: App represented by an opaque mobile platform token selected by a guardian.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

39. Represent app launched through an Ocentra-controlled shortcut, launcher, or policy adapter?

- settingId: `app-guide-core-terms-managed-app-004`
- policyLane: `enforcement`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-managed-app`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 50; sourceText: App launched through an Ocentra-controlled shortcut, launcher, or policy adapter.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

40. Represent app process started by Ocentra and tracked with an owned process/session id?

- settingId: `app-guide-core-terms-managed-app-005`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-managed-app`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 52; sourceText: App process started by Ocentra and tracked with an owned process/session id.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### app-guide-core-terms-unmanaged-app

41. Represent a normal user-installed desktop app without Ocentra policy?

- settingId: `app-guide-core-terms-unmanaged-app-001`
- policyLane: `setup`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-unmanaged-app`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 64; sourceText: A normal user-installed desktop app without Ocentra policy.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

42. Represent a portable executable?

- settingId: `app-guide-core-terms-unmanaged-app-002`
- policyLane: `rules`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-unmanaged-app`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 65; sourceText: A portable executable.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

43. Represent a copied or renamed executable?

- settingId: `app-guide-core-terms-unmanaged-app-003`
- policyLane: `rules`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-unmanaged-app`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 66; sourceText: A copied or renamed executable.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

44. Represent a helper process launched by a known app but not mapped to a supported app identity?

- settingId: `app-guide-core-terms-unmanaged-app-004`
- policyLane: `enforcement`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-unmanaged-app`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 67; sourceText: A helper process launched by a known app but not mapped to a supported app identity.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

45. Represent a mobile app that the platform does not expose through the approved parental, enterprise, or device-owner APIs?

- settingId: `app-guide-core-terms-unmanaged-app-005`
- policyLane: `approvals`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-unmanaged-app`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 69; sourceText: A mobile app that the platform does not expose through the approved parental, enterprise, or device-owner APIs.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

46. Represent an app running on an unsupported platform adapter?

- settingId: `app-guide-core-terms-unmanaged-app-006`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-unmanaged-app`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 71; sourceText: An app running on an unsupported platform adapter.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

#### app-guide-core-terms-app-identity

47. Represent package id, bundle id, AppUserModelID, package family name, desktop entry id, or application token?

- settingId: `app-guide-core-terms-app-identity-001`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-identity`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 91; sourceText: Package id, bundle id, AppUserModelID, package family name, desktop entry id, or application token.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

48. Represent executable path, file hash, publisher/signature, product name, or version?

- settingId: `app-guide-core-terms-app-identity-002`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-identity`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 93; sourceText: Executable path, file hash, publisher/signature, product name, or version.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

49. Represent process id and parent process id for a running observation?

- settingId: `app-guide-core-terms-app-identity-003`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-identity`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 94; sourceText: Process id and parent process id for a running observation.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

50. Represent window id and foreground state for active-use evidence?

- settingId: `app-guide-core-terms-app-identity-004`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-identity`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 95; sourceText: Window id and foreground state for active-use evidence.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

51. Represent installer/source reference and install state for inventory evidence?

- settingId: `app-guide-core-terms-app-identity-005`
- policyLane: `setup`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-identity`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 96; sourceText: Installer/source reference and install state for inventory evidence.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

#### app-guide-core-terms-app-session-evidence

52. Represent app was observed?

- settingId: `app-guide-core-terms-app-session-evidence-001`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-session-evidence`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 107; sourceText: App was observed.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

53. Represent app was running?

- settingId: `app-guide-core-terms-app-session-evidence-002`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-session-evidence`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 108; sourceText: App was running.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

54. Represent app was foreground-active where foreground proof exists?

- settingId: `app-guide-core-terms-app-session-evidence-003`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-session-evidence`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 109; sourceText: App was foreground-active where foreground proof exists.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

55. Represent app had a running or foreground duration inside a time window?

- settingId: `app-guide-core-terms-app-session-evidence-004`
- policyLane: `schedule`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-session-evidence`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 110; sourceText: App had a running or foreground duration inside a time window.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

56. Represent app identity, category, or unknown state was derived from specific evidence ids?

- settingId: `app-guide-core-terms-app-session-evidence-005`
- policyLane: `evidence`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-session-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 111; sourceText: App identity, category, or unknown state was derived from specific evidence ids.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### app-guide-core-terms-app-control-action

57. Represent launch?

- settingId: `app-guide-core-terms-app-control-action-001`
- policyLane: `enforcement`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-control-action`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 123; sourceText: Launch.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

58. Represent warn?

- settingId: `app-guide-core-terms-app-control-action-002`
- policyLane: `rules`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-control-action`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 124; sourceText: Warn.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

59. Represent ask parent?

- settingId: `app-guide-core-terms-app-control-action-003`
- policyLane: `approvals`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-control-action`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 125; sourceText: Ask parent.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

60. Represent terminate owned or target process?

- settingId: `app-guide-core-terms-app-control-action-004`
- policyLane: `enforcement`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-control-action`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 126; sourceText: Terminate owned or target process.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

61. Represent suspend, hide, shield, or block app where the platform supports it?

- settingId: `app-guide-core-terms-app-control-action-005`
- policyLane: `enforcement`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-control-action`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 127; sourceText: Suspend, hide, shield, or block app where the platform supports it.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

62. Represent start, extend, expire, or roll back a time limit?

- settingId: `app-guide-core-terms-app-control-action-006`
- policyLane: `schedule`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-control-action`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 128; sourceText: Start, extend, expire, or roll back a time limit.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

63. Represent install or uninstall a managed app where the platform and custody model allow it?

- settingId: `app-guide-core-terms-app-control-action-007`
- policyLane: `audit`; sectionId: `app-guide-core-terms`; groupId: `app-guide-core-terms-app-control-action`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 129; sourceText: Install or uninstall a managed app where the platform and custody model allow it.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Section: app-guide-the-main-capability-truth

### app-guide-the-main-capability-truth

#### app-guide-the-main-capability-truth-the-main-capability-truth

64. Configure inventory layer.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-001`
- policyLane: `setup`; sectionId: `app-guide-the-main-capability-truth`; groupId: `app-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 148; sourceText: Inventory layer: app appears installed, launchable, removable, managed, or unknown.
- acceptedOptions: App Appears Installed | Launchable | Removable | Managed | Unknown
- helperText: strict app control requires real platform adapter or managed-device proof.

65. Configure runtime layer.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-002`
- policyLane: `schedule`; sectionId: `app-guide-the-main-capability-truth`; groupId: `app-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 150; sourceText: Runtime layer: process/package/app session is running, foreground, background, stale, or unavailable.
- acceptedOptions: Process/package/app Session Is Running | Foreground | Background | Stale | Unavailable
- helperText: app claims require fresh evidence references with confidence and custody.

66. Configure duration layer.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-003`
- policyLane: `schedule`; sectionId: `app-guide-the-main-capability-truth`; groupId: `app-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 152; sourceText: Duration layer: running and foreground time are derived from stored evidence, not portal refresh.
- acceptedOptions: Running And Foreground Time Are Derived From Stored Evidence | Not Portal Refresh
- helperText: app claims require fresh evidence references with confidence and custody.

67. Configure policy layer.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-004`
- policyLane: `schedule`; sectionId: `app-guide-the-main-capability-truth`; groupId: `app-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 154; sourceText: Policy layer: parent rule matches app identity, category, unknown state, schedule, budget, or approval state.
- acceptedOptions: Parent Rule Matches App Identity | Category | Unknown State | Schedule | Budget | Approval State
- helperText: app claims require fresh evidence references with confidence and custody.

68. Configure enforcement layer.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-005`
- policyLane: `schedule`; sectionId: `app-guide-the-main-capability-truth`; groupId: `app-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 156; sourceText: Enforcement layer: child-device adapter executes terminate, block, shield, suspend, hide, launch, install, uninstall, or time-limit actions.
- acceptedOptions: Child Device Adapter Executes Terminate | Block | Shield | Suspend | Hide | Launch | Install | Uninstall | Time Limit Actions
- helperText: strict app control requires real platform adapter or managed-device proof.

69. Configure audit layer.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-006`
- policyLane: `reports`; sectionId: `app-guide-the-main-capability-truth`; groupId: `app-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 158; sourceText: Audit layer: every parent-visible claim carries evidence source, custody, policy decision, adapter result, and capability status.
- acceptedOptions: Every Parent Visible Claim Carries Evidence Source | Custody | Policy Decision | Adapter Result | And Capability Status
- helperText: app claims require fresh evidence references with confidence and custody.

## Section: app-guide-capability-matrix

### app-guide-capability-matrix

#### app-guide-capability-matrix-capability-matrix

70. Represent capability matrix row | Capability=Installed app inventory | Windows=Yes, partial by source | macOS=Yes, partial by source | Linux=Yes, partial by distro/desktop | Android=Yes, visibility-limited | iOS/iPadOS=Limited, token/MDM-managed paths | Required proof=Inventory adapter and source ids | Important limit=Inventory is not proof of current use?

- settingId: `app-guide-capability-matrix-capability-matrix-001`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 169; sourceText: Capability matrix row | Capability=Installed app inventory | Windows=Yes, partial by source | macOS=Yes, partial by source | Linux=Yes, partial by distro/desktop | Android=Yes, visibility-limited | iOS/iPadOS=Limited, token/MDM-managed paths | Required proof=Inventory adapter and source ids | Important limit=Inventory is not proof of current use.
- acceptedOptions: Capability: Installed app inventory | Windows: Yes, partial by source | macOS: Yes, partial by source | Linux: Yes, partial by distro/desktop | Android: Yes, visibility-limited | iOS/iPadOS: Limited, token/MDM-managed paths | Required proof: Inventory adapter and source ids | Important limit: Inventory is not proof of current use.
- helperText: strict app control requires real platform adapter or managed-device proof.

71. Represent capability matrix row | Capability=Package/process identity | Windows=Strong for observed processes/packages | macOS=Strong for bundles/processes | Linux=Varies by package and desktop entry | Android=Strong package id when visible | iOS/iPadOS=Opaque tokens or managed app metadata | Required proof=Identity fields plus confidence | Important limit=Renames, helpers, and wrappers reduce confidence?

- settingId: `app-guide-capability-matrix-capability-matrix-002`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 170; sourceText: Capability matrix row | Capability=Package/process identity | Windows=Strong for observed processes/packages | macOS=Strong for bundles/processes | Linux=Varies by package and desktop entry | Android=Strong package id when visible | iOS/iPadOS=Opaque tokens or managed app metadata | Required proof=Identity fields plus confidence | Important limit=Renames, helpers, and wrappers reduce confidence.
- acceptedOptions: Capability: Package/process identity | Windows: Strong for observed processes/packages | macOS: Strong for bundles/processes | Linux: Varies by package and desktop entry | Android: Strong package id when visible | iOS/iPadOS: Opaque tokens or managed app metadata | Required proof: Identity fields plus confidence | Important limit: Renames, helpers, and wrappers reduce confidence.
- helperText: app claims require fresh evidence references with confidence and custody.

72. Represent capability matrix row | Capability=Running app observation | Windows=Yes | macOS=Yes | Linux=Yes | Android=Limited; usage/accessibility/DO paths | iOS/iPadOS=Limited through Screen Time/MDM signals | Required proof=Runtime observation evidence | Important limit=Background services may not equal user-facing app use?

- settingId: `app-guide-capability-matrix-capability-matrix-003`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 171; sourceText: Capability matrix row | Capability=Running app observation | Windows=Yes | macOS=Yes | Linux=Yes | Android=Limited; usage/accessibility/DO paths | iOS/iPadOS=Limited through Screen Time/MDM signals | Required proof=Runtime observation evidence | Important limit=Background services may not equal user-facing app use.
- acceptedOptions: Capability: Running app observation | Windows: Yes | macOS: Yes | Linux: Yes | Android: Limited; usage/accessibility/DO paths | iOS/iPadOS: Limited through Screen Time/MDM signals | Required proof: Runtime observation evidence | Important limit: Background services may not equal user-facing app use.
- helperText: strict app control requires real platform adapter or managed-device proof.

73. Represent capability matrix row | Capability=Foreground app evidence | Windows=Yes | macOS=Permission-dependent | Linux=Desktop-environment-dependent | Android=Usage stats/accessibility-dependent | iOS/iPadOS=Device Activity thresholds, not raw focus | Required proof=Fresh foreground or activity evidence | Important limit=Foreground does not prove in-app content?

- settingId: `app-guide-capability-matrix-capability-matrix-004`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 172; sourceText: Capability matrix row | Capability=Foreground app evidence | Windows=Yes | macOS=Permission-dependent | Linux=Desktop-environment-dependent | Android=Usage stats/accessibility-dependent | iOS/iPadOS=Device Activity thresholds, not raw focus | Required proof=Fresh foreground or activity evidence | Important limit=Foreground does not prove in-app content.
- acceptedOptions: Capability: Foreground app evidence | Windows: Yes | macOS: Permission-dependent | Linux: Desktop-environment-dependent | Android: Usage stats/accessibility-dependent | iOS/iPadOS: Device Activity thresholds, not raw focus | Required proof: Fresh foreground or activity evidence | Important limit: Foreground does not prove in-app content.
- helperText: app claims require fresh evidence references with confidence and custody.

74. Represent capability matrix row | Capability=Running duration | Windows=Yes | macOS=Yes | Linux=Yes | Android=Usage-stat/session-dependent | iOS/iPadOS=Device Activity threshold-based | Required proof=Ordered observations/session model | Important limit=Gaps and restarts need stale handling?

- settingId: `app-guide-capability-matrix-capability-matrix-005`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 173; sourceText: Capability matrix row | Capability=Running duration | Windows=Yes | macOS=Yes | Linux=Yes | Android=Usage-stat/session-dependent | iOS/iPadOS=Device Activity threshold-based | Required proof=Ordered observations/session model | Important limit=Gaps and restarts need stale handling.
- acceptedOptions: Capability: Running duration | Windows: Yes | macOS: Yes | Linux: Yes | Android: Usage-stat/session-dependent | iOS/iPadOS: Device Activity threshold-based | Required proof: Ordered observations/session model | Important limit: Gaps and restarts need stale handling.
- helperText: app claims require fresh evidence references with confidence and custody.

75. Represent capability matrix row | Capability=Foreground duration | Windows=Yes | macOS=Permission-dependent | Linux=Desktop-environment-dependent | Android=Usage-stat/accessibility-dependent | iOS/iPadOS=Threshold/event-based | Required proof=Foreground observations or platform events | Important limit=Portal polling must not count time?

- settingId: `app-guide-capability-matrix-capability-matrix-006`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 174; sourceText: Capability matrix row | Capability=Foreground duration | Windows=Yes | macOS=Permission-dependent | Linux=Desktop-environment-dependent | Android=Usage-stat/accessibility-dependent | iOS/iPadOS=Threshold/event-based | Required proof=Foreground observations or platform events | Important limit=Portal polling must not count time.
- acceptedOptions: Capability: Foreground duration | Windows: Yes | macOS: Permission-dependent | Linux: Desktop-environment-dependent | Android: Usage-stat/accessibility-dependent | iOS/iPadOS: Threshold/event-based | Required proof: Foreground observations or platform events | Important limit: Portal polling must not count time.
- helperText: app claims require fresh evidence references with confidence and custody.

76. Represent capability matrix row | Capability=App categories | Windows=Derived from catalog/package/source | macOS=Derived from catalog/package/source | Linux=Derived from desktop/package metadata | Android=Package/category where exposed | iOS/iPadOS=Opaque category tokens through Screen Time | Required proof=Category source and confidence | Important limit=Category labels are policy inputs, not hidden blocks?

- settingId: `app-guide-capability-matrix-capability-matrix-007`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 175; sourceText: Capability matrix row | Capability=App categories | Windows=Derived from catalog/package/source | macOS=Derived from catalog/package/source | Linux=Derived from desktop/package metadata | Android=Package/category where exposed | iOS/iPadOS=Opaque category tokens through Screen Time | Required proof=Category source and confidence | Important limit=Category labels are policy inputs, not hidden blocks.
- acceptedOptions: Capability: App categories | Windows: Derived from catalog/package/source | macOS: Derived from catalog/package/source | Linux: Derived from desktop/package metadata | Android: Package/category where exposed | iOS/iPadOS: Opaque category tokens through Screen Time | Required proof: Category source and confidence | Important limit: Category labels are policy inputs, not hidden blocks.
- helperText: app claims require fresh evidence references with confidence and custody.

77. Represent capability matrix row | Capability=Launch app | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes, with package intents where allowed | iOS/iPadOS=Limited; open intents/managed flows | Required proof=Launch adapter result | Important limit=Launch does not imply ongoing control?

- settingId: `app-guide-capability-matrix-capability-matrix-008`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 176; sourceText: Capability matrix row | Capability=Launch app | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes, with package intents where allowed | iOS/iPadOS=Limited; open intents/managed flows | Required proof=Launch adapter result | Important limit=Launch does not imply ongoing control.
- acceptedOptions: Capability: Launch app | Windows: Yes | macOS: Yes | Linux: Yes | Android: Yes, with package intents where allowed | iOS/iPadOS: Limited; open intents/managed flows | Required proof: Launch adapter result | Important limit: Launch does not imply ongoing control.
- helperText: app claims require fresh evidence references with confidence and custody.

78. Represent capability matrix row | Capability=Terminate app | Windows=Yes, where permission permits | macOS=Yes, where permission permits | Linux=Yes, where permission permits | Android=Limited; device-owner/admin paths | iOS/iPadOS=No general third-party terminate | Required proof=Target identity and adapter result | Important limit=Unsaved data and race conditions need UX/audit?

- settingId: `app-guide-capability-matrix-capability-matrix-009`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 177; sourceText: Capability matrix row | Capability=Terminate app | Windows=Yes, where permission permits | macOS=Yes, where permission permits | Linux=Yes, where permission permits | Android=Limited; device-owner/admin paths | iOS/iPadOS=No general third-party terminate | Required proof=Target identity and adapter result | Important limit=Unsaved data and race conditions need UX/audit.
- acceptedOptions: Capability: Terminate app | Windows: Yes, where permission permits | macOS: Yes, where permission permits | Linux: Yes, where permission permits | Android: Limited; device-owner/admin paths | iOS/iPadOS: No general third-party terminate | Required proof: Target identity and adapter result | Important limit: Unsaved data and race conditions need UX/audit.
- helperText: strict app control requires real platform adapter or managed-device proof.

79. Represent capability matrix row | Capability=Suspend/hide/shield app | Windows=App control policy dependent | macOS=MDM/profile dependent | Linux=Desktop/policy dependent | Android=Device owner/profile owner capable | iOS/iPadOS=Screen Time/Managed Settings capable | Required proof=Platform management proof | Important limit=Mobile support depends on entitlements/setup?

- settingId: `app-guide-capability-matrix-capability-matrix-010`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 178; sourceText: Capability matrix row | Capability=Suspend/hide/shield app | Windows=App control policy dependent | macOS=MDM/profile dependent | Linux=Desktop/policy dependent | Android=Device owner/profile owner capable | iOS/iPadOS=Screen Time/Managed Settings capable | Required proof=Platform management proof | Important limit=Mobile support depends on entitlements/setup.
- acceptedOptions: Capability: Suspend/hide/shield app | Windows: App control policy dependent | macOS: MDM/profile dependent | Linux: Desktop/policy dependent | Android: Device owner/profile owner capable | iOS/iPadOS: Screen Time/Managed Settings capable | Required proof: Platform management proof | Important limit: Mobile support depends on entitlements/setup.
- helperText: strict app control requires real platform adapter or managed-device proof.

80. Represent capability matrix row | Capability=Block launch | Windows=AppLocker/WDAC or similar proof required | macOS=MDM/system policy proof required | Linux=Policy/permission proof required | Android=Device owner/profile owner capable | iOS/iPadOS=Screen Time shield or MDM restriction | Required proof=Pre-launch enforcement proof | Important limit=Current repo must not claim broad blocking without proof?

- settingId: `app-guide-capability-matrix-capability-matrix-011`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 179; sourceText: Capability matrix row | Capability=Block launch | Windows=AppLocker/WDAC or similar proof required | macOS=MDM/system policy proof required | Linux=Policy/permission proof required | Android=Device owner/profile owner capable | iOS/iPadOS=Screen Time shield or MDM restriction | Required proof=Pre-launch enforcement proof | Important limit=Current repo must not claim broad blocking without proof.
- acceptedOptions: Capability: Block launch | Windows: AppLocker/WDAC or similar proof required | macOS: MDM/system policy proof required | Linux: Policy/permission proof required | Android: Device owner/profile owner capable | iOS/iPadOS: Screen Time shield or MDM restriction | Required proof: Pre-launch enforcement proof | Important limit: Current repo must not claim broad blocking without proof.
- helperText: strict app control requires real platform adapter or managed-device proof.

81. Represent capability matrix row | Capability=Time-limit app use | Windows=Yes for app sessions and owned terminate | macOS=Possible with platform proof | Linux=Possible with platform proof | Android=Usage/DevicePolicy/Accessibility proof | iOS/iPadOS=Device Activity threshold/shield path | Required proof=Timer plus action/result audit | Important limit=Needs fallback when action cannot enforce?

- settingId: `app-guide-capability-matrix-capability-matrix-012`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 180; sourceText: Capability matrix row | Capability=Time-limit app use | Windows=Yes for app sessions and owned terminate | macOS=Possible with platform proof | Linux=Possible with platform proof | Android=Usage/DevicePolicy/Accessibility proof | iOS/iPadOS=Device Activity threshold/shield path | Required proof=Timer plus action/result audit | Important limit=Needs fallback when action cannot enforce.
- acceptedOptions: Capability: Time-limit app use | Windows: Yes for app sessions and owned terminate | macOS: Possible with platform proof | Linux: Possible with platform proof | Android: Usage/DevicePolicy/Accessibility proof | iOS/iPadOS: Device Activity threshold/shield path | Required proof: Timer plus action/result audit | Important limit: Needs fallback when action cannot enforce.
- helperText: strict app control requires real platform adapter or managed-device proof.

82. Represent capability matrix row | Capability=Install app | Windows=Installer/package manager path | macOS=Installer/MDM/package path | Linux=Package manager path | Android=Package installer/device owner/MDM | iOS/iPadOS=MDM/App Store managed distribution | Required proof=Install adapter/custody proof | Important limit=User consent, store policy, and signing matter?

- settingId: `app-guide-capability-matrix-capability-matrix-013`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 181; sourceText: Capability matrix row | Capability=Install app | Windows=Installer/package manager path | macOS=Installer/MDM/package path | Linux=Package manager path | Android=Package installer/device owner/MDM | iOS/iPadOS=MDM/App Store managed distribution | Required proof=Install adapter/custody proof | Important limit=User consent, store policy, and signing matter.
- acceptedOptions: Capability: Install app | Windows: Installer/package manager path | macOS: Installer/MDM/package path | Linux: Package manager path | Android: Package installer/device owner/MDM | iOS/iPadOS: MDM/App Store managed distribution | Required proof: Install adapter/custody proof | Important limit: User consent, store policy, and signing matter.
- helperText: strict app control requires real platform adapter or managed-device proof.

83. Represent capability matrix row | Capability=Uninstall app | Windows=Installer/package manager path | macOS=Installer/MDM/package path | Linux=Package manager path | Android=Device owner/MDM/package path | iOS/iPadOS=MDM-managed app removal only | Required proof=Removal adapter/custody proof | Important limit=Personal app removal is often not available?

- settingId: `app-guide-capability-matrix-capability-matrix-014`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 182; sourceText: Capability matrix row | Capability=Uninstall app | Windows=Installer/package manager path | macOS=Installer/MDM/package path | Linux=Package manager path | Android=Device owner/MDM/package path | iOS/iPadOS=MDM-managed app removal only | Required proof=Removal adapter/custody proof | Important limit=Personal app removal is often not available.
- acceptedOptions: Capability: Uninstall app | Windows: Installer/package manager path | macOS: Installer/MDM/package path | Linux: Package manager path | Android: Device owner/MDM/package path | iOS/iPadOS: MDM-managed app removal only | Required proof: Removal adapter/custody proof | Important limit: Personal app removal is often not available.
- helperText: strict app control requires real platform adapter or managed-device proof.

84. Represent capability matrix row | Capability=Child-facing message | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes | iOS/iPadOS=Shield UI where supported | Required proof=Local UI/notification/shield result | Important limit=Do not show parent diagnostics to child?

- settingId: `app-guide-capability-matrix-capability-matrix-015`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 183; sourceText: Capability matrix row | Capability=Child-facing message | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes | iOS/iPadOS=Shield UI where supported | Required proof=Local UI/notification/shield result | Important limit=Do not show parent diagnostics to child.
- acceptedOptions: Capability: Child-facing message | Windows: Yes | macOS: Yes | Linux: Yes | Android: Yes | iOS/iPadOS: Shield UI where supported | Required proof: Local UI/notification/shield result | Important limit: Do not show parent diagnostics to child.
- helperText: strict app control requires real platform adapter or managed-device proof.

85. Represent capability matrix row | Capability=Parent report | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes, if evidence exists | iOS/iPadOS=Yes, token/capability-limited | Required proof=Stored evidence and custody labels | Important limit=Reports must distinguish raw vs redacted fields?

- settingId: `app-guide-capability-matrix-capability-matrix-016`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 184; sourceText: Capability matrix row | Capability=Parent report | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes, if evidence exists | iOS/iPadOS=Yes, token/capability-limited | Required proof=Stored evidence and custody labels | Important limit=Reports must distinguish raw vs redacted fields.
- acceptedOptions: Capability: Parent report | Windows: Yes | macOS: Yes | Linux: Yes | Android: Yes, if evidence exists | iOS/iPadOS: Yes, token/capability-limited | Required proof: Stored evidence and custody labels | Important limit: Reports must distinguish raw vs redacted fields.
- helperText: app claims require fresh evidence references with confidence and custody.

86. Represent capability matrix row | Capability=Audit/retention | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes | iOS/iPadOS=Yes | Required proof=Journal/query retention policy | Important limit=Local-first custody remains default?

- settingId: `app-guide-capability-matrix-capability-matrix-017`
- policyLane: `evidence`; sectionId: `app-guide-capability-matrix`; groupId: `app-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 185; sourceText: Capability matrix row | Capability=Audit/retention | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes | iOS/iPadOS=Yes | Required proof=Journal/query retention policy | Important limit=Local-first custody remains default.
- acceptedOptions: Capability: Audit/retention | Windows: Yes | macOS: Yes | Linux: Yes | Android: Yes | iOS/iPadOS: Yes | Required proof: Journal/query retention policy | Important limit: Local-first custody remains default.
- helperText: app claims require fresh evidence references with confidence and custody.

## Section: app-guide-app-evidence-what-is-possible

### app-guide-app-evidence-what-is-possible

#### app-guide-app-evidence-what-is-possible-installed-app-inventory

87. Represent app display name where safe?

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-001`
- policyLane: `setup`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 196; sourceText: App display name where safe.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

88. Represent package id, bundle id, package family name, desktop entry id, AppUserModelID, or app token?

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-002`
- policyLane: `setup`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 197; sourceText: Package id, bundle id, package family name, desktop entry id, AppUserModelID, or app token.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

89. Configure install source.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-003`
- policyLane: `setup`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-many`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 199; sourceText: Install source: installer, store package, app bundle, desktop entry, package manager, managed app distribution, or unknown.
- acceptedOptions: Installer | Store Package | App Bundle | Desktop Entry | Package Manager | Managed App Distribution | Unknown
- helperText: strict app control requires real platform adapter or managed-device proof.

90. Represent version, publisher, signature, hash, install path, and executable path where available and policy permits?

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-004`
- policyLane: `setup`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 201; sourceText: Version, publisher, signature, hash, install path, and executable path where available and policy permits.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

91. Represent category metadata from platform, catalog, desktop entry, app store metadata, or parent-maintained catalog?

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-005`
- policyLane: `setup`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 203; sourceText: Category metadata from platform, catalog, desktop entry, app store metadata, or parent-maintained catalog.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

92. Represent install, update, uninstall, hidden, suspended, shielded, managed, unmanaged, unsupported, or permission-limited state where the platform exposes it?

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-006`
- policyLane: `setup`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 205; sourceText: Install, update, uninstall, hidden, suspended, shielded, managed, unmanaged, unsupported, or permission-limited state where the platform exposes it.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

93. Represent it is partial on every platform?

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-007`
- policyLane: `setup`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 210; sourceText: It is partial on every platform.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

94. Represent it can miss portable apps, per-user installs, wrapped apps, web apps, and apps hidden by platform privacy?

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-008`
- policyLane: `setup`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 211; sourceText: It can miss portable apps, per-user installs, wrapped apps, web apps, and apps hidden by platform privacy.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

95. Represent it can report apps that are installed but never used?

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-009`
- policyLane: `reports`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 213; sourceText: It can report apps that are installed but never used.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

96. Represent mobile app lists can be package-visibility-limited, tokenized, supervised-only, or MDM-only?

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-010`
- policyLane: `setup`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 214; sourceText: Mobile app lists can be package-visibility-limited, tokenized, supervised-only, or MDM-only.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

97. Represent inventory should never be used as proof of activity without runtime evidence?

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-011`
- policyLane: `schedule`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-installed-app-inventory`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 216; sourceText: Inventory should never be used as proof of activity without runtime evidence.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

#### app-guide-app-evidence-what-is-possible-process-and-window-evidence

98. Represent process id, parent process id, executable path, process name, command-line handling status, user/session reference, and launch time where available?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-001`
- policyLane: `schedule`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 222; sourceText: Process id, parent process id, executable path, process name, command-line handling status, user/session reference, and launch time where available.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

99. Represent publisher/signature/hash metadata where safe?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-002`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 224; sourceText: Publisher/signature/hash metadata where safe.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

100.  Represent window id, title, active/foreground state, minimized/background state, and last foreground timestamp where available?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-003`
- policyLane: `schedule`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 225; sourceText: Window id, title, active/foreground state, minimized/background state, and last foreground timestamp where available.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

101.  Represent sessionization into running and foreground durations?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-004`
- policyLane: `schedule`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 227; sourceText: Sessionization into running and foreground durations.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

102.  Represent unknown, permission-limited, stale, unsupported, and adapter-error states?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-005`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 228; sourceText: Unknown, permission-limited, stale, unsupported, and adapter-error states.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

103.  Represent process names can be renamed?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-006`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 232; sourceText: Process names can be renamed.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

104.  Represent helper processes may not represent user-facing apps?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-007`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 233; sourceText: Helper processes may not represent user-facing apps.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

105.  Represent foreground window title may contain sensitive text and may be stale or misleading?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-008`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 234; sourceText: Foreground window title may contain sensitive text and may be stale or misleading.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

106.  Represent foreground proof does not reveal what happened inside the app?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-009`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 236; sourceText: Foreground proof does not reveal what happened inside the app.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

107.  Represent background process duration is not the same as child attention?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-010`
- policyLane: `schedule`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 237; sourceText: Background process duration is not the same as child attention.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

108.  Represent elevated, protected, sandboxed, or cross-user processes can be unreadable or uncontrollable?

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-011`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 238; sourceText: Elevated, protected, sandboxed, or cross-user processes can be unreadable or uncontrollable.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### app-guide-app-evidence-what-is-possible-foreground-use-and-duration

109.  Which app is active now?

- settingId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration-001`
- policyLane: `schedule`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 245; sourceText: Which app is active now?
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

110.  How long was this app in foreground today?

- settingId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration-002`
- policyLane: `schedule`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 246; sourceText: How long was this app in foreground today?
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

111.  Did the time budget run out?

- settingId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration-003`
- policyLane: `schedule`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 247; sourceText: Did the time budget run out?
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

112.  Which evidence ids prove the count?

- settingId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration-004`
- policyLane: `schedule`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 248; sourceText: Which evidence ids prove the count?
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### app-guide-app-evidence-what-is-possible-app-categories

113.  Represent education?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-001`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 264; sourceText: Education.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

114.  Represent productivity?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-002`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 265; sourceText: Productivity.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

115.  Represent communication?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-003`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 266; sourceText: Communication.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

116.  Represent entertainment?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-004`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 267; sourceText: Entertainment.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

117.  Represent social?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-005`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 268; sourceText: Social.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

118.  Represent browser?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-006`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 269; sourceText: Browser.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

119.  Represent game?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-007`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 270; sourceText: Game.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

120.  Represent creative?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-008`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 271; sourceText: Creative.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

121.  Represent system?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-009`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 272; sourceText: System.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

122.  Represent unknown?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-010`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 273; sourceText: Unknown.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

123.  Represent category is not content?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-011`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 280; sourceText: Category is not content.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

124.  Represent category confidence must be recorded?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-012`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 281; sourceText: Category confidence must be recorded.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

125.  Represent parent rules decide actions. Category labels alone should not block?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-013`
- policyLane: `enforcement`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 282; sourceText: Parent rules decide actions. Category labels alone should not block.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

126.  Represent some platforms expose categories as opaque tokens rather than raw identifiers?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-014`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 283; sourceText: Some platforms expose categories as opaque tokens rather than raw identifiers.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

127.  Represent unknown or ambiguous categories should degrade to observe, ask, or parent review according to explicit policy?

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-015`
- policyLane: `evidence`; sectionId: `app-guide-app-evidence-what-is-possible`; groupId: `app-guide-app-evidence-what-is-possible-app-categories`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 284; sourceText: Unknown or ambiguous categories should degrade to observe, ask, or parent review according to explicit policy.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Section: app-guide-app-control-what-is-possible

### app-guide-app-control-what-is-possible

#### app-guide-app-control-what-is-possible-launch

128.  Represent open an approved app?

- settingId: `app-guide-app-control-what-is-possible-launch-001`
- policyLane: `approvals`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-launch`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 296; sourceText: Open an approved app.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

129.  Represent relaunch a blocked/closed app later after a time budget resets?

- settingId: `app-guide-app-control-what-is-possible-launch-002`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-launch`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 297; sourceText: Relaunch a blocked/closed app later after a time budget resets.
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

130.  Represent open an app as part of an ask-parent approval?

- settingId: `app-guide-app-control-what-is-possible-launch-003`
- policyLane: `approvals`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-launch`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 298; sourceText: Open an app as part of an ask-parent approval.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

131.  Represent prefer a managed app path or managed browser path for certain tasks?

- settingId: `app-guide-app-control-what-is-possible-launch-004`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-launch`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 299; sourceText: Prefer a managed app path or managed browser path for certain tasks.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

132.  Represent launching an app does not guarantee it stays foreground?

- settingId: `app-guide-app-control-what-is-possible-launch-005`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-launch`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 303; sourceText: Launching an app does not guarantee it stays foreground.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

133.  Represent launching an unmanaged app can move outside Ocentra control?

- settingId: `app-guide-app-control-what-is-possible-launch-006`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-launch`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 304; sourceText: Launching an unmanaged app can move outside Ocentra control.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

134.  Represent mobile launch behavior depends on platform foreground and intent rules?

- settingId: `app-guide-app-control-what-is-possible-launch-007`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-launch`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 305; sourceText: Mobile launch behavior depends on platform foreground and intent rules.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### app-guide-app-control-what-is-possible-terminate

135.  Represent stop an app after a block or time-limit decision?

- settingId: `app-guide-app-control-what-is-possible-terminate-001`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-terminate`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 311; sourceText: Stop an app after a block or time-limit decision.
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

136.  Represent stop an owned child process?

- settingId: `app-guide-app-control-what-is-possible-terminate-002`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-terminate`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 312; sourceText: Stop an owned child process.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

137.  Represent stop a target process when identity still matches the policy target?

- settingId: `app-guide-app-control-what-is-possible-terminate-003`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-terminate`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 313; sourceText: Stop a target process when identity still matches the policy target.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

138.  Represent record already-exited, target-changed, permission-limited, failed, or succeeded results?

- settingId: `app-guide-app-control-what-is-possible-terminate-004`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-terminate`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 314; sourceText: Record already-exited, target-changed, permission-limited, failed, or succeeded results.
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

139.  Represent termination can lose unsaved work?

- settingId: `app-guide-app-control-what-is-possible-terminate-005`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-terminate`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 319; sourceText: Termination can lose unsaved work.
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

140.  Represent target processes can exit and relaunch between detection and action?

- settingId: `app-guide-app-control-what-is-possible-terminate-006`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-terminate`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 320; sourceText: Target processes can exit and relaunch between detection and action.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

141.  Represent parent/child UX should support grace periods, warnings, and ask-parent flows?

- settingId: `app-guide-app-control-what-is-possible-terminate-007`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-terminate`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 321; sourceText: Parent/child UX should support grace periods, warnings, and ask-parent flows.
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

142.  Represent some platforms do not allow third-party apps to kill other apps?

- settingId: `app-guide-app-control-what-is-possible-terminate-008`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-terminate`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 322; sourceText: Some platforms do not allow third-party apps to kill other apps.
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

143.  Represent protected, elevated, system, or different-user processes may be unavailable?

- settingId: `app-guide-app-control-what-is-possible-terminate-009`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-terminate`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 323; sourceText: Protected, elevated, system, or different-user processes may be unavailable.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### app-guide-app-control-what-is-possible-suspend-hide-shield-or-block

144.  Represent windows application control policy, AppLocker, WDAC/App Control for Business, or a narrower service adapter where proven?

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-001`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 332; sourceText: Windows application control policy, AppLocker, WDAC/App Control for Business, or a narrower service adapter where proven.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

145.  Represent macOS MDM profile, system extension, endpoint/security tooling, or managed app restriction where entitled and deployed?

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-002`
- policyLane: `setup`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 334; sourceText: macOS MDM profile, system extension, endpoint/security tooling, or managed app restriction where entitled and deployed.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

146.  Represent linux policy, desktop/session integration, package-manager restriction, or service-level control where proven?

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-003`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 336; sourceText: Linux policy, desktop/session integration, package-manager restriction, or service-level control where proven.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

147.  Represent android DevicePolicyManager package hide/suspend, device owner/profile owner, managed configuration, or accessibility/VPN-adjacent UX where approved?

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-004`
- policyLane: `approvals`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 338; sourceText: Android DevicePolicyManager package hide/suspend, device owner/profile owner, managed configuration, or accessibility/VPN-adjacent UX where approved.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

148.  Represent iOS/iPadOS Screen Time Family Controls, Managed Settings shields, Device Activity thresholds, or MDM restrictions where entitled/supervised?

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-005`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 340; sourceText: iOS/iPadOS Screen Time Family Controls, Managed Settings shields, Device Activity thresholds, or MDM restrictions where entitled/supervised.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

149.  Represent broad app blocking is a privileged OS capability, not a normal UI toggle?

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-006`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 345; sourceText: Broad app blocking is a privileged OS capability, not a normal UI toggle.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

150.  Represent policy setup can require admin rights, device-owner enrollment, MDM, supervision, entitlements, app review, signing, or store distribution?

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-007`
- policyLane: `setup`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 346; sourceText: Policy setup can require admin rights, device-owner enrollment, MDM, supervision, entitlements, app review, signing, or store distribution.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

151.  Represent some systems support shielding/visibility restrictions rather than process termination?

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-008`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 348; sourceText: Some systems support shielding/visibility restrictions rather than process termination.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

152.  Represent platform APIs may expose opaque identifiers for privacy?

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-009`
- policyLane: `enforcement`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 350; sourceText: Platform APIs may expose opaque identifiers for privacy.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

153.  Represent rollback and uninstall paths must be documented before strict policies ship?

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-010`
- policyLane: `setup`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 351; sourceText: Rollback and uninstall paths must be documented before strict policies ship.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

#### app-guide-app-control-what-is-possible-time-limits

154.  Represent app/session identity?

- settingId: `app-guide-app-control-what-is-possible-time-limits-001`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 359; sourceText: App/session identity.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

155.  Represent running or foreground duration proof?

- settingId: `app-guide-app-control-what-is-possible-time-limits-002`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 360; sourceText: Running or foreground duration proof.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

156.  Represent schedule and budget state?

- settingId: `app-guide-app-control-what-is-possible-time-limits-003`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 361; sourceText: Schedule and budget state.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

157.  Represent warning threshold and grace state?

- settingId: `app-guide-app-control-what-is-possible-time-limits-004`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 362; sourceText: Warning threshold and grace state.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

158.  Represent parent approval or extension state?

- settingId: `app-guide-app-control-what-is-possible-time-limits-005`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 363; sourceText: Parent approval or extension state.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

159.  Represent enforcement fallback for unsupported action?

- settingId: `app-guide-app-control-what-is-possible-time-limits-006`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `parent-domain`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 364; sourceText: Enforcement fallback for unsupported action.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

160.  Represent audit event for warning, timeout, action, failure, extension, and rollback?

- settingId: `app-guide-app-control-what-is-possible-time-limits-007`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 365; sourceText: Audit event for warning, timeout, action, failure, extension, and rollback.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

161.  Represent a timer without action is report-only?

- settingId: `app-guide-app-control-what-is-possible-time-limits-008`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 369; sourceText: A timer without action is report-only.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

162.  Represent foreground time and running time should be separate settings?

- settingId: `app-guide-app-control-what-is-possible-time-limits-009`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 370; sourceText: Foreground time and running time should be separate settings.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

163.  Represent cross-device time budgets need sync/custody rules?

- settingId: `app-guide-app-control-what-is-possible-time-limits-010`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 371; sourceText: Cross-device time budgets need sync/custody rules.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

164.  Represent mobile time-limit enforcement depends on platform-specific APIs?

- settingId: `app-guide-app-control-what-is-possible-time-limits-011`
- policyLane: `schedule`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-time-limits`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 372; sourceText: Mobile time-limit enforcement depends on platform-specific APIs.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

#### app-guide-app-control-what-is-possible-install-and-uninstall

165.  Represent windows MSI/MSIX/package manager or managed installer?

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-001`
- policyLane: `setup`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-install-and-uninstall`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 381; sourceText: Windows MSI/MSIX/package manager or managed installer.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

166.  Represent macOS installer/package/MDM managed app or declarative package management?

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-002`
- policyLane: `setup`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-install-and-uninstall`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 382; sourceText: macOS installer/package/MDM managed app or declarative package management.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

167.  Represent linux package manager, Flatpak, Snap, AppImage-managed wrapper, or desktop entry?

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-003`
- policyLane: `setup`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-install-and-uninstall`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 383; sourceText: Linux package manager, Flatpak, Snap, AppImage-managed wrapper, or desktop entry.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

168.  Represent android package installer, device owner/profile owner, managed Play, or MDM?

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-004`
- policyLane: `setup`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-install-and-uninstall`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 385; sourceText: Android package installer, device owner/profile owner, managed Play, or MDM.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

169.  Represent iOS/iPadOS MDM managed app distribution and managed app removal?

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-005`
- policyLane: `setup`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-install-and-uninstall`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 386; sourceText: iOS/iPadOS MDM managed app distribution and managed app removal.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

170.  Represent ocentra must not remove personal apps unless a platform-approved managed path and parent/child custody model explicitly allow it?

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-006`
- policyLane: `approvals`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-install-and-uninstall`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 390; sourceText: Ocentra must not remove personal apps unless a platform-approved managed path and parent/child custody model explicitly allow it.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

171.  Represent store policies, signing, entitlements, user consent, device enrollment, and uninstall rights vary sharply?

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-007`
- policyLane: `setup`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-install-and-uninstall`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 392; sourceText: Store policies, signing, entitlements, user consent, device enrollment, and uninstall rights vary sharply.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

172.  Represent install/uninstall actions must be audited separately from normal app observation?

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-008`
- policyLane: `audit`; sectionId: `app-guide-app-control-what-is-possible`; groupId: `app-guide-app-control-what-is-possible-install-and-uninstall`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 394; sourceText: Install/uninstall actions must be audited separately from normal app observation.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Section: app-guide-managed-unmanaged-and-unknown-apps

### app-guide-managed-unmanaged-and-unknown-apps

#### app-guide-managed-unmanaged-and-unknown-apps-managed-apps

173.  Represent known identity from package/bundle/app token?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps-001`
- policyLane: `evidence`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 403; sourceText: Known identity from package/bundle/app token.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

174.  Represent known policy source?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps-002`
- policyLane: `evidence`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 404; sourceText: Known policy source.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

175.  Represent install or update provenance?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps-003`
- policyLane: `setup`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 405; sourceText: Install or update provenance.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

176.  Represent known allowed/blocked/shielded state?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps-004`
- policyLane: `enforcement`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 406; sourceText: Known allowed/blocked/shielded state.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

177.  Represent stronger app lifecycle action where the platform supports it?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps-005`
- policyLane: `evidence`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 407; sourceText: Stronger app lifecycle action where the platform supports it.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps

178.  Represent running and foreground observation?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-001`
- policyLane: `evidence`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 416; sourceText: Running and foreground observation.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

179.  Represent session duration?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-002`
- policyLane: `schedule`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 417; sourceText: Session duration.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

180.  Represent category candidate?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-003`
- policyLane: `evidence`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 418; sourceText: Category candidate.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

181.  Represent ask-parent or warning events?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-004`
- policyLane: `evidence`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 419; sourceText: Ask-parent or warning events.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

182.  Represent terminate where allowed?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-005`
- policyLane: `enforcement`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 420; sourceText: Terminate where allowed.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

183.  Represent report-only unknown or bypass state?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-006`
- policyLane: `reports`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 421; sourceText: Report-only unknown or bypass state.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### app-guide-managed-unmanaged-and-unknown-apps-unknown-apps

184.  Represent observe only?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps-001`
- policyLane: `evidence`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 430; sourceText: Observe only.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

185.  Represent ask parent on first run?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps-002`
- policyLane: `approvals`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 431; sourceText: Ask parent on first run.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

186.  Represent warn child?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps-003`
- policyLane: `evidence`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 432; sourceText: Warn child.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

187.  Represent count time under unknown-app budget?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps-004`
- policyLane: `schedule`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 433; sourceText: Count time under unknown-app budget.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

188.  Represent block or terminate only when the parent selected that posture and the platform adapter can prove the action?

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps-005`
- policyLane: `enforcement`; sectionId: `app-guide-managed-unmanaged-and-unknown-apps`; groupId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 434; sourceText: Block or terminate only when the parent selected that posture and the platform adapter can prove the action.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Section: app-guide-child-facing-actions

### app-guide-child-facing-actions

#### app-guide-child-facing-actions-child-facing-actions

189.  Represent warn before time limit?

- settingId: `app-guide-child-facing-actions-child-facing-actions-001`
- policyLane: `schedule`; sectionId: `app-guide-child-facing-actions`; groupId: `app-guide-child-facing-actions-child-facing-actions`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 445; sourceText: Warn before time limit.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

190.  Represent show time remaining?

- settingId: `app-guide-child-facing-actions-child-facing-actions-002`
- policyLane: `schedule`; sectionId: `app-guide-child-facing-actions`; groupId: `app-guide-child-facing-actions-child-facing-actions`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 446; sourceText: Show time remaining.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

191.  Represent show that parent approval is needed?

- settingId: `app-guide-child-facing-actions-child-facing-actions-003`
- policyLane: `approvals`; sectionId: `app-guide-child-facing-actions`; groupId: `app-guide-child-facing-actions-child-facing-actions`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 447; sourceText: Show that parent approval is needed.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

192.  Represent show whether an app is paused, shielded, blocked, or closed by parent policy?

- settingId: `app-guide-child-facing-actions-child-facing-actions-004`
- policyLane: `enforcement`; sectionId: `app-guide-child-facing-actions`; groupId: `app-guide-child-facing-actions-child-facing-actions`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 448; sourceText: Show whether an app is paused, shielded, blocked, or closed by parent policy.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

193.  Represent offer ask-parent, request more time, or use allowed alternative where policy supports it?

- settingId: `app-guide-child-facing-actions-child-facing-actions-005`
- policyLane: `schedule`; sectionId: `app-guide-child-facing-actions`; groupId: `app-guide-child-facing-actions-child-facing-actions`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 449; sourceText: Offer ask-parent, request more time, or use allowed alternative where policy supports it.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

194.  Represent hide parent diagnostics, evidence ids, adapter errors, hashes, and internal policy fields from the child surface?

- settingId: `app-guide-child-facing-actions-child-facing-actions-006`
- policyLane: `enforcement`; sectionId: `app-guide-child-facing-actions`; groupId: `app-guide-child-facing-actions-child-facing-actions`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 451; sourceText: Hide parent diagnostics, evidence ids, adapter errors, hashes, and internal policy fields from the child surface.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Section: app-guide-reports-custody-retention-and-audit

### app-guide-reports-custody-retention-and-audit

#### app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit

195.  Represent installed/detectable app inventory?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 470; sourceText: Installed/detectable app inventory.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

196.  Represent running now?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-002`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 471; sourceText: Running now.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

197.  Represent foreground now?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-003`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 472; sourceText: Foreground now.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

198.  Represent recent sessions?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-004`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 473; sourceText: Recent sessions.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

199.  Represent daily app/category rollups?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-005`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 474; sourceText: Daily app/category rollups.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

200.  Represent time budgets and remaining time?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-006`
- policyLane: `schedule`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 475; sourceText: Time budgets and remaining time.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

201.  Represent unknown and permission-limited apps?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-007`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `permission-limited`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-limited`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 476; sourceText: Unknown and permission-limited apps.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

202.  Represent policy decisions?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-008`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 477; sourceText: Policy decisions.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

203.  Represent enforcement actions and failures?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-009`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 478; sourceText: Enforcement actions and failures.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

204.  Represent approval requests and parent responses?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-010`
- policyLane: `approvals`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 479; sourceText: Approval requests and parent responses.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

205.  Represent evidence ids?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-011`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 483; sourceText: Evidence ids.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

206.  Represent source adapter?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-012`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 484; sourceText: Source adapter.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

207.  Represent capability state?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-013`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 485; sourceText: Capability state.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

208.  Represent custody label?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-014`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 486; sourceText: Custody label.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

209.  Represent collection scope?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-015`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 487; sourceText: Collection scope.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

210.  Represent retention policy?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-016`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 488; sourceText: Retention policy.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

211.  Represent redaction status?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-017`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 489; sourceText: Redaction status.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

212.  Represent policy version and decision id when policy contributed?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-018`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 490; sourceText: Policy version and decision id when policy contributed.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

213.  Represent adapter result id when enforcement contributed?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-019`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 491; sourceText: Adapter result id when enforcement contributed.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

214.  Represent raw process/window evidence should be retained for the shortest useful local audit window?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-020`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 495; sourceText: Raw process/window evidence should be retained for the shortest useful local audit window.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

215.  Represent daily rollups can be retained longer than raw observations if they are redacted?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-021`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 497; sourceText: Daily rollups can be retained longer than raw observations if they are redacted.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

216.  Represent exact executable paths and window titles may be sensitive and should have narrower retention and reveal controls?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-022`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 499; sourceText: Exact executable paths and window titles may be sensitive and should have narrower retention and reveal controls.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

217.  Represent ocentra-hosted storage is not the default child-activity store?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-023`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 501; sourceText: Ocentra-hosted storage is not the default child-activity store.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

218.  Represent parent export and deletion must preserve audit integrity while respecting retention settings?

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-024`
- policyLane: `reports`; sectionId: `app-guide-reports-custody-retention-and-audit`; groupId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 502; sourceText: Parent export and deletion must preserve audit integrity while respecting retention settings.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

## Section: app-guide-platform-capability-notes

### app-guide-platform-capability-notes

#### app-guide-platform-capability-notes-windows

219.  Represent installed-app inventory from uninstall records, Start Menu shortcuts, Microsoft Store packages, known install paths, package query APIs, and executable metadata?

- settingId: `app-guide-platform-capability-notes-windows-001`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 514; sourceText: Installed-app inventory from uninstall records, Start Menu shortcuts, Microsoft Store packages, known install paths, package query APIs, and executable metadata.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

220.  Represent process enumeration and process metadata?

- settingId: `app-guide-platform-capability-notes-windows-002`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 517; sourceText: Process enumeration and process metadata.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

221.  Represent foreground-window observation?

- settingId: `app-guide-platform-capability-notes-windows-003`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 518; sourceText: Foreground-window observation.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

222.  Represent running and foreground sessionization?

- settingId: `app-guide-platform-capability-notes-windows-004`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 519; sourceText: Running and foreground sessionization.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

223.  Represent owned-process launch and termination?

- settingId: `app-guide-platform-capability-notes-windows-005`
- policyLane: `enforcement`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 520; sourceText: Owned-process launch and termination.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

224.  Represent narrow target process termination after typed policy decisions?

- settingId: `app-guide-platform-capability-notes-windows-006`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 521; sourceText: Narrow target process termination after typed policy decisions.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

225.  Represent broad app control through AppLocker, WDAC/App Control for Business, managed installer policy, or equivalent only after explicit host proof?

- settingId: `app-guide-platform-capability-notes-windows-007`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 522; sourceText: Broad app control through AppLocker, WDAC/App Control for Business, managed installer policy, or equivalent only after explicit host proof.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

226.  Represent package lifecycle actions through installer/package mechanisms where product setup owns the package?

- settingId: `app-guide-platform-capability-notes-windows-008`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 524; sourceText: Package lifecycle actions through installer/package mechanisms where product setup owns the package.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

227.  Represent appLocker/WDAC behavior depends on Windows edition, policy deployment, signing, administrator rights, audit/enforce mode, and reboot or refresh behavior?

- settingId: `app-guide-platform-capability-notes-windows-009`
- policyLane: `audit`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 529; sourceText: AppLocker/WDAC behavior depends on Windows edition, policy deployment, signing, administrator rights, audit/enforce mode, and reboot or refresh behavior.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

228.  Represent microsoft Store package identity and Win32 executable identity are different evidence families?

- settingId: `app-guide-platform-capability-notes-windows-010`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 531; sourceText: Microsoft Store package identity and Win32 executable identity are different evidence families.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

229.  Represent blocking by path alone can be bypassed by copy/rename unless hash, signer, or managed installer proof is used?

- settingId: `app-guide-platform-capability-notes-windows-011`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 533; sourceText: Blocking by path alone can be bypassed by copy/rename unless hash, signer, or managed installer proof is used.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

230.  Represent the current roadmap distinguishes owned-process terminate and app time-limit proof from broad app blocking. Do not claim broad blocking until the adapter is proven?

- settingId: `app-guide-platform-capability-notes-windows-012`
- policyLane: `schedule`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 535; sourceText: The current roadmap distinguishes owned-process terminate and app time-limit proof from broad app blocking. Do not claim broad blocking until the adapter is proven.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

#### app-guide-platform-capability-notes-macos

231.  Represent application bundle inventory?

- settingId: `app-guide-platform-capability-notes-macos-001`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 546; sourceText: Application bundle inventory.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

232.  Represent running process and window/frontmost app observation with the required permissions?

- settingId: `app-guide-platform-capability-notes-macos-002`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 547; sourceText: Running process and window/frontmost app observation with the required permissions.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

233.  Represent launch Services, bundle identifiers, code signing, and app metadata?

- settingId: `app-guide-platform-capability-notes-macos-003`
- policyLane: `enforcement`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 549; sourceText: Launch Services, bundle identifiers, code signing, and app metadata.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

234.  Represent mDM managed app distribution and restrictions where enrolled?

- settingId: `app-guide-platform-capability-notes-macos-004`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 550; sourceText: MDM managed app distribution and restrictions where enrolled.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

235.  Represent system Extensions, Endpoint Security, or Network Extension paths where entitled and deployed?

- settingId: `app-guide-platform-capability-notes-macos-005`
- policyLane: `rules`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 551; sourceText: System Extensions, Endpoint Security, or Network Extension paths where entitled and deployed.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

236.  Represent managed browser/app controls through configuration profiles where supported?

- settingId: `app-guide-platform-capability-notes-macos-006`
- policyLane: `rules`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 553; sourceText: Managed browser/app controls through configuration profiles where supported.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

237.  Represent accessibility, Screen Recording, Full Disk Access, Endpoint Security, Network Extension, and MDM posture change what is possible?

- settingId: `app-guide-platform-capability-notes-macos-007`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 557; sourceText: Accessibility, Screen Recording, Full Disk Access, Endpoint Security, Network Extension, and MDM posture change what is possible.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

238.  Represent some controls require supervised or managed devices?

- settingId: `app-guide-platform-capability-notes-macos-008`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 559; sourceText: Some controls require supervised or managed devices.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

239.  Represent do not assume Windows process control maps directly to macOS?

- settingId: `app-guide-platform-capability-notes-macos-009`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 560; sourceText: Do not assume Windows process control maps directly to macOS.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### app-guide-platform-capability-notes-linux

240.  Represent desktop entries and menu categories?

- settingId: `app-guide-platform-capability-notes-linux-001`
- policyLane: `rules`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 570; sourceText: Desktop entries and menu categories.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

241.  Represent package manager inventory?

- settingId: `app-guide-platform-capability-notes-linux-002`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 571; sourceText: Package manager inventory.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

242.  Represent flatpak, Snap, AppImage, or custom install metadata?

- settingId: `app-guide-platform-capability-notes-linux-003`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 572; sourceText: Flatpak, Snap, AppImage, or custom install metadata.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

243.  Represent process observation through OS process tables?

- settingId: `app-guide-platform-capability-notes-linux-004`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 573; sourceText: Process observation through OS process tables.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

244.  Represent foreground-window observation through X11, Wayland compositor protocols, or desktop-specific APIs where available?

- settingId: `app-guide-platform-capability-notes-linux-005`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 574; sourceText: Foreground-window observation through X11, Wayland compositor protocols, or desktop-specific APIs where available.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

245.  Represent process termination where permission permits?

- settingId: `app-guide-platform-capability-notes-linux-006`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 576; sourceText: Process termination where permission permits.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

246.  Represent policy controls through service, user session, package, desktop, firewall, or container mechanisms where proven?

- settingId: `app-guide-platform-capability-notes-linux-007`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 577; sourceText: Policy controls through service, user session, package, desktop, firewall, or container mechanisms where proven.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

247.  Represent wayland commonly restricts global window inspection compared with X11?

- settingId: `app-guide-platform-capability-notes-linux-008`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 582; sourceText: Wayland commonly restricts global window inspection compared with X11.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

248.  Represent desktop entry category metadata is useful but not a complete app ontology?

- settingId: `app-guide-platform-capability-notes-linux-009`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 583; sourceText: Desktop entry category metadata is useful but not a complete app ontology.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

249.  Represent package managers differ by distro?

- settingId: `app-guide-platform-capability-notes-linux-010`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 584; sourceText: Package managers differ by distro.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

250.  Represent broad app blocking should be treated as manual-required until a concrete adapter is proven on the target distro and desktop?

- settingId: `app-guide-platform-capability-notes-linux-011`
- policyLane: `enforcement`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 585; sourceText: Broad app blocking should be treated as manual-required until a concrete adapter is proven on the target distro and desktop.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

#### app-guide-platform-capability-notes-android

251.  Represent package inventory through PackageManager subject to package visibility rules?

- settingId: `app-guide-platform-capability-notes-android-001`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 595; sourceText: Package inventory through PackageManager subject to package visibility rules.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

252.  Represent usage events/statistics when the user grants Usage Access or the app has the required privileged context?

- settingId: `app-guide-platform-capability-notes-android-002`
- policyLane: `rules`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 596; sourceText: Usage events/statistics when the user grants Usage Access or the app has the required privileged context.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

253.  Represent foreground visibility through UsageStatsManager or Accessibility where approved and enabled?

- settingId: `app-guide-platform-capability-notes-android-003`
- policyLane: `approvals`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 598; sourceText: Foreground visibility through UsageStatsManager or Accessibility where approved and enabled.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

254.  Represent devicePolicyManager package hiding, suspension, permission policy, managed configuration, and package lifecycle control for device owner/profile owner contexts?

- settingId: `app-guide-platform-capability-notes-android-004`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 600; sourceText: DevicePolicyManager package hiding, suspension, permission policy, managed configuration, and package lifecycle control for device owner/profile owner contexts.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

255.  Represent managed Play or MDM package installation/removal where deployed?

- settingId: `app-guide-platform-capability-notes-android-005`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 603; sourceText: Managed Play or MDM package installation/removal where deployed.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

256.  Represent always-on VPN with lockdown for network mediation, separate from app foreground proof?

- settingId: `app-guide-platform-capability-notes-android-006`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 604; sourceText: Always-on VPN with lockdown for network mediation, separate from app foreground proof.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

257.  Represent a normal Android app cannot generally control all other apps?

- settingId: `app-guide-platform-capability-notes-android-007`
- policyLane: `rules`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 609; sourceText: A normal Android app cannot generally control all other apps.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

258.  Represent package visibility rules can hide installed apps from inventory queries?

- settingId: `app-guide-platform-capability-notes-android-008`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 610; sourceText: Package visibility rules can hide installed apps from inventory queries.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

259.  Represent usage access is permission-gated and can be revoked?

- settingId: `app-guide-platform-capability-notes-android-009`
- policyLane: `rules`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 611; sourceText: Usage access is permission-gated and can be revoked.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

260.  Represent accessibility is sensitive and must not be used as a stealth content capture path?

- settingId: `app-guide-platform-capability-notes-android-010`
- policyLane: `rules`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 612; sourceText: Accessibility is sensitive and must not be used as a stealth content capture path.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

261.  Represent device owner/profile owner changes the capability class and setup burden?

- settingId: `app-guide-platform-capability-notes-android-011`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 614; sourceText: Device owner/profile owner changes the capability class and setup burden.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

262.  Represent the roadmap currently treats Android package lifecycle proof as manual-required until real device artifacts exist?

- settingId: `app-guide-platform-capability-notes-android-012`
- policyLane: `evidence`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 615; sourceText: The roadmap currently treats Android package lifecycle proof as manual-required until real device artifacts exist.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### app-guide-platform-capability-notes-ios-and-ipados

263.  Configure screen time frameworks.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-001`
- policyLane: `schedule`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `single-choice-compact`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 624; sourceText: Screen Time frameworks: Family Controls, Managed Settings, Device Activity.
- acceptedOptions: Family Controls | Managed Settings | Device Activity
- helperText: app-control-capability-registry

264.  Represent opaque selections for applications, categories, and web domains?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-002`
- policyLane: `rules`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 625; sourceText: Opaque selections for applications, categories, and web domains.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

265.  Represent app/category shielding through Managed Settings?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-003`
- policyLane: `enforcement`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 626; sourceText: App/category shielding through Managed Settings.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

266.  Represent threshold/event monitoring through Device Activity?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-004`
- policyLane: `rules`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 627; sourceText: Threshold/event monitoring through Device Activity.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

267.  Represent mDM managed app install/removal and restrictions for enrolled devices?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-005`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 628; sourceText: MDM managed app install/removal and restrictions for enrolled devices.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

268.  Represent supervised-device restrictions where applicable?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-006`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 629; sourceText: Supervised-device restrictions where applicable.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

269.  Represent third-party apps do not get a general raw list of every installed app for parental control?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-007`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 633; sourceText: Third-party apps do not get a general raw list of every installed app for parental control.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

270.  Represent screen Time APIs are privacy-preserving and token-based; tokens can be voided if authorization is revoked?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-008`
- policyLane: `schedule`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 635; sourceText: Screen Time APIs are privacy-preserving and token-based; tokens can be voided if authorization is revoked.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

271.  Represent app shielding is not the same as process termination or raw app telemetry?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-009`
- policyLane: `enforcement`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 637; sourceText: App shielding is not the same as process termination or raw app telemetry.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

272.  Represent mDM and supervision determine app install/removal and restriction scope?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-010`
- policyLane: `setup`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 638; sourceText: MDM and supervision determine app install/removal and restriction scope.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

273.  Represent family Controls entitlements, App Store review, TestFlight, and runtime authorization are separate proof requirements?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-011`
- policyLane: `schedule`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 639; sourceText: Family Controls entitlements, App Store review, TestFlight, and runtime authorization are separate proof requirements.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

274.  Represent the roadmap separates iOS signing/entitlements from TestFlight and runtime API entitlements; do not claim iOS control until each is proven?

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-012`
- policyLane: `schedule`; sectionId: `app-guide-platform-capability-notes`; groupId: `app-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 641; sourceText: The roadmap separates iOS signing/entitlements from TestFlight and runtime API entitlements; do not claim iOS control until each is proven.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Section: app-guide-policy-modes-to-represent-later-in-ui

### app-guide-policy-modes-to-represent-later-in-ui

#### app-guide-policy-modes-to-represent-later-in-ui-observe-app-use

275.  Represent detect installed or launchable apps where available?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-001`
- policyLane: `setup`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 653; sourceText: Detect installed or launchable apps where available.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

276.  Represent show running and foreground state?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-002`
- policyLane: `evidence`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 654; sourceText: Show running and foreground state.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

277.  Represent build running and foreground session durations?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-003`
- policyLane: `schedule`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 655; sourceText: Build running and foreground session durations.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

278.  Represent show unknown, unsupported, stale, and permission-limited states?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-004`
- policyLane: `evidence`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 656; sourceText: Show unknown, unsupported, stale, and permission-limited states.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

279.  Represent produce reports without changing device behavior?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-005`
- policyLane: `reports`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 657; sourceText: Produce reports without changing device behavior.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

280.  Represent app blocking?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-006`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 661; sourceText: App blocking.
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

281.  Represent mobile device-owner/supervised setup?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-007`
- policyLane: `setup`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 662; sourceText: Mobile device-owner/supervised setup.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

282.  Represent broad application control?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-008`
- policyLane: `rules`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 663; sourceText: Broad application control.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

283.  Represent guaranteed launch blocking?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-009`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 667; sourceText: Guaranteed launch blocking.
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

284.  Represent install/uninstall control?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-010`
- policyLane: `setup`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 668; sourceText: Install/uninstall control.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

285.  Represent app-internal content knowledge?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-011`
- policyLane: `rules`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 669; sourceText: App-internal content knowledge.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

#### app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use

286.  Represent matching app activity remains allowed temporarily?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-001`
- policyLane: `rules`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 675; sourceText: Matching app activity remains allowed temporarily.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

287.  Represent the child sees a warning or ask-parent state?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-002`
- policyLane: `rules`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 676; sourceText: The child sees a warning or ask-parent state.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

288.  Represent the parent sees the app/session evidence and can approve, deny, or extend?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-003`
- policyLane: `approvals`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 677; sourceText: The parent sees the app/session evidence and can approve, deny, or extend.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

289.  Represent app/session evidence?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-004`
- policyLane: `evidence`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 681; sourceText: App/session evidence.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

290.  Represent child-facing local UI or notification/shield path?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-005`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 682; sourceText: Child-facing local UI or notification/shield path.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

291.  Represent parent approval contract?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-006`
- policyLane: `approvals`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 683; sourceText: Parent approval contract.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

292.  Represent expiry and audit?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-007`
- policyLane: `audit`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 684; sourceText: Expiry and audit.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

#### app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps

293.  Represent running or foreground time is counted against a parent budget?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-001`
- policyLane: `schedule`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 690; sourceText: Running or foreground time is counted against a parent budget.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

294.  Represent warning and grace rules apply?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-002`
- policyLane: `schedule`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 691; sourceText: Warning and grace rules apply.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

295.  Represent when the budget expires, the configured action runs if the adapter supports it?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-003`
- policyLane: `schedule`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 692; sourceText: When the budget expires, the configured action runs if the adapter supports it.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

296.  Represent sessionization from stored evidence?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-004`
- policyLane: `schedule`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 697; sourceText: Sessionization from stored evidence.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

297.  Represent timer recovery after restart?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-005`
- policyLane: `schedule`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 698; sourceText: Timer recovery after restart.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

298.  Represent policy decision and enforcement audit?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-006`
- policyLane: `schedule`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 699; sourceText: Policy decision and enforcement audit.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

299.  Represent fallback when enforcement is unavailable?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-007`
- policyLane: `schedule`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `parent-domain`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 700; sourceText: Fallback when enforcement is unavailable.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

#### app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps

300.  Represent parent rules prevent or interrupt app access?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-001`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 706; sourceText: Parent rules prevent or interrupt app access.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

301.  Represent on desktop this may mean launch block or process termination?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-002`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 707; sourceText: On desktop this may mean launch block or process termination.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

302.  Represent on mobile this may mean shield/hide/suspend through approved platform APIs?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-003`
- policyLane: `approvals`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 708; sourceText: On mobile this may mean shield/hide/suspend through approved platform APIs.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

303.  Represent platform-specific control proof?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-004`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 712; sourceText: Platform-specific control proof.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

304.  Represent parent exceptions?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-005`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 713; sourceText: Parent exceptions.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

305.  Represent rollback path?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-006`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 714; sourceText: Rollback path.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

306.  Represent audit for every strict action?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-007`
- policyLane: `audit`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 715; sourceText: Audit for every strict action.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

307.  Represent app deletion?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-008`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 719; sourceText: App deletion.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

308.  Represent exact in-app activity knowledge?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-009`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 720; sourceText: Exact in-app activity knowledge.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

309.  Represent parity across platforms?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-010`
- policyLane: `enforcement`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 721; sourceText: Parity across platforms.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

#### app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle

310.  Represent install, update, uninstall, hide, or remove managed apps through a platform management boundary?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-001`
- policyLane: `setup`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 727; sourceText: Install, update, uninstall, hide, or remove managed apps through a platform management boundary.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

311.  Represent managed-device or package-management setup?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-002`
- policyLane: `setup`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 732; sourceText: Managed-device or package-management setup.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

312.  Represent signing/store/MDM or installer proof?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-003`
- policyLane: `setup`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 733; sourceText: Signing/store/MDM or installer proof.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

313.  Represent custody model for personal vs managed apps?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-004`
- policyLane: `audit`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 734; sourceText: Custody model for personal vs managed apps.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

314.  Represent explicit parent-visible state and audit?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-005`
- policyLane: `reports`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 735; sourceText: Explicit parent-visible state and audit.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

315.  Represent personal device and family device expectations differ. The product should not imply corporate MDM behavior unless the device is actually managed that way?

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-006`
- policyLane: `setup`; sectionId: `app-guide-policy-modes-to-represent-later-in-ui`; groupId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 739; sourceText: Personal device and family device expectations differ. The product should not imply corporate MDM behavior unless the device is actually managed that way.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Section: app-guide-current-ocentra-parent-posture

### app-guide-current-ocentra-parent-posture

#### app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

316.  Represent app/game evidence contracts use stored local evidence and query/read models?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001`
- policyLane: `evidence`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 746; sourceText: App/game evidence contracts use stored local evidence and query/read models.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

317.  Represent process/window evidence can support native app sessions, running time, and foreground time?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-002`
- policyLane: `schedule`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 747; sourceText: Process/window evidence can support native app sessions, running time, and foreground time.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

318.  Represent local AI can consume evidence references or structured digests; it does not scan the OS?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-003`
- policyLane: `evidence`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 749; sourceText: Local AI can consume evidence references or structured digests; it does not scan the OS.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

319.  Represent policy decisions must reference evidence and remain deterministic?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-004`
- policyLane: `evidence`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 751; sourceText: Policy decisions must reference evidence and remain deterministic.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

320.  Represent v0.8 enforcement work has typed contracts, capability status, timer/recovery, and audit scaffolding?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-005`
- policyLane: `schedule`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 752; sourceText: V0.8 enforcement work has typed contracts, capability status, timer/recovery, and audit scaffolding.
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

321.  Represent windows has proof direction for owned-process terminate and app time-limit behavior, but broad app blocking remains manual-required until a real adapter proves it?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-006`
- policyLane: `schedule`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 754; sourceText: Windows has proof direction for owned-process terminate and app time-limit behavior, but broad app blocking remains manual-required until a real adapter proves it.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

322.  Represent android package lifecycle and iOS Screen Time/entitlement behavior are manual-required until real device/platform proof exists?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-007`
- policyLane: `schedule`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 757; sourceText: Android package lifecycle and iOS Screen Time/entitlement behavior are manual-required until real device/platform proof exists.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

323.  Represent parent portal is an authoring and visibility surface. It does not run app inventory, timers, policy evaluation, or enforcement?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-008`
- policyLane: `schedule`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 759; sourceText: Parent portal is an authoring and visibility surface. It does not run app inventory, timers, policy evaluation, or enforcement.
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

324.  Represent [`docs/architecture/app-game-evidence-sessions.md`](architecture/app-game-evidence-sessions.md)?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-009`
- policyLane: `evidence`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 764; sourceText: [`docs/architecture/app-game-evidence-sessions.md`](architecture/app-game-evidence-sessions.md)
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

325.  Represent [`docs/expectations/app-game-evidence.md`](expectations/app-game-evidence.md)?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-010`
- policyLane: `evidence`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 765; sourceText: [`docs/expectations/app-game-evidence.md`](expectations/app-game-evidence.md)
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

326.  Represent [`docs/expectations/policy.md`](expectations/policy.md)?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-011`
- policyLane: `rules`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 766; sourceText: [`docs/expectations/policy.md`](expectations/policy.md)
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

327.  Represent [`docs/expectations/enforcement.md`](expectations/enforcement.md)?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-012`
- policyLane: `enforcement`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 767; sourceText: [`docs/expectations/enforcement.md`](expectations/enforcement.md)
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

328.  Represent [`docs/product-roadmap.md`](product-roadmap.md)?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-013`
- policyLane: `rules`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 768; sourceText: [`docs/product-roadmap.md`](product-roadmap.md)
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

329.  Represent [`docs/managed-unmanaged-browser.md`](../../../plans/browser-plan/workpacks/managed-unmanaged-browser.md)?

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-014`
- policyLane: `rules`; sectionId: `app-guide-current-ocentra-parent-posture`; groupId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 769; sourceText: [`docs/managed-unmanaged-browser.md`](../../../plans/browser-plan/workpacks/managed-unmanaged-browser.md)
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Section: app-guide-future-ui-rules

### app-guide-future-ui-rules

#### app-guide-future-ui-rules-future-ui-rules

330.  Represent show inventory, running, foreground, time-limit, install, uninstall, shield, suspend, block, and terminate as separate capability rows?

- settingId: `app-guide-future-ui-rules-future-ui-rules-001`
- policyLane: `schedule`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 775; sourceText: Show inventory, running, foreground, time-limit, install, uninstall, shield, suspend, block, and terminate as separate capability rows.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

331.  Represent show the evidence source next to every app claim?

- settingId: `app-guide-future-ui-rules-future-ui-rules-002`
- policyLane: `evidence`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 777; sourceText: Show the evidence source next to every app claim.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

332.  Represent show exact package/process identity only when proof exists and retention allows it?

- settingId: `app-guide-future-ui-rules-future-ui-rules-003`
- policyLane: `audit`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 778; sourceText: Show exact package/process identity only when proof exists and retention allows it.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

333.  Represent show unknown apps as unknown, not as risky by default?

- settingId: `app-guide-future-ui-rules-future-ui-rules-004`
- policyLane: `evidence`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 780; sourceText: Show unknown apps as unknown, not as risky by default.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

334.  Represent show app category as a label with source/confidence, not as an automatic decision?

- settingId: `app-guide-future-ui-rules-future-ui-rules-005`
- policyLane: `evidence`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 781; sourceText: Show app category as a label with source/confidence, not as an automatic decision.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

335.  Represent show app time limits only when duration evidence exists or when the rule is clearly marked as pending proof?

- settingId: `app-guide-future-ui-rules-future-ui-rules-006`
- policyLane: `schedule`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 783; sourceText: Show app time limits only when duration evidence exists or when the rule is clearly marked as pending proof.
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

336.  Represent show strict actions as ready, unsupported, permission-required, manual-required, degraded, dry-run-only, adapter-error, or blocked-by-setup?

- settingId: `app-guide-future-ui-rules-future-ui-rules-007`
- policyLane: `setup`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 785; sourceText: Show strict actions as ready, unsupported, permission-required, manual-required, degraded, dry-run-only, adapter-error, or blocked-by-setup.
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

337.  Represent keep managed-device setup state close to mobile actions?

- settingId: `app-guide-future-ui-rules-future-ui-rules-008`
- policyLane: `setup`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 787; sourceText: Keep managed-device setup state close to mobile actions.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

338.  Represent keep child-facing messages separate from parent diagnostics?

- settingId: `app-guide-future-ui-rules-future-ui-rules-009`
- policyLane: `rules`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 788; sourceText: Keep child-facing messages separate from parent diagnostics.
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

339.  Configure every strict action should have an audit path.

- settingId: `app-guide-future-ui-rules-future-ui-rules-010`
- policyLane: `schedule`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 789; sourceText: Every strict action should have an audit path: evidence, parent rule, compiled policy, adapter mechanism, outcome, timestamp, and rollback/unavailable state.
- acceptedOptions: Evidence | Parent Rule | Compiled Policy | Adapter Mechanism | Outcome | Timestamp | And Rollback/unavailable State
- helperText: app claims require fresh evidence references with confidence and custody.

340.  Represent observe only;?

- settingId: `app-guide-future-ui-rules-future-ui-rules-011`
- policyLane: `rules`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 794; sourceText: observe only;
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

341.  Represent warn on app use;?

- settingId: `app-guide-future-ui-rules-future-ui-rules-012`
- policyLane: `rules`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 795; sourceText: warn on app use;
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

342.  Represent ask parent on app use;?

- settingId: `app-guide-future-ui-rules-future-ui-rules-013`
- policyLane: `approvals`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 796; sourceText: ask parent on app use;
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

343.  Represent set app/category/unknown-app time budgets;?

- settingId: `app-guide-future-ui-rules-future-ui-rules-014`
- policyLane: `schedule`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 797; sourceText: set app/category/unknown-app time budgets;
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

344.  Represent close or terminate selected apps after a timer;?

- settingId: `app-guide-future-ui-rules-future-ui-rules-015`
- policyLane: `schedule`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `number-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 798; sourceText: close or terminate selected apps after a timer;
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

345.  Represent shield, suspend, or block apps where the platform supports it;?

- settingId: `app-guide-future-ui-rules-future-ui-rules-016`
- policyLane: `enforcement`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `none`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 799; sourceText: shield, suspend, or block apps where the platform supports it;
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

346.  Represent manage app installs/uninstalls only inside a platform-approved custody model?

- settingId: `app-guide-future-ui-rules-future-ui-rules-017`
- policyLane: `approvals`; sectionId: `app-guide-future-ui-rules`; groupId: `app-guide-future-ui-rules-future-ui-rules`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: docs/app-control-capability-guide.md
- sourceLine: 800; sourceText: manage app installs/uninstalls only inside a platform-approved custody model.
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.
