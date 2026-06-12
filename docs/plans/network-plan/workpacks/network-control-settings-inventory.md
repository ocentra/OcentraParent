<!-- agent-capsule -->

> Agent Capsule
> Doc: Network Control Settings Inventory
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Network Control Settings Inventory

Generated from `BaselineNetworkControlCatalog`.
Total settings: 363

Use this as the raw review list for deciding parent-facing grouping, proof gaps, and policy UX.
This is a generated inventory of current typed catalog data, not product-complete implementation proof.

## Source Documents

- docs/network-control-capability-guide.md
- docs/network-control-schema-proposal.md

## Tab: rules

### network-management

#### network-management-controls

1.  Enable network management?

- settingId: `network.enabled`
- policyLane: `rules`; sectionId: `network-management`; groupId: `network-management-controls`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 144; sourceText: Enable network management?
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

2.  What should happen to network activity?

- settingId: `network.defaultPosture`
- policyLane: `rules`; sectionId: `network-management`; groupId: `network-management-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 166; sourceText: What should happen to network activity?
- acceptedOptions: Allow | Observe | Warn | Ask | Limit | Block
- helperText: network-control-capability-registry

3.  How should network management run on this device?

- settingId: `network.managementMode`
- policyLane: `rules`; sectionId: `network-management`; groupId: `network-management-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 215; sourceText: How should network management run on this device?
- acceptedOptions: Local Child Agent | Lan Live | Authoring Only | Unavailable
- helperText: network-control-capability-registry

### network-guide-core-terms

#### network-guide-core-terms-network-control

4.  Represent keep local-network exceptions for printers, LAN pairing, parental devices, and trusted home services?

- settingId: `network-guide-core-terms-network-control-001-014`
- policyLane: `rules`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-control`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 74; sourceText: keep local-network exceptions for printers, LAN pairing, parental devices, and trusted home services.
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

#### network-guide-core-terms-local-network-exception

5.  Represent school/home subnet where the parent explicitly allows discovery;?

- settingId: `network-guide-core-terms-local-network-exception-001-018`
- policyLane: `rules`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-local-network-exception`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 97; sourceText: school/home subnet where the parent explicitly allows discovery;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### local-network

#### local-network-controls

6.  How should local-network exceptions behave?

- settingId: `localNetwork.exceptionMode`
- policyLane: `rules`; sectionId: `local-network`; groupId: `local-network-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 566; sourceText: How should local-network exceptions behave?
- acceptedOptions: Disabled | Explicit Services Only | Trusted Subnets | Allow Private Networks | Parent Request For New Local Destination
- helperText: network-control-capability-registry

7.  Which local exceptions should be available?

- settingId: `localNetwork.defaultExceptions`
- policyLane: `rules`; sectionId: `local-network`; groupId: `local-network-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 580; sourceText: Which local exceptions should be available?
- acceptedOptions: Loopback Agent | Lan Parent Controller | Dns Resolver | Dhcp | Printer | Nas | School Subnet | Mdns | Ssdp | Router Admin
- helperText: network-control-capability-registry

### network-guide-the-main-capability-truth

#### network-guide-the-main-capability-truth-the-main-capability-truth

8.  Represent lAN versus internet classification;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-023`
- policyLane: `rules`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 119; sourceText: LAN versus internet classification;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

9.  Represent page title or active tab;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-029`
- policyLane: `rules`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 129; sourceText: page title or active tab;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

10. Represent page body, chat content, search terms, form values, cookies, tokens, or credentials;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-030`
- policyLane: `rules`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network controls must not collect decrypted content or payload fields; use metadata evidence only.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 130; sourceText: page body, chat content, search terms, form values, cookies, tokens, or credentials;
- acceptedOptions: Enabled | Disabled
- helperText: Network controls must not collect decrypted content or payload fields; use metadata evidence only.

11. Represent specific video or post within a CDN-backed service;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-031`
- policyLane: `rules`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 132; sourceText: specific video or post within a CDN-backed service;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

12. Represent which tab caused a network request;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-032`
- policyLane: `rules`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 133; sourceText: which tab caused a network request;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

13. Represent child intent or safety classification without a separate policy/AI contract?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-034`
- policyLane: `rules`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 135; sourceText: child intent or safety classification without a separate policy/AI contract.
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### network-guide-network-visibility-what-is-possible

#### network-guide-network-visibility-what-is-possible-lan-versus-internet

14. Represent loopback;?

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-082`
- policyLane: `rules`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-lan-versus-internet`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 227; sourceText: loopback;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

15. Represent local agent service;?

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-083`
- policyLane: `rules`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-lan-versus-internet`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 228; sourceText: local agent service;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

16. Represent lAN parent controller;?

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-084`
- policyLane: `rules`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-lan-versus-internet`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 229; sourceText: LAN parent controller;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

17. Represent local subnet;?

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-085`
- policyLane: `rules`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-lan-versus-internet`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 230; sourceText: local subnet;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

18. Represent public internet;?

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-087`
- policyLane: `rules`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-lan-versus-internet`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 232; sourceText: public internet;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

19. Represent unknown route;?

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-088`
- policyLane: `rules`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-lan-versus-internet`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 233; sourceText: unknown route;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### network-guide-network-control-layers

#### network-guide-network-control-layers-cloud-relay

20. Represent remote rule update delivery;?

- settingId: `network-guide-network-control-layers-cloud-relay-001-151`
- policyLane: `rules`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-cloud-relay`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 391; sourceText: remote rule update delivery;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

21. Represent control of arbitrary child internet traffic;?

- settingId: `network-guide-network-control-layers-cloud-relay-001-156`
- policyLane: `rules`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-cloud-relay`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 399; sourceText: control of arbitrary child internet traffic;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### network-guide-modern-network-limits

#### network-guide-modern-network-limits-private-relay-and-platform-privacy-features

22. Represent allow;?

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-166`
- policyLane: `rules`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 445; sourceText: allow;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

23. Represent observe;?

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-167`
- policyLane: `rules`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 446; sourceText: observe;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

24. Represent warn;?

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-168`
- policyLane: `rules`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 447; sourceText: warn;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### network-guide-platform-capability-notes

#### network-guide-platform-capability-notes-windows

25. Represent eTW for network event streams if loss/decode/privilege states are typed;?

- settingId: `network-guide-platform-capability-notes-windows-001-222`
- policyLane: `rules`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 540; sourceText: ETW for network event streams if loss/decode/privilege states are typed;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

26. Represent product claims should follow real host proof, not contract presence?

- settingId: `network-guide-platform-capability-notes-windows-001-231`
- policyLane: `rules`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 555; sourceText: product claims should follow real host proof, not contract presence.
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### network-guide-platform-capability-notes-macos

27. Represent consumer child-agent claims must stay behind Apple-approved capabilities?

- settingId: `network-guide-platform-capability-notes-macos-001-239`
- policyLane: `rules`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 575; sourceText: consumer child-agent claims must stay behind Apple-approved capabilities.
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### network-guide-platform-capability-notes-linux

28. Represent desktop foreground and app identity vary;?

- settingId: `network-guide-platform-capability-notes-linux-001-244`
- policyLane: `rules`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 591; sourceText: desktop foreground and app identity vary;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

29. Represent claims need distro-specific validation?

- settingId: `network-guide-platform-capability-notes-linux-001-247`
- policyLane: `rules`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 594; sourceText: claims need distro-specific validation.
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### network-guide-platform-capability-notes-android

30. Represent usage Stats, accessibility, or browser/app-specific integrations for foreground/app context when explicitly approved?

- settingId: `network-guide-platform-capability-notes-android-001-252`
- policyLane: `rules`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 608; sourceText: Usage Stats, accessibility, or browser/app-specific integrations for foreground/app context when explicitly approved.
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### network-guide-policy-modes-to-represent-later-in-ui

#### network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity

31. Represent decrypted content;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-282`
- policyLane: `rules`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Network controls must not collect decrypted content or payload fields; use metadata evidence only.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 688; sourceText: decrypted content;
- acceptedOptions: Enabled | Disabled
- helperText: Network controls must not collect decrypted content or payload fields; use metadata evidence only.

#### network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions

32. Represent exact exception scope;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions-001-303`
- policyLane: `rules`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 753; sourceText: exact exception scope;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

33. Represent risk of broad private-network allow rules?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions-001-305`
- policyLane: `rules`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 755; sourceText: risk of broad private-network allow rules.
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### network-guide-current-ocentra-parent-posture

#### network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

34. Represent [`docs/expectations/policy.md`](../../../expectations/policy.md)?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-321`
- policyLane: `rules`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 793; sourceText: [`docs/expectations/policy.md`](../../../expectations/policy.md)
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

35. Represent [`docs/product-roadmap.md`](../../../product-roadmap.md)?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-324`
- policyLane: `rules`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 796; sourceText: [`docs/product-roadmap.md`](../../../product-roadmap.md)
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### network-guide-future-ui-rules

#### network-guide-future-ui-rules-future-ui-rules

36. Represent keep LAN exceptions visible beside strict rules?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-330`
- policyLane: `rules`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 810; sourceText: Keep LAN exceptions visible beside strict rules.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

37. Represent observe only;?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-334`
- policyLane: `rules`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 821; sourceText: observe only;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

38. Represent local network exceptions;?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-339`
- policyLane: `rules`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 826; sourceText: local network exceptions;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

## Tab: evidence

### network-guide-core-terms

#### network-guide-core-terms-network-flow-evidence

39. Represent local IP and port;?

- settingId: `network-guide-core-terms-network-flow-evidence-001-001`
- policyLane: `evidence`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-flow-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 27; sourceText: local IP and port;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

40. Represent remote IP and port;?

- settingId: `network-guide-core-terms-network-flow-evidence-001-002`
- policyLane: `evidence`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-flow-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 28; sourceText: remote IP and port;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

41. Represent transport protocol;?

- settingId: `network-guide-core-terms-network-flow-evidence-001-003`
- policyLane: `evidence`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-flow-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 29; sourceText: transport protocol;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

42. Represent tCP state;?

- settingId: `network-guide-core-terms-network-flow-evidence-001-004`
- policyLane: `evidence`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-flow-evidence`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 30; sourceText: TCP state;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

43. Represent process id and process identity where available;?

- settingId: `network-guide-core-terms-network-flow-evidence-001-005`
- policyLane: `evidence`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-flow-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 31; sourceText: process id and process identity where available;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

44. Represent dNS/domain attribution where available;?

- settingId: `network-guide-core-terms-network-flow-evidence-001-006`
- policyLane: `evidence`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-flow-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 32; sourceText: DNS/domain attribution where available;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-core-terms-local-network-exception

45. Represent loopback service ports used by the child-device agent;?

- settingId: `network-guide-core-terms-local-network-exception-001-015`
- policyLane: `evidence`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-local-network-exception`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 94; sourceText: loopback service ports used by the child-device agent;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

46. Represent lAN pairing ports between parent controller and child agent;?

- settingId: `network-guide-core-terms-local-network-exception-001-016`
- policyLane: `evidence`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-local-network-exception`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 95; sourceText: LAN pairing ports between parent controller and child agent;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

47. Represent multicast and broadcast protocols that are needed for device discovery?

- settingId: `network-guide-core-terms-local-network-exception-001-019`
- policyLane: `evidence`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-local-network-exception`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 98; sourceText: multicast and broadcast protocols that are needed for device discovery.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### evidence-scope

#### evidence-scope-controls

48. What network evidence may rules use?

- settingId: `evidence.metadataScope`
- policyLane: `evidence`; sectionId: `evidence-scope`; groupId: `evidence-scope-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 238; sourceText: What network evidence may rules use?
- acceptedOptions: Process | Ip | Port | Protocol | Domain Candidate | Dns Source State | Connection Count | Duration | Bytes When Available | Interface | Route | Lan Wan Classification | Vpn Proxy Tunnel Indicator | Adapter State
- helperText: network-control-capability-registry

49. What proof is enough for network decisions?

- settingId: `evidence.requiredProof`
- policyLane: `evidence`; sectionId: `evidence-scope`; groupId: `evidence-scope-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 272; sourceText: What proof is enough for network decisions?
- acceptedOptions: Capability Only | Endpoint Observed | Process Attributed Flow | Domain Candidate | Domain Known | Stored Flow Summary | Managed Browser Joined Domain | Adapter Enforcement Proof
- helperText: network-control-capability-registry

50. What if network proof is unavailable?

- settingId: `evidence.whenProofUnavailable`
- policyLane: `evidence`; sectionId: `evidence-scope`; groupId: `evidence-scope-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 289; sourceText: What if network proof is unavailable?
- acceptedOptions: Allow | Observe | Warn | Ask | Block Until Ready | Mark Unavailable
- helperText: network-control-capability-registry

51. What must network rules never collect?

- settingId: `evidence.neverCollect`
- policyLane: `evidence`; sectionId: `evidence-scope`; groupId: `evidence-scope-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 297; sourceText: What must network rules never collect?
- acceptedOptions: Decrypted Https Payload | Packet Payload | Page Body | Chat Content | Search Terms | Form Values | Cookies | Tokens | Credentials | Raw Packet Dumps | Raw Trace Files
- helperText: network-control-capability-registry

### domain-dns

#### domain-dns-controls

52. How should domain controls work?

- settingId: `dns.mode`
- policyLane: `evidence`; sectionId: `domain-dns`; groupId: `domain-dns-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 340; sourceText: How should domain controls work?
- acceptedOptions: Disabled | Observe And Classify | Managed Resolver Preferred | Managed Resolver Required | Block Unapproved Encrypted Dns
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

53. Which domain attribution sources may be used?

- settingId: `dns.allowedAttributionSources`
- policyLane: `evidence`; sectionId: `domain-dns`; groupId: `domain-dns-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 354; sourceText: Which domain attribution sources may be used?
- acceptedOptions: Observed Dns Event | Dns Client Cache | Managed Resolver Log | Reverse Dns | Static Hosts | Managed Browser Join | Parent Rule Domain
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

54. How should encrypted DNS be handled?

- settingId: `dns.encryptedDnsHandling`
- policyLane: `evidence`; sectionId: `domain-dns`; groupId: `domain-dns-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 370; sourceText: How should encrypted DNS be handled?
- acceptedOptions: Allow | Report Only | Warn | Ask | Block Unknown Resolvers | Require Managed Resolver
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### flow-rules

#### flow-rules-controls

55. What network targets should rules match?

- settingId: `rules.allowedTargetTypes`
- policyLane: `evidence`; sectionId: `flow-rules`; groupId: `flow-rules-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 397; sourceText: What network targets should rules match?
- acceptedOptions: Domain | Ip | Cidr | Port | Protocol | Process | Interface | Route | Destination Category | Vpn Proxy Tunnel | New Destination | High Volume | Repeated Failure | Capability State
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

56. What actions can network rules take?

- settingId: `rules.allowedActions`
- policyLane: `evidence`; sectionId: `flow-rules`; groupId: `flow-rules-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 429; sourceText: What actions can network rules take?
- acceptedOptions: Allow | Observe | Warn | Ask | Limit | Block | Terminate Process | Require Managed Network
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

57. How should conflicting network rules resolve?

- settingId: `rules.conflictResolution`
- policyLane: `evidence`; sectionId: `flow-rules`; groupId: `flow-rules-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 446; sourceText: How should conflicting network rules resolve?
- acceptedOptions: Explicit Local Exception Beats Block | Process Rule Beats Domain Rule | Domain Beats Ip Category | Managed Browser Domain Beats Dns Candidate | Block Beats Allow | Fresh Proof Beats Stale Proof | Adapter Proof Required For Enforcement
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### reports

#### reports-controls

58. Which network report fields should parents see?

- settingId: `reports.visibleFields`
- policyLane: `evidence`; sectionId: `reports`; groupId: `reports-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 610; sourceText: Which network report fields should parents see?
- acceptedOptions: Recent Flows | Top Processes | Top Domains | Top Ips | Ports Protocols | Bandwidth | Time Budgets | New Destinations | Repeated Failures | Vpn Proxy Tunnel Indicators | Local Exceptions Used | Policy Decisions | Block Results | Source Capability | Custody Label
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

59. Show unknown and ambiguous states?

- settingId: `reports.showUncertainty`
- policyLane: `evidence`; sectionId: `reports`; groupId: `reports-controls`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 644; sourceText: Show unknown and ambiguous states?
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-the-main-capability-truth

#### network-guide-the-main-capability-truth-the-main-capability-truth

60. Represent iP, port, protocol, and process rules;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-021`
- policyLane: `evidence`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 117; sourceText: IP, port, protocol, and process rules;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

61. Represent unusual new destination reporting;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-025`
- policyLane: `evidence`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 121; sourceText: unusual new destination reporting;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

62. Represent local AI/policy digests with evidence ids;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-026`
- policyLane: `evidence`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 122; sourceText: local AI/policy digests with evidence ids;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

63. Represent app/browser correlation when the flow also has process or managed-browser evidence?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-027`
- policyLane: `evidence`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 123; sourceText: app/browser correlation when the flow also has process or managed-browser evidence.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

64. Represent exact URL path or query in normal HTTPS;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-028`
- policyLane: `evidence`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 128; sourceText: exact URL path or query in normal HTTPS;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

### network-guide-capability-matrix

#### network-guide-capability-matrix-capability-matrix

65. Represent Detect remote port capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-036`
- policyLane: `evidence`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 145; sourceText: Capability matrix row | Capability=Detect remote port | What can be possible=Yes where endpoint/packet metadata is exposed | Required layer=Endpoint/flow observation | Important limit=Port does not prove application semantics.
- acceptedOptions: Capability Detect Remote Port | What Can Be Possible Yes Where Endpoint Packet Metadata Is Exposed | Required Layer Endpoint Flow Observation | Important Limit Port Does Not Prove Application Semantics
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

66. Represent Detect protocol capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-037`
- policyLane: `evidence`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 146; sourceText: Capability matrix row | Capability=Detect protocol | What can be possible=TCP/UDP/IP protocol usually; app protocol sometimes | Required layer=Endpoint/flow observation, DPI if approved | Important limit=QUIC over UDP/443 can hide higher-level HTTP details.
- acceptedOptions: Capability Detect Protocol | What Can Be Possible Tcp Udp Ip Protocol Usually App Protocol Sometimes | Required Layer Endpoint Flow Observation Dpi If Approved | Important Limit Quic Over Udp 443 Can Hide Higher Level Http Details
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

67. Represent Cloud relay control capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-051`
- policyLane: `evidence`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 160; sourceText: Capability matrix row | Capability=Cloud relay control | What can be possible=Possible only for Ocentra protocol traffic or parent-authorized path | Required layer=Relay/control-plane contract | Important limit=Cannot control arbitrary child internet traffic by itself.
- acceptedOptions: Capability Cloud Relay Control | What Can Be Possible Possible Only For Ocentra Protocol Traffic Or Parent Authorized Path | Required Layer Relay Control Plane Contract | Important Limit Cannot Control Arbitrary Child Internet Traffic By Itself
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

68. Represent LAN exception capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-052`
- policyLane: `evidence`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 161; sourceText: Capability matrix row | Capability=LAN exception | What can be possible=Possible | Required layer=Route/interface/subnet/service policy | Important limit=Too broad an exception can hide unwanted local traffic.
- acceptedOptions: Capability Lan Exception | What Can Be Possible Possible | Required Layer Route Interface Subnet Service Policy | Important Limit Too Broad An Exception Can Hide Unwanted Local Traffic
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

69. Represent Suspicious indicator report capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-053`
- policyLane: `evidence`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 162; sourceText: Capability matrix row | Capability=Suspicious indicator report | What can be possible=Possible | Required layer=Stored network digest plus deterministic/AI labels | Important limit=Indicator must cite evidence and keep uncertainty.
- acceptedOptions: Capability Suspicious Indicator Report | What Can Be Possible Possible | Required Layer Stored Network Digest Plus Deterministic Ai Labels | Important Limit Indicator Must Cite Evidence And Keep Uncertainty
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-network-visibility-what-is-possible

#### network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol

70. Represent endpoint snapshots for local and remote IP/port/protocol;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-055`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 171; sourceText: endpoint snapshots for local and remote IP/port/protocol;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

71. Represent tCP and UDP owner PID tables where supported;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-056`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 172; sourceText: TCP and UDP owner PID tables where supported;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

72. Represent managed resolver logs where Ocentra controls the resolver path;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-058`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 174; sourceText: managed resolver logs where Ocentra controls the resolver path;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

73. Represent managed browser URL evidence when there is an explicit join to flow evidence?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-060`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 176; sourceText: managed browser URL evidence when there is an explicit join to flow evidence.
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

74. Represent `domain-known`;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-061`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 180; sourceText: `domain-known`;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

75. Represent `domain-candidate`;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-062`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 181; sourceText: `domain-candidate`;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

76. Represent `domain-ambiguous`;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-063`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 182; sourceText: `domain-ambiguous`;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

77. Represent `ip-only`;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-064`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 183; sourceText: `ip-only`;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

78. Represent `dns-unavailable`;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-065`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 184; sourceText: `dns-unavailable`;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

79. Represent `dns-stale`;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-066`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 185; sourceText: `dns-stale`;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

80. Represent `encrypted-content-unavailable`;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-067`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 186; sourceText: `encrypted-content-unavailable`;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

81. Represent `process-attributed`;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-068`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 187; sourceText: `process-attributed`;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

82. Represent `process-unknown`;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-069`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 188; sourceText: `process-unknown`;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-network-visibility-what-is-possible-process-and-app-attribution

83. Which process opened network connections?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-072`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 199; sourceText: Which process opened network connections?
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

84. Which destination did this process contact?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-073`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 200; sourceText: Which destination did this process contact?
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

85. Did an unmanaged browser or unknown app create traffic?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-074`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 201; sourceText: Did an unmanaged browser or unknown app create traffic?
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

86. Did a known app suddenly contact a new destination?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-075`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 202; sourceText: Did a known app suddenly contact a new destination?
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

87. Represent exact browser URL;?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-077`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 207; sourceText: exact browser URL;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

88. Represent active tab;?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-078`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 208; sourceText: active tab;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

89. Represent page title;?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-079`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 209; sourceText: page title;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

90. Represent user intent;?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-080`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 210; sourceText: user intent;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-network-visibility-what-is-possible-lan-versus-internet

91. Represent captive portal or public Wi-Fi;?

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-089`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-lan-versus-internet`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 234; sourceText: captive portal or public Wi-Fi;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-network-visibility-what-is-possible-suspicious-indicators

92. Represent new destination for child/device/process;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-096`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 261; sourceText: new destination for child/device/process;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

93. Represent high-volume unknown process;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-097`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 262; sourceText: high-volume unknown process;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

94. Represent repeated connection failures;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-098`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 263; sourceText: repeated connection failures;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

95. Represent dNS mismatch or excessive DNS churn;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-099`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 264; sourceText: DNS mismatch or excessive DNS churn;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

96. Represent dNS unavailable while traffic continues;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-100`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 265; sourceText: DNS unavailable while traffic continues;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

97. Represent direct IP traffic to public internet;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-101`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 266; sourceText: direct IP traffic to public internet;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

98. Represent unusual port or protocol for a child device;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-103`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 268; sourceText: unusual port or protocol for a child device;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

99. Represent domain/IP reputation category when the category source is explicit;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-104`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 269; sourceText: domain/IP reputation category when the category source is explicit;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

100.  Represent lAN scan-like pattern;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-105`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 270; sourceText: LAN scan-like pattern;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

101.  Represent traffic from an unmanaged browser or unsupported app;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-106`
- policyLane: `evidence`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 271; sourceText: traffic from an unmanaged browser or unsupported app;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-network-control-layers

#### network-guide-network-control-layers-dns-and-managed-resolver

102.  Represent parent-friendly domain rules;?

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-108`
- policyLane: `evidence`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-dns-and-managed-resolver`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 286; sourceText: parent-friendly domain rules;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

103.  Represent broad device or profile coverage when DNS path is controlled;?

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-109`
- policyLane: `evidence`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-dns-and-managed-resolver`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 287; sourceText: broad device or profile coverage when DNS path is controlled;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

104.  Represent useful reporting for domains and categories;?

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-110`
- policyLane: `evidence`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-dns-and-managed-resolver`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 288; sourceText: useful reporting for domains and categories;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

105.  Represent works even when the app is not a browser if it uses the managed resolver?

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-111`
- policyLane: `evidence`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-dns-and-managed-resolver`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 289; sourceText: works even when the app is not a browser if it uses the managed resolver.
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

106.  Represent several domains can resolve to one IP, and one domain can resolve to many IPs;?

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-113`
- policyLane: `evidence`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-dns-and-managed-resolver`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 294; sourceText: several domains can resolve to one IP, and one domain can resolve to many IPs;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

107.  Represent apps can use hard-coded IPs;?

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-114`
- policyLane: `evidence`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-dns-and-managed-resolver`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 295; sourceText: apps can use hard-coded IPs;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

108.  Represent dNS control does not see HTTPS path/query;?

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-116`
- policyLane: `evidence`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-dns-and-managed-resolver`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 298; sourceText: DNS control does not see HTTPS path/query;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

#### network-guide-network-control-layers-cloud-relay

109.  Represent report query routing;?

- settingId: `network-guide-network-control-layers-cloud-relay-001-153`
- policyLane: `evidence`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-cloud-relay`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 393; sourceText: report query routing;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

110.  Represent device reachability metadata;?

- settingId: `network-guide-network-control-layers-cloud-relay-001-154`
- policyLane: `evidence`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-cloud-relay`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 394; sourceText: device reachability metadata;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

111.  Represent stateless report compilation from parent-authorized sources?

- settingId: `network-guide-network-control-layers-cloud-relay-001-155`
- policyLane: `evidence`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-cloud-relay`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 395; sourceText: stateless report compilation from parent-authorized sources.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-modern-network-limits

#### network-guide-modern-network-limits-https-doh-quic-ech-and-cdns

112.  Represent domain rules must tolerate ambiguous or unavailable domain evidence;?

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-160`
- policyLane: `evidence`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 417; sourceText: domain rules must tolerate ambiguous or unavailable domain evidence;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

113.  Represent eCH reduces SNI-based classification, so DNS/resolver or browser evidence becomes more important;?

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-164`
- policyLane: `evidence`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 422; sourceText: ECH reduces SNI-based classification, so DNS/resolver or browser evidence becomes more important;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

114.  Represent cDN-backed sites need domain/category evidence, not IP-only overclaiming?

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-165`
- policyLane: `evidence`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 424; sourceText: CDN-backed sites need domain/category evidence, not IP-only overclaiming.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-platform-capability-notes

#### network-guide-platform-capability-notes-windows

115.  Represent iP Helper endpoint snapshots for TCP/UDP owner PID state;?

- settingId: `network-guide-platform-capability-notes-windows-001-220`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 538; sourceText: IP Helper endpoint snapshots for TCP/UDP owner PID state;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

116.  Represent process/window evidence for app/browser correlation;?

- settingId: `network-guide-platform-capability-notes-windows-001-225`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 543; sourceText: process/window evidence for app/browser correlation;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

117.  Represent managed browser evidence for exact URL/tab state?

- settingId: `network-guide-platform-capability-notes-windows-001-226`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 544; sourceText: managed browser evidence for exact URL/tab state.
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

#### network-guide-platform-capability-notes-macos

118.  Represent process observation with platform permissions;?

- settingId: `network-guide-platform-capability-notes-macos-001-234`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 566; sourceText: process observation with platform permissions;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

119.  Represent browser-managed evidence for exact URL/tab state;?

- settingId: `network-guide-platform-capability-notes-macos-001-235`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 567; sourceText: browser-managed evidence for exact URL/tab state;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

120.  Represent network Extension, System Extension, TCC, MDM, signing, notarization, and App Store review affect what is shippable;?

- settingId: `network-guide-platform-capability-notes-macos-001-237`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 572; sourceText: Network Extension, System Extension, TCC, MDM, signing, notarization, and App Store review affect what is shippable;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-platform-capability-notes-linux

121.  Represent process and socket inspection through procfs/netlink where permitted;?

- settingId: `network-guide-platform-capability-notes-linux-001-241`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 585; sourceText: process and socket inspection through procfs/netlink where permitted;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

122.  Represent managed browser evidence for exact URL/tab state?

- settingId: `network-guide-platform-capability-notes-linux-001-243`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 587; sourceText: managed browser evidence for exact URL/tab state.
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

#### network-guide-platform-capability-notes-android

123.  Represent managed DNS, app restrictions, or package lifecycle controls where the device management posture permits;?

- settingId: `network-guide-platform-capability-notes-android-001-251`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 606; sourceText: managed DNS, app restrictions, or package lifecycle controls where the device management posture permits;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

124.  Represent exact URL in arbitrary mobile browsers is not generally reliable from network metadata;?

- settingId: `network-guide-platform-capability-notes-android-001-254`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 615; sourceText: exact URL in arbitrary mobile browsers is not generally reliable from network metadata;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

#### network-guide-platform-capability-notes-ios-and-ipados

125.  Configure screen time frameworks.

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-257`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 626; sourceText: Screen Time frameworks: Family Controls, Managed Settings, Device Activity;
- acceptedOptions: Family Controls | Managed Settings | Device Activity
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

126.  Represent web domain shielding through managed settings where allowed;?

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-258`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 627; sourceText: web domain shielding through managed settings where allowed;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

127.  Represent network Extension content filter or URL filter paths where entitlement and deployment permit;?

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-259`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 628; sourceText: Network Extension content filter or URL filter paths where entitlement and deployment permit;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

128.  Represent mDM/supervision for stronger managed-device content filtering;?

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-260`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `child-agent`; capabilityState: `manual-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 630; sourceText: MDM/supervision for stronger managed-device content filtering;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

129.  Represent app/category/domain tokens rather than raw browser history in Screen Time flows?

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-261`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network controls must not collect decrypted content or payload fields; use metadata evidence only.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 631; sourceText: app/category/domain tokens rather than raw browser history in Screen Time flows.
- acceptedOptions: Represented | Not Represented
- helperText: Network controls must not collect decrypted content or payload fields; use metadata evidence only.

130.  Represent third-party apps do not get general packet inspection or arbitrary exact URL telemetry from other apps;?

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-262`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 636; sourceText: third-party apps do not get general packet inspection or arbitrary exact URL telemetry from other apps;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

131.  Represent entitlements, review, supervision, and deployment model determine capability;?

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-263`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `child-agent`; capabilityState: `manual-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 638; sourceText: entitlements, review, supervision, and deployment model determine capability;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

132.  Represent web domain shielding is not the same as full browser history capture;?

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-264`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 639; sourceText: web domain shielding is not the same as full browser history capture;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

133.  Represent parent iOS app claims and child iOS agent claims must stay separate?

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-265`
- policyLane: `evidence`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 640; sourceText: parent iOS app claims and child iOS agent claims must stay separate.
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-policy-modes-to-represent-later-in-ui

#### network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity

134.  Represent record and summarize network metadata;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-277`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 679; sourceText: record and summarize network metadata;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

135.  Represent show process, destination, protocol, port, DNS/domain, volume, and capability states where available;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-278`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 680; sourceText: show process, destination, protocol, port, DNS/domain, volume, and capability states where available;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

136.  Represent classify suspicious indicators in report-only mode?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-279`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 682; sourceText: classify suspicious indicators in report-only mode.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

137.  Represent exact URL rules;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-281`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 687; sourceText: exact URL rules;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

138.  Represent guaranteed process attribution on every platform?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-283`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 689; sourceText: guaranteed process attribution on every platform.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-policy-modes-to-represent-later-in-ui-domain-rules

139.  Represent use managed browser evidence for exact domain/origin when available;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-285`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 697; sourceText: use managed browser evidence for exact domain/origin when available;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

140.  Represent use DNS/network attribution with confidence when browser evidence is absent?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-286`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 698; sourceText: use DNS/network attribution with confidence when browser evidence is absent.
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

141.  Represent full URL path/query control;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-287`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 702; sourceText: full URL path/query control;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

#### network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules

142.  Represent can break legitimate infrastructure;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules-001-291`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 715; sourceText: can break legitimate infrastructure;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

143.  Represent can miss app semantics;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules-001-292`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `parent-domain`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 716; sourceText: can miss app semantics;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

144.  Represent can be too broad for CDN-backed services?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules-001-293`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 717; sourceText: can be too broad for CDN-backed services.
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions

145.  Represent allow specific local services, subnets, protocols, or Ocentra pairing traffic while internet rules remain strict?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions-001-302`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 748; sourceText: allow specific local services, subnets, protocols, or Ocentra pairing traffic while internet rules remain strict.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

146.  Represent last used evidence;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions-001-304`
- policyLane: `evidence`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 754; sourceText: last used evidence;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### network-guide-current-ocentra-parent-posture

#### network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

147.  Represent network flow evidence is local-first metadata, not decrypted content?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-312`
- policyLane: `evidence`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Network controls must not collect decrypted content or payload fields; use metadata evidence only.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 776; sourceText: Network flow evidence is local-first metadata, not decrypted content.
- acceptedOptions: Enabled | Disabled
- helperText: Network controls must not collect decrypted content or payload fields; use metadata evidence only.

148.  Represent browser URL/tab evidence remains the exact URL source?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-313`
- policyLane: `evidence`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 777; sourceText: Browser URL/tab evidence remains the exact URL source.
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

149.  Represent lAN and cloud relay are typed control/report paths, not default hosted child evidence stores?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-317`
- policyLane: `evidence`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 784; sourceText: LAN and cloud relay are typed control/report paths, not default hosted child evidence stores.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

150.  Represent [`docs/architecture/network-flow-evidence-capture.md`](../../../architecture/network-flow-evidence-capture.md)?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-319`
- policyLane: `evidence`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 791; sourceText: [`docs/architecture/network-flow-evidence-capture.md`](../../../architecture/network-flow-evidence-capture.md)
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

151.  Represent [`docs/expectations/network-flow-evidence.md`](../../../expectations/network-flow-evidence.md)?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-320`
- policyLane: `evidence`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 792; sourceText: [`docs/expectations/network-flow-evidence.md`](../../../expectations/network-flow-evidence.md)
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-future-ui-rules

#### network-guide-future-ui-rules-future-ui-rules

152.  Represent show exact URL controls only when managed browser or explicit URL-filter capability is available?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-326`
- policyLane: `evidence`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 803; sourceText: Show exact URL controls only when managed browser or explicit URL-filter capability is available.
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

153.  Represent show domain rules as domain evidence, not exact URL evidence?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-327`
- policyLane: `evidence`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 805; sourceText: Show domain rules as domain evidence, not exact URL evidence.
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

154.  Represent domain rules;?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-335`
- policyLane: `evidence`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 822; sourceText: domain rules;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

155.  Represent iP/port/protocol rules;?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-336`
- policyLane: `evidence`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 823; sourceText: IP/port/protocol rules;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

156.  Represent managed browser for exact URL rules;?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-341`
- policyLane: `evidence`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 828; sourceText: managed browser for exact URL rules;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

## Tab: enforcement

### network-guide-core-terms

#### network-guide-core-terms-network-flow-evidence

157.  Represent interface, route, LAN/WAN, VPN, proxy, or tunnel indicators;?

- settingId: `network-guide-core-terms-network-flow-evidence-001-007`
- policyLane: `enforcement`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-flow-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 33; sourceText: interface, route, LAN/WAN, VPN, proxy, or tunnel indicators;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-core-terms-network-control

158.  Represent allow, block, or rate-limit traffic by process, IP, port, protocol, domain, or category where supported;?

- settingId: `network-guide-core-terms-network-control-001-010`
- policyLane: `enforcement`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-control`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 68; sourceText: allow, block, or rate-limit traffic by process, IP, port, protocol, domain, or category where supported;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

159.  Represent force or configure DNS, proxy, VPN, firewall, WFP, packet filter, or router policy;?

- settingId: `network-guide-core-terms-network-control-001-011`
- policyLane: `enforcement`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-control`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 70; sourceText: force or configure DNS, proxy, VPN, firewall, WFP, packet filter, or router policy;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

160.  Represent terminate a process after a network policy decision;?

- settingId: `network-guide-core-terms-network-control-001-012`
- policyLane: `enforcement`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-control`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 72; sourceText: terminate a process after a network policy decision;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-core-terms-local-network-exception

161.  Represent local printer, NAS, DNS resolver, router, or media device;?

- settingId: `network-guide-core-terms-local-network-exception-001-017`
- policyLane: `enforcement`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-local-network-exception`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 96; sourceText: local printer, NAS, DNS resolver, router, or media device;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### vpn-proxy-tunnel

#### vpn-proxy-tunnel-controls

162.  What should happen to VPN/proxy/tunnel indicators?

- settingId: `tunnel.mode`
- policyLane: `enforcement`; sectionId: `vpn-proxy-tunnel`; groupId: `vpn-proxy-tunnel-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 480; sourceText: What should happen to VPN/proxy/tunnel indicators?
- acceptedOptions: Allow | Observe | Warn | Ask | Block | Require Managed Network
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

163.  Which tunnel indicators should count?

- settingId: `tunnel.indicators`
- policyLane: `enforcement`; sectionId: `vpn-proxy-tunnel`; groupId: `vpn-proxy-tunnel-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 488; sourceText: Which tunnel indicators should count?
- acceptedOptions: Vpn Interface | Proxy Process | Proxy Config | Known Tunnel Port | Tor Like Process | Unknown Encrypted Relay | Masque Like Flow | Dns Unavailable With Public Traffic
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-the-main-capability-truth

#### network-guide-the-main-capability-truth-the-main-capability-truth

164.  Represent domain allow/block rules;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-020`
- policyLane: `enforcement`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 116; sourceText: domain allow/block rules;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

165.  Represent vPN/proxy/tunnel indicators;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-022`
- policyLane: `enforcement`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 118; sourceText: VPN/proxy/tunnel indicators;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-capability-matrix

#### network-guide-capability-matrix-capability-matrix

166.  Represent Detect remote IP capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-035`
- policyLane: `enforcement`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 144; sourceText: Capability matrix row | Capability=Detect remote IP | What can be possible=Yes on most endpoint, firewall, VPN, router, or flow adapters | Required layer=Endpoint/flow observation | Important limit=IP alone may be CDN/shared, NATed, private, or anycast.
- acceptedOptions: Capability Detect Remote Ip | What Can Be Possible Yes On Most Endpoint Firewall Vpn Router Or Flow Adapters | Required Layer Endpoint Flow Observation | Important Limit Ip Alone May Be Cdn Shared Nated Private Or Anycast
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

167.  Represent Attribute process capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-038`
- policyLane: `enforcement`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 147; sourceText: Capability matrix row | Capability=Attribute process | What can be possible=Often on Windows endpoint adapters; varies elsewhere | Required layer=OS endpoint/process adapter | Important limit=Router/DNS-only data usually cannot identify the local process.
- acceptedOptions: Capability Attribute Process | What Can Be Possible Often On Windows Endpoint Adapters Varies Elsewhere | Required Layer Os Endpoint Process Adapter | Important Limit Router Dns Only Data Usually Cannot Identify The Local Process
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

168.  Represent Attribute exact URL capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-040`
- policyLane: `enforcement`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 149; sourceText: Capability matrix row | Capability=Attribute exact URL | What can be possible=No from network alone | Required layer=Managed browser, explicit URL filter, proxy | Important limit=Normal HTTPS hides path/query from passive network observers.
- acceptedOptions: Capability Attribute Exact Url | What Can Be Possible No From Network Alone | Required Layer Managed Browser Explicit Url Filter Proxy | Important Limit Normal Https Hides Path Query From Passive Network Observers
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

169.  Represent Block IP capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-041`
- policyLane: `enforcement`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 150; sourceText: Capability matrix row | Capability=Block IP | What can be possible=Often possible | Required layer=Firewall, WFP, VPN, packet filter, router | Important limit=Shared IP/CDN can overblock unrelated services.
- acceptedOptions: Capability Block Ip | What Can Be Possible Often Possible | Required Layer Firewall Wfp Vpn Packet Filter Router | Important Limit Shared Ip Cdn Can Overblock Unrelated Services
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

170.  Represent Block exact URL capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-043`
- policyLane: `enforcement`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 152; sourceText: Capability matrix row | Capability=Block exact URL | What can be possible=Possible only with browser/proxy/URL filter proof | Required layer=Managed browser, explicit URL filter, HTTP proxy | Important limit=Not a generic firewall claim. HTTPS interception is out of current scope.
- acceptedOptions: Capability Block Exact Url | What Can Be Possible Possible Only With Browser Proxy Url Filter Proof | Required Layer Managed Browser Explicit Url Filter Http Proxy | Important Limit Not A Generic Firewall Claim Https Interception Is Out Of Current Scope
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

171.  Represent Block process network capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-044`
- policyLane: `enforcement`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 153; sourceText: Capability matrix row | Capability=Block process network | What can be possible=Possible on some OS adapters | Required layer=WFP/firewall/app control/VPN with app binding | Important limit=Requires platform proof and robust process identity.
- acceptedOptions: Capability Block Process Network | What Can Be Possible Possible On Some Os Adapters | Required Layer Wfp Firewall App Control Vpn With App Binding | Important Limit Requires Platform Proof And Robust Process Identity
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

172.  Represent Block protocol or port capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-045`
- policyLane: `enforcement`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 154; sourceText: Capability matrix row | Capability=Block protocol or port | What can be possible=Often possible | Required layer=Firewall, WFP, packet filter, router | Important limit=Can break legitimate services and may not classify encrypted app traffic.
- acceptedOptions: Capability Block Protocol Or Port | What Can Be Possible Often Possible | Required Layer Firewall Wfp Packet Filter Router | Important Limit Can Break Legitimate Services And May Not Classify Encrypted App Traffic
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

173.  Represent Detect VPN/proxy/tunnel capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-048`
- policyLane: `enforcement`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 157; sourceText: Capability matrix row | Capability=Detect VPN/proxy/tunnel | What can be possible=Possible as indicator | Required layer=Adapter/interface/route/process/flow heuristics | Important limit=Indicator does not prove the tunneled destination or content.
- acceptedOptions: Capability Detect Vpn Proxy Tunnel | What Can Be Possible Possible As Indicator | Required Layer Adapter Interface Route Process Flow Heuristics | Important Limit Indicator Does Not Prove The Tunneled Destination Or Content
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

174.  Represent Force all traffic through adapter capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-049`
- policyLane: `enforcement`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 158; sourceText: Capability matrix row | Capability=Force all traffic through adapter | What can be possible=Possible on some platforms | Required layer=Always-on VPN, WFP, router, MDM/profile | Important limit=Requires privileges, setup, and bypass proof.
- acceptedOptions: Capability Force All Traffic Through Adapter | What Can Be Possible Possible On Some Platforms | Required Layer Always On Vpn Wfp Router Mdm Profile | Important Limit Requires Privileges Setup And Bypass Proof
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

175.  Represent Router-level control capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-050`
- policyLane: `enforcement`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 159; sourceText: Capability matrix row | Capability=Router-level control | What can be possible=Possible in managed router scenarios | Required layer=Router API/DNS/firewall | Important limit=Cannot usually see local process or active tab.
- acceptedOptions: Capability Router Level Control | What Can Be Possible Possible In Managed Router Scenarios | Required Layer Router Api Dns Firewall | Important Limit Cannot Usually See Local Process Or Active Tab
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

### network-guide-network-visibility-what-is-possible

#### network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol

176.  Represent firewall, VPN, WFP, packet filter, router, or proxy observations;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-059`
- policyLane: `enforcement`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 175; sourceText: firewall, VPN, WFP, packet filter, router, or proxy observations;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

177.  Represent `adapter-unavailable`;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-070`
- policyLane: `enforcement`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 189; sourceText: `adapter-unavailable`;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

178.  Represent `adapter-permission-required`?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-071`
- policyLane: `enforcement`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 190; sourceText: `adapter-permission-required`.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-network-visibility-what-is-possible-process-and-app-attribution

179.  Did a child-controlled process use VPN/proxy/tunnel-like behavior?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-076`
- policyLane: `enforcement`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 203; sourceText: Did a child-controlled process use VPN/proxy/tunnel-like behavior?
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

180.  Represent category enforcement without a parent-authored rule?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-081`
- policyLane: `enforcement`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 211; sourceText: category enforcement without a parent-authored rule.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-network-visibility-what-is-possible-lan-versus-internet

181.  Represent private address through VPN;?

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-086`
- policyLane: `enforcement`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-lan-versus-internet`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 231; sourceText: private address through VPN;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

182.  Represent router/cloud relay metadata?

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-090`
- policyLane: `enforcement`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-lan-versus-internet`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 235; sourceText: router/cloud relay metadata.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-network-visibility-what-is-possible-suspicious-indicators

183.  Represent vPN, proxy, Tor-like, tunnel, MASQUE-like, or unknown adapter indicator;?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-102`
- policyLane: `enforcement`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 267; sourceText: VPN, proxy, Tor-like, tunnel, MASQUE-like, or unknown adapter indicator;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-network-control-layers

#### network-guide-network-control-layers-dns-and-managed-resolver

184.  Represent apps can use DNS-over-HTTPS or DNS-over-TLS unless the platform routes or blocks those paths;?

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-115`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-dns-and-managed-resolver`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 296; sourceText: apps can use DNS-over-HTTPS or DNS-over-TLS unless the platform routes or blocks those paths;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

185.  Represent dNS control usually cannot identify a local process unless an endpoint adapter joins it?

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-117`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-dns-and-managed-resolver`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 299; sourceText: DNS control usually cannot identify a local process unless an endpoint adapter joins it.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-network-control-layers-firewall-wfp-and-packet-filter

186.  Represent strong local enforcement on supported platforms;?

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-118`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 310; sourceText: strong local enforcement on supported platforms;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

187.  Represent can block non-browser apps;?

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-119`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 311; sourceText: can block non-browser apps;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

188.  Represent can enforce LAN/internet route and port rules;?

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-120`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 312; sourceText: can enforce LAN/internet route and port rules;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

189.  Represent can overblock shared infrastructure;?

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-123`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 319; sourceText: can overblock shared infrastructure;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

190.  Represent does not decrypt HTTPS;?

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-124`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 320; sourceText: does not decrypt HTTPS;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

191.  Represent may be bypassed by other privileged network layers if not properly installed;?

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-125`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 321; sourceText: may be bypassed by other privileged network layers if not properly installed;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

192.  Represent must be performance-tested and rollback-capable?

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-126`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 322; sourceText: must be performance-tested and rollback-capable.
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-network-control-layers-vpn-or-tunnel-adapter

193.  Represent can cover many apps on mobile and desktop;?

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-127`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-vpn-or-tunnel-adapter`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 330; sourceText: can cover many apps on mobile and desktop;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

194.  Represent can centralize DNS/domain policy;?

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-128`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-vpn-or-tunnel-adapter`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 331; sourceText: can centralize DNS/domain policy;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

195.  Represent can implement always-on or lockdown modes on platforms that support them;?

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-129`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-vpn-or-tunnel-adapter`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 332; sourceText: can implement always-on or lockdown modes on platforms that support them;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

196.  Represent can provide flow counters and route indicators?

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-130`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-vpn-or-tunnel-adapter`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 333; sourceText: can provide flow counters and route indicators.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

197.  Represent requires visible setup and platform permission;?

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-131`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-vpn-or-tunnel-adapter`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `permission-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 337; sourceText: requires visible setup and platform permission;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

198.  Represent can conflict with school/work VPNs;?

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-132`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-vpn-or-tunnel-adapter`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 338; sourceText: can conflict with school/work VPNs;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

199.  Represent may not reveal original process or exact URL;?

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-133`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-vpn-or-tunnel-adapter`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 339; sourceText: may not reveal original process or exact URL;
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

200.  Represent always-on/lockdown claims require platform proof?

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-135`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-vpn-or-tunnel-adapter`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 341; sourceText: always-on/lockdown claims require platform proof.
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-network-control-layers-proxy

201.  Represent explicit domain allow/block;?

- settingId: `network-guide-network-control-layers-proxy-001-136`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-proxy`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 350; sourceText: explicit domain allow/block;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

202.  Represent central policy for apps that honor proxy settings;?

- settingId: `network-guide-network-control-layers-proxy-001-137`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-proxy`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 351; sourceText: central policy for apps that honor proxy settings;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

203.  Represent potential full URL control for plain HTTP or explicitly managed browser/proxy integrations?

- settingId: `network-guide-network-control-layers-proxy-001-138`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-proxy`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 352; sourceText: potential full URL control for plain HTTP or explicitly managed browser/proxy integrations.
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

204.  Represent many apps ignore system proxy settings;?

- settingId: `network-guide-network-control-layers-proxy-001-139`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-proxy`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 357; sourceText: many apps ignore system proxy settings;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

205.  Represent hTTPS path/query remains hidden without TLS interception, which is out of current Ocentra Parent scope;?

- settingId: `network-guide-network-control-layers-proxy-001-140`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-proxy`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 358; sourceText: HTTPS path/query remains hidden without TLS interception, which is out of current Ocentra Parent scope;
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

206.  Represent certificates, trust, and privacy risk are high if interception is introduced later;?

- settingId: `network-guide-network-control-layers-proxy-001-141`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-proxy`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 360; sourceText: certificates, trust, and privacy risk are high if interception is introduced later;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

207.  Represent qUIC and direct sockets can bypass unless separately controlled?

- settingId: `network-guide-network-control-layers-proxy-001-142`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-proxy`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 362; sourceText: QUIC and direct sockets can bypass unless separately controlled.
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### network-guide-network-control-layers-router

208.  Represent home-wide DNS, IP, port, or device rules;?

- settingId: `network-guide-network-control-layers-router-001-143`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-router`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 371; sourceText: home-wide DNS, IP, port, or device rules;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

209.  Represent useful for IoT and guest devices;?

- settingId: `network-guide-network-control-layers-router-001-144`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-router`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 372; sourceText: useful for IoT and guest devices;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

210.  Represent usually cannot see local process, active tab, foreground app, or child profile;?

- settingId: `network-guide-network-control-layers-router-001-146`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-router`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 377; sourceText: usually cannot see local process, active tab, foreground app, or child profile;
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

211.  Represent device identity can be ambiguous with MAC randomization, NAT, VPN, or shared devices;?

- settingId: `network-guide-network-control-layers-router-001-147`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-router`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 378; sourceText: device identity can be ambiguous with MAC randomization, NAT, VPN, or shared devices;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

212.  Represent remote/off-home traffic is not covered;?

- settingId: `network-guide-network-control-layers-router-001-148`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-router`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 380; sourceText: remote/off-home traffic is not covered;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

213.  Represent router vendor APIs vary widely;?

- settingId: `network-guide-network-control-layers-router-001-149`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-router`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 381; sourceText: router vendor APIs vary widely;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

214.  Represent router proof is separate from Windows endpoint proof?

- settingId: `network-guide-network-control-layers-router-001-150`
- policyLane: `enforcement`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-router`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 382; sourceText: router proof is separate from Windows endpoint proof.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-modern-network-limits

#### network-guide-modern-network-limits-https-doh-quic-ech-and-cdns

215.  Represent exact URL rules require managed browser, explicit URL filter, or proxy proof;?

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-159`
- policyLane: `enforcement`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 416; sourceText: exact URL rules require managed browser, explicit URL filter, or proxy proof;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

216.  Represent iP block rules should warn about shared infrastructure risk;?

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-161`
- policyLane: `enforcement`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 418; sourceText: IP block rules should warn about shared infrastructure risk;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

217.  Represent doH/DoT policy needs a specific resolver-control or network-blocking posture;?

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-162`
- policyLane: `enforcement`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 419; sourceText: DoH/DoT policy needs a specific resolver-control or network-blocking posture;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

218.  Represent qUIC blocking can force TCP fallback for some browsers/sites, but that is a compatibility decision and not a content-inspection feature;?

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-163`
- policyLane: `enforcement`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 420; sourceText: QUIC blocking can force TCP fallback for some browsers/sites, but that is a compatibility decision and not a content-inspection feature;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-modern-network-limits-private-relay-and-platform-privacy-features

219.  Represent block only where a platform adapter proves support?

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-171`
- policyLane: `enforcement`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 450; sourceText: block only where a platform adapter proves support.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-domain-blocking-versus-exact-url-blocking

#### network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking

220.  Represent allow or block `example.invalid`;?

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-172`
- policyLane: `enforcement`; sectionId: `network-guide-domain-blocking-versus-exact-url-blocking`; groupId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 456; sourceText: allow or block `example.invalid`;
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

221.  Represent allow or block subdomains;?

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-173`
- policyLane: `enforcement`; sectionId: `network-guide-domain-blocking-versus-exact-url-blocking`; groupId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 457; sourceText: allow or block subdomains;
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

222.  Represent block a category when the category source is explicit;?

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-174`
- policyLane: `enforcement`; sectionId: `network-guide-domain-blocking-versus-exact-url-blocking`; groupId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 458; sourceText: block a category when the category source is explicit;
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

223.  Represent block a known resolver, VPN gateway, proxy, or tunnel host;?

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-175`
- policyLane: `enforcement`; sectionId: `network-guide-domain-blocking-versus-exact-url-blocking`; groupId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 459; sourceText: block a known resolver, VPN gateway, proxy, or tunnel host;
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

224.  Represent block an app from making network connections to a domain/IP/port?

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-176`
- policyLane: `enforcement`; sectionId: `network-guide-domain-blocking-versus-exact-url-blocking`; groupId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 460; sourceText: block an app from making network connections to a domain/IP/port.
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

225.  Represent allow or block a specific page path;?

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-177`
- policyLane: `enforcement`; sectionId: `network-guide-domain-blocking-versus-exact-url-blocking`; groupId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 464; sourceText: allow or block a specific page path;
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

226.  Represent handle query strings;?

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-178`
- policyLane: `enforcement`; sectionId: `network-guide-domain-blocking-versus-exact-url-blocking`; groupId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 465; sourceText: handle query strings;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

227.  Represent classify a specific video page;?

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-179`
- policyLane: `enforcement`; sectionId: `network-guide-domain-blocking-versus-exact-url-blocking`; groupId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 466; sourceText: classify a specific video page;
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

228.  Represent block a browser download source URL?

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-180`
- policyLane: `enforcement`; sectionId: `network-guide-domain-blocking-versus-exact-url-blocking`; groupId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 467; sourceText: block a browser download source URL.
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

### network-guide-platform-capability-notes

#### network-guide-platform-capability-notes-windows

229.  Represent windows Firewall for IP, port, protocol, service, and application rules;?

- settingId: `network-guide-platform-capability-notes-windows-001-223`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 541; sourceText: Windows Firewall for IP, port, protocol, service, and application rules;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

230.  Represent windows Filtering Platform for future observation and enforcement adapters;?

- settingId: `network-guide-platform-capability-notes-windows-001-224`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 542; sourceText: Windows Filtering Platform for future observation and enforcement adapters;
- acceptedOptions: Represented | Not Represented
- helperText: real-platform-network-adapter-proof

231.  Represent wFP, ETW, firewall, and service paths may require admin rights and careful installer/service setup;?

- settingId: `network-guide-platform-capability-notes-windows-001-227`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 548; sourceText: WFP, ETW, firewall, and service paths may require admin rights and careful installer/service setup;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

232.  Represent broad domain/network blocking is still manual-required until a real adapter proof exists;?

- settingId: `network-guide-platform-capability-notes-windows-001-228`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 550; sourceText: broad domain/network blocking is still manual-required until a real adapter proof exists;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-platform-capability-notes-macos

233.  Represent network Extension content filter or app proxy paths where entitled and approved;?

- settingId: `network-guide-platform-capability-notes-macos-001-232`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 563; sourceText: Network Extension content filter or app proxy paths where entitled and approved;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

234.  Represent dNS/proxy/VPN settings when allowed by setup?

- settingId: `network-guide-platform-capability-notes-macos-001-236`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 568; sourceText: DNS/proxy/VPN settings when allowed by setup.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

235.  Represent do not assume Windows WFP or process-control semantics map to macOS;?

- settingId: `network-guide-platform-capability-notes-macos-001-238`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 574; sourceText: do not assume Windows WFP or process-control semantics map to macOS;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-platform-capability-notes-linux

236.  Represent nftables/netfilter, iptables compatibility, or distro firewall managers;?

- settingId: `network-guide-platform-capability-notes-linux-001-240`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 584; sourceText: nftables/netfilter, iptables compatibility, or distro firewall managers;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

237.  Represent dNS/proxy/VPN controls;?

- settingId: `network-guide-platform-capability-notes-linux-001-242`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 586; sourceText: DNS/proxy/VPN controls;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

238.  Represent router/server Linux is not the same as a child desktop agent;?

- settingId: `network-guide-platform-capability-notes-linux-001-246`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 593; sourceText: router/server Linux is not the same as a child desktop agent;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-platform-capability-notes-android

239.  Represent vpnService for an app-owned VPN path;?

- settingId: `network-guide-platform-capability-notes-android-001-248`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 602; sourceText: VpnService for an app-owned VPN path;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

240.  Represent always-on VPN and lockdown when user, device owner, or profile owner setup permits;?

- settingId: `network-guide-platform-capability-notes-android-001-249`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 603; sourceText: always-on VPN and lockdown when user, device owner, or profile owner setup permits;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

241.  Represent a normal app cannot broadly firewall every other app without a VPN-style or device-management boundary;?

- settingId: `network-guide-platform-capability-notes-android-001-253`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 613; sourceText: a normal app cannot broadly firewall every other app without a VPN-style or device-management boundary;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

242.  Represent per-app VPN and always-on behavior need device proof and UX setup;?

- settingId: `network-guide-platform-capability-notes-android-001-255`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 617; sourceText: per-app VPN and always-on behavior need device proof and UX setup;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

243.  Represent school/work VPN conflicts need explicit handling?

- settingId: `network-guide-platform-capability-notes-android-001-256`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 618; sourceText: school/work VPN conflicts need explicit handling.
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-platform-capability-notes-router-and-home-network

244.  Represent dHCP/device identity;?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-266`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 649; sourceText: DHCP/device identity;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

245.  Represent dNS resolver policy;?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-267`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 650; sourceText: DNS resolver policy;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

246.  Represent firewall/IP/port rules;?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-268`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 651; sourceText: firewall/IP/port rules;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

247.  Represent lAN device grouping;?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-270`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 653; sourceText: LAN device grouping;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

248.  Represent parent-owned router API integration?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-271`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 654; sourceText: parent-owned router API integration.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

249.  Represent weak process/child attribution;?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-272`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 658; sourceText: weak process/child attribution;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

250.  Represent vendor-specific APIs;?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-273`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 659; sourceText: vendor-specific APIs;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

251.  Represent mAC randomization and device sharing;?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-274`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 660; sourceText: MAC randomization and device sharing;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

252.  Represent no off-home coverage;?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-275`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 661; sourceText: no off-home coverage;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

253.  Represent no exact URL path/query without explicit proxy/filter integration?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-276`
- policyLane: `enforcement`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 662; sourceText: no exact URL path/query without explicit proxy/filter integration.
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

### network-guide-policy-modes-to-represent-later-in-ui

#### network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity

254.  Represent network blocking;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-280`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 686; sourceText: network blocking;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-policy-modes-to-represent-later-in-ui-domain-rules

255.  Represent allow, warn, ask, limit, or block domains and subdomains where a domain-control layer is available;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-284`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 695; sourceText: allow, warn, ask, limit, or block domains and subdomains where a domain-control layer is available;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

256.  Represent cDN-safe IP blocking;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-288`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 703; sourceText: CDN-safe IP blocking;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

257.  Represent process attribution from router or DNS-only data?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-289`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 704; sourceText: process attribution from router or DNS-only data.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules

258.  Represent allow or block remote IPs, CIDRs, local/remote ports, and transport protocols through firewall/WFP/VPN/router/packet-filter adapters?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules-001-290`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 710; sourceText: allow or block remote IPs, CIDRs, local/remote ports, and transport protocols through firewall/WFP/VPN/router/packet-filter adapters.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling

259.  Represent optionally require managed network path where supported?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling-001-295`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 724; sourceText: optionally require managed network path where supported.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

260.  Represent ocentra knows tunneled destinations or content?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling-001-296`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 728; sourceText: Ocentra knows tunneled destinations or content.
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement

261.  Represent child-device agent applies platform network control after a typed policy decision references stored evidence or a parent-authored target?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-306`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 761; sourceText: child-device agent applies platform network control after a typed policy decision references stored evidence or a parent-authored target.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

262.  Represent adapter capability proof;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-307`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 766; sourceText: adapter capability proof;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

263.  Represent policy decision;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-308`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 767; sourceText: policy decision;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

264.  Represent visible unsupported/degraded state?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-311`
- policyLane: `enforcement`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 770; sourceText: visible unsupported/degraded state.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-current-ocentra-parent-posture

#### network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

265.  Represent windows is first, but broad network/domain blocking remains manual-required until real OS adapter proof exists?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-316`
- policyLane: `enforcement`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 782; sourceText: Windows is first, but broad network/domain blocking remains manual-required until real OS adapter proof exists.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

266.  Represent portal UI authors rules and shows capability states. It does not run capture, policy evaluation, enforcement, timers, OS commands, or scripts?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-318`
- policyLane: `enforcement`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 786; sourceText: Portal UI authors rules and shows capability states. It does not run capture, policy evaluation, enforcement, timers, OS commands, or scripts.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

267.  Represent [`docs/expectations/enforcement.md`](../../../expectations/enforcement.md)?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-322`
- policyLane: `enforcement`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 794; sourceText: [`docs/expectations/enforcement.md`](../../../expectations/enforcement.md)
- acceptedOptions: Represented | Not Represented
- helperText: real-platform-network-adapter-proof

### network-guide-future-ui-rules

#### network-guide-future-ui-rules-future-ui-rules

268.  Represent show IP-only, domain-ambiguous, DNS-unavailable, encrypted-content-unavailable, process-unknown, adapter-unavailable, and stale states directly?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-328`
- policyLane: `enforcement`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 806; sourceText: Show IP-only, domain-ambiguous, DNS-unavailable, encrypted-content-unavailable, process-unknown, adapter-unavailable, and stale states directly.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

269.  Configure show capability status beside each strict action.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-331`
- policyLane: `enforcement`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 811; sourceText: Show capability status beside each strict action: ready, unsupported, permission-required, adapter-missing, proof-missing, degraded, monitor-only, manual-required, or unavailable.
- acceptedOptions: Ready | Unsupported | Permission Required | Adapter Missing | Proof Missing | Degraded | Monitor Only | Manual Required | Unavailable
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

270.  Represent vPN/proxy/tunnel handling;?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-337`
- policyLane: `enforcement`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 824; sourceText: VPN/proxy/tunnel handling;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

271.  Represent strict network enforcement where proven;?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-340`
- policyLane: `enforcement`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 827; sourceText: strict network enforcement where proven;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

272.  Represent router or cloud relay options only where separately configured and proven?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-342`
- policyLane: `enforcement`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 829; sourceText: router or cloud relay options only where separately configured and proven.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

## Tab: schedule

### network-guide-core-terms

#### network-guide-core-terms-network-flow-evidence

273.  Represent timestamps, connection counts, duration, and bytes sent/received where the adapter can prove them;?

- settingId: `network-guide-core-terms-network-flow-evidence-001-008`
- policyLane: `schedule`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-flow-evidence`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 34; sourceText: timestamps, connection counts, duration, and bytes sent/received where the adapter can prove them;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### budgets

#### budgets-controls

274.  Enable network budgets?

- settingId: `budgets.enabled`
- policyLane: `schedule`; sectionId: `budgets`; groupId: `budgets-controls`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 530; sourceText: Enable network budgets?
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

275.  What should network budgets count?

- settingId: `budgets.countingMode`
- policyLane: `schedule`; sectionId: `budgets`; groupId: `budgets-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-schema-proposal.md
- sourceLine: 537; sourceText: What should network budgets count?
- acceptedOptions: Flow Bytes When Available | Connection Count | Network Active Time | Foreground Correlated Time | New Destination Count
- helperText: network-control-capability-registry

### network-guide-the-main-capability-truth

#### network-guide-the-main-capability-truth-the-main-capability-truth

276.  Represent bandwidth and connection-count summaries;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-024`
- policyLane: `schedule`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 120; sourceText: bandwidth and connection-count summaries;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### network-guide-capability-matrix

#### network-guide-capability-matrix-capability-matrix

277.  Represent Bandwidth budget capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-046`
- policyLane: `schedule`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 155; sourceText: Capability matrix row | Capability=Bandwidth budget | What can be possible=Possible where byte counters are reliable | Required layer=Flow counters, firewall/VPN/router counters | Important limit=DNS-only and endpoint snapshots may not provide byte counts.
- acceptedOptions: Capability Bandwidth Budget | What Can Be Possible Possible Where Byte Counters Are Reliable | Required Layer Flow Counters Firewall Vpn Router Counters | Important Limit Dns Only And Endpoint Snapshots May Not Provide Byte Counts
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

278.  Represent Time budget capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-047`
- policyLane: `schedule`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 156; sourceText: Capability matrix row | Capability=Time budget | What can be possible=Possible as flow-active, process-active, or foreground-app time | Required layer=Flow/process/browser/session timers | Important limit=Network-active time is not the same as active child attention.
- acceptedOptions: Capability Time Budget | What Can Be Possible Possible As Flow Active Process Active Or Foreground App Time | Required Layer Flow Process Browser Session Timers | Important Limit Network Active Time Is Not The Same As Active Child Attention
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-network-visibility-what-is-possible

#### network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets

279.  Configure flow byte budget.

- settingId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets-001-091`
- policyLane: `schedule`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 244; sourceText: flow byte budget: bytes sent/received by matching flows;
- acceptedOptions: Bytes Sent Received By Matching Flows
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

280.  Configure connection-count budget.

- settingId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets-001-092`
- policyLane: `schedule`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 245; sourceText: connection-count budget: number of connections or new destinations;
- acceptedOptions: Number Of Connections | New Destinations
- helperText: network-control-capability-registry

281.  Configure network-active time.

- settingId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets-001-093`
- policyLane: `schedule`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 246; sourceText: network-active time: time a matching flow was active;
- acceptedOptions: Time A Matching Flow Was Active
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

282.  Configure foreground-correlated time.

- settingId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets-001-094`
- policyLane: `schedule`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 247; sourceText: foreground-correlated time: network-active time while a process/browser/app was foreground;
- acceptedOptions: Network Active Time While A Process Browser App Was Foreground
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

283.  Configure schedule budget.

- settingId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets-001-095`
- policyLane: `schedule`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 249; sourceText: schedule budget: whether a rule is active during a local time window.
- acceptedOptions: Whether A Rule Is Active During A Local Time Window
- helperText: network-control-capability-registry

#### network-guide-network-visibility-what-is-possible-suspicious-indicators

284.  Represent traffic during blocked schedule or after budget exhaustion?

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-107`
- policyLane: `schedule`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-suspicious-indicators`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 272; sourceText: traffic during blocked schedule or after budget exhaustion.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-network-control-layers

#### network-guide-network-control-layers-router

285.  Represent can enforce local network access and internet schedules?

- settingId: `network-guide-network-control-layers-router-001-145`
- policyLane: `schedule`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-router`
- cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 373; sourceText: can enforce local network access and internet schedules.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-platform-capability-notes

#### network-guide-platform-capability-notes-windows

286.  Represent endpoint snapshots can miss short-lived flows and may not provide bytes;?

- settingId: `network-guide-platform-capability-notes-windows-001-229`
- policyLane: `schedule`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 552; sourceText: endpoint snapshots can miss short-lived flows and may not provide bytes;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-platform-capability-notes-router-and-home-network

287.  Represent bandwidth/time schedules;?

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-269`
- policyLane: `schedule`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-router-and-home-network`
- cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 652; sourceText: bandwidth/time schedules;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-policy-modes-to-represent-later-in-ui

#### network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets

288.  Represent apply budgets to flow bytes, connection counts, network-active duration, or foreground-correlated duration according to the selected evidence source?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets-001-297`
- policyLane: `schedule`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 734; sourceText: apply budgets to flow bytes, connection counts, network-active duration, or foreground-correlated duration according to the selected evidence source.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

289.  Represent counted evidence type;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets-001-298`
- policyLane: `schedule`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 739; sourceText: counted evidence type;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

290.  Represent reset window;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets-001-299`
- policyLane: `schedule`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 740; sourceText: reset window;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

291.  Represent degraded behavior when counters are missing;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets-001-300`
- policyLane: `schedule`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `future-gap`; runtimeOwner: `parent-domain`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 741; sourceText: degraded behavior when counters are missing;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

292.  Represent whether background traffic counts?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets-001-301`
- policyLane: `schedule`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 742; sourceText: whether background traffic counts.
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### network-guide-future-ui-rules

#### network-guide-future-ui-rules-future-ui-rules

293.  Represent keep process, domain, IP, port, protocol, VPN/proxy/tunnel, LAN exception, and bandwidth/time budget rules as separate target types?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-329`
- policyLane: `schedule`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 808; sourceText: Keep process, domain, IP, port, protocol, VPN/proxy/tunnel, LAN exception, and bandwidth/time budget rules as separate target types.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

294.  Represent bandwidth and network-active time budgets;?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-338`
- policyLane: `schedule`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 825; sourceText: bandwidth and network-active time budgets;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

## Tab: approvals

### network-guide-core-terms

#### network-guide-core-terms-network-control

295.  Represent warn, ask parent, or report instead of blocking;?

- settingId: `network-guide-core-terms-network-control-001-013`
- policyLane: `approvals`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-control`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 73; sourceText: warn, ask parent, or report instead of blocking;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-network-control-layers

#### network-guide-network-control-layers-firewall-wfp-and-packet-filter

296.  Represent may need admin rights, service installation, signed drivers, system extensions, MDM, or entitlement approval;?

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-122`
- policyLane: `approvals`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 317; sourceText: may need admin rights, service installation, signed drivers, system extensions, MDM, or entitlement approval;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-network-control-layers-cloud-relay

297.  Represent parent approval requests;?

- settingId: `network-guide-network-control-layers-cloud-relay-001-152`
- policyLane: `approvals`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-cloud-relay`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 392; sourceText: parent approval requests;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### network-guide-modern-network-limits

#### network-guide-modern-network-limits-private-relay-and-platform-privacy-features

298.  Represent ask parent;?

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-169`
- policyLane: `approvals`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 448; sourceText: ask parent;
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### network-guide-policy-modes-to-represent-later-in-ui

#### network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling

299.  Represent observe, warn, ask, block, or require approval for VPN/proxy/tunnel indicators;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling-001-294`
- policyLane: `approvals`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 723; sourceText: observe, warn, ask, block, or require approval for VPN/proxy/tunnel indicators;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

## Tab: reports

### network-guide-reports-custody-retention-and-audit

#### network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit

300.  Represent recent flows;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-181`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 481; sourceText: recent flows;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

301.  Represent top processes;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-182`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 482; sourceText: top processes;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

302.  Represent top domains;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-183`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 483; sourceText: top domains;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

303.  Represent top IPs;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-184`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 484; sourceText: top IPs;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

304.  Represent top ports/protocols;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-185`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 485; sourceText: top ports/protocols;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

305.  Represent new destinations;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-186`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 486; sourceText: new destinations;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

306.  Represent bandwidth summaries;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-187`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 487; sourceText: bandwidth summaries;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

307.  Represent time-window summaries;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-188`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 488; sourceText: time-window summaries;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

308.  Represent vPN/proxy/tunnel indicators;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-189`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 489; sourceText: VPN/proxy/tunnel indicators;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

309.  Represent lAN exceptions used;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-190`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 490; sourceText: LAN exceptions used;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

310.  Represent blocked/allowed/warned/asked decisions;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-191`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 491; sourceText: blocked/allowed/warned/asked decisions;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

311.  Represent enforcement failures and unavailable states;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-192`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 492; sourceText: enforcement failures and unavailable states;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

312.  Represent source/custody labels?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-193`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 493; sourceText: source/custody labels.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

313.  Represent live local child agent;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-194`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 497; sourceText: live local child agent;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

314.  Represent live LAN child agent;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-195`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 498; sourceText: live LAN child agent;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

315.  Represent child-device encrypted journal;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-196`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 499; sourceText: child-device encrypted journal;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

316.  Represent child-device SQLite query store;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-197`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 500; sourceText: child-device SQLite query store;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

317.  Represent parent-device cache;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-198`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 501; sourceText: parent-device cache;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

318.  Represent parent-owned export;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-199`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 502; sourceText: parent-owned export;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

319.  Represent parent-authorized relay;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-200`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 503; sourceText: parent-authorized relay;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

320.  Represent ocentra-hosted non-activity metadata;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-201`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 504; sourceText: Ocentra-hosted non-activity metadata;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

321.  Represent unavailable?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-202`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 505; sourceText: unavailable.
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

322.  Represent raw flow evidence retention;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-203`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 509; sourceText: raw flow evidence retention;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

323.  Represent domain summary retention;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-204`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 510; sourceText: domain summary retention;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

324.  Represent bandwidth summary retention;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-205`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 511; sourceText: bandwidth summary retention;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

325.  Represent policy/audit retention;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-206`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 512; sourceText: policy/audit retention;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

326.  Represent exported report retention;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-207`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 513; sourceText: exported report retention;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

327.  Represent deletion and expiry behavior;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-208`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 514; sourceText: deletion and expiry behavior;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

328.  Represent whether redacted summaries survive raw evidence deletion?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-209`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 515; sourceText: whether redacted summaries survive raw evidence deletion.
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

329.  Represent policy version;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-210`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 519; sourceText: policy version;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

330.  Represent rule id;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-211`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 520; sourceText: rule id;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

331.  Represent evidence id;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-212`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 521; sourceText: evidence id;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

332.  Represent adapter id and capability state;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-213`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 522; sourceText: adapter id and capability state;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

333.  Represent action requested;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-214`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 523; sourceText: action requested;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

334.  Represent action result;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-215`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 524; sourceText: action result;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

335.  Represent rollback or expiry state;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-216`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 525; sourceText: rollback or expiry state;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

336.  Represent parent approval or override reference;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-217`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 526; sourceText: parent approval or override reference;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

337.  Represent custody label;?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-218`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 527; sourceText: custody label;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

338.  Represent timestamp and source?

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-219`
- policyLane: `reports`; sectionId: `network-guide-reports-custody-retention-and-audit`; groupId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 528; sourceText: timestamp and source.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-future-ui-rules

#### network-guide-future-ui-rules-future-ui-rules

339.  Represent keep custody labels close to reports, AI summaries, exports, and parent assistant surfaces?

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-333`
- policyLane: `reports`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 816; sourceText: Keep custody labels close to reports, AI summaries, exports, and parent assistant surfaces.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

## Tab: audit

### network-guide-core-terms

#### network-guide-core-terms-network-flow-evidence

340.  Represent evidence id, source id, capability state, custody state, and retention state?

- settingId: `network-guide-core-terms-network-flow-evidence-001-009`
- policyLane: `audit`; sectionId: `network-guide-core-terms`; groupId: `network-guide-core-terms-network-flow-evidence`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 36; sourceText: evidence id, source id, capability state, custody state, and retention state.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-the-main-capability-truth

#### network-guide-the-main-capability-truth-the-main-capability-truth

341.  Represent per-process DNS attribution when the source is host-level DNS cache only;?

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-033`
- policyLane: `audit`; sectionId: `network-guide-the-main-capability-truth`; groupId: `network-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 134; sourceText: per-process DNS attribution when the source is host-level DNS cache only;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-capability-matrix

#### network-guide-capability-matrix-capability-matrix

342.  Represent Attribute domain capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-039`
- policyLane: `audit`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 148; sourceText: Capability matrix row | Capability=Attribute domain | What can be possible=Sometimes through DNS events/cache or managed resolver | Required layer=DNS adapter, resolver, browser join, proxy | Important limit=DoH, DoT, VPNs, ECH, CDNs, and cache ambiguity reduce confidence.
- acceptedOptions: Capability Attribute Domain | What Can Be Possible Sometimes Through Dns Events Cache Or Managed Resolver | Required Layer Dns Adapter Resolver Browser Join Proxy | Important Limit Doh Dot Vpns Ech Cdns And Cache Ambiguity Reduce Confidence
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

343.  Represent Block domain capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-042`
- policyLane: `audit`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 151; sourceText: Capability matrix row | Capability=Block domain | What can be possible=Possible through DNS, proxy, browser, VPN, WFP, router | Required layer=Domain resolver/control layer | Important limit=DoH/DoT, hard-coded IPs, CDNs, and cached connections can bypass or blur.
- acceptedOptions: Capability Block Domain | What Can Be Possible Possible Through Dns Proxy Browser Vpn Wfp Router | Required Layer Domain Resolver Control Layer | Important Limit Doh Dot Hard Coded Ips Cdns And Cached Connections Can Bypass Or Blur
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

344.  Represent Audit enforcement result capability status.

- settingId: `network-guide-capability-matrix-capability-matrix-001-054`
- policyLane: `audit`; sectionId: `network-guide-capability-matrix`; groupId: `network-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 163; sourceText: Capability matrix row | Capability=Audit enforcement result | What can be possible=Required for strict actions | Required layer=Journaled policy decision plus adapter result | Important limit=A rule value alone is not proof that traffic was blocked.
- acceptedOptions: Capability Audit Enforcement Result | What Can Be Possible Required For Strict Actions | Required Layer Journaled Policy Decision Plus Adapter Result | Important Limit A Rule Value Alone Is Not Proof That Traffic Was Blocked
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-network-visibility-what-is-possible

#### network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol

345.  Represent dNS client cache or observed resolver events;?

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-057`
- policyLane: `audit`; sectionId: `network-guide-network-visibility-what-is-possible`; groupId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 173; sourceText: DNS client cache or observed resolver events;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-network-control-layers

#### network-guide-network-control-layers-dns-and-managed-resolver

346.  Represent dNS answers can be cached before policy changes;?

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-112`
- policyLane: `audit`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-dns-and-managed-resolver`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 293; sourceText: DNS answers can be cached before policy changes;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### network-guide-network-control-layers-firewall-wfp-and-packet-filter

347.  Represent can produce auditable adapter results?

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-121`
- policyLane: `audit`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 313; sourceText: can produce auditable adapter results.
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-network-control-layers-vpn-or-tunnel-adapter

348.  Represent must not export child activity to Ocentra-hosted infrastructure by default;?

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-134`
- policyLane: `audit`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-vpn-or-tunnel-adapter`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 340; sourceText: must not export child activity to Ocentra-hosted infrastructure by default;
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### network-guide-network-control-layers-cloud-relay

349.  Represent storage of child network evidence by default;?

- settingId: `network-guide-network-control-layers-cloud-relay-001-157`
- policyLane: `audit`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-cloud-relay`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 400; sourceText: storage of child network evidence by default;
- acceptedOptions: Enabled | Disabled
- helperText: local-first-custody-and-retention-policy

350.  Represent exact network observations unless the child agent uploaded parent-authorized typed summaries under an explicit custody setting?

- settingId: `network-guide-network-control-layers-cloud-relay-001-158`
- policyLane: `audit`; sectionId: `network-guide-network-control-layers`; groupId: `network-guide-network-control-layers-cloud-relay`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 401; sourceText: exact network observations unless the child agent uploaded parent-authorized typed summaries under an explicit custody setting.
- acceptedOptions: Enabled | Disabled
- helperText: local-first-custody-and-retention-policy

### network-guide-platform-capability-notes

#### network-guide-platform-capability-notes-windows

351.  Represent dNS client cache or DNS event observation;?

- settingId: `network-guide-platform-capability-notes-windows-001-221`
- policyLane: `audit`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 539; sourceText: DNS client cache or DNS event observation;
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

352.  Represent dNS cache is host-level unless a stronger source proves per-process attribution;?

- settingId: `network-guide-platform-capability-notes-windows-001-230`
- policyLane: `audit`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 553; sourceText: DNS cache is host-level unless a stronger source proves per-process attribution;
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### network-guide-policy-modes-to-represent-later-in-ui

#### network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement

353.  Represent audit event;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-309`
- policyLane: `audit`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 768; sourceText: audit event;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

354.  Represent rollback or expiry path;?

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-310`
- policyLane: `audit`; sectionId: `network-guide-policy-modes-to-represent-later-in-ui`; groupId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 769; sourceText: rollback or expiry path;
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### network-guide-current-ocentra-parent-posture

#### network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

355.  Represent policy can consume stored network summaries and unusual indicators only after they are journaled and queryable?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-314`
- policyLane: `audit`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 778; sourceText: Policy can consume stored network summaries and unusual indicators only after they are journaled and queryable.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

356.  Represent enforcement is scaffold/protocol/audit work unless a real platform adapter proof exists?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-315`
- policyLane: `audit`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 780; sourceText: Enforcement is scaffold/protocol/audit work unless a real platform adapter proof exists.
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

357.  Represent [`docs/expectations/data-custody.md`](../../../expectations/data-custody.md)?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-323`
- policyLane: `audit`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 795; sourceText: [`docs/expectations/data-custody.md`](../../../expectations/data-custody.md)
- acceptedOptions: Enabled | Disabled
- helperText: local-first-custody-and-retention-policy

### network-guide-future-ui-rules

#### network-guide-future-ui-rules-future-ui-rules

358.  Configure require proof for enforcement claims.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-332`
- policyLane: `audit`; sectionId: `network-guide-future-ui-rules`; groupId: `network-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 814; sourceText: Require proof for enforcement claims: parent rule, evidence reference, policy decision, adapter action, adapter result, audit row, and rollback/expiry state.
- acceptedOptions: Parent Rule | Evidence Reference | Policy Decision | Adapter Action | Adapter Result | Audit Row | And Rollback Expiry State
- helperText: real-platform-network-adapter-proof

## Tab: setup

### network-guide-modern-network-limits

#### network-guide-modern-network-limits-private-relay-and-platform-privacy-features

359.  Represent require managed browser/network path;?

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-170`
- policyLane: `setup`; sectionId: `network-guide-modern-network-limits`; groupId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 449; sourceText: require managed browser/network path;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### network-guide-platform-capability-notes

#### network-guide-platform-capability-notes-macos

360.  Represent configuration profiles or MDM for stronger managed-device cases;?

- settingId: `network-guide-platform-capability-notes-macos-001-233`
- policyLane: `setup`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-macos`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `child-agent`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 565; sourceText: configuration profiles or MDM for stronger managed-device cases;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### network-guide-platform-capability-notes-linux

361.  Represent privilege and service installation differ by distro;?

- settingId: `network-guide-platform-capability-notes-linux-001-245`
- policyLane: `setup`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-linux`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `child-agent`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 592; sourceText: privilege and service installation differ by distro;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### network-guide-platform-capability-notes-android

362.  Represent devicePolicyManager controls for device-owner or profile-owner deployments;?

- settingId: `network-guide-platform-capability-notes-android-001-250`
- policyLane: `setup`; sectionId: `network-guide-platform-capability-notes`; groupId: `network-guide-platform-capability-notes-android`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 605; sourceText: DevicePolicyManager controls for device-owner or profile-owner deployments;
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### network-guide-current-ocentra-parent-posture

#### network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

363.  Represent [`docs/managed-unmanaged-browser.md`](../../../plans/browser-plan/workpacks/managed-unmanaged-browser.md)?

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-325`
- policyLane: `setup`; sectionId: `network-guide-current-ocentra-parent-posture`; groupId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/network-control-capability-guide.md
- sourceLine: 797; sourceText: [`docs/managed-unmanaged-browser.md`](../../../plans/browser-plan/workpacks/managed-unmanaged-browser.md)
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry
