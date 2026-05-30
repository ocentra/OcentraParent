# Screen Control Settings Inventory

Generated from `BaselineScreenControlCatalog`.
Total settings: 474

Use this as the raw review list for deciding parent-facing grouping.

## Tab: evidence

### Core Terms

#### Screen Evidence

1.  parent-enabled setting;

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0001`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 26
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

2.  local capture of the approved scope;

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0003`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 28
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

3.  deletion of the raw image or frame data.

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0007`
- policyLane: `evidence`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 33
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Screenshot

4.  full screen or display;

- settingId: `screen-capability-guide-bullet-core-terms-screenshot-0008`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 41
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

5.  active window;

- settingId: `screen-capability-guide-bullet-core-terms-screenshot-0009`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 42
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

6.  managed browser window;

- settingId: `screen-capability-guide-bullet-core-terms-screenshot-0010`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 43
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

7.  selected app window;

- settingId: `screen-capability-guide-bullet-core-terms-screenshot-0011`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 44
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Evidence Reference

8.  image digest;

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0019`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 96
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

9.  deletion state;

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0020`
- policyLane: `evidence`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 97
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

10. foreground app/window evidence;

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0021`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 98
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

11. app/game session evidence;

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0023`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 100
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

12. network digest evidence;

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0024`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 101
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

13. parent setting version;

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0026`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 103
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Managed Browser Or Window Capture

#### Managed Browser Or Window Capture

14. capture only the Ocentra-managed browser window;

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0070`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 214
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

15. capture only an approved app window;

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0071`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 215
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

16. correlate capture with managed session id, window id, process id, and evidence refs.

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0073`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 217
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

17. a managed browser screenshot is still not the source of exact URL truth;

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0074`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: managed-browser-evidence-required
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 221
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

18. window title and visible pixels can be stale or misleading;

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0077`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 225
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### App And Window Correlation

#### App And Window Correlation

19. foreground process/window evidence;

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0191`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 480
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

20. managed browser session and tab evidence;

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0192`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 481
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

21. app/game session summary;

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0193`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 482
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

22. network flow digest;

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0194`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 483
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

23. app duration from repeated screenshots alone;

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0198`
- policyLane: `evidence`; cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 493
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

24. network destination from visible content alone;

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0199`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 494
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

25. child intent from category alone.

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0200`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 495
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Proposal Overview

#### Proposal Overview

26. Portal-rendered question/option UI.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0276`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 7
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

27. Local screenshot or frame capture only after explicit enablement.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0280`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 11
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

28. Small patch updates from Portal.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0284`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 15
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

29. Deterministic compile into an effective local execution plan.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0286`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 17
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

30. Effect Schema validation.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0287`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 23
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

31. Branded ids from schema brands, not manual brands.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0288`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 24
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

32. Decode helpers.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0289`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 25
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

33. Rust protocol parity only after the TypeScript contracts are explicit and test-backed.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0292`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 29
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Implementation Notes For Worker

#### Implementation Notes For Worker

34. Do not let Portal define arbitrary JSON paths. `writesTo` paths should be schema-known authoring paths.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0296`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1677
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

35. Add Rust parity only for Rust-crossing commands/events after TypeScript contracts and tests are stable.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0306`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1692
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Authoring Manifest - Screen recording

#### Screen recording fields

36. Should screen recording be allowed?

- settingId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields-0316`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 312
- acceptedOptions: Disabled | Manual Parent Test Only | Short Local Buffer | Triggered Frame Sampling | Authoring Only Manual Required | Default disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

37. What is the maximum local recording segment length?

- settingId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields-0317`
- policyLane: `evidence`; cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 326
- acceptedOptions: Default 15 | Minimum 1 | Maximum 60
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

38. Allow continuous screen recording?

- settingId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields-0319`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `os-adapter`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 347
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Scheduling

#### Scheduling fields

39. When should capture pause?

- settingId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields-0325`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 412
- acceptedOptions: Screen Locked | Protected Surface | Permission Required | Permission Limited | Queue Unavailable | Model Unavailable | Battery Saver | Metered Connection | Parent Paused | Default screen-locked | Default protected-surface | Default permission-required | Default queue-unavailable | Default model-unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest Metadata

#### Control Kinds

40. Control kind: boolean.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0363`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 119
- acceptedOptions: Boolean
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

41. Control kind: single-choice.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0364`
- policyLane: `evidence`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 120
- acceptedOptions: Single Choice
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

42. Control kind: multi-choice.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0365`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 121
- acceptedOptions: Multi Choice
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

43. Control kind: number.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0366`
- policyLane: `evidence`; cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 122
- acceptedOptions: Number
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

44. Control kind: duration.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0367`
- policyLane: `evidence`; cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 123
- acceptedOptions: Duration
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

45. Control kind: threshold.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0372`
- policyLane: `evidence`; cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 128
- acceptedOptions: Threshold
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

46. Control kind: read-only-status.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0373`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 129
- acceptedOptions: Read Only Status
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

#### Condition Kinds

47. Condition kind: equals.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0374`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 132
- acceptedOptions: Equals
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

48. Condition kind: notEquals.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0375`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 133
- acceptedOptions: NotEquals
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

49. Condition kind: includes.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0376`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 134
- acceptedOptions: Includes
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

50. Condition kind: notIncludes.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0377`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 135
- acceptedOptions: NotIncludes
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

51. Condition kind: all.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0378`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 136
- acceptedOptions: All
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

52. Condition kind: any.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0379`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 137
- acceptedOptions: Any
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

## Tab: rules

### Screenshot Possibilities And Limits

#### What Is Not Reliable

53. parent policy outcome without a typed parent rule.

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0058`
- policyLane: `rules`; cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 183
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Managed Browser Or Window Capture

#### Managed Browser Or Window Capture

54. exact URL and title require browser evidence from CDP, extension, browser policy, or another approved browser integration;

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0075`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: managed-browser-evidence-required
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 222
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### App And Window Correlation

#### App And Window Correlation

55. parent rule and setting version.

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0196`
- policyLane: `rules`; cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 485
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Future UI Rules

#### Future UI Rules

56. show policy use as observe-only, dry-run, enforcement-eligible, or disabled;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0251`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 591
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

57. local observe-only summaries;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0255`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 598
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

58. local policy dry-run;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0256`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 599
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

59. manual parent test capture;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0258`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 601
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Proposal Overview

#### Proposal Overview

60. Child-agent local persisted policy.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0278`
- policyLane: `rules`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 9
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

61. Offline operation from the last valid policy.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0279`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 10
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Implementation Notes For Worker

#### Implementation Notes For Worker

62. Use Effect Schema to validate the full policy after every patch.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0297`
- policyLane: `rules`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1678
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

63. Compile the effective policy in the child-agent/service boundary, not in Portal.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0298`
- policyLane: `rules`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1679
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

64. Add explicit tests for offline behavior: child agent continues using the last valid compiled policy when Portal is disconnected.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0304`
- policyLane: `rules`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1688
- acceptedOptions: Child Agent Continues Using The Last Valid Compiled Policy When Portal Is Disconnected
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Authoring Manifest - Policy use

#### Policy use fields

65. Allow screen summaries to be used by policy?

- settingId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields-0346`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 717
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

66. What screen-derived targets may policy match?

- settingId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields-0347`
- policyLane: `rules`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 724
- acceptedOptions: Visible Category | Risk Signal | Ocr Snippet Presence | Unknown State | Protected Surface | Capability State | Default visible-category | Default risk-signal | Default unknown-state
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

67. Which evidence refs are required before policy use?

- settingId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields-0349`
- policyLane: `rules`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 751
- acceptedOptions: Screen Summary | Queue Deletion State | Local Model Runtime | Foreground App Window | Managed Browser State | App Game Session | Network Digest | Parent Setting Version | Policy Version | Default screen-summary | Default queue-deletion-state | Default local-model-runtime | Default parent-setting-version
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest Metadata

#### Rendering Rules

68. Rendering rule hideInvisibleFields: true.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0355`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 109
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

69. Rendering rule showDisabledFieldsWithReason: true.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0356`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 110
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

70. Rendering rule neverInventFieldsOutsideManifest: true.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0357`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 111
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

71. Rendering rule writeOnlyThroughWritesToPath: true.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0358`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 112
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

72. Rendering rule previewBeforeApply: true.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0359`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 113
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

73. Rendering rule showCapabilityStateBesideSensitiveControls: true.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0361`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 115
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

#### Control Kinds

74. Control kind: target-list.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0370`
- policyLane: `rules`; cardKind: `target-list-card`; selectionMode: `multi`; controlKind: `target-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 126
- acceptedOptions: Target List
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

75. Control kind: rule-list.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0371`
- policyLane: `rules`; cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 127
- acceptedOptions: Rule List
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Update Protocol

#### Commands

76. screen-policy.preview.requested: Portal asks whether proposed changes validate and what effective policy would result.

- settingId: `screen-update-command-update-protocol-commands-0410`
- policyLane: `rules`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1434
- acceptedOptions: Screen Policy.preview.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Agent Rules

77. Agent rule validateFullPolicyAfterPatch: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0415`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1533
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

78. Agent rule compileFullEffectivePolicyAfterEveryAcceptedChange: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0416`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1534
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

79. Agent rule runCaptureOnlyInChildAgent: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0420`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1538
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

80. Agent rule enforceLocallyWhenPortalOffline: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0422`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1540
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

81. Agent rule rejectUnknownPaths: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0423`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1541
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

82. Agent rule rejectInvalidEnumValues: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0424`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1542
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

83. Agent rule rejectHostedProcessingForSchemaVersionOne: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0427`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1545
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

84. Agent rule recordSkippedAttemptsAsCapabilityEvents: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0431`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1549
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Effective Policy Document

#### Visible Category Policy Targets

85. Visible category target school: default action observe.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0467`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1375
- acceptedOptions: Observe
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

86. Visible category target video: default action observe.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0468`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1378
- acceptedOptions: Observe
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

87. Visible category target chat: default action observe.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0469`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1381
- acceptedOptions: Observe
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

88. Visible category target game: default action observe.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0470`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1384
- acceptedOptions: Observe
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

89. Visible category target adult-content: default action ask.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0471`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1387
- acceptedOptions: Ask
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

90. Visible category target violence: default action ask.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0472`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1390
- acceptedOptions: Ask
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

91. Visible category target bypass-tool: default action warn.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0473`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1393
- acceptedOptions: Warn
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

92. Visible category target unknown: default action ask.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0474`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1396
- acceptedOptions: Ask
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

## Tab: schedule

### Core Terms

#### Evidence Reference

93. local model/runtime status;

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0025`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 102
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### The Main Capability Truth

#### The Main Capability Truth

94. the parent explicitly enabled screen analysis for the child/device/schedule;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0028`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 116
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Screenshot Possibilities And Limits

#### What Is Possible

95. one-time manual parent test capture during setup;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0047`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 166
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

96. cadence capture with conservative intervals;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0048`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 167
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

97. event-triggered capture after foreground app change, managed URL change, app/game foreground start, unusual network digest, policy ambiguity, or local AI uncertainty;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0049`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 168
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Screen Recording Possibilities And Limits

#### Screen Recording Possibilities And Limits

98. event-triggered frame sampling for transitions;

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0061`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 194
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

99. bandwidth, CPU/GPU, battery, and model runtime load are higher;

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0066`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 202
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### OCR And Image Classification

#### OCR And Image Classification

100.  model/runtime ref and prompt/template version;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0087`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 246
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Triggers And Scheduling

#### Triggers And Scheduling

101.  disabled by default;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0092`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `disabled`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 277
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

102.  conservative interval such as several minutes;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0093`
- policyLane: `schedule`; cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 278
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

103.  stricter shorter interval only when explicitly enabled;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0094`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 279
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

104.  schedule-aware capture windows;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0095`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 280
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

105.  pause during sleep, lock, protected surface, permission-required state, or battery/resource pressure.

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0096`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 281
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

106.  foreground app change;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0097`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 285
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

107.  active window change;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0098`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 286
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

108.  managed browser URL change;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0099`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 287
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

109.  app/game foreground start;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0100`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 288
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

110.  unusual network digest;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0101`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 289
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

111.  policy ambiguity;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0102`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 290
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

112.  local AI uncertainty;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0103`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 291
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

113.  child ask-parent flow;

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0104`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 292
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

114.  manual parent setup/test capture.

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0105`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 293
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Platform Capability Notes

#### Windows

115.  capture support must be checked at runtime;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0149`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 383
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### iOS And iPadOS

116.  Screen Time frameworks: Family Controls, Managed Settings, Device Activity;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0183`
- policyLane: `schedule`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 463
- acceptedOptions: Family Controls | Managed Settings | Device Activity
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

117.  Managed Settings shields and Device Activity schedules/events;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0185`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 465
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

118.  web-domain and app/category usage controls through Screen Time tokens.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0186`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 466
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

119.  Screen Time APIs are privacy-preserving and entitlement/review-gated;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0189`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 472
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

120.  iOS child-device support should rely on approved Screen Time/Device Activity/Managed Settings paths rather than desktop-style pixel capture unless a specific Apple-approved capability is proven.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0190`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 473
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### App And Window Correlation

#### App And Window Correlation

121.  local model/runtime status;

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0195`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 484
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Child-Facing Disclosure

#### Child-Facing Disclosure

122.  reason text for warnings, asks, blocks, or time limits;

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0205`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 507
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Parent Reports

#### Parent Reports

123.  local model/runtime status;

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0213`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 523
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Custody And Audit

#### Custody And Audit

124.  capability state at capture time;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0219`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 539
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

125.  local model/runtime ref;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0223`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 543
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Future UI Rules

#### Future UI Rules

126.  show cadence and triggers separately;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0246`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 584
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

127.  show local model/runtime status;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0248`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `local-ai-runtime`; capabilityState: `future-gap`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 586
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

128.  trigger-only capture;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0259`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 602
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

129.  cadence plus trigger capture;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0260`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 603
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Capability Matrix

#### Prove duration

130.  Capability Prove duration: full screen No, single point in time; active window No, single point in time; managed browser/window No, single point in time; local OCR/vision No; important limit Duration belongs to app/game/window/session evidence or recording-specific proof..

- settingId: `screen-capability-matrix-row-capability-matrix-prove-duration-0269`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 145
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Proposal Overview

#### Proposal Overview

131.  No naked domain strings in app/runtime code.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0290`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 26
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Implementation Notes For Worker

#### Implementation Notes For Worker

132.  Keep authoring manifest ids, field ids, section ids, option ids, policy ids, rule ids, schedule ids, trigger ids, queue job ids, result ids, capability ids, custody labels, and evidence refs branded.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0295`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1675
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

133.  Treat the authoring manifest as UI guidance only. Runtime capture, queue, analysis, policy, and enforcement must rely on validated policy and compiled effective policy.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0301`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1683
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

134.  Add explicit tests for hidden/visible branch behavior so UI cannot show cadence, OCR snippet storage, strict mode, or enforcement eligibility controls when screen analysis is disabled.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0303`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1686
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Scheduling

#### Scheduling fields

135.  Enable scheduled capture?

- settingId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields-0321`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 374
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

136.  How often may scheduled capture run?

- settingId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields-0322`
- policyLane: `schedule`; cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 381
- acceptedOptions: Default 300 | Minimum 60 | Maximum 3600
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

137.  Allow the shortest supported cadence?

- settingId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields-0323`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 394
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Authoring Manifest - Triggers

#### Triggers fields

138.  Enable event-triggered capture?

- settingId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields-0326`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 447
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

139.  Which events may request screen analysis?

- settingId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields-0327`
- policyLane: `schedule`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 454
- acceptedOptions: Foreground App Change | Active Window Change | Managed Browser Url Change | App Game Foreground Start | Unusual Network Digest | Policy Ambiguity | Local Ai Uncertainty | Ask Parent Flow | Manual Parent Test Capture | Default foreground-app-change | Default managed-browser-url-change | Default policy-ambiguity | Default manual-parent-test-capture
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

140.  How long should repeated triggers wait?

- settingId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields-0328`
- policyLane: `schedule`; cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 481
- acceptedOptions: Default 120 | Minimum 15 | Maximum 900
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

141.  What is the maximum number of screen analysis jobs per hour?

- settingId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields-0329`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 494
- acceptedOptions: Default 12 | Minimum 0 | Maximum 60
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest Metadata

#### Control Kinds

142.  Control kind: schedule.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0368`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 124
- acceptedOptions: Schedule
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Capability Registry

#### capture-scope

143.  Capability windows-graphics-capture-full-screen: kind capture-scope; state manual-required; proof real-host-permission-and-capture-proof-required; affects fields capture.allowedScopes, schedule.cadenceCaptureEnabled.

- settingId: `screen-capability-registry-entry-capability-registry-capture-scope-0385`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1565
- acceptedOptions: Manual Required | Real Host Permission And Capture Proof Required | Capture.allowedScopes | Schedule.cadenceCaptureEnabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### source-correlation

144.  Capability managed-browser-window-correlation: kind source-correlation; state ready; proof runtime-read-model-required; affects fields capture.requireManagedBrowserCorrelationForWebClaims, policy.requireEvidenceRefs.

- settingId: `screen-capability-registry-entry-capability-registry-source-correlation-0387`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1579
- acceptedOptions: Ready | Runtime Read Model Required | Capture.requireManagedBrowserCorrelationForWebClaims | Policy.requireEvidenceRefs
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

145.  Capability foreground-app-window-correlation: kind source-correlation; state ready; proof runtime-read-model-required; affects fields capture.requireAppWindowCorrelation, policy.requireEvidenceRefs.

- settingId: `screen-capability-registry-entry-capability-registry-source-correlation-0388`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1586
- acceptedOptions: Ready | Runtime Read Model Required | Capture.requireAppWindowCorrelation | Policy.requireEvidenceRefs
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### local-analysis

146.  Capability local-ocr-runtime: kind local-analysis; state manual-required; proof local-model-runtime-proof-required; affects fields analysis.allowedTasks, analysis.ocrTextEnabled, analysis.minimumPolicyConfidence.

- settingId: `screen-capability-registry-entry-capability-registry-local-analysis-0390`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1605
- acceptedOptions: Manual Required | Local Model Runtime Proof Required | Analysis.allowedTasks | Analysis.ocrTextEnabled | Analysis.minimumPolicyConfidence
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

147.  Capability local-vision-classifier: kind local-analysis; state manual-required; proof local-model-runtime-proof-required; affects fields analysis.allowedTasks, analysis.minimumPolicyConfidence, policy.allowedTargetTypes.

- settingId: `screen-capability-registry-entry-capability-registry-local-analysis-0391`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1612
- acceptedOptions: Manual Required | Local Model Runtime Proof Required | Analysis.allowedTasks | Analysis.minimumPolicyConfidence | Policy.allowedTargetTypes
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### redaction

148.  Capability screen-redaction-runtime: kind redaction; state manual-required; proof redaction-validation-required; affects fields redaction.mode, redaction.neverStore, redaction.whenUnavailable.

- settingId: `screen-capability-registry-entry-capability-registry-redaction-0392`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1619
- acceptedOptions: Manual Required | Redaction Validation Required | Redaction.mode | Redaction.neverStore | Redaction.whenUnavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### platform-policy

149.  Capability ios-screentime-managed-settings: kind platform-policy; state manual-required; proof apple-entitlement-and-device-proof-required; affects fields policy.allowedTargetTypes, reports.visibleFields.

- settingId: `screen-capability-registry-entry-capability-registry-platform-policy-0396`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1647
- acceptedOptions: Manual Required | Apple Entitlement And Device Proof Required | Policy.allowedTargetTypes | Reports.visibleFields
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Capability State Meanings

150.  ready: Runtime reports the capability can be used within the configured boundary, subject to per-attempt checks.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0397`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1655
- acceptedOptions: Ready
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

151.  model-unavailable: Local OCR/vision runtime is missing, disabled, loading, failed, or overloaded.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0404`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1662
- acceptedOptions: Model Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Update Protocol

#### Agent Rules

152.  Agent rule persistPolicyBeforeSchedulerSwitch: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0417`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1535
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

## Tab: approvals

### Policy Value Document

#### Fallbacks

153.  Fallback lowConfidence: ask-parent.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0442`
- policyLane: `approvals`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1212
- acceptedOptions: Ask Parent
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

154.  Fallback policyUse.lowConfidenceFallback: ask-parent.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0446`
- policyLane: `approvals`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1020
- acceptedOptions: Ask Parent
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

## Tab: enforcement

### Screenshot Possibilities And Limits

#### What Is Possible

155.  risk signals such as possible credential prompt, explicit content signal, bypass tool, unsafe visible content, self-harm signal, suspicious login, or unknown;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0045`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 162
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

156.  correlation with foreground app, active window title, managed browser state, app/game session, and network digest refs;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0046`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 164
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### What Is Not Reliable

157.  exact active browser URL unless managed browser evidence proves it;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0051`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 176
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

158.  what the child typed before or after the frame;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0052`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 177
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

159.  duration of use;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0053`
- policyLane: `enforcement`; cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 178
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

160.  whether visible text came from a webpage, chat, image, ad, overlay, or stale window;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0054`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 179
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

161.  hidden background tabs or background apps;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0055`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 180
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

162.  decrypted network content;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0056`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 181
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

163.  app identity without OS process/window correlation;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0057`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 182
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Screen Recording Possibilities And Limits

#### Screen Recording Possibilities And Limits

164.  recording creates more raw sensitive data than screenshots;

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0063`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 199
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

## Tab: audit

### Core Terms

#### Evidence Reference

165.  policy or AI decision that consumed the summary.

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0027`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 104
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### The Main Capability Truth

#### The Main Capability Truth

166.  platform parity before real OS/device proof;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0041`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 132
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Screenshot Possibilities And Limits

#### What Is Possible

167.  queue lifecycle and deletion proof.

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0050`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 170
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Platform Capability Notes

#### Windows

168.  product claims should follow real host proof, not contract presence.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0153`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 389
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### macOS

169.  macOS parity requires real host proof, not package scaffold proof.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0161`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 409
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

#### Linux

170.  foreground-window proof varies by compositor and desktop environment;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0170`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 430
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

171.  Linux support needs distro/backend-specific proof.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0171`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 431
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Parent Reports

#### Parent Reports

172.  policy/AI decisions that consumed the screen summary;

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0216`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 526
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Custody And Audit

#### Custody And Audit

173.  parent setting version and actor ref;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0218`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 538
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

174.  capture reason and scope;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0220`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 540
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

175.  queue job id and image digest;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0221`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 541
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

176.  encryption and deletion lifecycle;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0222`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 542
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

177.  validation result;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0224`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 544
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

178.  summary/result id;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0225`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 545
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

179.  policy decision id;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0226`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 546
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

180.  enforcement result if any;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0227`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 547
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

181.  custody label;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0228`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 548
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

182.  retention/deletion state;

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0229`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 549
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

183.  adapter errors or permission changes.

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0230`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 550
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Proof Requirements

#### Proof Requirements

184.  parent setting enabled through typed contracts;

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0231`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 559
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

185.  child-device agent or service detects capability and permission state;

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0232`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 560
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

186.  capture occurs only inside approved scope;

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0233`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 561
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

187.  image/frame enters encrypted temp queue;

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0234`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 562
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

188.  local OCR/vision analyzes it;

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0235`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 563
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

189.  schema validation accepts/rejects output correctly;

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0236`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 564
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

190.  raw image deletes after success or TTL expiry;

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0237`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 565
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

191.  journal and SQLite expose summary/read model;

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0238`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 566
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

192.  portal renders settings, status, summary, refs, custody, and deletion state;

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0239`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 567
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

193.  no Ocentra-hosted upload happens by default;

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0240`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 568
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

194.  protected/permission-required cases are visible as unavailable, not fake success.

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0241`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 569
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Future UI Rules

#### Future UI Rules

195.  show exact proof requirement before screen-derived rules can enforce;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0252`
- policyLane: `audit`; cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 592
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Capability Matrix

#### Capture a still image

196.  Capability Capture a still image: full screen Possible on desktop platforms with permission/proof; active window Possible where OS exposes window capture; managed browser/window Possible if the managed boundary is active; local OCR/vision Input only after capture; important limit Must skip protected/locked/permission-required states..

- settingId: `screen-capability-matrix-row-capability-matrix-capture-a-still-image-0263`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 139
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Capture a recording stream

197.  Capability Capture a recording stream: full screen Possible but high-sensitivity; active window Possible where OS supports selected window/app stream; managed browser/window Possible if managed scope is selected; local OCR/vision Usually sampled into frames or summaries; important limit Not default; needs stronger opt-in and proof..

- settingId: `screen-capability-matrix-row-capability-matrix-capture-a-recording-stream-0264`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 140
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Prove exact URL

198.  Capability Prove exact URL: full screen No; active window No; managed browser/window Only if browser evidence proves it separately; local OCR/vision No; important limit Pixels can show text that looks like a URL, but that is not managed tab proof..

- settingId: `screen-capability-matrix-row-capability-matrix-prove-exact-url-0267`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: managed-browser-evidence-required
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 143
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Feed enforcement

199.  Capability Feed enforcement: full screen Not directly; active window Not directly; managed browser/window Not directly; local OCR/vision Not directly; important limit Enforcement requires typed policy decision and audit..

- settingId: `screen-capability-matrix-row-capability-matrix-feed-enforcement-0273`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 149
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Proposal Overview

#### Proposal Overview

200.  Local child-agent persistence, queue encryption/deletion, compile, rollback, and audit behavior.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0293`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 30
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Implementation Notes For Worker

#### Implementation Notes For Worker

201.  Reject partial states. For example, `policyUse.enabled: true` requires a valid confidence threshold, deletion proof requirement, evidence refs, and fallback behavior.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0300`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1681
- acceptedOptions: True` Requires A Valid Confidence Threshold | Deletion Proof Requirement | Evidence Refs | And Fallback Behavior
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Policy use

#### Policy use fields

202.  What if screen proof is unavailable?

- settingId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields-0348`
- policyLane: `audit`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 743
- acceptedOptions: Allow | Observe | Warn | Ask | Block Until Ready | Mark Unavailable | Default mark-unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Audit

#### Audit fields

203.  Which audit fields are required?

- settingId: `screen-authoring-field-authoring-manifest-audit-audit-fields-0352`
- policyLane: `audit`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 834
- acceptedOptions: Parent Setting Version | Capability State | Capture Reason | Capture Scope | Queue Job Id | Image Digest | Local Model Runtime | Validation Result | Deletion State | Custody Label | Policy Decision Ref | Enforcement Result Ref | Adapter Error | Permission State | Default parent-setting-version | Default capability-state | Default capture-reason | Default queue-job-id | Default image-digest | Default local-model-runtime | Default validation-result | Default deletion-state | Default custody-label | Default policy-decision-ref
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

204.  Audit every capture attempt, including skipped attempts?

- settingId: `screen-authoring-field-authoring-manifest-audit-audit-fields-0353`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 868
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

205.  Audit every delete-pending or delete-failed state?

- settingId: `screen-authoring-field-authoring-manifest-audit-audit-fields-0354`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 875
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest Metadata

#### Condition Kinds

206.  Condition kind: proofAtLeast.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0382`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 140
- acceptedOptions: ProofAtLeast
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Capability Registry

#### capture-scope

207.  Capability windows-graphics-capture-active-window: kind capture-scope; state manual-required; proof real-host-permission-and-capture-proof-required; affects fields capture.allowedScopes, capture.defaultScope, policy.allowedTargetTypes.

- settingId: `screen-capability-registry-entry-capability-registry-capture-scope-0384`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1558
- acceptedOptions: Manual Required | Real Host Permission And Capture Proof Required | Capture.allowedScopes | Capture.defaultScope | Policy.allowedTargetTypes
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### recording

208.  Capability windows-graphics-capture-recording-stream: kind recording; state manual-required; proof real-host-recording-stream-retention-and-deletion-proof-required; affects fields recording.mode, recording.maxSegmentSeconds, recording.frameSamplingMode.

- settingId: `screen-capability-registry-entry-capability-registry-recording-0386`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1572
- acceptedOptions: Manual Required | Real Host Recording Stream Retention And Deletion Proof Required | Recording.mode | Recording.maxSegmentSeconds | Recording.frameSamplingMode
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### queue

209.  Capability encrypted-screen-temp-queue: kind queue; state ready; proof queue-encryption-deletion-tests-required; affects fields queue.temporaryImageTtlSeconds, queue.maxRetryCount, queue.deleteAfterSuccess, queue.deleteAfterExpiry.

- settingId: `screen-capability-registry-entry-capability-registry-queue-0389`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1593
- acceptedOptions: Ready | Queue Encryption Deletion Tests Required | Queue.temporaryImageTtlSeconds | Queue.maxRetryCount | Queue.deleteAfterSuccess | Queue.deleteAfterExpiry
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### platform-capture

210.  Capability macos-screencapturekit: kind platform-capture; state manual-required; proof macos-host-screen-recording-permission-proof-required; affects fields capture.allowedScopes.

- settingId: `screen-capability-registry-entry-capability-registry-platform-capture-0393`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1626
- acceptedOptions: Manual Required | Macos Host Screen Recording Permission Proof Required | Capture.allowedScopes
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

211.  Capability linux-xdg-desktop-portal-screencast: kind platform-capture; state manual-required; proof distro-desktop-portal-pipewire-proof-required; affects fields capture.allowedScopes.

- settingId: `screen-capability-registry-entry-capability-registry-platform-capture-0394`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1633
- acceptedOptions: Manual Required | Distro Desktop Portal Pipewire Proof Required | Capture.allowedScopes
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

212.  Capability android-media-projection: kind platform-capture; state manual-required; proof android-user-consent-foreground-service-proof-required; affects fields capture.allowedScopes, screen.requiredDisclosure, recording.mode.

- settingId: `screen-capability-registry-entry-capability-registry-platform-capture-0395`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1640
- acceptedOptions: Manual Required | Android User Consent Foreground Service Proof Required | Capture.allowedScopes | Screen.requiredDisclosure | Recording.mode
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Capability State Meanings

213.  adapter-error: The platform adapter failed and must record an audit result.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0407`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1665
- acceptedOptions: Adapter Error
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

214.  manual-required: Contracts can represent the setting, but product support requires real host/device proof.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0408`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1666
- acceptedOptions: Manual Required
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Update Protocol

#### Agent Rules

215.  Agent rule rejectPolicyUseWithoutDeletionProof: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0428`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1546
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Policy Value Document

#### Fallbacks

216.  Fallback protectedSurface: skip-and-audit.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0436`
- policyLane: `audit`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1206
- acceptedOptions: Skip And Audit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

217.  Fallback screenLocked: skip-and-audit.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0437`
- policyLane: `audit`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1207
- acceptedOptions: Skip And Audit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Effective Policy Document

#### Proof Requirements

218.  Proof requirement screenPolicyUse: validated-screen-summary-with-deleted-image.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0450`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1318
- acceptedOptions: Validated Screen Summary With Deleted Image
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

219.  Proof requirement exactWebClaims: managed-browser-evidence-required.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0451`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1319
- acceptedOptions: Managed Browser Evidence Required
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

220.  Proof requirement appWindowClaims: foreground-app-window-evidence-required.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0452`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1320
- acceptedOptions: Foreground App Window Evidence Required
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

221.  Proof requirement riskSignalRules: validated-screen-summary-confidence-threshold.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0453`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1321
- acceptedOptions: Validated Screen Summary Confidence Threshold
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

222.  Proof requirement enforcementEligibility: typed-policy-decision-with-screen-evidence-ref.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0454`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1322
- acceptedOptions: Typed Policy Decision With Screen Evidence Ref
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

223.  Proof requirement reportOnly: stale-or-degraded-allowed.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0455`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1323
- acceptedOptions: Stale Or Degraded Allowed
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

#### Fallback Decisions

224.  Effective fallback proofUnavailable: mark-unavailable.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0456`
- policyLane: `audit`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1326
- acceptedOptions: Mark Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

225.  Effective fallback staleEvidence: report-only.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0457`
- policyLane: `audit`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1327
- acceptedOptions: Report Only
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

226.  Effective fallback screenLocked: skip-and-audit.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0458`
- policyLane: `audit`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1207
- acceptedOptions: Skip And Audit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

227.  Effective fallback protectedSurface: skip-and-audit.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0459`
- policyLane: `audit`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1206
- acceptedOptions: Skip And Audit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

228.  Effective fallback modelUnavailable: retry-within-ttl-then-delete.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0460`
- policyLane: `audit`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1209
- acceptedOptions: Retry Within Ttl Then Delete
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

229.  Effective fallback queueUnavailable: fail-closed.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0461`
- policyLane: `audit`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1208
- acceptedOptions: Fail Closed
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

230.  Effective fallback adapterError: mark-degraded-and-audit.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0462`
- policyLane: `audit`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1332
- acceptedOptions: Mark Degraded And Audit
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

231.  Effective fallback deleteFailed: surface-health-and-retry.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0463`
- policyLane: `audit`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1213
- acceptedOptions: Surface Health And Retry
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Rules In Priority Order

232.  Effective rule parent-request-explicit-content-signal: priority 100; decision ask; target risk-signal; proof validated-screen-summary-with-deleted-image; minimum confidence 0.8.

- settingId: `screen-effective-rule-effective-policy-document-rules-in-priority-order-0464`
- policyLane: `audit`; cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1026
- acceptedOptions: Ask | Risk Signal | Validated Screen Summary With Deleted Image
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

233.  Effective rule warn-bypass-tool-visible: priority 200; decision warn; target visible-category; proof validated-screen-summary-with-source-correlation; minimum confidence 0.75.

- settingId: `screen-effective-rule-effective-policy-document-rules-in-priority-order-0465`
- policyLane: `audit`; cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1045
- acceptedOptions: Warn | Visible Category | Validated Screen Summary With Source Correlation
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

234.  Effective rule unknown-screen-state-ask: priority 900; decision ask; target unknown-state; proof screen-analysis-attempted; minimum confidence 0.

- settingId: `screen-effective-rule-effective-policy-document-rules-in-priority-order-0466`
- policyLane: `audit`; cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1063
- acceptedOptions: Ask | Unknown State | Screen Analysis Attempted
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

## Tab: reports

### Screen Recording Possibilities And Limits

#### Screen Recording Possibilities And Limits

235.  child-facing disclosure must be clearer;

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0065`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 201
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### OCR And Image Classification

#### OCR And Image Classification

236.  local redaction before journal/report storage;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0081`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 237
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Retention And Custody

#### Retention And Custody

237.  parent report: summary, refs, confidence, custody label, and deletion state;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0115`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `multi`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 323
- acceptedOptions: Summary | Refs | Confidence | Custody Label | And Deletion State
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Platform Capability Notes

#### Android

238.  Play policy and user disclosure constraints matter.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0182`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `portal-only`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 455
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Child-Facing Disclosure

#### Child-Facing Disclosure

239.  parent setting state visible in the parent portal;

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0201`
- policyLane: `reports`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 503
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

240.  child-facing disclosure that screen analysis may run locally;

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0202`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 504
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

241.  clear difference between observe-only, dry-run, and enforcement-eligible modes;

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0203`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 505
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

242.  current permission-required or disabled state;

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0204`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 506
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

243.  no hidden background capture claims.

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0206`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 508
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Parent Reports

#### Parent Reports

244.  setting state and who changed it;

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0207`
- policyLane: `reports`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 517
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

245.  capture reason and scope;

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0208`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 518
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

246.  category candidates and confidence;

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0209`
- policyLane: `reports`; cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 519
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

247.  risk signals and confidence;

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0210`
- policyLane: `reports`; cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 520
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

248.  bounded OCR snippets only when enabled and redacted;

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0211`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 521
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

249.  source evidence refs;

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0212`
- policyLane: `reports`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 522
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

250.  custody/source label;

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0214`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 524
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

251.  deletion state and image digest;

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0215`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 525
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

252.  unavailable, protected, permission-required, low-confidence, stale, expired, invalid, delete-pending, or delete-failed states.

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0217`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `protected`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 527
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Future UI Rules

#### Future UI Rules

253.  keep parent reports evidence-cited and custody-labeled.

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0253`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 593
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Capability Matrix

#### Show parent report

254.  Capability Show parent report: full screen Summary, confidence, refs, deletion state; active window Same; managed browser/window Same; local OCR/vision Same; important limit Raw screenshot hidden by default..

- settingId: `screen-capability-matrix-row-capability-matrix-show-parent-report-0274`
- policyLane: `reports`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `unavailable`; runtimeOwner: `portal-only`; capabilityState: `unavailable`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 150
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Screen analysis

#### Screen analysis fields

255.  Which disclosure requirements apply before capture?

- settingId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields-0310`
- policyLane: `reports`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 219
- acceptedOptions: Parent Setting Visible | Child Facing Local Analysis Disclosure | Capture Indicator When Platform Provides It | Raw Capture Not Retained By Default | Cloud Processing Disabled By Default | Report Custody Labels Visible | Default parent-setting-visible | Default child-facing-local-analysis-disclosure | Default raw-capture-not-retained-by-default | Default cloud-processing-disabled-by-default
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Redaction

#### Redaction fields

256.  What must never be stored in summaries or reports?

- settingId: `screen-authoring-field-authoring-manifest-redaction-redaction-fields-0338`
- policyLane: `reports`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 605
- acceptedOptions: Passwords | Tokens | Payment Data | Private Keys | Recovery Codes | Raw Image Bytes | Raw Local Paths | Browser Secrets | Cookies | Keystrokes | Decrypted Payloads | Microphone Audio | Camera Video | Default passwords | Default tokens | Default payment-data | Default private-keys | Default recovery-codes | Default raw-image-bytes | Default raw-local-paths | Default browser-secrets | Default decrypted-payloads
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Reports

#### Reports fields

257.  Which fields should parent reports show?

- settingId: `screen-authoring-field-authoring-manifest-reports-reports-fields-0350`
- policyLane: `reports`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 780
- acceptedOptions: Setting State | Capability State | Capture Reason | Capture Scope | Category Candidates | Risk Signals | Confidence | Ocr Snippets | Redaction Notes | Source Evidence Refs | Local Model Runtime | Policy Decision Refs | Custody Label | Deletion State | Image Digest | Default setting-state | Default capability-state | Default capture-reason | Default capture-scope | Default category-candidates | Default risk-signals | Default confidence | Default source-evidence-refs | Default custody-label | Default deletion-state
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

258.  Show raw screenshots in parent reports by default?

- settingId: `screen-authoring-field-authoring-manifest-reports-reports-fields-0351`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `portal-only`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 815
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest Metadata

#### Rendering Rules

259.  Rendering rule showDisclosureBeforeEnable: true.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0360`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 114
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Policy Value Document

#### Fallbacks

260.  Fallback childDeviceOffline: last-known-report-only.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0444`
- policyLane: `reports`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1214
- acceptedOptions: Last Known Report Only
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

## Tab: data

### Core Terms

#### Screen Evidence

261.  encrypted temporary queue storage;

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0004`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 29
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### OCR

262.  credential-like text redaction;

- settingId: `screen-capability-guide-bullet-core-terms-ocr-0015`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 74
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Evidence Reference

263.  queue job lifecycle;

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0018`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 95
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### The Main Capability Truth

#### The Main Capability Truth

264.  the raw image or frame is stored only in an encrypted temporary local queue;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0032`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 120
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

265.  confidence, category, risk signal, redaction, custody, and deletion states validate;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0034`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 122
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

266.  the stored long-lived evidence is a summary plus refs, not raw pixels;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0035`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 123
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Screenshot Possibilities And Limits

#### What Is Possible

267.  OCR snippets when parent settings allow them and local redaction permits storage;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0044`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 161
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Screen Recording Possibilities And Limits

#### Screen Recording Possibilities And Limits

268.  storage, deletion, and failure handling are harder to prove;

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0064`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 200
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### OCR And Image Classification

#### OCR And Image Classification

269.  disabled unless parent enables snippet storage or local analysis needs transient text;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0079`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 235
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

270.  invalid output rejection before storage or policy use.

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0091`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 250
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Redaction And Minimization

#### Redaction And Minimization

271.  no raw screenshot shown by default;

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0106`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `local-ai-runtime`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 305
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

272.  no raw local file paths in portal copy/debug output;

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0107`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 306
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

273.  no encrypted image refs outside the child agent;

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0108`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 307
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

274.  OCR snippets bounded and redacted;

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0109`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 308
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

275.  credential-like text, passwords, tokens, payment fields, private keys, recovery codes, and session values redacted or omitted;

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0110`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 309
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

276.  protected regions skipped where the platform or local detector can identify them;

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0111`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 311
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

277.  uncertain redaction state degrades policy eligibility.

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0112`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 312
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Retention And Custody

#### Retention And Custody

278.  raw image/frame: encrypted temporary queue only, deleted after success or TTL expiry;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0113`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 321
- acceptedOptions: Encrypted Temporary Queue Only | Deleted After Success | TTL Expiry
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

279.  stored summary: local journal and SQLite query store;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0114`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 322
- acceptedOptions: Local Journal And SQLite Query Store
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

280.  parent cache/export: explicit parent-owned destination only;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0116`
- policyLane: `data`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 324
- acceptedOptions: Explicit Parent Owned Destination Only
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

281.  Ocentra-hosted storage: no child screen activity by default.

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0117`
- policyLane: `data`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 325
- acceptedOptions: No Child Screen Activity By Default
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

282.  `child-device-temp-queue`;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0118`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 329
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

283.  `child-device-journal`;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0119`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 330
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

284.  `child-device-query-store`;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0120`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 331
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

285.  `live-local-child-agent`;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0121`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 332
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

286.  `live-lan-child-agent`;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0122`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 333
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

287.  `parent-device-cache`;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0123`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 334
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

288.  `parent-owned-export`;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0124`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 335
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

289.  `ocentra-hosted-non-activity`;

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0125`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 336
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

290.  `unavailable`.

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0126`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 337
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Permission-Required And Unavailable States

#### Permission-Required And Unavailable States

291.  queue unavailable;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0137`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 356
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

292.  redaction unavailable;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0138`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 357
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Platform Capability Notes

#### Windows

293.  encrypted temporary queue and journal/SQLite read model;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0147`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 378
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### macOS

294.  encrypted local queue and summary storage.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0158`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 402
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Linux

295.  restore/persistent permission behavior differs across desktop portals;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0169`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 429
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Future UI Rules

#### Future UI Rules

296.  show whether OCR snippet storage is enabled;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0244`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 582
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

297.  show redaction mode and redaction failures;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0245`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `local-ai-runtime`; capabilityState: `future-gap`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 583
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

298.  show queue health and deletion health;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0247`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 585
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

299.  show raw capture retention as off by default;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0249`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 587
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

300.  show capability state close to each action: ready, unsupported, permission-required, permission-limited, protected-surface, model-unavailable, queue-unavailable, adapter-error, degraded, disabled-by-parent, or manual-required;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0250`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 588
- acceptedOptions: Ready | Unsupported | Permission Required | Permission Limited | Protected Surface | Model Unavailable | Queue Unavailable | Adapter Error | Degraded | Disabled By Parent | Manual Required
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

301.  strict deletion and no raw image retention by default.

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0262`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 605
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Capability Matrix

#### Retain raw capture

302.  Capability Retain raw capture: full screen No by default; active window No by default; managed browser/window No by default; local OCR/vision No by default; important limit Future retention needs separate custody/legal/privacy design..

- settingId: `screen-capability-matrix-row-capability-matrix-retain-raw-capture-0275`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 151
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Proposal Overview

#### Proposal Overview

303.  Encrypted temporary image queueing.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0281`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 12
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

304.  Typed summary storage with evidence refs and deletion state.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0283`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 14
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

305.  Tests for every parser, authoring manifest field, policy value shape, compile rule, patch command, capability state, queue state, deletion state, confidence value, and invalid-state rejection.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0291`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 27
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Implementation Notes For Worker

#### Implementation Notes For Worker

306.  Add explicit tests that invalid confidence, missing source refs, missing deletion state, protected surfaces, and delete failures cannot produce enforcement-eligible screen summaries.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0305`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1690
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - OCR and vision

#### OCR and vision fields

307.  Store bounded OCR text snippets in summaries?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0332`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 536
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Redaction

#### Redaction fields

308.  How should visible text and sensitive regions be redacted?

- settingId: `screen-authoring-field-authoring-manifest-redaction-redaction-fields-0337`
- policyLane: `data`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 597
- acceptedOptions: Off | Summary Only | Strict Local | Credential Sensitive | Parent Review Required | Default strict-local
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

309.  What if redaction is unavailable?

- settingId: `screen-authoring-field-authoring-manifest-redaction-redaction-fields-0339`
- policyLane: `data`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 637
- acceptedOptions: Summary Only Not Policy Eligible | Mark Invalid | Delete And Audit | Ask Parent | Default summary-only-not-policy-eligible
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Queue and retention

#### Queue and retention fields

310.  How long may a temporary image remain queued?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0340`
- policyLane: `data`; cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 656
- acceptedOptions: Default 300 | Minimum 30 | Maximum 1800
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

311.  How many local analysis retries are allowed before deletion?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0341`
- policyLane: `data`; cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 665
- acceptedOptions: Default 2 | Minimum 0 | Maximum 5
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

312.  Delete raw image after successful analysis?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0342`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 674
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

313.  Delete raw image after TTL expiry?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0343`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 682
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

314.  Retain raw screenshots or recordings?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0344`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `parent-owned-storage`; capabilityState: `unavailable`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 690
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

315.  Allow Ocentra-hosted processing of child screen images?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0345`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `parent-owned-storage`; capabilityState: `unavailable`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 698
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest Metadata

#### Rendering Rules

316.  Rendering rule showRawCaptureRetentionAsOffByDefault: true.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0362`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 116
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Control Kinds

317.  Control kind: retention.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0369`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 125
- acceptedOptions: Retention
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Capability Registry

#### Capability State Meanings

318.  queue-unavailable: Encrypted temporary queue cannot be opened or validated.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0405`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1663
- acceptedOptions: Queue Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Update Protocol

#### Agent Rules

319.  Agent rule deleteQueuedImagesOnInvalidOutput: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0430`
- policyLane: `data`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1548
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Policy Value Document

#### Fallbacks

320.  Fallback queueUnavailable: fail-closed.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0438`
- policyLane: `data`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1208
- acceptedOptions: Fail Closed
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

321.  Fallback modelUnavailable: retry-within-ttl-then-delete.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0439`
- policyLane: `data`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1209
- acceptedOptions: Retry Within Ttl Then Delete
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

322.  Fallback redactionUnavailable: summary-only-not-policy-eligible.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0440`
- policyLane: `data`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1210
- acceptedOptions: Summary Only Not Policy Eligible
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

323.  Fallback invalidModelOutput: delete-and-mark-invalid.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0441`
- policyLane: `data`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1211
- acceptedOptions: Delete And Mark Invalid
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

324.  Fallback deleteFailed: surface-health-and-retry.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0443`
- policyLane: `data`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1213
- acceptedOptions: Surface Health And Retry
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

## Tab: ai

### Core Terms

#### Screen Evidence

325.  local OCR/vision analysis;

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0005`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 30
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

326.  schema-valid summary, category candidates, risk signals, confidence, evidence refs, digest, and deletion state;

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0006`
- policyLane: `ai`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 31
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Screenshot

327.  protected or unsupported scope represented as unavailable, not captured.

- settingId: `screen-capability-guide-bullet-core-terms-screenshot-0012`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 45
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### OCR

328.  visible text snippets;

- settingId: `screen-capability-guide-bullet-core-terms-ocr-0013`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 72
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

329.  text category hints;

- settingId: `screen-capability-guide-bullet-core-terms-ocr-0014`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 73
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

330.  unsafe phrase or bypass-tool signals;

- settingId: `screen-capability-guide-bullet-core-terms-ocr-0016`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 75
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

331.  policy explanation references.

- settingId: `screen-capability-guide-bullet-core-terms-ocr-0017`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 76
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

#### Evidence Reference

332.  managed browser URL/tab evidence where available;

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0022`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 99
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### The Main Capability Truth

#### The Main Capability Truth

333.  the current platform adapter supports the requested scope;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0029`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 117
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

334.  required OS permission or management state is present;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0030`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 118
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

335.  protected surfaces are skipped or represented as unavailable;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0031`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 119
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

336.  local OCR/vision returns schema-valid output;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0033`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 121
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

337.  policy and enforcement consume only typed summaries and evidence refs.

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0036`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 124
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

338.  hidden capture;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0037`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `local-ai-runtime`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 128
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

339.  cloud/API AI screenshot processing by default;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0038`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `local-ai-runtime`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 129
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

340.  permanent screenshot history by default;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0039`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `local-ai-runtime`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 130
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

341.  exact page, URL, chat, password, or intent from pixels alone;

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0040`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 131
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

342.  enforcement from raw model text or unvalidated image classification.

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0042`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 133
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Screenshot Possibilities And Limits

#### What Is Possible

343.  visible activity categories such as school, video, chat, game, shopping, productivity, adult content, violence, bypass tool, unknown, or low confidence;

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0043`
- policyLane: `ai`; cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 159
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Screen Recording Possibilities And Limits

#### Screen Recording Possibilities And Limits

344.  short rolling local analysis buffer that never becomes a retained video archive;

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0060`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 193
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

345.  accessibility-like visible flow analysis where the platform permits it.

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0062`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `portal-only`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 195
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

346.  protected media and secure surfaces may appear black, unavailable, omitted, or blocked depending on OS;

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0067`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 203
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### OCR And Image Classification

#### OCR And Image Classification

347.  bounded snippet count and character length;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0080`
- policyLane: `ai`; cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 236
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

348.  sensitive tokens, passwords, credential-like text, payment data, and secrets redacted or skipped;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0082`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 238
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

349.  OCR-disabled state represented explicitly;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0083`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 239
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

350.  unsupported language, low resolution, or low confidence represented as unknown/degraded.

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0084`
- policyLane: `ai`; cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 240
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

351.  enum-backed categories and risk signals, not open-ended model prose;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0085`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 244
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

352.  confidence in `0..1`;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0086`
- policyLane: `ai`; cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 245
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

353.  uncertainty reason;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0088`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 247
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

354.  source evidence refs;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0089`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 248
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

355.  policy eligibility flag;

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0090`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 249
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Permission-Required And Unavailable States

#### Permission-Required And Unavailable States

356.  disabled by parent;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0127`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 346
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

357.  unsupported platform;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0128`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 347
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

358.  unsupported scope;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0129`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 348
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

359.  permission required;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0130`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 349
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

360.  permission denied;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0131`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 350
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

361.  permission limited;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0132`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-limited`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-limited`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 351
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

362.  protected surface;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0133`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 352
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

363.  screen locked;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0134`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 353
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

364.  session unavailable;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0135`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 354
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

365.  model unavailable;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0136`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 355
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

366.  degraded;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0139`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 358
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

367.  adapter error;

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0140`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 359
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

368.  ready.

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0141`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 360
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Platform Capability Notes

#### Windows

369.  local OCR through Windows OCR APIs or another local model boundary where packaged/available;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0145`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 376
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

370.  local vision classification through an Ocentra-owned local model/provider boundary;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0146`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 377
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

371.  secure desktop, lock screen, UAC prompts, credential surfaces, protected media, or DRM-protected content must be skipped or represented as protected/unavailable;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0151`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 386
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### macOS

372.  local Vision framework OCR/classification or Ocentra local model boundary;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0156`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 400
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

373.  protected windows or windows that opt out of sharing may be unavailable or omitted;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0160`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 408
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Linux

374.  local OCR/vision through Ocentra-owned local model/provider boundary;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0165`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 422
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

375.  available source types vary by backend;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0168`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 428
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Android

376.  on-device ML Kit or Ocentra local model boundary for OCR/image labeling where allowed;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0175`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 443
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### iOS And iPadOS

377.  third-party parental-control apps should not claim arbitrary hidden screenshot or screen-recording access;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0187`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 470
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### App And Window Correlation

#### App And Window Correlation

378.  exact URL from window title or OCR alone;

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0197`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: managed-browser-evidence-required
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 492
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Future UI Rules

#### Future UI Rules

379.  show screen analysis disabled by default;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0242`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 579
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

380.  show capture scope as full screen, active display, active window, managed browser/window, app window, or unavailable;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0243`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `local-ai-runtime`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 580
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

381.  no screen analysis;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0254`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 597
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

382.  local enforcement-eligible summaries with confidence thresholds;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0257`
- policyLane: `ai`; cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `future-gap`; runtimeOwner: `local-ai-runtime`; capabilityState: `future-gap`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 600
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

383.  OCR snippets off or bounded/on;

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0261`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `local-ai-runtime`; capabilityState: `future-gap`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 604
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Capability Matrix

#### Classify visible activity

384.  Capability Classify visible activity: full screen Broad but sensitive; active window Narrower and usually more relevant; managed browser/window Narrowest for web/app context; local OCR/vision Yes, with confidence; important limit Category is not policy authority..

- settingId: `screen-capability-matrix-row-capability-matrix-classify-visible-activity-0265`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 141
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Extract OCR snippets

385.  Capability Extract OCR snippets: full screen Possible; active window Possible; managed browser/window Possible; local OCR/vision Yes, if enabled; important limit Snippets must be bounded and redacted..

- settingId: `screen-capability-matrix-row-capability-matrix-extract-ocr-snippets-0266`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 142
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Prove app/window context

386.  Capability Prove app/window context: full screen Correlate with foreground evidence; active window Stronger when captured source is a window; managed browser/window Strong if managed session/window id is linked; local OCR/vision No by itself; important limit Capture source ids must be recorded..

- settingId: `screen-capability-matrix-row-capability-matrix-prove-app-window-context-0268`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 144
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Detect protected surfaces

387.  Capability Detect protected surfaces: full screen Platform dependent; active window Platform dependent; managed browser/window Platform dependent; local OCR/vision Not after the fact reliably; important limit Protected/secure/credential states must fail closed..

- settingId: `screen-capability-matrix-row-capability-matrix-detect-protected-surfaces-0270`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 146
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Feed local AI

388.  Capability Feed local AI: full screen Summary/ref only by default; active window Summary/ref only by default; managed browser/window Summary/ref only by default; local OCR/vision Yes after schema validation; important limit Raw image is not normal AI context-builder input..

- settingId: `screen-capability-matrix-row-capability-matrix-feed-local-ai-0271`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 147
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Feed policy

389.  Capability Feed policy: full screen Only via summary/ref; active window Only via summary/ref; managed browser/window Only via summary/ref; local OCR/vision Yes, after validation; important limit Requires parent rule and confidence threshold..

- settingId: `screen-capability-matrix-row-capability-matrix-feed-policy-0272`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 148
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Proposal Overview

#### Proposal Overview

390.  Parent-authored screen analysis settings.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0277`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 8
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

391.  Local OCR/vision analysis.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0282`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 13
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Implementation Notes For Worker

#### Implementation Notes For Worker

392.  Start with domain contracts before Portal UI.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0294`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1674
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

393.  Persist both policy revision and compiled effective policy hash.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0299`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1680
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

394.  Keep `retainRawCapture` and `hostedProcessingAllowed` false for this schema version.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0302`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1685
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Screen analysis

#### Screen analysis fields

395.  Enable local screen evidence analysis?

- settingId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields-0307`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 150
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

396.  How should screen analysis be used?

- settingId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields-0308`
- policyLane: `ai`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 174
- acceptedOptions: Observe Only | Policy Preview | Ask Parent | Can Enforce | Default observe-only
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

397.  Where should screen analysis run?

- settingId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields-0309`
- policyLane: `ai`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 207
- acceptedOptions: Local Child Agent | Lan Live Child Agent | Authoring Only | Unavailable | Default local-child-agent
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Capture scope

#### Capture scope fields

398.  Require managed browser evidence for exact web claims?

- settingId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields-0315`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: managed-browser-evidence-required
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 294
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Screen recording

#### Screen recording fields

399.  How may recording frames be used for analysis?

- settingId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields-0318`
- policyLane: `ai`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 339
- acceptedOptions: No Recording | Sample Keyframes Only | Sample At Trigger Boundary | Summarize Then Delete | Default no-recording
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

400.  How should raw recording data be retained?

- settingId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields-0320`
- policyLane: `ai`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 355
- acceptedOptions: No Raw Video Retention | Temporary Queue Only | Future Explicit Parent Controlled Retention | Default no-raw-video-retention
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - Scheduling

#### Scheduling fields

401.  When may screen analysis run?

- settingId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields-0324`
- policyLane: `ai`; cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 405
- acceptedOptions: Default always
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest - OCR and vision

#### OCR and vision fields

402.  Require local OCR/vision for screen analysis?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0330`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 514
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

403.  Which local analysis tasks are allowed?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0331`
- policyLane: `ai`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 521
- acceptedOptions: Visible Category Classification | Safety Indicator Classification | Ocr Transient Only | Ocr Snippet Storage | Sensitive Region Redaction | Managed Window Classification | Default visible-category-classification | Default safety-indicator-classification
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

404.  How many OCR snippets may be retained per summary?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0333`
- policyLane: `ai`; cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 543
- acceptedOptions: Default 3 | Minimum 0 | Maximum 10
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

405.  What is the maximum length of each retained OCR snippet?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0334`
- policyLane: `ai`; cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 556
- acceptedOptions: Default 120 | Minimum 0 | Maximum 500
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

406.  What confidence is required before screen summaries can affect policy?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0335`
- policyLane: `ai`; cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 569
- acceptedOptions: Default 0.8 | Minimum 0 | Maximum 1
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

407.  What if the local model returns invalid output?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0336`
- policyLane: `ai`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 578
- acceptedOptions: Delete And Mark Invalid | Retry Within Ttl | Mark Unavailable | Ask Parent | Default delete-and-mark-invalid
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest Metadata

#### Condition Kinds

408.  Condition kind: capabilityAvailable.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0380`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 138
- acceptedOptions: CapabilityAvailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Capability Registry

#### Capability State Meanings

409.  unsupported-scope: The requested full-screen, display, window, app, or managed-window scope is unavailable.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0400`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1658
- acceptedOptions: Unsupported Scope
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

410.  degraded: The capability can run with reduced scope, fidelity, freshness, or confidence.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0406`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1664
- acceptedOptions: Degraded
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Update Protocol

#### Commands

411.  screen-policy.get.requested: Portal asks the child agent for current screen policy value, effective policy, capability registry, and revision.

- settingId: `screen-update-command-update-protocol-commands-0409`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1424
- acceptedOptions: Screen Policy.get.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

412.  screen-policy.patch.requested: Portal sends a small settings change with an expected revision.

- settingId: `screen-update-command-update-protocol-commands-0411`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1456
- acceptedOptions: Screen Policy.patch.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

413.  screen-policy.rollback.requested: Parent asks child agent to roll back to previous valid revision.

- settingId: `screen-update-command-update-protocol-commands-0414`
- policyLane: `ai`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1523
- acceptedOptions: Screen Policy.rollback.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Agent Rules

414.  Agent rule keepPreviousValidRevision: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0418`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1536
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

415.  Agent rule rollbackOnCompileFailure: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0419`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1537
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

416.  Agent rule runOcrVisionOnlyInChildAgent: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0421`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1539
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

417.  Agent rule rejectConfidenceOutsideZeroOne: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0425`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1543
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

418.  Agent rule rejectRetainRawCaptureForSchemaVersionOne: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0426`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1544
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

419.  Agent rule rejectExactWebClaimsWithoutManagedBrowserEvidence: true.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0429`
- policyLane: `ai`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1547
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Policy Value Document

#### Fallbacks

420.  Fallback permissionDenied: mark-unavailable.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0433`
- policyLane: `ai`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1203
- acceptedOptions: Mark Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

421.  Fallback unsupportedScope: fall-back-to-supported-scope-or-unavailable.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0435`
- policyLane: `ai`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1205
- acceptedOptions: Fall Back To Supported Scope Or Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

422.  Fallback platformUnsupported: show-unavailable.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0445`
- policyLane: `ai`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1215
- acceptedOptions: Show Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

423.  Fallback policyUse.protectedSurfaceFallback: mark-unavailable.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0447`
- policyLane: `ai`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1021
- acceptedOptions: Mark Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

424.  Fallback policyUse.invalidOutputFallback: mark-unavailable.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0448`
- policyLane: `ai`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1022
- acceptedOptions: Mark Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

425.  Fallback portalAi.fallbackWhenUnavailable: manual-view.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0449`
- policyLane: `ai`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1146
- acceptedOptions: Manual View
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

## Tab: setup

### Screen Recording Possibilities And Limits

#### Screen Recording Possibilities And Limits

426.  short parent test session during setup;

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0059`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 192
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Proposal Overview

#### Proposal Overview

427.  Full policy replacement during setup/import/reset.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0285`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 16
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Capability Registry

#### Capability State Meanings

428.  disabled-by-parent: Parent setting disables the feature.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0398`
- policyLane: `setup`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `disabled`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1656
- acceptedOptions: Disabled By Parent
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Update Protocol

#### Commands

429.  screen-policy.replace.requested: Portal sends a full policy replacement for setup, import, reset, or wizard save.

- settingId: `screen-update-command-update-protocol-commands-0412`
- policyLane: `setup`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1486
- acceptedOptions: Screen Policy.replace.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

430.  screen-policy.manual-test-capture.requested: Parent requests one explicit setup/test capture through the child agent.

- settingId: `screen-update-command-update-protocol-commands-0413`
- policyLane: `setup`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1501
- acceptedOptions: Screen Policy.manual Test Capture.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

## Tab: platform

### Core Terms

#### Screen Evidence

431.  platform capability and permission check;

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0002`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 27
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Screen Recording Possibilities And Limits

#### Screen Recording Possibilities And Limits

432.  platform consent prompts and indicators are common and must not be bypassed;

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0068`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 204
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

433.  iOS and Android have especially strong user-consent and OS-policy limits.

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0069`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 205
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Managed Browser Or Window Capture

#### Managed Browser Or Window Capture

434.  exclude the Ocentra app window where the platform supports exclusion filters;

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0072`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 216
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

435.  window capture can miss popups, overlays, system prompts, or secondary windows;

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0076`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 224
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

436.  active window capture can break on virtual desktops, minimized windows, DRM/protected content, or permission changes.

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0078`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 226
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Platform Capability Notes

#### Windows

437.  Windows Graphics Capture for display or application-window capture with system UI consent;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0142`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 373
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

438.  screenshot or frame capture from an approved display/window scope;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0143`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 374
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

439.  foreground process/window evidence from the Rust agent;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0144`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 375
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

440.  managed browser/window correlation through managed browser evidence and process/window refs.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0148`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 379
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

441.  consent, notification border, packaged app identity, service/session boundaries, and user desktop state affect what can be captured;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0150`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 384
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

442.  service capture from a non-interactive session is not the same as user desktop capture;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0152`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 388
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### macOS

443.  ScreenCaptureKit display/app/window streams;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0154`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 398
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

444.  macOS Screen Recording permission;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0155`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 399
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

445.  process/window correlation where permissions and APIs allow;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0157`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 401
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

446.  Screen Recording permission, app restart after first grant, sandboxing, app bundle identity, TCC state, and signing/notarization affect real behavior;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0159`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 406
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### Linux

447.  XDG Desktop Portal ScreenCast for monitors, windows, or virtual sources where a portal backend supports them;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0162`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `portal-only`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 418
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

448.  PipeWire stream capture on Wayland-backed desktops;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0163`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 420
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

449.  X11 screenshot paths where still supported;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0164`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 421
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

450.  process/window correlation depending on desktop environment.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0166`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 423
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

451.  Wayland commonly requires a portal-mediated user selection flow;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0167`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `portal-only`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 427
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

#### Android

452.  MediaProjection for screen or, on modern Android, selected app-window sharing;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0172`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 439
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

453.  foreground service requirements for active capture;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0173`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 440
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

454.  UsageStats, accessibility, VPN/DNS, device owner, or managed profile only where explicitly approved and enabled;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0174`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 441
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

455.  package lifecycle and policy state from DevicePolicyManager where device-owner/profile-owner setup exists.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0176`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 444
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

456.  MediaProjection requires user consent and can be revoked;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0177`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 449
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

457.  Android 14 app-window sharing can restrict capture to a selected app and exclude system UI;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0178`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 450
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

458.  normal apps cannot silently monitor arbitrary screen content in the background as a parental-control agent;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0179`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 451
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

459.  device-owner/profile-owner state changes what is possible;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0180`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 453
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

460.  screen capture may be disabled by policy or protected by app/window flags;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0181`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 454
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### iOS And iPadOS

461.  ReplayKit for user-initiated app/screen recording or broadcasting flows;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0184`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 464
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

462.  ReplayKit is user-consent oriented and not a stealth child-monitoring API;

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0188`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-capability-guide.md`; sourceLine: 471
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### Authoring Manifest - Capture scope

#### Capture scope fields

463.  Which capture scopes are allowed?

- settingId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields-0311`
- policyLane: `platform`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 250
- acceptedOptions: Full Screen | Active Display | Active Window | Selected App Window | Managed Browser Window | Manual Parent Test Only | Default active-window | Default managed-browser-window
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

464.  What scope should be tried first?

- settingId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields-0312`
- policyLane: `platform`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 265
- acceptedOptions: Active Window | Managed Browser Window | Active Display | Full Screen | Manual Parent Test Only | Default active-window
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

465.  What should happen on protected surfaces?

- settingId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields-0313`
- policyLane: `platform`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 279
- acceptedOptions: Skip And Audit | Delete Partial And Audit | Pause Until Clear | Mark Unavailable | Default skip-and-audit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

466.  Require app or window evidence before policy can use screen summaries?

- settingId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields-0314`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 287
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Authoring Manifest Metadata

#### Condition Kinds

467.  Condition kind: platformIn.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0381`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 139
- acceptedOptions: PlatformIn
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

468.  Condition kind: permissionStateIn.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0383`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 141
- acceptedOptions: PermissionStateIn
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Capability Registry

#### Capability State Meanings

469.  unsupported-platform: Current platform cannot support this capability in the current build.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0399`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1657
- acceptedOptions: Unsupported Platform
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

470.  permission-required: OS permission, user consent, management state, or entitlement is required before capture.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0401`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1659
- acceptedOptions: Permission Required
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

471.  permission-limited: Permission exists but does not cover the requested scope.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0402`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1660
- acceptedOptions: Permission Limited
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

472.  protected-surface: Secure, locked, credential, DRM, or OS-protected surface prevents usable capture.

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0403`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1661
- acceptedOptions: Protected Surface
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### Policy Value Document

#### Fallbacks

473.  Fallback permissionRequired: show-setup-required.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0432`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1202
- acceptedOptions: Show Setup Required
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

474.  Fallback permissionLimited: mark-degraded.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0434`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: `docs/screen-evidence-analysis-schema-proposal.md`; sourceLine: 1204
- acceptedOptions: Mark Degraded
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.
