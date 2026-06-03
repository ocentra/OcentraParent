import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const TrackingStatusProofArtifactText = Schema.String.pipe(Schema.minLength(1));

export const TrackingStatusProofArtifactSchema = withParser(
  TrackingStatusProofArtifactText.pipe(Schema.brand('TrackingStatusProofArtifact'))
);
export type TrackingStatusProofArtifact = Infer<typeof TrackingStatusProofArtifactSchema>;

const artifact = TrackingStatusProofArtifactSchema.parse;

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
  TemporaryLiveMode: artifact('output/tracking-plan-proof/28-temporary-live-tracking-mode/proof-summary.json'),
  MissingDeviceMode: artifact('output/tracking-plan-proof/29-missing-device-mode/proof-summary.json'),
  RetentionDelete: artifact('output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json'),
} as const;
