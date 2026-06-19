import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  BrowserGameUnblockedSiteActionCandidateSchema,
  BrowserGameUnblockedSiteClassificationKindSchema,
  BrowserGameUnblockedSiteConfidenceSchema,
  BrowserGameUnblockedSiteDetectionIdSchema,
  BrowserGameUnblockedSiteDetectionSchemaVersionSchema,
  BrowserGameUnblockedSiteDetectionStateSchema,
  BrowserGameUnblockedSiteEvidenceRefsSchema,
  BrowserGameUnblockedSiteReasonCodesSchema,
  BrowserGameUnblockedSiteSignalIdSchema,
  BrowserGameUnblockedSiteSignalKindSchema,
  BrowserGameUnblockedSiteSurfaceKindSchema,
} from './browser-game-unblocked-site-detection-values';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const OptionalParentPolicyVersionSchema = Schema.Union(ParentPolicyVersionSchema, Schema.Null);

const BrowserGameUnblockedSiteSignalBaseSchema = Schema.Struct({
  signalId: BrowserGameUnblockedSiteSignalIdSchema,
  signalKind: BrowserGameUnblockedSiteSignalKindSchema,
  surfaceKind: BrowserGameUnblockedSiteSurfaceKindSchema,
  detectionState: BrowserGameUnblockedSiteDetectionStateSchema,
  confidence: BrowserGameUnblockedSiteConfidenceSchema,
  evidenceRefs: BrowserGameUnblockedSiteEvidenceRefsSchema,
  rawUrlStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  rawSearchQueryStored: Schema.Boolean,
  iframeContentCaptured: Schema.Boolean,
  exactUnmanagedUrlClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameUnblockedSiteSignalCandidate = Infer<typeof BrowserGameUnblockedSiteSignalBaseSchema>;

const BrowserGameUnblockedSiteSignalsSchema = Schema.Array(
  withParser(
    BrowserGameUnblockedSiteSignalBaseSchema.pipe(
      Schema.filter(
        (signal) =>
          browserGameUnblockedSiteSignalIsHonest(signal) ||
          'Expected browser-game unblocked-site signal to stay evidence-ref backed'
      )
    )
  )
);

const BrowserGameUnblockedSiteDetectionBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameUnblockedSiteDetectionSchemaVersionSchema,
  detectionId: BrowserGameUnblockedSiteDetectionIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  detectedAt: ParentTimestampSchema,
  surfaceKind: BrowserGameUnblockedSiteSurfaceKindSchema,
  classificationKind: BrowserGameUnblockedSiteClassificationKindSchema,
  detectionState: BrowserGameUnblockedSiteDetectionStateSchema,
  confidence: BrowserGameUnblockedSiteConfidenceSchema,
  sourceEvidenceRefs: BrowserGameUnblockedSiteEvidenceRefsSchema,
  signalRows: BrowserGameUnblockedSiteSignalsSchema,
  actionCandidate: BrowserGameUnblockedSiteActionCandidateSchema,
  managedRouteEvidenceRef: OptionalParentEvidenceRefSchema,
  portalIndexEvidenceRef: OptionalParentEvidenceRefSchema,
  iframeEvidenceRef: OptionalParentEvidenceRefSchema,
  searchIntentEvidenceRef: OptionalParentEvidenceRefSchema,
  unmanagedProcessEvidenceRef: OptionalParentEvidenceRefSchema,
  parentPolicyRef: OptionalParentPolicyVersionSchema,
  reasonCodes: BrowserGameUnblockedSiteReasonCodesSchema,
  deliveryState: Schema.Literal('contract-only'),
  rawUrlStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  rawSearchQueryStored: Schema.Boolean,
  iframeContentCaptured: Schema.Boolean,
  exactUnmanagedUrlClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  accountOrPurchaseFlowClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameUnblockedSiteDetectionCandidate = Infer<typeof BrowserGameUnblockedSiteDetectionBaseSchema>;

export const BrowserGameUnblockedSiteSignalSchema = withParser(
  BrowserGameUnblockedSiteSignalBaseSchema.pipe(
    Schema.filter(
      (signal) =>
        browserGameUnblockedSiteSignalIsHonest(signal) ||
        'Expected browser-game unblocked-site signal to stay evidence-ref backed'
    )
  )
);

export const BrowserGameUnblockedSiteDetectionSchema = withParser(
  BrowserGameUnblockedSiteDetectionBaseSchema.pipe(
    Schema.filter(
      (detection) =>
        browserGameUnblockedSiteDetectionIsHonest(detection) ||
        'Expected browser-game unblocked-site detection to stay candidate-only and non-executing'
    )
  )
);

export const decodeBrowserGameUnblockedSiteDetection = Schema.decodeUnknownSync(
  BrowserGameUnblockedSiteDetectionSchema
);

export type BrowserGameUnblockedSiteDetection = Infer<typeof BrowserGameUnblockedSiteDetectionSchema>;
export type BrowserGameUnblockedSiteSignal = Infer<typeof BrowserGameUnblockedSiteSignalSchema>;

type BrowserGameUnblockedSiteActionValidator = (detection: BrowserGameUnblockedSiteDetectionCandidate) => boolean;

const BrowserGameUnblockedSiteActionValidators = {
  'block-during-school-candidate': blockDuringSchoolActionIsSupported,
  'parent-review-candidate': askParentActionIsSupported,
  'allow-specific-game-candidate': allowSpecificGameActionIsSupported,
  'block-unknown-iframe-candidate': blockUnknownIframeActionIsSupported,
  'bypass-evidence-only-candidate': bypassEvidenceOnlyActionIsSupported,
  'manual-review-candidate': unsupportedUnblockedSiteCandidateAction,
  'unknown-candidate': unsupportedUnblockedSiteCandidateAction,
} satisfies Record<
  BrowserGameUnblockedSiteDetectionCandidate['actionCandidate'],
  BrowserGameUnblockedSiteActionValidator
>;

function browserGameUnblockedSiteSignalIsHonest(signal: BrowserGameUnblockedSiteSignalCandidate): boolean {
  if (browserGameUnblockedSiteSignalClaimsAuthority(signal)) {
    return false;
  }
  if (signal.detectionState === 'candidate') {
    return signal.signalKind !== 'unknown-signal' && signal.confidence !== 'unknown';
  }
  return signal.signalKind === 'unknown-signal' && signal.confidence === 'unknown';
}

function browserGameUnblockedSiteDetectionIsHonest(detection: BrowserGameUnblockedSiteDetectionCandidate): boolean {
  if (browserGameUnblockedSiteDetectionClaimsAuthority(detection) || detection.signalRows.length === 0) {
    return false;
  }
  if (detection.detectionState === 'candidate') {
    return (
      detection.classificationKind !== 'unknown' &&
      detection.confidence !== 'unknown' &&
      detection.surfaceKind !== 'manual-required' &&
      detection.surfaceKind !== 'unavailable' &&
      browserGameUnblockedSiteActionIsSupported(detection)
    );
  }
  return (
    detection.classificationKind === 'unknown' &&
    detection.confidence === 'unknown' &&
    (detection.actionCandidate === 'manual-review-candidate' || detection.actionCandidate === 'unknown-candidate') &&
    detection.reasonCodes.includes('manual-required')
  );
}

function browserGameUnblockedSiteActionIsSupported(detection: BrowserGameUnblockedSiteDetectionCandidate): boolean {
  return BrowserGameUnblockedSiteActionValidators[detection.actionCandidate](detection);
}

function bypassEvidenceOnlyActionIsSupported(detection: BrowserGameUnblockedSiteDetectionCandidate): boolean {
  return (
    detection.classificationKind === 'unmanaged-browser-game-bypass' &&
    detection.surfaceKind === 'unmanaged-browser-bypass' &&
    detection.unmanagedProcessEvidenceRef !== null &&
    detection.managedRouteEvidenceRef === null &&
    detection.reasonCodes.includes('unmanaged-browser-process-only')
  );
}

function blockDuringSchoolActionIsSupported(detection: BrowserGameUnblockedSiteDetectionCandidate): boolean {
  return (
    detectionHasManagedRoute(detection) &&
    detection.parentPolicyRef !== null &&
    detection.reasonCodes.includes('school-bypass-portal') &&
    hasCandidateSignal(detection, 'school-bypass-language')
  );
}

function askParentActionIsSupported(detection: BrowserGameUnblockedSiteDetectionCandidate): boolean {
  return (
    detectionHasManagedRoute(detection) &&
    detection.portalIndexEvidenceRef !== null &&
    hasAnyCandidateSignal(detection, ['unblocked-domain-keyword', 'game-portal-index'])
  );
}

function allowSpecificGameActionIsSupported(detection: BrowserGameUnblockedSiteDetectionCandidate): boolean {
  return (
    detectionHasManagedRoute(detection) &&
    detection.parentPolicyRef !== null &&
    detection.reasonCodes.includes('portal-index-detected')
  );
}

function blockUnknownIframeActionIsSupported(detection: BrowserGameUnblockedSiteDetectionCandidate): boolean {
  return (
    detectionHasManagedRoute(detection) &&
    detection.iframeEvidenceRef !== null &&
    detection.reasonCodes.includes('hidden-game-origin') &&
    hasCandidateSignal(detection, 'external-game-iframe')
  );
}

function unsupportedUnblockedSiteCandidateAction(): boolean {
  return false;
}

function detectionHasManagedRoute(detection: BrowserGameUnblockedSiteDetectionCandidate): boolean {
  return detection.managedRouteEvidenceRef !== null;
}

function hasAnyCandidateSignal(
  detection: BrowserGameUnblockedSiteDetectionCandidate,
  signalKinds: ReadonlyArray<BrowserGameUnblockedSiteSignalCandidate['signalKind']>
): boolean {
  return signalKinds.some((signalKind) => hasCandidateSignal(detection, signalKind));
}

function hasCandidateSignal(
  detection: BrowserGameUnblockedSiteDetectionCandidate,
  signalKind: BrowserGameUnblockedSiteSignalCandidate['signalKind']
): boolean {
  return detection.signalRows.some(
    (signal) => signal.detectionState === 'candidate' && signal.signalKind === signalKind
  );
}

function browserGameUnblockedSiteSignalClaimsAuthority(signal: BrowserGameUnblockedSiteSignalCandidate): boolean {
  return (
    signal.rawUrlStored ||
    signal.rawPageBodyStored ||
    signal.rawSearchQueryStored ||
    signal.iframeContentCaptured ||
    signal.exactUnmanagedUrlClaimed ||
    signal.policyDecisionClaimed ||
    signal.runtimeGateExecutedClaimed ||
    signal.enforcementClaimed
  );
}

function browserGameUnblockedSiteDetectionClaimsAuthority(
  detection: BrowserGameUnblockedSiteDetectionCandidate
): boolean {
  return (
    detection.rawUrlStored ||
    detection.rawPageBodyStored ||
    detection.rawSearchQueryStored ||
    detection.iframeContentCaptured ||
    detection.exactUnmanagedUrlClaimed ||
    detection.nativeGameControlClaimed ||
    detection.cloudFrameAnalysisClaimed ||
    detection.accountOrPurchaseFlowClaimed ||
    detection.uiRenderedClaimed ||
    detection.finalPolicyDecisionClaimed ||
    detection.runtimeGateExecutedClaimed ||
    detection.enforcementClaimed
  );
}
