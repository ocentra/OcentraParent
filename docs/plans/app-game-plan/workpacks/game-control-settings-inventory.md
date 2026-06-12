<!-- agent-capsule -->

> Agent Capsule
> Doc: Game Control Settings Inventory
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Game Control Settings Inventory

Generated from `BaselineGameControlAuthoringManifest`.
Total settings: 33

Use this as the raw review list for deciding parent-facing grouping, proof gaps, and policy UX.
This is a generated inventory of current typed catalog data, not product-complete implementation proof.

## Source Documents

- docs/game-control-capability-guide.md
- docs/game-control-schema-proposal.md

## Lane: rules

### game-management

#### game-management

1.  Enable game management?

- settingId: `game.enabled`
- policyLane: `rules`; sectionId: `game-management`; groupId: `game-management`
- cardKind: `toggle-card`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Enable game management?
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

2.  What should happen to game activity?

- settingId: `game.defaultPosture`
- policyLane: `rules`; sectionId: `game-management`; groupId: `game-management`
- cardKind: `many-option-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What should happen to game activity?
- acceptedOptions: Allow | Observe | Warn | Ask | Limit | Block
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

3.  How should game management run on this device?

- settingId: `game.managementMode`
- policyLane: `rules`; sectionId: `game-management`; groupId: `game-management`
- cardKind: `many-option-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `parent-domain`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: How should game management run on this device?
- acceptedOptions: Child device local | LAN live | Platform family controls | Authoring only | Unavailable
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

### inventory

#### inventory

4.  Which game inventory should be used?

- settingId: `inventory.mode`
- policyLane: `rules`; sectionId: `inventory`; groupId: `inventory`
- cardKind: `many-option-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unknown and possible-game evidence must stay labeled until deterministic proof exists.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which game inventory should be used?
- acceptedOptions: off | running-only | local-installed-and-running | launcher-backed | platform-family-controls | manual-list-only
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

5.  Which inventory sources may contribute game evidence?

- settingId: `inventory.sources`
- policyLane: `rules`; sectionId: `inventory`; groupId: `inventory`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unknown and possible-game evidence must stay labeled until deterministic proof exists.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which inventory sources may contribute game evidence?
- acceptedOptions: process-snapshot | foreground-window | installed-app-records | start-menu-shortcuts | store-packages | launcher-manifests | manual-parent-catalog | platform-family-controls | browser-managed-url | network-service-hint
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

6.  Which game classifications should appear in rules?

- settingId: `inventory.classificationStates`
- policyLane: `rules`; sectionId: `inventory`; groupId: `inventory`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-domain`; capabilityState: `degraded`
- proofRequirement: Unknown and possible-game evidence must stay labeled until deterministic proof exists.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which game classifications should appear in rules?
- acceptedOptions: known-game | known-app | known-launcher | launcher-game-candidate | possibly-game | unknown-process | permission-limited | unsupported-platform | stale | adapter-error
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

7.  Which rating or category sources may be used?

- settingId: `inventory.ratingSources`
- policyLane: `rules`; sectionId: `inventory`; groupId: `inventory`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unknown and possible-game evidence must stay labeled until deterministic proof exists.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which rating or category sources may be used?
- acceptedOptions: store-metadata | launcher-metadata | parent-catalog | rating-authority | local-classifier-digest | unknown
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

### game-rules

#### game-rules

8.  What game targets should rules match?

- settingId: `rules.allowedTargetTypes`
- policyLane: `rules`; sectionId: `game-rules`; groupId: `game-rules`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What game targets should rules match?
- acceptedOptions: known-game | game-title | game-category | rating-threshold | launcher-kind | launcher-app-id | executable-identity | package-id | game-session | possibly-game | unknown-process | browser-game-site | cloud-game-service | capability-state
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

9.  What actions may game rules use?

- settingId: `rules.allowedActions`
- policyLane: `rules`; sectionId: `game-rules`; groupId: `game-rules`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Strict action proof requires current target recheck, adapter capability, audit, and rollback path.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What actions may game rules use?
- acceptedOptions: allow | monitor | warn | ask | limit | terminate | block-launch | temporary-block | platform-shield | manual-required
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

10. Which game rules should apply?

- settingId: `rules.items`
- policyLane: `rules`; sectionId: `game-rules`; groupId: `game-rules`
- cardKind: `rule-list-card`; selectionMode: `none`; controlKind: `rule-list`
- effectStatus: `needs-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which game rules should apply?
- acceptedOptions: none
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Lane: schedule

### budgets

#### budgets

11. Use game time budgets?

- settingId: `budgets.enabled`
- policyLane: `schedule`; sectionId: `budgets`; groupId: `budgets`
- cardKind: `toggle-card`; selectionMode: `none`; controlKind: `toggle`
- effectStatus: `needs-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Use game time budgets?
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

12. How many game minutes are allowed per day?

- settingId: `budgets.defaultDailyMinutes`
- policyLane: `schedule`; sectionId: `budgets`; groupId: `budgets`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: How many game minutes are allowed per day?
- acceptedOptions: none
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

13. How many minutes before a limit should the child be warned?

- settingId: `budgets.warningThresholdMinutes`
- policyLane: `schedule`; sectionId: `budgets`; groupId: `budgets`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: How many minutes before a limit should the child be warned?
- acceptedOptions: none
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

14. How many grace minutes are allowed before strict action?

- settingId: `budgets.graceMinutes`
- policyLane: `schedule`; sectionId: `budgets`; groupId: `budgets`
- cardKind: `status-card`; selectionMode: `none`; controlKind: `number`
- effectStatus: `needs-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: How many grace minutes are allowed before strict action?
- acceptedOptions: none
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Lane: approvals

### approvals

#### approvals

15. What should need parent approval?

- settingId: `approvals.requiredFor`
- policyLane: `approvals`; sectionId: `approvals`; groupId: `approvals`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What should need parent approval?
- acceptedOptions: blocked-game | new-game | unknown-game | possibly-game | launcher-game-candidate | time-extension | rating-threshold | multiplayer-capable-game | platform-setup
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

16. What happens if parent does not answer?

- settingId: `approvals.unansweredDefault`
- policyLane: `approvals`; sectionId: `approvals`; groupId: `approvals`
- cardKind: `compact-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What happens if parent does not answer?
- acceptedOptions: deny | allow-temporarily | continue-observe-only | keep-waiting
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

17. Which parent responses are allowed?

- settingId: `approvals.allowedParentResponses`
- policyLane: `approvals`; sectionId: `approvals`; groupId: `approvals`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which parent responses are allowed?
- acceptedOptions: approve-once | approve-session | approve-until-time | approve-for-schedule | deny | extend-time | cancel
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Lane: enforcement

### native-games

#### native-games

18. How should native games be handled?

- settingId: `nativeGames.mode`
- policyLane: `enforcement`; sectionId: `native-games`; groupId: `native-games`
- cardKind: `many-option-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: How should native games be handled?
- acceptedOptions: off | observe | warn | ask | observe-and-limit | block-when-proven
- helperText: Native game controls depend on process, package, foreground, and protected-process capability proof.

19. Which native game identities may rules target?

- settingId: `nativeGames.allowedIdentityTypes`
- policyLane: `enforcement`; sectionId: `native-games`; groupId: `native-games`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which native game identities may rules target?
- acceptedOptions: package-id | executable-path | executable-hash | publisher-signature | launcher-app-id | game-title | game-category | rating-threshold | unknown-candidate
- helperText: Native game controls depend on process, package, foreground, and protected-process capability proof.

20. Which strict native game actions may be used?

- settingId: `nativeGames.strictActions`
- policyLane: `enforcement`; sectionId: `native-games`; groupId: `native-games`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict action proof requires current target recheck, adapter capability, audit, and rollback path.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which strict native game actions may be used?
- acceptedOptions: ask | time-limit | terminate-accessible-process | block-launch | temporary-block | repair-required | observe-only
- helperText: Native game controls depend on process, package, foreground, and protected-process capability proof.

### launcher-games

#### launcher-games

21. Which launchers or stores should be considered?

- settingId: `launchers.supportedKinds`
- policyLane: `enforcement`; sectionId: `launcher-games`; groupId: `launcher-games`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Launcher proof must not treat launcher-only activity as active gameplay.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which launchers or stores should be considered?
- acceptedOptions: steam | epic-games | xbox-app | microsoft-store | riot-client | battle-net | ea-app | ubisoft-connect | gog-galaxy | roblox | minecraft-launcher | unknown-launcher
- helperText: Launcher activity is not automatically game play; manifests and child-process attribution remain separate.

22. How should launcher-only time be handled?

- settingId: `launchers.launcherOnlyHandling`
- policyLane: `enforcement`; sectionId: `launcher-games`; groupId: `launcher-games`
- cardKind: `many-option-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Launcher proof must not treat launcher-only activity as active gameplay.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: How should launcher-only time be handled?
- acceptedOptions: ignore | report-separately | count-as-possible-game | ask-after-threshold | block
- helperText: Launcher activity is not automatically game play; manifests and child-process attribution remain separate.

23. What if launcher manifests cannot be read?

- settingId: `launchers.whenManifestUnavailable`
- policyLane: `enforcement`; sectionId: `launcher-games`; groupId: `launcher-games`
- cardKind: `compact-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Launcher proof must not treat launcher-only activity as active gameplay.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What if launcher manifests cannot be read?
- acceptedOptions: use-process-evidence-only | mark-unavailable | ask | manual-parent-catalog
- helperText: Launcher activity is not automatically game play; manifests and child-process attribution remain separate.

### browser-cloud-games

#### browser-cloud-games

24. How should browser and cloud games be counted?

- settingId: `browserCloud.mode`
- policyLane: `enforcement`; sectionId: `browser-cloud-games`; groupId: `browser-cloud-games`
- cardKind: `many-option-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Browser-game title proof requires managed browser URL/title evidence or explicit platform integration.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: How should browser and cloud games be counted?
- acceptedOptions: off | report-only | managed-proof-only | domain-service-hint | count-cloud-client | ask
- helperText: Browser and cloud games keep their surface-specific proof boundary; network hints are not exact title proof.

25. Which evidence may classify browser or cloud game use?

- settingId: `browserCloud.allowedEvidence`
- policyLane: `enforcement`; sectionId: `browser-cloud-games`; groupId: `browser-cloud-games`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Browser-game title proof requires managed browser URL/title evidence or explicit platform integration.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which evidence may classify browser or cloud game use?
- acceptedOptions: managed-browser-url | managed-browser-title | domain-service-hint | cloud-client-process | network-flow-service-hint | platform-family-activity | parent-catalog
- helperText: Browser and cloud games keep their surface-specific proof boundary; network hints are not exact title proof.

## Lane: audit

### audit

#### audit

26. What should game actions audit?

- settingId: `audit.requiredFields`
- policyLane: `audit`; sectionId: `audit`; groupId: `audit`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What should game actions audit?
- acceptedOptions: policy-decision | evidence-ref | ai-ref | adapter-result | timer-state | parent-override | rollback | policy-version | capability-state | custody-label | protected-process-status | target-recheck
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Lane: evidence

### session-evidence

#### session-evidence

27. What proof is enough for game rules?

- settingId: `evidence.requiredProof`
- policyLane: `evidence`; sectionId: `session-evidence`; groupId: `session-evidence`
- cardKind: `many-option-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What proof is enough for game rules?
- acceptedOptions: process-running | foreground-window | foreground-known-game-session | launcher-attributed-session | package-identity-session | managed-browser-game-url | platform-family-activity | manual-parent-catalog
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

28. Which time should count toward game budgets?

- settingId: `evidence.durationCountingMode`
- policyLane: `evidence`; sectionId: `session-evidence`; groupId: `session-evidence`
- cardKind: `many-option-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Which time should count toward game budgets?
- acceptedOptions: foreground-game-time | running-game-process-time | launcher-child-game-time | known-game-only-time | known-and-possible-game-time | browser-managed-game-time | cloud-client-foreground-time | platform-reported-game-time
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

29. What if game proof is unavailable?

- settingId: `evidence.whenProofUnavailable`
- policyLane: `evidence`; sectionId: `session-evidence`; groupId: `session-evidence`
- cardKind: `many-option-single-choice`; selectionMode: `none`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What if game proof is unavailable?
- acceptedOptions: allow | observe | warn | ask | block-until-ready | mark-unavailable
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

30. What must game rules never collect?

- settingId: `evidence.neverCollect`
- policyLane: `evidence`; sectionId: `session-evidence`; groupId: `session-evidence`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What must game rules never collect?
- acceptedOptions: screenshots | keystrokes | chat-content | voice-content | game-memory | decrypted-network-payload | launcher-credentials | private-social-graph | raw-anti-cheat-data | purchase-history | cloud-save-content
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Lane: reports

### reports

#### reports

31. What should parents see in game reports?

- settingId: `reports.visibleFields`
- policyLane: `reports`; sectionId: `reports`; groupId: `reports`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: What should parents see in game reports?
- acceptedOptions: inventory-status | running-now | foreground-now | recent-sessions | daily-rollups | unknown-candidates | launcher-status | rating-category | approval-events | block-results | time-budget | policy-decisions | source-capability
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

32. How long should raw process/window game evidence be kept?

- settingId: `retention.rawEvidence`
- policyLane: `reports`; sectionId: `reports`; groupId: `reports`
- cardKind: `retention-card`; selectionMode: `none`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: How long should raw process/window game evidence be kept?
- acceptedOptions: fresh-only | 24-hours | 7-days | 30-days | until-reset | delete-expired
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

33. Where may game data be used?

- settingId: `custody.allowedUses`
- policyLane: `reports`; sectionId: `reports`; groupId: `reports`
- cardKind: `many-option-multi-choice`; selectionMode: `none`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/game-control-schema-proposal.md
- sourceLine: none; sourceText: Where may game data be used?
- acceptedOptions: child-local | lan-live | parent-cache | parent-export | parent-report | unavailable
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.
