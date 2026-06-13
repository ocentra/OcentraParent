import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const SocialPlatformConnectorAuthorizationSchemaVersionSchema = withParser(
  Schema.Literal('social-platform-connector-authorization-boundary')
);

export const SocialPlatformConnectorAuthorizationIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialPlatformConnectorAuthorizationId')
);

export const SocialPlatformConnectorProviderSchema = withParser(
  Schema.Literal(
    'google-youtube-supervision',
    'meta-family-center',
    'tiktok-family-pairing',
    'platform-export-import',
    'parent-provided-account-ref'
  )
);

export const SocialPlatformConnectorAuthorizationStateSchema = withParser(
  Schema.Literal('parent-authorized', 'parent-revoked', 'expired', 'manual-required', 'unavailable', 'not-implemented')
);

export const SocialPlatformConnectorProofStateSchema = withParser(
  Schema.Literal(
    'parent-consent-record-only',
    'provider-artifact-required',
    'manual-export-required',
    'revoked',
    'expired',
    'not-implemented',
    'unavailable'
  )
);

export const SocialPlatformConnectorCustodyStateSchema = withParser(
  Schema.Literal(
    'no-token-stored',
    'parent-owned-token-required',
    'manual-export-required',
    'redacted-parent-input-only',
    'not-applicable'
  )
);

export const SocialPlatformConnectorScopeSchema = withParser(
  Schema.Literal(
    'account-supervision-state',
    'family-center-state',
    'family-pairing-state',
    'manual-export-file',
    'parent-declared-account-ref',
    'video-channel-metadata'
  )
);

export const SocialPlatformConnectorAuthorizationReasonSchema = withParser(
  Schema.Literal(
    'optional-adjacent-source',
    'parent-authorization-required',
    'visible-setting-required',
    'provider-api-not-implemented',
    'token-storage-not-implemented',
    'manual-export-required',
    'redacted-input-required',
    'core-gating-independent',
    'message-content-unavailable',
    'feed-content-unavailable'
  )
);

export const SocialPlatformConnectorScopesSchema = Schema.Array(SocialPlatformConnectorScopeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social connector scopes')
);

export const SocialPlatformConnectorReasonsSchema = Schema.Array(SocialPlatformConnectorAuthorizationReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social connector authorization reasons')
);

export const SocialPlatformConnectorProofRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social connector proof refs')
);

export const SocialPlatformConnectorBoundarySchema = brandedNonEmptyStringSchema('SocialPlatformConnectorBoundary');

export type SocialPlatformConnectorProvider = Infer<typeof SocialPlatformConnectorProviderSchema>;
export type SocialPlatformConnectorAuthorizationState = Infer<typeof SocialPlatformConnectorAuthorizationStateSchema>;
export type SocialPlatformConnectorProofState = Infer<typeof SocialPlatformConnectorProofStateSchema>;

