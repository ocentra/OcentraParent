<!-- agent-capsule -->

> Agent Capsule
> Doc: Game Control Schema Proposal
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Game Control Schema Proposal

Status: worker handoff proposal. This is not final source code.

This document proposes a structured game control schema that can support:

- Portal-rendered question/option UI.
- Parent-authored game settings.
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
- Local child-agent persistence, compile, rollback, timer recovery, capability
  gating, and audit behavior.

## Architecture

The proposal has four related documents.

### Authoring Manifest

The authoring manifest tells Portal what questions to show, what controls to
render, which options are allowed, where the answer writes into the policy value
document, and when the field is visible or enabled.

Portal must not invent game policy questions outside this manifest. If the UI
needs a new question, the manifest and value schema need a contract update.

### Policy Value Document

The policy value document is the parent-authored game policy. It is the durable
source of parent intent. The child agent validates it as a whole after any
update.

### Effective Policy Document

The effective policy document is the compiled execution plan. The child agent
uses it for enforcement. It should be deterministic, flat enough for runtime,
and explicit about fallback behavior when proof is unavailable.

### Policy Update Commands

Portal sends typed update commands. The child agent validates, persists,
compiles, and acknowledges. Portal is never in the enforcement hot path.

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
  "proposalStatus": "worker-handoff-design-proposal-not-runtime-contract",
  "proposalIntent": "Guide the implementation of game policy authoring, storage, compile, capability gating, time-budget recovery, approval, and enforcement contracts.",
  "workerInstruction": {
    "takeAsGuideOnly": true,
    "mustTranslateToEffectSchema": true,
    "mustUseSchemaBrands": true,
    "mustAddDecodeHelpers": true,
    "mustAddTests": true,
    "mustPreserveLocalChildAgentEnforcement": true,
    "mustNotCopyJsonDirectlyIntoRuntime": true,
    "mustMirrorRustOnlyAfterTypeScriptContracts": true
  },
  "contractFamilies": {
    "authoringManifest": "Portal-rendered sections, questions, options, visibility, enabled state, writesTo paths, and validation hints.",
    "policyValue": "Parent-authored durable policy state stored and versioned by the child agent.",
    "effectivePolicy": "Compiled deterministic child-agent execution plan.",
    "updateProtocol": "Typed get, preview, patch, replace, ack, reject, and rollback commands.",
    "capabilityRegistry": "Runtime device/platform/game capability states used to hide, disable, or degrade fields."
  },
  "authoringManifest": {
    "manifestId": "game-control-authoring-v1",
    "policyKind": "game-control",
    "schemaVersion": 1,
    "title": "Game controls",
    "renderingRules": {
      "hideInvisibleFields": true,
      "showDisabledFieldsWithReason": true,
      "neverInventFieldsOutsideManifest": true,
      "writeOnlyThroughWritesToPath": true,
      "previewBeforeApply": true,
      "showProofLevelNearStrictActions": true
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
        "sectionId": "game-management",
        "title": "Game management",
        "purpose": "Top-level game policy switch and default posture.",
        "fields": [
          {
            "fieldId": "game.enabled",
            "kind": "boolean",
            "question": "Enable game management?",
            "writesTo": "/gamePolicy/enabled",
            "defaultValue": false,
            "uiPriority": 10,
            "whenFalse": {
              "policyMeaning": "Game activity is allowed and game controls do not enforce.",
              "hiddenSections": [
                "inventory",
                "session-evidence",
                "native-games",
                "launcher-games",
                "browser-cloud-games",
                "game-rules",
                "budgets",
                "approvals",
                "reports",
                "audit"
              ]
            }
          },
          {
            "fieldId": "game.defaultPosture",
            "kind": "single-choice",
            "question": "What should happen to game activity?",
            "writesTo": "/gamePolicy/defaultPosture",
            "defaultValue": "observe",
            "visibleWhen": {
              "path": "/gamePolicy/enabled",
              "equals": true
            },
            "options": [
              {
                "value": "allow",
                "label": "Allow",
                "meaning": "Game activity is allowed unless a more specific rule changes it.",
                "relevantSections": ["exceptions", "reports", "audit"]
              },
              {
                "value": "observe",
                "label": "Observe",
                "meaning": "Game activity is allowed, evidence is collected according to data scope, and decisions are report-only.",
                "relevantSections": ["inventory", "session-evidence", "reports", "retention", "audit"]
              },
              {
                "value": "warn",
                "label": "Warn",
                "meaning": "Game activity is allowed, but matching activity warns the child and records parent-visible events.",
                "relevantSections": ["child-facing", "game-rules", "reports", "audit"]
              },
              {
                "value": "ask",
                "label": "Ask",
                "meaning": "Game activity needs parent approval unless an allow rule or override applies.",
                "relevantSections": ["approvals", "overrides", "child-facing", "reports", "audit"]
              },
              {
                "value": "limit",
                "label": "Limit",
                "meaning": "Game activity is allowed inside configured schedules and budgets.",
                "relevantSections": ["budgets", "schedules", "approvals", "reports", "audit"]
              },
              {
                "value": "block",
                "label": "Block",
                "meaning": "Game activity is blocked by default unless an explicit exception or parent override allows it.",
                "relevantSections": ["exceptions", "approvals", "child-facing", "audit"]
              }
            ]
          },
          {
            "fieldId": "game.managementMode",
            "kind": "single-choice",
            "question": "How should game management run on this device?",
            "writesTo": "/gamePolicy/managementMode",
            "defaultValue": "local-child-agent",
            "visibleWhen": {
              "path": "/gamePolicy/enabled",
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
                "value": "platform-family-controls",
                "label": "Platform family controls"
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
        "title": "Game inventory",
        "purpose": "Configure which installed/detectable games, launchers, stores, and unknown candidates may be shown or used.",
        "visibleWhen": {
          "path": "/gamePolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "inventory.mode",
            "kind": "single-choice",
            "question": "Which game inventory should be used?",
            "writesTo": "/gamePolicy/inventory/mode",
            "defaultValue": "local-installed-and-running",
            "options": [
              "off",
              "running-only",
              "local-installed-and-running",
              "launcher-backed",
              "platform-family-controls",
              "manual-list-only"
            ]
          },
          {
            "fieldId": "inventory.sources",
            "kind": "multi-choice",
            "question": "Which inventory sources may contribute game evidence?",
            "writesTo": "/gamePolicy/inventory/sources",
            "defaultValue": [
              "process-snapshot",
              "foreground-window",
              "installed-app-records",
              "store-packages",
              "launcher-manifests"
            ],
            "options": [
              "process-snapshot",
              "foreground-window",
              "installed-app-records",
              "start-menu-shortcuts",
              "store-packages",
              "launcher-manifests",
              "manual-parent-catalog",
              "platform-family-controls",
              "browser-managed-url",
              "network-service-hint"
            ]
          },
          {
            "fieldId": "inventory.classificationStates",
            "kind": "multi-choice",
            "question": "Which game classifications should appear in rules?",
            "writesTo": "/gamePolicy/inventory/classificationStates",
            "defaultValue": [
              "known-game",
              "known-launcher",
              "launcher-game-candidate",
              "possibly-game",
              "unknown-process"
            ],
            "options": [
              "known-game",
              "known-app",
              "known-launcher",
              "launcher-game-candidate",
              "possibly-game",
              "unknown-process",
              "permission-limited",
              "unsupported-platform",
              "stale",
              "adapter-error"
            ]
          },
          {
            "fieldId": "inventory.ratingSources",
            "kind": "multi-choice",
            "question": "Which rating or category sources may be used?",
            "writesTo": "/gamePolicy/inventory/ratingSources",
            "defaultValue": ["store-metadata", "parent-catalog", "rating-authority"],
            "options": [
              "store-metadata",
              "launcher-metadata",
              "parent-catalog",
              "rating-authority",
              "local-classifier-digest",
              "unknown"
            ]
          }
        ]
      },
      {
        "sectionId": "session-evidence",
        "title": "Game session evidence",
        "purpose": "Choose which proof is enough to count game time or support strict game rules.",
        "visibleWhen": {
          "path": "/gamePolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "evidence.requiredProof",
            "kind": "single-choice",
            "question": "What proof is enough for game rules?",
            "writesTo": "/gamePolicy/evidence/requiredProof",
            "defaultValue": "foreground-known-game-session",
            "options": [
              "process-running",
              "foreground-window",
              "foreground-known-game-session",
              "launcher-attributed-session",
              "package-identity-session",
              "managed-browser-game-url",
              "platform-family-activity",
              "manual-parent-catalog"
            ]
          },
          {
            "fieldId": "evidence.durationCountingMode",
            "kind": "single-choice",
            "question": "Which time should count toward game budgets?",
            "writesTo": "/gamePolicy/evidence/durationCountingMode",
            "defaultValue": "foreground-game-time",
            "options": [
              "foreground-game-time",
              "running-game-process-time",
              "launcher-child-game-time",
              "known-game-only-time",
              "known-and-possible-game-time",
              "browser-managed-game-time",
              "cloud-client-foreground-time",
              "platform-reported-game-time"
            ]
          },
          {
            "fieldId": "evidence.whenProofUnavailable",
            "kind": "single-choice",
            "question": "What if game proof is unavailable?",
            "writesTo": "/gamePolicy/evidence/whenProofUnavailable",
            "defaultValue": "ask",
            "options": ["allow", "observe", "warn", "ask", "block-until-ready", "mark-unavailable"]
          },
          {
            "fieldId": "evidence.neverCollect",
            "kind": "multi-choice",
            "question": "What must game rules never collect?",
            "writesTo": "/gamePolicy/evidence/neverCollect",
            "defaultValue": [
              "screenshots",
              "keystrokes",
              "chat-content",
              "voice-content",
              "game-memory",
              "decrypted-network-payload",
              "launcher-credentials",
              "private-social-graph",
              "raw-anti-cheat-data"
            ],
            "options": [
              "screenshots",
              "keystrokes",
              "chat-content",
              "voice-content",
              "game-memory",
              "decrypted-network-payload",
              "launcher-credentials",
              "private-social-graph",
              "raw-anti-cheat-data",
              "purchase-history",
              "cloud-save-content"
            ]
          }
        ]
      },
      {
        "sectionId": "native-games",
        "title": "Native games",
        "purpose": "Configure desktop/native executable and package controls.",
        "visibleWhen": {
          "path": "/gamePolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "nativeGames.mode",
            "kind": "single-choice",
            "question": "How should native games be handled?",
            "writesTo": "/gamePolicy/nativeGames/mode",
            "defaultValue": "observe-and-limit",
            "options": ["off", "observe", "warn", "ask", "observe-and-limit", "block-when-proven"]
          },
          {
            "fieldId": "nativeGames.allowedIdentityTypes",
            "kind": "multi-choice",
            "question": "Which native game identities may rules target?",
            "writesTo": "/gamePolicy/nativeGames/allowedIdentityTypes",
            "defaultValue": ["package-id", "executable-hash", "publisher-signature", "launcher-app-id", "game-title"],
            "options": [
              "package-id",
              "executable-path",
              "executable-hash",
              "publisher-signature",
              "launcher-app-id",
              "game-title",
              "game-category",
              "rating-threshold",
              "unknown-candidate"
            ]
          },
          {
            "fieldId": "nativeGames.strictActions",
            "kind": "multi-choice",
            "question": "Which strict native game actions may be used?",
            "writesTo": "/gamePolicy/nativeGames/strictActions",
            "defaultValue": ["ask", "time-limit", "terminate-accessible-process"],
            "options": [
              "ask",
              "time-limit",
              "terminate-accessible-process",
              "block-launch",
              "temporary-block",
              "repair-required",
              "observe-only"
            ]
          }
        ]
      },
      {
        "sectionId": "launcher-games",
        "title": "Launchers and stores",
        "purpose": "Configure launcher-backed attribution and limits without treating launcher activity as guaranteed gameplay.",
        "visibleWhen": {
          "path": "/gamePolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "launchers.supportedKinds",
            "kind": "multi-choice",
            "question": "Which launchers or stores should be considered?",
            "writesTo": "/gamePolicy/launchers/supportedKinds",
            "defaultValue": ["steam", "xbox-app", "microsoft-store", "epic-games", "roblox", "minecraft-launcher"],
            "options": [
              "steam",
              "epic-games",
              "xbox-app",
              "microsoft-store",
              "riot-client",
              "battle-net",
              "ea-app",
              "ubisoft-connect",
              "gog-galaxy",
              "roblox",
              "minecraft-launcher",
              "unknown-launcher"
            ]
          },
          {
            "fieldId": "launchers.launcherOnlyHandling",
            "kind": "single-choice",
            "question": "How should launcher-only time be handled?",
            "writesTo": "/gamePolicy/launchers/launcherOnlyHandling",
            "defaultValue": "report-separately",
            "options": ["ignore", "report-separately", "count-as-possible-game", "ask-after-threshold", "block"]
          },
          {
            "fieldId": "launchers.whenManifestUnavailable",
            "kind": "single-choice",
            "question": "What if launcher manifests cannot be read?",
            "writesTo": "/gamePolicy/launchers/whenManifestUnavailable",
            "defaultValue": "use-process-evidence-only",
            "options": ["use-process-evidence-only", "mark-unavailable", "ask", "manual-parent-catalog"]
          }
        ]
      },
      {
        "sectionId": "browser-cloud-games",
        "title": "Browser and cloud games",
        "purpose": "Configure browser-game and cloud-game handling using the correct evidence boundary.",
        "visibleWhen": {
          "path": "/gamePolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "browserCloud.mode",
            "kind": "single-choice",
            "question": "How should browser and cloud games be counted?",
            "writesTo": "/gamePolicy/browserCloud/mode",
            "defaultValue": "managed-proof-only",
            "options": ["off", "report-only", "managed-proof-only", "domain-service-hint", "count-cloud-client", "ask"]
          },
          {
            "fieldId": "browserCloud.allowedEvidence",
            "kind": "multi-choice",
            "question": "Which evidence may classify browser or cloud game use?",
            "writesTo": "/gamePolicy/browserCloud/allowedEvidence",
            "defaultValue": ["managed-browser-url", "cloud-client-process", "parent-catalog"],
            "options": [
              "managed-browser-url",
              "managed-browser-title",
              "domain-service-hint",
              "cloud-client-process",
              "network-flow-service-hint",
              "platform-family-activity",
              "parent-catalog"
            ]
          }
        ]
      },
      {
        "sectionId": "game-rules",
        "title": "Game rules",
        "purpose": "Rules for games, launchers, categories, ratings, unknown candidates, sessions, and capability states.",
        "visibleWhen": {
          "all": [
            {
              "path": "/gamePolicy/enabled",
              "equals": true
            },
            {
              "path": "/gamePolicy/defaultPosture",
              "notEquals": "allow"
            }
          ]
        },
        "fields": [
          {
            "fieldId": "rules.allowedTargetTypes",
            "kind": "multi-choice",
            "question": "What game targets should rules match?",
            "writesTo": "/gamePolicy/rules/allowedTargetTypes",
            "defaultValue": [
              "known-game",
              "game-category",
              "launcher-kind",
              "game-session",
              "possibly-game",
              "capability-state"
            ],
            "options": [
              "known-game",
              "game-title",
              "game-category",
              "rating-threshold",
              "launcher-kind",
              "launcher-app-id",
              "executable-identity",
              "package-id",
              "game-session",
              "possibly-game",
              "unknown-process",
              "browser-game-site",
              "cloud-game-service",
              "capability-state"
            ]
          },
          {
            "fieldId": "rules.allowedActions",
            "kind": "multi-choice",
            "question": "What actions may game rules use?",
            "writesTo": "/gamePolicy/rules/allowedActions",
            "defaultValue": ["allow", "monitor", "warn", "ask", "limit", "terminate"],
            "options": [
              "allow",
              "monitor",
              "warn",
              "ask",
              "limit",
              "terminate",
              "block-launch",
              "temporary-block",
              "platform-shield",
              "manual-required"
            ]
          },
          {
            "fieldId": "rules.items",
            "kind": "rule-list",
            "question": "Which game rules should apply?",
            "writesTo": "/gamePolicy/rules/items",
            "defaultValue": []
          }
        ]
      },
      {
        "sectionId": "budgets",
        "title": "Game time budgets",
        "purpose": "Budgets and schedules used when posture or a rule limits game activity.",
        "visibleWhen": {
          "any": [
            {
              "path": "/gamePolicy/defaultPosture",
              "equals": "limit"
            },
            {
              "path": "/gamePolicy/rules/allowedActions",
              "includes": "limit"
            }
          ]
        },
        "fields": [
          {
            "fieldId": "budgets.enabled",
            "kind": "boolean",
            "question": "Use game time budgets?",
            "writesTo": "/gamePolicy/budgets/enabled",
            "defaultValue": true
          },
          {
            "fieldId": "budgets.defaultDailyMinutes",
            "kind": "number",
            "question": "How many game minutes are allowed per day?",
            "writesTo": "/gamePolicy/budgets/defaultDailyMinutes",
            "defaultValue": 60,
            "min": 0,
            "max": 1440,
            "visibleWhen": {
              "path": "/gamePolicy/budgets/enabled",
              "equals": true
            }
          },
          {
            "fieldId": "budgets.warningThresholdMinutes",
            "kind": "number",
            "question": "How many minutes before a limit should the child be warned?",
            "writesTo": "/gamePolicy/budgets/warningThresholdMinutes",
            "defaultValue": 10,
            "min": 0,
            "max": 120
          },
          {
            "fieldId": "budgets.graceMinutes",
            "kind": "number",
            "question": "How many grace minutes are allowed before strict action?",
            "writesTo": "/gamePolicy/budgets/graceMinutes",
            "defaultValue": 5,
            "min": 0,
            "max": 60
          }
        ]
      },
      {
        "sectionId": "approvals",
        "title": "Parent approvals",
        "purpose": "Events that need parent approval and how approvals expire.",
        "visibleWhen": {
          "path": "/gamePolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "approvals.requiredFor",
            "kind": "multi-choice",
            "question": "What should need parent approval?",
            "writesTo": "/gamePolicy/approvals/requiredFor",
            "defaultValue": ["blocked-game", "new-game", "unknown-game", "time-extension"],
            "options": [
              "blocked-game",
              "new-game",
              "unknown-game",
              "possibly-game",
              "launcher-game-candidate",
              "time-extension",
              "rating-threshold",
              "multiplayer-capable-game",
              "platform-setup"
            ]
          },
          {
            "fieldId": "approvals.unansweredDefault",
            "kind": "single-choice",
            "question": "What happens if parent does not answer?",
            "writesTo": "/gamePolicy/approvals/unansweredDefault",
            "defaultValue": "deny",
            "options": ["deny", "allow-temporarily", "continue-observe-only", "keep-waiting"]
          },
          {
            "fieldId": "approvals.allowedParentResponses",
            "kind": "multi-choice",
            "question": "Which parent responses are allowed?",
            "writesTo": "/gamePolicy/approvals/allowedParentResponses",
            "defaultValue": ["approve-once", "approve-session", "approve-until-time", "deny", "extend-time", "cancel"],
            "options": [
              "approve-once",
              "approve-session",
              "approve-until-time",
              "approve-for-schedule",
              "deny",
              "extend-time",
              "cancel"
            ]
          }
        ]
      },
      {
        "sectionId": "reports",
        "title": "Reports and retention",
        "purpose": "What parents see and how long data stays available.",
        "visibleWhen": {
          "path": "/gamePolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "reports.visibleFields",
            "kind": "multi-choice",
            "question": "What should parents see in game reports?",
            "writesTo": "/gamePolicy/reports/visibleFields",
            "defaultValue": [
              "inventory-status",
              "running-now",
              "foreground-now",
              "recent-sessions",
              "time-budget",
              "policy-decisions",
              "source-capability"
            ],
            "options": [
              "inventory-status",
              "running-now",
              "foreground-now",
              "recent-sessions",
              "daily-rollups",
              "unknown-candidates",
              "launcher-status",
              "rating-category",
              "approval-events",
              "block-results",
              "time-budget",
              "policy-decisions",
              "source-capability"
            ]
          },
          {
            "fieldId": "retention.rawEvidence",
            "kind": "retention",
            "question": "How long should raw process/window game evidence be kept?",
            "writesTo": "/gamePolicy/retention/rawEvidence",
            "defaultValue": "7-days",
            "options": ["fresh-only", "24-hours", "7-days", "30-days", "until-reset", "delete-expired"]
          },
          {
            "fieldId": "custody.allowedUses",
            "kind": "multi-choice",
            "question": "Where may game data be used?",
            "writesTo": "/gamePolicy/custody/allowedUses",
            "defaultValue": ["child-local", "lan-live", "parent-cache", "parent-report"],
            "options": ["child-local", "lan-live", "parent-cache", "parent-export", "parent-report", "unavailable"]
          }
        ]
      },
      {
        "sectionId": "audit",
        "title": "Audit",
        "purpose": "Audit requirements for strict game actions.",
        "visibleWhen": {
          "path": "/gamePolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "audit.requiredFields",
            "kind": "multi-choice",
            "question": "What should game actions audit?",
            "writesTo": "/gamePolicy/audit/requiredFields",
            "defaultValue": [
              "policy-decision",
              "evidence-ref",
              "adapter-result",
              "timer-state",
              "parent-override",
              "rollback",
              "policy-version",
              "capability-state"
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
              "custody-label",
              "protected-process-status",
              "target-recheck"
            ]
          }
        ]
      }
    ]
  },
  "policyValue": {
    "documentId": "game-policy-mia-windows-laptop",
    "policyKind": "game-control",
    "schemaVersion": 1,
    "revision": 3,
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
    "gamePolicy": {
      "enabled": true,
      "defaultPosture": "limit",
      "managementMode": "local-child-agent",
      "inventory": {
        "mode": "local-installed-and-running",
        "sources": [
          "process-snapshot",
          "foreground-window",
          "installed-app-records",
          "store-packages",
          "launcher-manifests"
        ],
        "classificationStates": [
          "known-game",
          "known-launcher",
          "launcher-game-candidate",
          "possibly-game",
          "unknown-process",
          "permission-limited",
          "stale"
        ],
        "ratingSources": ["store-metadata", "parent-catalog", "rating-authority"],
        "unknownHandling": {
          "showUnknownCandidates": true,
          "askAfterForegroundMinutes": 10,
          "allowParentLabeling": true,
          "keepPossiblyGameSeparate": true
        }
      },
      "evidence": {
        "requiredProof": "foreground-known-game-session",
        "durationCountingMode": "foreground-game-time",
        "whenProofUnavailable": "ask",
        "freshnessSeconds": 30,
        "staleHandling": "report-only",
        "allowRunningOnlyForReports": true,
        "allowPossibleGameForBudgets": false,
        "allowNetworkServiceHintForGameRules": false,
        "neverCollect": [
          "screenshots",
          "keystrokes",
          "chat-content",
          "voice-content",
          "game-memory",
          "decrypted-network-payload",
          "launcher-credentials",
          "private-social-graph",
          "raw-anti-cheat-data"
        ]
      },
      "nativeGames": {
        "mode": "observe-and-limit",
        "allowedIdentityTypes": [
          "package-id",
          "executable-hash",
          "publisher-signature",
          "launcher-app-id",
          "game-title"
        ],
        "strictActions": ["ask", "time-limit", "terminate-accessible-process"],
        "terminate": {
          "enabled": true,
          "graceSeconds": 60,
          "requireTargetRecheck": true,
          "skipProtectedOrAntiCheatLimited": true,
          "ifTerminateFails": "ask",
          "childWarningBeforeTerminate": true
        },
        "blockLaunch": {
          "enabled": false,
          "mechanisms": ["windows-app-control", "package-policy"],
          "state": "manual-required",
          "rollbackRequired": true
        }
      },
      "launchers": {
        "supportedKinds": ["steam", "xbox-app", "microsoft-store", "epic-games", "roblox", "minecraft-launcher"],
        "launcherOnlyHandling": "report-separately",
        "whenManifestUnavailable": "use-process-evidence-only",
        "safeDataScope": [
          "launcher-kind",
          "launcher-app-id",
          "launcher-title",
          "install-path-hint",
          "executable-path-hint",
          "manifest-observed-at",
          "capability-status"
        ],
        "neverRead": ["launcher-credentials", "private-chat", "purchase-history", "cloud-save-content", "account-token"]
      },
      "browserCloud": {
        "mode": "managed-proof-only",
        "allowedEvidence": ["managed-browser-url", "cloud-client-process", "parent-catalog"],
        "browserGameHandling": "defer-to-browser-control-proof",
        "cloudGameHandling": "count-cloud-client-foreground-time",
        "networkHintHandling": "report-only"
      },
      "rules": {
        "allowedTargetTypes": [
          "known-game",
          "game-title",
          "game-category",
          "rating-threshold",
          "launcher-kind",
          "launcher-app-id",
          "executable-identity",
          "package-id",
          "game-session",
          "possibly-game",
          "unknown-process",
          "browser-game-site",
          "cloud-game-service",
          "capability-state"
        ],
        "allowedActions": [
          "allow",
          "monitor",
          "warn",
          "ask",
          "limit",
          "terminate",
          "block-launch",
          "temporary-block",
          "platform-shield",
          "manual-required"
        ],
        "conflictResolution": [
          "parent-override-wins",
          "emergency-mode-wins",
          "specific-game-beats-category",
          "specific-launcher-app-beats-launcher-kind",
          "known-game-proof-beats-possible-game-proof",
          "block-beats-allow",
          "fresh-proof-beats-stale-proof",
          "protected-process-degrades-to-ask"
        ],
        "items": [
          {
            "ruleId": "allow-school-game-club",
            "enabled": true,
            "priority": 100,
            "target": {
              "kind": "game-title",
              "values": ["school-chess-club"],
              "matchMode": "exact"
            },
            "action": {
              "kind": "allow",
              "reasonCode": "school-club"
            },
            "proofRequirement": "foreground-known-game-session",
            "scheduleId": "school-club-hours",
            "budgetId": null,
            "auditLevel": "decision"
          },
          {
            "ruleId": "limit-entertainment-games",
            "enabled": true,
            "priority": 200,
            "target": {
              "kind": "game-category",
              "values": ["entertainment-game"],
              "matchMode": "category"
            },
            "action": {
              "kind": "limit",
              "budgetId": "games-daily-budget",
              "reasonCode": "game-budget"
            },
            "proofRequirement": "foreground-known-game-session",
            "scheduleId": "after-homework",
            "auditLevel": "decision-and-timer"
          },
          {
            "ruleId": "ask-unknown-game",
            "enabled": true,
            "priority": 300,
            "target": {
              "kind": "possibly-game",
              "values": ["possibly-game", "unknown-process", "launcher-game-candidate"],
              "matchMode": "any"
            },
            "action": {
              "kind": "ask",
              "approvalKind": "unknown-game",
              "reasonCode": "unknown-game-candidate"
            },
            "proofRequirement": "foreground-window",
            "scheduleId": "always",
            "auditLevel": "decision"
          },
          {
            "ruleId": "block-mature-rated-games-at-bedtime",
            "enabled": true,
            "priority": 400,
            "target": {
              "kind": "rating-threshold",
              "values": ["mature-17-plus"],
              "matchMode": "rating-at-or-above"
            },
            "action": {
              "kind": "ask",
              "approvalKind": "rating-threshold",
              "reasonCode": "mature-game-bedtime"
            },
            "proofRequirement": "rating-source-and-session",
            "scheduleId": "bedtime",
            "auditLevel": "decision"
          },
          {
            "ruleId": "terminate-after-budget",
            "enabled": true,
            "priority": 500,
            "target": {
              "kind": "game-session",
              "values": ["budget-exhausted"],
              "matchMode": "timer-state"
            },
            "action": {
              "kind": "terminate",
              "reasonCode": "time-limit-reached"
            },
            "proofRequirement": "current-accessible-process",
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
          "scheduleId": "school-club-hours",
          "kind": "weekly-window",
          "timezone": "America/Toronto",
          "windows": [
            {
              "days": ["wednesday"],
              "start": "15:30",
              "end": "17:30"
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
        "durationCountingMode": "foreground-game-time",
        "warningThresholdMinutes": 10,
        "graceMinutes": 5,
        "reset": "daily",
        "items": [
          {
            "budgetId": "games-daily-budget",
            "targetKind": "game-category",
            "targetValues": ["entertainment-game"],
            "minutes": 60,
            "period": "daily",
            "whenExhausted": "terminate-after-grace"
          },
          {
            "budgetId": "cloud-games-budget",
            "targetKind": "cloud-game-service",
            "targetValues": ["cloud-game-service"],
            "minutes": 30,
            "period": "daily",
            "whenExhausted": "ask"
          }
        ]
      },
      "approvals": {
        "requiredFor": ["blocked-game", "new-game", "unknown-game", "time-extension", "rating-threshold"],
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
        "showSaveProgressWarning": true,
        "hideParentDiagnostics": true
      },
      "reports": {
        "visibleFields": [
          "inventory-status",
          "running-now",
          "foreground-now",
          "recent-sessions",
          "daily-rollups",
          "unknown-candidates",
          "launcher-status",
          "rating-category",
          "approval-events",
          "block-results",
          "time-budget",
          "policy-decisions",
          "source-capability"
        ],
        "summaries": [
          "by-child",
          "by-device",
          "by-game",
          "by-launcher",
          "by-category",
          "by-rating",
          "by-unknown-candidate"
        ],
        "showExactTitleRequiresProof": true
      },
      "retention": {
        "rawEvidence": "7-days",
        "inventorySnapshot": "30-days",
        "sessionSummary": "90-days",
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
          "custody-label",
          "target-recheck"
        ],
        "auditEveryDecision": true,
        "auditEveryStrictAction": true,
        "auditFailedAdapterActions": true,
        "auditPolicyPreview": true
      },
      "platforms": {
        "windows": {
          "enabled": true,
          "allowedAdapters": [
            "process-observation",
            "foreground-window-observation",
            "installed-app-inventory",
            "store-package-inventory",
            "launcher-manifest-reader",
            "owned-process-termination"
          ],
          "manualRequiredAdapters": ["broad-app-control-blocking", "network-service-filtering", "anti-cheat-safe-proof"]
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
        "consoles": {
          "enabled": false,
          "state": "platform-family-controls-only"
        },
        "webPortal": {
          "authoringOnly": true,
          "mayRunCapture": false,
          "mayEnforce": false
        }
      },
      "fallbacks": {
        "inventoryUnavailable": "observe-running-only",
        "foregroundUnavailable": "running-time-report-only",
        "launcherManifestUnavailable": "use-process-evidence-only",
        "unknownGameCandidate": "ask",
        "ratingUnavailable": "ask",
        "protectedProcess": "parent-request-report-unavailable",
        "antiCheatLimited": "parent-request-report-unavailable",
        "terminateFailed": "rollback-and-audit",
        "blockLaunchUnavailable": "warn",
        "childDeviceOffline": "last-valid-local-policy",
        "platformUnsupported": "show-unavailable"
      }
    }
  },
  "effectivePolicy": {
    "documentId": "game-effective-mia-windows-laptop",
    "compiledFromPolicyId": "game-policy-mia-windows-laptop",
    "compiledFromRevision": 3,
    "schemaVersion": 1,
    "effectivePolicyHash": "sha256:worker-must-compute-sample",
    "compiledAt": "2026-05-28T00:00:00.000Z",
    "scope": {
      "familyId": "family-local-1",
      "childId": "child-mia",
      "deviceId": "device-windows-laptop",
      "platform": "windows"
    },
    "gameActivityDefaultDecision": "limit",
    "gameManagementEnabled": true,
    "inventoryDecision": {
      "mode": "local-installed-and-running",
      "allowedSources": [
        "process-snapshot",
        "foreground-window",
        "installed-app-records",
        "store-packages",
        "launcher-manifests"
      ],
      "unknownCandidateHandling": "parent-request-after-foreground-threshold",
      "ratingSources": ["store-metadata", "parent-catalog", "rating-authority"]
    },
    "durationDecision": {
      "countingMode": "foreground-game-time",
      "proofRequirement": "foreground-known-game-session",
      "allowRunningOnlyForReports": true,
      "allowPossibleGameForBudgets": false,
      "timerRecoveryRequired": true
    },
    "nativeGameDecision": {
      "defaultAction": "time-limit",
      "terminateAfterBudget": true,
      "graceSeconds": 60,
      "requireTargetRecheck": true,
      "skipProtectedOrAntiCheatLimited": true,
      "auditRequired": true
    },
    "launcherDecision": {
      "launcherOnlyHandling": "report-separately",
      "whenManifestUnavailable": "use-process-evidence-only",
      "supportedKinds": ["steam", "xbox-app", "microsoft-store", "epic-games", "roblox", "minecraft-launcher"]
    },
    "browserCloudDecision": {
      "browserGameHandling": "defer-to-browser-control-proof",
      "cloudGameHandling": "count-cloud-client-foreground-time",
      "networkHintHandling": "report-only"
    },
    "proofRequirements": {
      "knownGameRules": "foreground-known-game-session",
      "categoryRules": "known-game-or-rated-catalog-session",
      "ratingRules": "rating-source-and-session",
      "launcherRules": "launcher-attributed-session",
      "possibleGameRules": "foreground-window",
      "terminateRules": "current-accessible-process",
      "timeBudgets": "foreground-window-or-platform-activity",
      "browserGameRules": "managed-browser-game-url",
      "reportOnly": "stale-or-degraded-allowed"
    },
    "fallbackDecisions": {
      "proofUnavailable": "ask",
      "staleEvidence": "report-only",
      "protectedProcess": "parent-request-report-unavailable",
      "antiCheatLimited": "parent-request-report-unavailable",
      "platformUnsupported": "unavailable",
      "adapterError": "rollback-and-audit"
    },
    "rulesInPriorityOrder": [
      {
        "ruleId": "allow-school-game-club",
        "priority": 100,
        "decision": "allow",
        "targetKind": "game-title",
        "proofRequirement": "foreground-known-game-session",
        "scheduleId": "school-club-hours"
      },
      {
        "ruleId": "limit-entertainment-games",
        "priority": 200,
        "decision": "limit",
        "targetKind": "game-category",
        "proofRequirement": "foreground-known-game-session",
        "scheduleId": "after-homework",
        "budgetId": "games-daily-budget"
      },
      {
        "ruleId": "ask-unknown-game",
        "priority": 300,
        "decision": "ask",
        "targetKind": "possibly-game",
        "proofRequirement": "foreground-window",
        "scheduleId": "always"
      },
      {
        "ruleId": "block-mature-rated-games-at-bedtime",
        "priority": 400,
        "decision": "ask",
        "targetKind": "rating-threshold",
        "proofRequirement": "rating-source-and-session",
        "scheduleId": "bedtime"
      },
      {
        "ruleId": "terminate-after-budget",
        "priority": 500,
        "decision": "terminate",
        "targetKind": "game-session",
        "proofRequirement": "current-accessible-process",
        "scheduleId": "always"
      }
    ],
    "runtimeTables": {
      "schedulesById": {
        "always": {
          "kind": "always"
        },
        "school-club-hours": {
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
        "games-daily-budget": {
          "minutes": 60,
          "period": "daily",
          "whenExhausted": "terminate-after-grace"
        },
        "cloud-games-budget": {
          "minutes": 30,
          "period": "daily",
          "whenExhausted": "ask"
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
        "custody-label",
        "target-recheck"
      ]
    }
  },
  "updateProtocol": {
    "commands": [
      {
        "commandType": "game-policy.get.requested",
        "purpose": "Portal asks the child agent for current policy value and revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "includeAuthoringManifest": true,
          "includeEffectivePolicy": true
        }
      },
      {
        "commandType": "game-policy.preview.requested",
        "purpose": "Portal asks whether proposed changes validate and what effective policy would result.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "baseRevision": 3,
          "patch": [
            {
              "op": "replace",
              "path": "/gamePolicy/defaultPosture",
              "value": "ask"
            }
          ]
        },
        "responseShape": {
          "accepted": true,
          "wouldCreateRevision": 4,
          "effectivePolicyPreviewHash": "sha256:static-sample-token",
          "warnings": [],
          "unsupportedSettings": []
        }
      },
      {
        "commandType": "game-policy.patch.requested",
        "purpose": "Portal sends a small settings change with an expected revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 3,
          "patch": [
            {
              "op": "replace",
              "path": "/gamePolicy/budgets/defaultDailyMinutes",
              "value": 45
            }
          ],
          "reason": "parent-ui-change"
        },
        "acceptedResponseShape": {
          "eventType": "game-policy.patch.accepted",
          "newRevision": 4,
          "policyHash": "sha256:static-sample-token",
          "effectivePolicyHash": "sha256:static-sample-token",
          "requiresRestart": false,
          "unsupportedSettings": []
        },
        "rejectedResponseShape": {
          "eventType": "game-policy.patch.rejected",
          "currentRevision": 4,
          "reason": "revision-conflict",
          "validationErrors": []
        }
      },
      {
        "commandType": "game-policy.replace.requested",
        "purpose": "Portal sends a full policy replacement for setup, import, reset, or wizard save.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 3,
          "replacementPolicy": {
            "documentId": "game-policy-mia-windows-laptop",
            "policyKind": "game-control",
            "schemaVersion": 1,
            "revision": 4
          },
          "reason": "parent-wizard-save"
        }
      },
      {
        "commandType": "game-policy.rollback.requested",
        "purpose": "Parent asks child agent to roll back to previous valid revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "targetRevision": 3,
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
      "restoreActiveTimersAfterRestart": true,
      "enforceLocallyWhenPortalOffline": true,
      "rejectUnknownPaths": true,
      "rejectInvalidEnumValues": true,
      "rejectPartialLimitWithoutBudget": true,
      "rejectTerminateWithoutTargetRecheck": true,
      "rejectBlockLaunchWithoutCapabilityOrFallback": true,
      "rejectBrowserGameRuleWithoutBrowserProofOrFallback": true
    }
  },
  "capabilityRegistry": {
    "deviceId": "device-windows-laptop",
    "generatedAt": "2026-05-28T00:00:00.000Z",
    "platform": "windows",
    "capabilities": [
      {
        "capabilityId": "windows-process-observation",
        "state": "ready",
        "proof": "runtime-read-model-required",
        "affectsFields": ["inventory.sources", "evidence.requiredProof", "rules.allowedTargetTypes"]
      },
      {
        "capabilityId": "windows-foreground-window-observation",
        "state": "ready",
        "proof": "runtime-read-model-required",
        "affectsFields": ["evidence.requiredProof", "evidence.durationCountingMode", "budgets.enabled"]
      },
      {
        "capabilityId": "windows-installed-app-inventory",
        "state": "ready",
        "proof": "runtime-read-model-required",
        "affectsFields": ["inventory.mode", "inventory.sources", "nativeGames.allowedIdentityTypes"]
      },
      {
        "capabilityId": "launcher-manifest-reader",
        "state": "manual-required",
        "proof": "not-yet-proven-for-each-launcher",
        "affectsFields": ["launchers.supportedKinds", "inventory.sources", "rules.allowedTargetTypes"]
      },
      {
        "capabilityId": "owned-process-termination",
        "state": "ready",
        "proof": "runtime-adapter-proof-required",
        "affectsFields": ["nativeGames.strictActions", "rules.allowedActions"]
      },
      {
        "capabilityId": "broad-app-control-blocking",
        "state": "manual-required",
        "proof": "not-yet-proven",
        "affectsFields": ["nativeGames.strictActions", "rules.allowedActions", "game.defaultPosture"]
      },
      {
        "capabilityId": "anti-cheat-protected-process-handling",
        "state": "degraded",
        "proof": "must-not-bypass-record-limits-only",
        "affectsFields": ["nativeGames.strictActions", "audit.requiredFields"]
      },
      {
        "capabilityId": "managed-browser-game-proof",
        "state": "manual-required",
        "proof": "browser-control-boundary-required",
        "affectsFields": ["browserCloud.mode", "browserCloud.allowedEvidence", "rules.allowedTargetTypes"]
      },
      {
        "capabilityId": "platform-family-controls",
        "state": "manual-required",
        "proof": "platform-integration-required",
        "affectsFields": ["game.managementMode", "browserCloud.allowedEvidence", "rules.allowedActions"]
      }
    ]
  }
}
```

## Implementation Notes For Worker

- Start with domain contracts before Portal UI.
- Keep authoring manifest ids, field ids, section ids, option ids, policy ids,
  rule ids, schedule ids, budget ids, approval ids, game refs, launcher refs,
  and capability ids branded.
- Do not let Portal define arbitrary JSON paths. `writesTo` paths should be
  schema-known authoring paths.
- Use Effect Schema to validate the full policy after every patch.
- Compile the effective policy in the child-agent/service boundary, not in
  Portal.
- Persist both policy revision and compiled effective policy hash.
- Restore timer state from journal/query state after child-agent restart.
- Reject partial states. For example, `defaultPosture: "limit"` needs either a
  valid budget or a fallback decision.
- Treat native game, launcher game, browser game, cloud game, mobile game, and
  console game targets as separate proof families.
- Keep possible-game and unknown-process states visible. Do not promote them to
  known games without deterministic evidence or a separately labeled classifier
  result.
- Require target recheck before termination or block-launch execution.
- Treat protected-process and anti-cheat-limited targets as degraded or
  unavailable rather than trying to bypass platform protections.
- Treat the authoring manifest as UI guidance only. Runtime enforcement must
  rely on validated policy and compiled effective policy.
- Add explicit tests for hidden/visible branch behavior so UI cannot show strict
  block-launch controls when the platform capability is unavailable.
- Add explicit tests for offline behavior: child agent continues enforcing the
  last valid compiled policy when Portal is disconnected.
