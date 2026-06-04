import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  BrowserAiAnalysisSchemaVersion,
  BrowserUrlAiAnalysisInputSchema,
  BrowserUrlAiAnalysisResultSchema,
  type BrowserUrlAiAnalysisInput,
  type BrowserUrlAiAnalysisResult,
} from './browser-ai-analysis-schemas';
import { ActivityEvidenceRefSchema } from './contracts';
import { ScreenAnalysisResultSchema, type ScreenAnalysisResult } from './screen-evidence-result';
import { ScreenCaptureReasonSchema } from './screen-evidence-states';

export const ScreenAiBrowserTriggerProofSchemaVersion = 1;

export const ScreenAiBrowserTriggerSurfaceSchema = withParser(
  Schema.Literal('managed-browser-url', 'browser-like-video', 'browser-like-social', 'browser-like-cloud-game')
);

export const ScreenAiBrowserTriggerStateSchema = withParser(Schema.Literal('ready', 'manual-required', 'unavailable'));

export const ScreenAiBrowserTriggerMobileParityStateSchema = withParser(
  Schema.Literal('not-in-scope', 'scaffold-only')
);

export const ScreenAiBrowserTriggerLocalAiExpectedStateSchema = withParser(Schema.Literal('ready', 'partial'));

const ScreenAiBrowserTriggerProofIdSchema = Schema.String.pipe(Schema.minLength(1));
const ScreenAiBrowserTriggerNoClaimFlagsSchema = Schema.Struct({
  rawBrowserStateIncluded: Schema.Literal(false),
  rawScreenFrameStored: Schema.Literal(false),
  remoteAiRequired: Schema.Literal(false),
  finalPolicyClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
  liveExternalAccountClaimed: Schema.Literal(false),
  mobileBrowserParityClaimed: Schema.Literal(false),
  cloudFrameAnalysisClaimed: Schema.Literal(false),
});

const ScreenAiBrowserTriggerProofRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenAiBrowserTriggerProofSchemaVersion),
  rowId: ScreenAiBrowserTriggerProofIdSchema,
  surface: ScreenAiBrowserTriggerSurfaceSchema,
  triggerReason: ScreenCaptureReasonSchema,
  triggerState: ScreenAiBrowserTriggerStateSchema,
  mobileParityState: ScreenAiBrowserTriggerMobileParityStateSchema,
  browserInput: BrowserUrlAiAnalysisInputSchema,
  browserResult: BrowserUrlAiAnalysisResultSchema,
  screenAnalysis: ScreenAnalysisResultSchema,
  localAiContextExpectedState: ScreenAiBrowserTriggerLocalAiExpectedStateSchema,
  sourceEvidenceRefs: Schema.Array(ActivityEvidenceRefSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected browser-trigger proof to cite source evidence')
  ),
  noClaimFlags: ScreenAiBrowserTriggerNoClaimFlagsSchema,
});

type ScreenAiBrowserTriggerProofRowCandidate = Infer<typeof ScreenAiBrowserTriggerProofRowBaseSchema>;

export const ScreenAiBrowserTriggerProofRowSchema = withParser(
  ScreenAiBrowserTriggerProofRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        screenAiBrowserTriggerProofRowIsConsistent(row) ||
        'Expected browser-trigger proof row to link browser, screen, local-AI, and no-claim states'
    )
  )
);

const ScreenAiBrowserTriggerProofBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenAiBrowserTriggerProofSchemaVersion),
  rows: Schema.Array(ScreenAiBrowserTriggerProofRowSchema).pipe(
    Schema.filter((rows) => rows.length === 4 || 'Expected the browser-trigger proof matrix to contain four rows')
  ),
});

export const ScreenAiBrowserTriggerProofSchema = withParser(
  ScreenAiBrowserTriggerProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        screenAiBrowserTriggerProofMatrixIsComplete(proof.rows) ||
        'Expected managed URL, video, social, and cloud-game browser trigger proof rows'
    )
  )
);

export type ScreenAiBrowserTriggerSurface = Infer<typeof ScreenAiBrowserTriggerSurfaceSchema>;
export type ScreenAiBrowserTriggerState = Infer<typeof ScreenAiBrowserTriggerStateSchema>;
export type ScreenAiBrowserTriggerMobileParityState = Infer<typeof ScreenAiBrowserTriggerMobileParityStateSchema>;
export type ScreenAiBrowserTriggerLocalAiExpectedState = Infer<typeof ScreenAiBrowserTriggerLocalAiExpectedStateSchema>;
export type ScreenAiBrowserTriggerProofRow = Infer<typeof ScreenAiBrowserTriggerProofRowSchema>;
export type ScreenAiBrowserTriggerProof = Infer<typeof ScreenAiBrowserTriggerProofSchema>;

const ObservedAt = '2026-06-04T16:47:00.000Z';
const AnalyzedAt = '2026-06-04T16:47:04.000Z';
const ExpiresAt = '2026-06-04T17:02:04.000Z';
const ChildProfileRef = 'child-browser-trigger';
const DeviceRef = 'windows-child-device';
const PolicyVersionRef = 'policy-browser-trigger-v1';
const PromptVersion = 'screen-ai-browser-trigger-prompt-v1';

const NoClaimFlags = {
  rawBrowserStateIncluded: false,
  rawScreenFrameStored: false,
  remoteAiRequired: false,
  finalPolicyClaimed: false,
  enforcementClaimed: false,
  liveExternalAccountClaimed: false,
  mobileBrowserParityClaimed: false,
  cloudFrameAnalysisClaimed: false,
} as const;

const EmptyPlatformIds = {
  videoId: null,
  channelId: null,
  playlistId: null,
  postId: null,
  query: null,
};

export const screenAiBrowserTriggerProofRows: ScreenAiBrowserTriggerProofRow[] = [
  row({
    rowId: 'screen-ai-browser-trigger-managed-url',
    surface: 'managed-browser-url',
    triggerReason: 'managedBrowserUrlChange',
    triggerState: 'ready',
    mobileParityState: 'not-in-scope',
    contentKind: 'article',
    platform: 'generic-web',
    normalizedUrl: 'https://learn.example.invalid/module',
    normalizedDomain: 'learn.example.invalid',
    requestedTask: 'url-safety',
    contentCategory: 'educational',
    videoKind: 'non-video',
    confidence: 'medium',
    degradedState: 'none',
    uncertaintyReasons: [],
    screenCategory: 'school',
    screenConfidence: 0.82,
    screenCapabilityStatus: 'ready',
    providerKind: 'localVision',
    imageDeletionState: 'deleted',
    custodyState: 'child-device-journal',
    policyEligible: true,
    localAiContextExpectedState: 'ready',
    summary: 'Managed browser URL trigger produced a local screen summary for school content.',
  }),
  row({
    rowId: 'screen-ai-browser-trigger-video',
    surface: 'browser-like-video',
    triggerReason: 'managedBrowserUrlChange',
    triggerState: 'ready',
    mobileParityState: 'not-in-scope',
    contentKind: 'video',
    platform: 'youtube',
    platformIds: { ...EmptyPlatformIds, videoId: 'video-ref-001' },
    normalizedUrl: 'https://video.example.invalid/watch/lesson',
    normalizedDomain: 'video.example.invalid',
    requestedTask: 'video-safety',
    contentCategory: 'entertainment',
    contentModifiers: ['metadata-only'],
    videoKind: 'video',
    confidence: 'medium',
    degradedState: 'none',
    uncertaintyReasons: [],
    screenCategory: 'video',
    screenConfidence: 0.74,
    screenCapabilityStatus: 'ready',
    providerKind: 'localVision',
    imageDeletionState: 'deleted',
    custodyState: 'child-device-journal',
    policyEligible: true,
    localAiContextExpectedState: 'ready',
    summary: 'Browser video trigger produced a local screen summary and browser AI candidate.',
  }),
  row({
    rowId: 'screen-ai-browser-trigger-social-manual',
    surface: 'browser-like-social',
    triggerReason: 'managedBrowserUrlChange',
    triggerState: 'manual-required',
    mobileParityState: 'scaffold-only',
    contentKind: 'social-feed',
    platform: 'facebook',
    normalizedUrl: 'https://social.example.invalid/feed',
    normalizedDomain: 'social.example.invalid',
    requestedTask: 'parent-summary',
    contentCategory: 'social',
    contentModifiers: ['login-required', 'metadata-only'],
    videoKind: 'non-video',
    confidence: 'low',
    degradedState: 'manual-required',
    uncertaintyReasons: ['hidden-load-unavailable', 'low-confidence'],
    screenCategory: 'chat',
    screenConfidence: 0.58,
    screenCapabilityStatus: 'ready',
    providerKind: 'localVision',
    imageDeletionState: 'deleted',
    custodyState: 'child-device-journal',
    policyEligible: false,
    localAiContextExpectedState: 'partial',
    summary: 'Social feed trigger is visible to screen/local-AI contracts but account/feed proof is manual-required.',
  }),
  row({
    rowId: 'screen-ai-browser-trigger-cloud-game-unavailable',
    surface: 'browser-like-cloud-game',
    triggerReason: 'browserGameDetected',
    triggerState: 'unavailable',
    mobileParityState: 'scaffold-only',
    contentKind: 'cloud-gaming',
    platform: 'generic-web',
    normalizedUrl: 'https://cloudplay.example.invalid/session',
    normalizedDomain: 'cloudplay.example.invalid',
    requestedTask: 'policy-decision-support',
    contentCategory: 'gaming',
    contentModifiers: ['metadata-only'],
    videoKind: 'unknown',
    confidence: 'unknown',
    degradedState: 'unavailable',
    uncertaintyReasons: ['hidden-load-unavailable', 'model-unavailable'],
    screenCategory: null,
    screenConfidence: 0.1,
    screenCapabilityStatus: 'protectedSurface',
    providerKind: 'unavailable',
    imageDeletionState: 'unavailableNoImage',
    custodyState: 'unavailable',
    policyEligible: false,
    localAiContextExpectedState: 'partial',
    summary: 'Cloud-game browser trigger stays unavailable for cloud-frame analysis and mobile parity.',
  }),
];

export const screenAiBrowserTriggerProof: ScreenAiBrowserTriggerProof = ScreenAiBrowserTriggerProofSchema.parse({
  schemaVersion: ScreenAiBrowserTriggerProofSchemaVersion,
  rows: screenAiBrowserTriggerProofRows,
});

export function screenAiBrowserTriggerProofSummary(
  rows: readonly ScreenAiBrowserTriggerProofRow[] = screenAiBrowserTriggerProofRows
) {
  return {
    totalRows: rows.length,
    readyRows: rows.filter((proofRow) => proofRow.triggerState === 'ready').length,
    manualRequiredRows: rows.filter((proofRow) => proofRow.triggerState === 'manual-required').length,
    unavailableRows: rows.filter((proofRow) => proofRow.triggerState === 'unavailable').length,
    localAiReadyRows: rows.filter((proofRow) => proofRow.localAiContextExpectedState === 'ready').length,
    localAiPartialRows: rows.filter((proofRow) => proofRow.localAiContextExpectedState === 'partial').length,
    productClaimed: false,
    enforcementClaimed: false,
    remoteAiRequired: false,
  };
}

export const decodeScreenAiBrowserTriggerProofRow = Schema.decodeUnknownSync(ScreenAiBrowserTriggerProofRowSchema);
export const decodeScreenAiBrowserTriggerProof = Schema.decodeUnknownSync(ScreenAiBrowserTriggerProofSchema);

interface RowInput {
  rowId: string;
  surface: ScreenAiBrowserTriggerSurface;
  triggerReason: Infer<typeof ScreenCaptureReasonSchema>;
  triggerState: ScreenAiBrowserTriggerState;
  mobileParityState: ScreenAiBrowserTriggerMobileParityState;
  contentKind: BrowserUrlAiAnalysisResult['contentKind'];
  platform: BrowserUrlAiAnalysisInput['platform'];
  platformIds?: BrowserUrlAiAnalysisInput['platformIds'];
  normalizedUrl: string;
  normalizedDomain: string;
  requestedTask: BrowserUrlAiAnalysisInput['requestedTask'];
  contentCategory: BrowserUrlAiAnalysisResult['contentCategory'];
  contentModifiers?: BrowserUrlAiAnalysisResult['contentModifiers'];
  videoKind: BrowserUrlAiAnalysisResult['videoKind'];
  confidence: BrowserUrlAiAnalysisResult['confidence'];
  degradedState: BrowserUrlAiAnalysisResult['degradedState'];
  uncertaintyReasons: BrowserUrlAiAnalysisResult['uncertaintyReasons'];
  screenCategory: ScreenAnalysisResult['primaryCategory'];
  screenConfidence: number;
  screenCapabilityStatus: ScreenAnalysisResult['capabilityStatus'];
  providerKind: ScreenAnalysisResult['providerKind'];
  imageDeletionState: ScreenAnalysisResult['imageDeletionState'];
  custodyState: ScreenAnalysisResult['custodyState'];
  policyEligible: boolean;
  localAiContextExpectedState: ScreenAiBrowserTriggerLocalAiExpectedState;
  summary: string;
}

function row(input: RowInput): ScreenAiBrowserTriggerProofRow {
  const browserEvidenceId = `${input.rowId}-browser-evidence`;
  const screenEvidenceId = `${input.rowId}-screen-evidence`;
  const sourceEvidenceRefs = [
    evidenceRef(browserEvidenceId, 'journal-entry'),
    evidenceRef(screenEvidenceId, 'local-db-row'),
  ];
  const browserInput = browserAiInput(input, browserEvidenceId, screenEvidenceId);
  const browserResult = browserAiResult(input, browserEvidenceId);
  const screenAnalysis = screenResult(input, sourceEvidenceRefs);

  return ScreenAiBrowserTriggerProofRowSchema.parse({
    schemaVersion: ScreenAiBrowserTriggerProofSchemaVersion,
    rowId: input.rowId,
    surface: input.surface,
    triggerReason: input.triggerReason,
    triggerState: input.triggerState,
    mobileParityState: input.mobileParityState,
    browserInput,
    browserResult,
    screenAnalysis,
    localAiContextExpectedState: input.localAiContextExpectedState,
    sourceEvidenceRefs,
    noClaimFlags: NoClaimFlags,
  });
}

function browserAiInput(
  input: RowInput,
  browserEvidenceId: string,
  screenEvidenceId: string
): BrowserUrlAiAnalysisInput {
  return BrowserUrlAiAnalysisInputSchema.parse({
    schemaVersion: BrowserAiAnalysisSchemaVersion,
    requestId: `${input.rowId}-request`,
    requestedAt: ObservedAt,
    childProfileRef: ChildProfileRef,
    deviceId: DeviceRef,
    policyVersionRef: PolicyVersionRef,
    sourceEvidenceIds: [browserEvidenceId],
    urlShapeClassificationId: `${input.rowId}-url-shape`,
    metadataEvidenceIds: [],
    memoryHitIds: [],
    graphRefs: [],
    parentRuleRefs: [`${input.rowId}-parent-rule`],
    scheduleContextRefs: [],
    normalizedUrl: input.normalizedUrl,
    normalizedDomain: input.normalizedDomain,
    platform: input.platform,
    platformIds: input.platformIds ?? EmptyPlatformIds,
    title: `${input.rowId} title ref`,
    description: input.summary,
    transcriptRefs: [],
    thumbnailRefs: [],
    screenEvidenceRefs: [screenEvidenceId],
    requestedTask: input.requestedTask,
    modelRuntimePreference: input.triggerState === 'ready' ? 'local-only' : 'manual-required',
    promptTemplate: {
      promptTemplateId: `${input.rowId}-prompt`,
      promptTemplateVersion: PromptVersion,
      requestedTask: input.requestedTask,
      allowedInputFieldRefs: ['sourceEvidenceIds', 'screenEvidenceRefs', 'parentRuleRefs'],
      rawPromptTextIncluded: false,
      capturesRawPageBody: false,
      capturesTranscriptText: false,
    },
    custodyLabel: input.triggerState === 'unavailable' ? 'unavailable' : 'child-device-local',
    rawBrowserStateIncluded: false,
    devToolsPayloadIncluded: false,
    sqlitePathIncluded: false,
    journalPathIncluded: false,
    osStateIncluded: false,
  });
}

function browserAiResult(input: RowInput, browserEvidenceId: string): BrowserUrlAiAnalysisResult {
  return BrowserUrlAiAnalysisResultSchema.parse({
    schemaVersion: BrowserAiAnalysisSchemaVersion,
    analysisId: `${input.rowId}-analysis`,
    requestId: `${input.rowId}-request`,
    analyzedAt: AnalyzedAt,
    expiresAt: ExpiresAt,
    sourceEvidenceIds: [browserEvidenceId],
    metadataEvidenceIds: [],
    memoryHitIds: [],
    graphRefs: [],
    parentRuleRefs: [`${input.rowId}-parent-rule`],
    contentKind: input.contentKind,
    videoKind: input.videoKind,
    contentCategory: input.contentCategory,
    contentModifiers: input.contentModifiers ?? ['metadata-only'],
    benefitSignals: ['neutral'],
    riskSignals: input.contentCategory === 'gaming' ? ['unknown-risk'] : ['privacy-risk'],
    recommendedPolicyInput: input.triggerState === 'ready' ? 'warn-candidate' : 'manual-review-candidate',
    confidence: input.confidence,
    uncertaintyReasons: input.uncertaintyReasons,
    parentSummary: input.summary,
    childSafeSummary: null,
    modelRuntimeRef: `${input.rowId}-model-runtime`,
    promptTemplate: {
      promptTemplateId: `${input.rowId}-prompt`,
      promptTemplateVersion: PromptVersion,
      requestedTask: input.requestedTask,
      allowedInputFieldRefs: ['sourceEvidenceIds', 'screenEvidenceRefs', 'parentRuleRefs'],
      rawPromptTextIncluded: false,
      capturesRawPageBody: false,
      capturesTranscriptText: false,
    },
    degradedState: input.degradedState,
    finalPolicyActionClaimed: false,
    enforcementActionClaimed: false,
    rawContentStored: false,
  });
}

function screenResult(
  input: RowInput,
  sourceEvidenceRefs: ScreenAnalysisResult['sourceEvidenceRefs']
): ScreenAnalysisResult {
  const primaryCategory = input.screenCategory;
  const candidateRefs =
    primaryCategory === null
      ? []
      : [{ category: primaryCategory, confidence: input.screenConfidence, evidenceRefs: sourceEvidenceRefs }];

  return ScreenAnalysisResultSchema.parse({
    schemaVersion: 1,
    screenAnalysisResultId: `${input.rowId}-screen-result`,
    queueJobId: `${input.rowId}-queue-job`,
    analyzedAt: AnalyzedAt,
    modelRuntimeRef: `${input.rowId}-screen-runtime`,
    modelId: `${input.rowId}-screen-model`,
    providerKind: input.providerKind,
    promptOrTemplateVersion: PromptVersion,
    captureReason: input.triggerReason,
    captureScope: input.triggerState === 'unavailable' ? 'unsupported' : 'managedBrowserWindow',
    capabilityStatus: input.screenCapabilityStatus,
    summary: input.summary,
    visibleCategoryCandidates: candidateRefs,
    primaryCategory,
    riskSignals:
      primaryCategory === null
        ? []
        : [{ signal: 'unknown', confidence: input.screenConfidence, evidenceRefs: sourceEvidenceRefs }],
    ocrTextSnippets: [],
    redactionNotes: [],
    confidence: input.screenConfidence,
    uncertaintyReason: input.triggerState === 'unavailable' ? 'protectedSurface' : null,
    sourceEvidenceRefs,
    imageDigest: `${input.rowId}-image-digest`,
    rawImageRetained: false,
    imageDeletionState: input.imageDeletionState,
    custodyState: input.custodyState,
    policyEligible: input.policyEligible,
  });
}

function evidenceRef(evidenceId: string, kind: Infer<typeof ActivityEvidenceRefSchema>['kind']) {
  return ActivityEvidenceRefSchema.parse({
    evidenceId,
    kind,
    digest: `${evidenceId}-digest`,
    uri: null,
  });
}

function screenAiBrowserTriggerProofRowIsConsistent(row: ScreenAiBrowserTriggerProofRowCandidate): boolean {
  return (
    browserResultMatchesInput(row) &&
    screenEvidenceIsLinked(row) &&
    rowStateMatchesContracts(row) &&
    noClaimFlagsAreClear(row)
  );
}

function browserResultMatchesInput(row: ScreenAiBrowserTriggerProofRowCandidate): boolean {
  return (
    row.browserResult.requestId === row.browserInput.requestId &&
    row.browserInput.sourceEvidenceIds.every((evidenceId) => row.browserResult.sourceEvidenceIds.includes(evidenceId))
  );
}

function screenEvidenceIsLinked(row: ScreenAiBrowserTriggerProofRowCandidate): boolean {
  const screenEvidenceIds = new Set(row.screenAnalysis.sourceEvidenceRefs.map((reference) => reference.evidenceId));
  return row.browserInput.screenEvidenceRefs.every((evidenceId) => screenEvidenceIds.has(evidenceId));
}

function rowStateMatchesContracts(row: ScreenAiBrowserTriggerProofRowCandidate): boolean {
  if (row.triggerState === 'ready') {
    return (
      row.browserResult.degradedState === 'none' &&
      row.screenAnalysis.capabilityStatus === 'ready' &&
      row.screenAnalysis.policyEligible &&
      row.localAiContextExpectedState === 'ready'
    );
  }
  if (row.triggerState === 'manual-required') {
    return (
      row.browserResult.degradedState === 'manual-required' &&
      !row.screenAnalysis.policyEligible &&
      row.localAiContextExpectedState === 'partial'
    );
  }
  return (
    row.browserResult.degradedState === 'unavailable' &&
    row.screenAnalysis.capabilityStatus !== 'ready' &&
    !row.screenAnalysis.policyEligible &&
    row.localAiContextExpectedState === 'partial'
  );
}

function noClaimFlagsAreClear(row: ScreenAiBrowserTriggerProofRowCandidate): boolean {
  return Object.values(row.noClaimFlags).every((value) => value === false);
}

function screenAiBrowserTriggerProofMatrixIsComplete(rows: readonly ScreenAiBrowserTriggerProofRow[]): boolean {
  const surfaces = new Set(rows.map((row) => row.surface));
  return (
    surfaces.has('managed-browser-url') &&
    surfaces.has('browser-like-video') &&
    surfaces.has('browser-like-social') &&
    surfaces.has('browser-like-cloud-game') &&
    rows.some((row) => row.triggerState === 'manual-required') &&
    rows.some((row) => row.triggerState === 'unavailable')
  );
}
