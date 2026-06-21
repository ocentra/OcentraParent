import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import { ParentEvidenceReferenceIdSchema } from './family-reference-primitives';

export const SocialIosScreenTimeCapabilityMatrixSchemaVersionSchema = withParser(
  Schema.Literal('social-ios-screen-time-capability-matrix')
);

export const SocialIosScreenTimeCapabilitySurfaceSchema = withParser(
  Schema.Literal(
    'ios-family-controls-authorization',
    'ios-application-token-selection',
    'ios-web-domain-token-selection',
    'ios-device-activity-monitor',
    'ios-managed-settings-application-shield',
    'ios-managed-settings-web-domain-shield'
  )
);

export const SocialIosScreenTimeTargetKindSchema = withParser(
  Schema.Literal(
    'social-ios-family-authorization',
    'social-ios-app-token',
    'social-ios-web-domain-token',
    'social-ios-device-activity',
    'social-ios-application-shield',
    'social-ios-web-domain-shield'
  )
);

export const SocialIosScreenTimeCapabilityStateSchema = withParser(
  Schema.Literal(
    'entitlement-required',
    'authorization-required',
    'token-selection-required',
    'manual-device-proof-required',
    'unavailable',
    'not-implemented'
  )
);

export const SocialIosScreenTimeProofStateSchema = withParser(
  Schema.Literal(
    'existing-ios-entitlement-proof-ref',
    'apple-entitlement-required',
    'family-authorization-required',
    'manual-device-proof-required',
    'adapter-not-implemented',
    'unavailable'
  )
);

export const SocialIosScreenTimePolicyScopeSchema = withParser(
  Schema.Literal(
    'app-token-level',
    'web-domain-token-level',
    'category-token-level',
    'manual-review-only',
    'not-available'
  )
);

export const SocialIosScreenTimeCapabilityReasonSchema = withParser(
  Schema.Literal(
    'family-controls-entitlement-required',
    'family-authorization-required',
    'opaque-token-required',
    'raw-app-identity-unavailable',
    'web-domain-token-limited',
    'device-activity-entitlement-required',
    'managed-settings-entitlement-required',
    'shield-state-device-proof-required',
    'route-level-unavailable',
    'content-proof-unavailable'
  )
);

export const SocialIosScreenTimeCapabilityReasonsSchema = Schema.Array(SocialIosScreenTimeCapabilityReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected iOS Screen Time capability reasons')
);

export const SocialIosScreenTimeProofRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected iOS Screen Time proof refs')
);

export const SocialIosScreenTimeCapabilityBoundarySchema = brandedNonEmptyStringSchema('SocialIosScreenTimeCapabilityBoundary');

export type SocialIosScreenTimeCapabilitySurface = Infer<typeof SocialIosScreenTimeCapabilitySurfaceSchema>;
export type SocialIosScreenTimeCapabilityState = Infer<typeof SocialIosScreenTimeCapabilityStateSchema>;
export type SocialIosScreenTimeProofState = Infer<typeof SocialIosScreenTimeProofStateSchema>;
