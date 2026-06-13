import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { BrowserSocialAiAnalysisResultSchema } from '../../packages/browser-domain/dist/browser-social-ai-analysis-schemas.js';
import {
  BrowserSocialBenefitSignalSchema,
  BrowserSocialRiskBenefitSignalSetSchema,
  BrowserSocialRiskSignalSchema,
  buildBrowserSocialRiskBenefitSignalSet,
} from '../../packages/browser-domain/dist/browser-social-riskbenefit-signals.js';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const social10ProofPath = join(repoRoot, 'test-results/social-ai-analysis-live-evidence-proof/proof.json');
const outputDirectory = join(repoRoot, 'output/browser-plan-proof/social-11-social-risk-benefit-signal-model');
const outputProofPath = join(outputDirectory, '11-live-evidence-risk-benefit-proof.json');
const testResultPath = join(repoRoot, 'test-results/social-risk-benefit-live-evidence-proof/proof.json');
const observedAt = new Date().toISOString();

const sourceFiles = [
  'packages/browser-domain/src/browser-social-riskbenefit-values.ts',
  'packages/browser-domain/src/browser-social-riskbenefit-signals.ts',
];
const builtFiles = [
  'packages/browser-domain/dist/browser-social-riskbenefit-values.js',
  'packages/browser-domain/dist/browser-social-riskbenefit-signals.js',
];

assertBuiltContractsAreFresh();
if (!existsSync(social10ProofPath)) {
  throw new Error(`Missing SOCIAL-10 live-evidence AI proof: ${relativePath(social10ProofPath)}`);
}

const social10Proof = JSON.parse(readFileSync(social10ProofPath, 'utf8'));
if (social10Proof.results.length < 3) {
  throw new Error(`Expected at least 3 SOCIAL-10 result rows, received ${social10Proof.results.length}`);
}

const analysisRows = social10Proof.results.map((result) => analysisResultFor(result, social10Proof.inputs));
const analysisParseChecks = analysisRows.map((row) => ({
  targetId: row.targetId,
  accepted: BrowserSocialAiAnalysisResultSchema.safeParse(row.analysis).success,
}));
if (!analysisParseChecks.every((check) => check.accepted)) {
  throw new Error('Expected reconstructed SOCIAL-10 degraded analysis refs to parse');
}

const signalRows = analysisRows.map((row) => {
  const riskSignal = unavailableRiskSignal(row.targetId, row.analysis.analysisId);
  const benefitSignal = unavailableBenefitSignal(row.targetId, row.analysis.analysisId);
  const signalSet = buildBrowserSocialRiskBenefitSignalSet({
    signalSetId: `${row.targetId}-risk-benefit-signal-set`,
    modeledAt: observedAt,
    socialAiAnalysisResult: row.analysis,
    signalSourceKind: 'social-ai-analysis',
    riskSignals: [riskSignal],
    benefitSignals: [benefitSignal],
  });
  return {
    targetId: row.targetId,
    riskSignal,
    benefitSignal,
    signalSet,
  };
});

const signalParseChecks = signalRows.map((row) => ({
  targetId: row.targetId,
  riskAccepted: BrowserSocialRiskSignalSchema.safeParse(row.riskSignal).success,
  benefitAccepted: BrowserSocialBenefitSignalSchema.safeParse(row.benefitSignal).success,
  signalSetAccepted: BrowserSocialRiskBenefitSignalSetSchema.safeParse(row.signalSet).success,
}));
if (!signalParseChecks.every((check) => check.riskAccepted && check.benefitAccepted && check.signalSetAccepted)) {
  throw new Error('Expected every SOCIAL-11 unavailable signal row and signal set to parse');
}

const negativeChecks = buildNegativeChecks(signalRows[0]);
if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected all SOCIAL-11 negative checks to reject dishonest signal claims');
}

const proof = {
  schemaVersion: 1,
  proofId: 'social-risk-benefit-live-evidence-proof',
  generatedAt: observedAt,
  branch: git(['branch', '--show-current']),
  commit: git(['rev-parse', 'HEAD']),
  baseCommit: git(['rev-parse', 'origin/main']),
  sourceProof: relativePath(social10ProofPath),
  liveEvidenceBoundary: {
    sourceAiProofUsesLiveSocial09Refs: true,
    sourceAiModelExecuted: social10Proof.liveEvidenceBoundary.aiModelExecuted,
    sourceAiResultCount: social10Proof.results.length,
    riskBenefitCandidateClassified: false,
    unavailableSignalRowsOnly: true,
    rawModelTextUsed: false,
    rawMessageContentUsed: false,
    rawFeedContentUsed: false,
    rawPageBodyUsed: false,
    accountIdentityVerifiedClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  },
  signalSets: signalRows.map((row) => ({
    targetId: row.targetId,
    signalSetId: row.signalSet.signalSetId,
    socialAiAnalysisId: row.signalSet.socialAiAnalysisId,
    platform: row.signalSet.platform,
    routeKind: row.signalSet.routeKind,
    signalSourceKind: row.signalSet.signalSourceKind,
    riskSignals: row.signalSet.riskSignals.map(summaryForSignal),
    benefitSignals: row.signalSet.benefitSignals.map(summaryForSignal),
    recommendedPolicyInput: row.signalSet.recommendedPolicyInput,
    confidence: row.signalSet.confidence,
    degradedState: row.signalSet.degradedState,
    uncertaintyReasons: row.signalSet.uncertaintyReasons,
    finalPolicyDecisionClaimed: row.signalSet.finalPolicyDecisionClaimed,
    enforcementClaimed: row.signalSet.enforcementClaimed,
    rawModelTextUsed: row.signalSet.rawModelTextUsed,
    rawMessageContentUsed: row.signalSet.rawMessageContentUsed,
    rawFeedContentUsed: row.signalSet.rawFeedContentUsed,
  })),
  analysisParseChecks,
  signalParseChecks,
  negativeChecks,
  noClaimChecks: {
    contentRiskClassification: false,
    finalPolicyDecision: false,
    enforcement: false,
    rawContentUse: false,
    accountIdentityVerification: false,
    nativeAppControl: false,
    connectorAuthorization: false,
    uiDelivery: false,
  },
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('social-risk-benefit-live-evidence-proof-ok=true');
console.log(`proof=${relativePath(testResultPath)}`);
console.log(`outputProof=${relativePath(outputProofPath)}`);
console.log(`signalSetCount=${signalRows.length}`);
console.log(`signalStates=${signalRows.map((row) => row.signalSet.degradedState).join(',')}`);

function analysisResultFor(result, inputs) {
  const input = inputs.find((candidate) => candidate.targetId === result.targetId);
  if (!input) {
    throw new Error(`Missing SOCIAL-10 input row for ${result.targetId}`);
  }
  return {
    targetId: result.targetId,
    analysis: {
      schemaVersion: 1,
      analysisId: `${result.targetId}-social-ai-analysis`,
      requestId: `${result.targetId}-social-ai-request`,
      analyzedAt: observedAt,
      expiresAt: new Date(Date.parse(observedAt) + 60 * 60 * 1000).toISOString(),
      sourceEvidenceIds: input.sourceEvidenceIds,
      socialRouteEvidenceId: input.sourceEvidenceIds[0],
      platform: result.platform,
      routeKind: result.routeKind,
      requestedTask: result.requestedTask,
      classifications: result.classifications,
      riskSignalRefs: [`${result.targetId}-risk-signal-ref-model-unavailable`],
      benefitSignalRefs: [`${result.targetId}-benefit-signal-ref-model-unavailable`],
      recommendedPolicyInput: result.recommendedPolicyInput,
      confidence: result.confidence,
      uncertaintyReasons: result.uncertaintyReasons,
      parentSummaryRef: `${result.targetId}-parent-summary-ref-model-unavailable`,
      childSafeSummaryRef: null,
      modelRuntimeRef: result.modelRuntimeRef,
      promptTemplate: socialPromptTemplate(result.requestedTask),
      degradedState: result.degradedState,
      finalPolicyActionClaimed: false,
      enforcementActionClaimed: false,
      rawModelTextStored: false,
      rawPageBodyStored: false,
      transcriptTextStored: false,
      rawMessageContentStored: false,
      rawFeedContentStored: false,
      screenshotStored: false,
      nativeAppControlClaimed: false,
      platformConnectorClaimed: false,
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

function unavailableRiskSignal(targetId, analysisId) {
  return {
    signalId: `${targetId}-risk-signal-unavailable`,
    kind: 'unknown-risk',
    severity: 'unknown',
    state: 'unavailable',
    confidence: 'unknown',
    evidenceRefs: [analysisId],
    rawMessageContentUsed: false,
    rawFeedContentUsed: false,
    rawPageBodyUsed: false,
    accountIdentityVerifiedClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
  };
}

function unavailableBenefitSignal(targetId, analysisId) {
  return {
    signalId: `${targetId}-benefit-signal-unavailable`,
    kind: 'unknown-benefit',
    severity: 'unknown',
    state: 'unavailable',
    confidence: 'unknown',
    evidenceRefs: [analysisId],
    rawMessageContentUsed: false,
    rawFeedContentUsed: false,
    rawPageBodyUsed: false,
    accountIdentityVerifiedClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
  };
}

function summaryForSignal(signal) {
  return {
    signalId: signal.signalId,
    kind: signal.kind,
    severity: signal.severity,
    state: signal.state,
    confidence: signal.confidence,
    evidenceRefs: signal.evidenceRefs,
  };
}

function buildNegativeChecks(validRow) {
  const invalidSignals = [
    ['risk-raw-message', { ...validRow.riskSignal, rawMessageContentUsed: true }],
    ['risk-feed-content', { ...validRow.riskSignal, rawFeedContentUsed: true }],
    ['risk-policy-decision', { ...validRow.riskSignal, policyDecisionClaimed: true }],
    ['risk-enforcement', { ...validRow.riskSignal, enforcementClaimed: true }],
    ['benefit-page-body', { ...validRow.benefitSignal, rawPageBodyUsed: true }],
    ['benefit-identity-claim', { ...validRow.benefitSignal, accountIdentityVerifiedClaimed: true }],
    ['fake-candidate-unknown-risk', { ...validRow.riskSignal, state: 'candidate' }],
  ];
  const invalidSets = [
    ['final-policy-decision', { ...validRow.signalSet, finalPolicyDecisionClaimed: true }],
    ['enforcement-claim', { ...validRow.signalSet, enforcementClaimed: true }],
    ['raw-model-text-used', { ...validRow.signalSet, rawModelTextUsed: true }],
    ['raw-message-content-used', { ...validRow.signalSet, rawMessageContentUsed: true }],
    ['native-app-control-claim', { ...validRow.signalSet, nativeAppControlClaimed: true }],
    ['connector-claim', { ...validRow.signalSet, platformConnectorClaimed: true }],
    ['empty-signal-set', { ...validRow.signalSet, riskSignals: [], benefitSignals: [] }],
  ];
  return [
    ...invalidSignals.map(([name, signal]) => ({
      name,
      rejected:
        !BrowserSocialRiskSignalSchema.safeParse(signal).success ||
        !BrowserSocialBenefitSignalSchema.safeParse(signal).success,
    })),
    ...invalidSets.map(([name, signalSet]) => ({
      name,
      rejected: !BrowserSocialRiskBenefitSignalSetSchema.safeParse(signalSet).success,
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
