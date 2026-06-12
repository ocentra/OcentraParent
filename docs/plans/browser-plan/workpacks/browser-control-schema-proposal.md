<!-- agent-capsule -->

> Agent Capsule
> Doc: Browser Control Schema Proposal
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Browser Control Schema Proposal

Status: proposal for worker handoff. This is not final source code.

This document proposes a structured browser control schema that can support:

- Portal-rendered question/option UI.
- Parent-authored browser settings.
- Child-agent local persisted policy.
- Offline enforcement from the last valid policy.
- Small patch updates from Portal.
- Full policy replacement during setup/import/reset.
- Deterministic compile into an effective enforcement plan.

The JSON in this document is intentionally product-shaped rather than
repo-strict. The implementation worker must not copy it directly into runtime
code. The worker should use it as a guide and then build proper Ocentra Parent
contracts with:

- Effect Schema validation.
- Branded ids from schema brands, not manual brands.
- Decode helpers.
- No naked domain strings in app/runtime code.
- Tests for every parser, authoring manifest field, policy value shape,
  compile rule, patch command, and invalid-state rejection.
- Rust protocol parity only after the TypeScript contracts are explicit and
  test-backed.
- Local child-agent persistence, compile, rollback, and audit behavior.

## Architecture

The proposal has four related documents.

### Authoring Manifest

The authoring manifest tells Portal what questions to show, what controls to
render, which options are allowed, where the answer writes into the policy value
document, and when the field is visible or enabled.

Portal must not invent browser policy questions outside this manifest. If the UI
needs a new question, the manifest and value schema need a contract update.

### Policy Value Document

The policy value document is the parent-authored browser policy. It is the
durable source of parent intent. The child agent validates it as a whole after
any update.

### Effective Policy Document

The effective policy document is the compiled execution plan. The child agent
uses it for enforcement. It should be deterministic, flat enough for runtime,
and explicit about fallback behavior when proof is unavailable.

### Policy Update Commands

Portal sends typed update commands. The child agent validates, persists, compiles,
and acknowledges. Portal is never in the enforcement hot path.

```text
Portal authoring UI
  -> policy update command
  -> child agent validates full policy value
  -> child agent persists policy revision
  -> child agent compiles effective policy
  -> child agent enforces locally
```

## Proposed Complete JSON Shape

The following JSON combines the proposed authoring manifest, policy value,
effective policy, update commands, and capability registry into one example so a
worker can see how the pieces relate.

```json
{
  "schemaVersion": 1,
  "proposalStatus": "design-proposal-not-runtime-contract",
  "proposalIntent": "Guide the implementation of browser policy authoring, storage, compile, and enforcement contracts.",
  "workerInstruction": {
    "takeAsGuideOnly": true,
    "mustTranslateToEffectSchema": true,
    "mustUseSchemaBrands": true,
    "mustAddDecodeHelpers": true,
    "mustAddTests": true,
    "mustPreserveLocalChildAgentEnforcement": true,
    "mustNotCopyJsonDirectlyIntoRuntime": true
  },
  "contractFamilies": {
    "authoringManifest": "Portal-rendered sections, questions, options, visibility, enabled state, writesTo paths, and validation hints.",
    "policyValue": "Parent-authored durable policy state stored and versioned by the child agent.",
    "effectivePolicy": "Compiled deterministic child-agent execution plan.",
    "updateProtocol": "Typed get, preview, patch, replace, ack, reject, and rollback commands.",
    "capabilityRegistry": "Runtime device/platform/browser capability states used to hide, disable, or degrade fields."
  },
  "authoringManifest": {
    "manifestId": "browser-control-authoring-v1",
    "policyKind": "browser-control",
    "schemaVersion": 1,
    "title": "Browser controls",
    "renderingRules": {
      "hideInvisibleFields": true,
      "showDisabledFieldsWithReason": true,
      "neverInventFieldsOutsideManifest": true,
      "writeOnlyThroughWritesToPath": true,
      "previewBeforeApply": true
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
        "sectionId": "browser-management",
        "title": "Browser management",
        "purpose": "Top-level browser policy switch and default posture.",
        "fields": [
          {
            "fieldId": "browser.enabled",
            "kind": "boolean",
            "question": "Enable browser management?",
            "writesTo": "/browserPolicy/enabled",
            "defaultValue": false,
            "uiPriority": 10,
            "whenFalse": {
              "policyMeaning": "Browser activity is allowed and browser controls do not enforce.",
              "hiddenSections": [
                "managed-browser",
                "unmanaged-browser",
                "url-tab-evidence",
                "web-rules",
                "budgets",
                "downloads",
                "approvals",
                "reports",
                "audit"
              ]
            }
          },
          {
            "fieldId": "browser.defaultPosture",
            "kind": "single-choice",
            "question": "What should happen to browser activity?",
            "writesTo": "/browserPolicy/defaultPosture",
            "defaultValue": "observe",
            "visibleWhen": {
              "path": "/browserPolicy/enabled",
              "equals": true
            },
            "options": [
              {
                "value": "allow",
                "label": "Allow",
                "meaning": "Browser activity is allowed unless a more specific rule changes it.",
                "relevantSections": ["exceptions", "reports", "audit"]
              },
              {
                "value": "observe",
                "label": "Observe",
                "meaning": "Browser activity is allowed, evidence is collected according to data scope, and decisions are report-only.",
                "relevantSections": ["url-tab-evidence", "unmanaged-browser", "reports", "retention", "audit"]
              },
              {
                "value": "warn",
                "label": "Warn",
                "meaning": "Browser activity is allowed, but matching activity warns the child and records parent-visible events.",
                "relevantSections": ["child-facing", "unmanaged-browser", "web-rules", "reports", "audit"]
              },
              {
                "value": "ask",
                "label": "Ask",
                "meaning": "Browser activity needs parent approval unless an allow rule or override applies.",
                "relevantSections": ["approvals", "overrides", "child-facing", "reports", "audit"]
              },
              {
                "value": "limit",
                "label": "Limit",
                "meaning": "Browser activity is allowed inside configured schedules and budgets.",
                "relevantSections": ["budgets", "schedules", "approvals", "unmanaged-browser", "reports", "audit"]
              },
              {
                "value": "block",
                "label": "Block",
                "meaning": "Browser activity is blocked by default unless an explicit exception or parent override allows it.",
                "relevantSections": ["exceptions", "approvals", "child-facing", "audit"]
              }
            ]
          },
          {
            "fieldId": "browser.managementMode",
            "kind": "single-choice",
            "question": "How should browser management run on this device?",
            "writesTo": "/browserPolicy/managementMode",
            "defaultValue": "local-child-agent",
            "visibleWhen": {
              "path": "/browserPolicy/enabled",
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
        "sectionId": "managed-browser",
        "title": "Managed browser",
        "purpose": "Configure the browser path that can support exact URL, tab, download, and request-level rules.",
        "visibleWhen": {
          "path": "/browserPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "managedBrowser.mode",
            "kind": "single-choice",
            "question": "How should managed browser be used?",
            "writesTo": "/browserPolicy/managedBrowser/mode",
            "defaultValue": "available-for-exact-rules",
            "options": [
              {
                "value": "disabled",
                "label": "Disabled"
              },
              {
                "value": "available-for-exact-rules",
                "label": "Use for exact rules"
              },
              {
                "value": "required-for-exact-rules",
                "label": "Required for exact rules"
              },
              {
                "value": "required-for-all-browsing",
                "label": "Required for all browsing"
              }
            ]
          },
          {
            "fieldId": "managedBrowser.allowedFamilies",
            "kind": "multi-choice",
            "question": "Which managed browsers are allowed?",
            "writesTo": "/browserPolicy/managedBrowser/allowedFamilies",
            "defaultValue": ["edge-stable", "chrome-stable", "chrome-for-testing"],
            "options": [
              "edge-stable",
              "edge-beta",
              "edge-dev",
              "chrome-stable",
              "chrome-beta",
              "chrome-dev",
              "chrome-for-testing",
              "brave",
              "firefox",
              "safari-webkit",
              "owned-webview"
            ]
          },
          {
            "fieldId": "managedBrowser.launchMode",
            "kind": "single-choice",
            "question": "How should allowed browsing launch?",
            "writesTo": "/browserPolicy/managedBrowser/launchMode",
            "defaultValue": "ocentra-launcher",
            "options": ["manual", "ocentra-launcher", "default-browser-route", "managed-shell", "admin-provisioned"]
          },
          {
            "fieldId": "managedBrowser.profileMode",
            "kind": "single-choice",
            "question": "How should the managed profile behave?",
            "writesTo": "/browserPolicy/managedBrowser/profileMode",
            "defaultValue": "persistent-managed-profile",
            "options": ["persistent-managed-profile", "clear-on-schedule", "clear-on-session-end", "ephemeral"]
          },
          {
            "fieldId": "managedBrowser.bridgeRequirements",
            "kind": "multi-choice",
            "question": "Which bridge security rules are required?",
            "writesTo": "/browserPolicy/managedBrowser/bridgeRequirements",
            "defaultValue": [
              "owned-profile",
              "loopback-only",
              "random-port",
              "reject-default-profile",
              "reject-unmanaged-profile",
              "redacted-refs",
              "close-on-session-end",
              "degrade-safely"
            ],
            "options": [
              "owned-profile",
              "loopback-only",
              "random-port",
              "reject-default-profile",
              "reject-unmanaged-profile",
              "redacted-refs",
              "close-on-session-end",
              "degrade-safely"
            ]
          },
          {
            "fieldId": "managedBrowser.integrationMechanisms",
            "kind": "multi-choice",
            "question": "Which managed browser integrations may be used?",
            "writesTo": "/browserPolicy/managedBrowser/integrationMechanisms",
            "defaultValue": ["chromium-cdp", "managed-extension-native-host", "browser-policy"],
            "options": [
              "chromium-cdp",
              "webdriver-bidi",
              "managed-extension-native-host",
              "browser-policy",
              "owned-webview"
            ]
          }
        ]
      },
      {
        "sectionId": "unmanaged-browser",
        "title": "Unmanaged browser",
        "purpose": "Choose what happens when browser-like activity is outside the managed boundary.",
        "visibleWhen": {
          "path": "/browserPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "unmanagedBrowser.mode",
            "kind": "single-choice",
            "question": "What should happen to unmanaged browsers?",
            "writesTo": "/browserPolicy/unmanagedBrowser/mode",
            "defaultValue": "monitor",
            "options": ["allow", "monitor", "warn", "ask", "relaunch-managed", "block"]
          },
          {
            "fieldId": "unmanagedBrowser.graceSeconds",
            "kind": "number",
            "question": "How long should the child get before unmanaged browser action applies?",
            "writesTo": "/browserPolicy/unmanagedBrowser/graceSeconds",
            "defaultValue": 0,
            "min": 0,
            "max": 900,
            "visibleWhen": {
              "path": "/browserPolicy/unmanagedBrowser/mode",
              "includes": ["warn", "ask", "relaunch-managed", "block"]
            }
          },
          {
            "fieldId": "unmanagedBrowser.allowRecoverLaunchUrl",
            "kind": "boolean",
            "question": "If a launch URL is visible, should it reopen in managed browser?",
            "writesTo": "/browserPolicy/unmanagedBrowser/allowRecoverLaunchUrl",
            "defaultValue": true,
            "visibleWhen": {
              "path": "/browserPolicy/unmanagedBrowser/mode",
              "equals": "relaunch-managed"
            }
          },
          {
            "fieldId": "unmanagedBrowser.classificationTargets",
            "kind": "multi-choice",
            "question": "Which unmanaged browser types should be detected?",
            "writesTo": "/browserPolicy/unmanagedBrowser/classificationTargets",
            "defaultValue": [
              "known-browser",
              "portable-browser",
              "renamed-browser",
              "browser-like-process",
              "private-or-tor"
            ],
            "options": [
              "known-browser",
              "portable-browser",
              "renamed-browser",
              "browser-like-process",
              "embedded-webview",
              "private-or-tor",
              "unknown"
            ]
          }
        ]
      },
      {
        "sectionId": "url-tab-evidence",
        "title": "URL and tab evidence",
        "purpose": "Choose what exact browser state may be collected and used.",
        "visibleWhen": {
          "all": [
            {
              "path": "/browserPolicy/enabled",
              "equals": true
            },
            {
              "path": "/browserPolicy/defaultPosture",
              "notEquals": "block"
            }
          ]
        },
        "fields": [
          {
            "fieldId": "evidence.urlScope",
            "kind": "single-choice",
            "question": "What URL detail may rules use?",
            "writesTo": "/browserPolicy/evidence/urlScope",
            "defaultValue": "domain-origin-title",
            "options": ["none", "domain-only", "domain-origin-title", "full-url-without-query", "full-url-with-query"]
          },
          {
            "fieldId": "evidence.requiredProof",
            "kind": "single-choice",
            "question": "What proof is enough for exact browser rules?",
            "writesTo": "/browserPolicy/evidence/requiredProof",
            "defaultValue": "fresh-managed-active-tab",
            "options": [
              "process-running",
              "foreground-window",
              "network-domain",
              "managed-tab-list",
              "fresh-managed-tab-list",
              "fresh-managed-active-tab"
            ]
          },
          {
            "fieldId": "evidence.whenProofUnavailable",
            "kind": "single-choice",
            "question": "What if browser proof is unavailable?",
            "writesTo": "/browserPolicy/evidence/whenProofUnavailable",
            "defaultValue": "ask",
            "options": ["allow", "observe", "warn", "ask", "block-until-ready", "mark-unavailable"]
          },
          {
            "fieldId": "evidence.neverCollect",
            "kind": "multi-choice",
            "question": "What must browser rules never collect?",
            "writesTo": "/browserPolicy/evidence/neverCollect",
            "defaultValue": [
              "page-body",
              "chat-content",
              "screenshots",
              "keystrokes",
              "form-values",
              "secrets",
              "decrypted-https-payload",
              "raw-protocol-dumps"
            ],
            "options": [
              "page-body",
              "chat-content",
              "screenshots",
              "keystrokes",
              "form-values",
              "secrets",
              "decrypted-https-payload",
              "raw-protocol-dumps"
            ]
          }
        ]
      },
      {
        "sectionId": "web-rules",
        "title": "Web rules",
        "purpose": "Rules for URLs, domains, categories, search, video, browser sessions, and browser processes.",
        "visibleWhen": {
          "all": [
            {
              "path": "/browserPolicy/enabled",
              "equals": true
            },
            {
              "path": "/browserPolicy/defaultPosture",
              "notEquals": "allow"
            }
          ]
        },
        "fields": [
          {
            "fieldId": "rules.allowedTargetTypes",
            "kind": "multi-choice",
            "question": "What browser targets should rules match?",
            "writesTo": "/browserPolicy/rules/allowedTargetTypes",
            "defaultValue": [
              "exact-url",
              "domain-origin",
              "site-category",
              "browser-session",
              "browser-process",
              "capability-state"
            ],
            "options": [
              "exact-url",
              "domain-origin",
              "site-category",
              "search-terms",
              "video-channel",
              "browser-session",
              "browser-process",
              "capability-state",
              "download"
            ]
          },
          {
            "fieldId": "rules.allowedActions",
            "kind": "multi-choice",
            "question": "What actions may browser rules use?",
            "writesTo": "/browserPolicy/rules/allowedActions",
            "defaultValue": ["allow", "warn", "ask", "limit", "block"],
            "options": [
              "allow",
              "monitor",
              "warn",
              "ask",
              "limit",
              "block",
              "redirect",
              "close-tab",
              "close-browser",
              "relaunch-managed"
            ]
          },
          {
            "fieldId": "rules.items",
            "kind": "rule-list",
            "question": "Which browser rules should apply?",
            "writesTo": "/browserPolicy/rules/items",
            "defaultValue": []
          }
        ]
      },
      {
        "sectionId": "budgets",
        "title": "Browser time budgets",
        "purpose": "Budgets and schedules used when posture or a rule limits browser activity.",
        "visibleWhen": {
          "any": [
            {
              "path": "/browserPolicy/defaultPosture",
              "equals": "limit"
            },
            {
              "path": "/browserPolicy/rules/allowedActions",
              "includes": "limit"
            }
          ]
        },
        "fields": [
          {
            "fieldId": "budgets.enabled",
            "kind": "boolean",
            "question": "Use browser time budgets?",
            "writesTo": "/browserPolicy/budgets/enabled",
            "defaultValue": true
          },
          {
            "fieldId": "budgets.defaultDailyMinutes",
            "kind": "number",
            "question": "How many browser minutes are allowed per day?",
            "writesTo": "/browserPolicy/budgets/defaultDailyMinutes",
            "defaultValue": 60,
            "min": 0,
            "max": 1440,
            "visibleWhen": {
              "path": "/browserPolicy/budgets/enabled",
              "equals": true
            }
          },
          {
            "fieldId": "budgets.countingMode",
            "kind": "single-choice",
            "question": "Which browser time should count?",
            "writesTo": "/browserPolicy/budgets/countingMode",
            "defaultValue": "foreground-browser-time",
            "options": [
              "foreground-browser-time",
              "managed-active-tab-time",
              "managed-session-time",
              "all-browser-process-time",
              "unmanaged-as-unknown-web-time"
            ]
          }
        ]
      },
      {
        "sectionId": "downloads",
        "title": "Browser downloads",
        "purpose": "Download monitoring and decisions where the browser or OS can prove download evidence.",
        "visibleWhen": {
          "path": "/browserPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "downloads.mode",
            "kind": "single-choice",
            "question": "What should happen to browser downloads?",
            "writesTo": "/browserPolicy/downloads/mode",
            "defaultValue": "observe",
            "options": ["off", "observe", "warn", "ask", "block-risky", "block-all"]
          },
          {
            "fieldId": "downloads.blockedTypes",
            "kind": "multi-choice",
            "question": "Which download types should be blocked or asked?",
            "writesTo": "/browserPolicy/downloads/blockedTypes",
            "defaultValue": ["executable", "script", "unknown"],
            "options": ["executable", "script", "archive", "media", "unknown", "large-file", "browser-danger"]
          }
        ]
      },
      {
        "sectionId": "approvals",
        "title": "Parent approvals",
        "purpose": "Events that need parent approval and how approvals expire.",
        "visibleWhen": {
          "path": "/browserPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "approvals.requiredFor",
            "kind": "multi-choice",
            "question": "What should need parent approval?",
            "writesTo": "/browserPolicy/approvals/requiredFor",
            "defaultValue": ["blocked-site", "new-domain", "unmanaged-browser", "download", "time-extension"],
            "options": [
              "blocked-site",
              "new-domain",
              "unknown-category",
              "unmanaged-browser",
              "download",
              "time-extension",
              "managed-setup",
              "new-browser-install"
            ]
          },
          {
            "fieldId": "approvals.unansweredDefault",
            "kind": "single-choice",
            "question": "What happens if parent does not answer?",
            "writesTo": "/browserPolicy/approvals/unansweredDefault",
            "defaultValue": "deny",
            "options": ["deny", "allow-temporarily", "continue-observe-only", "keep-waiting"]
          }
        ]
      },
      {
        "sectionId": "reports",
        "title": "Reports and retention",
        "purpose": "What parents see and how long data stays available.",
        "visibleWhen": {
          "path": "/browserPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "reports.visibleFields",
            "kind": "multi-choice",
            "question": "What should parents see in browser reports?",
            "writesTo": "/browserPolicy/reports/visibleFields",
            "defaultValue": [
              "managed-status",
              "recent-domain-title",
              "unmanaged-use",
              "policy-decisions",
              "block-results",
              "time-budget",
              "source-capability"
            ],
            "options": [
              "managed-status",
              "recent-url",
              "recent-domain-title",
              "unmanaged-use",
              "policy-decisions",
              "block-results",
              "time-budget",
              "download-events",
              "source-capability"
            ]
          },
          {
            "fieldId": "retention.exactUrl",
            "kind": "retention",
            "question": "How long should exact URL evidence be kept?",
            "writesTo": "/browserPolicy/retention/exactUrl",
            "defaultValue": "7-days",
            "options": ["fresh-only", "24-hours", "7-days", "30-days", "until-reset", "delete-expired"]
          },
          {
            "fieldId": "custody.allowedUses",
            "kind": "multi-choice",
            "question": "Where may browser data be used?",
            "writesTo": "/browserPolicy/custody/allowedUses",
            "defaultValue": ["child-local", "lan-live", "parent-cache", "parent-report"],
            "options": ["child-local", "lan-live", "parent-cache", "parent-export", "parent-report", "unavailable"]
          }
        ]
      },
      {
        "sectionId": "audit",
        "title": "Audit",
        "purpose": "Audit requirements for strict browser actions.",
        "visibleWhen": {
          "path": "/browserPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "audit.requiredFields",
            "kind": "multi-choice",
            "question": "What should browser actions audit?",
            "writesTo": "/browserPolicy/audit/requiredFields",
            "defaultValue": [
              "policy-decision",
              "evidence-ref",
              "adapter-result",
              "timer-state",
              "parent-override",
              "rollback",
              "policy-version"
            ],
            "options": [
              "policy-decision",
              "evidence-ref",
              "ai-ref",
              "adapter-result",
              "timer-state",
              "parent-override",
              "rollback",
              "policy-version",
              "capability-state",
              "custody-label"
            ]
          }
        ]
      }
    ]
  },
  "policyValue": {
    "documentId": "browser-policy-mia-windows-laptop",
    "policyKind": "browser-control",
    "schemaVersion": 1,
    "revision": 17,
    "policyHash": "sha256:worker-must-compute-sample",
    "scope": {
      "familyId": "family-local-1",
      "childId": "child-mia",
      "deviceId": "device-windows-laptop",
      "platform": "windows",
      "source": "parent-authored"
    },
    "lifecycle": {
      "status": "active",
      "createdAt": "2026-05-28T00:00:00.000Z",
      "updatedAt": "2026-05-28T00:00:00.000Z",
      "updatedBy": "parent-controller",
      "validFrom": "2026-05-28T00:00:00.000Z",
      "expiresAt": null
    },
    "browserPolicy": {
      "enabled": true,
      "defaultPosture": "limit",
      "managementMode": "local-child-agent",
      "managedBrowser": {
        "mode": "required-for-exact-rules",
        "allowedFamilies": ["edge-stable", "chrome-stable", "chrome-for-testing"],
        "launchMode": "ocentra-launcher",
        "profileMode": "persistent-managed-profile",
        "integrationMechanisms": ["chromium-cdp", "managed-extension-native-host", "browser-policy"],
        "bridgeRequirements": [
          "owned-profile",
          "loopback-only",
          "random-port",
          "reject-default-profile",
          "reject-unmanaged-profile",
          "redacted-refs",
          "close-on-session-end",
          "degrade-safely"
        ],
        "startup": {
          "autoLaunchAtSignIn": false,
          "openLinksInManagedBrowser": true,
          "restoreTabs": false,
          "startPage": {
            "kind": "approved-home",
            "target": "school-dashboard"
          }
        },
        "profileControls": {
          "blockPrivateBrowsingWhenSupported": true,
          "blockGuestModeWhenSupported": true,
          "allowChildExtensionInstall": false,
          "allowProfileSwitching": false,
          "clearSchedule": "manual"
        },
        "setup": {
          "requireSetupCheckBeforeEnforce": true,
          "allowRepairRequest": true,
          "showUnavailableControls": true
        }
      },
      "unmanagedBrowser": {
        "mode": "relaunch-managed",
        "graceSeconds": 15,
        "allowRecoverLaunchUrl": true,
        "classificationTargets": [
          "known-browser",
          "portable-browser",
          "renamed-browser",
          "browser-like-process",
          "embedded-webview",
          "private-or-tor",
          "unknown"
        ],
        "allowedExceptions": [
          {
            "exceptionId": "unmanaged-browser-school-proctor",
            "targetKind": "executable-signature",
            "targetValue": "school-proctor-browser-signature-ref",
            "action": "allow",
            "scheduleId": "school-hours",
            "expiresAt": null
          }
        ],
        "escalation": {
          "afterAttempts": 3,
          "withinMinutes": 30,
          "nextAction": "ask"
        }
      },
      "evidence": {
        "urlScope": "domain-origin-title",
        "requiredProof": "fresh-managed-active-tab",
        "whenProofUnavailable": "ask",
        "freshnessSeconds": 30,
        "staleHandling": "report-only",
        "allowTabListOnlyForReports": true,
        "allowNetworkDomainForDomainRules": true,
        "allowNetworkDomainForExactUrlRules": false,
        "redaction": {
          "redactQueryString": true,
          "redactSensitiveParameters": true,
          "showExactUrlRequiresReveal": true
        },
        "neverCollect": [
          "page-body",
          "chat-content",
          "screenshots",
          "keystrokes",
          "form-values",
          "secrets",
          "decrypted-https-payload",
          "raw-protocol-dumps"
        ]
      },
      "rules": {
        "allowedTargetTypes": [
          "exact-url",
          "domain-origin",
          "site-category",
          "search-terms",
          "video-channel",
          "browser-session",
          "browser-process",
          "capability-state",
          "download"
        ],
        "allowedActions": [
          "allow",
          "monitor",
          "warn",
          "ask",
          "limit",
          "block",
          "redirect",
          "close-tab",
          "close-browser",
          "relaunch-managed"
        ],
        "conflictResolution": [
          "parent-override-wins",
          "emergency-mode-wins",
          "specific-url-beats-domain",
          "domain-beats-category",
          "download-block-beats-site-allow",
          "block-beats-allow",
          "managed-exact-proof-beats-network-proof",
          "fresh-proof-beats-stale-proof"
        ],
        "items": [
          {
            "ruleId": "allow-school-domains",
            "enabled": true,
            "priority": 100,
            "target": {
              "kind": "domain-origin",
              "values": ["school.example.invalid", "library.example.invalid"],
              "matchMode": "exact-or-subdomain"
            },
            "action": {
              "kind": "allow",
              "reasonCode": "school-work"
            },
            "proofRequirement": "domain-or-managed-url",
            "scheduleId": "school-hours",
            "budgetId": null,
            "auditLevel": "decision"
          },
          {
            "ruleId": "limit-video-after-homework",
            "enabled": true,
            "priority": 200,
            "target": {
              "kind": "site-category",
              "values": ["video-entertainment"],
              "matchMode": "category"
            },
            "action": {
              "kind": "limit",
              "budgetId": "video-evening-budget",
              "reasonCode": "entertainment-budget"
            },
            "proofRequirement": "managed-url-or-local-ai-category",
            "scheduleId": "after-homework",
            "auditLevel": "decision-and-timer"
          },
          {
            "ruleId": "ask-new-domain",
            "enabled": true,
            "priority": 300,
            "target": {
              "kind": "capability-state",
              "values": ["new-domain", "unknown-category"],
              "matchMode": "any"
            },
            "action": {
              "kind": "ask",
              "approvalKind": "new-domain",
              "reasonCode": "unknown-web"
            },
            "proofRequirement": "fresh-managed-active-tab",
            "scheduleId": "always",
            "auditLevel": "decision"
          },
          {
            "ruleId": "block-executable-downloads",
            "enabled": true,
            "priority": 400,
            "target": {
              "kind": "download",
              "values": ["executable", "script", "unknown"],
              "matchMode": "file-type"
            },
            "action": {
              "kind": "ask",
              "approvalKind": "download",
              "reasonCode": "download-risk"
            },
            "proofRequirement": "managed-download-metadata",
            "scheduleId": "always",
            "auditLevel": "decision"
          },
          {
            "ruleId": "block-private-tor",
            "enabled": true,
            "priority": 500,
            "target": {
              "kind": "browser-process",
              "values": ["tor-browser", "private-or-tor"],
              "matchMode": "browser-class"
            },
            "action": {
              "kind": "block",
              "reasonCode": "browser-bypass"
            },
            "proofRequirement": "process-running",
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
        },
        {
          "scheduleId": "bedtime",
          "kind": "weekly-window",
          "timezone": "America/Toronto",
          "windows": [
            {
              "days": ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"],
              "start": "21:00",
              "end": "06:00"
            }
          ]
        }
      ],
      "budgets": {
        "enabled": true,
        "defaultDailyMinutes": 60,
        "countingMode": "foreground-browser-time",
        "warningThresholdMinutes": 10,
        "graceMinutes": 5,
        "reset": "daily",
        "items": [
          {
            "budgetId": "browser-daily-budget",
            "targetKind": "browser-session",
            "minutes": 60,
            "period": "daily",
            "whenExhausted": "ask"
          },
          {
            "budgetId": "video-evening-budget",
            "targetKind": "site-category",
            "targetValues": ["video-entertainment"],
            "minutes": 30,
            "period": "daily",
            "whenExhausted": "block"
          }
        ]
      },
      "downloads": {
        "mode": "ask",
        "blockedTypes": ["executable", "script", "unknown"],
        "metadataScope": [
          "source-url",
          "final-url",
          "filename",
          "mime-type",
          "file-size",
          "browser-danger-status",
          "completion-state"
        ],
        "unmanagedDownloadHandling": "file-evidence-only",
        "whenSourceUnavailable": "ask"
      },
      "approvals": {
        "requiredFor": [
          "blocked-site",
          "new-domain",
          "unknown-category",
          "unmanaged-browser",
          "download",
          "time-extension",
          "managed-setup"
        ],
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
        "showUseManagedBrowserAction": true,
        "hideParentDiagnostics": true
      },
      "reports": {
        "visibleFields": [
          "managed-status",
          "recent-url",
          "recent-domain-title",
          "unmanaged-use",
          "policy-decisions",
          "block-results",
          "time-budget",
          "download-events",
          "source-capability"
        ],
        "summaries": ["by-child", "by-device", "by-browser", "by-domain", "by-category", "by-bypass-attempt"],
        "showExactUrlRequiresReveal": true
      },
      "retention": {
        "exactUrl": "7-days",
        "domainSummary": "30-days",
        "policyAudit": "90-days",
        "downloadAudit": "30-days",
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
      "portalAi": {
        "allowSummaries": true,
        "allowPolicyExplanation": true,
        "allowRuleSuggestions": false,
        "allowEvidenceRefs": true,
        "allowRawContent": false,
        "requiresManualReview": true,
        "fallbackWhenUnavailable": "manual-view"
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
        "auditEveryStrictAction": true,
        "auditFailedAdapterActions": true,
        "auditPolicyPreview": true
      },
      "platforms": {
        "windows": {
          "enabled": true,
          "allowedAdapters": [
            "managed-edge",
            "managed-chrome",
            "chrome-for-testing",
            "process-observation",
            "foreground-window-observation",
            "owned-process-termination",
            "managed-extension-native-host",
            "chromium-cdp"
          ],
          "manualRequiredAdapters": ["app-control-blocking", "wfp-domain-filtering"]
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
        "webPortal": {
          "authoringOnly": true,
          "mayRunCapture": false,
          "mayConnectToBrowserBridge": false
        }
      },
      "fallbacks": {
        "managedProfileMissing": "ask",
        "bridgeMissing": "ask",
        "extensionDisabled": "warn",
        "nativeHostMissing": "repair-prompt",
        "unsupportedBrowser": "monitor",
        "staleEvidence": "report-only",
        "networkAdapterUnavailable": "continue-managed-browser-controls",
        "processControlUnavailable": "warn",
        "enforcementFailure": "rollback-and-audit",
        "childDeviceOffline": "last-known-report-only",
        "platformUnsupported": "show-unavailable"
      }
    }
  },
  "effectivePolicy": {
    "documentId": "browser-effective-mia-windows-laptop",
    "compiledFromPolicyId": "browser-policy-mia-windows-laptop",
    "compiledFromRevision": 17,
    "schemaVersion": 1,
    "effectivePolicyHash": "sha256:worker-must-compute-sample",
    "compiledAt": "2026-05-28T00:00:00.000Z",
    "scope": {
      "familyId": "family-local-1",
      "childId": "child-mia",
      "deviceId": "device-windows-laptop",
      "platform": "windows"
    },
    "browserActivityDefaultDecision": "limit",
    "browserManagementEnabled": true,
    "managedBrowserDecision": {
      "requiredForExactWebRules": true,
      "requiredForAllBrowsing": false,
      "allowedFamilies": ["edge-stable", "chrome-stable", "chrome-for-testing"],
      "requiredBridgeSecurity": [
        "owned-profile",
        "loopback-only",
        "random-port",
        "reject-default-profile",
        "reject-unmanaged-profile",
        "redacted-refs",
        "close-on-session-end",
        "degrade-safely"
      ]
    },
    "unmanagedBrowserDecision": {
      "defaultAction": "relaunch-managed",
      "graceSeconds": 15,
      "recoverLaunchUrlWhenAvailable": true,
      "ifUrlCannotBeRecovered": "open-managed-start-page",
      "auditRequired": true
    },
    "proofRequirements": {
      "exactUrlRules": "fresh-managed-active-tab",
      "domainRules": "domain-or-managed-url",
      "browserProcessRules": "process-running",
      "downloadRules": "managed-download-metadata",
      "timeBudgets": "foreground-window-or-managed-session",
      "reportOnly": "stale-or-degraded-allowed"
    },
    "fallbackDecisions": {
      "proofUnavailable": "ask",
      "staleEvidence": "report-only",
      "managedBridgeMissing": "ask",
      "platformUnsupported": "unavailable",
      "adapterError": "rollback-and-audit"
    },
    "rulesInPriorityOrder": [
      {
        "ruleId": "allow-school-domains",
        "priority": 100,
        "decision": "allow",
        "targetKind": "domain-origin",
        "proofRequirement": "domain-or-managed-url",
        "scheduleId": "school-hours"
      },
      {
        "ruleId": "limit-video-after-homework",
        "priority": 200,
        "decision": "limit",
        "targetKind": "site-category",
        "proofRequirement": "managed-url-or-local-ai-category",
        "scheduleId": "after-homework",
        "budgetId": "video-evening-budget"
      },
      {
        "ruleId": "ask-new-domain",
        "priority": 300,
        "decision": "ask",
        "targetKind": "capability-state",
        "proofRequirement": "fresh-managed-active-tab",
        "scheduleId": "always"
      },
      {
        "ruleId": "block-executable-downloads",
        "priority": 400,
        "decision": "ask",
        "targetKind": "download",
        "proofRequirement": "managed-download-metadata",
        "scheduleId": "always"
      },
      {
        "ruleId": "block-private-tor",
        "priority": 500,
        "decision": "block",
        "targetKind": "browser-process",
        "proofRequirement": "process-running",
        "scheduleId": "always"
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
        },
        "bedtime": {
          "kind": "weekly-window",
          "timezone": "America/Toronto"
        }
      },
      "budgetsById": {
        "browser-daily-budget": {
          "minutes": 60,
          "period": "daily",
          "whenExhausted": "ask"
        },
        "video-evening-budget": {
          "minutes": 30,
          "period": "daily",
          "whenExhausted": "block"
        }
      }
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
        "commandType": "browser-policy.get.requested",
        "purpose": "Portal asks the child agent for current policy value and revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "includeAuthoringManifest": true,
          "includeEffectivePolicy": true
        }
      },
      {
        "commandType": "browser-policy.preview.requested",
        "purpose": "Portal asks whether proposed changes validate and what effective policy would result.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "baseRevision": 17,
          "patch": [
            {
              "op": "replace",
              "path": "/browserPolicy/defaultPosture",
              "value": "block"
            }
          ]
        },
        "responseShape": {
          "accepted": true,
          "wouldCreateRevision": 18,
          "effectivePolicyPreviewHash": "sha256:static-sample-token",
          "warnings": [],
          "unsupportedSettings": []
        }
      },
      {
        "commandType": "browser-policy.patch.requested",
        "purpose": "Portal sends a small settings change with an expected revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 17,
          "patch": [
            {
              "op": "replace",
              "path": "/browserPolicy/unmanagedBrowser/mode",
              "value": "block"
            }
          ],
          "reason": "parent-ui-change"
        },
        "acceptedResponseShape": {
          "eventType": "browser-policy.patch.accepted",
          "newRevision": 18,
          "policyHash": "sha256:static-sample-token",
          "effectivePolicyHash": "sha256:static-sample-token",
          "requiresRestart": false,
          "unsupportedSettings": []
        },
        "rejectedResponseShape": {
          "eventType": "browser-policy.patch.rejected",
          "currentRevision": 18,
          "reason": "revision-conflict",
          "validationErrors": []
        }
      },
      {
        "commandType": "browser-policy.replace.requested",
        "purpose": "Portal sends a full policy replacement for setup, import, reset, or wizard save.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 17,
          "replacementPolicy": {
            "documentId": "browser-policy-mia-windows-laptop",
            "policyKind": "browser-control",
            "schemaVersion": 1,
            "revision": 18
          },
          "reason": "parent-wizard-save"
        }
      },
      {
        "commandType": "browser-policy.rollback.requested",
        "purpose": "Parent asks child agent to roll back to previous valid revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "targetRevision": 17,
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
      "rejectPartialLimitWithoutBudget": true,
      "rejectExactUrlRuleWithoutManagedRequirementOrFallback": true
    }
  },
  "capabilityRegistry": {
    "deviceId": "device-windows-laptop",
    "generatedAt": "2026-05-28T00:00:00.000Z",
    "platform": "windows",
    "capabilities": [
      {
        "capabilityId": "managed-edge-cdp",
        "state": "ready",
        "proof": "manual-required-or-runtime-read-model",
        "affectsFields": ["managedBrowser.allowedFamilies", "evidence.requiredProof", "rules.allowedTargetTypes"]
      },
      {
        "capabilityId": "managed-chrome-cdp",
        "state": "ready",
        "proof": "manual-required-or-runtime-read-model",
        "affectsFields": ["managedBrowser.allowedFamilies", "evidence.requiredProof", "rules.allowedTargetTypes"]
      },
      {
        "capabilityId": "managed-extension-native-host",
        "state": "manual-required",
        "proof": "not-yet-proven",
        "affectsFields": ["downloads.mode", "downloads.blockedTypes", "evidence.urlScope"]
      },
      {
        "capabilityId": "unmanaged-process-terminate",
        "state": "ready",
        "proof": "runtime-adapter-proof-required",
        "affectsFields": ["unmanagedBrowser.mode"]
      },
      {
        "capabilityId": "windows-app-control-block",
        "state": "manual-required",
        "proof": "not-yet-proven",
        "affectsFields": ["browser.defaultPosture", "unmanagedBrowser.mode"]
      },
      {
        "capabilityId": "network-domain-filtering",
        "state": "manual-required",
        "proof": "not-yet-proven",
        "affectsFields": ["rules.allowedTargetTypes", "evidence.requiredProof"]
      }
    ]
  }
}
```

## Implementation Notes For Worker

- Start with domain contracts before Portal UI.
- Keep authoring manifest ids, field ids, section ids, option ids, policy ids,
  rule ids, schedule ids, and capability ids branded.
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
- Add explicit tests for hidden/visible branch behavior so UI cannot show budget
  controls when browser management is disabled or ask/download controls when the
  posture makes them irrelevant.
- Add explicit tests for offline behavior: child agent continues enforcing the
  last valid compiled policy when Portal is disconnected.
