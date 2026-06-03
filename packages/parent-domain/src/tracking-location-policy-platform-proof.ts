import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  TrackingPlatformProofRouteStateSchema,
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

export const TrackingPlatformProofRouteSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    platform: Schema.Literal('android', 'ios', 'windows', 'macos', 'linux', 'web'),
    foregroundLocation: TrackingPlatformProofRouteStateSchema,
    backgroundLocation: TrackingPlatformProofRouteStateSchema,
    geofence: TrackingPlatformProofRouteStateSchema,
    deviceStatus: TrackingPlatformProofRouteStateSchema,
    proofArtifactRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    manualRequiredReason: Schema.Union(TrackingPolicyReasonCodeSchema, Schema.Null),
  })
);
