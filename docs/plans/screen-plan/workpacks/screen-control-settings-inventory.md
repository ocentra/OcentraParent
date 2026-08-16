<!-- agent-capsule -->

> Agent Capsule
> Doc: Screen Control Settings Inventory
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Screen Control Settings Inventory

Generated from `BaselineScreenControlCatalog`.
Total settings: 474

Use this as the raw review list for deciding parent-facing grouping, proof gaps, and policy UX.
This is a generated inventory of current typed catalog data, not product-complete implementation proof.

## Source Documents

- docs/screen-evidence-analysis-capability-guide.md
- docs/screen-evidence-analysis-schema-proposal.md

## Tab: evidence

### screen-capability-guide-bullet-core-terms

#### screen-capability-guide-bullet-core-terms-screen-evidence

1.  Use parent-enabled setting;?

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0001`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screen-evidence`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 26; sourceText: parent-enabled setting;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

2.  Use local capture of the approved scope;?

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0003`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screen-evidence`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 28; sourceText: local capture of the approved scope;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

3.  Use deletion of the raw image or frame data?

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0007`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screen-evidence`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 33; sourceText: deletion of the raw image or frame data.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-core-terms-screenshot

4.  Use full screen or display;?

- settingId: `screen-capability-guide-bullet-core-terms-screenshot-0008`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screenshot`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 41; sourceText: full screen or display;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

5.  Use active window;?

- settingId: `screen-capability-guide-bullet-core-terms-screenshot-0009`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screenshot`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 42; sourceText: active window;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

6.  Use managed browser window;?

- settingId: `screen-capability-guide-bullet-core-terms-screenshot-0010`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screenshot`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 43; sourceText: managed browser window;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

7.  Use selected app window;?

- settingId: `screen-capability-guide-bullet-core-terms-screenshot-0011`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screenshot`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 44; sourceText: selected app window;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-core-terms-evidence-reference

8.  Use image digest;?

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0019`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-evidence-reference`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 96; sourceText: image digest;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

9.  Use deletion state;?

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0020`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-evidence-reference`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 97; sourceText: deletion state;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

10. Represent foreground app/window evidence;.

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0021`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-evidence-reference`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 98; sourceText: foreground app/window evidence;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

11. Represent app/game session evidence;.

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0023`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-evidence-reference`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 100; sourceText: app/game session evidence;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

12. Represent network digest evidence;.

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0024`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-evidence-reference`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 101; sourceText: network digest evidence;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

13. Use parent setting version;?

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0026`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-evidence-reference`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 103; sourceText: parent setting version;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-capability-guide-bullet-managed-browser-or-window-capture

#### screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture

14. Use capture only the Ocentra-managed browser window;?

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0070`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-managed-browser-or-window-capture`; groupId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 214; sourceText: capture only the Ocentra-managed browser window;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

15. Use capture only an approved app window;?

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0071`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-managed-browser-or-window-capture`; groupId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 215; sourceText: capture only an approved app window;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

16. Use correlate capture with managed session id, window id, process id, and evidence refs?

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0073`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-managed-browser-or-window-capture`; groupId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 217; sourceText: correlate capture with managed session id, window id, process id, and evidence refs.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

17. Use a managed browser screenshot is still not the source of exact URL truth;?

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0074`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-managed-browser-or-window-capture`; groupId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: managed-browser-evidence-required
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 221; sourceText: a managed browser screenshot is still not the source of exact URL truth;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

18. Use window title and visible pixels can be stale or misleading;?

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0077`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-managed-browser-or-window-capture`; groupId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 225; sourceText: window title and visible pixels can be stale or misleading;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-app-and-window-correlation

#### screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation

19. Represent foreground process/window evidence;.

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0191`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-app-and-window-correlation`; groupId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 480; sourceText: foreground process/window evidence;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

20. Represent managed browser session and tab evidence;.

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0192`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-app-and-window-correlation`; groupId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 481; sourceText: managed browser session and tab evidence;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

21. Represent app/game session summary;.

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0193`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-app-and-window-correlation`; groupId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 482; sourceText: app/game session summary;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

22. Use network flow digest;?

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0194`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-app-and-window-correlation`; groupId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 483; sourceText: network flow digest;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

23. Use app duration from repeated screenshots alone;?

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0198`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-app-and-window-correlation`; groupId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation`
- cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 493; sourceText: app duration from repeated screenshots alone;
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

24. Use network destination from visible content alone;?

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0199`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-app-and-window-correlation`; groupId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 494; sourceText: network destination from visible content alone;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

25. Use child intent from category alone?

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0200`
- policyLane: `evidence`; sectionId: `screen-capability-guide-bullet-app-and-window-correlation`; groupId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 495; sourceText: child intent from category alone.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-proposal-overview

#### screen-schema-proposal-bullet-proposal-overview-proposal-overview

26. Use portal-rendered question/option UI?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0276`
- policyLane: `evidence`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 7; sourceText: Portal-rendered question/option UI.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

27. Use local screenshot or frame capture only after explicit enablement?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0280`
- policyLane: `evidence`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 11; sourceText: Local screenshot or frame capture only after explicit enablement.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

28. Use small patch updates from Portal?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0284`
- policyLane: `evidence`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 15; sourceText: Small patch updates from Portal.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

29. Use deterministic compile into an effective local execution plan?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0286`
- policyLane: `evidence`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 17; sourceText: Deterministic compile into an effective local execution plan.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

30. Represent effect Schema validation.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0287`
- policyLane: `evidence`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 23; sourceText: Effect Schema validation.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

31. Represent branded ids from schema brands, not manual brands.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0288`
- policyLane: `evidence`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 24; sourceText: Branded ids from schema brands, not manual brands.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

32. Use decode helpers?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0289`
- policyLane: `evidence`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 25; sourceText: Decode helpers.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

33. Represent rust protocol parity only after the TypeScript contracts are explicit and test-backed.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0292`
- policyLane: `evidence`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 29; sourceText: Rust protocol parity only after the TypeScript contracts are explicit and test-backed.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-schema-proposal-bullet-implementation-notes-for-worker

#### screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker

34. Represent do not let Portal define arbitrary JSON paths. `writesTo` paths should be schema-known authoring paths.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0296`
- policyLane: `evidence`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1677; sourceText: Do not let Portal define arbitrary JSON paths. `writesTo` paths should be schema-known authoring paths.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

35. Represent add Rust parity only for Rust-crossing commands/events after TypeScript contracts and tests are stable.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0306`
- policyLane: `evidence`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1692; sourceText: Add Rust parity only for Rust-crossing commands/events after TypeScript contracts and tests are stable.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-authoring-field-authoring-manifest-screen-recording

#### screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields

36. Should screen recording be allowed?

- settingId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields-0316`
- policyLane: `evidence`; sectionId: `screen-authoring-field-authoring-manifest-screen-recording`; groupId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 312; sourceText: Should screen recording be allowed?
- acceptedOptions: Disabled | Manual Parent Test Only | Short Local Buffer | Triggered Frame Sampling | Authoring Only Manual Required | Default disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

37. What is the maximum local recording segment length?

- settingId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields-0317`
- policyLane: `evidence`; sectionId: `screen-authoring-field-authoring-manifest-screen-recording`; groupId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields`
- cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 326; sourceText: What is the maximum local recording segment length?
- acceptedOptions: Default 15 | Minimum 1 | Maximum 60
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

38. Allow continuous screen recording?

- settingId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields-0319`
- policyLane: `evidence`; sectionId: `screen-authoring-field-authoring-manifest-screen-recording`; groupId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `os-adapter`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 347; sourceText: Allow continuous screen recording?
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-scheduling

#### screen-authoring-field-authoring-manifest-scheduling-scheduling-fields

39. When should capture pause?

- settingId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields-0325`
- policyLane: `evidence`; sectionId: `screen-authoring-field-authoring-manifest-scheduling`; groupId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 412; sourceText: When should capture pause?
- acceptedOptions: Screen Locked | Protected Surface | Permission Required | Permission Limited | Queue Unavailable | Model Unavailable | Battery Saver | Metered Connection | Parent Paused | Default screen-locked | Default protected-surface | Default permission-required | Default queue-unavailable | Default model-unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-control-kind-authoring-manifest-metadata

#### screen-control-kind-authoring-manifest-metadata-control-kinds

40. Choose control kind.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0363`
- policyLane: `evidence`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 119; sourceText: Control kind: boolean.
- acceptedOptions: Boolean
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

41. Choose control kind.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0364`
- policyLane: `evidence`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 120; sourceText: Control kind: single-choice.
- acceptedOptions: Single Choice
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

42. Choose control kind.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0365`
- policyLane: `evidence`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 121; sourceText: Control kind: multi-choice.
- acceptedOptions: Multi Choice
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

43. Choose control kind.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0366`
- policyLane: `evidence`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 122; sourceText: Control kind: number.
- acceptedOptions: Number
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

44. Choose control kind.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0367`
- policyLane: `evidence`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 123; sourceText: Control kind: duration.
- acceptedOptions: Duration
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

45. Choose control kind.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0372`
- policyLane: `evidence`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 128; sourceText: Control kind: threshold.
- acceptedOptions: Threshold
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

46. Represent control kind: read-only-status.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0373`
- policyLane: `evidence`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 129; sourceText: Control kind: read-only-status.
- acceptedOptions: Read Only Status
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-condition-kind-authoring-manifest-metadata

#### screen-condition-kind-authoring-manifest-metadata-condition-kinds

47. Represent condition kind: equals.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0374`
- policyLane: `evidence`; sectionId: `screen-condition-kind-authoring-manifest-metadata`; groupId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 132; sourceText: Condition kind: equals.
- acceptedOptions: Equals
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

48. Represent condition kind: notEquals.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0375`
- policyLane: `evidence`; sectionId: `screen-condition-kind-authoring-manifest-metadata`; groupId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 133; sourceText: Condition kind: notEquals.
- acceptedOptions: NotEquals
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

49. Represent condition kind: includes.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0376`
- policyLane: `evidence`; sectionId: `screen-condition-kind-authoring-manifest-metadata`; groupId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 134; sourceText: Condition kind: includes.
- acceptedOptions: Includes
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

50. Represent condition kind: notIncludes.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0377`
- policyLane: `evidence`; sectionId: `screen-condition-kind-authoring-manifest-metadata`; groupId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 135; sourceText: Condition kind: notIncludes.
- acceptedOptions: NotIncludes
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

51. Represent condition kind: all.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0378`
- policyLane: `evidence`; sectionId: `screen-condition-kind-authoring-manifest-metadata`; groupId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 136; sourceText: Condition kind: all.
- acceptedOptions: All
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

52. Represent condition kind: any.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0379`
- policyLane: `evidence`; sectionId: `screen-condition-kind-authoring-manifest-metadata`; groupId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 137; sourceText: Condition kind: any.
- acceptedOptions: Any
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

## Tab: rules

### screen-capability-guide-bullet-screenshot-possibilities-and-limits

#### screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable

53. Use parent policy outcome without a typed parent rule?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0058`
- policyLane: `rules`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable`
- cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 183; sourceText: parent policy outcome without a typed parent rule.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-managed-browser-or-window-capture

#### screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture

54. Use exact URL and title require browser evidence from CDP, extension, browser policy, or another approved browser integration;?

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0075`
- policyLane: `rules`; sectionId: `screen-capability-guide-bullet-managed-browser-or-window-capture`; groupId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: managed-browser-evidence-required
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 222; sourceText: exact URL and title require browser evidence from CDP, extension, browser policy, or another approved browser integration;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-app-and-window-correlation

#### screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation

55. Use parent rule and setting version?

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0196`
- policyLane: `rules`; sectionId: `screen-capability-guide-bullet-app-and-window-correlation`; groupId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation`
- cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 485; sourceText: parent rule and setting version.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-future-ui-rules

#### screen-capability-guide-bullet-future-ui-rules-future-ui-rules

56. Use show policy use as observe-only, dry-run, enforcement-eligible, or disabled;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0251`
- policyLane: `rules`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 591; sourceText: show policy use as observe-only, dry-run, enforcement-eligible, or disabled;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

57. Use local observe-only summaries;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0255`
- policyLane: `rules`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 598; sourceText: local observe-only summaries;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

58. Use local policy dry-run;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0256`
- policyLane: `rules`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 599; sourceText: local policy dry-run;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

59. Use manual parent test capture;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0258`
- policyLane: `rules`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 601; sourceText: manual parent test capture;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-proposal-overview

#### screen-schema-proposal-bullet-proposal-overview-proposal-overview

60. Represent child-agent local persisted policy.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0278`
- policyLane: `rules`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 9; sourceText: Child-agent local persisted policy.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

61. Use offline operation from the last valid policy?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0279`
- policyLane: `rules`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 10; sourceText: Offline operation from the last valid policy.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-schema-proposal-bullet-implementation-notes-for-worker

#### screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker

62. Represent use Effect Schema to validate the full policy after every patch.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0297`
- policyLane: `rules`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1678; sourceText: Use Effect Schema to validate the full policy after every patch.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

63. Represent compile the effective policy in the child-agent/service boundary, not in Portal.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0298`
- policyLane: `rules`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1679; sourceText: Compile the effective policy in the child-agent/service boundary, not in Portal.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

64. Represent add explicit tests for offline behavior: child agent continues using the last valid compiled policy when Portal is disconnected.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0304`
- policyLane: `rules`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1688; sourceText: Add explicit tests for offline behavior: child agent continues using the last valid compiled policy when Portal is disconnected.
- acceptedOptions: Child Agent Continues Using The Last Valid Compiled Policy When Portal Is Disconnected
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-authoring-field-authoring-manifest-policy-use

#### screen-authoring-field-authoring-manifest-policy-use-policy-use-fields

65. Allow screen summaries to be used by policy?

- settingId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields-0346`
- policyLane: `rules`; sectionId: `screen-authoring-field-authoring-manifest-policy-use`; groupId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 717; sourceText: Allow screen summaries to be used by policy?
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

66. What screen-derived targets may policy match?

- settingId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields-0347`
- policyLane: `rules`; sectionId: `screen-authoring-field-authoring-manifest-policy-use`; groupId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 724; sourceText: What screen-derived targets may policy match?
- acceptedOptions: Visible Category | Risk Signal | Ocr Snippet Presence | Unknown State | Protected Surface | Capability State | Default visible-category | Default risk-signal | Default unknown-state
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

67. Which evidence refs are required before policy use?

- settingId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields-0349`
- policyLane: `rules`; sectionId: `screen-authoring-field-authoring-manifest-policy-use`; groupId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 751; sourceText: Which evidence refs are required before policy use?
- acceptedOptions: Screen Summary | Queue Deletion State | Local Model Runtime | Foreground App Window | Managed Browser State | App Game Session | Network Digest | Parent Setting Version | Policy Version | Default screen-summary | Default queue-deletion-state | Default local-model-runtime | Default parent-setting-version
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-rendering-rule-authoring-manifest-metadata

#### screen-rendering-rule-authoring-manifest-metadata-rendering-rules

68. Choose rendering rule hideInvisibleFields.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0355`
- policyLane: `rules`; sectionId: `screen-rendering-rule-authoring-manifest-metadata`; groupId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 109; sourceText: Rendering rule hideInvisibleFields: true.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

69. Choose rendering rule showDisabledFieldsWithReason.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0356`
- policyLane: `rules`; sectionId: `screen-rendering-rule-authoring-manifest-metadata`; groupId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 110; sourceText: Rendering rule showDisabledFieldsWithReason: true.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

70. Choose rendering rule neverInventFieldsOutsideManifest.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0357`
- policyLane: `rules`; sectionId: `screen-rendering-rule-authoring-manifest-metadata`; groupId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 111; sourceText: Rendering rule neverInventFieldsOutsideManifest: true.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

71. Choose rendering rule writeOnlyThroughWritesToPath.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0358`
- policyLane: `rules`; sectionId: `screen-rendering-rule-authoring-manifest-metadata`; groupId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 112; sourceText: Rendering rule writeOnlyThroughWritesToPath: true.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

72. Choose rendering rule previewBeforeApply.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0359`
- policyLane: `rules`; sectionId: `screen-rendering-rule-authoring-manifest-metadata`; groupId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 113; sourceText: Rendering rule previewBeforeApply: true.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

73. Choose rendering rule showCapabilityStateBesideSensitiveControls.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0361`
- policyLane: `rules`; sectionId: `screen-rendering-rule-authoring-manifest-metadata`; groupId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 115; sourceText: Rendering rule showCapabilityStateBesideSensitiveControls: true.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-control-kind-authoring-manifest-metadata

#### screen-control-kind-authoring-manifest-metadata-control-kinds

74. Choose control kind.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0370`
- policyLane: `rules`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `target-list-card`; selectionMode: `multi`; controlKind: `target-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 126; sourceText: Control kind: target-list.
- acceptedOptions: Target List
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

75. Choose control kind.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0371`
- policyLane: `rules`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 127; sourceText: Control kind: rule-list.
- acceptedOptions: Rule List
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-update-command-update-protocol

#### screen-update-command-update-protocol-commands

76. Support screen-policy.preview.requested?

- settingId: `screen-update-command-update-protocol-commands-0410`
- policyLane: `rules`; sectionId: `screen-update-command-update-protocol`; groupId: `screen-update-command-update-protocol-commands`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1434; sourceText: screen-policy.preview.requested: Portal asks whether proposed changes validate and what effective policy would result.
- acceptedOptions: Screen Policy.preview.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-agent-rule-update-protocol

#### screen-agent-rule-update-protocol-agent-rules

77. Choose agent rule validateFullPolicyAfterPatch.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0415`
- policyLane: `rules`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1533; sourceText: Agent rule validateFullPolicyAfterPatch: true.
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

78. Choose agent rule compileFullEffectivePolicyAfterEveryAcceptedChange.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0416`
- policyLane: `rules`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1534; sourceText: Agent rule compileFullEffectivePolicyAfterEveryAcceptedChange: true.
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

79. Choose agent rule runCaptureOnlyInChildAgent.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0420`
- policyLane: `rules`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1538; sourceText: Agent rule runCaptureOnlyInChildAgent: true.
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

80. Choose agent rule enforceLocallyWhenPortalOffline.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0422`
- policyLane: `rules`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1540; sourceText: Agent rule enforceLocallyWhenPortalOffline: true.
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

81. Choose agent rule rejectUnknownPaths.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0423`
- policyLane: `rules`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1541; sourceText: Agent rule rejectUnknownPaths: true.
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

82. Choose agent rule rejectInvalidEnumValues.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0424`
- policyLane: `rules`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1542; sourceText: Agent rule rejectInvalidEnumValues: true.
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

83. Choose agent rule rejectHostedProcessingForSchemaVersionOne.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0427`
- policyLane: `rules`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1545; sourceText: Agent rule rejectHostedProcessingForSchemaVersionOne: true.
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

84. Choose agent rule recordSkippedAttemptsAsCapabilityEvents.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0431`
- policyLane: `rules`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1549; sourceText: Agent rule recordSkippedAttemptsAsCapabilityEvents: true.
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-visible-category-target-effective-policy-document

#### screen-visible-category-target-effective-policy-document-visible-category-policy-targets

85. Choose visible category target school.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0467`
- policyLane: `rules`; sectionId: `screen-visible-category-target-effective-policy-document`; groupId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1375; sourceText: Visible category target school: default action observe.
- acceptedOptions: Observe
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

86. Choose visible category target video.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0468`
- policyLane: `rules`; sectionId: `screen-visible-category-target-effective-policy-document`; groupId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1378; sourceText: Visible category target video: default action observe.
- acceptedOptions: Observe
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

87. Choose visible category target chat.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0469`
- policyLane: `rules`; sectionId: `screen-visible-category-target-effective-policy-document`; groupId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1381; sourceText: Visible category target chat: default action observe.
- acceptedOptions: Observe
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

88. Choose visible category target game.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0470`
- policyLane: `rules`; sectionId: `screen-visible-category-target-effective-policy-document`; groupId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1384; sourceText: Visible category target game: default action observe.
- acceptedOptions: Observe
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

89. Choose visible category target adult-content.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0471`
- policyLane: `rules`; sectionId: `screen-visible-category-target-effective-policy-document`; groupId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1387; sourceText: Visible category target adult-content: default action ask.
- acceptedOptions: Ask
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

90. Choose visible category target violence.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0472`
- policyLane: `rules`; sectionId: `screen-visible-category-target-effective-policy-document`; groupId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1390; sourceText: Visible category target violence: default action ask.
- acceptedOptions: Ask
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

91. Choose visible category target bypass-tool.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0473`
- policyLane: `rules`; sectionId: `screen-visible-category-target-effective-policy-document`; groupId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1393; sourceText: Visible category target bypass-tool: default action warn.
- acceptedOptions: Warn
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

92. Choose visible category target unknown.

- settingId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets-0474`
- policyLane: `rules`; sectionId: `screen-visible-category-target-effective-policy-document`; groupId: `screen-visible-category-target-effective-policy-document-visible-category-policy-targets`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1396; sourceText: Visible category target unknown: default action ask.
- acceptedOptions: Ask
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

## Tab: schedule

### screen-capability-guide-bullet-core-terms

#### screen-capability-guide-bullet-core-terms-evidence-reference

93. Represent local model/runtime status;.

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0025`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-evidence-reference`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 102; sourceText: local model/runtime status;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-the-main-capability-truth

#### screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth

94. Use the parent explicitly enabled screen analysis for the child/device/schedule;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0028`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 116; sourceText: the parent explicitly enabled screen analysis for the child/device/schedule;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-screenshot-possibilities-and-limits

#### screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible

95. Use one-time manual parent test capture during setup;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0047`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 166; sourceText: one-time manual parent test capture during setup;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

96. Use cadence capture with conservative intervals;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0048`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible`
- cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 167; sourceText: cadence capture with conservative intervals;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

97. Use event-triggered capture after foreground app change, managed URL change, app/game foreground start, unusual network digest, policy ambiguity, or local AI uncertainty;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0049`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 168; sourceText: event-triggered capture after foreground app change, managed URL change, app/game foreground start, unusual network digest, policy ambiguity, or local AI uncertainty;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-screen-recording-possibilities-and-limits

#### screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits

98. Use event-triggered frame sampling for transitions;?

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0061`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 194; sourceText: event-triggered frame sampling for transitions;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

99. Use bandwidth, CPU/GPU, battery, and model runtime load are higher;?

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0066`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 202; sourceText: bandwidth, CPU/GPU, battery, and model runtime load are higher;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-ocr-and-image-classification

#### screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification

100.  Use model/runtime ref and prompt/template version;?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0087`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 246; sourceText: model/runtime ref and prompt/template version;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-triggers-and-scheduling

#### screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling

101.  Use disabled by default;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0092`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `disabled`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 277; sourceText: disabled by default;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

102.  Use conservative interval such as several minutes;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0093`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 278; sourceText: conservative interval such as several minutes;
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

103.  Use stricter shorter interval only when explicitly enabled;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0094`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 279; sourceText: stricter shorter interval only when explicitly enabled;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

104.  Use schedule-aware capture windows;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0095`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 280; sourceText: schedule-aware capture windows;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

105.  Use pause during sleep, lock, protected surface, permission-required state, or battery/resource pressure?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0096`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 281; sourceText: pause during sleep, lock, protected surface, permission-required state, or battery/resource pressure.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

106.  Use foreground app change;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0097`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 285; sourceText: foreground app change;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

107.  Use active window change;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0098`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 286; sourceText: active window change;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

108.  Use managed browser URL change;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0099`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 287; sourceText: managed browser URL change;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

109.  Use app/game foreground start;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0100`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 288; sourceText: app/game foreground start;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

110.  Use unusual network digest;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0101`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 289; sourceText: unusual network digest;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

111.  Use policy ambiguity;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0102`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 290; sourceText: policy ambiguity;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

112.  Use local AI uncertainty;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0103`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 291; sourceText: local AI uncertainty;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

113.  Use child ask-parent flow;?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0104`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 292; sourceText: child ask-parent flow;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

114.  Use manual parent setup/test capture?

- settingId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling-0105`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-triggers-and-scheduling`; groupId: `screen-capability-guide-bullet-triggers-and-scheduling-triggers-and-scheduling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 293; sourceText: manual parent setup/test capture.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-platform-capability-notes

#### screen-capability-guide-bullet-platform-capability-notes-windows

115.  Use capture support must be checked at runtime;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0149`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 383; sourceText: capture support must be checked at runtime;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados

116.  Choose screen Time frameworks.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0183`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 463; sourceText: Screen Time frameworks: Family Controls, Managed Settings, Device Activity;
- acceptedOptions: Family Controls | Managed Settings | Device Activity
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

117.  Use managed Settings shields and Device Activity schedules/events;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0185`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados`
- cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 465; sourceText: Managed Settings shields and Device Activity schedules/events;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

118.  Use web-domain and app/category usage controls through Screen Time tokens?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0186`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 466; sourceText: web-domain and app/category usage controls through Screen Time tokens.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

119.  Use screen Time APIs are privacy-preserving and entitlement/review-gated;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0189`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 472; sourceText: Screen Time APIs are privacy-preserving and entitlement/review-gated;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

120.  Use iOS child-device support should rely on approved Screen Time/Device Activity/Managed Settings paths rather than desktop-style pixel capture unless a specific Apple-approved capability is proven?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0190`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 473; sourceText: iOS child-device support should rely on approved Screen Time/Device Activity/Managed Settings paths rather than desktop-style pixel capture unless a specific Apple-approved capability is proven.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-app-and-window-correlation

#### screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation

121.  Represent local model/runtime status;.

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0195`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-app-and-window-correlation`; groupId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 484; sourceText: local model/runtime status;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-child-facing-disclosure

#### screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure

122.  Represent reason text for warnings, asks, blocks, or time limits;.

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0205`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-child-facing-disclosure`; groupId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 507; sourceText: reason text for warnings, asks, blocks, or time limits;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-capability-guide-bullet-parent-reports

#### screen-capability-guide-bullet-parent-reports-parent-reports

123.  Represent local model/runtime status;.

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0213`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 523; sourceText: local model/runtime status;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-custody-and-audit

#### screen-capability-guide-bullet-custody-and-audit-custody-and-audit

124.  Use capability state at capture time;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0219`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 539; sourceText: capability state at capture time;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

125.  Use local model/runtime ref;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0223`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 543; sourceText: local model/runtime ref;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-future-ui-rules

#### screen-capability-guide-bullet-future-ui-rules-future-ui-rules

126.  Use show cadence and triggers separately;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0246`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 584; sourceText: show cadence and triggers separately;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

127.  Use show local model/runtime status;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0248`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `local-ai-runtime`; capabilityState: `future-gap`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 586; sourceText: show local model/runtime status;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

128.  Use trigger-only capture;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0259`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 602; sourceText: trigger-only capture;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

129.  Use cadence plus trigger capture;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0260`
- policyLane: `schedule`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 603; sourceText: cadence plus trigger capture;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-matrix-row-capability-matrix

#### screen-capability-matrix-row-capability-matrix-prove-duration

130.  Represent capability Prove duration: full screen No, single point in time; active window No, single point in time; managed browser/window No, single point in time; local OCR/vision No; important limit Duration belongs to app/game/window/session evidence or recording-specific proof..

- settingId: `screen-capability-matrix-row-capability-matrix-prove-duration-0269`
- policyLane: `schedule`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-prove-duration`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 145; sourceText: Capability Prove duration: full screen No, single point in time; active window No, single point in time; managed browser/window No, single point in time; local OCR/vision No; important limit Duration belongs to app/game/window/session evidence or recording-specific proof..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-proposal-overview

#### screen-schema-proposal-bullet-proposal-overview-proposal-overview

131.  Use no naked domain strings in app/runtime code?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0290`
- policyLane: `schedule`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 26; sourceText: No naked domain strings in app/runtime code.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-implementation-notes-for-worker

#### screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker

132.  Use keep authoring manifest ids, field ids, section ids, option ids, policy ids, rule ids, schedule ids, trigger ids, queue job ids, result ids, capability ids, custody labels, and evidence refs branded?

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0295`
- policyLane: `schedule`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1675; sourceText: Keep authoring manifest ids, field ids, section ids, option ids, policy ids, rule ids, schedule ids, trigger ids, queue job ids, result ids, capability ids, custody labels, and evidence refs branded.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

133.  Use treat the authoring manifest as UI guidance only. Runtime capture, queue, analysis, policy, and enforcement must rely on validated policy and compiled effective policy?

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0301`
- policyLane: `schedule`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1683; sourceText: Treat the authoring manifest as UI guidance only. Runtime capture, queue, analysis, policy, and enforcement must rely on validated policy and compiled effective policy.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

134.  Use add explicit tests for hidden/visible branch behavior so UI cannot show cadence, OCR snippet storage, strict mode, or enforcement eligibility controls when screen analysis is disabled?

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0303`
- policyLane: `schedule`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1686; sourceText: Add explicit tests for hidden/visible branch behavior so UI cannot show cadence, OCR snippet storage, strict mode, or enforcement eligibility controls when screen analysis is disabled.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-scheduling

#### screen-authoring-field-authoring-manifest-scheduling-scheduling-fields

135.  Enable scheduled capture?

- settingId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields-0321`
- policyLane: `schedule`; sectionId: `screen-authoring-field-authoring-manifest-scheduling`; groupId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 374; sourceText: Enable scheduled capture?
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

136.  How often may scheduled capture run?

- settingId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields-0322`
- policyLane: `schedule`; sectionId: `screen-authoring-field-authoring-manifest-scheduling`; groupId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields`
- cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 381; sourceText: How often may scheduled capture run?
- acceptedOptions: Default 300 | Minimum 60 | Maximum 3600
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

137.  Allow the shortest supported cadence?

- settingId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields-0323`
- policyLane: `schedule`; sectionId: `screen-authoring-field-authoring-manifest-scheduling`; groupId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 394; sourceText: Allow the shortest supported cadence?
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-authoring-field-authoring-manifest-triggers

#### screen-authoring-field-authoring-manifest-triggers-triggers-fields

138.  Enable event-triggered capture?

- settingId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields-0326`
- policyLane: `schedule`; sectionId: `screen-authoring-field-authoring-manifest-triggers`; groupId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 447; sourceText: Enable event-triggered capture?
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

139.  Which events may request screen analysis?

- settingId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields-0327`
- policyLane: `schedule`; sectionId: `screen-authoring-field-authoring-manifest-triggers`; groupId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 454; sourceText: Which events may request screen analysis?
- acceptedOptions: Foreground App Change | Active Window Change | Managed Browser Url Change | App Game Foreground Start | Unusual Network Digest | Policy Ambiguity | Local Ai Uncertainty | Ask Parent Flow | Manual Parent Test Capture | Default foreground-app-change | Default managed-browser-url-change | Default policy-ambiguity | Default manual-parent-test-capture
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

140.  How long should repeated triggers wait?

- settingId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields-0328`
- policyLane: `schedule`; sectionId: `screen-authoring-field-authoring-manifest-triggers`; groupId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields`
- cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 481; sourceText: How long should repeated triggers wait?
- acceptedOptions: Default 120 | Minimum 15 | Maximum 900
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

141.  What is the maximum number of screen analysis jobs per hour?

- settingId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields-0329`
- policyLane: `schedule`; sectionId: `screen-authoring-field-authoring-manifest-triggers`; groupId: `screen-authoring-field-authoring-manifest-triggers-triggers-fields`
- cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 494; sourceText: What is the maximum number of screen analysis jobs per hour?
- acceptedOptions: Default 12 | Minimum 0 | Maximum 60
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-control-kind-authoring-manifest-metadata

#### screen-control-kind-authoring-manifest-metadata-control-kinds

142.  Choose control kind.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0368`
- policyLane: `schedule`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 124; sourceText: Control kind: schedule.
- acceptedOptions: Schedule
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-capability-registry-entry-capability-registry

#### screen-capability-registry-entry-capability-registry-capture-scope

143.  Represent capability windows-graphics-capture-full-screen: kind capture-scope; state manual-required; proof real-host-permission-and-capture-proof-required; affects fields capture.allowedScopes, schedule.cadenceCaptureEnabled.

- settingId: `screen-capability-registry-entry-capability-registry-capture-scope-0385`
- policyLane: `schedule`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-capture-scope`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1565; sourceText: Capability windows-graphics-capture-full-screen: kind capture-scope; state manual-required; proof real-host-permission-and-capture-proof-required; affects fields capture.allowedScopes, schedule.cadenceCaptureEnabled.
- acceptedOptions: Manual Required | Real Host Permission And Capture Proof Required | Capture.allowedScopes | Schedule.cadenceCaptureEnabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-registry-entry-capability-registry-source-correlation

144.  Represent capability managed-browser-window-correlation: kind source-correlation; state ready; proof runtime-read-model-required; affects fields capture.requireManagedBrowserCorrelationForWebClaims, policy.requireEvidenceRefs.

- settingId: `screen-capability-registry-entry-capability-registry-source-correlation-0387`
- policyLane: `schedule`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-source-correlation`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1579; sourceText: Capability managed-browser-window-correlation: kind source-correlation; state ready; proof runtime-read-model-required; affects fields capture.requireManagedBrowserCorrelationForWebClaims, policy.requireEvidenceRefs.
- acceptedOptions: Ready | Runtime Read Model Required | Capture.requireManagedBrowserCorrelationForWebClaims | Policy.requireEvidenceRefs
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

145.  Represent capability foreground-app-window-correlation: kind source-correlation; state ready; proof runtime-read-model-required; affects fields capture.requireAppWindowCorrelation, policy.requireEvidenceRefs.

- settingId: `screen-capability-registry-entry-capability-registry-source-correlation-0388`
- policyLane: `schedule`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-source-correlation`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1586; sourceText: Capability foreground-app-window-correlation: kind source-correlation; state ready; proof runtime-read-model-required; affects fields capture.requireAppWindowCorrelation, policy.requireEvidenceRefs.
- acceptedOptions: Ready | Runtime Read Model Required | Capture.requireAppWindowCorrelation | Policy.requireEvidenceRefs
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-registry-entry-capability-registry-local-analysis

146.  Represent capability local-ocr-runtime: kind local-analysis; state manual-required; proof local-model-runtime-proof-required; affects fields analysis.allowedTasks, analysis.ocrTextEnabled, analysis.minimumPolicyConfidence.

- settingId: `screen-capability-registry-entry-capability-registry-local-analysis-0390`
- policyLane: `schedule`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-local-analysis`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1605; sourceText: Capability local-ocr-runtime: kind local-analysis; state manual-required; proof local-model-runtime-proof-required; affects fields analysis.allowedTasks, analysis.ocrTextEnabled, analysis.minimumPolicyConfidence.
- acceptedOptions: Manual Required | Local Model Runtime Proof Required | Analysis.allowedTasks | Analysis.ocrTextEnabled | Analysis.minimumPolicyConfidence
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

147.  Represent capability local-vision-classifier: kind local-analysis; state manual-required; proof local-model-runtime-proof-required; affects fields analysis.allowedTasks, analysis.minimumPolicyConfidence, policy.allowedTargetTypes.

- settingId: `screen-capability-registry-entry-capability-registry-local-analysis-0391`
- policyLane: `schedule`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-local-analysis`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1612; sourceText: Capability local-vision-classifier: kind local-analysis; state manual-required; proof local-model-runtime-proof-required; affects fields analysis.allowedTasks, analysis.minimumPolicyConfidence, policy.allowedTargetTypes.
- acceptedOptions: Manual Required | Local Model Runtime Proof Required | Analysis.allowedTasks | Analysis.minimumPolicyConfidence | Policy.allowedTargetTypes
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-registry-entry-capability-registry-redaction

148.  Represent capability screen-redaction-runtime: kind redaction; state manual-required; proof redaction-validation-required; affects fields redaction.mode, redaction.neverStore, redaction.whenUnavailable.

- settingId: `screen-capability-registry-entry-capability-registry-redaction-0392`
- policyLane: `schedule`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-redaction`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1619; sourceText: Capability screen-redaction-runtime: kind redaction; state manual-required; proof redaction-validation-required; affects fields redaction.mode, redaction.neverStore, redaction.whenUnavailable.
- acceptedOptions: Manual Required | Redaction Validation Required | Redaction.mode | Redaction.neverStore | Redaction.whenUnavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-registry-entry-capability-registry-platform-policy

149.  Represent capability ios-screentime-managed-settings: kind platform-policy; state manual-required; proof apple-entitlement-and-device-proof-required; affects fields policy.allowedTargetTypes, reports.visibleFields.

- settingId: `screen-capability-registry-entry-capability-registry-platform-policy-0396`
- policyLane: `schedule`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-platform-policy`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1647; sourceText: Capability ios-screentime-managed-settings: kind platform-policy; state manual-required; proof apple-entitlement-and-device-proof-required; affects fields policy.allowedTargetTypes, reports.visibleFields.
- acceptedOptions: Manual Required | Apple Entitlement And Device Proof Required | Policy.allowedTargetTypes | Reports.visibleFields
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-state-meaning-capability-registry

#### screen-capability-state-meaning-capability-registry-capability-state-meanings

150.  Show ready capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0397`
- policyLane: `schedule`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1655; sourceText: ready: Runtime reports the capability can be used within the configured boundary, subject to per-attempt checks.
- acceptedOptions: Ready
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

151.  Show model-unavailable capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0404`
- policyLane: `schedule`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1662; sourceText: model-unavailable: Local OCR/vision runtime is missing, disabled, loading, failed, or overloaded.
- acceptedOptions: Model Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-agent-rule-update-protocol

#### screen-agent-rule-update-protocol-agent-rules

152.  Choose agent rule persistPolicyBeforeSchedulerSwitch.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0417`
- policyLane: `schedule`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1535; sourceText: Agent rule persistPolicyBeforeSchedulerSwitch: true.
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

## Tab: approvals

### screen-policy-fallback-policy-value-document

#### screen-policy-fallback-policy-value-document-fallbacks

153.  Choose fallback lowConfidence.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0442`
- policyLane: `approvals`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1212; sourceText: Fallback lowConfidence: ask-parent.
- acceptedOptions: Ask Parent
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

154.  Choose fallback policyUse.lowConfidenceFallback.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0446`
- policyLane: `approvals`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1020; sourceText: Fallback policyUse.lowConfidenceFallback: ask-parent.
- acceptedOptions: Ask Parent
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

## Tab: enforcement

### screen-capability-guide-bullet-screenshot-possibilities-and-limits

#### screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible

155.  Use risk signals such as possible credential prompt, explicit content signal, bypass tool, unsafe visible content, self-harm signal, suspicious login, or unknown;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0045`
- policyLane: `enforcement`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 162; sourceText: risk signals such as possible credential prompt, explicit content signal, bypass tool, unsafe visible content, self-harm signal, suspicious login, or unknown;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

156.  Represent correlation with foreground app, active window title, managed browser state, app/game session, and network digest refs;.

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0046`
- policyLane: `enforcement`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 164; sourceText: correlation with foreground app, active window title, managed browser state, app/game session, and network digest refs;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable

157.  Represent exact active browser URL unless managed browser evidence proves it;.

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0051`
- policyLane: `enforcement`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 176; sourceText: exact active browser URL unless managed browser evidence proves it;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

158.  Use what the child typed before or after the frame;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0052`
- policyLane: `enforcement`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 177; sourceText: what the child typed before or after the frame;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

159.  Use duration of use;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0053`
- policyLane: `enforcement`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable`
- cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 178; sourceText: duration of use;
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

160.  Use whether visible text came from a webpage, chat, image, ad, overlay, or stale window;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0054`
- policyLane: `enforcement`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 179; sourceText: whether visible text came from a webpage, chat, image, ad, overlay, or stale window;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

161.  Use hidden background tabs or background apps;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0055`
- policyLane: `enforcement`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 180; sourceText: hidden background tabs or background apps;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

162.  Use decrypted network content;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0056`
- policyLane: `enforcement`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 181; sourceText: decrypted network content;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

163.  Use app identity without OS process/window correlation;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable-0057`
- policyLane: `enforcement`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-not-reliable`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 182; sourceText: app identity without OS process/window correlation;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-screen-recording-possibilities-and-limits

#### screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits

164.  Use recording creates more raw sensitive data than screenshots;?

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0063`
- policyLane: `enforcement`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 199; sourceText: recording creates more raw sensitive data than screenshots;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

## Tab: audit

### screen-capability-guide-bullet-core-terms

#### screen-capability-guide-bullet-core-terms-evidence-reference

165.  Represent policy or AI decision that consumed the summary.

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0027`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-evidence-reference`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 104; sourceText: policy or AI decision that consumed the summary.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-the-main-capability-truth

#### screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth

166.  Represent platform parity before real OS/device proof;.

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0041`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 132; sourceText: platform parity before real OS/device proof;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-screenshot-possibilities-and-limits

#### screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible

167.  Use queue lifecycle and deletion proof?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0050`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 170; sourceText: queue lifecycle and deletion proof.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-platform-capability-notes

#### screen-capability-guide-bullet-platform-capability-notes-windows

168.  Represent product claims should follow real host proof, not contract presence.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0153`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 389; sourceText: product claims should follow real host proof, not contract presence.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-macos

169.  Use macOS parity requires real host proof, not package scaffold proof?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0161`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 409; sourceText: macOS parity requires real host proof, not package scaffold proof.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

#### screen-capability-guide-bullet-platform-capability-notes-linux

170.  Represent foreground-window proof varies by compositor and desktop environment;.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0170`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-linux`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 430; sourceText: foreground-window proof varies by compositor and desktop environment;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

171.  Represent linux support needs distro/backend-specific proof.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0171`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-linux`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 431; sourceText: Linux support needs distro/backend-specific proof.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-capability-guide-bullet-parent-reports

#### screen-capability-guide-bullet-parent-reports-parent-reports

172.  Represent policy/AI decisions that consumed the screen summary;.

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0216`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 526; sourceText: policy/AI decisions that consumed the screen summary;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-custody-and-audit

#### screen-capability-guide-bullet-custody-and-audit-custody-and-audit

173.  Use parent setting version and actor ref;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0218`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 538; sourceText: parent setting version and actor ref;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

174.  Use capture reason and scope;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0220`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 540; sourceText: capture reason and scope;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

175.  Use queue job id and image digest;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0221`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 541; sourceText: queue job id and image digest;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

176.  Use encryption and deletion lifecycle;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0222`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 542; sourceText: encryption and deletion lifecycle;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

177.  Use validation result;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0224`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 544; sourceText: validation result;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

178.  Represent summary/result id;.

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0225`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 545; sourceText: summary/result id;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

179.  Use policy decision id;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0226`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 546; sourceText: policy decision id;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

180.  Use enforcement result if any;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0227`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 547; sourceText: enforcement result if any;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

181.  Use custody label;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0228`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 548; sourceText: custody label;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

182.  Use retention/deletion state;?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0229`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 549; sourceText: retention/deletion state;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

183.  Use adapter errors or permission changes?

- settingId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit-0230`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-custody-and-audit`; groupId: `screen-capability-guide-bullet-custody-and-audit-custody-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 550; sourceText: adapter errors or permission changes.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-proof-requirements

#### screen-capability-guide-bullet-proof-requirements-proof-requirements

184.  Use parent setting enabled through typed contracts;?

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0231`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 559; sourceText: parent setting enabled through typed contracts;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

185.  Represent child-device agent or service detects capability and permission state;.

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0232`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 560; sourceText: child-device agent or service detects capability and permission state;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

186.  Use capture occurs only inside approved scope;?

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0233`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 561; sourceText: capture occurs only inside approved scope;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

187.  Use image/frame enters encrypted temp queue;?

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0234`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 562; sourceText: image/frame enters encrypted temp queue;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

188.  Use local OCR/vision analyzes it;?

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0235`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 563; sourceText: local OCR/vision analyzes it;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

189.  Use schema validation accepts/rejects output correctly;?

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0236`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 564; sourceText: schema validation accepts/rejects output correctly;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

190.  Use raw image deletes after success or TTL expiry;?

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0237`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 565; sourceText: raw image deletes after success or TTL expiry;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

191.  Represent journal and SQLite expose summary/read model;.

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0238`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 566; sourceText: journal and SQLite expose summary/read model;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

192.  Use portal renders settings, status, summary, refs, custody, and deletion state;?

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0239`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 567; sourceText: portal renders settings, status, summary, refs, custody, and deletion state;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

193.  Use no Ocentra-hosted upload happens by default;?

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0240`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 568; sourceText: no Ocentra-hosted upload happens by default;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

194.  Use protected/permission-required cases are visible as unavailable, not fake success?

- settingId: `screen-capability-guide-bullet-proof-requirements-proof-requirements-0241`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-proof-requirements`; groupId: `screen-capability-guide-bullet-proof-requirements-proof-requirements`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 569; sourceText: protected/permission-required cases are visible as unavailable, not fake success.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-future-ui-rules

#### screen-capability-guide-bullet-future-ui-rules-future-ui-rules

195.  Use show exact proof requirement before screen-derived rules can enforce;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0252`
- policyLane: `audit`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 592; sourceText: show exact proof requirement before screen-derived rules can enforce;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-matrix-row-capability-matrix

#### screen-capability-matrix-row-capability-matrix-capture-a-still-image

196.  Represent capability Capture a still image: full screen Possible on desktop platforms with permission/proof; active window Possible where OS exposes window capture; managed browser/window Possible if the managed boundary is active; local OCR/vision Input only after capture; important limit Must skip protected/locked/permission-required states..

- settingId: `screen-capability-matrix-row-capability-matrix-capture-a-still-image-0263`
- policyLane: `audit`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-capture-a-still-image`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 139; sourceText: Capability Capture a still image: full screen Possible on desktop platforms with permission/proof; active window Possible where OS exposes window capture; managed browser/window Possible if the managed boundary is active; local OCR/vision Input only after capture; important limit Must skip protected/locked/permission-required states..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-matrix-row-capability-matrix-capture-a-recording-stream

197.  Represent capability Capture a recording stream: full screen Possible but high-sensitivity; active window Possible where OS supports selected window/app stream; managed browser/window Possible if managed scope is selected; local OCR/vision Usually sampled into frames or summaries; important limit Not default; needs stronger opt-in and proof..

- settingId: `screen-capability-matrix-row-capability-matrix-capture-a-recording-stream-0264`
- policyLane: `audit`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-capture-a-recording-stream`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 140; sourceText: Capability Capture a recording stream: full screen Possible but high-sensitivity; active window Possible where OS supports selected window/app stream; managed browser/window Possible if managed scope is selected; local OCR/vision Usually sampled into frames or summaries; important limit Not default; needs stronger opt-in and proof..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-matrix-row-capability-matrix-prove-exact-url

198.  Represent capability Prove exact URL: full screen No; active window No; managed browser/window Only if browser evidence proves it separately; local OCR/vision No; important limit Pixels can show text that looks like a URL, but that is not managed tab proof..

- settingId: `screen-capability-matrix-row-capability-matrix-prove-exact-url-0267`
- policyLane: `audit`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-prove-exact-url`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: managed-browser-evidence-required
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 143; sourceText: Capability Prove exact URL: full screen No; active window No; managed browser/window Only if browser evidence proves it separately; local OCR/vision No; important limit Pixels can show text that looks like a URL, but that is not managed tab proof..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-matrix-row-capability-matrix-feed-enforcement

199.  Represent capability Feed enforcement: full screen Not directly; active window Not directly; managed browser/window Not directly; local OCR/vision Not directly; important limit Enforcement requires typed policy decision and audit..

- settingId: `screen-capability-matrix-row-capability-matrix-feed-enforcement-0273`
- policyLane: `audit`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-feed-enforcement`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 149; sourceText: Capability Feed enforcement: full screen Not directly; active window Not directly; managed browser/window Not directly; local OCR/vision Not directly; important limit Enforcement requires typed policy decision and audit..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-proposal-overview

#### screen-schema-proposal-bullet-proposal-overview-proposal-overview

200.  Represent local child-agent persistence, queue encryption/deletion, compile, rollback, and audit behavior.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0293`
- policyLane: `audit`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 30; sourceText: Local child-agent persistence, queue encryption/deletion, compile, rollback, and audit behavior.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-implementation-notes-for-worker

#### screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker

201.  Choose reject partial states. For example, `policyUse.enabled.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0300`
- policyLane: `audit`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1681; sourceText: Reject partial states. For example, `policyUse.enabled: true` requires a valid confidence threshold, deletion proof requirement, evidence refs, and fallback behavior.
- acceptedOptions: True` Requires A Valid Confidence Threshold | Deletion Proof Requirement | Evidence Refs | And Fallback Behavior
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-policy-use

#### screen-authoring-field-authoring-manifest-policy-use-policy-use-fields

202.  What if screen proof is unavailable?

- settingId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields-0348`
- policyLane: `audit`; sectionId: `screen-authoring-field-authoring-manifest-policy-use`; groupId: `screen-authoring-field-authoring-manifest-policy-use-policy-use-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 743; sourceText: What if screen proof is unavailable?
- acceptedOptions: Allow | Observe | Warn | Ask | Block Until Ready | Mark Unavailable | Default mark-unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-audit

#### screen-authoring-field-authoring-manifest-audit-audit-fields

203.  Which audit fields are required?

- settingId: `screen-authoring-field-authoring-manifest-audit-audit-fields-0352`
- policyLane: `audit`; sectionId: `screen-authoring-field-authoring-manifest-audit`; groupId: `screen-authoring-field-authoring-manifest-audit-audit-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 834; sourceText: Which audit fields are required?
- acceptedOptions: Parent Setting Version | Capability State | Capture Reason | Capture Scope | Queue Job Id | Image Digest | Local Model Runtime | Validation Result | Deletion State | Custody Label | Policy Decision Ref | Enforcement Result Ref | Adapter Error | Permission State | Default parent-setting-version | Default capability-state | Default capture-reason | Default queue-job-id | Default image-digest | Default local-model-runtime | Default validation-result | Default deletion-state | Default custody-label | Default policy-decision-ref
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

204.  Audit every capture attempt, including skipped attempts?

- settingId: `screen-authoring-field-authoring-manifest-audit-audit-fields-0353`
- policyLane: `audit`; sectionId: `screen-authoring-field-authoring-manifest-audit`; groupId: `screen-authoring-field-authoring-manifest-audit-audit-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 868; sourceText: Audit every capture attempt, including skipped attempts?
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

205.  Audit every delete-pending or delete-failed state?

- settingId: `screen-authoring-field-authoring-manifest-audit-audit-fields-0354`
- policyLane: `audit`; sectionId: `screen-authoring-field-authoring-manifest-audit`; groupId: `screen-authoring-field-authoring-manifest-audit-audit-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 875; sourceText: Audit every delete-pending or delete-failed state?
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-condition-kind-authoring-manifest-metadata

#### screen-condition-kind-authoring-manifest-metadata-condition-kinds

206.  Represent condition kind: proofAtLeast.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0382`
- policyLane: `audit`; sectionId: `screen-condition-kind-authoring-manifest-metadata`; groupId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 140; sourceText: Condition kind: proofAtLeast.
- acceptedOptions: ProofAtLeast
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-capability-registry-entry-capability-registry

#### screen-capability-registry-entry-capability-registry-capture-scope

207.  Represent capability windows-graphics-capture-active-window: kind capture-scope; state manual-required; proof real-host-permission-and-capture-proof-required; affects fields capture.allowedScopes, capture.defaultScope, policy.allowedTargetTypes.

- settingId: `screen-capability-registry-entry-capability-registry-capture-scope-0384`
- policyLane: `audit`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-capture-scope`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1558; sourceText: Capability windows-graphics-capture-active-window: kind capture-scope; state manual-required; proof real-host-permission-and-capture-proof-required; affects fields capture.allowedScopes, capture.defaultScope, policy.allowedTargetTypes.
- acceptedOptions: Manual Required | Real Host Permission And Capture Proof Required | Capture.allowedScopes | Capture.defaultScope | Policy.allowedTargetTypes
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-registry-entry-capability-registry-recording

208.  Represent capability windows-graphics-capture-recording-stream: kind recording; state manual-required; proof real-host-recording-stream-retention-and-deletion-proof-required; affects fields recording.mode, recording.maxSegmentSeconds, recording.frameSamplingMode.

- settingId: `screen-capability-registry-entry-capability-registry-recording-0386`
- policyLane: `audit`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-recording`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1572; sourceText: Capability windows-graphics-capture-recording-stream: kind recording; state manual-required; proof real-host-recording-stream-retention-and-deletion-proof-required; affects fields recording.mode, recording.maxSegmentSeconds, recording.frameSamplingMode.
- acceptedOptions: Manual Required | Real Host Recording Stream Retention And Deletion Proof Required | Recording.mode | Recording.maxSegmentSeconds | Recording.frameSamplingMode
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-registry-entry-capability-registry-queue

209.  Represent capability encrypted-screen-temp-queue: kind queue; state ready; proof queue-encryption-deletion-tests-required; affects fields queue.temporaryImageTtlSeconds, queue.maxRetryCount, queue.deleteAfterSuccess, queue.deleteAfterExpiry.

- settingId: `screen-capability-registry-entry-capability-registry-queue-0389`
- policyLane: `audit`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-queue`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1593; sourceText: Capability encrypted-screen-temp-queue: kind queue; state ready; proof queue-encryption-deletion-tests-required; affects fields queue.temporaryImageTtlSeconds, queue.maxRetryCount, queue.deleteAfterSuccess, queue.deleteAfterExpiry.
- acceptedOptions: Ready | Queue Encryption Deletion Tests Required | Queue.temporaryImageTtlSeconds | Queue.maxRetryCount | Queue.deleteAfterSuccess | Queue.deleteAfterExpiry
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-registry-entry-capability-registry-platform-capture

210.  Represent capability macos-screencapturekit: kind platform-capture; state manual-required; proof macos-host-screen-recording-permission-proof-required; affects fields capture.allowedScopes.

- settingId: `screen-capability-registry-entry-capability-registry-platform-capture-0393`
- policyLane: `audit`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-platform-capture`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1626; sourceText: Capability macos-screencapturekit: kind platform-capture; state manual-required; proof macos-host-screen-recording-permission-proof-required; affects fields capture.allowedScopes.
- acceptedOptions: Manual Required | Macos Host Screen Recording Permission Proof Required | Capture.allowedScopes
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

211.  Represent capability linux-xdg-desktop-portal-screencast: kind platform-capture; state manual-required; proof distro-desktop-portal-pipewire-proof-required; affects fields capture.allowedScopes.

- settingId: `screen-capability-registry-entry-capability-registry-platform-capture-0394`
- policyLane: `audit`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-platform-capture`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1633; sourceText: Capability linux-xdg-desktop-portal-screencast: kind platform-capture; state manual-required; proof distro-desktop-portal-pipewire-proof-required; affects fields capture.allowedScopes.
- acceptedOptions: Manual Required | Distro Desktop Portal Pipewire Proof Required | Capture.allowedScopes
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

212.  Represent capability android-media-projection: kind platform-capture; state manual-required; proof android-user-consent-foreground-service-proof-required; affects fields capture.allowedScopes, screen.requiredDisclosure, recording.mode.

- settingId: `screen-capability-registry-entry-capability-registry-platform-capture-0395`
- policyLane: `audit`; sectionId: `screen-capability-registry-entry-capability-registry`; groupId: `screen-capability-registry-entry-capability-registry-platform-capture`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1640; sourceText: Capability android-media-projection: kind platform-capture; state manual-required; proof android-user-consent-foreground-service-proof-required; affects fields capture.allowedScopes, screen.requiredDisclosure, recording.mode.
- acceptedOptions: Manual Required | Android User Consent Foreground Service Proof Required | Capture.allowedScopes | Screen.requiredDisclosure | Recording.mode
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-state-meaning-capability-registry

#### screen-capability-state-meaning-capability-registry-capability-state-meanings

213.  Show adapter-error capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0407`
- policyLane: `audit`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1665; sourceText: adapter-error: The platform adapter failed and must record an audit result.
- acceptedOptions: Adapter Error
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

214.  Show manual-required capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0408`
- policyLane: `audit`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `manual-proof`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1666; sourceText: manual-required: Contracts can represent the setting, but product support requires real host/device proof.
- acceptedOptions: Manual Required
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-agent-rule-update-protocol

#### screen-agent-rule-update-protocol-agent-rules

215.  Choose agent rule rejectPolicyUseWithoutDeletionProof.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0428`
- policyLane: `audit`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1546; sourceText: Agent rule rejectPolicyUseWithoutDeletionProof: true.
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-policy-fallback-policy-value-document

#### screen-policy-fallback-policy-value-document-fallbacks

216.  Choose fallback protectedSurface.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0436`
- policyLane: `audit`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1206; sourceText: Fallback protectedSurface: skip-and-audit.
- acceptedOptions: Skip And Audit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

217.  Choose fallback screenLocked.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0437`
- policyLane: `audit`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1207; sourceText: Fallback screenLocked: skip-and-audit.
- acceptedOptions: Skip And Audit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-effective-proof-requirement-effective-policy-document

#### screen-effective-proof-requirement-effective-policy-document-proof-requirements

218.  Represent proof requirement screenPolicyUse: validated-screen-summary-with-deleted-image.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0450`
- policyLane: `audit`; sectionId: `screen-effective-proof-requirement-effective-policy-document`; groupId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1318; sourceText: Proof requirement screenPolicyUse: validated-screen-summary-with-deleted-image.
- acceptedOptions: Validated Screen Summary With Deleted Image
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

219.  Represent proof requirement exactWebClaims: managed-browser-evidence-required.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0451`
- policyLane: `audit`; sectionId: `screen-effective-proof-requirement-effective-policy-document`; groupId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1319; sourceText: Proof requirement exactWebClaims: managed-browser-evidence-required.
- acceptedOptions: Managed Browser Evidence Required
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

220.  Represent proof requirement appWindowClaims: foreground-app-window-evidence-required.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0452`
- policyLane: `audit`; sectionId: `screen-effective-proof-requirement-effective-policy-document`; groupId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1320; sourceText: Proof requirement appWindowClaims: foreground-app-window-evidence-required.
- acceptedOptions: Foreground App Window Evidence Required
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

221.  Represent proof requirement riskSignalRules: validated-screen-summary-confidence-threshold.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0453`
- policyLane: `audit`; sectionId: `screen-effective-proof-requirement-effective-policy-document`; groupId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1321; sourceText: Proof requirement riskSignalRules: validated-screen-summary-confidence-threshold.
- acceptedOptions: Validated Screen Summary Confidence Threshold
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

222.  Represent proof requirement enforcementEligibility: typed-policy-decision-with-screen-evidence-ref.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0454`
- policyLane: `audit`; sectionId: `screen-effective-proof-requirement-effective-policy-document`; groupId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1322; sourceText: Proof requirement enforcementEligibility: typed-policy-decision-with-screen-evidence-ref.
- acceptedOptions: Typed Policy Decision With Screen Evidence Ref
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

223.  Represent proof requirement reportOnly: stale-or-degraded-allowed.

- settingId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements-0455`
- policyLane: `audit`; sectionId: `screen-effective-proof-requirement-effective-policy-document`; groupId: `screen-effective-proof-requirement-effective-policy-document-proof-requirements`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1323; sourceText: Proof requirement reportOnly: stale-or-degraded-allowed.
- acceptedOptions: Stale Or Degraded Allowed
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-effective-fallback-effective-policy-document

#### screen-effective-fallback-effective-policy-document-fallback-decisions

224.  Choose effective fallback proofUnavailable.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0456`
- policyLane: `audit`; sectionId: `screen-effective-fallback-effective-policy-document`; groupId: `screen-effective-fallback-effective-policy-document-fallback-decisions`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1326; sourceText: Effective fallback proofUnavailable: mark-unavailable.
- acceptedOptions: Mark Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

225.  Choose effective fallback staleEvidence.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0457`
- policyLane: `audit`; sectionId: `screen-effective-fallback-effective-policy-document`; groupId: `screen-effective-fallback-effective-policy-document-fallback-decisions`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1327; sourceText: Effective fallback staleEvidence: report-only.
- acceptedOptions: Report Only
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

226.  Choose effective fallback screenLocked.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0458`
- policyLane: `audit`; sectionId: `screen-effective-fallback-effective-policy-document`; groupId: `screen-effective-fallback-effective-policy-document-fallback-decisions`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1207; sourceText: Effective fallback screenLocked: skip-and-audit.
- acceptedOptions: Skip And Audit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

227.  Choose effective fallback protectedSurface.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0459`
- policyLane: `audit`; sectionId: `screen-effective-fallback-effective-policy-document`; groupId: `screen-effective-fallback-effective-policy-document-fallback-decisions`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1206; sourceText: Effective fallback protectedSurface: skip-and-audit.
- acceptedOptions: Skip And Audit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

228.  Choose effective fallback modelUnavailable.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0460`
- policyLane: `audit`; sectionId: `screen-effective-fallback-effective-policy-document`; groupId: `screen-effective-fallback-effective-policy-document-fallback-decisions`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1209; sourceText: Effective fallback modelUnavailable: retry-within-ttl-then-delete.
- acceptedOptions: Retry Within Ttl Then Delete
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

229.  Choose effective fallback queueUnavailable.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0461`
- policyLane: `audit`; sectionId: `screen-effective-fallback-effective-policy-document`; groupId: `screen-effective-fallback-effective-policy-document-fallback-decisions`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1208; sourceText: Effective fallback queueUnavailable: fail-closed.
- acceptedOptions: Fail Closed
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

230.  Choose effective fallback adapterError.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0462`
- policyLane: `audit`; sectionId: `screen-effective-fallback-effective-policy-document`; groupId: `screen-effective-fallback-effective-policy-document-fallback-decisions`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1332; sourceText: Effective fallback adapterError: mark-degraded-and-audit.
- acceptedOptions: Mark Degraded And Audit
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

231.  Choose effective fallback deleteFailed.

- settingId: `screen-effective-fallback-effective-policy-document-fallback-decisions-0463`
- policyLane: `audit`; sectionId: `screen-effective-fallback-effective-policy-document`; groupId: `screen-effective-fallback-effective-policy-document-fallback-decisions`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1213; sourceText: Effective fallback deleteFailed: surface-health-and-retry.
- acceptedOptions: Surface Health And Retry
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-effective-rule-effective-policy-document

#### screen-effective-rule-effective-policy-document-rules-in-priority-order

232.  Choose effective rule parent-request-explicit-content-signal.

- settingId: `screen-effective-rule-effective-policy-document-rules-in-priority-order-0464`
- policyLane: `audit`; sectionId: `screen-effective-rule-effective-policy-document`; groupId: `screen-effective-rule-effective-policy-document-rules-in-priority-order`
- cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1026; sourceText: Effective rule parent-request-explicit-content-signal: priority 100; decision ask; target risk-signal; proof validated-screen-summary-with-deleted-image; minimum confidence 0.8.
- acceptedOptions: Ask | Risk Signal | Validated Screen Summary With Deleted Image
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

233.  Choose effective rule warn-bypass-tool-visible.

- settingId: `screen-effective-rule-effective-policy-document-rules-in-priority-order-0465`
- policyLane: `audit`; sectionId: `screen-effective-rule-effective-policy-document`; groupId: `screen-effective-rule-effective-policy-document-rules-in-priority-order`
- cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1045; sourceText: Effective rule warn-bypass-tool-visible: priority 200; decision warn; target visible-category; proof validated-screen-summary-with-source-correlation; minimum confidence 0.75.
- acceptedOptions: Warn | Visible Category | Validated Screen Summary With Source Correlation
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

234.  Choose effective rule unknown-screen-state-ask.

- settingId: `screen-effective-rule-effective-policy-document-rules-in-priority-order-0466`
- policyLane: `audit`; sectionId: `screen-effective-rule-effective-policy-document`; groupId: `screen-effective-rule-effective-policy-document-rules-in-priority-order`
- cardKind: `rule-list-card`; selectionMode: `multi`; controlKind: `rule-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1063; sourceText: Effective rule unknown-screen-state-ask: priority 900; decision ask; target unknown-state; proof screen-analysis-attempted; minimum confidence 0.
- acceptedOptions: Ask | Unknown State | Screen Analysis Attempted
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

## Tab: reports

### screen-capability-guide-bullet-screen-recording-possibilities-and-limits

#### screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits

235.  Use child-facing disclosure must be clearer;?

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0065`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 201; sourceText: child-facing disclosure must be clearer;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-ocr-and-image-classification

#### screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification

236.  Use local redaction before journal/report storage;?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0081`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 237; sourceText: local redaction before journal/report storage;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-retention-and-custody

#### screen-capability-guide-bullet-retention-and-custody-retention-and-custody

237.  Choose parent report.

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0115`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `retention-card`; selectionMode: `multi`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 323; sourceText: parent report: summary, refs, confidence, custody label, and deletion state;
- acceptedOptions: Summary | Refs | Confidence | Custody Label | And Deletion State
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-platform-capability-notes

#### screen-capability-guide-bullet-platform-capability-notes-android

238.  Use play policy and user disclosure constraints matter?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0182`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `portal-only`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 455; sourceText: Play policy and user disclosure constraints matter.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-child-facing-disclosure

#### screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure

239.  Represent parent setting state visible in the parent portal;.

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0201`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-child-facing-disclosure`; groupId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 503; sourceText: parent setting state visible in the parent portal;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

240.  Use child-facing disclosure that screen analysis may run locally;?

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0202`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-child-facing-disclosure`; groupId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 504; sourceText: child-facing disclosure that screen analysis may run locally;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

241.  Use clear difference between observe-only, dry-run, and enforcement-eligible modes;?

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0203`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-child-facing-disclosure`; groupId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 505; sourceText: clear difference between observe-only, dry-run, and enforcement-eligible modes;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

242.  Use current permission-required or disabled state;?

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0204`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-child-facing-disclosure`; groupId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 506; sourceText: current permission-required or disabled state;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

243.  Use no hidden background capture claims?

- settingId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure-0206`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-child-facing-disclosure`; groupId: `screen-capability-guide-bullet-child-facing-disclosure-child-facing-disclosure`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 508; sourceText: no hidden background capture claims.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-parent-reports

#### screen-capability-guide-bullet-parent-reports-parent-reports

244.  Represent setting state and who changed it;.

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0207`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 517; sourceText: setting state and who changed it;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

245.  Use capture reason and scope;?

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0208`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 518; sourceText: capture reason and scope;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

246.  Use category candidates and confidence;?

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0209`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 519; sourceText: category candidates and confidence;
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

247.  Use risk signals and confidence;?

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0210`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 520; sourceText: risk signals and confidence;
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

248.  Use bounded OCR snippets only when enabled and redacted;?

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0211`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 521; sourceText: bounded OCR snippets only when enabled and redacted;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

249.  Represent source evidence refs;.

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0212`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 522; sourceText: source evidence refs;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

250.  Use custody/source label;?

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0214`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 524; sourceText: custody/source label;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

251.  Use deletion state and image digest;?

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0215`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 525; sourceText: deletion state and image digest;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

252.  Use unavailable, protected, permission-required, low-confidence, stale, expired, invalid, delete-pending, or delete-failed states?

- settingId: `screen-capability-guide-bullet-parent-reports-parent-reports-0217`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-parent-reports`; groupId: `screen-capability-guide-bullet-parent-reports-parent-reports`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `protected`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 527; sourceText: unavailable, protected, permission-required, low-confidence, stale, expired, invalid, delete-pending, or delete-failed states.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-future-ui-rules

#### screen-capability-guide-bullet-future-ui-rules-future-ui-rules

253.  Use keep parent reports evidence-cited and custody-labeled?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0253`
- policyLane: `reports`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 593; sourceText: keep parent reports evidence-cited and custody-labeled.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-capability-matrix-row-capability-matrix

#### screen-capability-matrix-row-capability-matrix-show-parent-report

254.  Represent capability Show parent report: full screen Summary, confidence, refs, deletion state; active window Same; managed browser/window Same; local OCR/vision Same; important limit Raw screenshot hidden by default..

- settingId: `screen-capability-matrix-row-capability-matrix-show-parent-report-0274`
- policyLane: `reports`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-show-parent-report`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `unavailable`; runtimeOwner: `portal-only`; capabilityState: `unavailable`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 150; sourceText: Capability Show parent report: full screen Summary, confidence, refs, deletion state; active window Same; managed browser/window Same; local OCR/vision Same; important limit Raw screenshot hidden by default..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-screen-analysis

#### screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields

255.  Which disclosure requirements apply before capture?

- settingId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields-0310`
- policyLane: `reports`; sectionId: `screen-authoring-field-authoring-manifest-screen-analysis`; groupId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 219; sourceText: Which disclosure requirements apply before capture?
- acceptedOptions: Parent Setting Visible | Child Facing Local Analysis Disclosure | Capture Indicator When Platform Provides It | Raw Capture Not Retained By Default | Cloud Processing Disabled By Default | Report Custody Labels Visible | Default parent-setting-visible | Default child-facing-local-analysis-disclosure | Default raw-capture-not-retained-by-default | Default cloud-processing-disabled-by-default
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-redaction

#### screen-authoring-field-authoring-manifest-redaction-redaction-fields

256.  What must never be stored in summaries or reports?

- settingId: `screen-authoring-field-authoring-manifest-redaction-redaction-fields-0338`
- policyLane: `reports`; sectionId: `screen-authoring-field-authoring-manifest-redaction`; groupId: `screen-authoring-field-authoring-manifest-redaction-redaction-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 605; sourceText: What must never be stored in summaries or reports?
- acceptedOptions: Passwords | Tokens | Payment Data | Private Keys | Recovery Codes | Raw Image Bytes | Raw Local Paths | Browser Secrets | Cookies | Keystrokes | Decrypted Payloads | Microphone Audio | Camera Video | Default passwords | Default tokens | Default payment-data | Default private-keys | Default recovery-codes | Default raw-image-bytes | Default raw-local-paths | Default browser-secrets | Default decrypted-payloads
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-reports

#### screen-authoring-field-authoring-manifest-reports-reports-fields

257.  Which fields should parent reports show?

- settingId: `screen-authoring-field-authoring-manifest-reports-reports-fields-0350`
- policyLane: `reports`; sectionId: `screen-authoring-field-authoring-manifest-reports`; groupId: `screen-authoring-field-authoring-manifest-reports-reports-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 780; sourceText: Which fields should parent reports show?
- acceptedOptions: Setting State | Capability State | Capture Reason | Capture Scope | Category Candidates | Risk Signals | Confidence | Ocr Snippets | Redaction Notes | Source Evidence Refs | Local Model Runtime | Policy Decision Refs | Custody Label | Deletion State | Image Digest | Default setting-state | Default capability-state | Default capture-reason | Default capture-scope | Default category-candidates | Default risk-signals | Default confidence | Default source-evidence-refs | Default custody-label | Default deletion-state
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

258.  Show raw screenshots in parent reports by default?

- settingId: `screen-authoring-field-authoring-manifest-reports-reports-fields-0351`
- policyLane: `reports`; sectionId: `screen-authoring-field-authoring-manifest-reports`; groupId: `screen-authoring-field-authoring-manifest-reports-reports-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `portal-only`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 815; sourceText: Show raw screenshots in parent reports by default?
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-rendering-rule-authoring-manifest-metadata

#### screen-rendering-rule-authoring-manifest-metadata-rendering-rules

259.  Choose rendering rule showDisclosureBeforeEnable.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0360`
- policyLane: `reports`; sectionId: `screen-rendering-rule-authoring-manifest-metadata`; groupId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 114; sourceText: Rendering rule showDisclosureBeforeEnable: true.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-policy-fallback-policy-value-document

#### screen-policy-fallback-policy-value-document-fallbacks

260.  Choose fallback childDeviceOffline.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0444`
- policyLane: `reports`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1214; sourceText: Fallback childDeviceOffline: last-known-report-only.
- acceptedOptions: Last Known Report Only
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

## Tab: data

### screen-capability-guide-bullet-core-terms

#### screen-capability-guide-bullet-core-terms-screen-evidence

261.  Use encrypted temporary queue storage;?

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0004`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screen-evidence`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 29; sourceText: encrypted temporary queue storage;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-core-terms-ocr

262.  Use credential-like text redaction;?

- settingId: `screen-capability-guide-bullet-core-terms-ocr-0015`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-ocr`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 74; sourceText: credential-like text redaction;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-core-terms-evidence-reference

263.  Use queue job lifecycle;?

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0018`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-evidence-reference`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 95; sourceText: queue job lifecycle;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-the-main-capability-truth

#### screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth

264.  Use the raw image or frame is stored only in an encrypted temporary local queue;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0032`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 120; sourceText: the raw image or frame is stored only in an encrypted temporary local queue;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

265.  Use confidence, category, risk signal, redaction, custody, and deletion states validate;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0034`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 122; sourceText: confidence, category, risk signal, redaction, custody, and deletion states validate;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

266.  Represent the stored long-lived evidence is a summary plus refs, not raw pixels;.

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0035`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 123; sourceText: the stored long-lived evidence is a summary plus refs, not raw pixels;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-screenshot-possibilities-and-limits

#### screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible

267.  Use oCR snippets when parent settings allow them and local redaction permits storage;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0044`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 161; sourceText: OCR snippets when parent settings allow them and local redaction permits storage;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-screen-recording-possibilities-and-limits

#### screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits

268.  Use storage, deletion, and failure handling are harder to prove;?

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0064`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 200; sourceText: storage, deletion, and failure handling are harder to prove;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-ocr-and-image-classification

#### screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification

269.  Use disabled unless parent enables snippet storage or local analysis needs transient text;?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0079`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 235; sourceText: disabled unless parent enables snippet storage or local analysis needs transient text;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

270.  Use invalid output rejection before storage or policy use?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0091`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 250; sourceText: invalid output rejection before storage or policy use.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-redaction-and-minimization

#### screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization

271.  Use no raw screenshot shown by default;?

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0106`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-redaction-and-minimization`; groupId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `local-ai-runtime`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 305; sourceText: no raw screenshot shown by default;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

272.  Use no raw local file paths in portal copy/debug output;?

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0107`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-redaction-and-minimization`; groupId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 306; sourceText: no raw local file paths in portal copy/debug output;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

273.  Use no encrypted image refs outside the child agent;?

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0108`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-redaction-and-minimization`; groupId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 307; sourceText: no encrypted image refs outside the child agent;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

274.  Use oCR snippets bounded and redacted;?

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0109`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-redaction-and-minimization`; groupId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 308; sourceText: OCR snippets bounded and redacted;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

275.  Use credential-like text, passwords, tokens, payment fields, private keys, recovery codes, and session values redacted or omitted;?

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0110`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-redaction-and-minimization`; groupId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 309; sourceText: credential-like text, passwords, tokens, payment fields, private keys, recovery codes, and session values redacted or omitted;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

276.  Use protected regions skipped where the platform or local detector can identify them;?

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0111`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-redaction-and-minimization`; groupId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 311; sourceText: protected regions skipped where the platform or local detector can identify them;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

277.  Represent uncertain redaction state degrades policy eligibility.

- settingId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization-0112`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-redaction-and-minimization`; groupId: `screen-capability-guide-bullet-redaction-and-minimization-redaction-and-minimization`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 312; sourceText: uncertain redaction state degrades policy eligibility.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-retention-and-custody

#### screen-capability-guide-bullet-retention-and-custody-retention-and-custody

278.  Choose raw image/frame.

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0113`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 321; sourceText: raw image/frame: encrypted temporary queue only, deleted after success or TTL expiry;
- acceptedOptions: Encrypted Temporary Queue Only | Deleted After Success | TTL Expiry
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

279.  Represent stored summary: local journal and SQLite query store;.

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0114`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 322; sourceText: stored summary: local journal and SQLite query store;
- acceptedOptions: Local Journal And SQLite Query Store
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

280.  Choose parent cache/export.

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0116`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 324; sourceText: parent cache/export: explicit parent-owned destination only;
- acceptedOptions: Explicit Parent Owned Destination Only
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

281.  Choose ocentra-hosted storage.

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0117`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 325; sourceText: Ocentra-hosted storage: no child screen activity by default.
- acceptedOptions: No Child Screen Activity By Default
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

282.  Use `child-device-temp-queue`;?

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0118`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 329; sourceText: `child-device-temp-queue`;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

283.  Use `child-device-journal`;?

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0119`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 330; sourceText: `child-device-journal`;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

284.  Use `child-device-query-store`;?

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0120`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 331; sourceText: `child-device-query-store`;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

285.  Use `live-local-child-agent`;?

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0121`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 332; sourceText: `live-local-child-agent`;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

286.  Use `live-lan-child-agent`;?

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0122`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 333; sourceText: `live-lan-child-agent`;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

287.  Use `parent-device-cache`;?

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0123`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 334; sourceText: `parent-device-cache`;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

288.  Use `parent-owned-export`;?

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0124`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 335; sourceText: `parent-owned-export`;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

289.  Use `ocentra-hosted-non-activity`;?

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0125`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 336; sourceText: `ocentra-hosted-non-activity`;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

290.  Use `unavailable`?

- settingId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody-0126`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-retention-and-custody`; groupId: `screen-capability-guide-bullet-retention-and-custody-retention-and-custody`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 337; sourceText: `unavailable`.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-permission-required-and-unavailable-states

#### screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states

291.  Use queue unavailable;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0137`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 356; sourceText: queue unavailable;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

292.  Use redaction unavailable;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0138`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 357; sourceText: redaction unavailable;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-platform-capability-notes

#### screen-capability-guide-bullet-platform-capability-notes-windows

293.  Use encrypted temporary queue and journal/SQLite read model;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0147`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 378; sourceText: encrypted temporary queue and journal/SQLite read model;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-macos

294.  Represent encrypted local queue and summary storage.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0158`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-macos`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 402; sourceText: encrypted local queue and summary storage.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-linux

295.  Use restore/persistent permission behavior differs across desktop portals;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0169`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 429; sourceText: restore/persistent permission behavior differs across desktop portals;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-future-ui-rules

#### screen-capability-guide-bullet-future-ui-rules-future-ui-rules

296.  Use show whether OCR snippet storage is enabled;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0244`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 582; sourceText: show whether OCR snippet storage is enabled;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

297.  Use show redaction mode and redaction failures;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0245`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `local-ai-runtime`; capabilityState: `future-gap`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 583; sourceText: show redaction mode and redaction failures;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

298.  Use show queue health and deletion health;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0247`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 585; sourceText: show queue health and deletion health;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

299.  Use show raw capture retention as off by default;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0249`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 587; sourceText: show raw capture retention as off by default;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

300.  Represent show capability state close to each action: ready, unsupported, permission-required, permission-limited, protected-surface, model-unavailable, queue-unavailable, adapter-error, degraded, disabled-by-parent, or manual-required;.

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0250`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 588; sourceText: show capability state close to each action: ready, unsupported, permission-required, permission-limited, protected-surface, model-unavailable, queue-unavailable, adapter-error, degraded, disabled-by-parent, or manual-required;
- acceptedOptions: Ready | Unsupported | Permission Required | Permission Limited | Protected Surface | Model Unavailable | Queue Unavailable | Adapter Error | Degraded | Disabled By Parent | Manual Required
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

301.  Use strict deletion and no raw image retention by default?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0262`
- policyLane: `data`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 605; sourceText: strict deletion and no raw image retention by default.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-matrix-row-capability-matrix

#### screen-capability-matrix-row-capability-matrix-retain-raw-capture

302.  Represent capability Retain raw capture: full screen No by default; active window No by default; managed browser/window No by default; local OCR/vision No by default; important limit Future retention needs separate custody/legal/privacy design..

- settingId: `screen-capability-matrix-row-capability-matrix-retain-raw-capture-0275`
- policyLane: `data`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-retain-raw-capture`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 151; sourceText: Capability Retain raw capture: full screen No by default; active window No by default; managed browser/window No by default; local OCR/vision No by default; important limit Future retention needs separate custody/legal/privacy design..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-proposal-overview

#### screen-schema-proposal-bullet-proposal-overview-proposal-overview

303.  Use encrypted temporary image queueing?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0281`
- policyLane: `data`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 12; sourceText: Encrypted temporary image queueing.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

304.  Use typed summary storage with evidence refs and deletion state?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0283`
- policyLane: `data`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 14; sourceText: Typed summary storage with evidence refs and deletion state.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

305.  Represent tests for every parser, authoring manifest field, policy value shape, compile rule, patch command, capability state, queue state, deletion state, confidence value, and invalid-state rejection.

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0291`
- policyLane: `data`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 27; sourceText: Tests for every parser, authoring manifest field, policy value shape, compile rule, patch command, capability state, queue state, deletion state, confidence value, and invalid-state rejection.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-implementation-notes-for-worker

#### screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker

306.  Use add explicit tests that invalid confidence, missing source refs, missing deletion state, protected surfaces, and delete failures cannot produce enforcement-eligible screen summaries?

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0305`
- policyLane: `data`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1690; sourceText: Add explicit tests that invalid confidence, missing source refs, missing deletion state, protected surfaces, and delete failures cannot produce enforcement-eligible screen summaries.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-ocr-and-vision

#### screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields

307.  Store bounded OCR text snippets in summaries?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0332`
- policyLane: `data`; sectionId: `screen-authoring-field-authoring-manifest-ocr-and-vision`; groupId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 536; sourceText: Store bounded OCR text snippets in summaries?
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-redaction

#### screen-authoring-field-authoring-manifest-redaction-redaction-fields

308.  How should visible text and sensitive regions be redacted?

- settingId: `screen-authoring-field-authoring-manifest-redaction-redaction-fields-0337`
- policyLane: `data`; sectionId: `screen-authoring-field-authoring-manifest-redaction`; groupId: `screen-authoring-field-authoring-manifest-redaction-redaction-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 597; sourceText: How should visible text and sensitive regions be redacted?
- acceptedOptions: Off | Summary Only | Strict Local | Credential Sensitive | Parent Review Required | Default strict-local
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

309.  What if redaction is unavailable?

- settingId: `screen-authoring-field-authoring-manifest-redaction-redaction-fields-0339`
- policyLane: `data`; sectionId: `screen-authoring-field-authoring-manifest-redaction`; groupId: `screen-authoring-field-authoring-manifest-redaction-redaction-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 637; sourceText: What if redaction is unavailable?
- acceptedOptions: Summary Only Not Policy Eligible | Mark Invalid | Delete And Audit | Ask Parent | Default summary-only-not-policy-eligible
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-queue-and-retention

#### screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields

310.  How long may a temporary image remain queued?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0340`
- policyLane: `data`; sectionId: `screen-authoring-field-authoring-manifest-queue-and-retention`; groupId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields`
- cardKind: `duration-card`; selectionMode: `numeric`; controlKind: `duration`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 656; sourceText: How long may a temporary image remain queued?
- acceptedOptions: Default 300 | Minimum 30 | Maximum 1800
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

311.  How many local analysis retries are allowed before deletion?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0341`
- policyLane: `data`; sectionId: `screen-authoring-field-authoring-manifest-queue-and-retention`; groupId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields`
- cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 665; sourceText: How many local analysis retries are allowed before deletion?
- acceptedOptions: Default 2 | Minimum 0 | Maximum 5
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

312.  Delete raw image after successful analysis?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0342`
- policyLane: `data`; sectionId: `screen-authoring-field-authoring-manifest-queue-and-retention`; groupId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 674; sourceText: Delete raw image after successful analysis?
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

313.  Delete raw image after TTL expiry?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0343`
- policyLane: `data`; sectionId: `screen-authoring-field-authoring-manifest-queue-and-retention`; groupId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 682; sourceText: Delete raw image after TTL expiry?
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

314.  Retain raw screenshots or recordings?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0344`
- policyLane: `data`; sectionId: `screen-authoring-field-authoring-manifest-queue-and-retention`; groupId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `parent-owned-storage`; capabilityState: `unavailable`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 690; sourceText: Retain raw screenshots or recordings?
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

315.  Allow Ocentra-hosted processing of child screen images?

- settingId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields-0345`
- policyLane: `data`; sectionId: `screen-authoring-field-authoring-manifest-queue-and-retention`; groupId: `screen-authoring-field-authoring-manifest-queue-and-retention-queue-and-retention-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `parent-owned-storage`; capabilityState: `unavailable`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 698; sourceText: Allow Ocentra-hosted processing of child screen images?
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-rendering-rule-authoring-manifest-metadata

#### screen-rendering-rule-authoring-manifest-metadata-rendering-rules

316.  Choose rendering rule showRawCaptureRetentionAsOffByDefault.

- settingId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules-0362`
- policyLane: `data`; sectionId: `screen-rendering-rule-authoring-manifest-metadata`; groupId: `screen-rendering-rule-authoring-manifest-metadata-rendering-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 116; sourceText: Rendering rule showRawCaptureRetentionAsOffByDefault: true.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-control-kind-authoring-manifest-metadata

#### screen-control-kind-authoring-manifest-metadata-control-kinds

317.  Choose control kind.

- settingId: `screen-control-kind-authoring-manifest-metadata-control-kinds-0369`
- policyLane: `data`; sectionId: `screen-control-kind-authoring-manifest-metadata`; groupId: `screen-control-kind-authoring-manifest-metadata-control-kinds`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 125; sourceText: Control kind: retention.
- acceptedOptions: Retention
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-capability-state-meaning-capability-registry

#### screen-capability-state-meaning-capability-registry-capability-state-meanings

318.  Show queue-unavailable capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0405`
- policyLane: `data`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1663; sourceText: queue-unavailable: Encrypted temporary queue cannot be opened or validated.
- acceptedOptions: Queue Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-agent-rule-update-protocol

#### screen-agent-rule-update-protocol-agent-rules

319.  Choose agent rule deleteQueuedImagesOnInvalidOutput.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0430`
- policyLane: `data`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1548; sourceText: Agent rule deleteQueuedImagesOnInvalidOutput: true.
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-policy-fallback-policy-value-document

#### screen-policy-fallback-policy-value-document-fallbacks

320.  Choose fallback queueUnavailable.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0438`
- policyLane: `data`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1208; sourceText: Fallback queueUnavailable: fail-closed.
- acceptedOptions: Fail Closed
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

321.  Choose fallback modelUnavailable.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0439`
- policyLane: `data`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1209; sourceText: Fallback modelUnavailable: retry-within-ttl-then-delete.
- acceptedOptions: Retry Within Ttl Then Delete
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

322.  Choose fallback redactionUnavailable.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0440`
- policyLane: `data`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1210; sourceText: Fallback redactionUnavailable: summary-only-not-policy-eligible.
- acceptedOptions: Summary Only Not Policy Eligible
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

323.  Choose fallback invalidModelOutput.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0441`
- policyLane: `data`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1211; sourceText: Fallback invalidModelOutput: delete-and-mark-invalid.
- acceptedOptions: Delete And Mark Invalid
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

324.  Choose fallback deleteFailed.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0443`
- policyLane: `data`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1213; sourceText: Fallback deleteFailed: surface-health-and-retry.
- acceptedOptions: Surface Health And Retry
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

## Tab: ai

### screen-capability-guide-bullet-core-terms

#### screen-capability-guide-bullet-core-terms-screen-evidence

325.  Use local OCR/vision analysis;?

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0005`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screen-evidence`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 30; sourceText: local OCR/vision analysis;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

326.  Use schema-valid summary, category candidates, risk signals, confidence, evidence refs, digest, and deletion state;?

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0006`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screen-evidence`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 31; sourceText: schema-valid summary, category candidates, risk signals, confidence, evidence refs, digest, and deletion state;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-core-terms-screenshot

327.  Use protected or unsupported scope represented as unavailable, not captured?

- settingId: `screen-capability-guide-bullet-core-terms-screenshot-0012`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screenshot`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 45; sourceText: protected or unsupported scope represented as unavailable, not captured.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-core-terms-ocr

328.  Use visible text snippets;?

- settingId: `screen-capability-guide-bullet-core-terms-ocr-0013`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-ocr`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 72; sourceText: visible text snippets;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

329.  Use text category hints;?

- settingId: `screen-capability-guide-bullet-core-terms-ocr-0014`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-ocr`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 73; sourceText: text category hints;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

330.  Use unsafe phrase or bypass-tool signals;?

- settingId: `screen-capability-guide-bullet-core-terms-ocr-0016`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-ocr`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 75; sourceText: unsafe phrase or bypass-tool signals;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

331.  Use policy explanation references?

- settingId: `screen-capability-guide-bullet-core-terms-ocr-0017`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-ocr`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 76; sourceText: policy explanation references.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

#### screen-capability-guide-bullet-core-terms-evidence-reference

332.  Represent managed browser URL/tab evidence where available;.

- settingId: `screen-capability-guide-bullet-core-terms-evidence-reference-0022`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-evidence-reference`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 99; sourceText: managed browser URL/tab evidence where available;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-the-main-capability-truth

#### screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth

333.  Use the current platform adapter supports the requested scope;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0029`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 117; sourceText: the current platform adapter supports the requested scope;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

334.  Use required OS permission or management state is present;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0030`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 118; sourceText: required OS permission or management state is present;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

335.  Use protected surfaces are skipped or represented as unavailable;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0031`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 119; sourceText: protected surfaces are skipped or represented as unavailable;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

336.  Use local OCR/vision returns schema-valid output;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0033`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 121; sourceText: local OCR/vision returns schema-valid output;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

337.  Represent policy and enforcement consume only typed summaries and evidence refs.

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0036`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 124; sourceText: policy and enforcement consume only typed summaries and evidence refs.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

338.  Use hidden capture;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0037`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `local-ai-runtime`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 128; sourceText: hidden capture;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

339.  Use cloud/API AI screenshot processing by default;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0038`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `local-ai-runtime`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 129; sourceText: cloud/API AI screenshot processing by default;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

340.  Use permanent screenshot history by default;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0039`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `unavailable`; runtimeOwner: `local-ai-runtime`; capabilityState: `unavailable`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 130; sourceText: permanent screenshot history by default;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

341.  Use exact page, URL, chat, password, or intent from pixels alone;?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0040`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 131; sourceText: exact page, URL, chat, password, or intent from pixels alone;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

342.  Use enforcement from raw model text or unvalidated image classification?

- settingId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth-0042`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-the-main-capability-truth`; groupId: `screen-capability-guide-bullet-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 133; sourceText: enforcement from raw model text or unvalidated image classification.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-screenshot-possibilities-and-limits

#### screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible

343.  Use visible activity categories such as school, video, chat, game, shopping, productivity, adult content, violence, bypass tool, unknown, or low confidence;?

- settingId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible-0043`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screenshot-possibilities-and-limits-what-is-possible`
- cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 159; sourceText: visible activity categories such as school, video, chat, game, shopping, productivity, adult content, violence, bypass tool, unknown, or low confidence;
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-screen-recording-possibilities-and-limits

#### screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits

344.  Use short rolling local analysis buffer that never becomes a retained video archive;?

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0060`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 193; sourceText: short rolling local analysis buffer that never becomes a retained video archive;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

345.  Use accessibility-like visible flow analysis where the platform permits it?

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0062`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `portal-only`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 195; sourceText: accessibility-like visible flow analysis where the platform permits it.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

346.  Use protected media and secure surfaces may appear black, unavailable, omitted, or blocked depending on OS;?

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0067`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 203; sourceText: protected media and secure surfaces may appear black, unavailable, omitted, or blocked depending on OS;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-ocr-and-image-classification

#### screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification

347.  Use bounded snippet count and character length;?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0080`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 236; sourceText: bounded snippet count and character length;
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

348.  Use sensitive tokens, passwords, credential-like text, payment data, and secrets redacted or skipped;?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0082`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 238; sourceText: sensitive tokens, passwords, credential-like text, payment data, and secrets redacted or skipped;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

349.  Use oCR-disabled state represented explicitly;?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0083`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 239; sourceText: OCR-disabled state represented explicitly;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

350.  Use unsupported language, low resolution, or low confidence represented as unknown/degraded?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0084`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 240; sourceText: unsupported language, low resolution, or low confidence represented as unknown/degraded.
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

351.  Use enum-backed categories and risk signals, not open-ended model prose;?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0085`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 244; sourceText: enum-backed categories and risk signals, not open-ended model prose;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

352.  Use confidence in `0..1`;?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0086`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 245; sourceText: confidence in `0..1`;
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

353.  Use uncertainty reason;?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0088`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 247; sourceText: uncertainty reason;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

354.  Represent source evidence refs;.

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0089`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 248; sourceText: source evidence refs;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

355.  Use policy eligibility flag;?

- settingId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification-0090`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-ocr-and-image-classification`; groupId: `screen-capability-guide-bullet-ocr-and-image-classification-ocr-and-image-classification`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 249; sourceText: policy eligibility flag;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-permission-required-and-unavailable-states

#### screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states

356.  Use disabled by parent;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0127`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 346; sourceText: disabled by parent;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

357.  Use unsupported platform;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0128`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 347; sourceText: unsupported platform;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

358.  Use unsupported scope;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0129`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 348; sourceText: unsupported scope;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

359.  Use permission required;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0130`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 349; sourceText: permission required;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

360.  Use permission denied;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0131`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 350; sourceText: permission denied;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

361.  Represent permission limited;.

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0132`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-limited`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-limited`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 351; sourceText: permission limited;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

362.  Use protected surface;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0133`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 352; sourceText: protected surface;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

363.  Use screen locked;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0134`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 353; sourceText: screen locked;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

364.  Use session unavailable;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0135`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 354; sourceText: session unavailable;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

365.  Use model unavailable;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0136`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 355; sourceText: model unavailable;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

366.  Use degraded;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0139`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 358; sourceText: degraded;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

367.  Use adapter error;?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0140`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 359; sourceText: adapter error;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

368.  Use ready?

- settingId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states-0141`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-permission-required-and-unavailable-states`; groupId: `screen-capability-guide-bullet-permission-required-and-unavailable-states-permission-required-and-unavailable-states`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 360; sourceText: ready.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-platform-capability-notes

#### screen-capability-guide-bullet-platform-capability-notes-windows

369.  Use local OCR through Windows OCR APIs or another local model boundary where packaged/available;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0145`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 376; sourceText: local OCR through Windows OCR APIs or another local model boundary where packaged/available;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

370.  Use local vision classification through an Ocentra-owned local model/provider boundary;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0146`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 377; sourceText: local vision classification through an Ocentra-owned local model/provider boundary;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

371.  Use secure desktop, lock screen, UAC prompts, credential surfaces, protected media, or DRM-protected content must be skipped or represented as protected/unavailable;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0151`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 386; sourceText: secure desktop, lock screen, UAC prompts, credential surfaces, protected media, or DRM-protected content must be skipped or represented as protected/unavailable;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-macos

372.  Use local Vision framework OCR/classification or Ocentra local model boundary;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0156`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 400; sourceText: local Vision framework OCR/classification or Ocentra local model boundary;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

373.  Use protected windows or windows that opt out of sharing may be unavailable or omitted;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0160`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 408; sourceText: protected windows or windows that opt out of sharing may be unavailable or omitted;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-linux

374.  Use local OCR/vision through Ocentra-owned local model/provider boundary;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0165`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 422; sourceText: local OCR/vision through Ocentra-owned local model/provider boundary;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

375.  Use available source types vary by backend;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0168`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 428; sourceText: available source types vary by backend;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-android

376.  Use on-device ML Kit or Ocentra local model boundary for OCR/image labeling where allowed;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0175`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 443; sourceText: on-device ML Kit or Ocentra local model boundary for OCR/image labeling where allowed;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados

377.  Use third-party parental-control apps should not claim arbitrary hidden screenshot or screen-recording access;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0187`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 470; sourceText: third-party parental-control apps should not claim arbitrary hidden screenshot or screen-recording access;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-app-and-window-correlation

#### screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation

378.  Use exact URL from window title or OCR alone;?

- settingId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation-0197`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-app-and-window-correlation`; groupId: `screen-capability-guide-bullet-app-and-window-correlation-app-and-window-correlation`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: managed-browser-evidence-required
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 492; sourceText: exact URL from window title or OCR alone;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-future-ui-rules

#### screen-capability-guide-bullet-future-ui-rules-future-ui-rules

379.  Use show screen analysis disabled by default;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0242`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 579; sourceText: show screen analysis disabled by default;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

380.  Use show capture scope as full screen, active display, active window, managed browser/window, app window, or unavailable;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0243`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `local-ai-runtime`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 580; sourceText: show capture scope as full screen, active display, active window, managed browser/window, app window, or unavailable;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

381.  Use no screen analysis;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0254`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 597; sourceText: no screen analysis;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

382.  Use local enforcement-eligible summaries with confidence thresholds;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0257`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `future-gap`; runtimeOwner: `local-ai-runtime`; capabilityState: `future-gap`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 600; sourceText: local enforcement-eligible summaries with confidence thresholds;
- acceptedOptions: Configured Value | Minimum | Maximum
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

383.  Use oCR snippets off or bounded/on;?

- settingId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules-0261`
- policyLane: `ai`; sectionId: `screen-capability-guide-bullet-future-ui-rules`; groupId: `screen-capability-guide-bullet-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `local-ai-runtime`; capabilityState: `future-gap`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 604; sourceText: OCR snippets off or bounded/on;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-matrix-row-capability-matrix

#### screen-capability-matrix-row-capability-matrix-classify-visible-activity

384.  Represent capability Classify visible activity: full screen Broad but sensitive; active window Narrower and usually more relevant; managed browser/window Narrowest for web/app context; local OCR/vision Yes, with confidence; important limit Category is not policy authority..

- settingId: `screen-capability-matrix-row-capability-matrix-classify-visible-activity-0265`
- policyLane: `ai`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-classify-visible-activity`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 141; sourceText: Capability Classify visible activity: full screen Broad but sensitive; active window Narrower and usually more relevant; managed browser/window Narrowest for web/app context; local OCR/vision Yes, with confidence; important limit Category is not policy authority..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-matrix-row-capability-matrix-extract-ocr-snippets

385.  Represent capability Extract OCR snippets: full screen Possible; active window Possible; managed browser/window Possible; local OCR/vision Yes, if enabled; important limit Snippets must be bounded and redacted..

- settingId: `screen-capability-matrix-row-capability-matrix-extract-ocr-snippets-0266`
- policyLane: `ai`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-extract-ocr-snippets`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 142; sourceText: Capability Extract OCR snippets: full screen Possible; active window Possible; managed browser/window Possible; local OCR/vision Yes, if enabled; important limit Snippets must be bounded and redacted..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-matrix-row-capability-matrix-prove-app-window-context

386.  Represent capability Prove app/window context: full screen Correlate with foreground evidence; active window Stronger when captured source is a window; managed browser/window Strong if managed session/window id is linked; local OCR/vision No by itself; important limit Capture source ids must be recorded..

- settingId: `screen-capability-matrix-row-capability-matrix-prove-app-window-context-0268`
- policyLane: `ai`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-prove-app-window-context`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 144; sourceText: Capability Prove app/window context: full screen Correlate with foreground evidence; active window Stronger when captured source is a window; managed browser/window Strong if managed session/window id is linked; local OCR/vision No by itself; important limit Capture source ids must be recorded..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-matrix-row-capability-matrix-detect-protected-surfaces

387.  Represent capability Detect protected surfaces: full screen Platform dependent; active window Platform dependent; managed browser/window Platform dependent; local OCR/vision Not after the fact reliably; important limit Protected/secure/credential states must fail closed..

- settingId: `screen-capability-matrix-row-capability-matrix-detect-protected-surfaces-0270`
- policyLane: `ai`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-detect-protected-surfaces`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 146; sourceText: Capability Detect protected surfaces: full screen Platform dependent; active window Platform dependent; managed browser/window Platform dependent; local OCR/vision Not after the fact reliably; important limit Protected/secure/credential states must fail closed..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-matrix-row-capability-matrix-feed-local-ai

388.  Represent capability Feed local AI: full screen Summary/ref only by default; active window Summary/ref only by default; managed browser/window Summary/ref only by default; local OCR/vision Yes after schema validation; important limit Raw image is not normal AI context-builder input..

- settingId: `screen-capability-matrix-row-capability-matrix-feed-local-ai-0271`
- policyLane: `ai`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-feed-local-ai`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 147; sourceText: Capability Feed local AI: full screen Summary/ref only by default; active window Summary/ref only by default; managed browser/window Summary/ref only by default; local OCR/vision Yes after schema validation; important limit Raw image is not normal AI context-builder input..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-matrix-row-capability-matrix-feed-policy

389.  Represent capability Feed policy: full screen Only via summary/ref; active window Only via summary/ref; managed browser/window Only via summary/ref; local OCR/vision Yes, after validation; important limit Requires parent rule and confidence threshold..

- settingId: `screen-capability-matrix-row-capability-matrix-feed-policy-0272`
- policyLane: `ai`; sectionId: `screen-capability-matrix-row-capability-matrix`; groupId: `screen-capability-matrix-row-capability-matrix-feed-policy`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 148; sourceText: Capability Feed policy: full screen Only via summary/ref; active window Only via summary/ref; managed browser/window Only via summary/ref; local OCR/vision Yes, after validation; important limit Requires parent rule and confidence threshold..
- acceptedOptions: Full Screen/display | Active Window | Managed Browser/window | Local OCR/vision | Important Limit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-proposal-overview

#### screen-schema-proposal-bullet-proposal-overview-proposal-overview

390.  Use parent-authored screen analysis settings?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0277`
- policyLane: `ai`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 8; sourceText: Parent-authored screen analysis settings.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

391.  Use local OCR/vision analysis?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0282`
- policyLane: `ai`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 13; sourceText: Local OCR/vision analysis.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-implementation-notes-for-worker

#### screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker

392.  Represent start with domain contracts before Portal UI.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0294`
- policyLane: `ai`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1674; sourceText: Start with domain contracts before Portal UI.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

393.  Use persist both policy revision and compiled effective policy hash?

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0299`
- policyLane: `ai`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1680; sourceText: Persist both policy revision and compiled effective policy hash.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

394.  Represent keep `retainRawCapture` and `hostedProcessingAllowed` false for this schema version.

- settingId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker-0302`
- policyLane: `ai`; sectionId: `screen-schema-proposal-bullet-implementation-notes-for-worker`; groupId: `screen-schema-proposal-bullet-implementation-notes-for-worker-implementation-notes-for-worker`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1685; sourceText: Keep `retainRawCapture` and `hostedProcessingAllowed` false for this schema version.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-screen-analysis

#### screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields

395.  Enable local screen evidence analysis?

- settingId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields-0307`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-screen-analysis`; groupId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 150; sourceText: Enable local screen evidence analysis?
- acceptedOptions: Enabled | Disabled | Default false
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

396.  How should screen analysis be used?

- settingId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields-0308`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-screen-analysis`; groupId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 174; sourceText: How should screen analysis be used?
- acceptedOptions: Observe Only | Policy Preview | Ask Parent | Can Enforce | Default observe-only
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

397.  Where should screen analysis run?

- settingId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields-0309`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-screen-analysis`; groupId: `screen-authoring-field-authoring-manifest-screen-analysis-screen-analysis-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 207; sourceText: Where should screen analysis run?
- acceptedOptions: Local Child Agent | Lan Live Child Agent | Authoring Only | Unavailable | Default local-child-agent
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-capture-scope

#### screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields

398.  Require managed browser evidence for exact web claims?

- settingId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields-0315`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-capture-scope`; groupId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: managed-browser-evidence-required
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 294; sourceText: Require managed browser evidence for exact web claims?
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-screen-recording

#### screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields

399.  How may recording frames be used for analysis?

- settingId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields-0318`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-screen-recording`; groupId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 339; sourceText: How may recording frames be used for analysis?
- acceptedOptions: No Recording | Sample Keyframes Only | Sample At Trigger Boundary | Summarize Then Delete | Default no-recording
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

400.  How should raw recording data be retained?

- settingId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields-0320`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-screen-recording`; groupId: `screen-authoring-field-authoring-manifest-screen-recording-screen-recording-fields`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 355; sourceText: How should raw recording data be retained?
- acceptedOptions: No Raw Video Retention | Temporary Queue Only | Future Explicit Parent Controlled Retention | Default no-raw-video-retention
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-scheduling

#### screen-authoring-field-authoring-manifest-scheduling-scheduling-fields

401.  When may screen analysis run?

- settingId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields-0324`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-scheduling`; groupId: `screen-authoring-field-authoring-manifest-scheduling-scheduling-fields`
- cardKind: `schedule-card`; selectionMode: `schedule`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 405; sourceText: When may screen analysis run?
- acceptedOptions: Default always
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-authoring-field-authoring-manifest-ocr-and-vision

#### screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields

402.  Require local OCR/vision for screen analysis?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0330`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-ocr-and-vision`; groupId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 514; sourceText: Require local OCR/vision for screen analysis?
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

403.  Which local analysis tasks are allowed?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0331`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-ocr-and-vision`; groupId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 521; sourceText: Which local analysis tasks are allowed?
- acceptedOptions: Visible Category Classification | Safety Indicator Classification | Ocr Transient Only | Ocr Snippet Storage | Sensitive Region Redaction | Managed Window Classification | Default visible-category-classification | Default safety-indicator-classification
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

404.  How many OCR snippets may be retained per summary?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0333`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-ocr-and-vision`; groupId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields`
- cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 543; sourceText: How many OCR snippets may be retained per summary?
- acceptedOptions: Default 3 | Minimum 0 | Maximum 10
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

405.  What is the maximum length of each retained OCR snippet?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0334`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-ocr-and-vision`; groupId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields`
- cardKind: `number-card`; selectionMode: `numeric`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 556; sourceText: What is the maximum length of each retained OCR snippet?
- acceptedOptions: Default 120 | Minimum 0 | Maximum 500
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

406.  What confidence is required before screen summaries can affect policy?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0335`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-ocr-and-vision`; groupId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields`
- cardKind: `threshold-card`; selectionMode: `numeric`; controlKind: `threshold`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 569; sourceText: What confidence is required before screen summaries can affect policy?
- acceptedOptions: Default 0.8 | Minimum 0 | Maximum 1
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

407.  What if the local model returns invalid output?

- settingId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields-0336`
- policyLane: `ai`; sectionId: `screen-authoring-field-authoring-manifest-ocr-and-vision`; groupId: `screen-authoring-field-authoring-manifest-ocr-and-vision-ocr-and-vision-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 578; sourceText: What if the local model returns invalid output?
- acceptedOptions: Delete And Mark Invalid | Retry Within Ttl | Mark Unavailable | Ask Parent | Default delete-and-mark-invalid
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-condition-kind-authoring-manifest-metadata

#### screen-condition-kind-authoring-manifest-metadata-condition-kinds

408.  Represent condition kind: capabilityAvailable.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0380`
- policyLane: `ai`; sectionId: `screen-condition-kind-authoring-manifest-metadata`; groupId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 138; sourceText: Condition kind: capabilityAvailable.
- acceptedOptions: CapabilityAvailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-state-meaning-capability-registry

#### screen-capability-state-meaning-capability-registry-capability-state-meanings

409.  Show unsupported-scope capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0400`
- policyLane: `ai`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1658; sourceText: unsupported-scope: The requested full-screen, display, window, app, or managed-window scope is unavailable.
- acceptedOptions: Unsupported Scope
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

410.  Show degraded capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0406`
- policyLane: `ai`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1664; sourceText: degraded: The capability can run with reduced scope, fidelity, freshness, or confidence.
- acceptedOptions: Degraded
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-update-command-update-protocol

#### screen-update-command-update-protocol-commands

411.  Support screen-policy.get.requested?

- settingId: `screen-update-command-update-protocol-commands-0409`
- policyLane: `ai`; sectionId: `screen-update-command-update-protocol`; groupId: `screen-update-command-update-protocol-commands`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1424; sourceText: screen-policy.get.requested: Portal asks the child agent for current screen policy value, effective policy, capability registry, and revision.
- acceptedOptions: Screen Policy.get.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

412.  Support screen-policy.patch.requested?

- settingId: `screen-update-command-update-protocol-commands-0411`
- policyLane: `ai`; sectionId: `screen-update-command-update-protocol`; groupId: `screen-update-command-update-protocol-commands`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1456; sourceText: screen-policy.patch.requested: Portal sends a small settings change with an expected revision.
- acceptedOptions: Screen Policy.patch.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

413.  Support screen-policy.rollback.requested?

- settingId: `screen-update-command-update-protocol-commands-0414`
- policyLane: `ai`; sectionId: `screen-update-command-update-protocol`; groupId: `screen-update-command-update-protocol-commands`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1523; sourceText: screen-policy.rollback.requested: Parent asks child agent to roll back to previous valid revision.
- acceptedOptions: Screen Policy.rollback.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-agent-rule-update-protocol

#### screen-agent-rule-update-protocol-agent-rules

414.  Choose agent rule keepPreviousValidRevision.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0418`
- policyLane: `ai`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1536; sourceText: Agent rule keepPreviousValidRevision: true.
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

415.  Choose agent rule rollbackOnCompileFailure.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0419`
- policyLane: `ai`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1537; sourceText: Agent rule rollbackOnCompileFailure: true.
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

416.  Choose agent rule runOcrVisionOnlyInChildAgent.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0421`
- policyLane: `ai`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1539; sourceText: Agent rule runOcrVisionOnlyInChildAgent: true.
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

417.  Choose agent rule rejectConfidenceOutsideZeroOne.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0425`
- policyLane: `ai`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1543; sourceText: Agent rule rejectConfidenceOutsideZeroOne: true.
- acceptedOptions: Required | Rejected If False
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

418.  Choose agent rule rejectRetainRawCaptureForSchemaVersionOne.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0426`
- policyLane: `ai`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1544; sourceText: Agent rule rejectRetainRawCaptureForSchemaVersionOne: true.
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

419.  Choose agent rule rejectExactWebClaimsWithoutManagedBrowserEvidence.

- settingId: `screen-agent-rule-update-protocol-agent-rules-0429`
- policyLane: `ai`; sectionId: `screen-agent-rule-update-protocol`; groupId: `screen-agent-rule-update-protocol-agent-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1547; sourceText: Agent rule rejectExactWebClaimsWithoutManagedBrowserEvidence: true.
- acceptedOptions: Required | Rejected If False
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-policy-fallback-policy-value-document

#### screen-policy-fallback-policy-value-document-fallbacks

420.  Choose fallback permissionDenied.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0433`
- policyLane: `ai`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1203; sourceText: Fallback permissionDenied: mark-unavailable.
- acceptedOptions: Mark Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

421.  Choose fallback unsupportedScope.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0435`
- policyLane: `ai`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1205; sourceText: Fallback unsupportedScope: fall-back-to-supported-scope-or-unavailable.
- acceptedOptions: Fall Back To Supported Scope Or Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

422.  Choose fallback platformUnsupported.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0445`
- policyLane: `ai`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1215; sourceText: Fallback platformUnsupported: show-unavailable.
- acceptedOptions: Show Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

423.  Choose fallback policyUse.protectedSurfaceFallback.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0447`
- policyLane: `ai`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1021; sourceText: Fallback policyUse.protectedSurfaceFallback: mark-unavailable.
- acceptedOptions: Mark Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

424.  Choose fallback policyUse.invalidOutputFallback.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0448`
- policyLane: `ai`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1022; sourceText: Fallback policyUse.invalidOutputFallback: mark-unavailable.
- acceptedOptions: Mark Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

425.  Choose fallback portalAi.fallbackWhenUnavailable.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0449`
- policyLane: `ai`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: schema-valid-local-analysis-output-with-confidence-and-redaction-state
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1146; sourceText: Fallback portalAi.fallbackWhenUnavailable: manual-view.
- acceptedOptions: Manual View
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

## Tab: setup

### screen-capability-guide-bullet-screen-recording-possibilities-and-limits

#### screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits

426.  Use short parent test session during setup;?

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0059`
- policyLane: `setup`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 192; sourceText: short parent test session during setup;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-schema-proposal-bullet-proposal-overview

#### screen-schema-proposal-bullet-proposal-overview-proposal-overview

427.  Use full policy replacement during setup/import/reset?

- settingId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview-0285`
- policyLane: `setup`; sectionId: `screen-schema-proposal-bullet-proposal-overview`; groupId: `screen-schema-proposal-bullet-proposal-overview-proposal-overview`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `agent-protocol`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 16; sourceText: Full policy replacement during setup/import/reset.
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-capability-state-meaning-capability-registry

#### screen-capability-state-meaning-capability-registry-capability-state-meanings

428.  Show disabled-by-parent capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0398`
- policyLane: `setup`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `disabled`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1656; sourceText: disabled-by-parent: Parent setting disables the feature.
- acceptedOptions: Disabled By Parent
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-update-command-update-protocol

#### screen-update-command-update-protocol-commands

429.  Support screen-policy.replace.requested?

- settingId: `screen-update-command-update-protocol-commands-0412`
- policyLane: `setup`; sectionId: `screen-update-command-update-protocol`; groupId: `screen-update-command-update-protocol-commands`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1486; sourceText: screen-policy.replace.requested: Portal sends a full policy replacement for setup, import, reset, or wizard save.
- acceptedOptions: Screen Policy.replace.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

430.  Support screen-policy.manual-test-capture.requested?

- settingId: `screen-update-command-update-protocol-commands-0413`
- policyLane: `setup`; sectionId: `screen-update-command-update-protocol`; groupId: `screen-update-command-update-protocol-commands`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1501; sourceText: screen-policy.manual-test-capture.requested: Parent requests one explicit setup/test capture through the child agent.
- acceptedOptions: Screen Policy.manual Test Capture.requested
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

## Tab: platform

### screen-capability-guide-bullet-core-terms

#### screen-capability-guide-bullet-core-terms-screen-evidence

431.  Represent platform capability and permission check;.

- settingId: `screen-capability-guide-bullet-core-terms-screen-evidence-0002`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-core-terms`; groupId: `screen-capability-guide-bullet-core-terms-screen-evidence`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 27; sourceText: platform capability and permission check;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-screen-recording-possibilities-and-limits

#### screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits

432.  Represent platform consent prompts and indicators are common and must not be bypassed;.

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0068`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 204; sourceText: platform consent prompts and indicators are common and must not be bypassed;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

433.  Represent iOS and Android have especially strong user-consent and OS-policy limits.

- settingId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits-0069`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits`; groupId: `screen-capability-guide-bullet-screen-recording-possibilities-and-limits-screen-recording-possibilities-and-limits`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 205; sourceText: iOS and Android have especially strong user-consent and OS-policy limits.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-managed-browser-or-window-capture

#### screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture

434.  Use exclude the Ocentra app window where the platform supports exclusion filters;?

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0072`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-managed-browser-or-window-capture`; groupId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 216; sourceText: exclude the Ocentra app window where the platform supports exclusion filters;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

435.  Use window capture can miss popups, overlays, system prompts, or secondary windows;?

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0076`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-managed-browser-or-window-capture`; groupId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 224; sourceText: window capture can miss popups, overlays, system prompts, or secondary windows;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

436.  Use active window capture can break on virtual desktops, minimized windows, DRM/protected content, or permission changes?

- settingId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture-0078`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-managed-browser-or-window-capture`; groupId: `screen-capability-guide-bullet-managed-browser-or-window-capture-managed-browser-or-window-capture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 226; sourceText: active window capture can break on virtual desktops, minimized windows, DRM/protected content, or permission changes.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-guide-bullet-platform-capability-notes

#### screen-capability-guide-bullet-platform-capability-notes-windows

437.  Use windows Graphics Capture for display or application-window capture with system UI consent;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0142`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 373; sourceText: Windows Graphics Capture for display or application-window capture with system UI consent;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

438.  Use screenshot or frame capture from an approved display/window scope;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0143`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: encrypted-temporary-queue-and-raw-capture-deletion-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 374; sourceText: screenshot or frame capture from an approved display/window scope;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

439.  Represent foreground process/window evidence from the Rust agent;.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0144`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 375; sourceText: foreground process/window evidence from the Rust agent;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

440.  Represent managed browser/window correlation through managed browser evidence and process/window refs.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0148`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 379; sourceText: managed browser/window correlation through managed browser evidence and process/window refs.
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

441.  Use consent, notification border, packaged app identity, service/session boundaries, and user desktop state affect what can be captured;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0150`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 384; sourceText: consent, notification border, packaged app identity, service/session boundaries, and user desktop state affect what can be captured;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

442.  Use service capture from a non-interactive session is not the same as user desktop capture;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-windows-0152`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 388; sourceText: service capture from a non-interactive session is not the same as user desktop capture;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-macos

443.  Use screenCaptureKit display/app/window streams;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0154`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 398; sourceText: ScreenCaptureKit display/app/window streams;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

444.  Use macOS Screen Recording permission;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0155`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 399; sourceText: macOS Screen Recording permission;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

445.  Use process/window correlation where permissions and APIs allow;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0157`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 401; sourceText: process/window correlation where permissions and APIs allow;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

446.  Use screen Recording permission, app restart after first grant, sandboxing, app bundle identity, TCC state, and signing/notarization affect real behavior;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-macos-0159`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 406; sourceText: Screen Recording permission, app restart after first grant, sandboxing, app bundle identity, TCC state, and signing/notarization affect real behavior;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-linux

447.  Use xDG Desktop Portal ScreenCast for monitors, windows, or virtual sources where a portal backend supports them;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0162`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `portal-only`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 418; sourceText: XDG Desktop Portal ScreenCast for monitors, windows, or virtual sources where a portal backend supports them;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

448.  Use pipeWire stream capture on Wayland-backed desktops;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0163`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 420; sourceText: PipeWire stream capture on Wayland-backed desktops;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

449.  Use x11 screenshot paths where still supported;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0164`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 421; sourceText: X11 screenshot paths where still supported;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

450.  Use process/window correlation depending on desktop environment?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0166`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 423; sourceText: process/window correlation depending on desktop environment.
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

451.  Use wayland commonly requires a portal-mediated user selection flow;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-linux-0167`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `portal-only`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 427; sourceText: Wayland commonly requires a portal-mediated user selection flow;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

#### screen-capability-guide-bullet-platform-capability-notes-android

452.  Use mediaProjection for screen or, on modern Android, selected app-window sharing;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0172`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 439; sourceText: MediaProjection for screen or, on modern Android, selected app-window sharing;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

453.  Use foreground service requirements for active capture;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0173`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 440; sourceText: foreground service requirements for active capture;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

454.  Use usageStats, accessibility, VPN/DNS, device owner, or managed profile only where explicitly approved and enabled;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0174`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 441; sourceText: UsageStats, accessibility, VPN/DNS, device owner, or managed profile only where explicitly approved and enabled;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

455.  Represent package lifecycle and policy state from DevicePolicyManager where device-owner/profile-owner setup exists.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0176`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 444; sourceText: package lifecycle and policy state from DevicePolicyManager where device-owner/profile-owner setup exists.
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

456.  Use mediaProjection requires user consent and can be revoked;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0177`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 449; sourceText: MediaProjection requires user consent and can be revoked;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

457.  Use android 14 app-window sharing can restrict capture to a selected app and exclude system UI;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0178`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 450; sourceText: Android 14 app-window sharing can restrict capture to a selected app and exclude system UI;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

458.  Represent normal apps cannot silently monitor arbitrary screen content in the background as a parental-control agent;.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0179`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 451; sourceText: normal apps cannot silently monitor arbitrary screen content in the background as a parental-control agent;
- acceptedOptions: Configured | Unavailable
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

459.  Represent device-owner/profile-owner state changes what is possible;.

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0180`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 453; sourceText: device-owner/profile-owner state changes what is possible;
- acceptedOptions: Configured | Unavailable
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

460.  Use screen capture may be disabled by policy or protected by app/window flags;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-android-0181`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 454; sourceText: screen capture may be disabled by policy or protected by app/window flags;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

#### screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados

461.  Use replayKit for user-initiated app/screen recording or broadcasting flows;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0184`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 464; sourceText: ReplayKit for user-initiated app/screen recording or broadcasting flows;
- acceptedOptions: Enabled | Disabled
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

462.  Use replayKit is user-consent oriented and not a stealth child-monitoring API;?

- settingId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados-0188`
- policyLane: `platform`; sectionId: `screen-capability-guide-bullet-platform-capability-notes`; groupId: `screen-capability-guide-bullet-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-capability-guide.md
- sourceLine: 471; sourceText: ReplayKit is user-consent oriented and not a stealth child-monitoring API;
- acceptedOptions: Enabled | Disabled
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

### screen-authoring-field-authoring-manifest-capture-scope

#### screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields

463.  Which capture scopes are allowed?

- settingId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields-0311`
- policyLane: `platform`; sectionId: `screen-authoring-field-authoring-manifest-capture-scope`; groupId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 250; sourceText: Which capture scopes are allowed?
- acceptedOptions: Full Screen | Active Display | Active Window | Selected App Window | Managed Browser Window | Manual Parent Test Only | Default active-window | Default managed-browser-window
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

464.  What scope should be tried first?

- settingId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields-0312`
- policyLane: `platform`; sectionId: `screen-authoring-field-authoring-manifest-capture-scope`; groupId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 265; sourceText: What scope should be tried first?
- acceptedOptions: Active Window | Managed Browser Window | Active Display | Full Screen | Manual Parent Test Only | Default active-window
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

465.  What should happen on protected surfaces?

- settingId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields-0313`
- policyLane: `platform`; sectionId: `screen-authoring-field-authoring-manifest-capture-scope`; groupId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 279; sourceText: What should happen on protected surfaces?
- acceptedOptions: Skip And Audit | Delete Partial And Audit | Pause Until Clear | Mark Unavailable | Default skip-and-audit
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

466.  Require app or window evidence before policy can use screen summaries?

- settingId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields-0314`
- policyLane: `platform`; sectionId: `screen-authoring-field-authoring-manifest-capture-scope`; groupId: `screen-authoring-field-authoring-manifest-capture-scope-capture-scope-fields`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 287; sourceText: Require app or window evidence before policy can use screen summaries?
- acceptedOptions: Enabled | Disabled | Default true
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-condition-kind-authoring-manifest-metadata

#### screen-condition-kind-authoring-manifest-metadata-condition-kinds

467.  Represent condition kind: platformIn.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0381`
- policyLane: `platform`; sectionId: `screen-condition-kind-authoring-manifest-metadata`; groupId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 139; sourceText: Condition kind: platformIn.
- acceptedOptions: PlatformIn
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

468.  Represent condition kind: permissionStateIn.

- settingId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds-0383`
- policyLane: `platform`; sectionId: `screen-condition-kind-authoring-manifest-metadata`; groupId: `screen-condition-kind-authoring-manifest-metadata-condition-kinds`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 141; sourceText: Condition kind: permissionStateIn.
- acceptedOptions: PermissionStateIn
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-capability-state-meaning-capability-registry

#### screen-capability-state-meaning-capability-registry-capability-state-meanings

469.  Show unsupported-platform capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0399`
- policyLane: `platform`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1657; sourceText: unsupported-platform: Current platform cannot support this capability in the current build.
- acceptedOptions: Unsupported Platform
- helperText: Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.

470.  Show permission-required capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0401`
- policyLane: `platform`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1659; sourceText: permission-required: OS permission, user consent, management state, or entitlement is required before capture.
- acceptedOptions: Permission Required
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

471.  Show permission-limited capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0402`
- policyLane: `platform`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1660; sourceText: permission-limited: Permission exists but does not cover the requested scope.
- acceptedOptions: Permission Limited
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

472.  Show protected-surface capability state?

- settingId: `screen-capability-state-meaning-capability-registry-capability-state-meanings-0403`
- policyLane: `platform`; sectionId: `screen-capability-state-meaning-capability-registry`; groupId: `screen-capability-state-meaning-capability-registry-capability-state-meanings`
- cardKind: `status-card`; selectionMode: `status`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1661; sourceText: protected-surface: Secure, locked, credential, DRM, or OS-protected surface prevents usable capture.
- acceptedOptions: Protected Surface
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

### screen-policy-fallback-policy-value-document

#### screen-policy-fallback-policy-value-document-fallbacks

473.  Choose fallback permissionRequired.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0432`
- policyLane: `platform`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1202; sourceText: Fallback permissionRequired: show-setup-required.
- acceptedOptions: Show Setup Required
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.

474.  Choose fallback permissionLimited.

- settingId: `screen-policy-fallback-policy-value-document-fallbacks-0434`
- policyLane: `platform`; sectionId: `screen-policy-fallback-policy-value-document`; groupId: `screen-policy-fallback-policy-value-document-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-limited`; runtimeOwner: `os-adapter`; capabilityState: `permission-limited`
- proofRequirement: real-platform-capture-permission-and-scope-proof
- sourceDocument: docs/screen-evidence-analysis-schema-proposal.md
- sourceLine: 1204; sourceText: Fallback permissionLimited: mark-degraded.
- acceptedOptions: Mark Degraded
- helperText: Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.
