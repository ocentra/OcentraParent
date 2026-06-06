import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { SocialVideoSourcePrivacyEvidenceIdSchema } from './social-video-source-privacy';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivitySubjectIdSchema,
  ActivityTimestampSchema,
} from './primitives';

const NonEmptySocialVideoSourceCustodyText = Schema.String.pipe(Schema.minLength(1));
const OptionalSourceCustodyRefsSchema = Schema.Array(NonEmptySocialVideoSourceCustodyText);
const SourceCustodyEvidenceRefsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social/video source custody evidence refs')
);
const SourceCustodyPrivacyRefsSchema = Schema.Array(SocialVideoSourcePrivacyEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social/video source privacy refs')
);

export const SocialVideoSourceCustodySettingsSchemaVersion = 1;

export const SocialVideoSourceCustodySettingsIdSchema = withParser(
  NonEmptySocialVideoSourceCustodyText.pipe(Schema.brand('SocialVideoSourceCustodySettingsId'))
);

export const SocialVideoSourceCustodyPermissionStateSchema = withParser(
  Schema.Literal('enabled', 'disabled', 'parent-review-required', 'manual-required', 'unavailable')
);
export const SocialVideoSourceCustodyModeSchema = withParser(
  Schema.Literal(
    'local-redacted-refs-only',
    'parent-provided-refs-only',
    'connector-authorization-ref-only',
    'screen-summary-ref-only',
    'manual-required'
  )
);
export const SocialVideoSourceCustodySettingScopeSchema = withParser(
  Schema.Literal(
    'managed-browser-social-route',
    'bounded-video-metadata',
    'parent-provided-url-channel',
    'connector-authorization-ref',
    'screen-summary-ref',
    'native-platform-manual-required'
  )
);
export const SocialVideoSourceCustodyDownstreamUseSchema = withParser(
  Schema.Literal('ai-candidate-input', 'policy-candidate-input', 'parent-explanation', 'manual-review', 'audit-summary')
);
export const SocialVideoSourceCustodyRetentionModeSchema = withParser(
  Schema.Literal('no-raw-retention', 'redacted-ref-journal-only', 'manual-required')
);
export const SocialVideoSourceCustodyNoClaimSchema = withParser(
  Schema.Literal(
    'raw-message-content-not-allowed',
    'raw-video-content-not-allowed',
    'screenshot-custody-not-allowed',
    'connector-token-not-stored',
    'connector-api-not-called',
    'runtime-settings-ui-not-claimed',
    'runtime-custody-mutation-not-claimed',
    'final-policy-decision-not-claimed',
    'enforcement-not-claimed'
  )
);

const SocialVideoSourceCustodyUsesSchema = Schema.Array(SocialVideoSourceCustodyDownstreamUseSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social/video source custody downstream uses')
);
const SocialVideoSourceCustodyNoClaimsSchema = Schema.Array(SocialVideoSourceCustodyNoClaimSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social/video source custody no-claim labels')
);

const SocialVideoSourceCustodySettingsBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(SocialVideoSourceCustodySettingsSchemaVersion),
  settingsId: SocialVideoSourceCustodySettingsIdSchema,
  generatedAt: ActivityTimestampSchema,
  childProfileRef: ActivitySubjectIdSchema,
  deviceId: ActivityDeviceIdSchema,
  sourcePrivacyEvidenceIds: SourceCustodyPrivacyRefsSchema,
  evidenceRefs: SourceCustodyEvidenceRefsSchema,
  settingScope: SocialVideoSourceCustodySettingScopeSchema,
  permissionState: SocialVideoSourceCustodyPermissionStateSchema,
  custodyMode: SocialVideoSourceCustodyModeSchema,
  retentionMode: SocialVideoSourceCustodyRetentionModeSchema,
  permittedDownstreamUses: SocialVideoSourceCustodyUsesSchema,
  disabledUseReasons: OptionalSourceCustodyRefsSchema,
  parentReviewRefs: OptionalSourceCustodyRefsSchema,
  connectorAuthorizationRefs: OptionalSourceCustodyRefsSchema,
  manualProofRequirements: OptionalSourceCustodyRefsSchema,
  noClaimLabels: SocialVideoSourceCustodyNoClaimsSchema,
  rawMessageContentAllowed: Schema.Boolean,
  rawVideoContentAllowed: Schema.Boolean,
  screenshotCustodyAllowed: Schema.Boolean,
  connectorTokenStored: Schema.Boolean,
  connectorApiCalled: Schema.Boolean,
  runtimeSettingsUiClaimed: Schema.Boolean,
  runtimeCustodyMutationClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type SocialVideoSourceCustodySettingsCandidate = Infer<typeof SocialVideoSourceCustodySettingsBaseSchema>;

export const SocialVideoSourceCustodySettingsSchema = withParser(
  SocialVideoSourceCustodySettingsBaseSchema.pipe(
    Schema.filter(
      (settings) =>
        socialVideoSourceCustodyHasRequiredNoClaims(settings) ||
        'Expected social/video source custody settings to include required no-claim labels'
    )
  )
    .pipe(
      Schema.filter(
        (settings) =>
          socialVideoSourceCustodyAvoidsForbiddenClaims(settings) ||
          'Expected social/video source custody settings to reject raw custody connector runtime policy and enforcement claims'
      )
    )
    .pipe(
      Schema.filter(
        (settings) =>
          socialVideoSourceCustodyPermissionIsHonest(settings) ||
          'Expected disabled/manual/unavailable social/video source custody settings to stay out of policy input'
      )
    )
    .pipe(
      Schema.filter(
        (settings) =>
          socialVideoSourceCustodyModeMatchesRefs(settings) ||
          'Expected social/video source custody mode to cite matching parent connector screen or manual refs'
      )
    )
);

const SocialVideoSourceCustodySettingsInputSchema = withParser(
  SocialVideoSourceCustodySettingsBaseSchema.omit(
    'schemaVersion',
    'rawMessageContentAllowed',
    'rawVideoContentAllowed',
    'screenshotCustodyAllowed',
    'connectorTokenStored',
    'connectorApiCalled',
    'runtimeSettingsUiClaimed',
    'runtimeCustodyMutationClaimed',
    'finalPolicyDecisionClaimed',
    'enforcementClaimed'
  )
);

export const decodeSocialVideoSourceCustodySettings = Schema.decodeUnknownSync(SocialVideoSourceCustodySettingsSchema);

export type SocialVideoSourceCustodyDownstreamUse = Infer<typeof SocialVideoSourceCustodyDownstreamUseSchema>;
export type SocialVideoSourceCustodyMode = Infer<typeof SocialVideoSourceCustodyModeSchema>;
export type SocialVideoSourceCustodyNoClaim = Infer<typeof SocialVideoSourceCustodyNoClaimSchema>;
export type SocialVideoSourceCustodyPermissionState = Infer<typeof SocialVideoSourceCustodyPermissionStateSchema>;
export type SocialVideoSourceCustodySettingScope = Infer<typeof SocialVideoSourceCustodySettingScopeSchema>;
export type SocialVideoSourceCustodySettings = Infer<typeof SocialVideoSourceCustodySettingsSchema>;
export type SocialVideoSourceCustodySettingsId = Infer<typeof SocialVideoSourceCustodySettingsIdSchema>;

const RequiredNoClaimLabels: ReadonlyArray<SocialVideoSourceCustodyNoClaim> = [
  'raw-message-content-not-allowed',
  'raw-video-content-not-allowed',
  'screenshot-custody-not-allowed',
  'connector-token-not-stored',
  'connector-api-not-called',
  'runtime-settings-ui-not-claimed',
  'runtime-custody-mutation-not-claimed',
  'final-policy-decision-not-claimed',
  'enforcement-not-claimed',
];

export function buildSocialVideoSourceCustodySettings(
  input: Infer<typeof SocialVideoSourceCustodySettingsInputSchema>
): SocialVideoSourceCustodySettings {
  const parsed = SocialVideoSourceCustodySettingsInputSchema.parse(input);

  return SocialVideoSourceCustodySettingsSchema.parse({
    ...parsed,
    schemaVersion: SocialVideoSourceCustodySettingsSchemaVersion,
    rawMessageContentAllowed: false,
    rawVideoContentAllowed: false,
    screenshotCustodyAllowed: false,
    connectorTokenStored: false,
    connectorApiCalled: false,
    runtimeSettingsUiClaimed: false,
    runtimeCustodyMutationClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
  });
}

function socialVideoSourceCustodyHasRequiredNoClaims(settings: SocialVideoSourceCustodySettingsCandidate): boolean {
  const labels = new Set(settings.noClaimLabels);
  return RequiredNoClaimLabels.every((label) => labels.has(label));
}

function socialVideoSourceCustodyAvoidsForbiddenClaims(settings: SocialVideoSourceCustodySettingsCandidate): boolean {
  return (
    !settings.rawMessageContentAllowed &&
    !settings.rawVideoContentAllowed &&
    !settings.screenshotCustodyAllowed &&
    !settings.connectorTokenStored &&
    !settings.connectorApiCalled &&
    !settings.runtimeSettingsUiClaimed &&
    !settings.runtimeCustodyMutationClaimed &&
    !settings.finalPolicyDecisionClaimed &&
    !settings.enforcementClaimed
  );
}

function socialVideoSourceCustodyPermissionIsHonest(settings: SocialVideoSourceCustodySettingsCandidate): boolean {
  if (settings.permissionState === 'enabled') {
    return settings.disabledUseReasons.length === 0 && settings.manualProofRequirements.length === 0;
  }
  if (settings.permissionState === 'parent-review-required') {
    return (
      !settings.permittedDownstreamUses.includes('policy-candidate-input') &&
      settings.parentReviewRefs.length > 0 &&
      settings.disabledUseReasons.length > 0
    );
  }
  return (
    !settings.permittedDownstreamUses.includes('policy-candidate-input') &&
    settings.disabledUseReasons.length > 0 &&
    settings.manualProofRequirements.length > 0
  );
}

function socialVideoSourceCustodyModeMatchesRefs(settings: SocialVideoSourceCustodySettingsCandidate): boolean {
  if (settings.retentionMode === 'manual-required') {
    return settings.custodyMode === 'manual-required' && settings.manualProofRequirements.length > 0;
  }
  if (settings.custodyMode === 'parent-provided-refs-only') {
    return settings.settingScope === 'parent-provided-url-channel';
  }
  if (settings.custodyMode === 'connector-authorization-ref-only') {
    return settings.connectorAuthorizationRefs.length > 0 && settings.settingScope === 'connector-authorization-ref';
  }
  if (settings.custodyMode === 'screen-summary-ref-only') {
    return settings.settingScope === 'screen-summary-ref';
  }
  if (settings.custodyMode === 'manual-required') {
    return settings.manualProofRequirements.length > 0;
  }
  return true;
}
