# Screen AI Final Adapter Dependency Audit

Generated: 2026-06-08T02:30:18.687Z

## Source Artifacts

- adapterReadiness: `output/screen-ai-pipeline-proof/adapter-readiness/proof-summary.json`
- adapterReadinessReadModel: `output/screen-ai-pipeline-proof/adapter-readiness/read-model.json`
- finalProductPath: `output/screen-ai-pipeline-proof/final-product-path/proof-summary.json`
- linuxHostCustody: `output/screen-ai-pipeline-proof/linux-host-adapter-custody/proof-summary.json`
- androidMobileCustody: `output/screen-ai-pipeline-proof/android-mobile-control-custody/proof-summary.json`
- iosMobileCustody: `output/screen-ai-pipeline-proof/ios-mobile-control-custody/proof-summary.json`
- adapterDependencyHandoff: `output/screen-ai-pipeline-proof/adapter-dependency-handoff/proof-summary.json`
- adapterDependencyHandoffRows: `output/screen-ai-pipeline-proof/adapter-dependency-handoff/adapter-dependency-handoff.json`
- checklist: `docs/plans/screen-ai-pipeline-plan/implementation-checklist.md`

## Blocked Adapter Rows

- screen-ai-broad-installed-app-manual-required: manual-required, broad installed-app apply, rollback, and audit custody proof from a screen-derived block decision
- screen-ai-host-network-domain-manual-required: manual-required, host DNS/filter apply, rollback, and audit custody proof from a screen-derived network/domain decision
- screen-ai-managed-active-tab-not-claimed: not-claimed, managed active-tab exact URL apply, rollback, and audit custody proof from a screen-derived browser decision
- screen-ai-android-mobile-control-manual-required: manual-required, Android device-owner/managed-profile control proof from a screen-derived mobile decision
- screen-ai-ios-mobile-control-manual-required: manual-required, iOS Family Controls/DeviceActivity control proof from a screen-derived mobile decision
- screen-ai-linux-host-adapter-unavailable: unavailable, Linux host adapter apply, rollback, and audit custody proof from a screen-derived decision

## Custody Artifacts

- screen-ai-linux-host-adapter-unavailable: linux-host-custody-artifact-written-final-execution-blocked, executionClaimed=false
- screen-ai-android-mobile-control-manual-required: android-mobile-control-custody-artifact-written-final-execution-blocked, executionClaimed=false
- screen-ai-ios-mobile-control-manual-required: ios-mobile-control-custody-artifact-written-final-execution-blocked, executionClaimed=false

## Dependency Handoff Rows

- screen-ai-broad-installed-app-manual-required: codex-c, output/app-game-plan-proof/screen-derived-broad-installed-app-apply-rollback-audit/proof-summary.json
- screen-ai-host-network-domain-manual-required: E-D, output/network-plan-proof/screen-derived-host-network-domain-apply-rollback-audit/proof-summary.json
- screen-ai-managed-active-tab-not-claimed: codex-d, output/browser-plan-proof/screen-derived-managed-active-tab-apply-rollback-audit/proof-summary.json
- screen-ai-android-mobile-control-manual-required: primary/mobile-child-agent-sequencing, output/mobile-plan-proof/screen-derived-android-mobile-control-apply-rollback-audit/proof-summary.json
- screen-ai-ios-mobile-control-manual-required: primary/mobile-child-agent-sequencing, output/mobile-plan-proof/screen-derived-ios-mobile-control-apply-rollback-audit/proof-summary.json
- screen-ai-linux-host-adapter-unavailable: codex-b-after-linux-host-target, output/screen-ai-pipeline-proof/linux-host-adapter-execution/proof-summary.json

## Closure

```json
{
  "windowsOwnedProcessAdaptersProved": true,
  "finalPathArtifactGateStillValid": true,
  "portalReadModelAndDeletionStillProved": true,
  "broadBrowserNetworkMobileProductComplete": false,
  "openChecklistRowRetained": true,
  "executedAdapterRows": 2,
  "blockedAdapterRows": 6,
  "custodyArtifactRows": 3,
  "dependencyHandoffRows": 6,
  "claimUpgradeRows": 0
}
```
