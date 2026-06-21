import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserBoundaryStateSchema,
  BrowserExactUrlClaimStateSchema,
  BrowserUnmanagedDetectionStateSchema,
  BrowserUnmanagedFallbackActionStateSchema,
} from './browser-intervention-schemas';
import {
  BrowserUnmanagedDetectionConfidenceSchema,
  BrowserUnmanagedExecutablePathRefSchema,
  BrowserUnmanagedProcessHashRefSchema,
  type BrowserUnmanagedProcessKind,
  BrowserUnmanagedProcessKindSchema,
  BrowserUnmanagedProcessNameSchema,
  BrowserUnmanagedSignatureRefSchema,
} from './browser-unmanaged-process-schemas';
import {
  BrowserSocialUnmanagedBypassEvidenceIdSchema,
  BrowserSocialUnmanagedBypassReasonsSchema,
  BrowserSocialUnmanagedBypassSchemaVersion,
  BrowserSocialUnmanagedBypassSourceEvidenceIdsSchema,
  BrowserSocialUnmanagedBypassTargetStateSchema,
  OptionalBrowserSocialUnmanagedBypassTextSchema,
} from './browser-social-unmanaged-bypass-detector-values';

const OptionalExecutablePathRefSchema = Schema.Union(BrowserUnmanagedExecutablePathRefSchema, Schema.Null);
const OptionalProcessHashRefSchema = Schema.Union(BrowserUnmanagedProcessHashRefSchema, Schema.Null);
const OptionalSignatureRefSchema = Schema.Union(BrowserUnmanagedSignatureRefSchema, Schema.Null);

const BrowserSocialUnmanagedBypassInputBaseSchema = Schema.Struct({
  bypassEvidenceId: BrowserSocialUnmanagedBypassEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceEvidenceIds: BrowserSocialUnmanagedBypassSourceEvidenceIdsSchema,
  processKind: BrowserUnmanagedProcessKindSchema,
  processName: BrowserUnmanagedProcessNameSchema,
  executablePathRef: OptionalExecutablePathRefSchema,
  processHashRef: OptionalProcessHashRefSchema,
  signatureRef: OptionalSignatureRefSchema,
  confidence: BrowserUnmanagedDetectionConfidenceSchema,
  reasons: BrowserSocialUnmanagedBypassReasonsSchema,
  suspectedPlatformRef: OptionalBrowserSocialUnmanagedBypassTextSchema,
  browserBoundaryState: BrowserBoundaryStateSchema,
  exactUrlClaimState: BrowserExactUrlClaimStateSchema,
  unmanagedDetectionState: BrowserUnmanagedDetectionStateSchema,
  unmanagedFallbackAction: BrowserUnmanagedFallbackActionStateSchema,
});
export const BrowserSocialUnmanagedBypassInputSchema = withParser(
  BrowserSocialUnmanagedBypassInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialUnmanagedBypassInputIsConsistent(value) ||
        'Expected unmanaged social bypass input to remain process-only and non-enforcing'
    )
  )
);

const BrowserSocialUnmanagedBypassEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialUnmanagedBypassSchemaVersion),
  bypassEvidenceId: BrowserSocialUnmanagedBypassEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceEvidenceIds: BrowserSocialUnmanagedBypassSourceEvidenceIdsSchema,
  processKind: BrowserUnmanagedProcessKindSchema,
  processName: BrowserUnmanagedProcessNameSchema,
  executablePathRef: OptionalExecutablePathRefSchema,
  processHashRef: OptionalProcessHashRefSchema,
  signatureRef: OptionalSignatureRefSchema,
  confidence: BrowserUnmanagedDetectionConfidenceSchema,
  reasons: BrowserSocialUnmanagedBypassReasonsSchema,
  suspectedPlatformRef: OptionalBrowserSocialUnmanagedBypassTextSchema,
  targetState: BrowserSocialUnmanagedBypassTargetStateSchema,
  browserBoundaryState: BrowserBoundaryStateSchema,
  exactUrlClaimState: BrowserExactUrlClaimStateSchema,
  unmanagedDetectionState: BrowserUnmanagedDetectionStateSchema,
  unmanagedFallbackAction: BrowserUnmanagedFallbackActionStateSchema,
  managedBrowserRequired: Schema.Boolean,
  bypassOnly: Schema.Boolean,
  exactUrlClaimed: Schema.Boolean,
  routeEvidenceClaimed: Schema.Boolean,
  socialAccountProofClaimed: Schema.Boolean,
  feedVideoRouteClaimed: Schema.Boolean,
  messageContentClaimed: Schema.Boolean,
  accountIdentityClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
  childUiRenderedClaimed: Schema.Boolean,
  parentUiNotifiedClaimed: Schema.Boolean,
  processTerminatedClaimed: Schema.Boolean,
  managedBrowserRelaunchedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});
export const BrowserSocialUnmanagedBypassEvidenceSchema = withParser(
  BrowserSocialUnmanagedBypassEvidenceBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialUnmanagedBypassEvidenceIsConsistent(value) ||
        'Expected unmanaged social bypass evidence to reject exact social proof and runtime claims'
    )
  )
);

export const decodeBrowserSocialUnmanagedBypassEvidence = Schema.decodeUnknownSync(
  BrowserSocialUnmanagedBypassEvidenceSchema
);

export type BrowserSocialUnmanagedBypassInput = Infer<typeof BrowserSocialUnmanagedBypassInputSchema>;
export type BrowserSocialUnmanagedBypassEvidence = Infer<typeof BrowserSocialUnmanagedBypassEvidenceSchema>;

export function detectBrowserSocialUnmanagedBypass(
  input: BrowserSocialUnmanagedBypassInput
): BrowserSocialUnmanagedBypassEvidence {
  const parsed = BrowserSocialUnmanagedBypassInputSchema.parse(input);

  return BrowserSocialUnmanagedBypassEvidenceSchema.parse({
    schemaVersion: BrowserSocialUnmanagedBypassSchemaVersion,
    bypassEvidenceId: parsed.bypassEvidenceId,
    observedAt: parsed.observedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    processKind: parsed.processKind,
    processName: parsed.processName,
    executablePathRef: parsed.executablePathRef,
    processHashRef: parsed.processHashRef,
    signatureRef: parsed.signatureRef,
    confidence: parsed.confidence,
    reasons: parsed.reasons,
    suspectedPlatformRef: parsed.suspectedPlatformRef,
    targetState: bypassTargetStateForInput(parsed),
    browserBoundaryState: parsed.browserBoundaryState,
    exactUrlClaimState: parsed.exactUrlClaimState,
    unmanagedDetectionState: parsed.unmanagedDetectionState,
    unmanagedFallbackAction: parsed.unmanagedFallbackAction,
    managedBrowserRequired: true,
    bypassOnly: true,
    exactUrlClaimed: false,
    routeEvidenceClaimed: false,
    socialAccountProofClaimed: false,
    feedVideoRouteClaimed: false,
    messageContentClaimed: false,
    accountIdentityClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    childUiRenderedClaimed: false,
    parentUiNotifiedClaimed: false,
    processTerminatedClaimed: false,
    managedBrowserRelaunchedClaimed: false,
    enforcementClaimed: false,
  });
}

function socialUnmanagedBypassInputIsConsistent(value: Infer<typeof BrowserSocialUnmanagedBypassInputBaseSchema>) {
  if (value.browserBoundaryState === 'managed-session' || value.exactUrlClaimState === 'exact-url-proven') {
    return false;
  }
  if (!fallbackActionStaysReportOnly(value.unmanagedFallbackAction)) {
    return false;
  }
  return processKindMatchesReasons(value.processKind, value.reasons);
}

function socialUnmanagedBypassEvidenceIsConsistent(
  value: Infer<typeof BrowserSocialUnmanagedBypassEvidenceBaseSchema>
) {
  if (!value.managedBrowserRequired || !value.bypassOnly) {
    return false;
  }
  if (value.targetState === 'manual-required' && !value.reasons.includes('manual-required')) {
    return false;
  }
  if (value.browserBoundaryState === 'managed-session' || value.exactUrlClaimState === 'exact-url-proven') {
    return false;
  }
  return !socialUnmanagedBypassEvidenceClaimsRuntime(value);
}

function bypassTargetStateForInput(value: Infer<typeof BrowserSocialUnmanagedBypassInputBaseSchema>) {
  if (value.unmanagedDetectionState === 'manual-required') {
    return 'manual-required' as const;
  }
  if (value.unmanagedDetectionState === 'unavailable') {
    return 'unavailable' as const;
  }
  return 'bypass-detected' as const;
}

function processKindMatchesReasons(
  processKind: BrowserUnmanagedProcessKind,
  reasons: Infer<typeof BrowserSocialUnmanagedBypassReasonsSchema>
) {
  if (processKind === 'possible-social-bypass') {
    return reasons.includes('possible-social-bypass-process');
  }
  if (processKind === 'supported-browser') {
    return reasons.includes('supported-browser-outside-managed-session');
  }
  if (processKind === 'unsupported-browser') {
    return reasons.includes('unsupported-browser-social-attempt');
  }
  if (processKind === 'portable-browser') {
    return reasons.includes('portable-browser-social-attempt');
  }
  if (processKind === 'tor-privacy-browser') {
    return reasons.includes('tor-browser-social-attempt');
  }
  return reasons.includes('browser-like-social-attempt') || reasons.includes('manual-required');
}

function fallbackActionStaysReportOnly(value: Infer<typeof BrowserUnmanagedFallbackActionStateSchema>) {
  return (
    value === 'report-only' ||
    value === 'warn-child' ||
    value === 'parent-review' ||
    value === 'os-block-manual-required' ||
    value === 'allowed-unmanaged-exception' ||
    value === 'degraded' ||
    value === 'unavailable'
  );
}

function socialUnmanagedBypassEvidenceClaimsRuntime(
  value: Infer<typeof BrowserSocialUnmanagedBypassEvidenceBaseSchema>
) {
  return SocialUnmanagedBypassRuntimeClaimFields.some((field) => value[field] === true);
}

type BrowserSocialUnmanagedBypassEvidenceCandidate = Infer<typeof BrowserSocialUnmanagedBypassEvidenceBaseSchema>;

const SocialUnmanagedBypassRuntimeClaimFields = [
  'exactUrlClaimed',
  'routeEvidenceClaimed',
  'socialAccountProofClaimed',
  'feedVideoRouteClaimed',
  'messageContentClaimed',
  'accountIdentityClaimed',
  'nativeAppControlClaimed',
  'platformConnectorClaimed',
  'childUiRenderedClaimed',
  'parentUiNotifiedClaimed',
  'processTerminatedClaimed',
  'managedBrowserRelaunchedClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserSocialUnmanagedBypassEvidenceCandidate>;
