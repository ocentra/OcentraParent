import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  TrackingPlatformProofRouteStateSchema,
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

const TrackingPlatformProofRouteBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  platform: Schema.Literal('android', 'ios', 'windows', 'macos', 'linux', 'web'),
  foregroundLocation: TrackingPlatformProofRouteStateSchema,
  backgroundLocation: TrackingPlatformProofRouteStateSchema,
  geofence: TrackingPlatformProofRouteStateSchema,
  deviceStatus: TrackingPlatformProofRouteStateSchema,
  proofArtifactRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  manualRequiredReason: Schema.Union(TrackingPolicyReasonCodeSchema, Schema.Null),
});

export const TrackingPlatformProofRouteSchema = withParser(
  TrackingPlatformProofRouteBaseSchema.pipe(
    Schema.filter(
      (route) =>
        trackingPlatformContractProofHasArtifacts(route) ||
        'Tracking platform proof routes with contract-proved capability states require proof artifact references'
    )
  )
);

type TrackingPlatformProofRouteBase = Infer<typeof TrackingPlatformProofRouteBaseSchema>;

function trackingPlatformContractProofHasArtifacts(route: TrackingPlatformProofRouteBase) {
  const hasContractProof =
    route.foregroundLocation === 'contract-proved' ||
    route.backgroundLocation === 'contract-proved' ||
    route.geofence === 'contract-proved' ||
    route.deviceStatus === 'contract-proved';

  return !hasContractProof || route.proofArtifactRefs.length > 0;
}
