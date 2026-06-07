# Screen AI Final Adapter Dependency Audit

Generated: 2026-06-07T22:23:47.749Z

## Source Artifacts

- adapterReadiness: `output/screen-ai-pipeline-proof/adapter-readiness/proof-summary.json`
- adapterReadinessReadModel: `output/screen-ai-pipeline-proof/adapter-readiness/read-model.json`
- finalProductPath: `output/screen-ai-pipeline-proof/final-product-path/proof-summary.json`
- checklist: `docs/plans/screen-ai-pipeline-plan/implementation-checklist.md`

## Blocked Adapter Rows

- screen-ai-broad-installed-app-manual-required: manual-required, broad installed-app apply, rollback, and audit custody proof from a screen-derived block decision
- screen-ai-host-network-domain-manual-required: manual-required, host DNS/filter apply, rollback, and audit custody proof from a screen-derived network/domain decision
- screen-ai-managed-active-tab-not-claimed: not-claimed, managed active-tab exact URL apply, rollback, and audit custody proof from a screen-derived browser decision
- screen-ai-android-mobile-control-manual-required: manual-required, Android device-owner/managed-profile control proof from a screen-derived mobile decision
- screen-ai-ios-mobile-control-manual-required: manual-required, iOS Family Controls/DeviceActivity control proof from a screen-derived mobile decision
- screen-ai-linux-host-adapter-unavailable: unavailable, Linux host adapter apply, rollback, and audit custody proof from a screen-derived decision

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
  "claimUpgradeRows": 0
}
```
