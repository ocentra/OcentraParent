/* generated from crates/browser-core/src/browser_generated_social_ts.rs */

import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserUrlShapeClassificationIdSchema,
  BrowserUrlShapeTargetKindSchema,
} from './generated-browser-url-intelligence-schemas';
const OptionalBrowserSocialRouteTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const OptionalUrlShapeClassificationIdSchema = Schema.Union(BrowserUrlShapeClassificationIdSchema, Schema.Null);
const OptionalUrlShapeTargetKindSchema = Schema.Union(BrowserUrlShapeTargetKindSchema, Schema.Null);

export const BrowserSocialRouteSchemaVersion = 1;

export const BrowserSocialRouteEvidenceIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialRouteEvidenceId')
);

export const BrowserSocialPlatformSchema = withParser(
  Schema.Literal(
    'facebook',
    'instagram',
    'messenger',
    'tiktok',
    'snapchat',
    'youtube',
    'youtube-shorts',
    'vimeo',
    'twitch',
    'discord',
    'reddit',
    'x-twitter',
    'pinterest',
    'roblox',
    'generic-social',
    'unknown-social'
  )
);

export const BrowserSocialRouteKindSchema = withParser(
  Schema.Literal(
    'platform-access',
    'account-signup',
    'login',
    'account-switch',
    'profile',
    'feed',
    'short-video-feed',
    'video',
    'post',
    'livestream',
    'messaging-route',
    'upload-post',
    'settings-privacy',
    'unknown-social-route'
  )
);

export const BrowserSocialRouteSourceKindSchema = withParser(
  Schema.Literal('managed-browser-url-shape', 'unmanaged-browser-bypass', 'native-app-manual-required')
);

export const BrowserSocialRouteProofStateSchema = withParser(
  Schema.Literal('route-evidence', 'bypass-only', 'manual-required', 'unavailable')
);

const BrowserSocialRouteEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialRouteSchemaVersion),
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceEvidenceIds: Schema.Array(ActivityEvidenceIdSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected at least one social route source evidence id')
  ),
  urlShapeClassificationId: OptionalUrlShapeClassificationIdSchema,
  urlShapeTargetKind: OptionalUrlShapeTargetKindSchema,
  sourceKind: BrowserSocialRouteSourceKindSchema,
  proofState: BrowserSocialRouteProofStateSchema,
  platform: BrowserSocialPlatformSchema,
  routeKind: BrowserSocialRouteKindSchema,
  platformAccountRef: OptionalBrowserSocialRouteTextSchema,
  parentApprovalRequestRef: OptionalBrowserSocialRouteTextSchema,
  exactManagedBrowserRouteEvidence: Schema.Boolean,
  unmanagedBypassOnly: Schema.Boolean,
  manualRequired: Schema.Boolean,
  accountIdentityClaimed: Schema.Boolean,
  messageContentClaimed: Schema.Boolean,
  feedContentSemanticsClaimed: Schema.Boolean,
  aiDecisionClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
});

export const BrowserSocialRouteEvidenceSchema = withParser(
  BrowserSocialRouteEvidenceBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserSocialRouteEvidenceIsConsistent(value) ||
        'Expected browser social route evidence to preserve source, privacy, policy, and enforcement boundaries'
    )
  )
);

export const decodeBrowserSocialRouteEvidence = Schema.decodeUnknownSync(BrowserSocialRouteEvidenceSchema);

export type BrowserSocialPlatform = Infer<typeof BrowserSocialPlatformSchema>;
export type BrowserSocialRouteEvidence = Infer<typeof BrowserSocialRouteEvidenceSchema>;
export type BrowserSocialRouteEvidenceId = Infer<typeof BrowserSocialRouteEvidenceIdSchema>;
export type BrowserSocialRouteKind = Infer<typeof BrowserSocialRouteKindSchema>;
export type BrowserSocialRouteProofState = Infer<typeof BrowserSocialRouteProofStateSchema>;
export type BrowserSocialRouteSourceKind = Infer<typeof BrowserSocialRouteSourceKindSchema>;

function browserSocialRouteEvidenceIsConsistent(value: Infer<typeof BrowserSocialRouteEvidenceBaseSchema>) {
  if (socialRouteEvidenceClaimsAuthority(value)) {
    return false;
  }
  if (value.sourceKind === 'managed-browser-url-shape') {
    return managedBrowserSocialRouteEvidenceIsConsistent(value);
  }
  if (value.sourceKind === 'unmanaged-browser-bypass') {
    return unmanagedSocialBypassEvidenceIsConsistent(value);
  }
  return nativeSocialManualRequiredEvidenceIsConsistent(value);
}

function socialRouteEvidenceClaimsAuthority(value: Infer<typeof BrowserSocialRouteEvidenceBaseSchema>) {
  return (
    value.accountIdentityClaimed ||
    value.messageContentClaimed ||
    value.feedContentSemanticsClaimed ||
    value.aiDecisionClaimed ||
    value.policyDecisionClaimed ||
    value.enforcementClaimed ||
    value.nativeAppControlClaimed ||
    value.platformConnectorClaimed
  );
}

function managedBrowserSocialRouteEvidenceIsConsistent(value: Infer<typeof BrowserSocialRouteEvidenceBaseSchema>) {
  return (
    value.urlShapeClassificationId !== null &&
    value.urlShapeTargetKind !== null &&
    value.proofState === 'route-evidence' &&
    value.platform !== 'unknown-social' &&
    value.routeKind !== 'unknown-social-route' &&
    value.exactManagedBrowserRouteEvidence &&
    !value.unmanagedBypassOnly &&
    !value.manualRequired
  );
}

function unmanagedSocialBypassEvidenceIsConsistent(value: Infer<typeof BrowserSocialRouteEvidenceBaseSchema>) {
  return (
    value.urlShapeClassificationId === null &&
    value.urlShapeTargetKind === null &&
    value.proofState === 'bypass-only' &&
    value.routeKind === 'unknown-social-route' &&
    !value.exactManagedBrowserRouteEvidence &&
    value.unmanagedBypassOnly &&
    value.manualRequired &&
    value.platformAccountRef === null &&
    value.parentApprovalRequestRef === null
  );
}

function nativeSocialManualRequiredEvidenceIsConsistent(value: Infer<typeof BrowserSocialRouteEvidenceBaseSchema>) {
  return (
    value.urlShapeClassificationId === null &&
    value.urlShapeTargetKind === null &&
    value.proofState === 'manual-required' &&
    value.routeKind === 'unknown-social-route' &&
    !value.exactManagedBrowserRouteEvidence &&
    !value.unmanagedBypassOnly &&
    value.manualRequired &&
    value.platformAccountRef === null &&
    value.parentApprovalRequestRef === null
  );
}
