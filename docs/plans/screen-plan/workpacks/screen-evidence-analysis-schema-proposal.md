<!-- agent-capsule -->

> Agent Capsule
> Doc: Screen Evidence Analysis Schema Proposal
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Screen Evidence Analysis Schema Proposal

Status: proposal for worker handoff. This is not final source code.

This document proposes a structured screen evidence analysis schema that can support:

- Portal-rendered question/option UI.
- Parent-authored screen analysis settings.
- Child-agent local persisted policy.
- Offline operation from the last valid policy.
- Local screenshot or frame capture only after explicit enablement.
- Encrypted temporary image queueing.
- Local OCR/vision analysis.
- Typed summary storage with evidence refs and deletion state.
- Small patch updates from Portal.
- Full policy replacement during setup/import/reset.
- Deterministic compile into an effective local execution plan.

The JSON in this document is intentionally product-shaped rather than repo-strict. The implementation
worker must not copy it directly into runtime code. The worker should use it as a guide and then build
proper Ocentra Parent contracts with:

- Effect Schema validation.
- Branded ids from schema brands, not manual brands.
- Decode helpers.
- No naked domain strings in app/runtime code.
- Tests for every parser, authoring manifest field, policy value shape, compile rule, patch command,
  capability state, queue state, deletion state, confidence value, and invalid-state rejection.
- Rust protocol parity only after the TypeScript contracts are explicit and test-backed.
- Local child-agent persistence, queue encryption/deletion, compile, rollback, and audit behavior.

## Architecture

The proposal has five related documents.

### Authoring Manifest

The authoring manifest tells Portal what questions to show, what controls to render, which options are
allowed, where the answer writes into the policy value document, and when the field is visible or enabled.

Portal must not invent screen analysis questions outside this manifest. If the UI needs a new question, the
manifest and value schema need a contract update.

### Policy Value Document

The policy value document is the parent-authored screen analysis policy. It is the durable source of parent
intent. The child agent validates it as a whole after any update.

### Effective Policy Document

The effective policy document is the compiled execution plan. The child agent uses it for scheduling,
capture gating, queue behavior, local analysis, policy eligibility, and audit. It should be deterministic,
flat enough for runtime, and explicit about fallback behavior when proof is unavailable.

### Policy Update Commands

Portal sends typed update commands. The child agent validates, persists, compiles, and acknowledges. Portal
is never in the capture, OCR/vision, policy, or enforcement hot path.

```text
Portal authoring UI
  -> policy update command
  -> child agent validates full policy value
  -> child agent persists policy revision
  -> child agent compiles effective policy
  -> child agent gates capture, queue, local analysis, summary storage, and policy handoff locally
```

### Capability Registry

The capability registry tells Portal and policy compilation which platform features are ready, disabled,
unsupported, permission-required, permission-limited, protected, degraded, or manual-required. It is runtime
state, not parent intent.

## Proposed Complete JSON Shape

The following JSON combines the proposed authoring manifest, policy value, effective policy, update
commands, and capability registry into one example so a worker can see how the pieces relate.

```json
{
  "schemaVersion": 1,
  "proposalStatus": "worker-handoff-proposal-not-runtime-source",
  "proposalIntent": "Guide implementation of screen evidence analysis authoring, storage, queue, local analysis, compile, policy handoff, and audit contracts.",
  "workerInstruction": {
    "takeAsGuideOnly": true,
    "mustTranslateToEffectSchema": true,
    "mustUseSchemaBrands": true,
    "mustAddDecodeHelpers": true,
    "mustAddTests": true,
    "mustPreserveLocalChildAgentExecution": true,
    "mustKeepRawCaptureLocalOnlyByDefault": true,
    "mustNotCopyJsonDirectlyIntoRuntime": true,
    "rustParityOnlyAfterTypeScriptContracts": true
  },
  "contractFamilies": {
    "authoringManifest": "Portal-rendered sections, questions, options, visibility, enabled state, writesTo paths, and validation hints.",
    "policyValue": "Parent-authored durable screen analysis policy stored and versioned by the child agent.",
    "effectivePolicy": "Compiled deterministic child-agent execution plan for capture gating, queue behavior, local analysis, policy eligibility, and audit.",
    "updateProtocol": "Typed get, preview, patch, replace, ack, reject, and rollback commands.",
    "capabilityRegistry": "Runtime device/platform/scope/local-model capability states used to hide, disable, or degrade fields."
  },
  "authoringManifest": {
    "manifestId": "screen-evidence-analysis-authoring-v1",
    "policyKind": "screen-evidence-analysis",
    "schemaVersion": 1,
    "title": "Screen evidence analysis",
    "renderingRules": {
      "hideInvisibleFields": true,
      "showDisabledFieldsWithReason": true,
      "neverInventFieldsOutsideManifest": true,
      "writeOnlyThroughWritesToPath": true,
      "previewBeforeApply": true,
      "showDisclosureBeforeEnable": true,
      "showCapabilityStateBesideSensitiveControls": true,
      "showRawCaptureRetentionAsOffByDefault": true
    },
    "controlKinds": [
      "boolean",
      "single-choice",
      "multi-choice",
      "number",
      "duration",
      "schedule",
      "retention",
      "target-list",
      "rule-list",
      "threshold",
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
      "proofAtLeast",
      "permissionStateIn"
    ],
    "sections": [
      {
        "sectionId": "screen-analysis-management",
        "title": "Screen analysis",
        "purpose": "Top-level enablement, product posture, and disclosure requirements.",
        "fields": [
          {
            "fieldId": "screen.enabled",
            "kind": "boolean",
            "question": "Enable local screen evidence analysis?",
            "writesTo": "/screenPolicy/enabled",
            "defaultValue": false,
            "uiPriority": 10,
            "whenFalse": {
              "policyMeaning": "Screen analysis is disabled, no capture jobs are scheduled, and screen summaries do not affect policy.",
              "hiddenSections": [
                "capture-scope",
                "recording",
                "scheduling",
                "triggers",
                "ocr-vision",
                "redaction",
                "queue-retention",
                "policy-use",
                "child-facing",
                "reports",
                "audit"
              ]
            }
          },
          {
            "fieldId": "screen.defaultPosture",
            "kind": "single-choice",
            "question": "How should screen analysis be used?",
            "writesTo": "/screenPolicy/defaultPosture",
            "defaultValue": "observe-only",
            "visibleWhen": {
              "path": "/screenPolicy/enabled",
              "equals": true
            },
            "options": [
              {
                "value": "observe-only",
                "label": "Observe only",
                "meaning": "Local summaries can be shown in reports but do not affect policy decisions."
              },
              {
                "value": "policy-dry-run",
                "label": "Policy preview",
                "meaning": "Local summaries can feed dry-run policy previews without enforcement."
              },
              {
                "value": "ask-parent",
                "label": "Ask parent",
                "meaning": "Matching screen summaries can create permission requests after deterministic policy evaluation."
              },
              {
                "value": "enforcement-eligible",
                "label": "Can enforce",
                "meaning": "Screen summaries may contribute to enforcement only after typed policy decisions, thresholds, and audit requirements pass."
              }
            ]
          },
          {
            "fieldId": "screen.managementMode",
            "kind": "single-choice",
            "question": "Where should screen analysis run?",
            "writesTo": "/screenPolicy/managementMode",
            "defaultValue": "local-child-agent",
            "visibleWhen": {
              "path": "/screenPolicy/enabled",
              "equals": true
            },
            "options": ["local-child-agent", "lan-live-child-agent", "authoring-only", "unavailable"]
          },
          {
            "fieldId": "screen.requiredDisclosure",
            "kind": "multi-choice",
            "question": "Which disclosure requirements apply before capture?",
            "writesTo": "/screenPolicy/disclosure/requiredStates",
            "defaultValue": [
              "parent-setting-visible",
              "child-facing-local-analysis-disclosure",
              "raw-capture-not-retained-by-default",
              "cloud-processing-disabled-by-default"
            ],
            "options": [
              "parent-setting-visible",
              "child-facing-local-analysis-disclosure",
              "capture-indicator-when-platform-provides-it",
              "raw-capture-not-retained-by-default",
              "cloud-processing-disabled-by-default",
              "report-custody-labels-visible"
            ]
          }
        ]
      },
      {
        "sectionId": "capture-scope",
        "title": "Capture scope",
        "purpose": "Select which visible scope may be captured when the platform proves it.",
        "visibleWhen": {
          "path": "/screenPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "capture.allowedScopes",
            "kind": "multi-choice",
            "question": "Which capture scopes are allowed?",
            "writesTo": "/screenPolicy/capture/allowedScopes",
            "defaultValue": ["active-window", "managed-browser-window"],
            "options": [
              "full-screen",
              "active-display",
              "active-window",
              "selected-app-window",
              "managed-browser-window",
              "manual-parent-test-only"
            ]
          },
          {
            "fieldId": "capture.defaultScope",
            "kind": "single-choice",
            "question": "What scope should be tried first?",
            "writesTo": "/screenPolicy/capture/defaultScope",
            "defaultValue": "active-window",
            "options": [
              "active-window",
              "managed-browser-window",
              "active-display",
              "full-screen",
              "manual-parent-test-only"
            ]
          },
          {
            "fieldId": "capture.protectedSurfaceBehavior",
            "kind": "single-choice",
            "question": "What should happen on protected surfaces?",
            "writesTo": "/screenPolicy/capture/protectedSurfaceBehavior",
            "defaultValue": "skip-and-audit",
            "options": ["skip-and-audit", "delete-partial-and-audit", "pause-until-clear", "mark-unavailable"]
          },
          {
            "fieldId": "capture.requireAppWindowCorrelation",
            "kind": "boolean",
            "question": "Require app or window evidence before policy can use screen summaries?",
            "writesTo": "/screenPolicy/capture/requireAppWindowCorrelation",
            "defaultValue": true
          },
          {
            "fieldId": "capture.requireManagedBrowserCorrelationForWebClaims",
            "kind": "boolean",
            "question": "Require managed browser evidence for exact web claims?",
            "writesTo": "/screenPolicy/capture/requireManagedBrowserCorrelationForWebClaims",
            "defaultValue": true
          }
        ]
      },
      {
        "sectionId": "recording",
        "title": "Screen recording",
        "purpose": "Keep recording disabled by default and restrict any future recording to explicit local, short-lived, proof-gated modes.",
        "visibleWhen": {
          "path": "/screenPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "recording.mode",
            "kind": "single-choice",
            "question": "Should screen recording be allowed?",
            "writesTo": "/screenPolicy/recording/mode",
            "defaultValue": "disabled",
            "options": [
              "disabled",
              "manual-parent-test-only",
              "short-local-buffer",
              "triggered-frame-sampling",
              "authoring-only-manual-required"
            ]
          },
          {
            "fieldId": "recording.maxSegmentSeconds",
            "kind": "duration",
            "question": "What is the maximum local recording segment length?",
            "writesTo": "/screenPolicy/recording/maxSegmentSeconds",
            "defaultValue": 15,
            "min": 1,
            "max": 60,
            "visibleWhen": {
              "path": "/screenPolicy/recording/mode",
              "includes": ["manual-parent-test-only", "short-local-buffer", "triggered-frame-sampling"]
            }
          },
          {
            "fieldId": "recording.frameSamplingMode",
            "kind": "single-choice",
            "question": "How may recording frames be used for analysis?",
            "writesTo": "/screenPolicy/recording/frameSamplingMode",
            "defaultValue": "no-recording",
            "options": ["no-recording", "sample-keyframes-only", "sample-at-trigger-boundary", "summarize-then-delete"]
          },
          {
            "fieldId": "recording.allowContinuousRecording",
            "kind": "boolean",
            "question": "Allow continuous screen recording?",
            "writesTo": "/screenPolicy/recording/allowContinuousRecording",
            "defaultValue": false,
            "mustRemainFalseForSchemaVersion": 1
          },
          {
            "fieldId": "recording.retentionMode",
            "kind": "single-choice",
            "question": "How should raw recording data be retained?",
            "writesTo": "/screenPolicy/recording/retentionMode",
            "defaultValue": "no-raw-video-retention",
            "options": ["no-raw-video-retention", "temporary-queue-only", "future-explicit-parent-controlled-retention"]
          }
        ]
      },
      {
        "sectionId": "scheduling",
        "title": "Scheduling",
        "purpose": "Bound cadence capture and strict mode.",
        "visibleWhen": {
          "path": "/screenPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "schedule.cadenceCaptureEnabled",
            "kind": "boolean",
            "question": "Enable scheduled capture?",
            "writesTo": "/screenPolicy/schedule/cadenceCaptureEnabled",
            "defaultValue": false
          },
          {
            "fieldId": "schedule.cadenceSeconds",
            "kind": "duration",
            "question": "How often may scheduled capture run?",
            "writesTo": "/screenPolicy/schedule/cadenceSeconds",
            "defaultValue": 300,
            "min": 60,
            "max": 3600,
            "visibleWhen": {
              "path": "/screenPolicy/schedule/cadenceCaptureEnabled",
              "equals": true
            }
          },
          {
            "fieldId": "schedule.strictModeEnabled",
            "kind": "boolean",
            "question": "Allow the shortest supported cadence?",
            "writesTo": "/screenPolicy/schedule/strictModeEnabled",
            "defaultValue": false,
            "visibleWhen": {
              "path": "/screenPolicy/schedule/cadenceCaptureEnabled",
              "equals": true
            }
          },
          {
            "fieldId": "schedule.activeSchedules",
            "kind": "schedule",
            "question": "When may screen analysis run?",
            "writesTo": "/screenPolicy/schedule/activeScheduleIds",
            "defaultValue": ["always"]
          },
          {
            "fieldId": "schedule.pauseConditions",
            "kind": "multi-choice",
            "question": "When should capture pause?",
            "writesTo": "/screenPolicy/schedule/pauseConditions",
            "defaultValue": [
              "screen-locked",
              "protected-surface",
              "permission-required",
              "queue-unavailable",
              "model-unavailable"
            ],
            "options": [
              "screen-locked",
              "protected-surface",
              "permission-required",
              "permission-limited",
              "queue-unavailable",
              "model-unavailable",
              "battery-saver",
              "metered-connection",
              "parent-paused"
            ]
          }
        ]
      },
      {
        "sectionId": "triggers",
        "title": "Triggers",
        "purpose": "Choose event-triggered capture reasons and debouncing.",
        "visibleWhen": {
          "path": "/screenPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "triggers.triggerCaptureEnabled",
            "kind": "boolean",
            "question": "Enable event-triggered capture?",
            "writesTo": "/screenPolicy/triggers/triggerCaptureEnabled",
            "defaultValue": true
          },
          {
            "fieldId": "triggers.enabledTriggers",
            "kind": "multi-choice",
            "question": "Which events may request screen analysis?",
            "writesTo": "/screenPolicy/triggers/enabledTriggers",
            "defaultValue": [
              "foreground-app-change",
              "managed-browser-url-change",
              "policy-ambiguity",
              "manual-parent-test-capture"
            ],
            "options": [
              "foreground-app-change",
              "active-window-change",
              "managed-browser-url-change",
              "app-game-foreground-start",
              "unusual-network-digest",
              "policy-ambiguity",
              "local-ai-uncertainty",
              "ask-parent-flow",
              "manual-parent-test-capture"
            ],
            "visibleWhen": {
              "path": "/screenPolicy/triggers/triggerCaptureEnabled",
              "equals": true
            }
          },
          {
            "fieldId": "triggers.debounceSeconds",
            "kind": "duration",
            "question": "How long should repeated triggers wait?",
            "writesTo": "/screenPolicy/triggers/debounceSeconds",
            "defaultValue": 120,
            "min": 15,
            "max": 900,
            "visibleWhen": {
              "path": "/screenPolicy/triggers/triggerCaptureEnabled",
              "equals": true
            }
          },
          {
            "fieldId": "triggers.maxJobsPerHour",
            "kind": "number",
            "question": "What is the maximum number of screen analysis jobs per hour?",
            "writesTo": "/screenPolicy/triggers/maxJobsPerHour",
            "defaultValue": 12,
            "min": 0,
            "max": 60
          }
        ]
      },
      {
        "sectionId": "ocr-vision",
        "title": "OCR and vision",
        "purpose": "Configure local OCR, image classification, confidence, and invalid-output behavior.",
        "visibleWhen": {
          "path": "/screenPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "analysis.localModelRequired",
            "kind": "boolean",
            "question": "Require local OCR/vision for screen analysis?",
            "writesTo": "/screenPolicy/analysis/localModelRequired",
            "defaultValue": true
          },
          {
            "fieldId": "analysis.allowedTasks",
            "kind": "multi-choice",
            "question": "Which local analysis tasks are allowed?",
            "writesTo": "/screenPolicy/analysis/allowedTasks",
            "defaultValue": ["visible-category-classification", "safety-indicator-classification"],
            "options": [
              "visible-category-classification",
              "safety-indicator-classification",
              "ocr-transient-only",
              "ocr-snippet-storage",
              "sensitive-region-redaction",
              "managed-window-classification"
            ]
          },
          {
            "fieldId": "analysis.ocrTextEnabled",
            "kind": "boolean",
            "question": "Store bounded OCR text snippets in summaries?",
            "writesTo": "/screenPolicy/analysis/ocrTextEnabled",
            "defaultValue": false
          },
          {
            "fieldId": "analysis.ocrTextSnippetLimit",
            "kind": "number",
            "question": "How many OCR snippets may be retained per summary?",
            "writesTo": "/screenPolicy/analysis/ocrTextSnippetLimit",
            "defaultValue": 3,
            "min": 0,
            "max": 10,
            "visibleWhen": {
              "path": "/screenPolicy/analysis/ocrTextEnabled",
              "equals": true
            }
          },
          {
            "fieldId": "analysis.maxSnippetCharacters",
            "kind": "number",
            "question": "What is the maximum length of each retained OCR snippet?",
            "writesTo": "/screenPolicy/analysis/maxSnippetCharacters",
            "defaultValue": 120,
            "min": 0,
            "max": 500,
            "visibleWhen": {
              "path": "/screenPolicy/analysis/ocrTextEnabled",
              "equals": true
            }
          },
          {
            "fieldId": "analysis.minimumPolicyConfidence",
            "kind": "threshold",
            "question": "What confidence is required before screen summaries can affect policy?",
            "writesTo": "/screenPolicy/analysis/minimumPolicyConfidence",
            "defaultValue": 0.8,
            "min": 0,
            "max": 1
          },
          {
            "fieldId": "analysis.invalidOutputBehavior",
            "kind": "single-choice",
            "question": "What if the local model returns invalid output?",
            "writesTo": "/screenPolicy/analysis/invalidOutputBehavior",
            "defaultValue": "delete-and-mark-invalid",
            "options": ["delete-and-mark-invalid", "retry-within-ttl", "mark-unavailable", "ask-parent"]
          }
        ]
      },
      {
        "sectionId": "redaction",
        "title": "Redaction",
        "purpose": "Keep raw sensitive data out of summaries, reports, exports, and diagnostics.",
        "visibleWhen": {
          "path": "/screenPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "redaction.mode",
            "kind": "single-choice",
            "question": "How should visible text and sensitive regions be redacted?",
            "writesTo": "/screenPolicy/redaction/mode",
            "defaultValue": "strict-local",
            "options": ["off", "summary-only", "strict-local", "credential-sensitive", "parent-review-required"]
          },
          {
            "fieldId": "redaction.neverStore",
            "kind": "multi-choice",
            "question": "What must never be stored in summaries or reports?",
            "writesTo": "/screenPolicy/redaction/neverStore",
            "defaultValue": [
              "passwords",
              "tokens",
              "payment-data",
              "private-keys",
              "recovery-codes",
              "raw-image-bytes",
              "raw-local-paths",
              "browser-secrets",
              "decrypted-payloads"
            ],
            "options": [
              "passwords",
              "tokens",
              "payment-data",
              "private-keys",
              "recovery-codes",
              "raw-image-bytes",
              "raw-local-paths",
              "browser-secrets",
              "cookies",
              "keystrokes",
              "decrypted-payloads",
              "microphone-audio",
              "camera-video"
            ]
          },
          {
            "fieldId": "redaction.whenRedactionUnavailable",
            "kind": "single-choice",
            "question": "What if redaction is unavailable?",
            "writesTo": "/screenPolicy/redaction/whenUnavailable",
            "defaultValue": "summary-only-not-policy-eligible",
            "options": ["summary-only-not-policy-eligible", "mark-invalid", "delete-and-audit", "ask-parent"]
          }
        ]
      },
      {
        "sectionId": "queue-retention",
        "title": "Queue and retention",
        "purpose": "Bound encrypted temporary image storage and deletion behavior.",
        "visibleWhen": {
          "path": "/screenPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "queue.temporaryImageTtlSeconds",
            "kind": "duration",
            "question": "How long may a temporary image remain queued?",
            "writesTo": "/screenPolicy/queue/temporaryImageTtlSeconds",
            "defaultValue": 300,
            "min": 30,
            "max": 1800
          },
          {
            "fieldId": "queue.maxRetryCount",
            "kind": "number",
            "question": "How many local analysis retries are allowed before deletion?",
            "writesTo": "/screenPolicy/queue/maxRetryCount",
            "defaultValue": 2,
            "min": 0,
            "max": 5
          },
          {
            "fieldId": "queue.deleteAfterSuccess",
            "kind": "boolean",
            "question": "Delete raw image after successful analysis?",
            "writesTo": "/screenPolicy/queue/deleteAfterSuccess",
            "defaultValue": true,
            "mustRemainTrueForSchemaVersion": 1
          },
          {
            "fieldId": "queue.deleteAfterExpiry",
            "kind": "boolean",
            "question": "Delete raw image after TTL expiry?",
            "writesTo": "/screenPolicy/queue/deleteAfterExpiry",
            "defaultValue": true,
            "mustRemainTrueForSchemaVersion": 1
          },
          {
            "fieldId": "queue.retainRawCapture",
            "kind": "boolean",
            "question": "Retain raw screenshots or recordings?",
            "writesTo": "/screenPolicy/queue/retainRawCapture",
            "defaultValue": false,
            "mustRemainFalseForSchemaVersion": 1
          },
          {
            "fieldId": "queue.hostedProcessingAllowed",
            "kind": "boolean",
            "question": "Allow Ocentra-hosted processing of child screen images?",
            "writesTo": "/screenPolicy/queue/hostedProcessingAllowed",
            "defaultValue": false,
            "mustRemainFalseForSchemaVersion": 1
          }
        ]
      },
      {
        "sectionId": "policy-use",
        "title": "Policy use",
        "purpose": "Choose how validated local summaries may influence deterministic policy.",
        "visibleWhen": {
          "path": "/screenPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "policy.policyUseEnabled",
            "kind": "boolean",
            "question": "Allow screen summaries to be used by policy?",
            "writesTo": "/screenPolicy/policyUse/enabled",
            "defaultValue": false
          },
          {
            "fieldId": "policy.allowedTargetTypes",
            "kind": "multi-choice",
            "question": "What screen-derived targets may policy match?",
            "writesTo": "/screenPolicy/policyUse/allowedTargetTypes",
            "defaultValue": ["visible-category", "risk-signal", "unknown-state"],
            "options": [
              "visible-category",
              "risk-signal",
              "ocr-snippet-presence",
              "unknown-state",
              "protected-surface",
              "capability-state"
            ],
            "visibleWhen": {
              "path": "/screenPolicy/policyUse/enabled",
              "equals": true
            }
          },
          {
            "fieldId": "policy.whenProofUnavailable",
            "kind": "single-choice",
            "question": "What if screen proof is unavailable?",
            "writesTo": "/screenPolicy/policyUse/whenProofUnavailable",
            "defaultValue": "mark-unavailable",
            "options": ["allow", "observe", "warn", "ask", "block-until-ready", "mark-unavailable"]
          },
          {
            "fieldId": "policy.requireEvidenceRefs",
            "kind": "multi-choice",
            "question": "Which evidence refs are required before policy use?",
            "writesTo": "/screenPolicy/policyUse/requiredEvidenceRefs",
            "defaultValue": ["screen-summary", "queue-deletion-state", "local-model-runtime", "parent-setting-version"],
            "options": [
              "screen-summary",
              "queue-deletion-state",
              "local-model-runtime",
              "foreground-app-window",
              "managed-browser-state",
              "app-game-session",
              "network-digest",
              "parent-setting-version",
              "policy-version"
            ]
          }
        ]
      },
      {
        "sectionId": "reports",
        "title": "Reports",
        "purpose": "Select parent-visible summary fields and custody labels.",
        "visibleWhen": {
          "path": "/screenPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "reports.visibleFields",
            "kind": "multi-choice",
            "question": "Which fields should parent reports show?",
            "writesTo": "/screenPolicy/reports/visibleFields",
            "defaultValue": [
              "setting-state",
              "capability-state",
              "capture-reason",
              "capture-scope",
              "category-candidates",
              "risk-signals",
              "confidence",
              "source-evidence-refs",
              "custody-label",
              "deletion-state"
            ],
            "options": [
              "setting-state",
              "capability-state",
              "capture-reason",
              "capture-scope",
              "category-candidates",
              "risk-signals",
              "confidence",
              "ocr-snippets",
              "redaction-notes",
              "source-evidence-refs",
              "local-model-runtime",
              "policy-decision-refs",
              "custody-label",
              "deletion-state",
              "image-digest"
            ]
          },
          {
            "fieldId": "reports.rawScreenshotDefaultVisible",
            "kind": "boolean",
            "question": "Show raw screenshots in parent reports by default?",
            "writesTo": "/screenPolicy/reports/rawScreenshotDefaultVisible",
            "defaultValue": false,
            "mustRemainFalseForSchemaVersion": 1
          }
        ]
      },
      {
        "sectionId": "audit",
        "title": "Audit",
        "purpose": "Require audit refs for settings, queue lifecycle, local analysis, policy, and deletion.",
        "visibleWhen": {
          "path": "/screenPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "audit.requiredFields",
            "kind": "multi-choice",
            "question": "Which audit fields are required?",
            "writesTo": "/screenPolicy/audit/requiredFields",
            "defaultValue": [
              "parent-setting-version",
              "capability-state",
              "capture-reason",
              "queue-job-id",
              "image-digest",
              "local-model-runtime",
              "validation-result",
              "deletion-state",
              "custody-label",
              "policy-decision-ref"
            ],
            "options": [
              "parent-setting-version",
              "capability-state",
              "capture-reason",
              "capture-scope",
              "queue-job-id",
              "image-digest",
              "local-model-runtime",
              "validation-result",
              "deletion-state",
              "custody-label",
              "policy-decision-ref",
              "enforcement-result-ref",
              "adapter-error",
              "permission-state"
            ]
          },
          {
            "fieldId": "audit.auditEveryCaptureAttempt",
            "kind": "boolean",
            "question": "Audit every capture attempt, including skipped attempts?",
            "writesTo": "/screenPolicy/audit/auditEveryCaptureAttempt",
            "defaultValue": true
          },
          {
            "fieldId": "audit.auditEveryDeleteFailure",
            "kind": "boolean",
            "question": "Audit every delete-pending or delete-failed state?",
            "writesTo": "/screenPolicy/audit/auditEveryDeleteFailure",
            "defaultValue": true
          }
        ]
      }
    ]
  },
  "policyValue": {
    "documentId": "screen-policy-mia-windows-laptop",
    "policyKind": "screen-evidence-analysis",
    "schemaVersion": 1,
    "revision": 7,
    "updatedAt": "2026-05-28T00:00:00.000Z",
    "updatedByParentRef": "parent-local-1",
    "scope": {
      "familyId": "family-local-1",
      "childId": "child-mia",
      "deviceId": "device-windows-laptop",
      "platform": "windows"
    },
    "screenPolicy": {
      "enabled": true,
      "defaultPosture": "policy-dry-run",
      "managementMode": "local-child-agent",
      "disclosure": {
        "requiredStates": [
          "parent-setting-visible",
          "child-facing-local-analysis-disclosure",
          "raw-capture-not-retained-by-default",
          "cloud-processing-disabled-by-default",
          "report-custody-labels-visible"
        ],
        "childFacingDisclosureEnabled": true,
        "parentReportDisclosureEnabled": true
      },
      "capture": {
        "allowedScopes": ["active-window", "managed-browser-window"],
        "defaultScope": "active-window",
        "protectedSurfaceBehavior": "skip-and-audit",
        "requireAppWindowCorrelation": true,
        "requireManagedBrowserCorrelationForWebClaims": true
      },
      "recording": {
        "mode": "disabled",
        "maxSegmentSeconds": 15,
        "frameSamplingMode": "no-recording",
        "allowContinuousRecording": false,
        "retentionMode": "no-raw-video-retention"
      },
      "schedule": {
        "cadenceCaptureEnabled": false,
        "cadenceSeconds": 300,
        "strictModeEnabled": false,
        "activeScheduleIds": ["always"],
        "pauseConditions": [
          "screen-locked",
          "protected-surface",
          "permission-required",
          "queue-unavailable",
          "model-unavailable"
        ]
      },
      "triggers": {
        "triggerCaptureEnabled": true,
        "enabledTriggers": [
          "foreground-app-change",
          "managed-browser-url-change",
          "policy-ambiguity",
          "manual-parent-test-capture"
        ],
        "debounceSeconds": 120,
        "maxJobsPerHour": 12
      },
      "analysis": {
        "localModelRequired": true,
        "allowedTasks": [
          "visible-category-classification",
          "safety-indicator-classification",
          "ocr-transient-only",
          "sensitive-region-redaction"
        ],
        "ocrTextEnabled": false,
        "ocrTextSnippetLimit": 0,
        "maxSnippetCharacters": 0,
        "minimumPolicyConfidence": 0.8,
        "invalidOutputBehavior": "delete-and-mark-invalid",
        "allowedVisibleCategories": [
          "school",
          "video",
          "chat",
          "game",
          "shopping",
          "productivity",
          "adult-content",
          "violence",
          "bypass-tool",
          "unknown"
        ],
        "allowedRiskSignals": [
          "possible-credential-prompt",
          "explicit-content-signal",
          "bypass-tool-signal",
          "unsafe-visible-content",
          "self-harm-signal",
          "unknown"
        ]
      },
      "redaction": {
        "mode": "strict-local",
        "neverStore": [
          "passwords",
          "tokens",
          "payment-data",
          "private-keys",
          "recovery-codes",
          "raw-image-bytes",
          "raw-local-paths",
          "browser-secrets",
          "decrypted-payloads"
        ],
        "whenUnavailable": "summary-only-not-policy-eligible"
      },
      "queue": {
        "temporaryImageTtlSeconds": 300,
        "maxRetryCount": 2,
        "deleteAfterSuccess": true,
        "deleteAfterExpiry": true,
        "retainRawCapture": false,
        "hostedProcessingAllowed": false,
        "custodyState": "child-device-temp-queue"
      },
      "policyUse": {
        "enabled": true,
        "allowedTargetTypes": ["visible-category", "risk-signal", "unknown-state"],
        "whenProofUnavailable": "mark-unavailable",
        "requiredEvidenceRefs": [
          "screen-summary",
          "queue-deletion-state",
          "local-model-runtime",
          "parent-setting-version"
        ],
        "minimumConfidence": 0.8,
        "lowConfidenceFallback": "ask-parent",
        "protectedSurfaceFallback": "mark-unavailable",
        "invalidOutputFallback": "mark-unavailable"
      },
      "rules": [
        {
          "ruleId": "parent-request-explicit-content-signal",
          "enabled": true,
          "priority": 100,
          "target": {
            "kind": "risk-signal",
            "values": ["explicit-content-signal"],
            "matchMode": "any"
          },
          "action": {
            "kind": "ask",
            "approvalKind": "screen-risk",
            "reasonCode": "screen-explicit-content"
          },
          "proofRequirement": "validated-screen-summary-with-deleted-image",
          "minimumConfidence": 0.8,
          "scheduleId": "always",
          "auditLevel": "decision-and-deletion"
        },
        {
          "ruleId": "warn-bypass-tool-visible",
          "enabled": true,
          "priority": 200,
          "target": {
            "kind": "visible-category",
            "values": ["bypass-tool"],
            "matchMode": "any"
          },
          "action": {
            "kind": "warn",
            "reasonCode": "visible-bypass-tool"
          },
          "proofRequirement": "validated-screen-summary-with-source-correlation",
          "minimumConfidence": 0.75,
          "scheduleId": "always",
          "auditLevel": "decision"
        },
        {
          "ruleId": "unknown-screen-state-ask",
          "enabled": true,
          "priority": 900,
          "target": {
            "kind": "unknown-state",
            "values": ["low-confidence", "source-evidence-missing"],
            "matchMode": "any"
          },
          "action": {
            "kind": "ask",
            "approvalKind": "unknown-screen-context",
            "reasonCode": "screen-analysis-unknown"
          },
          "proofRequirement": "screen-analysis-attempted",
          "minimumConfidence": 0,
          "scheduleId": "always",
          "auditLevel": "decision"
        }
      ],
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
        }
      ],
      "reports": {
        "visibleFields": [
          "setting-state",
          "capability-state",
          "capture-reason",
          "capture-scope",
          "category-candidates",
          "risk-signals",
          "confidence",
          "source-evidence-refs",
          "custody-label",
          "deletion-state"
        ],
        "summaries": [
          "by-child",
          "by-device",
          "by-visible-category",
          "by-risk-signal",
          "by-unknown-state",
          "by-capability-state"
        ],
        "rawScreenshotDefaultVisible": false
      },
      "retention": {
        "rawCapture": "temporary-queue-only",
        "temporaryImageTtlSeconds": 300,
        "screenSummary": "30-days",
        "policyAudit": "90-days",
        "deleteExpired": true,
        "keepRedactedReport": true
      },
      "custody": {
        "allowedUses": ["child-local", "lan-live", "parent-cache", "parent-owned-export", "parent-report"],
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
        "allowRawCapture": false,
        "requiresManualReview": true,
        "fallbackWhenUnavailable": "manual-view"
      },
      "audit": {
        "requiredFields": [
          "parent-setting-version",
          "capability-state",
          "capture-reason",
          "queue-job-id",
          "image-digest",
          "local-model-runtime",
          "validation-result",
          "deletion-state",
          "custody-label",
          "policy-decision-ref"
        ],
        "auditEveryCaptureAttempt": true,
        "auditEveryStrictAction": true,
        "auditEveryDeleteFailure": true,
        "auditPolicyPreview": true
      },
      "platforms": {
        "windows": {
          "enabled": true,
          "allowedAdapters": [
            "windows-graphics-capture",
            "foreground-window-observation",
            "managed-browser-window-correlation",
            "encrypted-temp-queue",
            "local-ocr-vision"
          ],
          "manualRequiredAdapters": ["full-screen-capture-proof", "protected-surface-proof", "recording-stream-proof"]
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
          "state": "unavailable-for-hidden-capture"
        },
        "webPortal": {
          "authoringOnly": true,
          "mayRunCapture": false,
          "mayRunOcrVision": false,
          "mayReadRawQueue": false
        }
      },
      "fallbacks": {
        "permissionRequired": "show-setup-required",
        "permissionDenied": "mark-unavailable",
        "permissionLimited": "mark-degraded",
        "unsupportedScope": "fall-back-to-supported-scope-or-unavailable",
        "protectedSurface": "skip-and-audit",
        "screenLocked": "skip-and-audit",
        "queueUnavailable": "fail-closed",
        "modelUnavailable": "retry-within-ttl-then-delete",
        "redactionUnavailable": "summary-only-not-policy-eligible",
        "invalidModelOutput": "delete-and-mark-invalid",
        "lowConfidence": "ask-parent",
        "deleteFailed": "surface-health-and-retry",
        "childDeviceOffline": "last-known-report-only",
        "platformUnsupported": "show-unavailable"
      }
    }
  },
  "effectivePolicy": {
    "documentId": "screen-effective-mia-windows-laptop",
    "compiledFromPolicyId": "screen-policy-mia-windows-laptop",
    "compiledFromRevision": 7,
    "schemaVersion": 1,
    "effectivePolicyHash": "sha256:worker-must-compute-sample",
    "compiledAt": "2026-05-28T00:00:00.000Z",
    "scope": {
      "familyId": "family-local-1",
      "childId": "child-mia",
      "deviceId": "device-windows-laptop",
      "platform": "windows"
    },
    "screenAnalysisEnabled": true,
    "defaultDecisionMode": "policy-dry-run",
    "capturePlan": {
      "allowedScopesInOrder": ["active-window", "managed-browser-window"],
      "defaultScope": "active-window",
      "requireAppWindowCorrelation": true,
      "requireManagedBrowserCorrelationForWebClaims": true,
      "protectedSurfaceDecision": "skip-and-audit",
      "permissionRequiredDecision": "show-setup-required"
    },
    "recordingPlan": {
      "mode": "disabled",
      "maxSegmentSeconds": 15,
      "frameSamplingMode": "no-recording",
      "allowContinuousRecording": false,
      "retentionMode": "no-raw-video-retention",
      "recordingRequiresSeparateProof": true
    },
    "schedulePlan": {
      "cadenceCaptureEnabled": false,
      "cadenceSeconds": 300,
      "strictModeEnabled": false,
      "activeScheduleIds": ["always"],
      "pauseConditions": [
        "screen-locked",
        "protected-surface",
        "permission-required",
        "queue-unavailable",
        "model-unavailable"
      ]
    },
    "triggerPlan": {
      "enabled": true,
      "enabledTriggers": [
        "foreground-app-change",
        "managed-browser-url-change",
        "policy-ambiguity",
        "manual-parent-test-capture"
      ],
      "debounceSeconds": 120,
      "maxJobsPerHour": 12
    },
    "queuePlan": {
      "temporaryImageTtlSeconds": 300,
      "maxRetryCount": 2,
      "deleteAfterSuccess": true,
      "deleteAfterExpiry": true,
      "retainRawCapture": false,
      "hostedProcessingAllowed": false,
      "startupCleanupRequired": true,
      "failClosedIfEncryptionUnavailable": true
    },
    "analysisPlan": {
      "localModelRequired": true,
      "allowedTasks": [
        "visible-category-classification",
        "safety-indicator-classification",
        "ocr-transient-only",
        "sensitive-region-redaction"
      ],
      "storeOcrSnippets": false,
      "minimumPolicyConfidence": 0.8,
      "invalidOutputDecision": "delete-and-mark-invalid",
      "confidenceRange": {
        "min": 0,
        "max": 1,
        "rejectNaN": true,
        "rejectInfinity": true
      }
    },
    "redactionPlan": {
      "mode": "strict-local",
      "neverStore": [
        "passwords",
        "tokens",
        "payment-data",
        "private-keys",
        "recovery-codes",
        "raw-image-bytes",
        "raw-local-paths",
        "browser-secrets",
        "decrypted-payloads"
      ],
      "whenUnavailable": "summary-only-not-policy-eligible"
    },
    "proofRequirements": {
      "screenPolicyUse": "validated-screen-summary-with-deleted-image",
      "exactWebClaims": "managed-browser-evidence-required",
      "appWindowClaims": "foreground-app-window-evidence-required",
      "riskSignalRules": "validated-screen-summary-confidence-threshold",
      "enforcementEligibility": "typed-policy-decision-with-screen-evidence-ref",
      "reportOnly": "stale-or-degraded-allowed"
    },
    "fallbackDecisions": {
      "proofUnavailable": "mark-unavailable",
      "staleEvidence": "report-only",
      "screenLocked": "skip-and-audit",
      "protectedSurface": "skip-and-audit",
      "modelUnavailable": "retry-within-ttl-then-delete",
      "queueUnavailable": "fail-closed",
      "adapterError": "mark-degraded-and-audit",
      "deleteFailed": "surface-health-and-retry"
    },
    "rulesInPriorityOrder": [
      {
        "ruleId": "parent-request-explicit-content-signal",
        "priority": 100,
        "decision": "ask",
        "targetKind": "risk-signal",
        "proofRequirement": "validated-screen-summary-with-deleted-image",
        "minimumConfidence": 0.8,
        "scheduleId": "always"
      },
      {
        "ruleId": "warn-bypass-tool-visible",
        "priority": 200,
        "decision": "warn",
        "targetKind": "visible-category",
        "proofRequirement": "validated-screen-summary-with-source-correlation",
        "minimumConfidence": 0.75,
        "scheduleId": "always"
      },
      {
        "ruleId": "unknown-screen-state-ask",
        "priority": 900,
        "decision": "ask",
        "targetKind": "unknown-state",
        "proofRequirement": "screen-analysis-attempted",
        "minimumConfidence": 0,
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
        }
      },
      "visibleCategoryPolicyTargets": {
        "school": {
          "defaultAction": "observe"
        },
        "video": {
          "defaultAction": "observe"
        },
        "chat": {
          "defaultAction": "observe"
        },
        "game": {
          "defaultAction": "observe"
        },
        "adult-content": {
          "defaultAction": "ask"
        },
        "violence": {
          "defaultAction": "ask"
        },
        "bypass-tool": {
          "defaultAction": "warn"
        },
        "unknown": {
          "defaultAction": "ask"
        }
      }
    },
    "auditPlan": {
      "auditEveryCaptureAttempt": true,
      "auditEveryDecision": true,
      "auditEveryStrictAction": true,
      "auditEveryDeleteFailure": true,
      "requiredFields": [
        "parent-setting-version",
        "capability-state",
        "capture-reason",
        "capture-scope",
        "queue-job-id",
        "image-digest",
        "local-model-runtime",
        "validation-result",
        "deletion-state",
        "custody-label",
        "policy-decision-ref"
      ]
    }
  },
  "updateProtocol": {
    "commands": [
      {
        "commandType": "screen-policy.get.requested",
        "purpose": "Portal asks the child agent for current screen policy value, effective policy, capability registry, and revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "includeAuthoringManifest": true,
          "includeEffectivePolicy": true,
          "includeCapabilityRegistry": true
        }
      },
      {
        "commandType": "screen-policy.preview.requested",
        "purpose": "Portal asks whether proposed changes validate and what effective policy would result.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "baseRevision": 7,
          "patch": [
            {
              "op": "replace",
              "path": "/screenPolicy/defaultPosture",
              "value": "enforcement-eligible"
            }
          ]
        },
        "responseShape": {
          "accepted": true,
          "wouldCreateRevision": 8,
          "effectivePolicyPreviewHash": "sha256:static-sample-token",
          "warnings": ["enforcement-eligible requires typed policy decision and screen summary deletion proof"],
          "unsupportedSettings": []
        }
      },
      {
        "commandType": "screen-policy.patch.requested",
        "purpose": "Portal sends a small settings change with an expected revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 7,
          "patch": [
            {
              "op": "replace",
              "path": "/screenPolicy/triggers/maxJobsPerHour",
              "value": 6
            }
          ],
          "reason": "parent-ui-change"
        },
        "acceptedResponseShape": {
          "eventType": "screen-policy.patch.accepted",
          "newRevision": 8,
          "policyHash": "sha256:static-sample-token",
          "effectivePolicyHash": "sha256:static-sample-token",
          "requiresRestart": false,
          "unsupportedSettings": []
        },
        "rejectedResponseShape": {
          "eventType": "screen-policy.patch.rejected",
          "currentRevision": 8,
          "reason": "revision-conflict",
          "validationErrors": []
        }
      },
      {
        "commandType": "screen-policy.replace.requested",
        "purpose": "Portal sends a full policy replacement for setup, import, reset, or wizard save.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 7,
          "replacementPolicy": {
            "documentId": "screen-policy-mia-windows-laptop",
            "policyKind": "screen-evidence-analysis",
            "schemaVersion": 1,
            "revision": 8
          },
          "reason": "parent-wizard-save"
        }
      },
      {
        "commandType": "screen-policy.manual-test-capture.requested",
        "purpose": "Parent requests one explicit setup/test capture through the child agent.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "expectedRevision": 7,
          "captureReason": "manual-parent-test-capture",
          "requestedScope": "active-window",
          "requireDeletionProof": true
        },
        "acceptedResponseShape": {
          "eventType": "screen-policy.manual-test-capture.accepted",
          "queueJobId": "screen-job-id-sample-0001",
          "captureWillRunLocally": true,
          "rawCaptureRetention": false
        },
        "rejectedResponseShape": {
          "eventType": "screen-policy.manual-test-capture.rejected",
          "reason": "permission-required",
          "capabilityState": "permission-required"
        }
      },
      {
        "commandType": "screen-policy.rollback.requested",
        "purpose": "Parent asks child agent to roll back to previous valid revision.",
        "requestShape": {
          "targetDeviceId": "device-windows-laptop",
          "targetRevision": 6,
          "reason": "parent-rollback"
        }
      }
    ],
    "agentRules": {
      "validateFullPolicyAfterPatch": true,
      "compileFullEffectivePolicyAfterEveryAcceptedChange": true,
      "persistPolicyBeforeSchedulerSwitch": true,
      "keepPreviousValidRevision": true,
      "rollbackOnCompileFailure": true,
      "runCaptureOnlyInChildAgent": true,
      "runOcrVisionOnlyInChildAgent": true,
      "enforceLocallyWhenPortalOffline": true,
      "rejectUnknownPaths": true,
      "rejectInvalidEnumValues": true,
      "rejectConfidenceOutsideZeroOne": true,
      "rejectRetainRawCaptureForSchemaVersionOne": true,
      "rejectHostedProcessingForSchemaVersionOne": true,
      "rejectPolicyUseWithoutDeletionProof": true,
      "rejectExactWebClaimsWithoutManagedBrowserEvidence": true,
      "deleteQueuedImagesOnInvalidOutput": true,
      "recordSkippedAttemptsAsCapabilityEvents": true
    }
  },
  "capabilityRegistry": {
    "deviceId": "device-windows-laptop",
    "generatedAt": "2026-05-28T00:00:00.000Z",
    "platform": "windows",
    "capabilities": [
      {
        "capabilityId": "windows-graphics-capture-active-window",
        "capabilityKind": "capture-scope",
        "state": "manual-required",
        "proof": "real-host-permission-and-capture-proof-required",
        "affectsFields": ["capture.allowedScopes", "capture.defaultScope", "policy.allowedTargetTypes"]
      },
      {
        "capabilityId": "windows-graphics-capture-full-screen",
        "capabilityKind": "capture-scope",
        "state": "manual-required",
        "proof": "real-host-permission-and-capture-proof-required",
        "affectsFields": ["capture.allowedScopes", "schedule.cadenceCaptureEnabled"]
      },
      {
        "capabilityId": "windows-graphics-capture-recording-stream",
        "capabilityKind": "recording",
        "state": "manual-required",
        "proof": "real-host-recording-stream-retention-and-deletion-proof-required",
        "affectsFields": ["recording.mode", "recording.maxSegmentSeconds", "recording.frameSamplingMode"]
      },
      {
        "capabilityId": "managed-browser-window-correlation",
        "capabilityKind": "source-correlation",
        "state": "ready",
        "proof": "runtime-read-model-required",
        "affectsFields": ["capture.requireManagedBrowserCorrelationForWebClaims", "policy.requireEvidenceRefs"]
      },
      {
        "capabilityId": "foreground-app-window-correlation",
        "capabilityKind": "source-correlation",
        "state": "ready",
        "proof": "runtime-read-model-required",
        "affectsFields": ["capture.requireAppWindowCorrelation", "policy.requireEvidenceRefs"]
      },
      {
        "capabilityId": "encrypted-screen-temp-queue",
        "capabilityKind": "queue",
        "state": "ready",
        "proof": "queue-encryption-deletion-tests-required",
        "affectsFields": [
          "queue.temporaryImageTtlSeconds",
          "queue.maxRetryCount",
          "queue.deleteAfterSuccess",
          "queue.deleteAfterExpiry"
        ]
      },
      {
        "capabilityId": "local-ocr-runtime",
        "capabilityKind": "local-analysis",
        "state": "manual-required",
        "proof": "local-model-runtime-proof-required",
        "affectsFields": ["analysis.allowedTasks", "analysis.ocrTextEnabled", "analysis.minimumPolicyConfidence"]
      },
      {
        "capabilityId": "local-vision-classifier",
        "capabilityKind": "local-analysis",
        "state": "manual-required",
        "proof": "local-model-runtime-proof-required",
        "affectsFields": ["analysis.allowedTasks", "analysis.minimumPolicyConfidence", "policy.allowedTargetTypes"]
      },
      {
        "capabilityId": "screen-redaction-runtime",
        "capabilityKind": "redaction",
        "state": "manual-required",
        "proof": "redaction-validation-required",
        "affectsFields": ["redaction.mode", "redaction.neverStore", "redaction.whenUnavailable"]
      },
      {
        "capabilityId": "macos-screencapturekit",
        "capabilityKind": "platform-capture",
        "state": "manual-required",
        "proof": "macos-host-screen-recording-permission-proof-required",
        "affectsFields": ["capture.allowedScopes"]
      },
      {
        "capabilityId": "linux-xdg-desktop-portal-screencast",
        "capabilityKind": "platform-capture",
        "state": "manual-required",
        "proof": "distro-desktop-portal-pipewire-proof-required",
        "affectsFields": ["capture.allowedScopes"]
      },
      {
        "capabilityId": "android-media-projection",
        "capabilityKind": "platform-capture",
        "state": "manual-required",
        "proof": "android-user-consent-foreground-service-proof-required",
        "affectsFields": ["capture.allowedScopes", "screen.requiredDisclosure", "recording.mode"]
      },
      {
        "capabilityId": "ios-screentime-managed-settings",
        "capabilityKind": "platform-policy",
        "state": "manual-required",
        "proof": "apple-entitlement-and-device-proof-required",
        "affectsFields": ["policy.allowedTargetTypes", "reports.visibleFields"]
      }
    ],
    "stateMeanings": {
      "ready": "Runtime reports the capability can be used within the configured boundary, subject to per-attempt checks.",
      "disabled-by-parent": "Parent setting disables the feature.",
      "unsupported-platform": "Current platform cannot support this capability in the current build.",
      "unsupported-scope": "The requested full-screen, display, window, app, or managed-window scope is unavailable.",
      "permission-required": "OS permission, user consent, management state, or entitlement is required before capture.",
      "permission-limited": "Permission exists but does not cover the requested scope.",
      "protected-surface": "Secure, locked, credential, DRM, or OS-protected surface prevents usable capture.",
      "model-unavailable": "Local OCR/vision runtime is missing, disabled, loading, failed, or overloaded.",
      "queue-unavailable": "Encrypted temporary queue cannot be opened or validated.",
      "degraded": "The capability can run with reduced scope, fidelity, freshness, or confidence.",
      "adapter-error": "The platform adapter failed and must record an audit result.",
      "manual-required": "Contracts can represent the setting, but product support requires real host/device proof."
    }
  }
}
```

## Implementation Notes For Worker

- Start with domain contracts before Portal UI.
- Keep authoring manifest ids, field ids, section ids, option ids, policy ids, rule ids, schedule ids,
  trigger ids, queue job ids, result ids, capability ids, custody labels, and evidence refs branded.
- Do not let Portal define arbitrary JSON paths. `writesTo` paths should be schema-known authoring paths.
- Use Effect Schema to validate the full policy after every patch.
- Compile the effective policy in the child-agent/service boundary, not in Portal.
- Persist both policy revision and compiled effective policy hash.
- Reject partial states. For example, `policyUse.enabled: true` requires a valid confidence threshold,
  deletion proof requirement, evidence refs, and fallback behavior.
- Treat the authoring manifest as UI guidance only. Runtime capture, queue, analysis, policy, and
  enforcement must rely on validated policy and compiled effective policy.
- Keep `retainRawCapture` and `hostedProcessingAllowed` false for this schema version.
- Add explicit tests for hidden/visible branch behavior so UI cannot show cadence, OCR snippet storage,
  strict mode, or enforcement eligibility controls when screen analysis is disabled.
- Add explicit tests for offline behavior: child agent continues using the last valid compiled policy when
  Portal is disconnected.
- Add explicit tests that invalid confidence, missing source refs, missing deletion state, protected
  surfaces, and delete failures cannot produce enforcement-eligible screen summaries.
- Add Rust parity only for Rust-crossing commands/events after TypeScript contracts and tests are stable.

## Source References

Local planning references:

- [Screen Evidence Analysis Capability Guide](../../../plans/screen-plan/workpacks/screen-evidence-analysis-capability-guide.md)
- [Local Screen Evidence Analysis Queue Architecture](architecture/local-screen-evidence-analysis-queue.md)
- [Screen Evidence Analysis Expectations](expectations/screen-evidence.md)
- [AI Feature Expectations](expectations/ai.md)
- [Policy Feature Expectations](expectations/policy.md)
- [Enforcement Feature Expectations](expectations/enforcement.md)
- [Data Custody And Local-First Expectations](expectations/data-custody.md)
- [Real Evidence Proof Expectations](expectations/real-evidence-proof.md)
- [Ocentra Parent Product Roadmap](product-roadmap.md)
