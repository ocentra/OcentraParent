# Screen AI Adapter Dependency Handoff

Generated: 2026-06-08T04:01:08.975Z

## screen-ai-broad-installed-app-manual-required

- adapter class: broad-installed-app
- owner: codex-c (app-game/enforcement adapter layer)
- expected proof: `output/app-game-plan-proof/screen-derived-broad-installed-app-apply-rollback-audit/proof-summary.json`
- missing now: broad installed-app apply, rollback, and audit custody proof from a screen-derived block decision
- unblocks: screen-ai-pipeline-plan: Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.; product-capability-checklist: Local screen evidence summaries; product-capability-checklist: Child-safety AI decision

```json
{
  "sourcePolicyDecisionRef": "screen-derived block/time-limit policy decision id",
  "sourceActivityEvidenceRef": "screen analysis/activity evidence ref",
  "applyResultRef": "real broad installed-app adapter apply result",
  "rollbackOrExpiryRef": "rollback or expiry result for the same target",
  "auditRef": "durable adapter audit/custody ref",
  "rawImageRetained": false,
  "rawImageDeletedBeforeAdapter": true,
  "finalAdapterCompletionClaimed": true
}
```

## screen-ai-host-network-domain-manual-required

- adapter class: host-network-domain
- owner: E-D (network/domain enforcement adapter layer)
- expected proof: `output/network-plan-proof/screen-derived-host-network-domain-apply-rollback-audit/proof-summary.json`
- missing now: host DNS/filter apply, rollback, and audit custody proof from a screen-derived network/domain decision
- unblocks: screen-ai-pipeline-plan: Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.; product-capability-checklist: Local screen evidence summaries; product-capability-checklist: Child-safety AI decision

```json
{
  "sourcePolicyDecisionRef": "screen-derived network/domain policy decision id",
  "sourceNetworkEvidenceRef": "host/domain/IP evidence ref",
  "applyResultRef": "real DNS/filter/firewall apply result",
  "rollbackOrExpiryRef": "rollback or expiry result for the same rule",
  "auditRef": "durable network adapter audit/custody ref",
  "rawImageRetained": false,
  "rawImageDeletedBeforeAdapter": true,
  "finalAdapterCompletionClaimed": true
}
```

## screen-ai-managed-active-tab-not-claimed

- adapter class: managed-active-tab-exact-url
- owner: codex-d (browser managed-control adapter layer)
- expected proof: `output/browser-plan-proof/screen-derived-managed-active-tab-apply-rollback-audit/proof-summary.json`
- missing now: managed active-tab exact URL apply, rollback, and audit custody proof from a screen-derived browser decision
- unblocks: screen-ai-pipeline-plan: Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.; product-capability-checklist: Local screen evidence summaries; product-capability-checklist: Child-safety AI decision

```json
{
  "sourcePolicyDecisionRef": "screen-derived browser policy decision id",
  "sourceBrowserEvidenceRef": "managed active-tab URL/evidence ref",
  "applyResultRef": "real exact active-tab adapter apply result",
  "rollbackOrExpiryRef": "tab/action rollback or expiry result",
  "auditRef": "durable browser adapter audit/custody ref",
  "rawImageRetained": false,
  "rawImageDeletedBeforeAdapter": true,
  "finalAdapterCompletionClaimed": true
}
```

## screen-ai-android-mobile-control-manual-required

- adapter class: android-device-owner-or-managed-profile
- owner: primary/mobile-child-agent-sequencing (Android child-agent Device Owner or managed-profile adapter layer)
- expected proof: `output/mobile-plan-proof/screen-derived-android-mobile-control-apply-rollback-audit/proof-summary.json`
- missing now: Android device-owner/managed-profile control proof from a screen-derived mobile decision
- unblocks: screen-ai-pipeline-plan: Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.; product-capability-checklist: Local screen evidence summaries; product-capability-checklist: Child-safety AI decision

```json
{
  "sourcePolicyDecisionRef": "screen-derived mobile policy decision id",
  "sourceMobileEvidenceRef": "Android child-agent/device evidence ref",
  "applyResultRef": "real Device Owner, managed-profile, UsageStats, Accessibility, or VPN/DNS apply result",
  "rollbackOrExpiryRef": "rollback or expiry result for the same mobile control",
  "auditRef": "durable Android adapter audit/custody ref",
  "rawImageRetained": false,
  "rawImageDeletedBeforeAdapter": true,
  "finalAdapterCompletionClaimed": true
}
```

## screen-ai-ios-mobile-control-manual-required

- adapter class: ios-family-controls-device-activity
- owner: primary/mobile-child-agent-sequencing (iOS Family Controls and DeviceActivity adapter layer)
- expected proof: `output/mobile-plan-proof/screen-derived-ios-mobile-control-apply-rollback-audit/proof-summary.json`
- missing now: iOS Family Controls/DeviceActivity control proof from a screen-derived mobile decision
- unblocks: screen-ai-pipeline-plan: Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.; product-capability-checklist: Local screen evidence summaries; product-capability-checklist: Child-safety AI decision

```json
{
  "sourcePolicyDecisionRef": "screen-derived mobile policy decision id",
  "sourceMobileEvidenceRef": "iOS child-agent/device evidence ref",
  "applyResultRef": "real Family Controls, DeviceActivity, or Network Extension apply result",
  "rollbackOrExpiryRef": "rollback or expiry result for the same mobile control",
  "auditRef": "durable iOS adapter audit/custody ref",
  "rawImageRetained": false,
  "rawImageDeletedBeforeAdapter": true,
  "finalAdapterCompletionClaimed": true
}
```
