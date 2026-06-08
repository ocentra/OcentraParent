# Tracking Real Runtime Handoff Manual Validation Runbook

- generatedAt: 2026-06-08T01:05:00.000Z
- commit: 901be3cc799a791a3661cad0c9d62d366aa78a67
- currentProofTier: P3_LOCAL_DEV_MACHINE
- requiredProofTier: P4_REAL_RUNTIME_HANDOFF
- productReadyClaimed: false
- ciRunnableRowCount: 0
- fullProductUiLocalArtifactCount: 8
- androidEmulatorRequiredArtifactCount: 12
- androidEmulatorPresentArtifactCount: 12
- androidEmulatorMissingArtifactCount: 0
- androidEmulatorPermissionUiArtifactCount: 3
- androidEmulatorRuntimeArtifactCount: 8
- androidEmulatorLocalGeofenceTransitionCount: 3
- iosSimulatorRequiredArtifactCount: 13
- iosSimulatorPresentArtifactCount: 13
- iosSimulatorMissingArtifactCount: 0
- iosSimulatorPackageArtifactCount: 4
- iosSimulatorLocationManualRequiredArtifactCount: 3
- iosSimulatorPrivacyDisclosureArtifactCount: 2
- iosSimulatorManualRequiredRowCount: 7
- iosSimulatorMissingRuntimeArtifactCount: 9
- childRuntimeRequiredArtifactCount: 10
- childRuntimePresentArtifactCount: 0
- childRuntimeMissingArtifactCount: 10
- retentionRuntimeRequiredArtifactCount: 2
- retentionRuntimePresentArtifactCount: 1
- retentionRuntimeMissingArtifactCount: 1
- retentionPlatformPreflightRowCount: 3
- retentionPlatformPreflightManualRequiredRowCount: 3
- retentionPlatformPreflightRequiredArtifactCount: 6
- retentionPlatformPreflightPresentArtifactCount: 0
- retentionPlatformPreflightMissingArtifactCount: 6
- productionWorkerRequiredArtifactCount: 8
- productionWorkerPresentArtifactCount: 0
- productionWorkerMissingArtifactCount: 8
- claimAuditMissingArtifactCount: 61
- claimAuditPhysicalDeviceRequiredRowCount: 6
- claimAuditApprovedManualRequiredRowCount: 1
- claimAuditManualProviderRuntimeRequiredRowCount: 1
- claimAuditProductionRuntimeRequiredRowCount: 2
- claimAuditAcceptanceCriteriaCount: 36
- claimAuditManualValidationCommandCount: 27
- claimAuditArtifactAcceptanceNoteCount: 36

## android-physical-background-and-geofence

- blockerId: android-physical-background-proof-required
- sourceProofRef: test-results/tracking-physical-device-artifact-gate-proof/proof.json
- proofRoot: output/tracking-plan-proof/android-background-geofence
- status: manual-required
- readinessCategory: physical-device-required
- ciRunnable: false
- missingArtifacts: 10/10

### Required Validation Commands

- Run Android physical-device background location and geofence transition proof on enrolled child hardware
- Record device metadata, runtime permission state, location events, geofence transitions, logcat, parent UI receipt, and summary under output/tracking-plan-proof/android-background-geofence/

### Artifact Acceptance Notes

- Android artifacts must come from a physical child device or explicitly approved equivalent hardware run
- Foreground-only emulator samples do not satisfy this handoff row

### Claim Audit Acceptance Criteria

- Collect every required artifact under output/tracking-plan-proof/android-background-geofence before review.
- Keep required proof tier P4_PHYSICAL_DEVICE; local P3 artifacts cannot approve the claim.
- Cite source proof test-results/tracking-physical-device-artifact-gate-proof/proof.json and all supporting proof refs in the final handoff.
- Use real device or enrolled child runtime evidence with metadata, logs, screenshots, and transition or execution rows.

### Claim Audit Validation Commands

- node scripts/test/tracking-claim-audit-proof.mjs
- node scripts/test/tracking-product-readiness-closure-proof.mjs
- node scripts/test/tracking-real-runtime-handoff-proof.mjs

### Claim Audit Artifact Notes

- Required artifacts: 10.
- Proof root: output/tracking-plan-proof/android-background-geofence.
- Status can move only to review-required when all required artifacts are present; claimApproved remains false here.
- Missing artifacts stay blocking until the P4_PHYSICAL_DEVICE evidence is produced outside local CI.

### Missing Artifacts

- 00-run-metadata.json
- 01-device-metadata.json
- 02-permission-state.json
- 03-geofence-definition.json
- 04-location-events.ndjson
- 05-geofence-transitions.ndjson
- 06-alert-decision.json
- 07-parent-ui-screenshot.png
- 08-logcat.txt
- 09-result-summary.md

## ios-physical-background-and-region

- blockerId: ios-physical-region-proof-required
- sourceProofRef: test-results/tracking-physical-device-artifact-gate-proof/proof.json
- proofRoot: output/tracking-plan-proof/ios-region-monitoring
- status: manual-required
- readinessCategory: physical-device-required
- ciRunnable: false
- missingArtifacts: 10/10

### Required Validation Commands

- Run iOS physical-device Always authorization, region monitoring, background delivery, and relaunch proof
- Record device metadata, authorization state, region transitions, Xcode/device logs, screenshots, and summary under output/tracking-plan-proof/ios-region-monitoring/

### Artifact Acceptance Notes

- iOS artifacts must come from an entitled physical device run
- Simulator privacy disclosure or package-preview proof does not satisfy this handoff row

### Claim Audit Acceptance Criteria

- Collect every required artifact under output/tracking-plan-proof/ios-region-monitoring before review.
- Keep required proof tier P4_PHYSICAL_DEVICE; local P3 artifacts cannot approve the claim.
- Cite source proof test-results/tracking-physical-device-artifact-gate-proof/proof.json and all supporting proof refs in the final handoff.
- Use real device or enrolled child runtime evidence with metadata, logs, screenshots, and transition or execution rows.

### Claim Audit Validation Commands

- node scripts/test/tracking-claim-audit-proof.mjs
- node scripts/test/tracking-product-readiness-closure-proof.mjs
- node scripts/test/tracking-real-runtime-handoff-proof.mjs

### Claim Audit Artifact Notes

- Required artifacts: 10.
- Proof root: output/tracking-plan-proof/ios-region-monitoring.
- Status can move only to review-required when all required artifacts are present; claimApproved remains false here.
- Missing artifacts stay blocking until the P4_PHYSICAL_DEVICE evidence is produced outside local CI.

### Missing Artifacts

- 00-run-metadata.json
- 01-device-metadata.json
- 02-authorization-state.json
- 03-region-definition.json
- 04-location-events.ndjson
- 05-region-transitions.ndjson
- 06-alert-decision.json
- 07-screenshots
- 08-xcode-test-log.txt
- 09-result-summary.md

## child-device-runtime-execution

- blockerId: actual-child-device-runtime-required
- sourceProofRef: test-results/tracking-child-runtime-artifact-gate-proof/proof.json
- proofRoot: output/tracking-plan-proof/child-device-runtime-execution
- status: manual-required
- readinessCategory: physical-device-required
- ciRunnable: false
- missingArtifacts: 10/10

### Required Validation Commands

- Run actual child-device check-in, consent, safe/help, and timeout execution flow on child runtime hardware
- Record delivery envelope, execution result, visible child UI, parent receipt, runtime observations, device log, and summary under output/tracking-plan-proof/child-device-runtime-execution/

### Artifact Acceptance Notes

- Hosted disclosure screenshots do not satisfy rendered child-device runtime UI
- The artifact set must prove delivery and execution, not only copy or readiness

### Claim Audit Acceptance Criteria

- Collect every required artifact under output/tracking-plan-proof/child-runtime-delivery before review.
- Keep required proof tier P4_PHYSICAL_DEVICE; local P3 artifacts cannot approve the claim.
- Cite source proof test-results/tracking-child-runtime-artifact-gate-proof/proof.json and all supporting proof refs in the final handoff.
- Use real device or enrolled child runtime evidence with metadata, logs, screenshots, and transition or execution rows.

### Claim Audit Validation Commands

- node scripts/test/tracking-claim-audit-proof.mjs
- node scripts/test/tracking-product-readiness-closure-proof.mjs
- node scripts/test/tracking-real-runtime-handoff-proof.mjs

### Claim Audit Artifact Notes

- Required artifacts: 7.
- Proof root: output/tracking-plan-proof/child-runtime-delivery.
- Status can move only to review-required when all required artifacts are present; claimApproved remains false here.
- Missing artifacts stay blocking until the P4_PHYSICAL_DEVICE evidence is produced outside local CI.

### Missing Artifacts

- 00-run-metadata.json
- 01-child-device-metadata.json
- 02-delivery-envelope.json
- 03-execution-result.json
- 04-visible-child-ui-snapshot.png
- 05-parent-receipt.json
- 06-runtime-observation.ndjson
- 07-permission-consent-state.json
- 08-device-log.txt
- 09-result-summary.md

## full-product-parent-child-ui-runtime

- blockerId: full-product-parent-child-ui-required
- sourceProofRef: test-results/tracking-full-product-ui-runtime-artifact-gate-proof/proof.json
- proofRoot: output/tracking-plan-proof
- status: manual-required
- readinessCategory: physical-device-required
- ciRunnable: false
- missingArtifacts: 4/9

### Required Validation Commands

- Run full product parent and child UI runtime proof across parent overview, device detail, notifications, retention write, child check-in, child consent, safe/help, accessibility, and end-to-end trace
- Record the required product UI artifacts under output/tracking-plan-proof/product-parent-child-ui-runtime/

### Artifact Acceptance Notes

- Hosted-route screenshots only satisfy local/CI UI inventory, not full product runtime UI
- Child UI artifacts must come from rendered child runtime surfaces

### Claim Audit Acceptance Criteria

- Collect every required artifact under output/tracking-plan-proof/product-parent-child-ui-runtime before review.
- Keep required proof tier P4_PHYSICAL_DEVICE; local P3 artifacts cannot approve the claim.
- Cite source proof test-results/tracking-full-product-ui-runtime-artifact-gate-proof/proof.json and all supporting proof refs in the final handoff.
- Use real device or enrolled child runtime evidence with metadata, logs, screenshots, and transition or execution rows.

### Claim Audit Validation Commands

- node scripts/test/tracking-claim-audit-proof.mjs
- node scripts/test/tracking-product-readiness-closure-proof.mjs
- node scripts/test/tracking-real-runtime-handoff-proof.mjs

### Claim Audit Artifact Notes

- Required artifacts: 9.
- Proof root: output/tracking-plan-proof/product-parent-child-ui-runtime.
- Status can move only to review-required when all required artifacts are present; claimApproved remains false here.
- Missing artifacts stay blocking until the P4_PHYSICAL_DEVICE evidence is produced outside local CI.

### Missing Artifacts

- output/tracking-plan-proof/product-parent-child-ui-runtime/04-retention-settings-production-write-result.png
- output/tracking-plan-proof/product-parent-child-ui-runtime/05-child-device-rendered-check-in-runtime.png
- output/tracking-plan-proof/product-parent-child-ui-runtime/06-child-device-rendered-location-consent-runtime.png
- output/tracking-plan-proof/product-parent-child-ui-runtime/07-child-device-safe-help-response-runtime.png

## authority-enrolled-hard-control-runtime

- blockerId: authority-enrollment-proof-required
- sourceProofRef: test-results/tracking-authority-runtime-artifact-gate-proof/proof.json
- proofRoot: output/tracking-plan-proof
- status: manual-required
- readinessCategory: physical-device-required
- ciRunnable: false
- missingArtifacts: 20/20

### Required Validation Commands

- Run authority-enrolled hard-control tracking proof on enrolled child hardware
- Record enrollment status, control capability state, runtime observation, parent UI, device logs, and summary under output/tracking-plan-proof/authority-runtime/

### Artifact Acceptance Notes

- Manual-required authority rows and unsupported-platform states do not satisfy enrolled hard-control runtime proof
- The artifact set must prove authority status and runtime behavior together

### Claim Audit Acceptance Criteria

- Collect every required artifact under output/tracking-plan-proof/authority-runtime before review.
- Keep required proof tier P4_PHYSICAL_DEVICE; local P3 artifacts cannot approve the claim.
- Cite source proof test-results/tracking-authority-runtime-artifact-gate-proof/proof.json and all supporting proof refs in the final handoff.
- Use real device or enrolled child runtime evidence with metadata, logs, screenshots, and transition or execution rows.

### Claim Audit Validation Commands

- node scripts/test/tracking-claim-audit-proof.mjs
- node scripts/test/tracking-product-readiness-closure-proof.mjs
- node scripts/test/tracking-real-runtime-handoff-proof.mjs

### Claim Audit Artifact Notes

- Required artifacts: 6.
- Proof root: output/tracking-plan-proof/authority-runtime.
- Status can move only to review-required when all required artifacts are present; claimApproved remains false here.
- Missing artifacts stay blocking until the P4_PHYSICAL_DEVICE evidence is produced outside local CI.

### Missing Artifacts

- tracking-authority-android-device-owner-device-identity-proof
- tracking-authority-android-device-owner-enrollment-state-proof
- tracking-authority-android-device-owner-approved-capability-proof
- tracking-authority-android-device-owner-parent-visible-consent-proof
- tracking-authority-android-managed-profile-device-identity-proof
- tracking-authority-android-managed-profile-enrollment-state-proof
- tracking-authority-android-managed-profile-approved-capability-proof
- tracking-authority-android-managed-profile-parent-visible-consent-proof
- tracking-authority-ios-family-controls-entitlement-device-identity-proof
- tracking-authority-ios-family-controls-entitlement-enrollment-state-proof
- tracking-authority-ios-family-controls-entitlement-approved-capability-proof
- tracking-authority-ios-family-controls-entitlement-parent-visible-consent-proof
- tracking-authority-ios-app-review-approval-device-identity-proof
- tracking-authority-ios-app-review-approval-enrollment-state-proof
- tracking-authority-ios-app-review-approval-approved-capability-proof
- tracking-authority-ios-app-review-approval-parent-visible-consent-proof
- tracking-authority-desktop-managed-policy-device-identity-proof
- tracking-authority-desktop-managed-policy-enrollment-state-proof
- tracking-authority-desktop-managed-policy-approved-capability-proof
- tracking-authority-desktop-managed-policy-parent-visible-consent-proof

## provider-delivery-receipt-runtime

- blockerId: provider-delivery-receipt-runtime-required
- sourceProofRef: test-results/tracking-provider-delivery-artifact-gate-proof/proof.json
- proofRoot: output/tracking-plan-proof/notification-provider-delivery
- status: manual-required
- readinessCategory: manual-provider-runtime-required
- ciRunnable: false
- missingArtifacts: 11/11

### Required Validation Commands

- Run provider delivery and receipt ingestion proof with approved credentials and redacted runtime config
- Record provider attempt/response, receipt webhook, ingestion result, retry/quiet-hours worker log, parent notification UI, and summary under output/tracking-plan-proof/provider-delivery-runtime/

### Artifact Acceptance Notes

- Local outbox or preference status proof does not satisfy provider delivery runtime
- Provider credentials must be attested without leaking secrets

### Claim Audit Acceptance Criteria

- Collect every required artifact under output/tracking-plan-proof/provider-delivery-runtime before review.
- Keep required proof tier P4_MANUAL_PROVIDER_RUNTIME; local P3 artifacts cannot approve the claim.
- Cite source proof test-results/tracking-provider-delivery-artifact-gate-proof/proof.json and all supporting proof refs in the final handoff.
- Use real provider request, response, webhook receipt, parent-visible receipt, and provider audit log artifacts.

### Claim Audit Validation Commands

- node scripts/test/tracking-claim-audit-proof.mjs
- node scripts/test/tracking-product-readiness-closure-proof.mjs
- node scripts/test/tracking-real-runtime-handoff-proof.mjs

### Claim Audit Artifact Notes

- Required artifacts: 5.
- Proof root: output/tracking-plan-proof/provider-delivery-runtime.
- Status can move only to review-required when all required artifacts are present; claimApproved remains false here.
- Missing artifacts stay blocking until the P4_MANUAL_PROVIDER_RUNTIME evidence is produced outside local CI.

### Missing Artifacts

- 00-run-metadata.json
- 01-provider-runtime-config-redacted.json
- 02-credential-presence-attestation.json
- 03-minimal-payload-snapshot.json
- 04-provider-attempt.json
- 05-provider-response.json
- 06-receipt-webhook-event.json
- 07-receipt-ingestion-result.json
- 08-retry-quiet-hours-worker-log.txt
- 09-parent-notification-ui-screenshot.png
- 10-result-summary.md

## retention-product-runtime-enforcement

- blockerId: retention-platform-runtime-enforcement-required
- sourceProofRef: test-results/tracking-retention-runtime-artifact-gate-proof/proof.json
- proofRoot: output/tracking-plan-proof
- status: manual-required
- readinessCategory: physical-device-required
- ciRunnable: false
- missingArtifacts: 1/2

### Required Validation Commands

- Run retention product runtime enforcement proof against platform/runtime storage
- Record retention config, persisted cleanup/enforcement event, audit snapshot, UI/result evidence, and summary under output/tracking-plan-proof/retention-runtime/

### Artifact Acceptance Notes

- Local writable settings proof does not satisfy platform runtime retention enforcement
- The artifact set must show enforcement behavior, not only settings persistence

### Claim Audit Acceptance Criteria

- Collect every required artifact under output/tracking-plan-proof/tracking-retention before review.
- Keep required proof tier P4_PHYSICAL_DEVICE; local P3 artifacts cannot approve the claim.
- Cite source proof test-results/tracking-retention-runtime-artifact-gate-proof/proof.json and all supporting proof refs in the final handoff.
- Use real device or enrolled child runtime evidence with metadata, logs, screenshots, and transition or execution rows.

### Claim Audit Validation Commands

- node scripts/test/tracking-claim-audit-proof.mjs
- node scripts/test/tracking-product-readiness-closure-proof.mjs
- node scripts/test/tracking-real-runtime-handoff-proof.mjs

### Claim Audit Artifact Notes

- Required artifacts: 1.
- Proof root: output/tracking-plan-proof/tracking-retention.
- Status can move only to review-required when all required artifacts are present; claimApproved remains false here.
- Missing artifacts stay blocking until the P4_PHYSICAL_DEVICE evidence is produced outside local CI.

### Missing Artifacts

- tracking-retention/platform-runtime-retention-enforcement.json

## production-durable-workers-and-storage

- blockerId: production-durable-workers-required
- sourceProofRef: test-results/tracking-production-worker-runtime-artifact-gate-proof/proof.json
- proofRoot: output/tracking-plan-proof
- status: manual-required
- readinessCategory: production-runtime-required
- ciRunnable: false
- missingArtifacts: 8/8

### Required Validation Commands

- Run production durable worker/storage proof for location upload, retention cleanup, notification outbox, escalation timeout, provider receipt, child-device delivery, authority status, and audit storage
- Record all production worker artifacts under output/tracking-plan-proof/tracking-production/

### Artifact Acceptance Notes

- Local durable stores and production-readiness blockers do not satisfy production worker runtime proof
- Artifacts must come from the approved production-like worker/storage environment

### Claim Audit Acceptance Criteria

- Collect every required artifact under output/tracking-plan-proof/tracking-production before review.
- Keep required proof tier P4_PRODUCTION_RUNTIME; local P3 artifacts cannot approve the claim.
- Cite source proof test-results/tracking-production-worker-runtime-artifact-gate-proof/proof.json and all supporting proof refs in the final handoff.
- Use deployed worker/runtime artifacts plus durable storage evidence from the production environment.

### Claim Audit Validation Commands

- node scripts/test/tracking-claim-audit-proof.mjs
- node scripts/test/tracking-product-readiness-closure-proof.mjs
- node scripts/test/tracking-real-runtime-handoff-proof.mjs

### Claim Audit Artifact Notes

- Required artifacts: 8.
- Proof root: output/tracking-plan-proof/tracking-production.
- Status can move only to review-required when all required artifacts are present; claimApproved remains false here.
- Missing artifacts stay blocking until the P4_PRODUCTION_RUNTIME evidence is produced outside local CI.

### Missing Artifacts

- tracking-production/location-upload-worker-runtime.json
- tracking-production/retention-cleanup-worker-runtime.json
- tracking-production/notification-outbox-worker-runtime.json
- tracking-production/escalation-timeout-worker-runtime.json
- tracking-production/provider-receipt-worker-runtime.json
- tracking-production/child-device-delivery-worker-runtime.json
- tracking-production/authority-status-worker-runtime.json
- tracking-production/audit-durable-storage-runtime.json

## escalation-runtime-workers-and-storage

- blockerId: production-durable-workers-required
- sourceProofRef: test-results/tracking-escalation-runtime-artifact-gate-proof/proof.json
- proofRoot: output/tracking-plan-proof
- status: manual-required
- readinessCategory: production-runtime-required
- ciRunnable: false
- missingArtifacts: 13/13

### Required Validation Commands

- Run escalation timeout worker/storage runtime proof with real durable queue/storage evidence
- Record queue state, timeout execution, parent notification/escalation output, worker logs, and summary under output/tracking-plan-proof/escalation-runtime/

### Artifact Acceptance Notes

- Escalation contract or local fixture proof does not satisfy production worker/storage runtime
- Artifacts must prove timeout execution and durable storage behavior together

### Claim Audit Acceptance Criteria

- Collect every required artifact under output/tracking-plan-proof/tracking-escalation-runtime before review.
- Keep required proof tier P4_PRODUCTION_RUNTIME; local P3 artifacts cannot approve the claim.
- Cite source proof test-results/tracking-escalation-runtime-artifact-gate-proof/proof.json and all supporting proof refs in the final handoff.
- Use deployed worker/runtime artifacts plus durable storage evidence from the production environment.

### Claim Audit Validation Commands

- node scripts/test/tracking-claim-audit-proof.mjs
- node scripts/test/tracking-product-readiness-closure-proof.mjs
- node scripts/test/tracking-real-runtime-handoff-proof.mjs

### Claim Audit Artifact Notes

- Required artifacts: 5.
- Proof root: output/tracking-plan-proof/tracking-escalation-runtime.
- Status can move only to review-required when all required artifacts are present; claimApproved remains false here.
- Missing artifacts stay blocking until the P4_PRODUCTION_RUNTIME evidence is produced outside local CI.

### Missing Artifacts

- test-results/tracking-escalation-readiness-proof/proof.json
- test-results/tracking-provider-runtime-readiness-blocker-proof/proof.json
- 00-run-metadata.json
- 01-provider-runtime-config-redacted.json
- 02-credential-presence-attestation.json
- 03-minimal-payload-snapshot.json
- 04-provider-attempt.json
- 05-provider-response.json
- 06-receipt-webhook-event.json
- 07-receipt-ingestion-result.json
- 08-retry-quiet-hours-worker-log.txt
- 09-parent-notification-ui-screenshot.png
- 10-result-summary.md
