import { decodeTrackingStatusProofArtifact } from './portal-contract-text-contracts';
import { GeneratedPortalTrackingContracts } from './generated-portal-contracts';

const artifact = decodeTrackingStatusProofArtifact;

export const TrackingStatusProofArtifacts = {
  ContractBoundary: artifact('output/tracking-plan-proof/03-contract-boundary-and-effect-schemas/proof-summary.json'),
  PermissionCapability: artifact(
    'output/tracking-plan-proof/06-permission-and-capability-status-model/proof-summary.json'
  ),
  RuntimeLocationEvidence: artifact(
    'output/tracking-plan-proof/04-location-evidence-model/03-runtime-location-evidence.json'
  ),
  DeviceStatus: artifact('output/tracking-plan-proof/05-device-status-model/04-device-status-proof.json'),
  NearbyPlace: artifact('output/tracking-plan-proof/19-nearby-place-provider-abstraction/proof-summary.json'),
  AlertSeverity: artifact('output/tracking-plan-proof/26-alert-severity-and-notification-model/proof-summary.json'),
  ParentAcknowledgement: artifact(
    'output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/09-policy-alert-proof.json'
  ),
  ChildCheckIn: artifact('output/tracking-plan-proof/18-child-check-in-flow/09-policy-alert-proof.json'),
  ChildRuntimeUi: artifact(
    'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/19-child-runtime-ui-proof.json'
  ),
  FamilyDashboardRollup: artifact(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/23-family-dashboard-rollup-proof.json'
  ),
  RetentionSettingsReadModel: artifact(
    GeneratedPortalTrackingContracts.RetentionSettingsWrite.Defaults.ReadModelProofRefs[1]
  ),
  ReportExportReadModel: artifact(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/28-report-export-read-model-proof.json'
  ),
  ReportPolicyConsumer: artifact(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json'
  ),
  NotificationParentSurfaceHistory: artifact(
    'output/tracking-plan-proof/26-alert-severity-and-notification-model/26-notification-parent-surface-history-proof.json'
  ),
  ExpectedPlaceAlertPolicy: artifact(
    'output/tracking-plan-proof/16-expected-place-schedule-engine/29-expected-place-alert-policy-proof.json'
  ),
  ParentAcknowledgementActionReadiness: artifact(
    'output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/30-parent-acknowledgement-action-readiness-proof.json'
  ),
  RetentionSettingsWriteCommand: artifact(
    'output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json'
  ),
  RetentionLocalServiceState: artifact(
    'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json'
  ),
  HostedEvidenceDrawer: artifact(
    'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/20-evidence-drawer-hosted-ui-proof.json'
  ),
  UnsupportedManualPlatform: artifact('output/tracking-plan-proof/unsupported-platform-manual-proof/proof.json'),
  TemporaryLiveMode: artifact('output/tracking-plan-proof/28-temporary-live-tracking-mode/proof-summary.json'),
  MissingDeviceMode: artifact('output/tracking-plan-proof/29-missing-device-mode/proof.json'),
  RetentionDelete: artifact('output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json'),
} as const;
