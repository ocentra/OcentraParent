import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  BrowserSocialAiAnalysisInputSchema,
  BrowserSocialAiAnalysisResultSchema,
} from '../../packages/browser-domain/dist/browser-social-ai-analysis-schemas.js';
import { buildBrowserSocialAiAnalysisResult } from '@ocentra-parent/schema-domain/browser-social-ai-analysis-result-builder';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const social09ProofPath = join(repoRoot, 'test-results/social-video-metadata-live-proof/proof.json');
const outputDirectory = join(repoRoot, 'output/browser-plan-proof/social-10-social-ai-analysis-contracts');
const outputProofPath = join(outputDirectory, '11-live-evidence-ai-boundary-proof.json');
const testResultPath = join(repoRoot, 'test-results/social-ai-analysis-live-evidence-proof/proof.json');
const observedAt = new Date().toISOString();
const expiresAt = new Date(Date.parse(observedAt) + 60 * 60 * 1000).toISOString();

const sourceFiles = [
  'packages/browser-domain/src/browser-social-ai-analysis-values.ts',
  'packages/browser-domain/src/browser-social-ai-analysis-schemas.ts',
  'packages/schema-domain/src/browser-social-ai-analysis-result-builder.ts',
];
const builtFiles = [
  'packages/browser-domain/dist/browser-social-ai-analysis-values.js',
  'packages/browser-domain/dist/browser-social-ai-analysis-schemas.js',
  'packages/schema-domain/dist/browser-social-ai-analysis-result-builder.js',
];

assertBuiltContractsAreFresh();
if (!existsSync(social09ProofPath)) {
  throw new Error(`Missing SOCIAL-09 live metadata proof: ${relativePath(social09ProofPath)}`);
}

const social09Proof = JSON.parse(readFileSync(social09ProofPath, 'utf8'));
const sourceCaptures = social09Proof.captures.filter(
  (capture) => capture.contractMetadataCreated && capture.metadataSummary !== null
);
if (sourceCaptures.length < 3) {
  throw new Error(`Expected at least 3 SOCIAL-09 metadata captures, received ${sourceCaptures.length}`);
}

const inputRows = sourceCaptures.map((capture) => socialAiInputForCapture(capture));
const inputParseChecks = inputRows.map((row) => ({
  targetId: row.targetId,
  accepted: BrowserSocialAiAnalysisInputSchema.safeParse(row.input).success,
}));
if (!inputParseChecks.every((check) => check.accepted)) {
  throw new Error('Expected every SOCIAL-10 live-evidence AI input to parse');
}

const resultRows = inputRows.map((row) => {
  const input = BrowserSocialAiAnalysisInputSchema.parse(row.input);
  const result = buildBrowserSocialAiAnalysisResult({
    analysisId: `${row.targetId}-social-ai-analysis`,
    analyzedAt: observedAt,
    expiresAt,
    input,
    classifications: classificationsFor(row),
    riskSignalRefs: [`${row.targetId}-risk-signal-ref-model-unavailable`],
    benefitSignalRefs: [`${row.targetId}-benefit-signal-ref-model-unavailable`],
    recommendedPolicyInput: 'manual-review-candidate',
    confidence: 'unknown',
    uncertaintyReasons: ['model-unavailable'],
    parentSummaryRef: `${row.targetId}-parent-summary-ref-model-unavailable`,
    childSafeSummaryRef: null,
    modelRuntimeRef: null,
    degradedState: 'unavailable',
  });
  return {
    targetId: row.targetId,
    input,
    result,
  };
});

const resultParseChecks = resultRows.map((row) => ({
  targetId: row.targetId,
  accepted: BrowserSocialAiAnalysisResultSchema.safeParse(row.result).success,
}));
if (!resultParseChecks.every((check) => check.accepted)) {
  throw new Error('Expected every SOCIAL-10 degraded AI result to parse');
}

const negativeChecks = buildNegativeChecks(inputRows[0].input, resultRows[0].result);
if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected all SOCIAL-10 negative checks to reject dishonest AI boundary claims');
}

const proof = {
  schemaVersion: 1,
  proofId: 'social-ai-analysis-live-evidence-proof',
  generatedAt: observedAt,
  branch: git(['branch', '--show-current']),
  commit: git(['rev-parse', 'HEAD']),
  baseCommit: git(['rev-parse', 'origin/main']),
  sourceProof: relativePath(social09ProofPath),
  liveEvidenceBoundary: {
    sourceProofUsesRealPublicSocialVideoSurfaces: social09Proof.liveCaptureSummary.realPublicSocialVideoSurfacesUsed,
    sourceMetadataCaptureCount: sourceCaptures.length,
    sourceScreenshotsPersisted: social09Proof.liveCaptureSummary.screenshotsPersisted,
    aiModelExecuted: false,
    modelRuntimeRefPresent: false,
    degradedState: 'unavailable',
    rawPromptTextPersisted: false,
    rawPageBodyPersisted: false,
    rawFeedContentPersisted: false,
    rawMessageContentPersisted: false,
    transcriptTextPersisted: false,
    screenshotStoredInAiInputOrResult: false,
    finalPolicyActionClaimed: false,
    enforcementActionClaimed: false,
  },
  inputs: inputRows.map((row) => ({
    targetId: row.targetId,
    requestedTask: row.input.requestedTask,
    platform: row.input.platform,
    routeKind: row.input.routeKind,
    sourceEvidenceIds: row.input.sourceEvidenceIds,
    metadataEvidenceIds: row.input.metadataEvidenceIds,
    feedRouteClassificationIds: row.input.feedRouteClassificationIds,
    modelRuntimePreference: row.input.modelRuntimePreference,
    custodyLabel: row.input.custodyLabel,
  })),
  results: resultRows.map((row) => ({
    targetId: row.targetId,
    requestedTask: row.result.requestedTask,
    platform: row.result.platform,
    routeKind: row.result.routeKind,
    classifications: row.result.classifications,
    recommendedPolicyInput: row.result.recommendedPolicyInput,
    confidence: row.result.confidence,
    uncertaintyReasons: row.result.uncertaintyReasons,
    degradedState: row.result.degradedState,
    modelRuntimeRef: row.result.modelRuntimeRef,
    finalPolicyActionClaimed: row.result.finalPolicyActionClaimed,
    enforcementActionClaimed: row.result.enforcementActionClaimed,
    rawModelTextStored: row.result.rawModelTextStored,
    rawPageBodyStored: row.result.rawPageBodyStored,
    rawFeedContentStored: row.result.rawFeedContentStored,
    screenshotStored: row.result.screenshotStored,
  })),
  inputParseChecks,
  resultParseChecks,
  negativeChecks,
  noClaimChecks: {
    aiModelExecution: false,
    providerRuntimeSelection: false,
    finalPolicyDecision: false,
    enforcement: false,
    rawContentStorage: false,
    nativeAppControl: false,
    connectorAuthorization: false,
    uiDelivery: false,
  },
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('social-ai-analysis-live-evidence-proof-ok=true');
console.log(`proof=${relativePath(testResultPath)}`);
console.log(`outputProof=${relativePath(outputProofPath)}`);
console.log(`inputCount=${inputRows.length}`);
console.log(`resultStates=${resultRows.map((row) => row.result.degradedState).join(',')}`);

function socialAiInputForCapture(capture) {
  const metadata = capture.metadataSummary;
  const targetId = capture.targetId;
  const requestedTask = metadata.routeKind === 'feed' ? 'feed-risk-classification' : 'video-safety';
  return {
    targetId,
    input: {
      schemaVersion: 1,
      requestId: `${targetId}-social-ai-request`,
      requestedAt: observedAt,
      childProfileRef: 'child-profile-live-social-proof',
      deviceId: 'child-device-managed-browser-proof',
      sourceEvidenceIds: [`${targetId}-live-ai-source-ref`, metadata.metadataEvidenceId],
      socialRouteEvidenceId: metadata.socialRouteEvidenceId,
      urlShapeClassificationId: `${targetId}-live-url-shape`,
      platform: metadata.platform,
      routeKind: metadata.routeKind,
      feedRouteClassificationIds: metadata.routeKind === 'feed' ? [`${targetId}-feed-route-classification`] : [],
      metadataEvidenceIds: [metadata.metadataEvidenceId],
      accountFlowEvidenceIds: [],
      accountIdentityRefs: [],
      screenSummaryEvidenceRefs: [],
      parentRuleRefs: ['parent-rule-social-ai-review-required'],
      memoryHitIds: [],
      requestedTask,
      modelRuntimePreference: 'manual-required',
      promptTemplate: socialPromptTemplate(requestedTask),
      custodyLabel: 'child-device-local',
      rawBrowserStateIncluded: false,
      rawPageBodyIncluded: false,
      rawMessageContentIncluded: false,
      rawFeedContentIncluded: false,
      transcriptTextIncluded: false,
      screenshotIncluded: false,
      nativeAppStateIncluded: false,
      platformConnectorIncluded: false,
    },
  };
}

function socialPromptTemplate(task) {
  return {
    promptTemplateId: `social-ai-prompt-template-${task}`,
    promptTemplateVersion: `social-ai-prompt-template-${task}-v1`,
    requestedTask: task,
    allowedInputFieldRefs: ['social-route-evidence-ref', 'social-metadata-evidence-ref', 'parent-rule-ref'],
    rawPromptTextIncluded: false,
    capturesRawPageBody: false,
    capturesTranscriptText: false,
    capturesMessageContent: false,
    capturesFeedContent: false,
    capturesScreenshot: false,
  };
}

function classificationsFor(row) {
  if (row.input.routeKind === 'feed') {
    return ['feed-browsing', 'unknown'];
  }
  if (row.input.platform === 'youtube-shorts') {
    return ['short-video-browsing', 'unknown'];
  }
  return ['video-watch', 'unknown'];
}

function buildNegativeChecks(validInput, validResult) {
  const inputInvalidRows = [
    ['raw-browser-state-included', { ...validInput, rawBrowserStateIncluded: true }],
    ['raw-page-body-included', { ...validInput, rawPageBodyIncluded: true }],
    ['raw-message-content-included', { ...validInput, rawMessageContentIncluded: true }],
    ['raw-feed-content-included', { ...validInput, rawFeedContentIncluded: true }],
    ['transcript-text-included', { ...validInput, transcriptTextIncluded: true }],
    ['screenshot-included', { ...validInput, screenshotIncluded: true }],
    ['connector-included', { ...validInput, platformConnectorIncluded: true }],
    [
      'fake-video-task-without-metadata',
      {
        ...validInput,
        requestedTask: 'video-safety',
        promptTemplate: socialPromptTemplate('video-safety'),
        metadataEvidenceIds: [],
      },
    ],
  ];
  const resultInvalidRows = [
    ['final-policy-claim', { ...validResult, finalPolicyActionClaimed: true }],
    ['enforcement-claim', { ...validResult, enforcementActionClaimed: true }],
    ['raw-model-text-stored', { ...validResult, rawModelTextStored: true }],
    ['raw-page-body-stored', { ...validResult, rawPageBodyStored: true }],
    ['raw-feed-content-stored', { ...validResult, rawFeedContentStored: true }],
    ['screenshot-stored', { ...validResult, screenshotStored: true }],
    ['native-app-control-claim', { ...validResult, nativeAppControlClaimed: true }],
    ['connector-claim', { ...validResult, platformConnectorClaimed: true }],
    ['fake-nondegraded-model-success', { ...validResult, degradedState: 'none', modelRuntimeRef: null }],
  ];

  return [
    ...inputInvalidRows.map(([name, input]) => ({
      name,
      rejected: !BrowserSocialAiAnalysisInputSchema.safeParse(input).success,
    })),
    ...resultInvalidRows.map(([name, result]) => ({
      name,
      rejected: !BrowserSocialAiAnalysisResultSchema.safeParse(result).success,
    })),
  ];
}

function assertBuiltContractsAreFresh() {
  const newestSourceMtime = Math.max(...sourceFiles.map((file) => statSync(join(repoRoot, file)).mtimeMs));
  for (const builtFile of builtFiles) {
    const builtPath = join(repoRoot, builtFile);
    const builtMtime = statSync(builtPath).mtimeMs;
    if (builtMtime < newestSourceMtime) {
      throw new Error(`Build output is stale: ${builtFile}. Run cmd /c npm run build:contracts first.`);
    }
  }
}

function writeJson(path, value) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
