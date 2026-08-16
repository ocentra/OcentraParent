<!-- agent-capsule -->

> Agent Capsule
> Doc: App Control Schema Proposal
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# App Control Schema Proposal

Status: worker handoff proposal. This is not runtime source code.

This document proposes a structured app control schema for native and installed
application controls. It can support:

- Portal-rendered app policy question/option UI.
- Parent-authored app settings.
- Child-agent local persisted policy.
- Offline enforcement from the last valid policy.
- Small patch updates from Portal.
- Full policy replacement during setup/import/reset.
- Deterministic compile into an effective app enforcement plan.
- Capability-gated UI for Windows, macOS, Linux, Android, and iOS.

The JSON in this document is intentionally product-shaped rather than
repo-strict. The implementation worker must not copy it directly into runtime
code. The worker should use it as a guide and then build proper Ocentra Parent
contracts with:

- Effect Schema validation.
- Branded ids from schema brands, not manual brands.
- Decode helpers.
- No naked domain strings in app/runtime code.
- Tests for every parser, authoring manifest field, policy value shape,
  compile rule, patch command, invalid-state rejection, and unsupported platform
  state.
- Rust protocol parity only after the TypeScript contracts are explicit and
  test-backed.
- Local child-agent persistence, compile, timer recovery, rollback, and audit
  behavior.

## Architecture

The proposal has four related documents plus a capability registry.

### Authoring Manifest

The authoring manifest tells Portal what questions to show, what controls to
render, which options are allowed, where the answer writes into the policy value
document, and when the field is visible or enabled.

Portal must not invent app policy questions outside this manifest. If the UI
needs a new question, the manifest and value schema need a contract update.

### Policy Value Document

The policy value document is the parent-authored app policy. It is the durable
source of parent intent. The child agent validates it as a whole after any
update.

### Effective Policy Document

The effective policy document is the compiled execution plan. The child agent
uses it for local evaluation, timers, app/session matching, and enforcement
handoff. It should be deterministic, flat enough for runtime, and explicit about
fallback behavior when proof is unavailable.

### Policy Update Commands

Portal sends typed update commands. The child agent validates, persists,
compiles, and acknowledges. Portal is never in the enforcement hot path.

```text
Portal authoring UI
  -> policy update command
  -> child agent validates full policy value
  -> child agent persists policy revision
  -> child agent compiles effective policy
  -> child agent enforces locally where capability proof exists
```

## Proposed Complete JSON Shape

The following JSON combines the proposed authoring manifest, policy value,
effective policy, update commands, and capability registry into one example so a
worker can see how the pieces relate.

```json
{
  "schemaVersion": 1,
  "proposalStatus": "worker-handoff-proposal-not-runtime-contract",
  "proposalIntent": "Guide the implementation of native app policy authoring, storage, compile, enforcement, capability, and audit contracts.",
  "workerInstruction": {
    "takeAsGuideOnly": true,
    "mustTranslateToEffectSchema": true,
    "mustUseSchemaBrands": true,
    "mustAddDecodeHelpers": true,
    "mustAddTests": true,
    "mustPreserveLocalChildAgentEnforcement": true,
    "mustAddRustParityOnlyAfterTypeScriptContracts": true,
    "mustNotCopyJsonDirectlyIntoRuntime": true
  },
  "contractFamilies": {
    "authoringManifest": "Portal-rendered sections, questions, options, visibility, enabled state, writesTo paths, and validation hints.",
    "policyValue": "Parent-authored durable native app policy state stored and versioned by the child agent.",
    "effectivePolicy": "Compiled deterministic child-agent execution plan for app evidence matching, timers, and enforcement.",
    "updateProtocol": "Typed get, preview, patch, replace, ack, reject, rollback, and capability-refresh commands.",
    "capabilityRegistry": "Runtime device and platform capability states used to hide, disable, degrade, or explain fields."
  },
  "authoringManifest": {
    "manifestId": "app-control-authoring-v1",
    "policyKind": "app-control",
    "schemaVersion": 1,
    "title": "App controls",
    "renderingRules": {
      "hideInvisibleFields": true,
      "showDisabledFieldsWithReason": true,
      "neverInventFieldsOutsideManifest": true,
      "writeOnlyThroughWritesToPath": true,
      "previewBeforeApply": true,
      "showCapabilityStateBesideStrictActions": true,
      "showDryRunLabels": true
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
        "sectionId": "app-management",
        "title": "App management",
        "purpose": "Top-level native app policy switch and default posture.",
        "fields": [
          {
            "fieldId": "app.enabled",
            "kind": "boolean",
            "question": "Enable app management?",
            "writesTo": "/appPolicy/enabled",
            "defaultValue": false,
            "uiPriority": 10,
            "whenFalse": {
              "policyMeaning": "Native app activity is allowed and app controls do not enforce.",
              "hiddenSections": [
                "inventory",
                "runtime-evidence",
                "app-rules",
                "budgets",
                "enforcement",
                "app-lifecycle",
                "approvals",
                "reports",
                "audit"
              ]
            }
          },
          {
            "fieldId": "app.defaultPosture",
            "kind": "single-choice",
            "question": "What should happen to app activity?",
            "writesTo": "/appPolicy/defaultPosture",
            "defaultValue": "observe",
            "visibleWhen": {
              "path": "/appPolicy/enabled",
              "equals": true
            },
            "options": [
              {
                "value": "allow",
                "label": "Allow",
                "meaning": "App activity is allowed unless a more specific rule changes it.",
                "relevantSections": ["exceptions", "reports", "audit"]
              },
              {
                "value": "observe",
                "label": "Observe",
                "meaning": "App activity is allowed, evidence is collected according to data scope, and decisions are report-only.",
                "relevantSections": ["inventory", "runtime-evidence", "reports", "retention", "audit"]
              },
              {
                "value": "warn",
                "label": "Warn",
                "meaning": "Matching app activity warns the child and records parent-visible events.",
                "relevantSections": ["child-facing", "app-rules", "reports", "audit"]
              },
              {
                "value": "ask",
                "label": "Ask",
                "meaning": "Matching app activity needs parent approval unless an allow rule or override applies.",
                "relevantSections": ["approvals", "overrides", "child-facing", "reports", "audit"]
              },
              {
                "value": "limit",
                "label": "Limit",
                "meaning": "App activity is allowed inside configured schedules and budgets.",
                "relevantSections": ["budgets", "schedules", "approvals", "reports", "audit"]
              },
              {
                "value": "block",
                "label": "Block",
                "meaning": "App activity is blocked by default when platform capability proof exists; otherwise it degrades according to fallback policy.",
                "relevantSections": ["enforcement", "exceptions", "approvals", "child-facing", "audit"]
              }
            ]
          },
          {
            "fieldId": "app.managementMode",
            "kind": "single-choice",
            "question": "How should app management run on this device?",
            "writesTo": "/appPolicy/managementMode",
            "defaultValue": "local-child-agent",
            "visibleWhen": {
              "path": "/appPolicy/enabled",
              "equals": true
            },
            "options": [
              {
                "value": "local-child-agent",
                "label": "Child device local"
              },
              {
                "value": "lan-live",
                "label": "LAN live"
              },
              {
                "value": "authoring-only",
                "label": "Authoring only"
              },
              {
                "value": "unavailable",
                "label": "Unavailable"
              }
            ]
          }
        ]
      },
      {
        "sectionId": "inventory",
        "title": "Installed apps",
        "purpose": "Choose which inventory sources can support app matching and parent reports.",
        "visibleWhen": {
          "path": "/appPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "inventory.mode",
            "kind": "single-choice",
            "question": "How should installed app inventory be used?",
            "writesTo": "/appPolicy/inventory/mode",
            "defaultValue": "use-for-matching-and-reports",
            "options": ["disabled", "reports-only", "use-for-matching-and-reports", "required-for-strict-rules"]
          },
          {
            "fieldId": "inventory.sources",
            "kind": "multi-choice",
            "question": "Which app inventory sources are allowed?",
            "writesTo": "/appPolicy/inventory/sources",
            "defaultValue": [
              "os-installed-apps",
              "desktop-shortcuts",
              "store-packages",
              "executable-metadata",
              "parent-catalog"
            ],
            "options": [
              "os-installed-apps",
              "desktop-shortcuts",
              "store-packages",
              "package-manager",
              "managed-device-apps",
              "screen-time-tokens",
              "executable-metadata",
              "parent-catalog"
            ]
          },
          {
            "fieldId": "inventory.identityFields",
            "kind": "multi-choice",
            "question": "Which identity fields may app rules use?",
            "writesTo": "/appPolicy/inventory/identityFields",
            "defaultValue": [
              "package-id",
              "bundle-id",
              "app-user-model-id",
              "executable-path",
              "publisher-signature",
              "file-hash"
            ],
            "options": [
              "package-id",
              "bundle-id",
              "app-user-model-id",
              "desktop-entry-id",
              "application-token",
              "executable-path",
              "publisher-signature",
              "file-hash",
              "display-name",
              "parent-label"
            ]
          },
          {
            "fieldId": "inventory.unknownHandling",
            "kind": "single-choice",
            "question": "What should happen when an app cannot be identified?",
            "writesTo": "/appPolicy/inventory/unknownHandling",
            "defaultValue": "observe",
            "options": ["allow", "observe", "warn", "ask", "count-under-unknown-budget", "block-if-supported"]
          }
        ]
      },
      {
        "sectionId": "runtime-evidence",
        "title": "Runtime evidence",
        "purpose": "Choose what running and foreground app evidence can be collected and used.",
        "visibleWhen": {
          "path": "/appPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "evidence.runtimeSources",
            "kind": "multi-choice",
            "question": "Which runtime evidence sources may be used?",
            "writesTo": "/appPolicy/evidence/runtimeSources",
            "defaultValue": ["process-snapshot", "process-start-exit", "foreground-window", "app-session-summary"],
            "options": [
              "process-snapshot",
              "process-start-exit",
              "foreground-window",
              "usage-stats",
              "device-activity",
              "managed-device-state",
              "accessibility-approved-state",
              "app-session-summary"
            ]
          },
          {
            "fieldId": "evidence.requiredProof",
            "kind": "single-choice",
            "question": "What proof is enough for app rules?",
            "writesTo": "/appPolicy/evidence/requiredProof",
            "defaultValue": "fresh-app-session",
            "options": [
              "inventory-only",
              "process-running",
              "foreground-window",
              "fresh-app-session",
              "platform-usage-event",
              "managed-device-state"
            ]
          },
          {
            "fieldId": "evidence.durationMode",
            "kind": "single-choice",
            "question": "Which duration should time budgets count?",
            "writesTo": "/appPolicy/evidence/durationMode",
            "defaultValue": "foreground-time",
            "options": ["running-time", "foreground-time", "platform-usage-time", "manual-review-only"]
          },
          {
            "fieldId": "evidence.whenProofUnavailable",
            "kind": "single-choice",
            "question": "What if app proof is unavailable?",
            "writesTo": "/appPolicy/evidence/whenProofUnavailable",
            "defaultValue": "mark-unavailable",
            "options": ["allow", "observe", "warn", "ask", "block-until-ready", "mark-unavailable"]
          },
          {
            "fieldId": "evidence.neverCollect",
            "kind": "multi-choice",
            "question": "What must app controls never collect?",
            "writesTo": "/appPolicy/evidence/neverCollect",
            "defaultValue": [
              "screen-contents",
              "screenshots",
              "keystrokes",
              "chat-content",
              "voice-content",
              "app-internal-documents",
              "launcher-credentials",
              "decrypted-network-payload"
            ],
            "options": [
              "screen-contents",
              "screenshots",
              "keystrokes",
              "chat-content",
              "voice-content",
              "app-internal-documents",
              "launcher-credentials",
              "decrypted-network-payload",
              "raw-command-line-with-secrets"
            ]
          }
        ]
      },
      {
        "sectionId": "app-rules",
        "title": "App rules",
        "purpose": "Rules for native apps, app categories, unknown apps, managed apps, and app sessions.",
        "visibleWhen": {
          "all": [
            {
              "path": "/appPolicy/enabled",
              "equals": true
            },
            {
              "path": "/appPolicy/defaultPosture",
              "notEquals": "allow"
            }
          ]
        },
        "fields": [
          {
            "fieldId": "rules.allowedTargetTypes",
            "kind": "multi-choice",
            "question": "What app targets should rules match?",
            "writesTo": "/appPolicy/rules/allowedTargetTypes",
            "defaultValue": [
              "app-ref",
              "app-category",
              "executable-identity",
              "unknown-app",
              "app-session",
              "capability-state"
            ],
            "options": [
              "app-ref",
              "app-category",
              "package-id",
              "bundle-id",
              "application-token",
              "executable-identity",
              "publisher-signature",
              "unknown-app",
              "managed-app-state",
              "app-session",
              "capability-state"
            ]
          },
          {
            "fieldId": "rules.defaultUnknownRule",
            "kind": "single-choice",
            "question": "Default rule for unknown apps?",
            "writesTo": "/appPolicy/rules/defaultUnknownRule",
            "defaultValue": "ask-first-run",
            "options": ["allow", "observe", "warn", "ask-first-run", "limit", "block-if-supported"]
          },
          {
            "fieldId": "rules.matchConfidenceRequired",
            "kind": "single-choice",
            "question": "How strong must an app match be before strict action?",
            "writesTo": "/appPolicy/rules/matchConfidenceRequired",
            "defaultValue": "deterministic-or-parent-approved",
            "options": [
              "any-candidate",
              "catalog-confidence-high",
              "deterministic-or-parent-approved",
              "managed-device-proof"
            ]
          }
        ]
      },
      {
        "sectionId": "budgets",
        "title": "App time limits",
        "purpose": "Configure app and category budgets based on running or foreground session evidence.",
        "visibleWhen": {
          "all": [
            {
              "path": "/appPolicy/enabled",
              "equals": true
            },
            {
              "path": "/appPolicy/defaultPosture",
              "includes": ["limit", "ask", "block", "warn", "observe"]
            }
          ]
        },
        "fields": [
          {
            "fieldId": "budgets.enabled",
            "kind": "boolean",
            "question": "Enable app time budgets?",
            "writesTo": "/appPolicy/budgets/enabled",
            "defaultValue": true
          },
          {
            "fieldId": "budgets.defaultDailyMinutes",
            "kind": "number",
            "question": "Default daily app time limit in minutes?",
            "writesTo": "/appPolicy/budgets/defaultDailyMinutes",
            "defaultValue": 60,
            "min": 0,
            "max": 1440,
            "visibleWhen": {
              "path": "/appPolicy/budgets/enabled",
              "equals": true
            }
          },
          {
            "fieldId": "budgets.whenExhausted",
            "kind": "single-choice",
            "question": "What happens when app time runs out?",
            "writesTo": "/appPolicy/budgets/whenExhausted",
            "defaultValue": "ask",
            "options": [
              "observe",
              "warn",
              "ask",
              "terminate-if-supported",
              "shield-if-supported",
              "block-if-supported"
            ],
            "visibleWhen": {
              "path": "/appPolicy/budgets/enabled",
              "equals": true
            }
          }
        ]
      },
      {
        "sectionId": "enforcement",
        "title": "App enforcement",
        "purpose": "Configure strict app actions that require platform capability proof.",
        "visibleWhen": {
          "path": "/appPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "enforcement.allowedActions",
            "kind": "multi-choice",
            "question": "Which app enforcement actions may run?",
            "writesTo": "/appPolicy/enforcement/allowedActions",
            "defaultValue": ["warn", "ask-parent", "owned-process-terminate", "time-limit"],
            "options": [
              "warn",
              "ask-parent",
              "owned-process-terminate",
              "target-process-terminate",
              "block-launch",
              "shield-app",
              "suspend-package",
              "hide-package",
              "time-limit",
              "managed-install",
              "managed-uninstall"
            ]
          },
          {
            "fieldId": "enforcement.strictActionFallback",
            "kind": "single-choice",
            "question": "What if a strict app action is unsupported?",
            "writesTo": "/appPolicy/enforcement/strictActionFallback",
            "defaultValue": "parent-request-report-unavailable",
            "options": [
              "allow-and-report-unavailable",
              "observe-and-report-unavailable",
              "warn-and-report-unavailable",
              "parent-request-report-unavailable",
              "block-until-ready"
            ]
          },
          {
            "fieldId": "enforcement.graceSeconds",
            "kind": "number",
            "question": "How long should the child get before strict action applies?",
            "writesTo": "/appPolicy/enforcement/graceSeconds",
            "defaultValue": 15,
            "min": 0,
            "max": 900
          },
          {
            "fieldId": "enforcement.requireRollbackPlan",
            "kind": "boolean",
            "question": "Require rollback state for strict actions?",
            "writesTo": "/appPolicy/enforcement/requireRollbackPlan",
            "defaultValue": true
          }
        ]
      },
      {
        "sectionId": "app-lifecycle",
        "title": "Managed app lifecycle",
        "purpose": "Configure install, uninstall, hide, suspend, and managed app state only when platform setup supports it.",
        "visibleWhen": {
          "path": "/appPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "lifecycle.mode",
            "kind": "single-choice",
            "question": "How should app install and uninstall controls be handled?",
            "writesTo": "/appPolicy/lifecycle/mode",
            "defaultValue": "disabled",
            "options": ["disabled", "report-managed-state", "managed-apps-only", "device-owner-or-mdm-only"]
          },
          {
            "fieldId": "lifecycle.allowedOperations",
            "kind": "multi-choice",
            "question": "Which managed app lifecycle operations are allowed?",
            "writesTo": "/appPolicy/lifecycle/allowedOperations",
            "defaultValue": [],
            "options": [
              "install-managed-app",
              "uninstall-managed-app",
              "hide-managed-app",
              "suspend-managed-app",
              "remove-user-installed-app-if-platform-approved",
              "prevent-uninstall-if-platform-approved"
            ],
            "visibleWhen": {
              "path": "/appPolicy/lifecycle/mode",
              "notEquals": "disabled"
            }
          }
        ]
      },
      {
        "sectionId": "approvals",
        "title": "Approvals",
        "purpose": "Configure parent approval and child request behavior.",
        "visibleWhen": {
          "path": "/appPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "approvals.requiredFor",
            "kind": "multi-choice",
            "question": "Which app events require parent approval?",
            "writesTo": "/appPolicy/approvals/requiredFor",
            "defaultValue": ["unknown-app", "time-extension", "strict-action-unavailable"],
            "options": [
              "unknown-app",
              "new-app",
              "blocked-app",
              "time-extension",
              "managed-install",
              "managed-uninstall",
              "strict-action-unavailable",
              "category-override"
            ]
          },
          {
            "fieldId": "approvals.unansweredDefault",
            "kind": "single-choice",
            "question": "What happens if the parent does not answer?",
            "writesTo": "/appPolicy/approvals/unansweredDefault",
            "defaultValue": "deny",
            "options": ["allow", "deny", "keep-pending", "use-rule-fallback"]
          }
        ]
      },
      {
        "sectionId": "reports",
        "title": "Reports",
        "purpose": "Choose parent-visible app reports and data scope.",
        "visibleWhen": {
          "path": "/appPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "reports.visibleFields",
            "kind": "multi-choice",
            "question": "Which app report fields should be visible?",
            "writesTo": "/appPolicy/reports/visibleFields",
            "defaultValue": [
              "installed-apps",
              "running-now",
              "foreground-now",
              "session-rollups",
              "policy-decisions",
              "enforcement-results",
              "source-capability"
            ],
            "options": [
              "installed-apps",
              "running-now",
              "foreground-now",
              "session-rollups",
              "unknown-apps",
              "category-rollups",
              "time-budget",
              "policy-decisions",
              "enforcement-results",
              "approval-events",
              "managed-lifecycle-events",
              "source-capability"
            ]
          },
          {
            "fieldId": "retention.rawObservation",
            "kind": "retention",
            "question": "How long should raw app observations be kept?",
            "writesTo": "/appPolicy/retention/rawObservation",
            "defaultValue": "7-days"
          },
          {
            "fieldId": "retention.rollups",
            "kind": "retention",
            "question": "How long should app rollups be kept?",
            "writesTo": "/appPolicy/retention/rollups",
            "defaultValue": "30-days"
          }
        ]
      }
    ]
  },
  "policyValue": {
    "documentId": "app-policy-mia-windows-laptop",
    "policyKind": "app-control",
    "schemaVersion": 1,
    "revision": 12,
    "updatedAt": "2026-05-28T00:00:00.000Z",
    "scope": {
      "familyId": "family-local-1",
      "childId": "child-mia",
      "deviceId": "device-windows-laptop",
      "platform": "windows"
    },
    "appPolicy": {
      "enabled": true,
      "defaultPosture": "limit",
      "managementMode": "local-child-agent",
      "inventory": {
        "mode": "use-for-matching-and-reports",
        "sources": [
          "os-installed-apps",
          "desktop-shortcuts",
          "store-packages",
          "executable-metadata",
          "parent-catalog"
        ],
        "identityFields": [
          "package-id",
          "app-user-model-id",
          "executable-path",
          "publisher-signature",
          "file-hash",
          "parent-label"
        ],
        "unknownHandling": "ask"
      },
      "evidence": {
        "runtimeSources": ["process-snapshot", "process-start-exit", "foreground-window", "app-session-summary"],
        "requiredProof": "fresh-app-session",
        "durationMode": "foreground-time",
        "whenProofUnavailable": "mark-unavailable",
        "neverCollect": [
          "screen-contents",
          "screenshots",
          "keystrokes",
          "chat-content",
          "voice-content",
          "app-internal-documents",
          "launcher-credentials",
          "decrypted-network-payload"
        ]
      },
      "rules": {
        "allowedTargetTypes": [
          "app-ref",
          "app-category",
          "executable-identity",
          "unknown-app",
          "app-session",
          "capability-state"
        ],
        "defaultUnknownRule": "ask-first-run",
        "matchConfidenceRequired": "deterministic-or-parent-approved",
        "items": [
          {
            "ruleId": "allow-school-apps",
            "enabled": true,
            "priority": 100,
            "target": {
              "kind": "app-category",
              "values": ["education", "productivity"],
              "matchMode": "any"
            },
            "action": {
              "kind": "allow",
              "reasonCode": "school-work"
            },
            "proofRequirement": "fresh-app-session",
            "scheduleId": "school-hours",
            "auditLevel": "decision"
          },
          {
            "ruleId": "limit-entertainment-apps",
            "enabled": true,
            "priority": 200,
            "target": {
              "kind": "app-category",
              "values": ["entertainment", "social"],
              "matchMode": "any"
            },
            "action": {
              "kind": "limit",
              "budgetId": "entertainment-evening-budget",
              "reasonCode": "entertainment-budget"
            },
            "proofRequirement": "foreground-window",
            "scheduleId": "after-homework",
            "auditLevel": "decision-and-timer"
          },
          {
            "ruleId": "ask-unknown-app",
            "enabled": true,
            "priority": 300,
            "target": {
              "kind": "unknown-app",
              "values": ["unknown-process", "unmatched-package", "permission-limited"],
              "matchMode": "any"
            },
            "action": {
              "kind": "ask",
              "approvalKind": "unknown-app",
              "reasonCode": "unknown-app"
            },
            "proofRequirement": "process-running",
            "scheduleId": "always",
            "auditLevel": "decision"
          },
          {
            "ruleId": "block-adult-vpn-apps",
            "enabled": true,
            "priority": 400,
            "target": {
              "kind": "app-ref",
              "values": ["app-ref-parent-curated-vpn-bypass-list"],
              "matchMode": "parent-curated-list"
            },
            "action": {
              "kind": "block",
              "reasonCode": "bypass-tool"
            },
            "proofRequirement": "deterministic-app-identity",
            "scheduleId": "always",
            "auditLevel": "decision-and-adapter"
          }
        ]
      },
      "schedules": [
        {
          "scheduleId": "always",
          "kind": "always"
        },
        {
          "scheduleId": "school-hours",
          "kind": "weekly-window",
          "timezone": "America/Toronto",
          "windows": [
            {
              "days": ["monday", "tuesday", "wednesday", "thursday", "friday"],
              "start": "08:00",
              "end": "15:30"
            }
          ]
        },
        {
          "scheduleId": "after-homework",
          "kind": "weekly-window",
          "timezone": "America/Toronto",
          "windows": [
            {
              "days": ["monday", "tuesday", "wednesday", "thursday", "friday"],
              "start": "17:00",
              "end": "20:00"
            }
          ]
        }
      ],
      "budgets": {
        "enabled": true,
        "defaultDailyMinutes": 60,
        "durationMode": "foreground-time",
        "warningThresholdMinutes": 10,
        "graceMinutes": 5,
        "reset": "daily",
        "whenExhausted": "ask",
        "items": [
          {
            "budgetId": "app-daily-budget",
            "targetKind": "app-session",
            "minutes": 90,
            "period": "daily",
            "whenExhausted": "ask"
          },
          {
            "budgetId": "entertainment-evening-budget",
            "targetKind": "app-category",
            "targetValues": ["entertainment", "social"],
            "minutes": 45,
            "period": "daily",
            "whenExhausted": "terminate-if-supported"
          }
        ]
      },
      "enforcement": {
        "allowedActions": ["warn", "ask-parent", "owned-process-terminate", "target-process-terminate", "time-limit"],
        "strictActionFallback": "parent-request-report-unavailable",
        "graceSeconds": 15,
        "requireRollbackPlan": true,
        "dryRunDefault": false
      },
      "lifecycle": {
        "mode": "report-managed-state",
        "allowedOperations": [],
        "personalAppRemovalAllowed": false
      },
      "approvals": {
        "requiredFor": ["unknown-app", "time-extension", "strict-action-unavailable"],
        "unansweredDefault": "deny",
        "expiresAfterMinutes": 15,
        "allowChildNote": true,
        "allowedParentResponses": [
          "approve-once",
          "approve-session",
          "approve-until-time",
          "deny",
          "extend-time",
          "cancel"
        ]
      },
      "childFacing": {
        "showWarnText": true,
        "showBlockReason": true,
        "showAskParentState": true,
        "showTimeLeft": true,
        "showAlternativeAllowedApps": true,
        "hideParentDiagnostics": true
      },
      "reports": {
        "visibleFields": [
          "installed-apps",
          "running-now",
          "foreground-now",
          "session-rollups",
          "unknown-apps",
          "time-budget",
          "policy-decisions",
          "enforcement-results",
          "source-capability"
        ],
        "summaries": ["by-child", "by-device", "by-app", "by-category", "by-unknown-state", "by-policy-action"],
        "showExactExecutableRequiresReveal": true
      },
      "retention": {
        "rawObservation": "7-days",
        "sessionRollups": "30-days",
        "policyAudit": "90-days",
        "enforcementAudit": "90-days",
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
          "evidence-ref",
          "adapter-result",
          "timer-state",
          "parent-override",
          "rollback",
          "policy-version",
          "capability-state",
          "custody-label"
        ],
        "auditEveryDecision": true,
        "auditEveryStrictAction": true,
        "auditFailedAdapterActions": true,
        "auditPolicyPreview": true
      },
      "fallbacks": {
        "inventoryUnavailable": "observe",
        "foregroundUnavailable": "running-time-only",
        "unknownApp": "ask",
        "staleEvidence": "report-only",
        "terminateUnavailable": "parent-request-report-unavailable",
        "blockLaunchUnavailable": "parent-request-report-unavailable",
        "shieldUnavailable": "parent-request-report-unavailable",
        "managedDeviceSetupMissing": "show-unavailable",
        "adapterError": "rollback-and-audit",
        "childDeviceOffline": "last-valid-policy-local-only",
        "platformUnsupported": "show-unavailable"
      }
    }
  },
  "effectivePolicy": {
    "documentId": "app-effective-mia-windows-laptop",
    "compiledFromPolicyId": "app-policy-mia-windows-laptop",
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
    "appManagementEnabled": true,
    "appActivityDefaultDecision": "limit",
    "identityStrategy": {
      "inventorySources": [
        "os-installed-apps",
        "desktop-shortcuts",
        "store-packages",
        "executable-metadata",
        "parent-catalog"
      ],
      "strictActionMatchRequirement": "deterministic-or-parent-approved",
      "allowedIdentityFields": [
        "package-id",
        "app-user-model-id",
        "executable-path",
        "publisher-signature",
        "file-hash",
        "parent-label"
      ]
    },
    "sessionStrategy": {
      "requiredProofForRules": "fresh-app-session",
      "durationMode": "foreground-time",
      "staleAfterSeconds": 120,
      "closeSessionAfterGapSeconds": 300,
      "countPortalRefreshAsActivity": false
    },
    "unknownAppDecision": {
      "defaultAction": "ask",
      "countUnderBudget": true,
      "auditRequired": true
    },
    "proofRequirements": {
      "inventoryRules": "inventory-source-ref",
      "runningRules": "process-running",
      "foregroundRules": "foreground-window",
      "timeBudgets": "fresh-app-session",
      "strictAppBlock": "deterministic-app-identity-and-capability-ready",
      "managedLifecycle": "managed-device-state"
    },
    "fallbackDecisions": {
      "proofUnavailable": "mark-unavailable",
      "staleEvidence": "report-only",
      "terminateUnavailable": "parent-request-report-unavailable",
      "blockLaunchUnavailable": "parent-request-report-unavailable",
      "platformUnsupported": "unavailable",
      "adapterError": "rollback-and-audit"
    },
    "rulesInPriorityOrder": [
      {
        "ruleId": "allow-school-apps",
        "priority": 100,
        "decision": "allow",
        "targetKind": "app-category",
        "proofRequirement": "fresh-app-session",
        "scheduleId": "school-hours",
        "budgetId": null
      },
      {
        "ruleId": "limit-entertainment-apps",
        "priority": 200,
        "decision": "limit",
        "targetKind": "app-category",
        "proofRequirement": "foreground-window",
        "scheduleId": "after-homework",
        "budgetId": "entertainment-evening-budget"
      },
      {
        "ruleId": "ask-unknown-app",
        "priority": 300,
        "decision": "ask",
        "targetKind": "unknown-app",
        "proofRequirement": "process-running",
        "scheduleId": "always",
        "budgetId": null
      },
      {
        "ruleId": "block-adult-vpn-apps",
        "priority": 400,
        "decision": "block",
        "targetKind": "app-ref",
        "proofRequirement": "deterministic-app-identity",
        "scheduleId": "always",
        "budgetId": null
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
        "app-daily-budget": {
          "minutes": 90,
          "period": "daily",
          "durationMode": "foreground-time",
          "whenExhausted": "ask"
        },
        "entertainment-evening-budget": {
          "minutes": 45,
          "period": "daily",
          "durationMode": "foreground-time",
          "whenExhausted": "terminate-if-supported"
        }
      }
    },
    "enforcementPlan": {
      "allowedActions": ["warn", "ask-parent", "owned-process-terminate", "target-process-terminate", "time-limit"],
      "strictActionFallback": "parent-request-report-unavailable",
      "graceSeconds": 15,
      "requiresPolicyDecisionBeforeAdapter": true,
      "requiresCapabilityReadyForStrictAction": true,
      "requiresRollbackPlan": true,
      "restoreTimersAfterRestart": true
    },
    "auditPlan": {
      "auditEveryDecision": true,
      "auditEveryStrictAction": true,
      "requiredFields": [
        "policy-decision",
        "evidence-ref",
        "adapter-result",
        "timer-state",
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
        "commandType": "app-policy.get.requested",
        "purpose": "Portal asks the child agent for current app policy value, revision, effective policy, and capability state.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "includeAuthoringManifest": true,
          "includeEffectivePolicy": true,
          "includeCapabilityRegistry": true
        }
      },
      {
        "commandType": "app-policy.preview.requested",
        "purpose": "Portal asks whether proposed changes validate and what effective app policy would result.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "baseRevision": 12,
          "patch": [
            {
              "op": "replace",
              "path": "/appPolicy/defaultPosture",
              "value": "block"
            }
          ]
        },
        "responseShape": {
          "accepted": true,
          "wouldCreateRevision": 13,
          "effectivePolicyPreviewHash": "sha256:static-sample-token",
          "warnings": [
            {
              "warningId": "broad-app-blocking-manual-required",
              "messageToken": "strict-action-needs-platform-proof",
              "affectedPath": "/appPolicy/defaultPosture"
            }
          ],
          "unsupportedSettings": []
        }
      },
      {
        "commandType": "app-policy.patch.requested",
        "purpose": "Portal sends a small app settings change with an expected revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 12,
          "patch": [
            {
              "op": "replace",
              "path": "/appPolicy/budgets/defaultDailyMinutes",
              "value": 45
            }
          ],
          "reason": "parent-ui-change"
        },
        "acceptedResponseShape": {
          "eventType": "app-policy.patch.accepted",
          "newRevision": 13,
          "policyHash": "sha256:static-sample-token",
          "effectivePolicyHash": "sha256:static-sample-token",
          "requiresRestart": false,
          "unsupportedSettings": []
        },
        "rejectedResponseShape": {
          "eventType": "app-policy.patch.rejected",
          "currentRevision": 13,
          "reason": "revision-conflict",
          "validationErrors": []
        }
      },
      {
        "commandType": "app-policy.replace.requested",
        "purpose": "Portal sends a full app policy replacement for setup, import, reset, or wizard save.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 12,
          "replacementPolicy": {
            "documentId": "app-policy-mia-windows-laptop",
            "policyKind": "app-control",
            "schemaVersion": 1,
            "revision": 13
          },
          "reason": "parent-wizard-save"
        }
      },
      {
        "commandType": "app-policy.rollback.requested",
        "purpose": "Parent asks child agent to roll back to previous valid app policy revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "targetRevision": 12,
          "reason": "parent-rollback"
        }
      },
      {
        "commandType": "app-policy.capability-refresh.requested",
        "purpose": "Portal asks the child agent to refresh app capability state before showing strict controls.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "capabilityIds": [
            "windows-process-observation",
            "windows-foreground-window",
            "windows-owned-process-terminate",
            "windows-broad-app-blocking"
          ]
        },
        "responseShape": {
          "eventType": "app-policy.capability-refresh.completed",
          "generatedAt": "2026-05-28T00:00:00.000Z",
          "changedCapabilityIds": ["windows-owned-process-terminate"]
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
      "rejectPartialLimitWithoutBudget": true,
      "rejectStrictActionWithoutCapabilityOrFallback": true,
      "rejectManagedLifecycleWithoutManagedDeviceProof": true,
      "rejectForegroundBudgetWithoutForegroundOrUsageProof": true,
      "neverLetPortalExecuteEnforcement": true
    }
  },
  "capabilityRegistry": {
    "deviceId": "device-windows-laptop",
    "generatedAt": "2026-05-28T00:00:00.000Z",
    "platform": "windows",
    "capabilities": [
      {
        "capabilityId": "windows-app-inventory",
        "state": "ready",
        "proof": "runtime-read-model-required",
        "source": "os-installed-apps-and-package-query",
        "affectsFields": ["inventory.mode", "inventory.sources", "reports.visibleFields"]
      },
      {
        "capabilityId": "windows-process-observation",
        "state": "ready",
        "proof": "runtime-adapter-proof-required",
        "source": "process-snapshot-and-process-events",
        "affectsFields": ["evidence.runtimeSources", "rules.allowedTargetTypes"]
      },
      {
        "capabilityId": "windows-foreground-window",
        "state": "ready",
        "proof": "runtime-adapter-proof-required",
        "source": "foreground-window-observation",
        "affectsFields": ["evidence.durationMode", "budgets.enabled", "budgets.defaultDailyMinutes"]
      },
      {
        "capabilityId": "windows-owned-process-terminate",
        "state": "ready",
        "proof": "service-proof-required-before-product-claim",
        "source": "owned-process-termination-adapter",
        "affectsFields": ["enforcement.allowedActions", "budgets.whenExhausted"]
      },
      {
        "capabilityId": "windows-target-process-terminate",
        "state": "manual-required",
        "proof": "real-host-adapter-proof-required",
        "source": "target-process-termination-adapter",
        "affectsFields": ["enforcement.allowedActions", "rules.defaultUnknownRule"]
      },
      {
        "capabilityId": "windows-broad-app-blocking",
        "state": "manual-required",
        "proof": "applocker-wdac-or-equivalent-proof-required",
        "source": "application-control-policy",
        "affectsFields": ["app.defaultPosture", "enforcement.allowedActions"]
      },
      {
        "capabilityId": "android-package-lifecycle",
        "state": "manual-required",
        "proof": "device-owner-profile-owner-or-mdm-proof-required",
        "source": "device-policy-manager",
        "affectsFields": ["lifecycle.mode", "lifecycle.allowedOperations", "enforcement.allowedActions"]
      },
      {
        "capabilityId": "ios-screen-time-shielding",
        "state": "manual-required",
        "proof": "family-controls-managed-settings-device-activity-entitlement-proof-required",
        "source": "screen-time-frameworks",
        "affectsFields": ["enforcement.allowedActions", "budgets.whenExhausted", "rules.allowedTargetTypes"]
      },
      {
        "capabilityId": "macos-managed-app-control",
        "state": "manual-required",
        "proof": "mdm-system-extension-or-approved-api-proof-required",
        "source": "macos-managed-device-boundary",
        "affectsFields": ["lifecycle.mode", "enforcement.allowedActions"]
      },
      {
        "capabilityId": "linux-desktop-app-control",
        "state": "manual-required",
        "proof": "target-distro-desktop-adapter-proof-required",
        "source": "desktop-entry-package-process-policy",
        "affectsFields": ["inventory.sources", "enforcement.allowedActions"]
      }
    ]
  }
}
```

## Implementation Notes For Worker

- Start with domain contracts before Portal UI.
- Keep authoring manifest ids, field ids, section ids, option ids, policy ids,
  app ids, rule ids, schedule ids, budget ids, approval ids, and capability ids
  branded.
- Do not let Portal define arbitrary JSON paths. `writesTo` paths should be
  schema-known authoring paths.
- Use Effect Schema to validate the full policy after every patch.
- Compile the effective policy in the child-agent/service boundary, not in
  Portal.
- Persist both policy revision and compiled effective policy hash.
- Reject partial states. For example, `defaultPosture: "limit"` needs either a
  valid budget or a fallback decision.
- Treat the authoring manifest as UI guidance only. Runtime enforcement must
  rely on validated policy and compiled effective policy.
- Keep inventory, runtime evidence, duration, category, and enforcement proof as
  separate fields. An installed app is not automatically a running app, and a
  running app is not automatically controllable.
- Add explicit tests for hidden/visible branch behavior so UI cannot show
  install/uninstall controls when managed-device lifecycle proof is missing.
- Add explicit tests for unsupported platform behavior: Android device-owner
  missing, iOS entitlement missing, macOS MDM missing, Linux desktop proof
  missing, and Windows broad app blocking manual-required.
- Add explicit tests for offline behavior: child agent continues evaluating the
  last valid compiled policy and restores timers when Portal is disconnected.
- Add audit tests for strict actions: warning, ask-parent, timer expiry,
  terminate attempt, unsupported strict action fallback, rollback, and parent
  override.
