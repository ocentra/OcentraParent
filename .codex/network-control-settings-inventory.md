# Network Control Settings Inventory

Generated from `BaselineNetworkControlCatalog`.
Total settings: 363

Use this as the raw review list for deciding parent-facing grouping.

## Tab: rules

### Network management

#### Network management

1.  Enable network management?

- settingId: `network.enabled`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 144
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

2.  What should happen to network activity?

- settingId: `network.defaultPosture`
- policyLane: `rules`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 166
- acceptedOptions: Allow | Observe | Warn | Ask | Limit | Block
- helperText: network-control-capability-registry

3.  How should network management run on this device?

- settingId: `network.managementMode`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 215
- acceptedOptions: Local Child Agent | Lan Live | Authoring Only | Unavailable
- helperText: network-control-capability-registry

### Core Terms

#### Network Control

4.  keep local-network exceptions for printers, LAN pairing, parental devices, and trusted home services.

- settingId: `network-guide-core-terms-network-control-001-014`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 74
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

#### Local Network Exception

5.  school/home subnet where the parent explicitly allows discovery;

- settingId: `network-guide-core-terms-local-network-exception-001-018`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 97
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### Local network

#### Local network

6.  How should local-network exceptions behave?

- settingId: `localNetwork.exceptionMode`
- policyLane: `rules`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 566
- acceptedOptions: Disabled | Explicit Services Only | Trusted Subnets | Allow Private Networks | Parent Request For New Local Destination
- helperText: network-control-capability-registry

7.  Which local exceptions should be available?

- settingId: `localNetwork.defaultExceptions`
- policyLane: `rules`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 580
- acceptedOptions: Loopback Agent | Lan Parent Controller | Dns Resolver | Dhcp | Printer | Nas | School Subnet | Mdns | Ssdp | Router Admin
- helperText: network-control-capability-registry

### The Main Capability Truth

#### The Main Capability Truth

8.  LAN versus internet classification;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-023`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 119
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

9.  page title or active tab;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-029`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 129
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

10. page body, chat content, search terms, form values, cookies, tokens, or credentials;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-030`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network controls must not collect decrypted content or payload fields; use metadata evidence only.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 130
- acceptedOptions: Enabled | Disabled
- helperText: Network controls must not collect decrypted content or payload fields; use metadata evidence only.

11. specific video or post within a CDN-backed service;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-031`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 132
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

12. which tab caused a network request;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-032`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 133
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

13. child intent or safety classification without a separate policy/AI contract.

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-034`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 135
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### Network Visibility: What Is Possible

#### LAN Versus Internet

14. loopback;

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-082`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 227
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

15. local agent service;

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-083`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 228
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

16. LAN parent controller;

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-084`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 229
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

17. local subnet;

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-085`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 230
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

18. public internet;

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-087`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 232
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

19. unknown route;

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-088`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 233
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### Network Control Layers

#### Cloud Relay

20. remote rule update delivery;

- settingId: `network-guide-network-control-layers-cloud-relay-001-151`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 391
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

21. control of arbitrary child internet traffic;

- settingId: `network-guide-network-control-layers-cloud-relay-001-156`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 399
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### Modern Network Limits

#### Private Relay And Platform Privacy Features

22. allow;

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-166`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 445
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

23. observe;

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-167`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 446
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

24. warn;

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-168`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 447
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### Platform Capability Notes

#### Windows

25. ETW for network event streams if loss/decode/privilege states are typed;

- settingId: `network-guide-platform-capability-notes-windows-001-222`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 540
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

26. product claims should follow real host proof, not contract presence.

- settingId: `network-guide-platform-capability-notes-windows-001-231`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 555
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### macOS

27. consumer child-agent claims must stay behind Apple-approved capabilities.

- settingId: `network-guide-platform-capability-notes-macos-001-239`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 575
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### Linux

28. desktop foreground and app identity vary;

- settingId: `network-guide-platform-capability-notes-linux-001-244`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 591
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

29. claims need distro-specific validation.

- settingId: `network-guide-platform-capability-notes-linux-001-247`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 594
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### Android

30. Usage Stats, accessibility, or browser/app-specific integrations for foreground/app context when explicitly approved.

- settingId: `network-guide-platform-capability-notes-android-001-252`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 608
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### Policy Modes To Represent Later In UI

#### Observe Network Activity

31. decrypted content;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-282`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Network controls must not collect decrypted content or payload fields; use metadata evidence only.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 688
- acceptedOptions: Enabled | Disabled
- helperText: Network controls must not collect decrypted content or payload fields; use metadata evidence only.

#### Local Network Exceptions

32. exact exception scope;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions-001-303`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 753
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

33. risk of broad private-network allow rules.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions-001-305`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 755
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

34. [`docs/expectations/policy.md`](expectations/policy.md)

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-321`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 793
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

35. [`docs/product-roadmap.md`](product-roadmap.md)

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-324`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 796
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### Future UI Rules

#### Future UI Rules

36. Keep LAN exceptions visible beside strict rules.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-330`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 810
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

37. observe only;

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-334`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 821
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

38. local network exceptions;

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-339`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 826
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

## Tab: evidence

### Core Terms

#### Network Flow Evidence

39. local IP and port;

- settingId: `network-guide-core-terms-network-flow-evidence-001-001`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 27
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

40. remote IP and port;

- settingId: `network-guide-core-terms-network-flow-evidence-001-002`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 28
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

41. transport protocol;

- settingId: `network-guide-core-terms-network-flow-evidence-001-003`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 29
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

42. TCP state;

- settingId: `network-guide-core-terms-network-flow-evidence-001-004`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 30
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

43. process id and process identity where available;

- settingId: `network-guide-core-terms-network-flow-evidence-001-005`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 31
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

44. DNS/domain attribution where available;

- settingId: `network-guide-core-terms-network-flow-evidence-001-006`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 32
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Local Network Exception

45. loopback service ports used by the child-device agent;

- settingId: `network-guide-core-terms-local-network-exception-001-015`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 94
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

46. LAN pairing ports between parent controller and child agent;

- settingId: `network-guide-core-terms-local-network-exception-001-016`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 95
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

47. multicast and broadcast protocols that are needed for device discovery.

- settingId: `network-guide-core-terms-local-network-exception-001-019`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 98
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Evidence scope

#### Evidence scope

48. What network evidence may rules use?

- settingId: `evidence.metadataScope`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 238
- acceptedOptions: Process | Ip | Port | Protocol | Domain Candidate | Dns Source State | Connection Count | Duration | Bytes When Available | Interface | Route | Lan Wan Classification | Vpn Proxy Tunnel Indicator | Adapter State
- helperText: network-control-capability-registry

49. What proof is enough for network decisions?

- settingId: `evidence.requiredProof`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 272
- acceptedOptions: Capability Only | Endpoint Observed | Process Attributed Flow | Domain Candidate | Domain Known | Stored Flow Summary | Managed Browser Joined Domain | Adapter Enforcement Proof
- helperText: network-control-capability-registry

50. What if network proof is unavailable?

- settingId: `evidence.whenProofUnavailable`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 289
- acceptedOptions: Allow | Observe | Warn | Ask | Block Until Ready | Mark Unavailable
- helperText: network-control-capability-registry

51. What must network rules never collect?

- settingId: `evidence.neverCollect`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 297
- acceptedOptions: Decrypted Https Payload | Packet Payload | Page Body | Chat Content | Search Terms | Form Values | Cookies | Tokens | Credentials | Raw Packet Dumps | Raw Trace Files
- helperText: network-control-capability-registry

### Domains and DNS

#### Domains and DNS

52. How should domain controls work?

- settingId: `dns.mode`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 340
- acceptedOptions: Disabled | Observe And Classify | Managed Resolver Preferred | Managed Resolver Required | Block Unapproved Encrypted Dns
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

53. Which domain attribution sources may be used?

- settingId: `dns.allowedAttributionSources`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 354
- acceptedOptions: Observed Dns Event | Dns Client Cache | Managed Resolver Log | Reverse Dns | Static Hosts | Managed Browser Join | Parent Rule Domain
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

54. How should encrypted DNS be handled?

- settingId: `dns.encryptedDnsHandling`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 370
- acceptedOptions: Allow | Report Only | Warn | Ask | Block Unknown Resolvers | Require Managed Resolver
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Flow rules

#### Flow rules

55. What network targets should rules match?

- settingId: `rules.allowedTargetTypes`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 397
- acceptedOptions: Domain | Ip | Cidr | Port | Protocol | Process | Interface | Route | Destination Category | Vpn Proxy Tunnel | New Destination | High Volume | Repeated Failure | Capability State
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

56. What actions can network rules take?

- settingId: `rules.allowedActions`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 429
- acceptedOptions: Allow | Observe | Warn | Ask | Limit | Block | Terminate Process | Require Managed Network
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

57. How should conflicting network rules resolve?

- settingId: `rules.conflictResolution`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 446
- acceptedOptions: Explicit Local Exception Beats Block | Process Rule Beats Domain Rule | Domain Beats Ip Category | Managed Browser Domain Beats Dns Candidate | Block Beats Allow | Fresh Proof Beats Stale Proof | Adapter Proof Required For Enforcement
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Reports

#### Reports

58. Which network report fields should parents see?

- settingId: `reports.visibleFields`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 610
- acceptedOptions: Recent Flows | Top Processes | Top Domains | Top Ips | Ports Protocols | Bandwidth | Time Budgets | New Destinations | Repeated Failures | Vpn Proxy Tunnel Indicators | Local Exceptions Used | Policy Decisions | Block Results | Source Capability | Custody Label
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

59. Show unknown and ambiguous states?

- settingId: `reports.showUncertainty`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 644
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### The Main Capability Truth

#### The Main Capability Truth

60. IP, port, protocol, and process rules;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-021`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 117
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

61. unusual new destination reporting;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-025`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 121
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

62. local AI/policy digests with evidence ids;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-026`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 122
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

63. app/browser correlation when the flow also has process or managed-browser evidence.

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-027`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 123
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

64. exact URL path or query in normal HTTPS;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-028`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 128
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

### Capability Matrix

#### Capability Matrix

65. Capability matrix row | Capability=Detect remote port | What can be possible=Yes where endpoint/packet metadata is exposed | Required layer=Endpoint/flow observation | Important limit=Port does not prove application semantics.

- settingId: `network-guide-capability-matrix-capability-matrix-001-036`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 145
- acceptedOptions: Capability Detect Remote Port | What Can Be Possible Yes Where Endpoint Packet Metadata Is Exposed | Required Layer Endpoint Flow Observation | Important Limit Port Does Not Prove Application Semantics
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

66. Capability matrix row | Capability=Detect protocol | What can be possible=TCP/UDP/IP protocol usually; app protocol sometimes | Required layer=Endpoint/flow observation, DPI if approved | Important limit=QUIC over UDP/443 can hide higher-level HTTP details.

- settingId: `network-guide-capability-matrix-capability-matrix-001-037`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 146
- acceptedOptions: Capability Detect Protocol | What Can Be Possible Tcp Udp Ip Protocol Usually App Protocol Sometimes | Required Layer Endpoint Flow Observation Dpi If Approved | Important Limit Quic Over Udp 443 Can Hide Higher Level Http Details
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

67. Capability matrix row | Capability=Cloud relay control | What can be possible=Possible only for Ocentra protocol traffic or parent-authorized path | Required layer=Relay/control-plane contract | Important limit=Cannot control arbitrary child internet traffic by itself.

- settingId: `network-guide-capability-matrix-capability-matrix-001-051`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 160
- acceptedOptions: Capability Cloud Relay Control | What Can Be Possible Possible Only For Ocentra Protocol Traffic Or Parent Authorized Path | Required Layer Relay Control Plane Contract | Important Limit Cannot Control Arbitrary Child Internet Traffic By Itself
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

68. Capability matrix row | Capability=LAN exception | What can be possible=Possible | Required layer=Route/interface/subnet/service policy | Important limit=Too broad an exception can hide unwanted local traffic.

- settingId: `network-guide-capability-matrix-capability-matrix-001-052`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 161
- acceptedOptions: Capability Lan Exception | What Can Be Possible Possible | Required Layer Route Interface Subnet Service Policy | Important Limit Too Broad An Exception Can Hide Unwanted Local Traffic
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

69. Capability matrix row | Capability=Suspicious indicator report | What can be possible=Possible | Required layer=Stored network digest plus deterministic/AI labels | Important limit=Indicator must cite evidence and keep uncertainty.

- settingId: `network-guide-capability-matrix-capability-matrix-001-053`
- policyLane: `evidence`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 162
- acceptedOptions: Capability Suspicious Indicator Report | What Can Be Possible Possible | Required Layer Stored Network Digest Plus Deterministic Ai Labels | Important Limit Indicator Must Cite Evidence And Keep Uncertainty
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Network Visibility: What Is Possible

#### DNS, Domain, IP, Port, And Protocol

70. endpoint snapshots for local and remote IP/port/protocol;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-055`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 171
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

71. TCP and UDP owner PID tables where supported;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-056`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 172
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

72. managed resolver logs where Ocentra controls the resolver path;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-058`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 174
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

73. managed browser URL evidence when there is an explicit join to flow evidence.

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-060`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 176
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

74. `domain-known`;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-061`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 180
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

75. `domain-candidate`;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-062`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 181
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

76. `domain-ambiguous`;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-063`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 182
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

77. `ip-only`;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-064`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 183
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

78. `dns-unavailable`;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-065`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 184
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

79. `dns-stale`;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-066`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 185
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

80. `encrypted-content-unavailable`;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-067`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 186
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

81. `process-attributed`;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-068`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 187
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

82. `process-unknown`;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-069`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 188
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Process And App Attribution

83. Which process opened network connections?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-072`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 199
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

84. Which destination did this process contact?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-073`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 200
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

85. Did an unmanaged browser or unknown app create traffic?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-074`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 201
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

86. Did a known app suddenly contact a new destination?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-075`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 202
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

87. exact browser URL;

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-077`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 207
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

88. active tab;

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-078`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 208
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

89. page title;

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-079`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 209
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

90. user intent;

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-080`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 210
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### LAN Versus Internet

91. captive portal or public Wi-Fi;

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-089`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 234
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Suspicious Indicators

92. new destination for child/device/process;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-096`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 261
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

93. high-volume unknown process;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-097`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 262
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

94. repeated connection failures;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-098`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 263
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

95. DNS mismatch or excessive DNS churn;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-099`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 264
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

96. DNS unavailable while traffic continues;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-100`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 265
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

97. direct IP traffic to public internet;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-101`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 266
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

98. unusual port or protocol for a child device;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-103`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 268
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

99. domain/IP reputation category when the category source is explicit;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-104`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 269
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

100.  LAN scan-like pattern;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-105`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 270
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

101.  traffic from an unmanaged browser or unsupported app;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-106`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 271
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Network Control Layers

#### DNS And Managed Resolver

102.  parent-friendly domain rules;

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-108`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 286
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

103.  broad device or profile coverage when DNS path is controlled;

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-109`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 287
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

104.  useful reporting for domains and categories;

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-110`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 288
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

105.  works even when the app is not a browser if it uses the managed resolver.

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-111`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 289
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

106.  several domains can resolve to one IP, and one domain can resolve to many IPs;

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-113`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 294
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

107.  apps can use hard-coded IPs;

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-114`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 295
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

108.  DNS control does not see HTTPS path/query;

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-116`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 298
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

#### Cloud Relay

109.  report query routing;

- settingId: `network-guide-network-control-layers-cloud-relay-001-153`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 393
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

110.  device reachability metadata;

- settingId: `network-guide-network-control-layers-cloud-relay-001-154`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 394
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

111.  stateless report compilation from parent-authorized sources.

- settingId: `network-guide-network-control-layers-cloud-relay-001-155`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 395
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Modern Network Limits

#### HTTPS, DoH, QUIC, ECH, And CDNs

112.  domain rules must tolerate ambiguous or unavailable domain evidence;

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-160`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 417
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

113.  ECH reduces SNI-based classification, so DNS/resolver or browser evidence becomes more important;

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-164`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 422
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

114.  CDN-backed sites need domain/category evidence, not IP-only overclaiming.

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-165`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 424
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Platform Capability Notes

#### Windows

115.  IP Helper endpoint snapshots for TCP/UDP owner PID state;

- settingId: `network-guide-platform-capability-notes-windows-001-220`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 538
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

116.  process/window evidence for app/browser correlation;

- settingId: `network-guide-platform-capability-notes-windows-001-225`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 543
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

117.  managed browser evidence for exact URL/tab state.

- settingId: `network-guide-platform-capability-notes-windows-001-226`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 544
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

#### macOS

118.  process observation with platform permissions;

- settingId: `network-guide-platform-capability-notes-macos-001-234`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 566
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

119.  browser-managed evidence for exact URL/tab state;

- settingId: `network-guide-platform-capability-notes-macos-001-235`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 567
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

120.  Network Extension, System Extension, TCC, MDM, signing, notarization, and App Store review affect what is shippable;

- settingId: `network-guide-platform-capability-notes-macos-001-237`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 572
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Linux

121.  process and socket inspection through procfs/netlink where permitted;

- settingId: `network-guide-platform-capability-notes-linux-001-241`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 585
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

122.  managed browser evidence for exact URL/tab state.

- settingId: `network-guide-platform-capability-notes-linux-001-243`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 587
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

#### Android

123.  managed DNS, app restrictions, or package lifecycle controls where the device management posture permits;

- settingId: `network-guide-platform-capability-notes-android-001-251`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 606
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

124.  exact URL in arbitrary mobile browsers is not generally reliable from network metadata;

- settingId: `network-guide-platform-capability-notes-android-001-254`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 615
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

#### iOS And iPadOS

125.  Screen Time frameworks: Family Controls, Managed Settings, Device Activity;

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-257`
- policyLane: `evidence`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 626
- acceptedOptions: Family Controls | Managed Settings | Device Activity
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

126.  web domain shielding through managed settings where allowed;

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-258`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 627
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

127.  Network Extension content filter or URL filter paths where entitlement and deployment permit;

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-259`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 628
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

128.  MDM/supervision for stronger managed-device content filtering;

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-260`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `child-agent`; capabilityState: `manual-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 630
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

129.  app/category/domain tokens rather than raw browser history in Screen Time flows.

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-261`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network controls must not collect decrypted content or payload fields; use metadata evidence only.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 631
- acceptedOptions: Represented | Not Represented
- helperText: Network controls must not collect decrypted content or payload fields; use metadata evidence only.

130.  third-party apps do not get general packet inspection or arbitrary exact URL telemetry from other apps;

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-262`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 636
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

131.  entitlements, review, supervision, and deployment model determine capability;

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-263`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `child-agent`; capabilityState: `manual-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 638
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

132.  web domain shielding is not the same as full browser history capture;

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-264`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 639
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

133.  parent iOS app claims and child iOS agent claims must stay separate.

- settingId: `network-guide-platform-capability-notes-ios-and-ipados-001-265`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 640
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Policy Modes To Represent Later In UI

#### Observe Network Activity

134.  record and summarize network metadata;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-277`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 679
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

135.  show process, destination, protocol, port, DNS/domain, volume, and capability states where available;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-278`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 680
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

136.  classify suspicious indicators in report-only mode.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-279`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 682
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

137.  exact URL rules;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-281`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 687
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

138.  guaranteed process attribution on every platform.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-283`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 689
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Domain Rules

139.  use managed browser evidence for exact domain/origin when available;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-285`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 697
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

140.  use DNS/network attribution with confidence when browser evidence is absent.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-286`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 698
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

141.  full URL path/query control;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-287`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 702
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

#### IP, Port, And Protocol Rules

142.  can break legitimate infrastructure;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules-001-291`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 715
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

143.  can miss app semantics;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules-001-292`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `parent-domain`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 716
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

144.  can be too broad for CDN-backed services.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules-001-293`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 717
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Local Network Exceptions

145.  allow specific local services, subnets, protocols, or Ocentra pairing traffic while internet rules remain strict.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions-001-302`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 748
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

146.  last used evidence;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-local-network-exceptions-001-304`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 754
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

147.  Network flow evidence is local-first metadata, not decrypted content.

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-312`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Network controls must not collect decrypted content or payload fields; use metadata evidence only.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 776
- acceptedOptions: Enabled | Disabled
- helperText: Network controls must not collect decrypted content or payload fields; use metadata evidence only.

148.  Browser URL/tab evidence remains the exact URL source.

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-313`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 777
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

149.  LAN and cloud relay are typed control/report paths, not default hosted child evidence stores.

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-317`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 784
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

150.  [`docs/architecture/network-flow-evidence-capture.md`](architecture/network-flow-evidence-capture.md)

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-319`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 791
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

151.  [`docs/expectations/network-flow-evidence.md`](expectations/network-flow-evidence.md)

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-320`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 792
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Future UI Rules

#### Future UI Rules

152.  Show exact URL controls only when managed browser or explicit URL-filter capability is available.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-326`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 803
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

153.  Show domain rules as domain evidence, not exact URL evidence.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-327`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 805
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

154.  domain rules;

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-335`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 822
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

155.  IP/port/protocol rules;

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-336`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 823
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

156.  managed browser for exact URL rules;

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-341`
- policyLane: `evidence`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 828
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

## Tab: enforcement

### Core Terms

#### Network Flow Evidence

157.  interface, route, LAN/WAN, VPN, proxy, or tunnel indicators;

- settingId: `network-guide-core-terms-network-flow-evidence-001-007`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 33
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Network Control

158.  allow, block, or rate-limit traffic by process, IP, port, protocol, domain, or category where supported;

- settingId: `network-guide-core-terms-network-control-001-010`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 68
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

159.  force or configure DNS, proxy, VPN, firewall, WFP, packet filter, or router policy;

- settingId: `network-guide-core-terms-network-control-001-011`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 70
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

160.  terminate a process after a network policy decision;

- settingId: `network-guide-core-terms-network-control-001-012`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 72
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Local Network Exception

161.  local printer, NAS, DNS resolver, router, or media device;

- settingId: `network-guide-core-terms-local-network-exception-001-017`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 96
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### VPN, proxy, and tunnel

#### VPN, proxy, and tunnel

162.  What should happen to VPN/proxy/tunnel indicators?

- settingId: `tunnel.mode`
- policyLane: `enforcement`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 480
- acceptedOptions: Allow | Observe | Warn | Ask | Block | Require Managed Network
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

163.  Which tunnel indicators should count?

- settingId: `tunnel.indicators`
- policyLane: `enforcement`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 488
- acceptedOptions: Vpn Interface | Proxy Process | Proxy Config | Known Tunnel Port | Tor Like Process | Unknown Encrypted Relay | Masque Like Flow | Dns Unavailable With Public Traffic
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### The Main Capability Truth

#### The Main Capability Truth

164.  domain allow/block rules;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-020`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 116
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

165.  VPN/proxy/tunnel indicators;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-022`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 118
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Capability Matrix

#### Capability Matrix

166.  Capability matrix row | Capability=Detect remote IP | What can be possible=Yes on most endpoint, firewall, VPN, router, or flow adapters | Required layer=Endpoint/flow observation | Important limit=IP alone may be CDN/shared, NATed, private, or anycast.

- settingId: `network-guide-capability-matrix-capability-matrix-001-035`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 144
- acceptedOptions: Capability Detect Remote Ip | What Can Be Possible Yes On Most Endpoint Firewall Vpn Router Or Flow Adapters | Required Layer Endpoint Flow Observation | Important Limit Ip Alone May Be Cdn Shared Nated Private Or Anycast
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

167.  Capability matrix row | Capability=Attribute process | What can be possible=Often on Windows endpoint adapters; varies elsewhere | Required layer=OS endpoint/process adapter | Important limit=Router/DNS-only data usually cannot identify the local process.

- settingId: `network-guide-capability-matrix-capability-matrix-001-038`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 147
- acceptedOptions: Capability Attribute Process | What Can Be Possible Often On Windows Endpoint Adapters Varies Elsewhere | Required Layer Os Endpoint Process Adapter | Important Limit Router Dns Only Data Usually Cannot Identify The Local Process
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

168.  Capability matrix row | Capability=Attribute exact URL | What can be possible=No from network alone | Required layer=Managed browser, explicit URL filter, proxy | Important limit=Normal HTTPS hides path/query from passive network observers.

- settingId: `network-guide-capability-matrix-capability-matrix-001-040`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 149
- acceptedOptions: Capability Attribute Exact Url | What Can Be Possible No From Network Alone | Required Layer Managed Browser Explicit Url Filter Proxy | Important Limit Normal Https Hides Path Query From Passive Network Observers
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

169.  Capability matrix row | Capability=Block IP | What can be possible=Often possible | Required layer=Firewall, WFP, VPN, packet filter, router | Important limit=Shared IP/CDN can overblock unrelated services.

- settingId: `network-guide-capability-matrix-capability-matrix-001-041`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 150
- acceptedOptions: Capability Block Ip | What Can Be Possible Often Possible | Required Layer Firewall Wfp Vpn Packet Filter Router | Important Limit Shared Ip Cdn Can Overblock Unrelated Services
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

170.  Capability matrix row | Capability=Block exact URL | What can be possible=Possible only with browser/proxy/URL filter proof | Required layer=Managed browser, explicit URL filter, HTTP proxy | Important limit=Not a generic firewall claim. HTTPS interception is out of current scope.

- settingId: `network-guide-capability-matrix-capability-matrix-001-043`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 152
- acceptedOptions: Capability Block Exact Url | What Can Be Possible Possible Only With Browser Proxy Url Filter Proof | Required Layer Managed Browser Explicit Url Filter Http Proxy | Important Limit Not A Generic Firewall Claim Https Interception Is Out Of Current Scope
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

171.  Capability matrix row | Capability=Block process network | What can be possible=Possible on some OS adapters | Required layer=WFP/firewall/app control/VPN with app binding | Important limit=Requires platform proof and robust process identity.

- settingId: `network-guide-capability-matrix-capability-matrix-001-044`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 153
- acceptedOptions: Capability Block Process Network | What Can Be Possible Possible On Some Os Adapters | Required Layer Wfp Firewall App Control Vpn With App Binding | Important Limit Requires Platform Proof And Robust Process Identity
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

172.  Capability matrix row | Capability=Block protocol or port | What can be possible=Often possible | Required layer=Firewall, WFP, packet filter, router | Important limit=Can break legitimate services and may not classify encrypted app traffic.

- settingId: `network-guide-capability-matrix-capability-matrix-001-045`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 154
- acceptedOptions: Capability Block Protocol Or Port | What Can Be Possible Often Possible | Required Layer Firewall Wfp Packet Filter Router | Important Limit Can Break Legitimate Services And May Not Classify Encrypted App Traffic
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

173.  Capability matrix row | Capability=Detect VPN/proxy/tunnel | What can be possible=Possible as indicator | Required layer=Adapter/interface/route/process/flow heuristics | Important limit=Indicator does not prove the tunneled destination or content.

- settingId: `network-guide-capability-matrix-capability-matrix-001-048`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 157
- acceptedOptions: Capability Detect Vpn Proxy Tunnel | What Can Be Possible Possible As Indicator | Required Layer Adapter Interface Route Process Flow Heuristics | Important Limit Indicator Does Not Prove The Tunneled Destination Or Content
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

174.  Capability matrix row | Capability=Force all traffic through adapter | What can be possible=Possible on some platforms | Required layer=Always-on VPN, WFP, router, MDM/profile | Important limit=Requires privileges, setup, and bypass proof.

- settingId: `network-guide-capability-matrix-capability-matrix-001-049`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 158
- acceptedOptions: Capability Force All Traffic Through Adapter | What Can Be Possible Possible On Some Platforms | Required Layer Always On Vpn Wfp Router Mdm Profile | Important Limit Requires Privileges Setup And Bypass Proof
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

175.  Capability matrix row | Capability=Router-level control | What can be possible=Possible in managed router scenarios | Required layer=Router API/DNS/firewall | Important limit=Cannot usually see local process or active tab.

- settingId: `network-guide-capability-matrix-capability-matrix-001-050`
- policyLane: `enforcement`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 159
- acceptedOptions: Capability Router Level Control | What Can Be Possible Possible In Managed Router Scenarios | Required Layer Router Api Dns Firewall | Important Limit Cannot Usually See Local Process Or Active Tab
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

### Network Visibility: What Is Possible

#### DNS, Domain, IP, Port, And Protocol

176.  firewall, VPN, WFP, packet filter, router, or proxy observations;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-059`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 175
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

177.  `adapter-unavailable`;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-070`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `degraded`; runtimeOwner: `local-ai-runtime`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 189
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

178.  `adapter-permission-required`.

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-071`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `permission-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `permission-required`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 190
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Process And App Attribution

179.  Did a child-controlled process use VPN/proxy/tunnel-like behavior?

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-076`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 203
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

180.  category enforcement without a parent-authored rule.

- settingId: `network-guide-network-visibility-what-is-possible-process-and-app-attribution-001-081`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 211
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### LAN Versus Internet

181.  private address through VPN;

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-086`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 231
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

182.  router/cloud relay metadata.

- settingId: `network-guide-network-visibility-what-is-possible-lan-versus-internet-001-090`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 235
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Suspicious Indicators

183.  VPN, proxy, Tor-like, tunnel, MASQUE-like, or unknown adapter indicator;

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-102`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 267
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Network Control Layers

#### DNS And Managed Resolver

184.  apps can use DNS-over-HTTPS or DNS-over-TLS unless the platform routes or blocks those paths;

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-115`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 296
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

185.  DNS control usually cannot identify a local process unless an endpoint adapter joins it.

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-117`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 299
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Firewall, WFP, And Packet Filter

186.  strong local enforcement on supported platforms;

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-118`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 310
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

187.  can block non-browser apps;

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-119`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 311
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

188.  can enforce LAN/internet route and port rules;

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-120`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 312
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

189.  can overblock shared infrastructure;

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-123`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 319
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

190.  does not decrypt HTTPS;

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-124`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 320
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

191.  may be bypassed by other privileged network layers if not properly installed;

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-125`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 321
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

192.  must be performance-tested and rollback-capable.

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-126`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 322
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### VPN Or Tunnel Adapter

193.  can cover many apps on mobile and desktop;

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-127`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 330
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

194.  can centralize DNS/domain policy;

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-128`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 331
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

195.  can implement always-on or lockdown modes on platforms that support them;

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-129`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 332
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

196.  can provide flow counters and route indicators.

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-130`
- policyLane: `enforcement`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 333
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

197.  requires visible setup and platform permission;

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-131`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `permission-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 337
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

198.  can conflict with school/work VPNs;

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-132`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 338
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

199.  may not reveal original process or exact URL;

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-133`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 339
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

200.  always-on/lockdown claims require platform proof.

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-135`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 341
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Proxy

201.  explicit domain allow/block;

- settingId: `network-guide-network-control-layers-proxy-001-136`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 350
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

202.  central policy for apps that honor proxy settings;

- settingId: `network-guide-network-control-layers-proxy-001-137`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 351
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

203.  potential full URL control for plain HTTP or explicitly managed browser/proxy integrations.

- settingId: `network-guide-network-control-layers-proxy-001-138`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 352
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

204.  many apps ignore system proxy settings;

- settingId: `network-guide-network-control-layers-proxy-001-139`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 357
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

205.  HTTPS path/query remains hidden without TLS interception, which is out of current Ocentra Parent scope;

- settingId: `network-guide-network-control-layers-proxy-001-140`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 358
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

206.  certificates, trust, and privacy risk are high if interception is introduced later;

- settingId: `network-guide-network-control-layers-proxy-001-141`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 360
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

207.  QUIC and direct sockets can bypass unless separately controlled.

- settingId: `network-guide-network-control-layers-proxy-001-142`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 362
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### Router

208.  home-wide DNS, IP, port, or device rules;

- settingId: `network-guide-network-control-layers-router-001-143`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 371
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

209.  useful for IoT and guest devices;

- settingId: `network-guide-network-control-layers-router-001-144`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 372
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

210.  usually cannot see local process, active tab, foreground app, or child profile;

- settingId: `network-guide-network-control-layers-router-001-146`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 377
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

211.  device identity can be ambiguous with MAC randomization, NAT, VPN, or shared devices;

- settingId: `network-guide-network-control-layers-router-001-147`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 378
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

212.  remote/off-home traffic is not covered;

- settingId: `network-guide-network-control-layers-router-001-148`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 380
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

213.  router vendor APIs vary widely;

- settingId: `network-guide-network-control-layers-router-001-149`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 381
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

214.  router proof is separate from Windows endpoint proof.

- settingId: `network-guide-network-control-layers-router-001-150`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 382
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Modern Network Limits

#### HTTPS, DoH, QUIC, ECH, And CDNs

215.  exact URL rules require managed browser, explicit URL filter, or proxy proof;

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-159`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 416
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

216.  IP block rules should warn about shared infrastructure risk;

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-161`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 418
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

217.  DoH/DoT policy needs a specific resolver-control or network-blocking posture;

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-162`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 419
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

218.  QUIC blocking can force TCP fallback for some browsers/sites, but that is a compatibility decision and not a content-inspection feature;

- settingId: `network-guide-modern-network-limits-https-doh-quic-ech-and-cdns-001-163`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 420
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Private Relay And Platform Privacy Features

219.  block only where a platform adapter proves support.

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-171`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 450
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Domain Blocking Versus Exact URL Blocking

#### Domain Blocking Versus Exact URL Blocking

220.  allow or block `example.invalid`;

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-172`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 456
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

221.  allow or block subdomains;

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-173`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 457
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

222.  block a category when the category source is explicit;

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-174`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 458
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

223.  block a known resolver, VPN gateway, proxy, or tunnel host;

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-175`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 459
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

224.  block an app from making network connections to a domain/IP/port.

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-176`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 460
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

225.  allow or block a specific page path;

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-177`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 464
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

226.  handle query strings;

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-178`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 465
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

227.  classify a specific video page;

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-179`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 466
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

228.  block a browser download source URL.

- settingId: `network-guide-domain-blocking-versus-exact-url-blocking-domain-blocking-versus-exact-url-blocking-001-180`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 467
- acceptedOptions: Represented | Not Represented
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

### Platform Capability Notes

#### Windows

229.  Windows Firewall for IP, port, protocol, service, and application rules;

- settingId: `network-guide-platform-capability-notes-windows-001-223`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 541
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

230.  Windows Filtering Platform for future observation and enforcement adapters;

- settingId: `network-guide-platform-capability-notes-windows-001-224`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 542
- acceptedOptions: Represented | Not Represented
- helperText: real-platform-network-adapter-proof

231.  WFP, ETW, firewall, and service paths may require admin rights and careful installer/service setup;

- settingId: `network-guide-platform-capability-notes-windows-001-227`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 548
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

232.  broad domain/network blocking is still manual-required until a real adapter proof exists;

- settingId: `network-guide-platform-capability-notes-windows-001-228`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 550
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### macOS

233.  Network Extension content filter or app proxy paths where entitled and approved;

- settingId: `network-guide-platform-capability-notes-macos-001-232`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 563
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

234.  DNS/proxy/VPN settings when allowed by setup.

- settingId: `network-guide-platform-capability-notes-macos-001-236`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 568
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

235.  do not assume Windows WFP or process-control semantics map to macOS;

- settingId: `network-guide-platform-capability-notes-macos-001-238`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 574
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Linux

236.  nftables/netfilter, iptables compatibility, or distro firewall managers;

- settingId: `network-guide-platform-capability-notes-linux-001-240`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 584
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

237.  DNS/proxy/VPN controls;

- settingId: `network-guide-platform-capability-notes-linux-001-242`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 586
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

238.  router/server Linux is not the same as a child desktop agent;

- settingId: `network-guide-platform-capability-notes-linux-001-246`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 593
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Android

239.  VpnService for an app-owned VPN path;

- settingId: `network-guide-platform-capability-notes-android-001-248`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 602
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

240.  always-on VPN and lockdown when user, device owner, or profile owner setup permits;

- settingId: `network-guide-platform-capability-notes-android-001-249`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 603
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

241.  a normal app cannot broadly firewall every other app without a VPN-style or device-management boundary;

- settingId: `network-guide-platform-capability-notes-android-001-253`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 613
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

242.  per-app VPN and always-on behavior need device proof and UX setup;

- settingId: `network-guide-platform-capability-notes-android-001-255`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 617
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

243.  school/work VPN conflicts need explicit handling.

- settingId: `network-guide-platform-capability-notes-android-001-256`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 618
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Router And Home Network

244.  DHCP/device identity;

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-266`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 649
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

245.  DNS resolver policy;

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-267`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 650
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

246.  firewall/IP/port rules;

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-268`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 651
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

247.  LAN device grouping;

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-270`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 653
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

248.  parent-owned router API integration.

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-271`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 654
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

249.  weak process/child attribution;

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-272`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 658
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

250.  vendor-specific APIs;

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-273`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 659
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

251.  MAC randomization and device sharing;

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-274`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 660
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

252.  no off-home coverage;

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-275`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 661
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

253.  no exact URL path/query without explicit proxy/filter integration.

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-276`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 662
- acceptedOptions: Enabled | Disabled
- helperText: Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.

### Policy Modes To Represent Later In UI

#### Observe Network Activity

254.  network blocking;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-observe-network-activity-001-280`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 686
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Domain Rules

255.  allow, warn, ask, limit, or block domains and subdomains where a domain-control layer is available;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-284`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 695
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

256.  CDN-safe IP blocking;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-288`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 703
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

257.  process attribution from router or DNS-only data.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-domain-rules-001-289`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 704
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### IP, Port, And Protocol Rules

258.  allow or block remote IPs, CIDRs, local/remote ports, and transport protocols through firewall/WFP/VPN/router/packet-filter adapters.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-ip-port-and-protocol-rules-001-290`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 710
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### VPN, Proxy, And Tunnel Handling

259.  optionally require managed network path where supported.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling-001-295`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 724
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

260.  Ocentra knows tunneled destinations or content.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling-001-296`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 728
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Strict Network Enforcement

261.  child-device agent applies platform network control after a typed policy decision references stored evidence or a parent-authored target.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-306`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 761
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

262.  adapter capability proof;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-307`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 766
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

263.  policy decision;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-308`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 767
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

264.  visible unsupported/degraded state.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-311`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 770
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

265.  Windows is first, but broad network/domain blocking remains manual-required until real OS adapter proof exists.

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-316`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `local-ai-runtime`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 782
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

266.  Portal UI authors rules and shows capability states. It does not run capture, policy evaluation, enforcement, timers, OS commands, or scripts.

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-318`
- policyLane: `enforcement`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 786
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

267.  [`docs/expectations/enforcement.md`](expectations/enforcement.md)

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-322`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 794
- acceptedOptions: Represented | Not Represented
- helperText: real-platform-network-adapter-proof

### Future UI Rules

#### Future UI Rules

268.  Show IP-only, domain-ambiguous, DNS-unavailable, encrypted-content-unavailable, process-unknown, adapter-unavailable, and stale states directly.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-328`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 806
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

269.  Show capability status beside each strict action: ready, unsupported, permission-required, adapter-missing, proof-missing, degraded, monitor-only, manual-required, or unavailable.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-331`
- policyLane: `enforcement`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 811
- acceptedOptions: Ready | Unsupported | Permission Required | Adapter Missing | Proof Missing | Degraded | Monitor Only | Manual Required | Unavailable
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

270.  VPN/proxy/tunnel handling;

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-337`
- policyLane: `enforcement`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 824
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

271.  strict network enforcement where proven;

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-340`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 827
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

272.  router or cloud relay options only where separately configured and proven.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-342`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 829
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

## Tab: schedule

### Core Terms

#### Network Flow Evidence

273.  timestamps, connection counts, duration, and bytes sent/received where the adapter can prove them;

- settingId: `network-guide-core-terms-network-flow-evidence-001-008`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 34
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Budgets

#### Budgets

274.  Enable network budgets?

- settingId: `budgets.enabled`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 530
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

275.  What should network budgets count?

- settingId: `budgets.countingMode`
- policyLane: `schedule`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-schema-proposal.md`; sourceLine: 537
- acceptedOptions: Flow Bytes When Available | Connection Count | Network Active Time | Foreground Correlated Time | New Destination Count
- helperText: network-control-capability-registry

### The Main Capability Truth

#### The Main Capability Truth

276.  bandwidth and connection-count summaries;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-024`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 120
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### Capability Matrix

#### Capability Matrix

277.  Capability matrix row | Capability=Bandwidth budget | What can be possible=Possible where byte counters are reliable | Required layer=Flow counters, firewall/VPN/router counters | Important limit=DNS-only and endpoint snapshots may not provide byte counts.

- settingId: `network-guide-capability-matrix-capability-matrix-001-046`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 155
- acceptedOptions: Capability Bandwidth Budget | What Can Be Possible Possible Where Byte Counters Are Reliable | Required Layer Flow Counters Firewall Vpn Router Counters | Important Limit Dns Only And Endpoint Snapshots May Not Provide Byte Counts
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

278.  Capability matrix row | Capability=Time budget | What can be possible=Possible as flow-active, process-active, or foreground-app time | Required layer=Flow/process/browser/session timers | Important limit=Network-active time is not the same as active child attention.

- settingId: `network-guide-capability-matrix-capability-matrix-001-047`
- policyLane: `schedule`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 156
- acceptedOptions: Capability Time Budget | What Can Be Possible Possible As Flow Active Process Active Or Foreground App Time | Required Layer Flow Process Browser Session Timers | Important Limit Network Active Time Is Not The Same As Active Child Attention
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Network Visibility: What Is Possible

#### Bandwidth And Time Budgets

279.  flow byte budget: bytes sent/received by matching flows;

- settingId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets-001-091`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 244
- acceptedOptions: Bytes Sent Received By Matching Flows
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

280.  connection-count budget: number of connections or new destinations;

- settingId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets-001-092`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 245
- acceptedOptions: Number Of Connections | New Destinations
- helperText: network-control-capability-registry

281.  network-active time: time a matching flow was active;

- settingId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets-001-093`
- policyLane: `schedule`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 246
- acceptedOptions: Time A Matching Flow Was Active
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

282.  foreground-correlated time: network-active time while a process/browser/app was foreground;

- settingId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets-001-094`
- policyLane: `schedule`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 247
- acceptedOptions: Network Active Time While A Process Browser App Was Foreground
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

283.  schedule budget: whether a rule is active during a local time window.

- settingId: `network-guide-network-visibility-what-is-possible-bandwidth-and-time-budgets-001-095`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 249
- acceptedOptions: Whether A Rule Is Active During A Local Time Window
- helperText: network-control-capability-registry

#### Suspicious Indicators

284.  traffic during blocked schedule or after budget exhaustion.

- settingId: `network-guide-network-visibility-what-is-possible-suspicious-indicators-001-107`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 272
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Network Control Layers

#### Router

285.  can enforce local network access and internet schedules.

- settingId: `network-guide-network-control-layers-router-001-145`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 373
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Platform Capability Notes

#### Windows

286.  endpoint snapshots can miss short-lived flows and may not provide bytes;

- settingId: `network-guide-platform-capability-notes-windows-001-229`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 552
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Router And Home Network

287.  bandwidth/time schedules;

- settingId: `network-guide-platform-capability-notes-router-and-home-network-001-269`
- policyLane: `schedule`; cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 652
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Policy Modes To Represent Later In UI

#### Bandwidth And Time Budgets

288.  apply budgets to flow bytes, connection counts, network-active duration, or foreground-correlated duration according to the selected evidence source.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets-001-297`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 734
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

289.  counted evidence type;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets-001-298`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 739
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

290.  reset window;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets-001-299`
- policyLane: `schedule`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 740
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

291.  degraded behavior when counters are missing;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets-001-300`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `future-gap`; runtimeOwner: `parent-domain`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 741
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

292.  whether background traffic counts.

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-bandwidth-and-time-budgets-001-301`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 742
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### Future UI Rules

#### Future UI Rules

293.  Keep process, domain, IP, port, protocol, VPN/proxy/tunnel, LAN exception, and bandwidth/time budget rules as separate target types.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-329`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `local-ai-runtime`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 808
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

294.  bandwidth and network-active time budgets;

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-338`
- policyLane: `schedule`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 825
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

## Tab: approvals

### Core Terms

#### Network Control

295.  warn, ask parent, or report instead of blocking;

- settingId: `network-guide-core-terms-network-control-001-013`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 73
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Network Control Layers

#### Firewall, WFP, And Packet Filter

296.  may need admin rights, service installation, signed drivers, system extensions, MDM, or entitlement approval;

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-122`
- policyLane: `approvals`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 317
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Cloud Relay

297.  parent approval requests;

- settingId: `network-guide-network-control-layers-cloud-relay-001-152`
- policyLane: `approvals`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 392
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### Modern Network Limits

#### Private Relay And Platform Privacy Features

298.  ask parent;

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-169`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 448
- acceptedOptions: Represented | Not Represented
- helperText: network-control-capability-registry

### Policy Modes To Represent Later In UI

#### VPN, Proxy, And Tunnel Handling

299.  observe, warn, ask, block, or require approval for VPN/proxy/tunnel indicators;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-vpn-proxy-and-tunnel-handling-001-294`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 723
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

## Tab: reports

### Reports, Custody, Retention, And Audit

#### Reports, Custody, Retention, And Audit

300.  recent flows;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-181`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 481
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

301.  top processes;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-182`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 482
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

302.  top domains;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-183`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 483
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

303.  top IPs;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-184`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 484
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

304.  top ports/protocols;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-185`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 485
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

305.  new destinations;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-186`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 486
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

306.  bandwidth summaries;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-187`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 487
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

307.  time-window summaries;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-188`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 488
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

308.  VPN/proxy/tunnel indicators;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-189`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 489
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

309.  LAN exceptions used;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-190`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 490
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

310.  blocked/allowed/warned/asked decisions;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-191`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 491
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

311.  enforcement failures and unavailable states;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-192`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 492
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

312.  source/custody labels.

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-193`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 493
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

313.  live local child agent;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-194`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 497
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

314.  live LAN child agent;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-195`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 498
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

315.  child-device encrypted journal;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-196`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 499
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

316.  child-device SQLite query store;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-197`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 500
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

317.  parent-device cache;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-198`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 501
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

318.  parent-owned export;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-199`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 502
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

319.  parent-authorized relay;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-200`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 503
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

320.  Ocentra-hosted non-activity metadata;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-201`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 504
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

321.  unavailable.

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-202`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `degraded`; runtimeOwner: `portal-only`; capabilityState: `degraded`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 505
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

322.  raw flow evidence retention;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-203`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 509
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

323.  domain summary retention;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-204`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 510
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

324.  bandwidth summary retention;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-205`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 511
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

325.  policy/audit retention;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-206`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 512
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

326.  exported report retention;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-207`
- policyLane: `reports`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 513
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

327.  deletion and expiry behavior;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-208`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 514
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

328.  whether redacted summaries survive raw evidence deletion.

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-209`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 515
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

329.  policy version;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-210`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 519
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

330.  rule id;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-211`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 520
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

331.  evidence id;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-212`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `portal-only`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 521
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

332.  adapter id and capability state;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-213`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 522
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

333.  action requested;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-214`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 523
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

334.  action result;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-215`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 524
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

335.  rollback or expiry state;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-216`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 525
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

336.  parent approval or override reference;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-217`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 526
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

337.  custody label;

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-218`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 527
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

338.  timestamp and source.

- settingId: `network-guide-reports-custody-retention-and-audit-reports-custody-retention-and-audit-001-219`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 528
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Future UI Rules

#### Future UI Rules

339.  Keep custody labels close to reports, AI summaries, exports, and parent assistant surfaces.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-333`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `portal-only`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 816
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

## Tab: audit

### Core Terms

#### Network Flow Evidence

340.  evidence id, source id, capability state, custody state, and retention state.

- settingId: `network-guide-core-terms-network-flow-evidence-001-009`
- policyLane: `audit`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 36
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### The Main Capability Truth

#### The Main Capability Truth

341.  per-process DNS attribution when the source is host-level DNS cache only;

- settingId: `network-guide-the-main-capability-truth-the-main-capability-truth-001-033`
- policyLane: `audit`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 134
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Capability Matrix

#### Capability Matrix

342.  Capability matrix row | Capability=Attribute domain | What can be possible=Sometimes through DNS events/cache or managed resolver | Required layer=DNS adapter, resolver, browser join, proxy | Important limit=DoH, DoT, VPNs, ECH, CDNs, and cache ambiguity reduce confidence.

- settingId: `network-guide-capability-matrix-capability-matrix-001-039`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 148
- acceptedOptions: Capability Attribute Domain | What Can Be Possible Sometimes Through Dns Events Cache Or Managed Resolver | Required Layer Dns Adapter Resolver Browser Join Proxy | Important Limit Doh Dot Vpns Ech Cdns And Cache Ambiguity Reduce Confidence
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

343.  Capability matrix row | Capability=Block domain | What can be possible=Possible through DNS, proxy, browser, VPN, WFP, router | Required layer=Domain resolver/control layer | Important limit=DoH/DoT, hard-coded IPs, CDNs, and cached connections can bypass or blur.

- settingId: `network-guide-capability-matrix-capability-matrix-001-042`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 151
- acceptedOptions: Capability Block Domain | What Can Be Possible Possible Through Dns Proxy Browser Vpn Wfp Router | Required Layer Domain Resolver Control Layer | Important Limit Doh Dot Hard Coded Ips Cdns And Cached Connections Can Bypass Or Blur
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

344.  Capability matrix row | Capability=Audit enforcement result | What can be possible=Required for strict actions | Required layer=Journaled policy decision plus adapter result | Important limit=A rule value alone is not proof that traffic was blocked.

- settingId: `network-guide-capability-matrix-capability-matrix-001-054`
- policyLane: `audit`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 163
- acceptedOptions: Capability Audit Enforcement Result | What Can Be Possible Required For Strict Actions | Required Layer Journaled Policy Decision Plus Adapter Result | Important Limit A Rule Value Alone Is Not Proof That Traffic Was Blocked
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Network Visibility: What Is Possible

#### DNS, Domain, IP, Port, And Protocol

345.  DNS client cache or observed resolver events;

- settingId: `network-guide-network-visibility-what-is-possible-dns-domain-ip-port-and-protocol-001-057`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 173
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Network Control Layers

#### DNS And Managed Resolver

346.  DNS answers can be cached before policy changes;

- settingId: `network-guide-network-control-layers-dns-and-managed-resolver-001-112`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 293
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

#### Firewall, WFP, And Packet Filter

347.  can produce auditable adapter results.

- settingId: `network-guide-network-control-layers-firewall-wfp-and-packet-filter-001-121`
- policyLane: `audit`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 313
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### VPN Or Tunnel Adapter

348.  must not export child activity to Ocentra-hosted infrastructure by default;

- settingId: `network-guide-network-control-layers-vpn-or-tunnel-adapter-001-134`
- policyLane: `audit`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 340
- acceptedOptions: Represented | Not Represented
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

#### Cloud Relay

349.  storage of child network evidence by default;

- settingId: `network-guide-network-control-layers-cloud-relay-001-157`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 400
- acceptedOptions: Enabled | Disabled
- helperText: local-first-custody-and-retention-policy

350.  exact network observations unless the child agent uploaded parent-authorized typed summaries under an explicit custody setting.

- settingId: `network-guide-network-control-layers-cloud-relay-001-158`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 401
- acceptedOptions: Enabled | Disabled
- helperText: local-first-custody-and-retention-policy

### Platform Capability Notes

#### Windows

351.  DNS client cache or DNS event observation;

- settingId: `network-guide-platform-capability-notes-windows-001-221`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 539
- acceptedOptions: Enabled | Disabled
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

352.  DNS cache is host-level unless a stronger source proves per-process attribution;

- settingId: `network-guide-platform-capability-notes-windows-001-230`
- policyLane: `audit`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 553
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

### Policy Modes To Represent Later In UI

#### Strict Network Enforcement

353.  audit event;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-309`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 768
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

354.  rollback or expiry path;

- settingId: `network-guide-policy-modes-to-represent-later-in-ui-strict-network-enforcement-001-310`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 769
- acceptedOptions: Enabled | Disabled
- helperText: Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

355.  Policy can consume stored network summaries and unusual indicators only after they are journaled and queryable.

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-314`
- policyLane: `audit`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 778
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

356.  Enforcement is scaffold/protocol/audit work unless a real platform adapter proof exists.

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-315`
- policyLane: `audit`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 780
- acceptedOptions: Represented | Not Represented
- helperText: Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.

357.  [`docs/expectations/data-custody.md`](expectations/data-custody.md)

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-323`
- policyLane: `audit`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 795
- acceptedOptions: Enabled | Disabled
- helperText: local-first-custody-and-retention-policy

### Future UI Rules

#### Future UI Rules

358.  Require proof for enforcement claims: parent rule, evidence reference, policy decision, adapter action, adapter result, audit row, and rollback/expiry state.

- settingId: `network-guide-future-ui-rules-future-ui-rules-001-332`
- policyLane: `audit`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `action-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 814
- acceptedOptions: Parent Rule | Evidence Reference | Policy Decision | Adapter Action | Adapter Result | Audit Row | And Rollback Expiry State
- helperText: real-platform-network-adapter-proof

## Tab: setup

### Modern Network Limits

#### Private Relay And Platform Privacy Features

359.  require managed browser/network path;

- settingId: `network-guide-modern-network-limits-private-relay-and-platform-privacy-features-001-170`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 449
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### Platform Capability Notes

#### macOS

360.  configuration profiles or MDM for stronger managed-device cases;

- settingId: `network-guide-platform-capability-notes-macos-001-233`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `child-agent`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 565
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### Linux

361.  privilege and service installation differ by distro;

- settingId: `network-guide-platform-capability-notes-linux-001-245`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `child-agent`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 592
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

#### Android

362.  DevicePolicyManager controls for device-owner or profile-owner deployments;

- settingId: `network-guide-platform-capability-notes-android-001-250`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 605
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

363.  [`docs/managed-unmanaged-browser.md`](managed-unmanaged-browser.md)

- settingId: `network-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-001-325`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/network-control-capability-guide.md`; sourceLine: 797
- acceptedOptions: Enabled | Disabled
- helperText: network-control-capability-registry
