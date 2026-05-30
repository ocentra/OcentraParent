# Game Control Settings Inventory

Generated from `BaselineGameControlAuthoringManifest`.
Total settings: 33
Note: Games currently has 33 formal authoring settings from the schema proposal.
Note: The capability guide is represented as capability truths and a capability registry, not as separate parent-facing settings yet.
Note: Those guide-derived constraints are listed after the settings so the grouping pass can decide whether any should become explicit questions.

Use this as the raw review list for deciding parent-facing grouping.

## Tab: rules

### Game management

#### Game management

1.  Enable game management?

- settingId: `game.enabled`
- policyLane: `rules`; cardKind: `toggle-card`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

2.  What should happen to game activity?

- settingId: `game.defaultPosture`
- policyLane: `rules`; cardKind: `many-option-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: Allow | Observe | Warn | Ask | Limit | Block
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

3.  How should game management run on this device?

- settingId: `game.managementMode`
- policyLane: `rules`; cardKind: `many-option-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `parent-domain`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: Child device local | LAN live | Platform family controls | Authoring only | Unavailable
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

### Game inventory

#### Game inventory

4.  Which game inventory should be used?

- settingId: `inventory.mode`
- policyLane: `rules`; cardKind: `many-option-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unknown and possible-game evidence must stay labeled until deterministic proof exists.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: off | running-only | local-installed-and-running | launcher-backed | platform-family-controls | manual-list-only
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

5.  Which inventory sources may contribute game evidence?

- settingId: `inventory.sources`
- policyLane: `rules`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unknown and possible-game evidence must stay labeled until deterministic proof exists.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: process-snapshot | foreground-window | installed-app-records | start-menu-shortcuts | store-packages | launcher-manifests | manual-parent-catalog | platform-family-controls | browser-managed-url | network-service-hint
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

6.  Which game classifications should appear in rules?

- settingId: `inventory.classificationStates`
- policyLane: `rules`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-domain`; capabilityState: `degraded`
- proofRequirement: Unknown and possible-game evidence must stay labeled until deterministic proof exists.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: known-game | known-app | known-launcher | launcher-game-candidate | possibly-game | unknown-process | permission-limited | unsupported-platform | stale | adapter-error
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

7.  Which rating or category sources may be used?

- settingId: `inventory.ratingSources`
- policyLane: `rules`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Unknown and possible-game evidence must stay labeled until deterministic proof exists.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: store-metadata | launcher-metadata | parent-catalog | rating-authority | local-classifier-digest | unknown
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

### Game rules

#### Game rules

8.  What game targets should rules match?

- settingId: `rules.allowedTargetTypes`
- policyLane: `rules`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: known-game | game-title | game-category | rating-threshold | launcher-kind | launcher-app-id | executable-identity | package-id | game-session | possibly-game | unknown-process | browser-game-site | cloud-game-service | capability-state
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

9.  What actions may game rules use?

- settingId: `rules.allowedActions`
- policyLane: `rules`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Strict action proof requires current target recheck, adapter capability, audit, and rollback path.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: allow | monitor | warn | ask | limit | terminate | block-launch | temporary-block | platform-shield | manual-required
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

10. Which game rules should apply?

- settingId: `rules.items`
- policyLane: `rules`; cardKind: `rule-list-card`; selectionMode: `derived`; controlKind: `rule-list`
- effectStatus: `needs-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Tab: schedule

### Game time budgets

#### Game time budgets

11. Use game time budgets?

- settingId: `budgets.enabled`
- policyLane: `schedule`; cardKind: `toggle-card`; selectionMode: `derived`; controlKind: `toggle`
- effectStatus: `needs-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

12. How many game minutes are allowed per day?

- settingId: `budgets.defaultDailyMinutes`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

13. How many minutes before a limit should the child be warned?

- settingId: `budgets.warningThresholdMinutes`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

14. How many grace minutes are allowed before strict action?

- settingId: `budgets.graceMinutes`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `derived`; controlKind: `number`
- effectStatus: `needs-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Tab: approvals

### Parent approvals

#### Parent approvals

15. What should need parent approval?

- settingId: `approvals.requiredFor`
- policyLane: `approvals`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: blocked-game | new-game | unknown-game | possibly-game | launcher-game-candidate | time-extension | rating-threshold | multiplayer-capable-game | platform-setup
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

16. What happens if parent does not answer?

- settingId: `approvals.unansweredDefault`
- policyLane: `approvals`; cardKind: `compact-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: deny | allow-temporarily | continue-observe-only | keep-waiting
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

17. Which parent responses are allowed?

- settingId: `approvals.allowedParentResponses`
- policyLane: `approvals`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: approve-once | approve-session | approve-until-time | approve-for-schedule | deny | extend-time | cancel
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Tab: enforcement

### Native games

#### Native games

18. How should native games be handled?

- settingId: `nativeGames.mode`
- policyLane: `enforcement`; cardKind: `many-option-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: off | observe | warn | ask | observe-and-limit | block-when-proven
- helperText: Native game controls depend on process, package, foreground, and protected-process capability proof.

19. Which native game identities may rules target?

- settingId: `nativeGames.allowedIdentityTypes`
- policyLane: `enforcement`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `needs-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: package-id | executable-path | executable-hash | publisher-signature | launcher-app-id | game-title | game-category | rating-threshold | unknown-candidate
- helperText: Native game controls depend on process, package, foreground, and protected-process capability proof.

20. Which strict native game actions may be used?

- settingId: `nativeGames.strictActions`
- policyLane: `enforcement`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict action proof requires current target recheck, adapter capability, audit, and rollback path.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: ask | time-limit | terminate-accessible-process | block-launch | temporary-block | repair-required | observe-only
- helperText: Native game controls depend on process, package, foreground, and protected-process capability proof.

### Launchers and stores

#### Launchers and stores

21. Which launchers or stores should be considered?

- settingId: `launchers.supportedKinds`
- policyLane: `enforcement`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Launcher proof must not treat launcher-only activity as active gameplay.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: steam | epic-games | xbox-app | microsoft-store | riot-client | battle-net | ea-app | ubisoft-connect | gog-galaxy | roblox | minecraft-launcher | unknown-launcher
- helperText: Launcher activity is not automatically game play; manifests and child-process attribution remain separate.

22. How should launcher-only time be handled?

- settingId: `launchers.launcherOnlyHandling`
- policyLane: `enforcement`; cardKind: `many-option-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Launcher proof must not treat launcher-only activity as active gameplay.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: ignore | report-separately | count-as-possible-game | ask-after-threshold | block
- helperText: Launcher activity is not automatically game play; manifests and child-process attribution remain separate.

23. What if launcher manifests cannot be read?

- settingId: `launchers.whenManifestUnavailable`
- policyLane: `enforcement`; cardKind: `compact-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Launcher proof must not treat launcher-only activity as active gameplay.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: use-process-evidence-only | mark-unavailable | ask | manual-parent-catalog
- helperText: Launcher activity is not automatically game play; manifests and child-process attribution remain separate.

### Browser and cloud games

#### Browser and cloud games

24. How should browser and cloud games be counted?

- settingId: `browserCloud.mode`
- policyLane: `enforcement`; cardKind: `many-option-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Browser-game title proof requires managed browser URL/title evidence or explicit platform integration.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: off | report-only | managed-proof-only | domain-service-hint | count-cloud-client | ask
- helperText: Browser and cloud games keep their surface-specific proof boundary; network hints are not exact title proof.

25. Which evidence may classify browser or cloud game use?

- settingId: `browserCloud.allowedEvidence`
- policyLane: `enforcement`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Browser-game title proof requires managed browser URL/title evidence or explicit platform integration.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: managed-browser-url | managed-browser-title | domain-service-hint | cloud-client-process | network-flow-service-hint | platform-family-activity | parent-catalog
- helperText: Browser and cloud games keep their surface-specific proof boundary; network hints are not exact title proof.

## Tab: audit

### Audit

#### Audit

26. What should game actions audit?

- settingId: `audit.requiredFields`
- policyLane: `audit`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: policy-decision | evidence-ref | ai-ref | adapter-result | timer-state | parent-override | rollback | policy-version | capability-state | custody-label | protected-process-status | target-recheck
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Tab: evidence

### Game session evidence

#### Game session evidence

27. What proof is enough for game rules?

- settingId: `evidence.requiredProof`
- policyLane: `evidence`; cardKind: `many-option-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: process-running | foreground-window | foreground-known-game-session | launcher-attributed-session | package-identity-session | managed-browser-game-url | platform-family-activity | manual-parent-catalog
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

28. Which time should count toward game budgets?

- settingId: `evidence.durationCountingMode`
- policyLane: `evidence`; cardKind: `many-option-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Duration proof requires session id, process/package identity, observation gaps, and evidence refs.
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: foreground-game-time | running-game-process-time | launcher-child-game-time | known-game-only-time | known-and-possible-game-time | browser-managed-game-time | cloud-client-foreground-time | platform-reported-game-time
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

29. What if game proof is unavailable?

- settingId: `evidence.whenProofUnavailable`
- policyLane: `evidence`; cardKind: `many-option-single-choice`; selectionMode: `derived`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: allow | observe | warn | ask | block-until-ready | mark-unavailable
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

30. What must game rules never collect?

- settingId: `evidence.neverCollect`
- policyLane: `evidence`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: screenshots | keystrokes | chat-content | voice-content | game-memory | decrypted-network-payload | launcher-credentials | private-social-graph | raw-anti-cheat-data | purchase-history | cloud-save-content
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Tab: reports

### Reports and retention

#### Reports and retention

31. What should parents see in game reports?

- settingId: `reports.visibleFields`
- policyLane: `reports`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: inventory-status | running-now | foreground-now | recent-sessions | daily-rollups | unknown-candidates | launcher-status | rating-category | approval-events | block-results | time-budget | policy-decisions | source-capability
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

32. How long should raw process/window game evidence be kept?

- settingId: `retention.rawEvidence`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `derived`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: fresh-only | 24-hours | 7-days | 30-days | until-reset | delete-expired
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

33. Where may game data be used?

- settingId: `custody.allowedUses`
- policyLane: `reports`; cardKind: `many-option-multi-choice`; selectionMode: `derived`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/game-control-schema-proposal.md`; sourceLine: n/a
- acceptedOptions: child-local | lan-live | parent-cache | parent-export | parent-report | unavailable
- helperText: Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.

## Capability truth coverage

Total capability truths: 11

1.  Launcher evidence is useful, but it is not automatically game evidence.

- truthId: `game-truth-launcher-not-gameplay`
- capabilityState: `manual-required`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Launcher Or Store
- appliesToSettingIds: `launchers.supportedKinds`, `launchers.launcherOnlyHandling`

2.  Exact browser-game proof requires managed browser URL/tab evidence or another explicit browser boundary.

- truthId: `game-truth-browser-game-boundary`
- capabilityState: `manual-required`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Browser Game
- appliesToSettingIds: `browserCloud.mode`, `browserCloud.allowedEvidence`

3.  Network-only evidence can suggest a service or domain but usually cannot prove exact title.

- truthId: `game-truth-network-not-title-proof`
- capabilityState: `degraded`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Cloud Game
- appliesToSettingIds: `browserCloud.allowedEvidence`, `rules.allowedTargetTypes`

4.  Foreground time and duration proof must come from child-device process/window observations and evidence refs.

- truthId: `game-truth-foreground-duration-proof`
- capabilityState: `available`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Foreground Game Session > Duration Proof
- appliesToSettingIds: `evidence.requiredProof`, `evidence.durationCountingMode`, `budgets.enabled`

5.  Unknown and possible-game states must remain visible instead of being silently promoted to known games.

- truthId: `game-truth-unknown-stays-unknown`
- capabilityState: `degraded`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Game Categories And Age Ratings
- appliesToSettingIds: `inventory.classificationStates`, `rules.allowedTargetTypes`

6.  Ocentra must not try to bypass protected or anti-cheat process controls.

- truthId: `game-truth-anti-cheat-no-bypass`
- capabilityState: `degraded`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Protected Or Anti-Cheat Process
- appliesToSettingIds: `nativeGames.strictActions`, `audit.requiredFields`

7.  Termination requires target recheck, adapter result, timer/approval state, and evidence refs.

- truthId: `game-truth-terminate-recheck-audit`
- capabilityState: `protected`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Enforcement Actions > Terminate
- appliesToSettingIds: `nativeGames.strictActions`, `rules.allowedActions`, `audit.requiredFields`

8.  Parent approval is a typed policy path; unanswered requests follow deterministic child-agent fallback.

- truthId: `game-truth-parent-approval-agent-owned`
- capabilityState: `available`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Parent Approval
- appliesToSettingIds: `approvals.requiredFor`, `approvals.unansweredDefault`, `approvals.allowedParentResponses`

9.  Ocentra-hosted services must not become the default store for raw game activity evidence.

- truthId: `game-truth-local-custody-default`
- capabilityState: `available`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Reports, Custody, Retention, And Audit
- appliesToSettingIds: `reports.visibleFields`, `retention.rawEvidence`, `custody.allowedUses`

10. Mobile game control is platform-dependent and manual-required until device-owner or platform entitlement proof exists.

- truthId: `game-truth-mobile-manual-required`
- capabilityState: `manual-required`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Platform Capability Notes > Android > iOS And iPadOS
- appliesToSettingIds: `game.managementMode`, `browserCloud.allowedEvidence`

11. Console play is external platform family-control state unless Ocentra later builds an approved integration.

- truthId: `game-truth-console-platform-only`
- capabilityState: `manual-required`
- sourceDocument: `docs/game-control-capability-guide.md`
- sourceHeadingPath: Platform Capability Notes > Consoles
- appliesToSettingIds: `game.managementMode`, `rules.allowedActions`

## Capability registry

Total capabilities: 9

1.  `windows-process-observation`

- state: `available`
- proofRequirement: runtime-read-model-required
- affectsSettingIds: `inventory.sources`, `evidence.requiredProof`, `rules.allowedTargetTypes`

2.  `windows-foreground-window-observation`

- state: `available`
- proofRequirement: runtime-read-model-required
- affectsSettingIds: `evidence.requiredProof`, `evidence.durationCountingMode`, `budgets.enabled`

3.  `windows-installed-app-inventory`

- state: `available`
- proofRequirement: runtime-read-model-required
- affectsSettingIds: `inventory.mode`, `inventory.sources`, `nativeGames.allowedIdentityTypes`

4.  `launcher-manifest-reader`

- state: `manual-required`
- proofRequirement: not-yet-proven-for-each-launcher
- affectsSettingIds: `launchers.supportedKinds`, `inventory.sources`, `rules.allowedTargetTypes`

5.  `owned-process-termination`

- state: `available`
- proofRequirement: runtime-adapter-proof-required
- affectsSettingIds: `nativeGames.strictActions`, `rules.allowedActions`

6.  `broad-app-control-blocking`

- state: `manual-required`
- proofRequirement: not-yet-proven
- affectsSettingIds: `nativeGames.strictActions`, `rules.allowedActions`, `game.defaultPosture`

7.  `anti-cheat-protected-process-handling`

- state: `degraded`
- proofRequirement: must-not-bypass-record-limits-only
- affectsSettingIds: `nativeGames.strictActions`, `audit.requiredFields`

8.  `managed-browser-game-proof`

- state: `manual-required`
- proofRequirement: browser-control-boundary-required
- affectsSettingIds: `browserCloud.mode`, `browserCloud.allowedEvidence`, `rules.allowedTargetTypes`

9.  `platform-family-controls`

- state: `manual-required`
- proofRequirement: platform-integration-required
- affectsSettingIds: `game.managementMode`, `browserCloud.allowedEvidence`, `rules.allowedActions`
