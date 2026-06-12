<!-- agent-capsule -->

> Agent Capsule
> Doc: Network Control Schema Proposal
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Network Control Schema Proposal

Status: proposal for worker handoff. This is not final source code.

This document proposes a structured network control schema that can support:

- Portal-rendered question/option UI.
- Parent-authored network settings.
- Child-agent local persisted policy.
- Offline enforcement from the last valid policy.
- Small patch updates from Portal.
- Full policy replacement during setup/import/reset.
- Deterministic compile into an effective network enforcement plan.

The JSON in this document is intentionally product-shaped rather than
repo-strict. The implementation worker must not copy it directly into runtime
code. The worker should use it as a guide and then build proper Ocentra Parent
contracts with:

- Effect Schema validation.
- Branded ids from schema brands, not manual brands.
- Decode helpers.
- No naked domain strings in app/runtime code.
- Tests for every parser, authoring manifest field, policy value shape, compile
  rule, patch command, invalid-state rejection, and degraded capability state.
- Rust protocol parity only after the TypeScript contracts are explicit and
  test-backed.
- Local child-agent persistence, compile, rollback, enforcement, and audit
  behavior where network control crosses into the Rust service.

## Architecture

The proposal has four related documents.

### Authoring Manifest

The authoring manifest tells Portal what questions to show, what controls to
render, which options are allowed, where the answer writes into the policy value
document, and when the field is visible or enabled.

Portal must not invent network policy questions outside this manifest. If the UI
needs a new question, the manifest and value schema need a contract update.

### Policy Value Document

The policy value document is the parent-authored network policy. It is the
durable source of parent intent. The child agent validates it as a whole after
any update.

### Effective Policy Document

The effective policy document is the compiled execution plan. The child agent
uses it for local policy preview and enforcement. It should be deterministic,
flat enough for runtime, and explicit about fallback behavior when network proof
is unavailable.

### Policy Update Commands

Portal sends typed update commands. The child agent validates, persists, compiles,
and acknowledges. Portal is never in the enforcement hot path.

```text
Portal authoring UI
  -> policy update command
  -> child agent validates full policy value
  -> child agent persists policy revision
  -> child agent compiles effective policy
  -> child agent enforces locally where a proven adapter exists
```

## Proposed Complete JSON Shape

The following JSON combines the proposed authoring manifest, policy value,
effective policy, update commands, and capability registry into one example so a
worker can see how the pieces relate.

```json
{
  "schemaVersion": 1,
  "proposalStatus": "worker-handoff-design-proposal-not-runtime-contract",
  "proposalIntent": "Guide the implementation of network policy authoring, storage, compile, and enforcement contracts.",
  "workerInstruction": {
    "takeAsGuideOnly": true,
    "mustTranslateToEffectSchema": true,
    "mustUseSchemaBrands": true,
    "mustAddDecodeHelpers": true,
    "mustAddTests": true,
    "mustPreserveLocalChildAgentEnforcement": true,
    "mustNotCopyJsonDirectlyIntoRuntime": true,
    "mustAddRustParityOnlyAfterTypeScriptContracts": true
  },
  "contractFamilies": {
    "authoringManifest": "Portal-rendered sections, questions, options, visibility, enabled state, writesTo paths, and validation hints.",
    "policyValue": "Parent-authored durable network policy state stored and versioned by the child agent.",
    "effectivePolicy": "Compiled deterministic child-agent execution plan.",
    "updateProtocol": "Typed get, preview, patch, replace, ack, reject, and rollback commands.",
    "capabilityRegistry": "Runtime device/platform/network capability states used to hide, disable, or degrade fields."
  },
  "authoringManifest": {
    "manifestId": "network-control-authoring-v1",
    "policyKind": "network-control",
    "schemaVersion": 1,
    "title": "Network controls",
    "renderingRules": {
      "hideInvisibleFields": true,
      "showDisabledFieldsWithReason": true,
      "neverInventFieldsOutsideManifest": true,
      "writeOnlyThroughWritesToPath": true,
      "previewBeforeApply": true,
      "showExactUrlRequiresManagedBrowser": true,
      "showDomainConfidenceStates": true
    },
    "controlKinds": [
      "boolean",
      "single-choice",
      "multi-choice",
      "number",
      "duration",
      "schedule",
      "rule-list",
      "target-list",
      "retention",
      "action-list",
      "read-only-status"
    ],
    "conditionKinds": [
      "equals",
      "notEquals",
      "includes",
      "notIncludes",
      "all",
      "any",
      "capabilityAvailable",
      "platformIn",
      "proofAtLeast"
    ],
    "sections": [
      {
        "sectionId": "network-management",
        "title": "Network management",
        "purpose": "Top-level network policy switch and default posture.",
        "fields": [
          {
            "fieldId": "network.enabled",
            "kind": "boolean",
            "question": "Enable network management?",
            "writesTo": "/networkPolicy/enabled",
            "defaultValue": false,
            "uiPriority": 10,
            "whenFalse": {
              "policyMeaning": "Network activity is allowed and network controls do not enforce.",
              "hiddenSections": [
                "evidence-scope",
                "domain-dns",
                "flow-rules",
                "vpn-proxy-tunnel",
                "budgets",
                "local-network",
                "reports",
                "retention",
                "audit"
              ]
            }
          },
          {
            "fieldId": "network.defaultPosture",
            "kind": "single-choice",
            "question": "What should happen to network activity?",
            "writesTo": "/networkPolicy/defaultPosture",
            "defaultValue": "observe",
            "visibleWhen": {
              "path": "/networkPolicy/enabled",
              "equals": true
            },
            "options": [
              {
                "value": "allow",
                "label": "Allow",
                "meaning": "Network activity is allowed unless a more specific rule changes it.",
                "relevantSections": ["local-network", "reports", "audit"]
              },
              {
                "value": "observe",
                "label": "Observe",
                "meaning": "Network activity is allowed, metadata evidence is collected according to data scope, and decisions are report-only.",
                "relevantSections": ["evidence-scope", "reports", "retention", "audit"]
              },
              {
                "value": "warn",
                "label": "Warn",
                "meaning": "Matching activity warns the child and records parent-visible events.",
                "relevantSections": ["domain-dns", "flow-rules", "reports", "audit"]
              },
              {
                "value": "ask",
                "label": "Ask",
                "meaning": "Matching network activity needs parent approval unless an allow rule or override applies.",
                "relevantSections": ["flow-rules", "vpn-proxy-tunnel", "reports", "audit"]
              },
              {
                "value": "limit",
                "label": "Limit",
                "meaning": "Network activity is allowed inside configured schedules and budgets.",
                "relevantSections": ["budgets", "flow-rules", "reports", "audit"]
              },
              {
                "value": "block",
                "label": "Block",
                "meaning": "Network activity is blocked by default unless an explicit exception or parent override allows it.",
                "relevantSections": ["local-network", "flow-rules", "audit"]
              }
            ]
          },
          {
            "fieldId": "network.managementMode",
            "kind": "single-choice",
            "question": "How should network management run on this device?",
            "writesTo": "/networkPolicy/managementMode",
            "defaultValue": "local-child-agent",
            "visibleWhen": {
              "path": "/networkPolicy/enabled",
              "equals": true
            },
            "options": ["local-child-agent", "lan-live", "authoring-only", "unavailable"]
          }
        ]
      },
      {
        "sectionId": "evidence-scope",
        "title": "Evidence scope",
        "purpose": "Choose what network metadata may be collected and used.",
        "visibleWhen": {
          "path": "/networkPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "evidence.metadataScope",
            "kind": "multi-choice",
            "question": "What network evidence may rules use?",
            "writesTo": "/networkPolicy/evidence/metadataScope",
            "defaultValue": [
              "process",
              "ip",
              "port",
              "protocol",
              "domain-candidate",
              "dns-source-state",
              "connection-count",
              "duration",
              "bytes-when-available",
              "adapter-state"
            ],
            "options": [
              "process",
              "ip",
              "port",
              "protocol",
              "domain-candidate",
              "dns-source-state",
              "connection-count",
              "duration",
              "bytes-when-available",
              "interface",
              "route",
              "lan-wan-classification",
              "vpn-proxy-tunnel-indicator",
              "adapter-state"
            ]
          },
          {
            "fieldId": "evidence.requiredProof",
            "kind": "single-choice",
            "question": "What proof is enough for network decisions?",
            "writesTo": "/networkPolicy/evidence/requiredProof",
            "defaultValue": "stored-flow-summary",
            "options": [
              "capability-only",
              "endpoint-observed",
              "process-attributed-flow",
              "domain-candidate",
              "domain-known",
              "stored-flow-summary",
              "managed-browser-joined-domain",
              "adapter-enforcement-proof"
            ]
          },
          {
            "fieldId": "evidence.whenProofUnavailable",
            "kind": "single-choice",
            "question": "What if network proof is unavailable?",
            "writesTo": "/networkPolicy/evidence/whenProofUnavailable",
            "defaultValue": "observe",
            "options": ["allow", "observe", "warn", "ask", "block-until-ready", "mark-unavailable"]
          },
          {
            "fieldId": "evidence.neverCollect",
            "kind": "multi-choice",
            "question": "What must network rules never collect?",
            "writesTo": "/networkPolicy/evidence/neverCollect",
            "defaultValue": [
              "decrypted-https-payload",
              "packet-payload",
              "page-body",
              "chat-content",
              "search-terms",
              "form-values",
              "cookies",
              "tokens",
              "credentials",
              "raw-packet-dumps",
              "raw-trace-files"
            ],
            "options": [
              "decrypted-https-payload",
              "packet-payload",
              "page-body",
              "chat-content",
              "search-terms",
              "form-values",
              "cookies",
              "tokens",
              "credentials",
              "raw-packet-dumps",
              "raw-trace-files"
            ]
          }
        ]
      },
      {
        "sectionId": "domain-dns",
        "title": "Domains and DNS",
        "purpose": "Configure domain attribution and DNS/control behavior without claiming exact URL evidence.",
        "visibleWhen": {
          "path": "/networkPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "dns.mode",
            "kind": "single-choice",
            "question": "How should domain controls work?",
            "writesTo": "/networkPolicy/dns/mode",
            "defaultValue": "observe-and-classify",
            "options": [
              "disabled",
              "observe-and-classify",
              "managed-resolver-preferred",
              "managed-resolver-required",
              "block-unapproved-encrypted-dns"
            ]
          },
          {
            "fieldId": "dns.allowedAttributionSources",
            "kind": "multi-choice",
            "question": "Which domain attribution sources may be used?",
            "writesTo": "/networkPolicy/dns/allowedAttributionSources",
            "defaultValue": ["observed-dns-event", "dns-client-cache", "managed-browser-join"],
            "options": [
              "observed-dns-event",
              "dns-client-cache",
              "managed-resolver-log",
              "reverse-dns",
              "static-hosts",
              "managed-browser-join",
              "parent-rule-domain"
            ]
          },
          {
            "fieldId": "dns.encryptedDnsHandling",
            "kind": "single-choice",
            "question": "How should encrypted DNS be handled?",
            "writesTo": "/networkPolicy/dns/encryptedDnsHandling",
            "defaultValue": "report-only",
            "options": ["allow", "report-only", "warn", "ask", "block-unknown-resolvers", "require-managed-resolver"]
          }
        ]
      },
      {
        "sectionId": "flow-rules",
        "title": "Flow rules",
        "purpose": "Rules for domains, IPs, ports, protocols, processes, categories, and network capability states.",
        "visibleWhen": {
          "all": [
            {
              "path": "/networkPolicy/enabled",
              "equals": true
            },
            {
              "path": "/networkPolicy/defaultPosture",
              "notEquals": "allow"
            }
          ]
        },
        "fields": [
          {
            "fieldId": "rules.allowedTargetTypes",
            "kind": "multi-choice",
            "question": "What network targets should rules match?",
            "writesTo": "/networkPolicy/rules/allowedTargetTypes",
            "defaultValue": [
              "domain",
              "ip",
              "port",
              "protocol",
              "process",
              "destination-category",
              "vpn-proxy-tunnel",
              "capability-state"
            ],
            "options": [
              "domain",
              "ip",
              "cidr",
              "port",
              "protocol",
              "process",
              "interface",
              "route",
              "destination-category",
              "vpn-proxy-tunnel",
              "new-destination",
              "high-volume",
              "repeated-failure",
              "capability-state"
            ]
          },
          {
            "fieldId": "rules.allowedActions",
            "kind": "multi-choice",
            "question": "What actions can network rules take?",
            "writesTo": "/networkPolicy/rules/allowedActions",
            "defaultValue": ["allow", "observe", "warn", "ask", "limit", "block"],
            "options": [
              "allow",
              "observe",
              "warn",
              "ask",
              "limit",
              "block",
              "terminate-process",
              "require-managed-network"
            ]
          },
          {
            "fieldId": "rules.conflictResolution",
            "kind": "multi-choice",
            "question": "How should conflicting network rules resolve?",
            "writesTo": "/networkPolicy/rules/conflictResolution",
            "defaultValue": [
              "explicit-local-exception-beats-block",
              "process-rule-beats-domain-rule",
              "domain-beats-ip-category",
              "block-beats-allow",
              "fresh-proof-beats-stale-proof",
              "adapter-proof-required-for-enforcement"
            ],
            "options": [
              "explicit-local-exception-beats-block",
              "process-rule-beats-domain-rule",
              "domain-beats-ip-category",
              "managed-browser-domain-beats-dns-candidate",
              "block-beats-allow",
              "fresh-proof-beats-stale-proof",
              "adapter-proof-required-for-enforcement"
            ]
          }
        ]
      },
      {
        "sectionId": "vpn-proxy-tunnel",
        "title": "VPN, proxy, and tunnel",
        "purpose": "Choose how VPN, proxy, tunnel, Tor-like, and encrypted relay indicators are handled.",
        "visibleWhen": {
          "path": "/networkPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "tunnel.mode",
            "kind": "single-choice",
            "question": "What should happen to VPN/proxy/tunnel indicators?",
            "writesTo": "/networkPolicy/tunnel/mode",
            "defaultValue": "observe",
            "options": ["allow", "observe", "warn", "ask", "block", "require-managed-network"]
          },
          {
            "fieldId": "tunnel.indicators",
            "kind": "multi-choice",
            "question": "Which tunnel indicators should count?",
            "writesTo": "/networkPolicy/tunnel/indicators",
            "defaultValue": [
              "vpn-interface",
              "proxy-process",
              "known-tunnel-port",
              "tor-like-process",
              "unknown-encrypted-relay"
            ],
            "options": [
              "vpn-interface",
              "proxy-process",
              "proxy-config",
              "known-tunnel-port",
              "tor-like-process",
              "unknown-encrypted-relay",
              "masque-like-flow",
              "dns-unavailable-with-public-traffic"
            ]
          }
        ]
      },
      {
        "sectionId": "budgets",
        "title": "Budgets",
        "purpose": "Configure network bandwidth, connection-count, and network-active time limits.",
        "visibleWhen": {
          "all": [
            {
              "path": "/networkPolicy/enabled",
              "equals": true
            },
            {
              "path": "/networkPolicy/defaultPosture",
              "includes": ["observe", "warn", "ask", "limit", "block"]
            }
          ]
        },
        "fields": [
          {
            "fieldId": "budgets.enabled",
            "kind": "boolean",
            "question": "Enable network budgets?",
            "writesTo": "/networkPolicy/budgets/enabled",
            "defaultValue": false
          },
          {
            "fieldId": "budgets.countingMode",
            "kind": "single-choice",
            "question": "What should network budgets count?",
            "writesTo": "/networkPolicy/budgets/countingMode",
            "defaultValue": "flow-bytes-when-available",
            "visibleWhen": {
              "path": "/networkPolicy/budgets/enabled",
              "equals": true
            },
            "options": [
              "flow-bytes-when-available",
              "connection-count",
              "network-active-time",
              "foreground-correlated-time",
              "new-destination-count"
            ]
          }
        ]
      },
      {
        "sectionId": "local-network",
        "title": "Local network",
        "purpose": "Configure explicit local network and Ocentra protocol exceptions.",
        "visibleWhen": {
          "path": "/networkPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "localNetwork.exceptionMode",
            "kind": "single-choice",
            "question": "How should local-network exceptions behave?",
            "writesTo": "/networkPolicy/localNetwork/exceptionMode",
            "defaultValue": "explicit-services-only",
            "options": [
              "disabled",
              "explicit-services-only",
              "trusted-subnets",
              "allow-private-networks",
              "parent-request-for-new-local-destination"
            ]
          },
          {
            "fieldId": "localNetwork.defaultExceptions",
            "kind": "multi-choice",
            "question": "Which local exceptions should be available?",
            "writesTo": "/networkPolicy/localNetwork/defaultExceptions",
            "defaultValue": ["loopback-agent", "lan-parent-controller", "dns-resolver", "dhcp", "printer"],
            "options": [
              "loopback-agent",
              "lan-parent-controller",
              "dns-resolver",
              "dhcp",
              "printer",
              "nas",
              "school-subnet",
              "mdns",
              "ssdp",
              "router-admin"
            ]
          }
        ]
      },
      {
        "sectionId": "reports",
        "title": "Reports",
        "purpose": "Choose parent-visible network summaries and copy/debug behavior.",
        "visibleWhen": {
          "path": "/networkPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "reports.visibleFields",
            "kind": "multi-choice",
            "question": "Which network report fields should parents see?",
            "writesTo": "/networkPolicy/reports/visibleFields",
            "defaultValue": [
              "recent-flows",
              "top-processes",
              "top-domains",
              "top-ips",
              "ports-protocols",
              "bandwidth",
              "vpn-proxy-tunnel-indicators",
              "source-capability",
              "custody-label"
            ],
            "options": [
              "recent-flows",
              "top-processes",
              "top-domains",
              "top-ips",
              "ports-protocols",
              "bandwidth",
              "time-budgets",
              "new-destinations",
              "repeated-failures",
              "vpn-proxy-tunnel-indicators",
              "local-exceptions-used",
              "policy-decisions",
              "block-results",
              "source-capability",
              "custody-label"
            ]
          },
          {
            "fieldId": "reports.showUncertainty",
            "kind": "boolean",
            "question": "Show unknown and ambiguous states?",
            "writesTo": "/networkPolicy/reports/showUncertainty",
            "defaultValue": true
          }
        ]
      }
    ]
  },
  "policyValue": {
    "documentId": "network-policy-mia-windows-laptop",
    "policyKind": "network-control",
    "schemaVersion": 1,
    "revision": 12,
    "updatedAt": "2026-05-28T00:00:00.000Z",
    "scope": {
      "familyId": "family-local-1",
      "childId": "child-mia",
      "deviceId": "device-windows-laptop",
      "platform": "windows"
    },
    "networkPolicy": {
      "enabled": true,
      "defaultPosture": "observe",
      "managementMode": "local-child-agent",
      "evidence": {
        "metadataScope": [
          "process",
          "ip",
          "port",
          "protocol",
          "domain-candidate",
          "dns-source-state",
          "connection-count",
          "duration",
          "bytes-when-available",
          "adapter-state"
        ],
        "requiredProof": "stored-flow-summary",
        "whenProofUnavailable": "observe",
        "allowNetworkDomainForDomainRules": true,
        "allowNetworkDomainForExactUrlRules": false,
        "neverCollect": [
          "decrypted-https-payload",
          "packet-payload",
          "page-body",
          "chat-content",
          "search-terms",
          "form-values",
          "cookies",
          "tokens",
          "credentials",
          "raw-packet-dumps",
          "raw-trace-files"
        ]
      },
      "dns": {
        "mode": "observe-and-classify",
        "allowedAttributionSources": ["observed-dns-event", "dns-client-cache", "managed-browser-join"],
        "encryptedDnsHandling": "report-only",
        "managedResolverIds": ["family-managed-resolver"],
        "unknownResolverAction": "warn"
      },
      "rules": {
        "allowedTargetTypes": [
          "domain",
          "ip",
          "cidr",
          "port",
          "protocol",
          "process",
          "destination-category",
          "vpn-proxy-tunnel",
          "new-destination",
          "high-volume",
          "capability-state"
        ],
        "allowedActions": ["allow", "observe", "warn", "ask", "limit", "block", "require-managed-network"],
        "conflictResolution": [
          "explicit-local-exception-beats-block",
          "process-rule-beats-domain-rule",
          "domain-beats-ip-category",
          "managed-browser-domain-beats-dns-candidate",
          "block-beats-allow",
          "fresh-proof-beats-stale-proof",
          "adapter-proof-required-for-enforcement"
        ],
        "items": [
          {
            "ruleId": "allow-school-domains",
            "enabled": true,
            "priority": 100,
            "target": {
              "kind": "domain",
              "values": ["school.example.invalid", "library.example.invalid"],
              "matchMode": "exact-or-subdomain"
            },
            "action": {
              "kind": "allow",
              "reasonCode": "school-network"
            },
            "proofRequirement": "domain-known-or-managed-browser-domain",
            "scheduleId": "school-hours",
            "budgetId": null,
            "auditLevel": "decision"
          },
          {
            "ruleId": "parent-request-new-public-destination",
            "enabled": true,
            "priority": 200,
            "target": {
              "kind": "new-destination",
              "values": ["public-internet"],
              "matchMode": "first-seen-for-process"
            },
            "action": {
              "kind": "ask",
              "approvalKind": "new-network-destination",
              "reasonCode": "new-destination"
            },
            "proofRequirement": "stored-flow-summary",
            "scheduleId": "always",
            "budgetId": null,
            "auditLevel": "decision-and-evidence"
          },
          {
            "ruleId": "warn-tunnel-indicator",
            "enabled": true,
            "priority": 300,
            "target": {
              "kind": "vpn-proxy-tunnel",
              "values": ["vpn-interface", "tor-like-process", "unknown-encrypted-relay"],
              "matchMode": "any-indicator"
            },
            "action": {
              "kind": "warn",
              "reasonCode": "tunnel-indicator"
            },
            "proofRequirement": "process-attributed-flow-or-adapter-indicator",
            "scheduleId": "always",
            "budgetId": null,
            "auditLevel": "decision-and-adapter"
          },
          {
            "ruleId": "limit-video-cdn-bandwidth",
            "enabled": true,
            "priority": 400,
            "target": {
              "kind": "destination-category",
              "values": ["video-entertainment"],
              "matchMode": "category"
            },
            "action": {
              "kind": "limit",
              "budgetId": "video-network-budget",
              "reasonCode": "entertainment-network-budget"
            },
            "proofRequirement": "domain-known-or-managed-browser-category",
            "scheduleId": "after-homework",
            "budgetId": "video-network-budget",
            "auditLevel": "decision-and-timer"
          },
          {
            "ruleId": "block-unapproved-doh",
            "enabled": false,
            "priority": 500,
            "target": {
              "kind": "domain",
              "values": ["doh-resolver.example.invalid"],
              "matchMode": "exact-or-subdomain"
            },
            "action": {
              "kind": "block",
              "reasonCode": "unapproved-encrypted-dns"
            },
            "proofRequirement": "adapter-enforcement-proof",
            "scheduleId": "always",
            "budgetId": null,
            "auditLevel": "decision-and-adapter"
          }
        ]
      },
      "tunnel": {
        "mode": "warn",
        "indicators": [
          "vpn-interface",
          "proxy-process",
          "known-tunnel-port",
          "tor-like-process",
          "unknown-encrypted-relay"
        ],
        "whenInsideDestinationUnknown": "show-tunneled-unknown"
      },
      "budgets": {
        "enabled": true,
        "countingMode": "flow-bytes-when-available",
        "warningThresholdPercent": 80,
        "graceMinutes": 5,
        "reset": "daily",
        "items": [
          {
            "budgetId": "daily-network-budget",
            "targetKind": "all-network",
            "limit": {
              "kind": "bytes",
              "megabytes": 2048
            },
            "period": "daily",
            "whenExhausted": "ask"
          },
          {
            "budgetId": "video-network-budget",
            "targetKind": "destination-category",
            "targetValues": ["video-entertainment"],
            "limit": {
              "kind": "minutes",
              "minutes": 30
            },
            "period": "daily",
            "whenExhausted": "warn"
          }
        ]
      },
      "localNetwork": {
        "exceptionMode": "explicit-services-only",
        "defaultExceptions": ["loopback-agent", "lan-parent-controller", "dns-resolver", "dhcp", "printer"],
        "items": [
          {
            "exceptionId": "allow-loopback-agent",
            "kind": "loopback-service",
            "target": "ocentra-agent-loopback",
            "ports": [4477],
            "protocols": ["tcp"],
            "auditLevel": "summary"
          },
          {
            "exceptionId": "allow-parent-controller-lan",
            "kind": "lan-service",
            "target": "parent-controller",
            "ports": [4477, 4478],
            "protocols": ["tcp"],
            "auditLevel": "decision"
          }
        ]
      },
      "reports": {
        "visibleFields": [
          "recent-flows",
          "top-processes",
          "top-domains",
          "top-ips",
          "ports-protocols",
          "bandwidth",
          "vpn-proxy-tunnel-indicators",
          "source-capability",
          "custody-label"
        ],
        "summaries": ["by-child", "by-device", "by-process", "by-domain", "by-ip", "by-category", "by-indicator"],
        "showUncertainty": true,
        "showExactUrlRequiresManagedBrowser": true
      },
      "retention": {
        "rawFlowEvidence": "7-days",
        "domainSummary": "30-days",
        "bandwidthSummary": "30-days",
        "policyAudit": "90-days",
        "deleteExpired": true,
        "keepRedactedReport": true
      },
      "custody": {
        "allowedUses": ["child-local", "lan-live", "parent-cache", "parent-export", "parent-report"],
        "defaultStorage": "child-local",
        "hostedStorageDefault": false,
        "requireCustodyLabelForPortal": true,
        "requireCustodyLabelForAi": true,
        "requireCustodyLabelForExport": true
      },
      "audit": {
        "requiredFields": [
          "policy-decision",
          "network-evidence-ref",
          "adapter-result",
          "budget-state",
          "parent-override",
          "rollback",
          "policy-version",
          "capability-state",
          "custody-label"
        ],
        "auditEveryStrictAction": true,
        "auditFailedAdapterActions": true,
        "auditPolicyPreview": true
      },
      "platforms": {
        "windows": {
          "enabled": true,
          "allowedAdapters": [
            "ip-helper-endpoint-snapshot",
            "dns-client-cache",
            "windows-firewall",
            "windows-filtering-platform",
            "process-observation",
            "managed-browser-domain-join"
          ],
          "manualRequiredAdapters": ["windows-filtering-platform-enforcement", "managed-resolver-enforcement"]
        },
        "macos": {
          "enabled": false,
          "state": "manual-required"
        },
        "linux": {
          "enabled": false,
          "state": "manual-required"
        },
        "android": {
          "enabled": false,
          "state": "manual-required"
        },
        "ios": {
          "enabled": false,
          "state": "manual-required"
        },
        "router": {
          "enabled": false,
          "state": "not-implemented"
        },
        "cloudRelay": {
          "authoringAndRoutingOnly": true,
          "mayStoreChildNetworkEvidenceByDefault": false,
          "mayEnforceArbitraryInternetTraffic": false
        },
        "webPortal": {
          "authoringOnly": true,
          "mayRunCapture": false,
          "mayRunEnforcement": false
        }
      },
      "fallbacks": {
        "adapterUnavailable": "observe",
        "dnsUnavailable": "ip-only",
        "domainAmbiguous": "ask-or-observe",
        "processUnknown": "observe",
        "encryptedDnsDetected": "warn",
        "quicOrEchReducesAttribution": "domain-or-ip-only",
        "cdnIpShared": "avoid-ip-block-unless-explicit",
        "bandwidthCountersUnavailable": "count-connections-only",
        "enforcementFailure": "rollback-and-audit",
        "childDeviceOffline": "last-known-report-only",
        "platformUnsupported": "show-unavailable"
      }
    }
  },
  "effectivePolicy": {
    "documentId": "network-effective-mia-windows-laptop",
    "compiledFromPolicyId": "network-policy-mia-windows-laptop",
    "compiledFromRevision": 12,
    "schemaVersion": 1,
    "effectivePolicyHash": "sha256:worker-must-compute-sample",
    "compiledAt": "2026-05-28T00:00:00.000Z",
    "scope": {
      "familyId": "family-local-1",
      "childId": "child-mia",
      "deviceId": "device-windows-laptop",
      "platform": "windows"
    },
    "networkActivityDefaultDecision": "observe",
    "networkManagementEnabled": true,
    "evidenceDecision": {
      "allowedMetadata": [
        "process",
        "ip",
        "port",
        "protocol",
        "domain-candidate",
        "dns-source-state",
        "connection-count",
        "duration",
        "bytes-when-available",
        "adapter-state"
      ],
      "neverCollect": [
        "decrypted-https-payload",
        "packet-payload",
        "page-body",
        "chat-content",
        "search-terms",
        "form-values",
        "cookies",
        "tokens",
        "credentials",
        "raw-packet-dumps",
        "raw-trace-files"
      ],
      "exactUrlRequiresManagedBrowserOrUrlFilter": true
    },
    "proofRequirements": {
      "domainRules": "domain-known-or-managed-browser-domain",
      "ipRules": "endpoint-observed",
      "processRules": "process-attributed-flow",
      "vpnProxyTunnelRules": "process-attributed-flow-or-adapter-indicator",
      "bandwidthBudgets": "flow-counters-when-available",
      "timeBudgets": "network-active-or-foreground-correlated-time",
      "strictEnforcement": "adapter-enforcement-proof",
      "reportOnly": "stale-or-degraded-allowed"
    },
    "fallbackDecisions": {
      "adapterUnavailable": "observe",
      "dnsUnavailable": "ip-only",
      "domainAmbiguous": "ask-or-observe",
      "processUnknown": "observe",
      "encryptedDnsDetected": "warn",
      "quicOrEchReducesAttribution": "domain-or-ip-only",
      "cdnIpShared": "avoid-ip-block-unless-explicit",
      "bandwidthCountersUnavailable": "count-connections-only",
      "enforcementFailure": "rollback-and-audit",
      "platformUnsupported": "unavailable"
    },
    "rulesInPriorityOrder": [
      {
        "ruleId": "allow-school-domains",
        "priority": 100,
        "decision": "allow",
        "targetKind": "domain",
        "proofRequirement": "domain-known-or-managed-browser-domain",
        "scheduleId": "school-hours"
      },
      {
        "ruleId": "parent-request-new-public-destination",
        "priority": 200,
        "decision": "ask",
        "targetKind": "new-destination",
        "proofRequirement": "stored-flow-summary",
        "scheduleId": "always"
      },
      {
        "ruleId": "warn-tunnel-indicator",
        "priority": 300,
        "decision": "warn",
        "targetKind": "vpn-proxy-tunnel",
        "proofRequirement": "process-attributed-flow-or-adapter-indicator",
        "scheduleId": "always"
      },
      {
        "ruleId": "limit-video-cdn-bandwidth",
        "priority": 400,
        "decision": "limit",
        "targetKind": "destination-category",
        "proofRequirement": "domain-known-or-managed-browser-category",
        "scheduleId": "after-homework",
        "budgetId": "video-network-budget"
      },
      {
        "ruleId": "block-unapproved-doh",
        "priority": 500,
        "decision": "block",
        "targetKind": "domain",
        "proofRequirement": "adapter-enforcement-proof",
        "scheduleId": "always",
        "enabled": false
      }
    ],
    "runtimeTables": {
      "schedulesById": {
        "always": {
          "kind": "always"
        },
        "school-hours": {
          "kind": "weekly-window",
          "timezone": "America/Toronto"
        },
        "after-homework": {
          "kind": "weekly-window",
          "timezone": "America/Toronto"
        }
      },
      "budgetsById": {
        "daily-network-budget": {
          "kind": "bytes",
          "megabytes": 2048,
          "period": "daily",
          "whenExhausted": "ask"
        },
        "video-network-budget": {
          "kind": "minutes",
          "minutes": 30,
          "period": "daily",
          "whenExhausted": "warn"
        }
      },
      "localExceptionsById": {
        "allow-loopback-agent": {
          "kind": "loopback-service",
          "ports": [4477],
          "protocols": ["tcp"]
        },
        "allow-parent-controller-lan": {
          "kind": "lan-service",
          "ports": [4477, 4478],
          "protocols": ["tcp"]
        }
      }
    },
    "auditPlan": {
      "auditEveryDecision": true,
      "auditEveryStrictAction": true,
      "requiredFields": [
        "policy-decision",
        "network-evidence-ref",
        "adapter-result",
        "budget-state",
        "parent-override",
        "rollback",
        "policy-version",
        "capability-state",
        "custody-label"
      ]
    }
  },
  "updateProtocol": {
    "commands": [
      {
        "commandType": "network-policy.get.requested",
        "purpose": "Portal asks the child agent for current policy value and revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "includeAuthoringManifest": true,
          "includeEffectivePolicy": true
        }
      },
      {
        "commandType": "network-policy.preview.requested",
        "purpose": "Portal asks whether proposed changes validate and what effective policy would result.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "baseRevision": 12,
          "patch": [
            {
              "op": "replace",
              "path": "/networkPolicy/defaultPosture",
              "value": "warn"
            }
          ]
        },
        "responseShape": {
          "accepted": true,
          "wouldCreateRevision": 13,
          "effectivePolicyPreviewHash": "sha256:static-sample-token",
          "warnings": [],
          "unsupportedSettings": []
        }
      },
      {
        "commandType": "network-policy.patch.requested",
        "purpose": "Portal sends a small settings change with an expected revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 12,
          "patch": [
            {
              "op": "replace",
              "path": "/networkPolicy/tunnel/mode",
              "value": "ask"
            }
          ],
          "reason": "parent-ui-change"
        },
        "acceptedResponseShape": {
          "eventType": "network-policy.patch.accepted",
          "newRevision": 13,
          "policyHash": "sha256:static-sample-token",
          "effectivePolicyHash": "sha256:static-sample-token",
          "requiresRestart": false,
          "unsupportedSettings": []
        },
        "rejectedResponseShape": {
          "eventType": "network-policy.patch.rejected",
          "currentRevision": 13,
          "reason": "revision-conflict",
          "validationErrors": []
        }
      },
      {
        "commandType": "network-policy.replace.requested",
        "purpose": "Portal sends a full policy replacement for setup, import, reset, or wizard save.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 12,
          "replacementPolicy": {
            "documentId": "network-policy-mia-windows-laptop",
            "policyKind": "network-control",
            "schemaVersion": 1,
            "revision": 13
          },
          "reason": "parent-wizard-save"
        }
      },
      {
        "commandType": "network-policy.rollback.requested",
        "purpose": "Parent asks child agent to roll back to previous valid revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "targetRevision": 12,
          "reason": "parent-rollback"
        }
      }
    ],
    "agentRules": {
      "validateFullPolicyAfterPatch": true,
      "compileFullEffectivePolicyAfterEveryAcceptedChange": true,
      "persistPolicyBeforeEnforcementSwitch": true,
      "keepPreviousValidRevision": true,
      "rollbackOnCompileFailure": true,
      "enforceLocallyWhenPortalOffline": true,
      "rejectUnknownPaths": true,
      "rejectInvalidEnumValues": true,
      "rejectExactUrlRuleWithoutManagedBrowserOrUrlFilter": true,
      "rejectStrictBlockWithoutAdapterProofOrManualRequiredState": true,
      "rejectBudgetWithoutCountingMode": true,
      "rejectBroadPrivateNetworkAllowWithoutAuditLevel": true
    }
  },
  "capabilityRegistry": {
    "deviceId": "device-windows-laptop",
    "generatedAt": "2026-05-28T00:00:00.000Z",
    "platform": "windows",
    "capabilities": [
      {
        "capabilityId": "ip-helper-endpoint-snapshot",
        "state": "ready",
        "proof": "runtime-adapter-proof-required",
        "affectsFields": ["evidence.metadataScope", "rules.allowedTargetTypes", "reports.visibleFields"]
      },
      {
        "capabilityId": "dns-client-cache",
        "state": "ready",
        "proof": "runtime-adapter-proof-required",
        "affectsFields": ["dns.allowedAttributionSources", "reports.visibleFields"]
      },
      {
        "capabilityId": "windows-firewall-rule-control",
        "state": "manual-required",
        "proof": "not-yet-proven",
        "affectsFields": ["rules.allowedActions", "localNetwork.exceptionMode"]
      },
      {
        "capabilityId": "windows-filtering-platform-observation",
        "state": "manual-required",
        "proof": "not-yet-proven",
        "affectsFields": ["evidence.metadataScope", "rules.allowedTargetTypes"]
      },
      {
        "capabilityId": "windows-filtering-platform-enforcement",
        "state": "manual-required",
        "proof": "not-yet-proven",
        "affectsFields": ["rules.allowedActions", "evidence.requiredProof"]
      },
      {
        "capabilityId": "managed-resolver-enforcement",
        "state": "manual-required",
        "proof": "not-yet-proven",
        "affectsFields": ["dns.mode", "dns.encryptedDnsHandling"]
      },
      {
        "capabilityId": "managed-browser-domain-join",
        "state": "ready-if-browser-capability-ready",
        "proof": "browser-evidence-proof-required",
        "affectsFields": ["evidence.requiredProof", "rules.allowedTargetTypes"]
      },
      {
        "capabilityId": "router-network-control",
        "state": "not-implemented",
        "proof": "not-yet-proven",
        "affectsFields": ["localNetwork.exceptionMode", "rules.allowedActions"]
      },
      {
        "capabilityId": "cloud-relay-network-authoring",
        "state": "authoring-only",
        "proof": "relay-does-not-enforce-child-internet-traffic",
        "affectsFields": ["network.managementMode", "custody.allowedUses"]
      }
    ]
  }
}
```

## Implementation Notes For Worker

- Start with domain contracts before Portal UI.
- Keep authoring manifest ids, field ids, section ids, option ids, policy ids,
  rule ids, schedule ids, budget ids, exception ids, and capability ids branded.
- Do not let Portal define arbitrary JSON paths. `writesTo` paths should be
  schema-known authoring paths.
- Use Effect Schema to validate the full policy after every patch.
- Compile the effective policy in the child-agent/service boundary, not in
  Portal.
- Persist both policy revision and compiled effective policy hash.
- Reject partial states. For example, `defaultPosture: "limit"` needs a valid
  budget or an explicit fallback decision.
- Treat the authoring manifest as UI guidance only. Runtime enforcement must
  rely on validated policy and compiled effective policy.
- Keep exact URL controls out of this schema unless the implementation also
  references managed browser or explicit URL-filter capability.
- Add explicit tests for hidden/visible branch behavior so UI cannot show strict
  network enforcement controls when only observation is available.
- Add explicit tests for unknown, ambiguous, stale, encrypted-content
  unavailable, DNS unavailable, process unknown, and adapter unavailable states.
- Add explicit tests for offline behavior: child agent continues enforcing the
  last valid compiled policy when Portal is disconnected, but only through
  proven adapters.
