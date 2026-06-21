import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import { ParentEvidenceReferenceIdSchema } from './family-reference-primitives';

export const SocialAndroidNativeAppCapabilityMatrixSchemaVersionSchema = withParser(
  Schema.Literal('social-android-native-app-capability-matrix')
);

export const SocialAndroidNativeAppCapabilitySurfaceSchema = withParser(
  Schema.Literal(
    'android-package-visibility',
    'android-usage-stats-foreground',
    'android-accessibility-route-hints',
    'android-vpn-domain-hints',
    'android-device-owner-app-control',
    'android-managed-profile-config'
  )
);

export const SocialAndroidNativeAppTargetKindSchema = withParser(
  Schema.Literal(
    'social-native-app-presence',
    'social-native-app-foreground',
    'social-native-app-route-hint',
    'social-native-app-domain-hint',
    'social-native-app-blocking',
    'social-native-app-managed-config'
  )
);

export const SocialAndroidNativeAppCapabilityStateSchema = withParser(
  Schema.Literal(
    'app-level-capable-with-proof',
    'permission-required',
    'manual-required',
    'unavailable',
    'not-implemented'
  )
);

export const SocialAndroidNativeAppProofStateSchema = withParser(
  Schema.Literal(
    'existing-schema-domain-proof-ref',
    'permission-grant-required',
    'manual-device-proof-required',
    'adapter-not-implemented',
    'unavailable'
  )
);

export const SocialAndroidNativeAppPolicyScopeSchema = withParser(
  Schema.Literal('app-level-only', 'domain-level-only', 'manual-review-only', 'not-available')
);

export const SocialAndroidNativeAppCapabilityReasonSchema = withParser(
  Schema.Literal(
    'package-visibility-limited',
    'usage-access-required',
    'accessibility-explicit-approval-required',
    'vpn-domain-only',
    'device-owner-required',
    'managed-profile-required',
    'route-level-unavailable',
    'content-proof-unavailable'
  )
);

export const SocialAndroidNativeAppCapabilityReasonsSchema = Schema.Array(
  SocialAndroidNativeAppCapabilityReasonSchema
).pipe(Schema.filter((value) => value.length > 0 || 'Expected Android social native app capability reasons'));

export const SocialAndroidNativeAppProofRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected Android social native app proof refs')
);

export const SocialAndroidNativeAppCapabilityBoundarySchema = brandedNonEmptyStringSchema('SocialAndroidNativeAppCapabilityBoundary');

export type SocialAndroidNativeAppCapabilitySurface = Infer<typeof SocialAndroidNativeAppCapabilitySurfaceSchema>;
export type SocialAndroidNativeAppCapabilityState = Infer<typeof SocialAndroidNativeAppCapabilityStateSchema>;
export type SocialAndroidNativeAppProofState = Infer<typeof SocialAndroidNativeAppProofStateSchema>;
