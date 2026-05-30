# App Control Settings Inventory

Generated from `BaselineAppControlFullCatalog`.
Total settings: 346

Use this as the raw review list for deciding parent-facing grouping.

## Tab: rules

### App management

#### Default posture

1.  Enable app management?

- settingId: `app.enabled`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Enabled | Disabled
- helperText: Native app controls stay disabled until a parent enables this policy document.

2.  What should happen to app activity?

- settingId: `app.defaultPosture`
- policyLane: `rules`; cardKind: `single-choice-many`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Blocking is product-true only when the target platform adapter proves it.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Allow | Observe | Warn | Ask | Limit | Block
- helperText: Blocking is product-true only when the target platform adapter proves it.

3.  How should app management run on this device?

- settingId: `app.managementMode`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Portal authoring alone must not claim runtime enforcement.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Local Child Agent | Lan Live | Authoring Only | Unavailable
- helperText: Portal authoring alone must not claim runtime enforcement.

## Tab: evidence

### Installed apps

#### Inventory sources

4.  How should installed app inventory be used?

- settingId: `inventory.mode`
- policyLane: `evidence`; cardKind: `single-choice-compact`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `permission-limited`
- proofRequirement: Inventory is not proof of current use.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Disabled | Reports Only | Use For Matching And Reports | Required For Strict Rules
- helperText: Inventory is not proof of current use.

5.  Which app inventory sources are allowed?

- settingId: `inventory.sources`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: Portable, privacy-hidden, and mobile-tokenized apps may be absent.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Os Installed Apps | Desktop Shortcuts | Store Packages | Package Manager | Managed Device Apps | Screen Time Tokens | Executable Metadata | Parent Catalog
- helperText: Portable, privacy-hidden, and mobile-tokenized apps may be absent.

#### Identity strategy

6.  Which identity fields may app rules use?

- settingId: `inventory.identityFields`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: No single identity field proves an app on every platform.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Package Id | Bundle Id | App User Model Id | Desktop Entry Id | Application Token | Executable Path | Publisher Signature | File Hash | Display Name | Parent Label
- helperText: No single identity field proves an app on every platform.

## Tab: rules

### Installed apps

#### Identity strategy

7.  What should happen when an app cannot be identified?

- settingId: `inventory.unknownHandling`
- policyLane: `rules`; cardKind: `single-choice-many`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unknown apps must remain labeled unknown.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Allow | Observe | Warn | Ask | Count Under Unknown Budget | Block If Supported
- helperText: Unknown apps must remain labeled unknown.

## Tab: evidence

### Runtime evidence

#### Runtime sources

8.  Which runtime evidence sources may be used?

- settingId: `evidence.runtimeSources`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Foreground evidence and mobile usage visibility are platform-permission dependent.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Process Snapshot | Process Start Exit | Foreground Window | Usage Stats | Device Activity | Managed Device State | Accessibility Approved State | App Session Summary
- helperText: Foreground evidence and mobile usage visibility are platform-permission dependent.

#### Runtime proof

9.  What proof is enough for app rules?

- settingId: `evidence.requiredProof`
- policyLane: `evidence`; cardKind: `single-choice-many`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Inventory-only proof cannot justify strict runtime action.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Inventory Only | Process Running | Foreground Window | Fresh App Session | Platform Usage Event | Managed Device State
- helperText: Inventory-only proof cannot justify strict runtime action.

10. What if app proof is unavailable?

- settingId: `evidence.whenProofUnavailable`
- policyLane: `evidence`; cardKind: `single-choice-many`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unavailable proof is a parent-visible state, not a hidden allow or block.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Allow | Observe | Warn | Ask | Block Until Ready | Mark Unavailable
- helperText: Unavailable proof is a parent-visible state, not a hidden allow or block.

## Tab: schedule

### Runtime evidence

#### Duration proof

11. Which duration should time budgets count?

- settingId: `evidence.durationMode`
- policyLane: `schedule`; cardKind: `single-choice-compact`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Portal refresh cadence must not count as child activity.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Running Time | Foreground Time | Platform Usage Time | Manual Review Only
- helperText: Portal refresh cadence must not count as child activity.

## Tab: audit

### Runtime evidence

#### Data minimization

12. What must app controls never collect?

- settingId: `evidence.neverCollect`
- policyLane: `audit`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: App evidence does not prove content, keystrokes, screenshots, or chat text.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Screen Contents | Screenshots | Keystrokes | Chat Content | Voice Content | App Internal Documents | Launcher Credentials | Decrypted Network Payload | Raw Command Line With Secrets
- helperText: App evidence does not prove content, keystrokes, screenshots, or chat text.

## Tab: rules

### App rules

#### Rule targets

13. What app targets should rules match?

- settingId: `rules.allowedTargetTypes`
- policyLane: `rules`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Unknown, helper, wrapper, and renamed apps require honest confidence state.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: App Ref | App Category | Package Id | Bundle Id | Application Token | Executable Identity | Publisher Signature | Unknown App | Managed App State | App Session | Capability State
- helperText: Unknown, helper, wrapper, and renamed apps require honest confidence state.

14. How strong must an app match be before strict action?

- settingId: `rules.matchConfidenceRequired`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Strict action requires deterministic or approved app identity proof.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Any Candidate | Catalog Confidence High | Deterministic Or Parent Approved | Managed Device Proof
- helperText: Strict action requires deterministic or approved app identity proof.

#### Unknown apps

15. Default rule for unknown apps?

- settingId: `rules.defaultUnknownRule`
- policyLane: `rules`; cardKind: `single-choice-many`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Unknown remains unknown until adapter evidence maps it confidently.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Allow | Observe | Warn | Ask First Run | Limit | Block If Supported
- helperText: Unknown remains unknown until adapter evidence maps it confidently.

## Tab: schedule

### App time limits

#### Time budgets

16. Enable app time budgets?

- settingId: `budgets.enabled`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Budgets need child-agent timer state, not portal-rendered time.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Enabled | Disabled
- helperText: Budgets need child-agent timer state, not portal-rendered time.

17. Default daily app time limit in minutes?

- settingId: `budgets.defaultDailyMinutes`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Timer recovery and audit are child-agent responsibilities.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- helperText: Timer recovery and audit are child-agent responsibilities.

18. What happens when app time runs out?

- settingId: `budgets.whenExhausted`
- policyLane: `schedule`; cardKind: `single-choice-many`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Terminate, shield, or block after budget requires platform proof.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Observe | Warn | Ask | Terminate If Supported | Shield If Supported | Block If Supported
- helperText: Terminate, shield, or block after budget requires platform proof.

## Tab: enforcement

### App enforcement

#### Strict actions

19. Which app enforcement actions may run?

- settingId: `enforcement.allowedActions`
- policyLane: `enforcement`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Broad app blocking remains manual-required until a real adapter proves it.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Warn | Ask Parent | Owned Process Terminate | Target Process Terminate | Block Launch | Shield App | Suspend Package | Hide Package | Time Limit | Managed Install | Managed Uninstall
- helperText: Broad app blocking remains manual-required until a real adapter proves it.

20. What if a strict app action is unsupported?

- settingId: `enforcement.strictActionFallback`
- policyLane: `enforcement`; cardKind: `single-choice-many`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unsupported strict actions must surface as unavailable or parent-visible fallback.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Allow And Report Unavailable | Observe And Report Unavailable | Warn And Report Unavailable | Parent Request Report Unavailable | Block Until Ready
- helperText: Unsupported strict actions must surface as unavailable or parent-visible fallback.

21. How long should the child get before strict action applies?

- settingId: `enforcement.graceSeconds`
- policyLane: `enforcement`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Grace timers need local runtime and audit state.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- helperText: Grace timers need local runtime and audit state.

22. Require rollback state for strict actions?

- settingId: `enforcement.requireRollbackPlan`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `rust-service`; capabilityState: `available`
- proofRequirement: Strict actions need rollback or explicit unavailable outcome.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Enabled | Disabled
- helperText: Strict actions need rollback or explicit unavailable outcome.

## Tab: setup

### Managed app lifecycle

#### Managed lifecycle

23. How should app install and uninstall controls be handled?

- settingId: `lifecycle.mode`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Personal app install or removal often is not available.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Disabled | Report Managed State | Managed Apps Only | Device Owner Or Mdm Only
- helperText: Personal app install or removal often is not available.

24. Which managed app lifecycle operations are allowed?

- settingId: `lifecycle.allowedOperations`
- policyLane: `setup`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Install, uninstall, hide, and suspend depend on platform custody and policy.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Install Managed App | Uninstall Managed App | Hide Managed App | Suspend Managed App | Remove User Installed App If Platform Approved | Prevent Uninstall If Platform Approved
- helperText: Install, uninstall, hide, and suspend depend on platform custody and policy.

## Tab: approvals

### Approvals

#### Approval events

25. Which app events require parent approval?

- settingId: `approvals.requiredFor`
- policyLane: `approvals`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Approval state is policy data; child-agent still owns local action results.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Unknown App | New App | Blocked App | Time Extension | Managed Install | Managed Uninstall | Strict Action Unavailable | Category Override
- helperText: Approval state is policy data; child-agent still owns local action results.

26. What happens if the parent does not answer?

- settingId: `approvals.unansweredDefault`
- policyLane: `approvals`; cardKind: `single-choice-compact`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unanswered approvals must be deterministic and auditable.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Allow | Deny | Keep Pending | Use Rule Fallback
- helperText: Unanswered approvals must be deterministic and auditable.

## Tab: reports

### Reports

#### Report fields

27. Which app report fields should be visible?

- settingId: `reports.visibleFields`
- policyLane: `reports`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Reports must distinguish raw evidence from redacted rollups.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- acceptedOptions: Installed Apps | Running Now | Foreground Now | Session Rollups | Unknown Apps | Category Rollups | Time Budget | Policy Decisions | Enforcement Results | Approval Events | Managed Lifecycle Events | Source Capability
- helperText: Reports must distinguish raw evidence from redacted rollups.

## Tab: audit

### Reports

#### Retention

28. How long should raw app observations be kept?

- settingId: `retention.rawObservation`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Raw app observations should be short-lived and redacted where possible.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- helperText: Raw app observations should be short-lived and redacted where possible.

29. How long should app rollups be kept?

- settingId: `retention.rollups`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Rollups must preserve custody and evidence-reference boundaries.
- sourceDocument: `docs/app-control-schema-proposal.md`; sourceLine: 0
- helperText: Rollups must preserve custody and evidence-reference boundaries.

## Tab: evidence

### Core Terms

#### Native App

30. Windows Win32 desktop app.

- settingId: `app-guide-core-terms-native-app-001`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 28
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

31. Windows packaged app or Microsoft Store app.

- settingId: `app-guide-core-terms-native-app-002`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 29
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: rules

### Core Terms

#### Native App

32. macOS app bundle.

- settingId: `app-guide-core-terms-native-app-003`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 30
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

## Tab: evidence

### Core Terms

#### Native App

33. Linux desktop app, package app, Flatpak, Snap, AppImage, or command-backed desktop entry.

- settingId: `app-guide-core-terms-native-app-004`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 31
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

34. Android package.

- settingId: `app-guide-core-terms-native-app-005`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 33
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

## Tab: approvals

### Core Terms

#### Native App

35. iOS or iPadOS application selected through Apple-approved controls.

- settingId: `app-guide-core-terms-native-app-006`
- policyLane: `approvals`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 34
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: setup

### Core Terms

#### Managed App

36. App installed by a supervised or managed device flow.

- settingId: `app-guide-core-terms-managed-app-001`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 47
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: approvals

### Core Terms

#### Managed App

37. App allowlisted or denylisted by an OS application-control policy.

- settingId: `app-guide-core-terms-managed-app-002`
- policyLane: `approvals`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 48
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: rules

### Core Terms

#### Managed App

38. App represented by an opaque mobile platform token selected by a guardian.

- settingId: `app-guide-core-terms-managed-app-003`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 49
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### Core Terms

#### Managed App

39. App launched through an Ocentra-controlled shortcut, launcher, or policy adapter.

- settingId: `app-guide-core-terms-managed-app-004`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 50
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: evidence

### Core Terms

#### Managed App

40. App process started by Ocentra and tracked with an owned process/session id.

- settingId: `app-guide-core-terms-managed-app-005`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 52
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: setup

### Core Terms

#### Unmanaged App

41. A normal user-installed desktop app without Ocentra policy.

- settingId: `app-guide-core-terms-unmanaged-app-001`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 64
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: rules

### Core Terms

#### Unmanaged App

42. A portable executable.

- settingId: `app-guide-core-terms-unmanaged-app-002`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 65
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

43. A copied or renamed executable.

- settingId: `app-guide-core-terms-unmanaged-app-003`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 66
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### Core Terms

#### Unmanaged App

44. A helper process launched by a known app but not mapped to a supported app identity.

- settingId: `app-guide-core-terms-unmanaged-app-004`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 67
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: approvals

### Core Terms

#### Unmanaged App

45. A mobile app that the platform does not expose through the approved parental, enterprise, or device-owner APIs.

- settingId: `app-guide-core-terms-unmanaged-app-005`
- policyLane: `approvals`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 69
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Core Terms

#### Unmanaged App

46. An app running on an unsupported platform adapter.

- settingId: `app-guide-core-terms-unmanaged-app-006`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 71
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

#### App Identity

47. Package id, bundle id, AppUserModelID, package family name, desktop entry id, or application token.

- settingId: `app-guide-core-terms-app-identity-001`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 91
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

48. Executable path, file hash, publisher/signature, product name, or version.

- settingId: `app-guide-core-terms-app-identity-002`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 93
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

49. Process id and parent process id for a running observation.

- settingId: `app-guide-core-terms-app-identity-003`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 94
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

50. Window id and foreground state for active-use evidence.

- settingId: `app-guide-core-terms-app-identity-004`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 95
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: setup

### Core Terms

#### App Identity

51. Installer/source reference and install state for inventory evidence.

- settingId: `app-guide-core-terms-app-identity-005`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 96
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Core Terms

#### App Session Evidence

52. App was observed.

- settingId: `app-guide-core-terms-app-session-evidence-001`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 107
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

53. App was running.

- settingId: `app-guide-core-terms-app-session-evidence-002`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 108
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

54. App was foreground-active where foreground proof exists.

- settingId: `app-guide-core-terms-app-session-evidence-003`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 109
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### Core Terms

#### App Session Evidence

55. App had a running or foreground duration inside a time window.

- settingId: `app-guide-core-terms-app-session-evidence-004`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 110
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### Core Terms

#### App Session Evidence

56. App identity, category, or unknown state was derived from specific evidence ids.

- settingId: `app-guide-core-terms-app-session-evidence-005`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 111
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: enforcement

### Core Terms

#### App Control Action

57. Launch.

- settingId: `app-guide-core-terms-app-control-action-001`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 123
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: rules

### Core Terms

#### App Control Action

58. Warn.

- settingId: `app-guide-core-terms-app-control-action-002`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 124
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: approvals

### Core Terms

#### App Control Action

59. Ask parent.

- settingId: `app-guide-core-terms-app-control-action-003`
- policyLane: `approvals`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 125
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### Core Terms

#### App Control Action

60. Terminate owned or target process.

- settingId: `app-guide-core-terms-app-control-action-004`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 126
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

61. Suspend, hide, shield, or block app where the platform supports it.

- settingId: `app-guide-core-terms-app-control-action-005`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 127
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: schedule

### Core Terms

#### App Control Action

62. Start, extend, expire, or roll back a time limit.

- settingId: `app-guide-core-terms-app-control-action-006`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 128
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: audit

### Core Terms

#### App Control Action

63. Install or uninstall a managed app where the platform and custody model allow it.

- settingId: `app-guide-core-terms-app-control-action-007`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 129
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: setup

### The Main Capability Truth

#### The Main Capability Truth

64. Inventory layer: app appears installed, launchable, removable, managed, or unknown.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-001`
- policyLane: `setup`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 148
- acceptedOptions: App Appears Installed | Launchable | Removable | Managed | Unknown
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: schedule

### The Main Capability Truth

#### The Main Capability Truth

65. Runtime layer: process/package/app session is running, foreground, background, stale, or unavailable.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-002`
- policyLane: `schedule`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 150
- acceptedOptions: Process/package/app Session Is Running | Foreground | Background | Stale | Unavailable
- helperText: app claims require fresh evidence references with confidence and custody.

66. Duration layer: running and foreground time are derived from stored evidence, not portal refresh.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-003`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 152
- acceptedOptions: Running And Foreground Time Are Derived From Stored Evidence | Not Portal Refresh
- helperText: app claims require fresh evidence references with confidence and custody.

67. Policy layer: parent rule matches app identity, category, unknown state, schedule, budget, or approval state.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-004`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 154
- acceptedOptions: Parent Rule Matches App Identity | Category | Unknown State | Schedule | Budget | Approval State
- helperText: app claims require fresh evidence references with confidence and custody.

68. Enforcement layer: child-device adapter executes terminate, block, shield, suspend, hide, launch, install, uninstall, or time-limit actions.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-005`
- policyLane: `schedule`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 156
- acceptedOptions: Child Device Adapter Executes Terminate | Block | Shield | Suspend | Hide | Launch | Install | Uninstall | Time Limit Actions
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: reports

### The Main Capability Truth

#### The Main Capability Truth

69. Audit layer: every parent-visible claim carries evidence source, custody, policy decision, adapter result, and capability status.

- settingId: `app-guide-the-main-capability-truth-the-main-capability-truth-006`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 158
- acceptedOptions: Every Parent Visible Claim Carries Evidence Source | Custody | Policy Decision | Adapter Result | And Capability Status
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### Capability Matrix

#### Capability Matrix

70. Capability matrix row | Capability=Installed app inventory | Windows=Yes, partial by source | macOS=Yes, partial by source | Linux=Yes, partial by distro/desktop | Android=Yes, visibility-limited | iOS/iPadOS=Limited, token/MDM-managed paths | Required proof=Inventory adapter and source ids | Important limit=Inventory is not proof of current use.

- settingId: `app-guide-capability-matrix-capability-matrix-001`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 169
- acceptedOptions: Capability: Installed app inventory | Windows: Yes, partial by source | macOS: Yes, partial by source | Linux: Yes, partial by distro/desktop | Android: Yes, visibility-limited | iOS/iPadOS: Limited, token/MDM-managed paths | Required proof: Inventory adapter and source ids | Important limit: Inventory is not proof of current use.
- helperText: strict app control requires real platform adapter or managed-device proof.

71. Capability matrix row | Capability=Package/process identity | Windows=Strong for observed processes/packages | macOS=Strong for bundles/processes | Linux=Varies by package and desktop entry | Android=Strong package id when visible | iOS/iPadOS=Opaque tokens or managed app metadata | Required proof=Identity fields plus confidence | Important limit=Renames, helpers, and wrappers reduce confidence.

- settingId: `app-guide-capability-matrix-capability-matrix-002`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 170
- acceptedOptions: Capability: Package/process identity | Windows: Strong for observed processes/packages | macOS: Strong for bundles/processes | Linux: Varies by package and desktop entry | Android: Strong package id when visible | iOS/iPadOS: Opaque tokens or managed app metadata | Required proof: Identity fields plus confidence | Important limit: Renames, helpers, and wrappers reduce confidence.
- helperText: app claims require fresh evidence references with confidence and custody.

72. Capability matrix row | Capability=Running app observation | Windows=Yes | macOS=Yes | Linux=Yes | Android=Limited; usage/accessibility/DO paths | iOS/iPadOS=Limited through Screen Time/MDM signals | Required proof=Runtime observation evidence | Important limit=Background services may not equal user-facing app use.

- settingId: `app-guide-capability-matrix-capability-matrix-003`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 171
- acceptedOptions: Capability: Running app observation | Windows: Yes | macOS: Yes | Linux: Yes | Android: Limited; usage/accessibility/DO paths | iOS/iPadOS: Limited through Screen Time/MDM signals | Required proof: Runtime observation evidence | Important limit: Background services may not equal user-facing app use.
- helperText: strict app control requires real platform adapter or managed-device proof.

73. Capability matrix row | Capability=Foreground app evidence | Windows=Yes | macOS=Permission-dependent | Linux=Desktop-environment-dependent | Android=Usage stats/accessibility-dependent | iOS/iPadOS=Device Activity thresholds, not raw focus | Required proof=Fresh foreground or activity evidence | Important limit=Foreground does not prove in-app content.

- settingId: `app-guide-capability-matrix-capability-matrix-004`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 172
- acceptedOptions: Capability: Foreground app evidence | Windows: Yes | macOS: Permission-dependent | Linux: Desktop-environment-dependent | Android: Usage stats/accessibility-dependent | iOS/iPadOS: Device Activity thresholds, not raw focus | Required proof: Fresh foreground or activity evidence | Important limit: Foreground does not prove in-app content.
- helperText: app claims require fresh evidence references with confidence and custody.

74. Capability matrix row | Capability=Running duration | Windows=Yes | macOS=Yes | Linux=Yes | Android=Usage-stat/session-dependent | iOS/iPadOS=Device Activity threshold-based | Required proof=Ordered observations/session model | Important limit=Gaps and restarts need stale handling.

- settingId: `app-guide-capability-matrix-capability-matrix-005`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 173
- acceptedOptions: Capability: Running duration | Windows: Yes | macOS: Yes | Linux: Yes | Android: Usage-stat/session-dependent | iOS/iPadOS: Device Activity threshold-based | Required proof: Ordered observations/session model | Important limit: Gaps and restarts need stale handling.
- helperText: app claims require fresh evidence references with confidence and custody.

75. Capability matrix row | Capability=Foreground duration | Windows=Yes | macOS=Permission-dependent | Linux=Desktop-environment-dependent | Android=Usage-stat/accessibility-dependent | iOS/iPadOS=Threshold/event-based | Required proof=Foreground observations or platform events | Important limit=Portal polling must not count time.

- settingId: `app-guide-capability-matrix-capability-matrix-006`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 174
- acceptedOptions: Capability: Foreground duration | Windows: Yes | macOS: Permission-dependent | Linux: Desktop-environment-dependent | Android: Usage-stat/accessibility-dependent | iOS/iPadOS: Threshold/event-based | Required proof: Foreground observations or platform events | Important limit: Portal polling must not count time.
- helperText: app claims require fresh evidence references with confidence and custody.

76. Capability matrix row | Capability=App categories | Windows=Derived from catalog/package/source | macOS=Derived from catalog/package/source | Linux=Derived from desktop/package metadata | Android=Package/category where exposed | iOS/iPadOS=Opaque category tokens through Screen Time | Required proof=Category source and confidence | Important limit=Category labels are policy inputs, not hidden blocks.

- settingId: `app-guide-capability-matrix-capability-matrix-007`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 175
- acceptedOptions: Capability: App categories | Windows: Derived from catalog/package/source | macOS: Derived from catalog/package/source | Linux: Derived from desktop/package metadata | Android: Package/category where exposed | iOS/iPadOS: Opaque category tokens through Screen Time | Required proof: Category source and confidence | Important limit: Category labels are policy inputs, not hidden blocks.
- helperText: app claims require fresh evidence references with confidence and custody.

77. Capability matrix row | Capability=Launch app | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes, with package intents where allowed | iOS/iPadOS=Limited; open intents/managed flows | Required proof=Launch adapter result | Important limit=Launch does not imply ongoing control.

- settingId: `app-guide-capability-matrix-capability-matrix-008`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 176
- acceptedOptions: Capability: Launch app | Windows: Yes | macOS: Yes | Linux: Yes | Android: Yes, with package intents where allowed | iOS/iPadOS: Limited; open intents/managed flows | Required proof: Launch adapter result | Important limit: Launch does not imply ongoing control.
- helperText: app claims require fresh evidence references with confidence and custody.

78. Capability matrix row | Capability=Terminate app | Windows=Yes, where permission permits | macOS=Yes, where permission permits | Linux=Yes, where permission permits | Android=Limited; device-owner/admin paths | iOS/iPadOS=No general third-party terminate | Required proof=Target identity and adapter result | Important limit=Unsaved data and race conditions need UX/audit.

- settingId: `app-guide-capability-matrix-capability-matrix-009`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 177
- acceptedOptions: Capability: Terminate app | Windows: Yes, where permission permits | macOS: Yes, where permission permits | Linux: Yes, where permission permits | Android: Limited; device-owner/admin paths | iOS/iPadOS: No general third-party terminate | Required proof: Target identity and adapter result | Important limit: Unsaved data and race conditions need UX/audit.
- helperText: strict app control requires real platform adapter or managed-device proof.

79. Capability matrix row | Capability=Suspend/hide/shield app | Windows=App control policy dependent | macOS=MDM/profile dependent | Linux=Desktop/policy dependent | Android=Device owner/profile owner capable | iOS/iPadOS=Screen Time/Managed Settings capable | Required proof=Platform management proof | Important limit=Mobile support depends on entitlements/setup.

- settingId: `app-guide-capability-matrix-capability-matrix-010`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 178
- acceptedOptions: Capability: Suspend/hide/shield app | Windows: App control policy dependent | macOS: MDM/profile dependent | Linux: Desktop/policy dependent | Android: Device owner/profile owner capable | iOS/iPadOS: Screen Time/Managed Settings capable | Required proof: Platform management proof | Important limit: Mobile support depends on entitlements/setup.
- helperText: strict app control requires real platform adapter or managed-device proof.

80. Capability matrix row | Capability=Block launch | Windows=AppLocker/WDAC or similar proof required | macOS=MDM/system policy proof required | Linux=Policy/permission proof required | Android=Device owner/profile owner capable | iOS/iPadOS=Screen Time shield or MDM restriction | Required proof=Pre-launch enforcement proof | Important limit=Current repo must not claim broad blocking without proof.

- settingId: `app-guide-capability-matrix-capability-matrix-011`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 179
- acceptedOptions: Capability: Block launch | Windows: AppLocker/WDAC or similar proof required | macOS: MDM/system policy proof required | Linux: Policy/permission proof required | Android: Device owner/profile owner capable | iOS/iPadOS: Screen Time shield or MDM restriction | Required proof: Pre-launch enforcement proof | Important limit: Current repo must not claim broad blocking without proof.
- helperText: strict app control requires real platform adapter or managed-device proof.

81. Capability matrix row | Capability=Time-limit app use | Windows=Yes for app sessions and owned terminate | macOS=Possible with platform proof | Linux=Possible with platform proof | Android=Usage/DevicePolicy/Accessibility proof | iOS/iPadOS=Device Activity threshold/shield path | Required proof=Timer plus action/result audit | Important limit=Needs fallback when action cannot enforce.

- settingId: `app-guide-capability-matrix-capability-matrix-012`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 180
- acceptedOptions: Capability: Time-limit app use | Windows: Yes for app sessions and owned terminate | macOS: Possible with platform proof | Linux: Possible with platform proof | Android: Usage/DevicePolicy/Accessibility proof | iOS/iPadOS: Device Activity threshold/shield path | Required proof: Timer plus action/result audit | Important limit: Needs fallback when action cannot enforce.
- helperText: strict app control requires real platform adapter or managed-device proof.

82. Capability matrix row | Capability=Install app | Windows=Installer/package manager path | macOS=Installer/MDM/package path | Linux=Package manager path | Android=Package installer/device owner/MDM | iOS/iPadOS=MDM/App Store managed distribution | Required proof=Install adapter/custody proof | Important limit=User consent, store policy, and signing matter.

- settingId: `app-guide-capability-matrix-capability-matrix-013`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 181
- acceptedOptions: Capability: Install app | Windows: Installer/package manager path | macOS: Installer/MDM/package path | Linux: Package manager path | Android: Package installer/device owner/MDM | iOS/iPadOS: MDM/App Store managed distribution | Required proof: Install adapter/custody proof | Important limit: User consent, store policy, and signing matter.
- helperText: strict app control requires real platform adapter or managed-device proof.

83. Capability matrix row | Capability=Uninstall app | Windows=Installer/package manager path | macOS=Installer/MDM/package path | Linux=Package manager path | Android=Device owner/MDM/package path | iOS/iPadOS=MDM-managed app removal only | Required proof=Removal adapter/custody proof | Important limit=Personal app removal is often not available.

- settingId: `app-guide-capability-matrix-capability-matrix-014`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 182
- acceptedOptions: Capability: Uninstall app | Windows: Installer/package manager path | macOS: Installer/MDM/package path | Linux: Package manager path | Android: Device owner/MDM/package path | iOS/iPadOS: MDM-managed app removal only | Required proof: Removal adapter/custody proof | Important limit: Personal app removal is often not available.
- helperText: strict app control requires real platform adapter or managed-device proof.

84. Capability matrix row | Capability=Child-facing message | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes | iOS/iPadOS=Shield UI where supported | Required proof=Local UI/notification/shield result | Important limit=Do not show parent diagnostics to child.

- settingId: `app-guide-capability-matrix-capability-matrix-015`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 183
- acceptedOptions: Capability: Child-facing message | Windows: Yes | macOS: Yes | Linux: Yes | Android: Yes | iOS/iPadOS: Shield UI where supported | Required proof: Local UI/notification/shield result | Important limit: Do not show parent diagnostics to child.
- helperText: strict app control requires real platform adapter or managed-device proof.

85. Capability matrix row | Capability=Parent report | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes, if evidence exists | iOS/iPadOS=Yes, token/capability-limited | Required proof=Stored evidence and custody labels | Important limit=Reports must distinguish raw vs redacted fields.

- settingId: `app-guide-capability-matrix-capability-matrix-016`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 184
- acceptedOptions: Capability: Parent report | Windows: Yes | macOS: Yes | Linux: Yes | Android: Yes, if evidence exists | iOS/iPadOS: Yes, token/capability-limited | Required proof: Stored evidence and custody labels | Important limit: Reports must distinguish raw vs redacted fields.
- helperText: app claims require fresh evidence references with confidence and custody.

86. Capability matrix row | Capability=Audit/retention | Windows=Yes | macOS=Yes | Linux=Yes | Android=Yes | iOS/iPadOS=Yes | Required proof=Journal/query retention policy | Important limit=Local-first custody remains default.

- settingId: `app-guide-capability-matrix-capability-matrix-017`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 185
- acceptedOptions: Capability: Audit/retention | Windows: Yes | macOS: Yes | Linux: Yes | Android: Yes | iOS/iPadOS: Yes | Required proof: Journal/query retention policy | Important limit: Local-first custody remains default.
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: setup

### App Evidence: What Is Possible

#### Installed App Inventory

87. App display name where safe.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-001`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 196
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

88. Package id, bundle id, package family name, desktop entry id, AppUserModelID, or app token.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-002`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 197
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

89. Install source: installer, store package, app bundle, desktop entry, package manager, managed app distribution, or unknown.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-003`
- policyLane: `setup`; cardKind: `multi-choice-many`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 199
- acceptedOptions: Installer | Store Package | App Bundle | Desktop Entry | Package Manager | Managed App Distribution | Unknown
- helperText: strict app control requires real platform adapter or managed-device proof.

90. Version, publisher, signature, hash, install path, and executable path where available and policy permits.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-004`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 201
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

91. Category metadata from platform, catalog, desktop entry, app store metadata, or parent-maintained catalog.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-005`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 203
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

92. Install, update, uninstall, hidden, suspended, shielded, managed, unmanaged, unsupported, or permission-limited state where the platform exposes it.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-006`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 205
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

93. It is partial on every platform.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-007`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 210
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

94. It can miss portable apps, per-user installs, wrapped apps, web apps, and apps hidden by platform privacy.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-008`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 211
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: reports

### App Evidence: What Is Possible

#### Installed App Inventory

95. It can report apps that are installed but never used.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-009`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 213
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: setup

### App Evidence: What Is Possible

#### Installed App Inventory

96. Mobile app lists can be package-visibility-limited, tokenized, supervised-only, or MDM-only.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-010`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 214
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: schedule

### App Evidence: What Is Possible

#### Installed App Inventory

97. Inventory should never be used as proof of activity without runtime evidence.

- settingId: `app-guide-app-evidence-what-is-possible-installed-app-inventory-011`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 216
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

#### Process And Window Evidence

98. Process id, parent process id, executable path, process name, command-line handling status, user/session reference, and launch time where available.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-001`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 222
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### App Evidence: What Is Possible

#### Process And Window Evidence

99. Publisher/signature/hash metadata where safe.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-002`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 224
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### App Evidence: What Is Possible

#### Process And Window Evidence

100.  Window id, title, active/foreground state, minimized/background state, and last foreground timestamp where available.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-003`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 225
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

101.  Sessionization into running and foreground durations.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-004`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 227
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### App Evidence: What Is Possible

#### Process And Window Evidence

102.  Unknown, permission-limited, stale, unsupported, and adapter-error states.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-005`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 228
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

103.  Process names can be renamed.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-006`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 232
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

104.  Helper processes may not represent user-facing apps.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-007`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 233
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

105.  Foreground window title may contain sensitive text and may be stale or misleading.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-008`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 234
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

106.  Foreground proof does not reveal what happened inside the app.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-009`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 236
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### App Evidence: What Is Possible

#### Process And Window Evidence

107.  Background process duration is not the same as child attention.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-010`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 237
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### App Evidence: What Is Possible

#### Process And Window Evidence

108.  Elevated, protected, sandboxed, or cross-user processes can be unreadable or uncontrollable.

- settingId: `app-guide-app-evidence-what-is-possible-process-and-window-evidence-011`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 238
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### App Evidence: What Is Possible

#### Foreground Use And Duration

109.  Which app is active now?

- settingId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration-001`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 245
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

110.  How long was this app in foreground today?

- settingId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration-002`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 246
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

111.  Did the time budget run out?

- settingId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration-003`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 247
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

112.  Which evidence ids prove the count?

- settingId: `app-guide-app-evidence-what-is-possible-foreground-use-and-duration-004`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 248
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### App Evidence: What Is Possible

#### App Categories

113.  Education.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-001`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 264
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

114.  Productivity.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-002`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 265
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

115.  Communication.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-003`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 266
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

116.  Entertainment.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-004`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 267
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

117.  Social.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-005`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 268
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

118.  Browser.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-006`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 269
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

119.  Game.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-007`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 270
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

120.  Creative.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-008`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 271
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

121.  System.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-009`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 272
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

122.  Unknown.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-010`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 273
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

123.  Category is not content.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-011`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 280
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

124.  Category confidence must be recorded.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-012`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 281
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: enforcement

### App Evidence: What Is Possible

#### App Categories

125.  Parent rules decide actions. Category labels alone should not block.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-013`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 282
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### App Evidence: What Is Possible

#### App Categories

126.  Some platforms expose categories as opaque tokens rather than raw identifiers.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-014`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 283
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

127.  Unknown or ambiguous categories should degrade to observe, ask, or parent review according to explicit policy.

- settingId: `app-guide-app-evidence-what-is-possible-app-categories-015`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 284
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: approvals

### App Control: What Is Possible

#### Launch

128.  Open an approved app.

- settingId: `app-guide-app-control-what-is-possible-launch-001`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 296
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: schedule

### App Control: What Is Possible

#### Launch

129.  Relaunch a blocked/closed app later after a time budget resets.

- settingId: `app-guide-app-control-what-is-possible-launch-002`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 297
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

## Tab: approvals

### App Control: What Is Possible

#### Launch

130.  Open an app as part of an ask-parent approval.

- settingId: `app-guide-app-control-what-is-possible-launch-003`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 298
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### App Control: What Is Possible

#### Launch

131.  Prefer a managed app path or managed browser path for certain tasks.

- settingId: `app-guide-app-control-what-is-possible-launch-004`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 299
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

132.  Launching an app does not guarantee it stays foreground.

- settingId: `app-guide-app-control-what-is-possible-launch-005`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 303
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

133.  Launching an unmanaged app can move outside Ocentra control.

- settingId: `app-guide-app-control-what-is-possible-launch-006`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 304
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

134.  Mobile launch behavior depends on platform foreground and intent rules.

- settingId: `app-guide-app-control-what-is-possible-launch-007`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 305
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### App Control: What Is Possible

#### Terminate

135.  Stop an app after a block or time-limit decision.

- settingId: `app-guide-app-control-what-is-possible-terminate-001`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 311
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

## Tab: enforcement

### App Control: What Is Possible

#### Terminate

136.  Stop an owned child process.

- settingId: `app-guide-app-control-what-is-possible-terminate-002`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 312
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

137.  Stop a target process when identity still matches the policy target.

- settingId: `app-guide-app-control-what-is-possible-terminate-003`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 313
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

138.  Record already-exited, target-changed, permission-limited, failed, or succeeded results.

- settingId: `app-guide-app-control-what-is-possible-terminate-004`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 314
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

139.  Termination can lose unsaved work.

- settingId: `app-guide-app-control-what-is-possible-terminate-005`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 319
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

140.  Target processes can exit and relaunch between detection and action.

- settingId: `app-guide-app-control-what-is-possible-terminate-006`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 320
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### App Control: What Is Possible

#### Terminate

141.  Parent/child UX should support grace periods, warnings, and ask-parent flows.

- settingId: `app-guide-app-control-what-is-possible-terminate-007`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 321
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

## Tab: enforcement

### App Control: What Is Possible

#### Terminate

142.  Some platforms do not allow third-party apps to kill other apps.

- settingId: `app-guide-app-control-what-is-possible-terminate-008`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 322
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

143.  Protected, elevated, system, or different-user processes may be unavailable.

- settingId: `app-guide-app-control-what-is-possible-terminate-009`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 323
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### Suspend, Hide, Shield, Or Block

144.  Windows application control policy, AppLocker, WDAC/App Control for Business, or a narrower service adapter where proven.

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-001`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 332
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: setup

### App Control: What Is Possible

#### Suspend, Hide, Shield, Or Block

145.  macOS MDM profile, system extension, endpoint/security tooling, or managed app restriction where entitled and deployed.

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-002`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 334
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: enforcement

### App Control: What Is Possible

#### Suspend, Hide, Shield, Or Block

146.  Linux policy, desktop/session integration, package-manager restriction, or service-level control where proven.

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-003`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 336
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: approvals

### App Control: What Is Possible

#### Suspend, Hide, Shield, Or Block

147.  Android DevicePolicyManager package hide/suspend, device owner/profile owner, managed configuration, or accessibility/VPN-adjacent UX where approved.

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-004`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 338
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: schedule

### App Control: What Is Possible

#### Suspend, Hide, Shield, Or Block

148.  iOS/iPadOS Screen Time Family Controls, Managed Settings shields, Device Activity thresholds, or MDM restrictions where entitled/supervised.

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-005`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 340
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: enforcement

### App Control: What Is Possible

#### Suspend, Hide, Shield, Or Block

149.  Broad app blocking is a privileged OS capability, not a normal UI toggle.

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-006`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 345
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: setup

### App Control: What Is Possible

#### Suspend, Hide, Shield, Or Block

150.  Policy setup can require admin rights, device-owner enrollment, MDM, supervision, entitlements, app review, signing, or store distribution.

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-007`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 346
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: enforcement

### App Control: What Is Possible

#### Suspend, Hide, Shield, Or Block

151.  Some systems support shielding/visibility restrictions rather than process termination.

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-008`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 348
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

152.  Platform APIs may expose opaque identifiers for privacy.

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-009`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 350
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: setup

### App Control: What Is Possible

#### Suspend, Hide, Shield, Or Block

153.  Rollback and uninstall paths must be documented before strict policies ship.

- settingId: `app-guide-app-control-what-is-possible-suspend-hide-shield-or-block-010`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 351
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: schedule

### App Control: What Is Possible

#### Time Limits

154.  App/session identity.

- settingId: `app-guide-app-control-what-is-possible-time-limits-001`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 359
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

155.  Running or foreground duration proof.

- settingId: `app-guide-app-control-what-is-possible-time-limits-002`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 360
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

156.  Schedule and budget state.

- settingId: `app-guide-app-control-what-is-possible-time-limits-003`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 361
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

157.  Warning threshold and grace state.

- settingId: `app-guide-app-control-what-is-possible-time-limits-004`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 362
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

158.  Parent approval or extension state.

- settingId: `app-guide-app-control-what-is-possible-time-limits-005`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 363
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

159.  Enforcement fallback for unsupported action.

- settingId: `app-guide-app-control-what-is-possible-time-limits-006`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `parent-domain`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 364
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

160.  Audit event for warning, timeout, action, failure, extension, and rollback.

- settingId: `app-guide-app-control-what-is-possible-time-limits-007`
- policyLane: `schedule`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 365
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

161.  A timer without action is report-only.

- settingId: `app-guide-app-control-what-is-possible-time-limits-008`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 369
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

162.  Foreground time and running time should be separate settings.

- settingId: `app-guide-app-control-what-is-possible-time-limits-009`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 370
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

163.  Cross-device time budgets need sync/custody rules.

- settingId: `app-guide-app-control-what-is-possible-time-limits-010`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 371
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

164.  Mobile time-limit enforcement depends on platform-specific APIs.

- settingId: `app-guide-app-control-what-is-possible-time-limits-011`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 372
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: setup

### App Control: What Is Possible

#### Install And Uninstall

165.  Windows MSI/MSIX/package manager or managed installer.

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-001`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 381
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

166.  macOS installer/package/MDM managed app or declarative package management.

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-002`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 382
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

167.  Linux package manager, Flatpak, Snap, AppImage-managed wrapper, or desktop entry.

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-003`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 383
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

168.  Android package installer, device owner/profile owner, managed Play, or MDM.

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-004`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 385
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

169.  iOS/iPadOS MDM managed app distribution and managed app removal.

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-005`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 386
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: approvals

### App Control: What Is Possible

#### Install And Uninstall

170.  Ocentra must not remove personal apps unless a platform-approved managed path and parent/child custody model explicitly allow it.

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-006`
- policyLane: `approvals`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 390
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: setup

### App Control: What Is Possible

#### Install And Uninstall

171.  Store policies, signing, entitlements, user consent, device enrollment, and uninstall rights vary sharply.

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-007`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 392
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: audit

### App Control: What Is Possible

#### Install And Uninstall

172.  Install/uninstall actions must be audited separately from normal app observation.

- settingId: `app-guide-app-control-what-is-possible-install-and-uninstall-008`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 394
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Managed, Unmanaged, And Unknown Apps

#### Managed Apps

173.  Known identity from package/bundle/app token.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps-001`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 403
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

174.  Known policy source.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps-002`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 404
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: setup

### Managed, Unmanaged, And Unknown Apps

#### Managed Apps

175.  Install or update provenance.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps-003`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 405
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: enforcement

### Managed, Unmanaged, And Unknown Apps

#### Managed Apps

176.  Known allowed/blocked/shielded state.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps-004`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 406
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Managed, Unmanaged, And Unknown Apps

#### Managed Apps

177.  Stronger app lifecycle action where the platform supports it.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-managed-apps-005`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 407
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

#### Unmanaged Apps

178.  Running and foreground observation.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-001`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 416
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### Managed, Unmanaged, And Unknown Apps

#### Unmanaged Apps

179.  Session duration.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-002`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 417
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### Managed, Unmanaged, And Unknown Apps

#### Unmanaged Apps

180.  Category candidate.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-003`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 418
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

181.  Ask-parent or warning events.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-004`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 419
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: enforcement

### Managed, Unmanaged, And Unknown Apps

#### Unmanaged Apps

182.  Terminate where allowed.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-005`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 420
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: reports

### Managed, Unmanaged, And Unknown Apps

#### Unmanaged Apps

183.  Report-only unknown or bypass state.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unmanaged-apps-006`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 421
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### Managed, Unmanaged, And Unknown Apps

#### Unknown Apps

184.  Observe only.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps-001`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 430
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: approvals

### Managed, Unmanaged, And Unknown Apps

#### Unknown Apps

185.  Ask parent on first run.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps-002`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 431
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### Managed, Unmanaged, And Unknown Apps

#### Unknown Apps

186.  Warn child.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps-003`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 432
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### Managed, Unmanaged, And Unknown Apps

#### Unknown Apps

187.  Count time under unknown-app budget.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps-004`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 433
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: enforcement

### Managed, Unmanaged, And Unknown Apps

#### Unknown Apps

188.  Block or terminate only when the parent selected that posture and the platform adapter can prove the action.

- settingId: `app-guide-managed-unmanaged-and-unknown-apps-unknown-apps-005`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 434
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### Child-Facing Actions

#### Child-Facing Actions

189.  Warn before time limit.

- settingId: `app-guide-child-facing-actions-child-facing-actions-001`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 445
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

190.  Show time remaining.

- settingId: `app-guide-child-facing-actions-child-facing-actions-002`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 446
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: approvals

### Child-Facing Actions

#### Child-Facing Actions

191.  Show that parent approval is needed.

- settingId: `app-guide-child-facing-actions-child-facing-actions-003`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 447
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### Child-Facing Actions

#### Child-Facing Actions

192.  Show whether an app is paused, shielded, blocked, or closed by parent policy.

- settingId: `app-guide-child-facing-actions-child-facing-actions-004`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 448
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: schedule

### Child-Facing Actions

#### Child-Facing Actions

193.  Offer ask-parent, request more time, or use allowed alternative where policy supports it.

- settingId: `app-guide-child-facing-actions-child-facing-actions-005`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 449
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### Child-Facing Actions

#### Child-Facing Actions

194.  Hide parent diagnostics, evidence ids, adapter errors, hashes, and internal policy fields from the child surface.

- settingId: `app-guide-child-facing-actions-child-facing-actions-006`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 451
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: reports

### Reports, Custody, Retention, And Audit

#### Reports, Custody, Retention, And Audit

195.  Installed/detectable app inventory.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 470
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

196.  Running now.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-002`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 471
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

197.  Foreground now.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-003`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 472
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

198.  Recent sessions.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-004`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 473
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

199.  Daily app/category rollups.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-005`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 474
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### Reports, Custody, Retention, And Audit

#### Reports, Custody, Retention, And Audit

200.  Time budgets and remaining time.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-006`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 475
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

## Tab: reports

### Reports, Custody, Retention, And Audit

#### Reports, Custody, Retention, And Audit

201.  Unknown and permission-limited apps.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-007`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `permission-limited`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-limited`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 476
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

202.  Policy decisions.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-008`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 477
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

203.  Enforcement actions and failures.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-009`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 478
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

## Tab: approvals

### Reports, Custody, Retention, And Audit

#### Reports, Custody, Retention, And Audit

204.  Approval requests and parent responses.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-010`
- policyLane: `approvals`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 479
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

## Tab: reports

### Reports, Custody, Retention, And Audit

#### Reports, Custody, Retention, And Audit

205.  Evidence ids.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-011`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 483
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

206.  Source adapter.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-012`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 484
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

207.  Capability state.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-013`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 485
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

208.  Custody label.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-014`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 486
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

209.  Collection scope.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-015`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 487
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

210.  Retention policy.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-016`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 488
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

211.  Redaction status.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-017`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 489
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

212.  Policy version and decision id when policy contributed.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-018`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 490
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

213.  Adapter result id when enforcement contributed.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-019`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 491
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

214.  Raw process/window evidence should be retained for the shortest useful local audit window.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-020`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 495
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

215.  Daily rollups can be retained longer than raw observations if they are redacted.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-021`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 497
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

216.  Exact executable paths and window titles may be sensitive and should have narrower retention and reveal controls.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-022`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 499
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

217.  Ocentra-hosted storage is not the default child-activity store.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-023`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 501
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

218.  Parent export and deletion must preserve audit integrity while respecting retention settings.

- settingId: `app-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-024`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 502
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

## Tab: setup

### Platform Capability Notes

#### Windows

219.  Installed-app inventory from uninstall records, Start Menu shortcuts, Microsoft Store packages, known install paths, package query APIs, and executable metadata.

- settingId: `app-guide-platform-capability-notes-windows-001`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 514
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Platform Capability Notes

#### Windows

220.  Process enumeration and process metadata.

- settingId: `app-guide-platform-capability-notes-windows-002`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 517
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

221.  Foreground-window observation.

- settingId: `app-guide-platform-capability-notes-windows-003`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 518
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

222.  Running and foreground sessionization.

- settingId: `app-guide-platform-capability-notes-windows-004`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 519
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: enforcement

### Platform Capability Notes

#### Windows

223.  Owned-process launch and termination.

- settingId: `app-guide-platform-capability-notes-windows-005`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 520
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### Platform Capability Notes

#### Windows

224.  Narrow target process termination after typed policy decisions.

- settingId: `app-guide-platform-capability-notes-windows-006`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 521
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: setup

### Platform Capability Notes

#### Windows

225.  Broad app control through AppLocker, WDAC/App Control for Business, managed installer policy, or equivalent only after explicit host proof.

- settingId: `app-guide-platform-capability-notes-windows-007`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 522
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

226.  Package lifecycle actions through installer/package mechanisms where product setup owns the package.

- settingId: `app-guide-platform-capability-notes-windows-008`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 524
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: audit

### Platform Capability Notes

#### Windows

227.  AppLocker/WDAC behavior depends on Windows edition, policy deployment, signing, administrator rights, audit/enforce mode, and reboot or refresh behavior.

- settingId: `app-guide-platform-capability-notes-windows-009`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 529
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Platform Capability Notes

#### Windows

228.  Microsoft Store package identity and Win32 executable identity are different evidence families.

- settingId: `app-guide-platform-capability-notes-windows-010`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 531
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: setup

### Platform Capability Notes

#### Windows

229.  Blocking by path alone can be bypassed by copy/rename unless hash, signer, or managed installer proof is used.

- settingId: `app-guide-platform-capability-notes-windows-011`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 533
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: schedule

### Platform Capability Notes

#### Windows

230.  The current roadmap distinguishes owned-process terminate and app time-limit proof from broad app blocking. Do not claim broad blocking until the adapter is proven.

- settingId: `app-guide-platform-capability-notes-windows-012`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 535
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Platform Capability Notes

#### macOS

231.  Application bundle inventory.

- settingId: `app-guide-platform-capability-notes-macos-001`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 546
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

232.  Running process and window/frontmost app observation with the required permissions.

- settingId: `app-guide-platform-capability-notes-macos-002`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 547
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: enforcement

### Platform Capability Notes

#### macOS

233.  Launch Services, bundle identifiers, code signing, and app metadata.

- settingId: `app-guide-platform-capability-notes-macos-003`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 549
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

## Tab: setup

### Platform Capability Notes

#### macOS

234.  MDM managed app distribution and restrictions where enrolled.

- settingId: `app-guide-platform-capability-notes-macos-004`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 550
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: rules

### Platform Capability Notes

#### macOS

235.  System Extensions, Endpoint Security, or Network Extension paths where entitled and deployed.

- settingId: `app-guide-platform-capability-notes-macos-005`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 551
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

236.  Managed browser/app controls through configuration profiles where supported.

- settingId: `app-guide-platform-capability-notes-macos-006`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 553
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: setup

### Platform Capability Notes

#### macOS

237.  Accessibility, Screen Recording, Full Disk Access, Endpoint Security, Network Extension, and MDM posture change what is possible.

- settingId: `app-guide-platform-capability-notes-macos-007`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 557
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

238.  Some controls require supervised or managed devices.

- settingId: `app-guide-platform-capability-notes-macos-008`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 559
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: evidence

### Platform Capability Notes

#### macOS

239.  Do not assume Windows process control maps directly to macOS.

- settingId: `app-guide-platform-capability-notes-macos-009`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 560
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: rules

### Platform Capability Notes

#### Linux

240.  Desktop entries and menu categories.

- settingId: `app-guide-platform-capability-notes-linux-001`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 570
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: evidence

### Platform Capability Notes

#### Linux

241.  Package manager inventory.

- settingId: `app-guide-platform-capability-notes-linux-002`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 571
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

## Tab: setup

### Platform Capability Notes

#### Linux

242.  Flatpak, Snap, AppImage, or custom install metadata.

- settingId: `app-guide-platform-capability-notes-linux-003`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 572
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Platform Capability Notes

#### Linux

243.  Process observation through OS process tables.

- settingId: `app-guide-platform-capability-notes-linux-004`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 573
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

244.  Foreground-window observation through X11, Wayland compositor protocols, or desktop-specific APIs where available.

- settingId: `app-guide-platform-capability-notes-linux-005`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 574
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

245.  Process termination where permission permits.

- settingId: `app-guide-platform-capability-notes-linux-006`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 576
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

246.  Policy controls through service, user session, package, desktop, firewall, or container mechanisms where proven.

- settingId: `app-guide-platform-capability-notes-linux-007`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 577
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

247.  Wayland commonly restricts global window inspection compared with X11.

- settingId: `app-guide-platform-capability-notes-linux-008`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 582
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

248.  Desktop entry category metadata is useful but not a complete app ontology.

- settingId: `app-guide-platform-capability-notes-linux-009`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 583
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

249.  Package managers differ by distro.

- settingId: `app-guide-platform-capability-notes-linux-010`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 584
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

## Tab: enforcement

### Platform Capability Notes

#### Linux

250.  Broad app blocking should be treated as manual-required until a concrete adapter is proven on the target distro and desktop.

- settingId: `app-guide-platform-capability-notes-linux-011`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 585
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Platform Capability Notes

#### Android

251.  Package inventory through PackageManager subject to package visibility rules.

- settingId: `app-guide-platform-capability-notes-android-001`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 595
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

## Tab: rules

### Platform Capability Notes

#### Android

252.  Usage events/statistics when the user grants Usage Access or the app has the required privileged context.

- settingId: `app-guide-platform-capability-notes-android-002`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 596
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: approvals

### Platform Capability Notes

#### Android

253.  Foreground visibility through UsageStatsManager or Accessibility where approved and enabled.

- settingId: `app-guide-platform-capability-notes-android-003`
- policyLane: `approvals`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 598
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: setup

### Platform Capability Notes

#### Android

254.  DevicePolicyManager package hiding, suspension, permission policy, managed configuration, and package lifecycle control for device owner/profile owner contexts.

- settingId: `app-guide-platform-capability-notes-android-004`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 600
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

255.  Managed Play or MDM package installation/removal where deployed.

- settingId: `app-guide-platform-capability-notes-android-005`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 603
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Platform Capability Notes

#### Android

256.  Always-on VPN with lockdown for network mediation, separate from app foreground proof.

- settingId: `app-guide-platform-capability-notes-android-006`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 604
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: rules

### Platform Capability Notes

#### Android

257.  A normal Android app cannot generally control all other apps.

- settingId: `app-guide-platform-capability-notes-android-007`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 609
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: setup

### Platform Capability Notes

#### Android

258.  Package visibility rules can hide installed apps from inventory queries.

- settingId: `app-guide-platform-capability-notes-android-008`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 610
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: rules

### Platform Capability Notes

#### Android

259.  Usage access is permission-gated and can be revoked.

- settingId: `app-guide-platform-capability-notes-android-009`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 611
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

260.  Accessibility is sensitive and must not be used as a stealth content capture path.

- settingId: `app-guide-platform-capability-notes-android-010`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 612
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: setup

### Platform Capability Notes

#### Android

261.  Device owner/profile owner changes the capability class and setup burden.

- settingId: `app-guide-platform-capability-notes-android-011`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 614
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: evidence

### Platform Capability Notes

#### Android

262.  The roadmap currently treats Android package lifecycle proof as manual-required until real device artifacts exist.

- settingId: `app-guide-platform-capability-notes-android-012`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 615
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### Platform Capability Notes

#### iOS And iPadOS

263.  Screen Time frameworks: Family Controls, Managed Settings, Device Activity.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-001`
- policyLane: `schedule`; cardKind: `single-choice-compact`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 624
- acceptedOptions: Family Controls | Managed Settings | Device Activity
- helperText: app-control-capability-registry

## Tab: rules

### Platform Capability Notes

#### iOS And iPadOS

264.  Opaque selections for applications, categories, and web domains.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-002`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 625
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### Platform Capability Notes

#### iOS And iPadOS

265.  App/category shielding through Managed Settings.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-003`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 626
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: rules

### Platform Capability Notes

#### iOS And iPadOS

266.  Threshold/event monitoring through Device Activity.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-004`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 627
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: setup

### Platform Capability Notes

#### iOS And iPadOS

267.  MDM managed app install/removal and restrictions for enrolled devices.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-005`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 628
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

268.  Supervised-device restrictions where applicable.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-006`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 629
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

269.  Third-party apps do not get a general raw list of every installed app for parental control.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-007`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 633
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: schedule

### Platform Capability Notes

#### iOS And iPadOS

270.  Screen Time APIs are privacy-preserving and token-based; tokens can be voided if authorization is revoked.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-008`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 635
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### Platform Capability Notes

#### iOS And iPadOS

271.  App shielding is not the same as process termination or raw app telemetry.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-009`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 637
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: setup

### Platform Capability Notes

#### iOS And iPadOS

272.  MDM and supervision determine app install/removal and restriction scope.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-010`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 638
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: schedule

### Platform Capability Notes

#### iOS And iPadOS

273.  Family Controls entitlements, App Store review, TestFlight, and runtime authorization are separate proof requirements.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-011`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 639
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

274.  The roadmap separates iOS signing/entitlements from TestFlight and runtime API entitlements; do not claim iOS control until each is proven.

- settingId: `app-guide-platform-capability-notes-ios-and-ipados-012`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 641
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: setup

### Policy Modes To Represent Later In UI

#### Observe App Use

275.  Detect installed or launchable apps where available.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-001`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 653
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Policy Modes To Represent Later In UI

#### Observe App Use

276.  Show running and foreground state.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-002`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 654
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### Policy Modes To Represent Later In UI

#### Observe App Use

277.  Build running and foreground session durations.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-003`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 655
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### Policy Modes To Represent Later In UI

#### Observe App Use

278.  Show unknown, unsupported, stale, and permission-limited states.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-004`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 656
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: reports

### Policy Modes To Represent Later In UI

#### Observe App Use

279.  Produce reports without changing device behavior.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-005`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 657
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

## Tab: enforcement

### Policy Modes To Represent Later In UI

#### Observe App Use

280.  App blocking.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-006`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 661
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

## Tab: setup

### Policy Modes To Represent Later In UI

#### Observe App Use

281.  Mobile device-owner/supervised setup.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-007`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 662
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: rules

### Policy Modes To Represent Later In UI

#### Observe App Use

282.  Broad application control.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-008`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 663
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### Policy Modes To Represent Later In UI

#### Observe App Use

283.  Guaranteed launch blocking.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-009`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 667
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

## Tab: setup

### Policy Modes To Represent Later In UI

#### Observe App Use

284.  Install/uninstall control.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-010`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 668
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: rules

### Policy Modes To Represent Later In UI

#### Observe App Use

285.  App-internal content knowledge.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-observe-app-use-011`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 669
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

#### Warn Or Ask On App Use

286.  Matching app activity remains allowed temporarily.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-001`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 675
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

287.  The child sees a warning or ask-parent state.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-002`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 676
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: approvals

### Policy Modes To Represent Later In UI

#### Warn Or Ask On App Use

288.  The parent sees the app/session evidence and can approve, deny, or extend.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-003`
- policyLane: `approvals`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 677
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### Policy Modes To Represent Later In UI

#### Warn Or Ask On App Use

289.  App/session evidence.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-004`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 681
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: enforcement

### Policy Modes To Represent Later In UI

#### Warn Or Ask On App Use

290.  Child-facing local UI or notification/shield path.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-005`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 682
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: approvals

### Policy Modes To Represent Later In UI

#### Warn Or Ask On App Use

291.  Parent approval contract.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-006`
- policyLane: `approvals`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 683
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: audit

### Policy Modes To Represent Later In UI

#### Warn Or Ask On App Use

292.  Expiry and audit.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-warn-or-ask-on-app-use-007`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 684
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

## Tab: schedule

### Policy Modes To Represent Later In UI

#### Time-Limit Apps

293.  Running or foreground time is counted against a parent budget.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-001`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 690
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

294.  Warning and grace rules apply.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-002`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 691
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

295.  When the budget expires, the configured action runs if the adapter supports it.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-003`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 692
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

296.  Sessionization from stored evidence.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-004`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 697
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

297.  Timer recovery after restart.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-005`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 698
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

298.  Policy decision and enforcement audit.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-006`
- policyLane: `schedule`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 699
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

299.  Fallback when enforcement is unavailable.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-time-limit-apps-007`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `parent-domain`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 700
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### Policy Modes To Represent Later In UI

#### Block Or Shield Apps

300.  Parent rules prevent or interrupt app access.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-001`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 706
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

301.  On desktop this may mean launch block or process termination.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-002`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 707
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: approvals

### Policy Modes To Represent Later In UI

#### Block Or Shield Apps

302.  On mobile this may mean shield/hide/suspend through approved platform APIs.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-003`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 708
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: enforcement

### Policy Modes To Represent Later In UI

#### Block Or Shield Apps

303.  Platform-specific control proof.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-004`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 712
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

304.  Parent exceptions.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-005`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 713
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

305.  Rollback path.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-006`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 714
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: audit

### Policy Modes To Represent Later In UI

#### Block Or Shield Apps

306.  Audit for every strict action.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-007`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 715
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: enforcement

### Policy Modes To Represent Later In UI

#### Block Or Shield Apps

307.  App deletion.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-008`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 719
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

308.  Exact in-app activity knowledge.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-009`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 720
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

309.  Parity across platforms.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-block-or-shield-apps-010`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 721
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: setup

### Policy Modes To Represent Later In UI

#### Managed App Lifecycle

310.  Install, update, uninstall, hide, or remove managed apps through a platform management boundary.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-001`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 727
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

311.  Managed-device or package-management setup.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-002`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 732
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

312.  Signing/store/MDM or installer proof.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-003`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 733
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: audit

### Policy Modes To Represent Later In UI

#### Managed App Lifecycle

313.  Custody model for personal vs managed apps.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-004`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 734
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

## Tab: reports

### Policy Modes To Represent Later In UI

#### Managed App Lifecycle

314.  Explicit parent-visible state and audit.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-005`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 735
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

## Tab: setup

### Policy Modes To Represent Later In UI

#### Managed App Lifecycle

315.  Personal device and family device expectations differ. The product should not imply corporate MDM behavior unless the device is actually managed that way.

- settingId: `app-guide-policy-modes-to-represent-later-in-ui-managed-app-lifecycle-006`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 739
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

316.  App/game evidence contracts use stored local evidence and query/read models.

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 746
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

317.  Process/window evidence can support native app sessions, running time, and foreground time.

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-002`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 747
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

318.  Local AI can consume evidence references or structured digests; it does not scan the OS.

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-003`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 749
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

319.  Policy decisions must reference evidence and remain deterministic.

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-004`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 751
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

320.  V0.8 enforcement work has typed contracts, capability status, timer/recovery, and audit scaffolding.

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-005`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 752
- acceptedOptions: Represented | Not represented
- helperText: parent-owned-local-storage-and-redaction

321.  Windows has proof direction for owned-process terminate and app time-limit behavior, but broad app blocking remains manual-required until a real adapter proves it.

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-006`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 754
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

322.  Android package lifecycle and iOS Screen Time/entitlement behavior are manual-required until real device/platform proof exists.

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-007`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 757
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

323.  Parent portal is an authoring and visibility surface. It does not run app inventory, timers, policy evaluation, or enforcement.

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-008`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 759
- acceptedOptions: Represented | Not represented
- helperText: typed-local-app-evidence-required

## Tab: evidence

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

324.  [`docs/architecture/app-game-evidence-sessions.md`](architecture/app-game-evidence-sessions.md)

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-009`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 764
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

325.  [`docs/expectations/app-game-evidence.md`](expectations/app-game-evidence.md)

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-010`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 765
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: rules

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

326.  [`docs/expectations/policy.md`](expectations/policy.md)

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-011`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 766
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: enforcement

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

327.  [`docs/expectations/enforcement.md`](expectations/enforcement.md)

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-012`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 767
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: rules

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

328.  [`docs/product-roadmap.md`](product-roadmap.md)

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-013`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 768
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

329.  [`docs/managed-unmanaged-browser.md`](managed-unmanaged-browser.md)

- settingId: `app-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-014`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 769
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: schedule

### Future UI Rules

#### Future UI Rules

330.  Show inventory, running, foreground, time-limit, install, uninstall, shield, suspend, block, and terminate as separate capability rows.

- settingId: `app-guide-future-ui-rules-future-ui-rules-001`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 775
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: evidence

### Future UI Rules

#### Future UI Rules

331.  Show the evidence source next to every app claim.

- settingId: `app-guide-future-ui-rules-future-ui-rules-002`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 777
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: audit

### Future UI Rules

#### Future UI Rules

332.  Show exact package/process identity only when proof exists and retention allows it.

- settingId: `app-guide-future-ui-rules-future-ui-rules-003`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 778
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: evidence

### Future UI Rules

#### Future UI Rules

333.  Show unknown apps as unknown, not as risky by default.

- settingId: `app-guide-future-ui-rules-future-ui-rules-004`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 780
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

334.  Show app category as a label with source/confidence, not as an automatic decision.

- settingId: `app-guide-future-ui-rules-future-ui-rules-005`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 781
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: schedule

### Future UI Rules

#### Future UI Rules

335.  Show app time limits only when duration evidence exists or when the rule is clearly marked as pending proof.

- settingId: `app-guide-future-ui-rules-future-ui-rules-006`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 783
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: setup

### Future UI Rules

#### Future UI Rules

336.  Show strict actions as ready, unsupported, permission-required, manual-required, degraded, dry-run-only, adapter-error, or blocked-by-setup.

- settingId: `app-guide-future-ui-rules-future-ui-rules-007`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 785
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

337.  Keep managed-device setup state close to mobile actions.

- settingId: `app-guide-future-ui-rules-future-ui-rules-008`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 787
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: rules

### Future UI Rules

#### Future UI Rules

338.  Keep child-facing messages separate from parent diagnostics.

- settingId: `app-guide-future-ui-rules-future-ui-rules-009`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 788
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: schedule

### Future UI Rules

#### Future UI Rules

339.  Every strict action should have an audit path: evidence, parent rule, compiled policy, adapter mechanism, outcome, timestamp, and rollback/unavailable state.

- settingId: `app-guide-future-ui-rules-future-ui-rules-010`
- policyLane: `schedule`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 789
- acceptedOptions: Evidence | Parent Rule | Compiled Policy | Adapter Mechanism | Outcome | Timestamp | And Rollback/unavailable State
- helperText: app claims require fresh evidence references with confidence and custody.

## Tab: rules

### Future UI Rules

#### Future UI Rules

340.  observe only;

- settingId: `app-guide-future-ui-rules-future-ui-rules-011`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 794
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

341.  warn on app use;

- settingId: `app-guide-future-ui-rules-future-ui-rules-012`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 795
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: approvals

### Future UI Rules

#### Future UI Rules

342.  ask parent on app use;

- settingId: `app-guide-future-ui-rules-future-ui-rules-013`
- policyLane: `approvals`; cardKind: `toggle`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 796
- acceptedOptions: Represented | Not represented
- helperText: app-control-capability-registry

## Tab: schedule

### Future UI Rules

#### Future UI Rules

343.  set app/category/unknown-app time budgets;

- settingId: `app-guide-future-ui-rules-future-ui-rules-014`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: app claims require fresh evidence references with confidence and custody.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 797
- acceptedOptions: Represented | Not represented
- helperText: app claims require fresh evidence references with confidence and custody.

344.  close or terminate selected apps after a timer;

- settingId: `app-guide-future-ui-rules-future-ui-rules-015`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 798
- acceptedOptions: Represented | Not represented
- helperText: platform-adapter-proof-required-before-product-claim

## Tab: enforcement

### Future UI Rules

#### Future UI Rules

345.  shield, suspend, or block apps where the platform supports it;

- settingId: `app-guide-future-ui-rules-future-ui-rules-016`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `derived`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 799
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.

## Tab: approvals

### Future UI Rules

#### Future UI Rules

346.  manage app installs/uninstalls only inside a platform-approved custody model.

- settingId: `app-guide-future-ui-rules-future-ui-rules-017`
- policyLane: `approvals`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: strict app control requires real platform adapter or managed-device proof.
- sourceDocument: `docs/app-control-capability-guide.md`; sourceLine: 800
- acceptedOptions: Represented | Not represented
- helperText: strict app control requires real platform adapter or managed-device proof.
