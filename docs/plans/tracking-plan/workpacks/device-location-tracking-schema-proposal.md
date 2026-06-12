<!-- agent-capsule -->

> Agent Capsule
> Doc: Device Location Tracking Schema Proposal
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Device Location Tracking Schema Proposal

Status: proposal for worker handoff. This is not final source code.

This document proposes a structured device location schema that can support:

- Portal-rendered question/option UI.
- Parent-authored device location settings.
- Child-agent local persisted policy.
- Offline operation from the last valid policy.
- Live map sessions when capability proof exists.
- Geofence, arrival/departure, check-in, and missing-device flows.
- Small patch updates from Portal.
- Full policy replacement during setup/import/reset.
- Deterministic compile into an effective local execution plan.

The JSON in this document is intentionally product-shaped rather than
repo-strict. The implementation worker must not copy it directly into runtime
code. The worker should use it as a guide and then build proper Ocentra Parent
contracts with:

- Effect Schema validation.
- Branded ids from schema brands, not manual brands.
- Decode helpers.
- No naked domain strings in app/runtime code.
- Tests for every parser, authoring manifest field, policy value shape, compile
  rule, patch command, and invalid-state rejection.
- Rust protocol parity only after the TypeScript contracts are explicit and
  test-backed.
- Local child-agent persistence, compile, rollback, retention, and audit
  behavior.

## Architecture

The proposal has four related documents.

### Authoring Manifest

The authoring manifest tells Portal what questions to show, what controls to
render, which options are allowed, where the answer writes into the policy value
document, and when the field is visible or enabled.

Portal must not invent device location policy questions outside this manifest.
If the UI needs a new question, the manifest and value schema need a contract
update.

### Policy Value Document

The policy value document is the parent-authored device location policy. It is
the durable source of parent intent. The child agent validates it as a whole
after any update.

### Effective Policy Document

The effective policy document is the compiled execution plan. The child agent
uses it for local runtime behavior. It should be deterministic, flat enough for
runtime, and explicit about fallback behavior when proof is unavailable.

### Policy Update Commands

Portal sends typed update commands. The child agent validates, persists,
compiles, and acknowledges. Portal is never in the tracking, policy, or
notification hot path.

```text
Portal authoring UI
  -> policy update command
  -> child agent validates full policy value
  -> child agent persists policy revision
  -> child agent compiles effective policy
  -> child agent samples, checks in, geofences, reports, or degrades locally
```

## Proposed Complete JSON Shape

The following JSON combines the proposed authoring manifest, policy value,
effective policy, update commands, and capability registry into one example so a
worker can see how the pieces relate.

```json
{
  "schemaVersion": 1,
  "proposalStatus": "design-proposal-not-runtime-contract",
  "proposalIntent": "Guide the implementation of device location authoring, storage, compile, reporting, geofence, check-in, and live-session contracts.",
  "workerInstruction": {
    "takeAsGuideOnly": true,
    "mustTranslateToEffectSchema": true,
    "mustUseSchemaBrands": true,
    "mustAddDecodeHelpers": true,
    "mustAddTests": true,
    "mustPreserveLocalChildAgentExecution": true,
    "mustNotCopyJsonDirectlyIntoRuntime": true,
    "mustAddRustParityOnlyAfterTypeScriptContracts": true
  },
  "contractFamilies": {
    "authoringManifest": "Portal-rendered sections, questions, options, visibility, enabled state, writesTo paths, and validation hints.",
    "policyValue": "Parent-authored durable device location policy stored and versioned by the child agent.",
    "effectivePolicy": "Compiled deterministic child-agent execution plan for sampling, geofence, check-in, live sessions, reports, fallbacks, custody, and audit.",
    "updateProtocol": "Typed get, preview, patch, replace, live-session, check-in, geofence, ack, reject, and rollback commands.",
    "capabilityRegistry": "Runtime device/platform/location capability states used to hide, disable, or degrade fields."
  },
  "authoringManifest": {
    "manifestId": "device-location-authoring-v1",
    "policyKind": "device-location-tracking",
    "schemaVersion": 1,
    "title": "Device location",
    "renderingRules": {
      "hideInvisibleFields": true,
      "showDisabledFieldsWithReason": true,
      "neverInventFieldsOutsideManifest": true,
      "writeOnlyThroughWritesToPath": true,
      "previewBeforeApply": true,
      "showCapabilityStateNearStrictControls": true,
      "showCustodyAndRetentionNearHistoryControls": true
    },
    "controlKinds": [
      "boolean",
      "single-choice",
      "multi-choice",
      "number",
      "duration",
      "schedule",
      "place-list",
      "geofence-list",
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
      "permissionAtLeast",
      "proofAtLeast"
    ],
    "sections": [
      {
        "sectionId": "location-management",
        "title": "Location management",
        "purpose": "Top-level device location switch and default posture.",
        "fields": [
          {
            "fieldId": "location.enabled",
            "kind": "boolean",
            "question": "Enable device location features?",
            "writesTo": "/locationPolicy/enabled",
            "defaultValue": false,
            "uiPriority": 10,
            "whenFalse": {
              "policyMeaning": "Device location is not sampled, geofences are inactive, live map is unavailable, and only historical records already retained by policy remain visible.",
              "hiddenSections": [
                "location-mode",
                "permissions",
                "live-tracking",
                "last-known",
                "check-ins",
                "places-geofences",
                "alerts",
                "reports",
                "retention",
                "custody",
                "audit"
              ]
            }
          },
          {
            "fieldId": "location.defaultPosture",
            "kind": "single-choice",
            "question": "What location posture should this device use?",
            "writesTo": "/locationPolicy/defaultPosture",
            "defaultValue": "last-known-only",
            "visibleWhen": {
              "path": "/locationPolicy/enabled",
              "equals": true
            },
            "options": [
              {
                "value": "off",
                "label": "Off",
                "meaning": "Do not collect new location evidence."
              },
              {
                "value": "last-known-only",
                "label": "Last known",
                "meaning": "Show the newest available location evidence with freshness and accuracy labels."
              },
              {
                "value": "check-in-only",
                "label": "Check-in",
                "meaning": "Use child check-ins and optional one-time location samples rather than continuous tracking."
              },
              {
                "value": "geofence-alerts",
                "label": "Arrival alerts",
                "meaning": "Use places and schedules to record arrival, departure, dwell, missed arrival, and stale states."
              },
              {
                "value": "temporary-live",
                "label": "Temporary live",
                "meaning": "Allow parent-started time-limited live sessions when platform capability and disclosure requirements are met."
              },
              {
                "value": "missing-device",
                "label": "Missing device",
                "meaning": "Prioritize current or last-known location, contact state, and battery state when a device is marked missing."
              }
            ]
          },
          {
            "fieldId": "location.executionMode",
            "kind": "single-choice",
            "question": "Where should location behavior execute?",
            "writesTo": "/locationPolicy/executionMode",
            "defaultValue": "local-child-agent",
            "visibleWhen": {
              "path": "/locationPolicy/enabled",
              "equals": true
            },
            "options": ["local-child-agent", "lan-live", "authenticated-relay", "authoring-only", "unavailable"]
          }
        ]
      },
      {
        "sectionId": "permissions",
        "title": "Permission and disclosure",
        "purpose": "Represent OS permission requirements, precision, background state, and child-facing disclosure.",
        "visibleWhen": {
          "path": "/locationPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "permissions.minimumPermission",
            "kind": "single-choice",
            "question": "What location permission is required?",
            "writesTo": "/locationPolicy/permissions/minimumPermission",
            "defaultValue": "foreground-approximate",
            "options": [
              "none",
              "foreground-approximate",
              "foreground-precise",
              "background-approximate",
              "background-precise",
              "supervised-or-device-owner",
              "platform-managed-lost-mode"
            ]
          },
          {
            "fieldId": "permissions.whenPermissionMissing",
            "kind": "single-choice",
            "question": "What should happen if permission is missing?",
            "writesTo": "/locationPolicy/permissions/whenPermissionMissing",
            "defaultValue": "show-setup-required",
            "options": [
              "show-setup-required",
              "fallback-to-check-in",
              "fallback-to-last-known",
              "report-unavailable",
              "ask-parent",
              "disable-location-features"
            ]
          },
          {
            "fieldId": "permissions.allowApproximateFallback",
            "kind": "boolean",
            "question": "Allow approximate location when precise is not granted?",
            "writesTo": "/locationPolicy/permissions/allowApproximateFallback",
            "defaultValue": true
          },
          {
            "fieldId": "permissions.childDisclosure",
            "kind": "single-choice",
            "question": "What should the child device disclose?",
            "writesTo": "/locationPolicy/permissions/childDisclosure",
            "defaultValue": "show-mode-and-last-sample",
            "options": [
              "none",
              "show-enabled",
              "show-mode-and-last-sample",
              "show-live-session-active",
              "show-background-tracking-active"
            ]
          }
        ]
      },
      {
        "sectionId": "live-tracking",
        "title": "Live tracking",
        "purpose": "Configure temporary live map sessions and cadence limits.",
        "visibleWhen": {
          "path": "/locationPolicy/defaultPosture",
          "includes": ["temporary-live", "missing-device"]
        },
        "fields": [
          {
            "fieldId": "live.mode",
            "kind": "single-choice",
            "question": "When can live tracking run?",
            "writesTo": "/locationPolicy/live/mode",
            "defaultValue": "parent-started-temporary",
            "options": [
              "disabled",
              "parent-started-temporary",
              "during-active-trip",
              "during-missing-device",
              "during-alert-investigation"
            ]
          },
          {
            "fieldId": "live.maxSessionMinutes",
            "kind": "number",
            "question": "What is the maximum live session duration?",
            "writesTo": "/locationPolicy/live/maxSessionMinutes",
            "defaultValue": 30,
            "min": 1,
            "max": 240
          },
          {
            "fieldId": "live.updateCadence",
            "kind": "single-choice",
            "question": "How often should live tracking request updates?",
            "writesTo": "/locationPolicy/live/updateCadence",
            "defaultValue": "battery-balanced",
            "options": ["one-shot", "on-change", "battery-balanced", "high-accuracy-burst", "manual-refresh-only"]
          },
          {
            "fieldId": "live.whenBatteryLow",
            "kind": "single-choice",
            "question": "What should happen when battery is low?",
            "writesTo": "/locationPolicy/live/whenBatteryLow",
            "defaultValue": "reduce-cadence",
            "options": ["continue", "reduce-cadence", "last-known-only", "ask-parent", "stop-live-session"]
          }
        ]
      },
      {
        "sectionId": "last-known",
        "title": "Last known location",
        "purpose": "Control last-known reporting and stale evidence labels.",
        "visibleWhen": {
          "path": "/locationPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "lastKnown.showOnMap",
            "kind": "boolean",
            "question": "Show last known location on the map?",
            "writesTo": "/locationPolicy/lastKnown/showOnMap",
            "defaultValue": true
          },
          {
            "fieldId": "lastKnown.staleAfterMinutes",
            "kind": "number",
            "question": "When should a point become stale?",
            "writesTo": "/locationPolicy/lastKnown/staleAfterMinutes",
            "defaultValue": 15,
            "min": 1,
            "max": 1440
          },
          {
            "fieldId": "lastKnown.whenStale",
            "kind": "single-choice",
            "question": "What should the UI show when location is stale?",
            "writesTo": "/locationPolicy/lastKnown/whenStale",
            "defaultValue": "show-stale-with-contact-state",
            "options": ["hide-point", "show-stale", "show-stale-with-contact-state", "ask-check-in", "notify-parent"]
          }
        ]
      },
      {
        "sectionId": "check-ins",
        "title": "Check-ins",
        "purpose": "Configure child check-in prompts and response handling.",
        "visibleWhen": {
          "path": "/locationPolicy/defaultPosture",
          "includes": ["check-in-only", "geofence-alerts", "temporary-live", "missing-device"]
        },
        "fields": [
          {
            "fieldId": "checkIns.mode",
            "kind": "single-choice",
            "question": "How should check-ins work?",
            "writesTo": "/locationPolicy/checkIns/mode",
            "defaultValue": "parent-requested",
            "options": ["disabled", "parent-requested", "scheduled", "geofence-miss", "policy-triggered"]
          },
          {
            "fieldId": "checkIns.includeLocation",
            "kind": "single-choice",
            "question": "Should check-ins include location?",
            "writesTo": "/locationPolicy/checkIns/includeLocation",
            "defaultValue": "when-permitted",
            "options": ["never", "when-permitted", "require-current-location", "allow-child-choice"]
          },
          {
            "fieldId": "checkIns.unansweredAfterMinutes",
            "kind": "number",
            "question": "When is a check-in unanswered?",
            "writesTo": "/locationPolicy/checkIns/unansweredAfterMinutes",
            "defaultValue": 10,
            "min": 1,
            "max": 120
          },
          {
            "fieldId": "checkIns.allowedResponses",
            "kind": "multi-choice",
            "question": "Which child responses are allowed?",
            "writesTo": "/locationPolicy/checkIns/allowedResponses",
            "defaultValue": ["safe", "arriving", "leaving", "delayed", "need-help", "call-me"],
            "options": ["safe", "arriving", "leaving", "delayed", "need-help", "call-me", "custom-note"]
          }
        ]
      },
      {
        "sectionId": "places-geofences",
        "title": "Places and geofences",
        "purpose": "Configure parent-defined places, geofence transitions, schedules, and fallbacks.",
        "visibleWhen": {
          "path": "/locationPolicy/defaultPosture",
          "includes": ["geofence-alerts", "temporary-live", "missing-device"]
        },
        "fields": [
          {
            "fieldId": "places.enabled",
            "kind": "boolean",
            "question": "Enable parent-defined places?",
            "writesTo": "/locationPolicy/places/enabled",
            "defaultValue": true
          },
          {
            "fieldId": "places.minimumRadiusMeters",
            "kind": "number",
            "question": "What minimum radius should place geofences use?",
            "writesTo": "/locationPolicy/places/minimumRadiusMeters",
            "defaultValue": 150,
            "min": 100,
            "max": 5000
          },
          {
            "fieldId": "geofences.whenUnavailable",
            "kind": "single-choice",
            "question": "What if geofence monitoring is unavailable?",
            "writesTo": "/locationPolicy/geofences/whenUnavailable",
            "defaultValue": "fallback-to-check-in",
            "options": [
              "fallback-to-check-in",
              "fallback-to-sampled-location",
              "report-unavailable",
              "ask-parent",
              "disable-geofence-rules"
            ]
          },
          {
            "fieldId": "geofences.transitionTypes",
            "kind": "multi-choice",
            "question": "Which geofence transitions should be used?",
            "writesTo": "/locationPolicy/geofences/transitionTypes",
            "defaultValue": ["enter", "exit", "dwell"],
            "options": ["enter", "exit", "dwell", "missed-arrival", "stale-at-place"]
          }
        ]
      },
      {
        "sectionId": "alerts",
        "title": "Alerts",
        "purpose": "Choose which location states create parent notifications.",
        "visibleWhen": {
          "path": "/locationPolicy/enabled",
          "equals": true
        },
        "fields": [
          {
            "fieldId": "alerts.enabledReasons",
            "kind": "multi-choice",
            "question": "Which location events should notify a parent?",
            "writesTo": "/locationPolicy/alerts/enabledReasons",
            "defaultValue": ["missed-arrival", "unanswered-check-in", "device-offline-during-trip"],
            "options": [
              "arrival",
              "departure",
              "early-departure",
              "missed-arrival",
              "unanswered-check-in",
              "need-help-check-in",
              "device-offline-during-trip",
              "location-permission-lost",
              "live-session-started",
              "missing-device-found"
            ]
          },
          {
            "fieldId": "alerts.sensitiveDetailsInProviderBody",
            "kind": "single-choice",
            "question": "What location detail may appear in push/email/SMS bodies?",
            "writesTo": "/locationPolicy/alerts/sensitiveDetailsInProviderBody",
            "defaultValue": "minimal",
            "options": ["none", "minimal", "place-label-only", "approximate-area", "exact-coordinate"]
          }
        ]
      }
    ]
  },
  "policyValue": {
    "documentId": "location-policy-mia-phone",
    "policyKind": "device-location-tracking",
    "schemaVersion": 1,
    "revision": 4,
    "scope": {
      "familyId": "family-local-1",
      "childId": "child-mia",
      "deviceId": "device-android-phone",
      "platform": "android"
    },
    "locationPolicy": {
      "enabled": true,
      "defaultPosture": "geofence-alerts",
      "executionMode": "local-child-agent",
      "permissions": {
        "minimumPermission": "background-approximate",
        "whenPermissionMissing": "fallback-to-check-in",
        "allowApproximateFallback": true,
        "childDisclosure": "show-mode-and-last-sample"
      },
      "live": {
        "mode": "parent-started-temporary",
        "maxSessionMinutes": 30,
        "updateCadence": "battery-balanced",
        "whenBatteryLow": "reduce-cadence",
        "allowedReasons": ["during-active-trip", "during-alert-investigation", "during-missing-device"]
      },
      "lastKnown": {
        "showOnMap": true,
        "staleAfterMinutes": 15,
        "whenStale": "show-stale-with-contact-state"
      },
      "checkIns": {
        "mode": "geofence-miss",
        "includeLocation": "when-permitted",
        "unansweredAfterMinutes": 10,
        "allowedResponses": ["safe", "arriving", "leaving", "delayed", "need-help", "call-me"]
      },
      "places": {
        "enabled": true,
        "minimumRadiusMeters": 150,
        "items": [
          {
            "placeId": "place-home",
            "labelToken": "home",
            "center": {
              "latitude": 43.6532,
              "longitude": -79.3832
            },
            "radiusMeters": 200,
            "allowedPrecision": "approximate-or-better"
          },
          {
            "placeId": "place-school",
            "labelToken": "school",
            "center": {
              "latitude": 43.6677,
              "longitude": -79.3948
            },
            "radiusMeters": 250,
            "allowedPrecision": "approximate-or-better"
          }
        ]
      },
      "geofences": {
        "whenUnavailable": "fallback-to-check-in",
        "transitionTypes": ["enter", "exit", "dwell", "missed-arrival"],
        "rules": [
          {
            "ruleId": "arrive-school-weekday",
            "enabled": true,
            "priority": 100,
            "placeId": "place-school",
            "transition": "enter",
            "scheduleId": "school-arrival-window",
            "requiredProof": "platform-geofence-or-fresh-location",
            "action": "record-and-report"
          },
          {
            "ruleId": "left-school-early",
            "enabled": true,
            "priority": 200,
            "placeId": "place-school",
            "transition": "exit",
            "scheduleId": "school-hours",
            "requiredProof": "platform-geofence-or-fresh-location",
            "action": "notify-parent"
          },
          {
            "ruleId": "missed-practice-check-in",
            "enabled": true,
            "priority": 300,
            "placeId": "place-school",
            "transition": "missed-arrival",
            "scheduleId": "practice-arrival-window",
            "requiredProof": "schedule-and-last-contact",
            "action": "request-check-in"
          }
        ]
      },
      "alerts": {
        "enabledReasons": [
          "missed-arrival",
          "unanswered-check-in",
          "need-help-check-in",
          "device-offline-during-trip",
          "location-permission-lost"
        ],
        "sensitiveDetailsInProviderBody": "minimal",
        "quietHoursPolicyId": "family-default-quiet-hours",
        "dedupeWindowMinutes": 15
      },
      "reports": {
        "visibleFields": [
          "last-location",
          "freshness",
          "accuracy",
          "permission-state",
          "device-contact-state",
          "geofence-events",
          "check-ins",
          "live-session-audit",
          "custody-label"
        ],
        "showExactCoordinateRequiresReveal": true,
        "summaries": ["by-child", "by-device", "by-place", "by-day", "by-alert-reason"]
      },
      "retention": {
        "livePointStream": "24-hours",
        "locationPointHistory": "7-days",
        "placeTransitionAudit": "30-days",
        "checkInAudit": "90-days",
        "policyAudit": "180-days",
        "deleteExpired": true,
        "keepRedactedPlaceSummary": true
      },
      "custody": {
        "allowedUses": [
          "child-local",
          "lan-live",
          "parent-cache",
          "parent-owned-storage",
          "parent-export",
          "parent-report"
        ],
        "defaultStorage": "child-local",
        "hostedStorageDefault": false,
        "requireCustodyLabelForPortal": true,
        "requireCustodyLabelForAi": true,
        "requireCustodyLabelForExport": true,
        "allowOcentraHostedRawLocationHistory": false
      },
      "fallbacks": {
        "permissionMissing": "fallback-to-check-in",
        "precisePermissionMissing": "allow-approximate",
        "backgroundPermissionMissing": "foreground-or-check-in",
        "locationServiceDisabled": "show-setup-required",
        "geofenceUnavailable": "fallback-to-check-in",
        "deviceOffline": "last-known-report-only",
        "batteryLow": "reduce-cadence",
        "platformUnsupported": "show-unavailable",
        "adapterError": "rollback-and-audit",
        "staleEvidence": "show-stale-with-contact-state"
      },
      "audit": {
        "requiredFields": [
          "policy-decision",
          "evidence-ref",
          "location-source",
          "accuracy",
          "freshness",
          "adapter-result",
          "permission-state",
          "parent-reveal",
          "policy-version",
          "capability-state",
          "custody-label"
        ],
        "auditEveryLiveSession": true,
        "auditEveryNotificationIntent": true,
        "auditEveryExactCoordinateReveal": true,
        "auditFailedAdapterActions": true,
        "auditPolicyPreview": true
      }
    },
    "schedules": [
      {
        "scheduleId": "always",
        "kind": "always"
      },
      {
        "scheduleId": "school-arrival-window",
        "kind": "weekly-window",
        "timezone": "America/Toronto",
        "windows": [
          {
            "days": ["monday", "tuesday", "wednesday", "thursday", "friday"],
            "start": "07:30",
            "end": "09:00"
          }
        ]
      },
      {
        "scheduleId": "school-hours",
        "kind": "weekly-window",
        "timezone": "America/Toronto",
        "windows": [
          {
            "days": ["monday", "tuesday", "wednesday", "thursday", "friday"],
            "start": "09:00",
            "end": "15:30"
          }
        ]
      },
      {
        "scheduleId": "practice-arrival-window",
        "kind": "weekly-window",
        "timezone": "America/Toronto",
        "windows": [
          {
            "days": ["tuesday", "thursday"],
            "start": "17:00",
            "end": "18:15"
          }
        ]
      }
    ]
  },
  "effectivePolicy": {
    "documentId": "location-effective-mia-phone",
    "compiledFromPolicyId": "location-policy-mia-phone",
    "compiledFromRevision": 4,
    "schemaVersion": 1,
    "effectivePolicyHash": "sha256:worker-must-compute-sample",
    "compiledAt": "2026-05-28T00:00:00.000Z",
    "scope": {
      "familyId": "family-local-1",
      "childId": "child-mia",
      "deviceId": "device-android-phone",
      "platform": "android"
    },
    "locationEnabled": true,
    "defaultDecision": "geofence-alerts",
    "executionMode": "local-child-agent",
    "requiredPermission": "background-approximate",
    "precisionDecision": {
      "allowApproximate": true,
      "requirePreciseForExactCoordinateReveal": true,
      "minimumAccuracyMetersForGeofence": 500,
      "minimumAccuracyMetersForLiveMap": 1000,
      "markStaleAfterSeconds": 900
    },
    "samplingPlan": {
      "lastKnownAllowed": true,
      "foregroundRefreshAllowed": true,
      "backgroundRefreshAllowed": true,
      "defaultCadence": "battery-balanced",
      "lowBatteryCadence": "reduced",
      "stopWhenPermissionRevoked": true,
      "stopWhenDeviceOffline": true
    },
    "liveSessionPlan": {
      "enabled": true,
      "maxSessionSeconds": 1800,
      "allowedReasons": ["during-active-trip", "during-alert-investigation", "during-missing-device"],
      "requiresAudit": true,
      "requiresChildDisclosure": true,
      "deliveryPathsInPriorityOrder": ["lan-live", "authenticated-relay", "parent-cache"]
    },
    "geofencePlan": {
      "enabled": true,
      "minimumRadiusMeters": 150,
      "maxActiveGeofences": 100,
      "whenUnavailable": "fallback-to-check-in",
      "transitions": ["enter", "exit", "dwell", "missed-arrival"],
      "rulesInPriorityOrder": [
        {
          "ruleId": "arrive-school-weekday",
          "priority": 100,
          "placeId": "place-school",
          "transition": "enter",
          "scheduleId": "school-arrival-window",
          "decision": "record-and-report",
          "proofRequirement": "platform-geofence-or-fresh-location"
        },
        {
          "ruleId": "left-school-early",
          "priority": 200,
          "placeId": "place-school",
          "transition": "exit",
          "scheduleId": "school-hours",
          "decision": "notify-parent",
          "proofRequirement": "platform-geofence-or-fresh-location"
        },
        {
          "ruleId": "missed-practice-check-in",
          "priority": 300,
          "placeId": "place-school",
          "transition": "missed-arrival",
          "scheduleId": "practice-arrival-window",
          "decision": "request-check-in",
          "proofRequirement": "schedule-and-last-contact"
        }
      ]
    },
    "checkInPlan": {
      "enabled": true,
      "mode": "geofence-miss",
      "includeLocation": "when-permitted",
      "unansweredAfterSeconds": 600,
      "allowedResponses": ["safe", "arriving", "leaving", "delayed", "need-help", "call-me"]
    },
    "notificationPlan": {
      "enabledReasons": [
        "missed-arrival",
        "unanswered-check-in",
        "need-help-check-in",
        "device-offline-during-trip",
        "location-permission-lost"
      ],
      "providerBodyDetail": "minimal",
      "dedupeWindowSeconds": 900,
      "requiresEvidenceReference": true
    },
    "runtimeTables": {
      "placesById": {
        "place-home": {
          "labelToken": "home",
          "radiusMeters": 200,
          "allowedPrecision": "approximate-or-better"
        },
        "place-school": {
          "labelToken": "school",
          "radiusMeters": 250,
          "allowedPrecision": "approximate-or-better"
        }
      },
      "schedulesById": {
        "always": {
          "kind": "always"
        },
        "school-arrival-window": {
          "kind": "weekly-window",
          "timezone": "America/Toronto"
        },
        "school-hours": {
          "kind": "weekly-window",
          "timezone": "America/Toronto"
        },
        "practice-arrival-window": {
          "kind": "weekly-window",
          "timezone": "America/Toronto"
        }
      }
    },
    "fallbackDecisions": {
      "permissionMissing": "fallback-to-check-in",
      "precisePermissionMissing": "allow-approximate",
      "backgroundPermissionMissing": "foreground-or-check-in",
      "locationServiceDisabled": "show-setup-required",
      "geofenceUnavailable": "fallback-to-check-in",
      "deviceOffline": "last-known-report-only",
      "batteryLow": "reduce-cadence",
      "platformUnsupported": "show-unavailable",
      "adapterError": "rollback-and-audit",
      "staleEvidence": "show-stale-with-contact-state"
    },
    "custodyPlan": {
      "rawEvidenceDefaultStorage": "child-local",
      "hostedStorageDefault": false,
      "allowedUses": [
        "child-local",
        "lan-live",
        "parent-cache",
        "parent-owned-storage",
        "parent-export",
        "parent-report"
      ],
      "requireCustodyLabelForPortal": true,
      "requireCustodyLabelForAi": true,
      "requireCustodyLabelForExport": true
    },
    "retentionPlan": {
      "livePointStreamSeconds": 86400,
      "locationPointHistorySeconds": 604800,
      "placeTransitionAuditSeconds": 2592000,
      "checkInAuditSeconds": 7776000,
      "policyAuditSeconds": 15552000,
      "deleteExpired": true,
      "keepRedactedPlaceSummary": true
    },
    "auditPlan": {
      "auditEveryLiveSession": true,
      "auditEveryNotificationIntent": true,
      "auditEveryExactCoordinateReveal": true,
      "requiredFields": [
        "policy-decision",
        "evidence-ref",
        "location-source",
        "accuracy",
        "freshness",
        "adapter-result",
        "permission-state",
        "parent-reveal",
        "policy-version",
        "capability-state",
        "custody-label"
      ]
    }
  },
  "updateProtocol": {
    "commands": [
      {
        "commandType": "device-location-policy.get.requested",
        "purpose": "Portal asks the child agent for current policy value, revision, effective policy, and capability state.",
        "requestShape": {
          "targetDeviceId": "device-android-phone",
          "includeAuthoringManifest": true,
          "includeEffectivePolicy": true,
          "includeCapabilityRegistry": true
        }
      },
      {
        "commandType": "device-location-policy.preview.requested",
        "purpose": "Portal asks whether proposed changes validate and what effective policy would result.",
        "requestShape": {
          "targetDeviceId": "device-android-phone",
          "baseRevision": 4,
          "patch": [
            {
              "op": "replace",
              "path": "/locationPolicy/defaultPosture",
              "value": "temporary-live"
            }
          ]
        },
        "responseShape": {
          "accepted": true,
          "wouldCreateRevision": 5,
          "effectivePolicyPreviewHash": "sha256:static-sample-token",
          "warnings": ["background-permission-required-for-live"],
          "unsupportedSettings": []
        }
      },
      {
        "commandType": "device-location-policy.patch.requested",
        "purpose": "Portal sends a small settings change with an expected revision.",
        "requestShape": {
          "targetDeviceId": "device-android-phone",
          "expectedRevision": 4,
          "patch": [
            {
              "op": "replace",
              "path": "/locationPolicy/live/maxSessionMinutes",
              "value": 45
            }
          ],
          "reason": "parent-ui-change"
        },
        "acceptedResponseShape": {
          "eventType": "device-location-policy.patch.accepted",
          "newRevision": 5,
          "policyHash": "sha256:static-sample-token",
          "effectivePolicyHash": "sha256:static-sample-token",
          "requiresPermissionSetup": false,
          "unsupportedSettings": []
        },
        "rejectedResponseShape": {
          "eventType": "device-location-policy.patch.rejected",
          "currentRevision": 5,
          "reason": "revision-conflict",
          "validationErrors": []
        }
      },
      {
        "commandType": "device-location-policy.replace.requested",
        "purpose": "Portal sends a full policy replacement for setup, import, reset, or wizard save.",
        "requestShape": {
          "targetDeviceId": "device-android-phone",
          "expectedRevision": 4,
          "replacementPolicy": {
            "documentId": "location-policy-mia-phone",
            "policyKind": "device-location-tracking",
            "schemaVersion": 1,
            "revision": 5
          },
          "reason": "parent-wizard-save"
        }
      },
      {
        "commandType": "device-location.live-session.start.requested",
        "purpose": "Parent asks the child agent to start a temporary live map session.",
        "requestShape": {
          "targetDeviceId": "device-android-phone",
          "policyRevision": 4,
          "reason": "during-alert-investigation",
          "requestedDurationSeconds": 1200,
          "requestedCadence": "battery-balanced",
          "requestedPrecision": "approximate-or-better"
        },
        "acceptedResponseShape": {
          "eventType": "device-location.live-session.start.accepted",
          "sessionId": "location-live-session-1",
          "expiresAt": "2026-05-28T00:20:00.000Z",
          "deliveryPath": "lan-live",
          "capabilityState": "ready-background-approximate"
        },
        "rejectedResponseShape": {
          "eventType": "device-location.live-session.start.rejected",
          "reason": "permission-required",
          "fallback": "request-check-in"
        }
      },
      {
        "commandType": "device-location.check-in.requested",
        "purpose": "Parent asks the child device for a check-in and optional current location.",
        "requestShape": {
          "targetDeviceId": "device-android-phone",
          "reason": "missed-arrival",
          "includeLocation": "when-permitted",
          "expiresAfterSeconds": 600,
          "allowedResponses": ["safe", "arriving", "leaving", "delayed", "need-help", "call-me"]
        }
      },
      {
        "commandType": "device-location.geofence.sync.requested",
        "purpose": "Portal or parent-controller asks the child agent to sync compiled geofence definitions after policy acceptance.",
        "requestShape": {
          "targetDeviceId": "device-android-phone",
          "effectivePolicyHash": "sha256:static-sample-token",
          "placeIds": ["place-home", "place-school"]
        }
      },
      {
        "commandType": "device-location-policy.rollback.requested",
        "purpose": "Parent asks child agent to roll back to previous valid revision.",
        "requestShape": {
          "targetDeviceId": "device-android-phone",
          "targetRevision": 4,
          "reason": "parent-rollback"
        }
      }
    ],
    "events": [
      {
        "eventType": "device-location.sample.recorded",
        "purpose": "Child agent recorded a location point.",
        "eventShape": {
          "evidenceId": "location-evidence-1",
          "deviceId": "device-android-phone",
          "recordedAt": "2026-05-28T00:00:10.000Z",
          "providerTimestamp": "2026-05-28T00:00:08.000Z",
          "position": {
            "latitude": 43.6532,
            "longitude": -79.3832,
            "accuracyMeters": 120
          },
          "sourceKinds": ["fused", "wifi", "gps"],
          "permissionState": "background-approximate",
          "freshnessSeconds": 2,
          "custodyLabel": "child-local"
        }
      },
      {
        "eventType": "device-location.geofence.transition.recorded",
        "purpose": "Child agent recorded an arrival, departure, dwell, missed-arrival, or stale geofence state.",
        "eventShape": {
          "evidenceId": "geofence-evidence-1",
          "deviceId": "device-android-phone",
          "placeId": "place-school",
          "transition": "enter",
          "recordedAt": "2026-05-28T12:50:00.000Z",
          "proofRequirement": "platform-geofence-or-fresh-location",
          "proofState": "satisfied",
          "custodyLabel": "child-local"
        }
      },
      {
        "eventType": "device-location.check-in.recorded",
        "purpose": "Child device recorded a check-in response.",
        "eventShape": {
          "checkInId": "check-in-1",
          "deviceId": "device-android-phone",
          "response": "safe",
          "locationEvidenceId": "location-evidence-1",
          "recordedAt": "2026-05-28T22:00:00.000Z",
          "custodyLabel": "child-local"
        }
      }
    ],
    "agentRules": {
      "validateFullPolicyAfterPatch": true,
      "compileFullEffectivePolicyAfterEveryAcceptedChange": true,
      "persistPolicyBeforeRuntimeSwitch": true,
      "keepPreviousValidRevision": true,
      "rollbackOnCompileFailure": true,
      "executeLocallyWhenPortalOffline": true,
      "rejectUnknownPaths": true,
      "rejectInvalidEnumValues": true,
      "rejectGeofenceWithoutPlace": true,
      "rejectLiveModeWithoutDuration": true,
      "rejectExactCoordinateRevealWithoutAudit": true,
      "rejectHostedRawLocationHistoryByDefault": true,
      "rejectGeofenceCountAbovePlatformLimit": true,
      "rejectStrictLocationRuleWithoutFallback": true
    }
  },
  "capabilityRegistry": {
    "deviceId": "device-android-phone",
    "generatedAt": "2026-05-28T00:00:00.000Z",
    "platform": "android",
    "capabilities": [
      {
        "capabilityId": "android-foreground-location",
        "state": "ready",
        "permissionState": "foreground-approximate",
        "proof": "runtime-adapter-proof-required",
        "affectsFields": ["permissions.minimumPermission", "lastKnown.showOnMap", "checkIns.includeLocation"]
      },
      {
        "capabilityId": "android-background-location",
        "state": "permission-required",
        "permissionState": "not-granted",
        "proof": "manual-required-or-runtime-read-model",
        "affectsFields": ["live.mode", "geofences.transitionTypes", "permissions.minimumPermission"]
      },
      {
        "capabilityId": "android-geofencing",
        "state": "manual-required",
        "proof": "not-yet-proven",
        "limits": {
          "maxActiveGeofences": 100,
          "recommendedMinimumRadiusMeters": 100,
          "backgroundMayBeDelayed": true
        },
        "affectsFields": ["places.minimumRadiusMeters", "geofences.transitionTypes", "geofences.whenUnavailable"]
      },
      {
        "capabilityId": "android-precise-location",
        "state": "permission-required",
        "permissionState": "approximate-only",
        "proof": "runtime-adapter-proof-required",
        "affectsFields": ["permissions.allowApproximateFallback", "reports.showExactCoordinateRequiresReveal"]
      },
      {
        "capabilityId": "child-device-notifications",
        "state": "manual-required",
        "proof": "notification-domain-contract-required",
        "affectsFields": ["checkIns.mode", "alerts.enabledReasons"]
      },
      {
        "capabilityId": "lan-live-delivery",
        "state": "manual-required",
        "proof": "v0-9-route-and-device-proof-required",
        "affectsFields": ["location.executionMode", "live.mode"]
      },
      {
        "capabilityId": "parent-owned-storage-sync",
        "state": "not-implemented",
        "proof": "sync-export-contract-required",
        "affectsFields": ["custody.allowedUses", "reports.summaries", "retention.locationPointHistory"]
      },
      {
        "capabilityId": "ocentra-hosted-raw-location-history",
        "state": "blocked-by-default",
        "proof": "product-custody-rule",
        "affectsFields": ["custody.allowOcentraHostedRawLocationHistory"]
      }
    ],
    "platformDefaults": {
      "windows": {
        "locationApi": "windows-geolocator",
        "defaultState": "manual-required",
        "likelyLimits": ["desktop-accuracy-varies", "permission-required", "continuous-tracking-battery-cost"]
      },
      "macos": {
        "locationApi": "core-location",
        "defaultState": "manual-required",
        "likelyLimits": ["tcc-required", "region-monitoring-app-running-system-awake", "find-my-not-generic-api"]
      },
      "linux": {
        "locationApi": "geoclue-if-available",
        "defaultState": "manual-required",
        "likelyLimits": ["distro-dependent", "desktop-agent-dependent", "headless-unavailable"]
      },
      "android": {
        "locationApi": "fused-location-provider",
        "defaultState": "manual-required",
        "likelyLimits": [
          "foreground-background-permission-split",
          "approximate-precision",
          "background-throttling",
          "play-policy"
        ]
      },
      "ios": {
        "locationApi": "core-location",
        "defaultState": "manual-required",
        "likelyLimits": [
          "always-authorization-required",
          "background-mode-required",
          "reduced-accuracy",
          "mdm-lost-mode-supervision-only"
        ]
      },
      "webPortal": {
        "authoringOnly": true,
        "mayRunChildLocationCapture": false,
        "mayEvaluateLocationPolicy": false
      }
    }
  }
}
```

## Implementation Notes For Worker

- Start with domain contracts before Portal UI.
- Keep authoring manifest ids, field ids, section ids, option ids, policy ids,
  place ids, geofence ids, schedule ids, check-in ids, live-session ids, and
  capability ids branded.
- Do not let Portal define arbitrary JSON paths. `writesTo` paths should be
  schema-known authoring paths.
- Use Effect Schema to validate the full policy after every patch.
- Compile the effective policy in the child-agent/service boundary, not in
  Portal.
- Persist both policy revision and compiled effective policy hash.
- Reject partial states. For example, `defaultPosture: "temporary-live"` needs a
  max session duration, permission fallback, custody plan, and audit plan.
- Treat the authoring manifest as UI guidance only. Runtime behavior must rely
  on validated policy and compiled effective policy.
- Add explicit tests for hidden/visible branch behavior so UI cannot show live
  controls when location is disabled or geofence controls when the platform is
  unavailable.
- Add explicit tests for offline behavior: child agent continues using the last
  valid compiled policy and reports last-known/contact state when Portal is
  disconnected.
- Add explicit tests for custody and retention so raw location history cannot
  become Ocentra-hosted default storage.
- Add Rust protocol parity only for command/event shapes that cross into the
  Rust service after TypeScript contracts are stable.
