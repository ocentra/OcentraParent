import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { spawnSync } from 'node:child_process';

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const outputDir = resolve(repoRoot, 'output', 'ai-plan-proof', 'screen-ai-confidence-policy-proof');
const testResultDir = resolve(repoRoot, 'test-results', 'screen-ai-confidence-policy-proof');
const sourceProofPath = resolve(
  repoRoot,
  'output',
  'screen-ai-pipeline-proof',
  'service-winrt-ocr-policy',
  'proof-summary.json'
);

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    shell: process.platform === 'win32',
    stdio: 'inherit',
  });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

run('npm', ['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

const { buildScreenAiConfidencePolicyProof } = await import(
  pathToFileURL(resolve(repoRoot, 'packages', 'parent-domain', 'dist', 'screen-ai-confidence-policy-proof.js'))
);

const sourceProof = JSON.parse(await readFile(sourceProofPath, 'utf8'));
const sourceRow = sourceProof.sourceAnalysisRow;
const sourcePolicy = sourceProof.policy;
const sourceEvidenceReferences = sourcePolicy.evidenceReferenceIds.map((evidenceReferenceId) => ({
  evidenceReferenceId,
  kind: evidenceReferenceId.includes('journal') ? 'journal-event' : 'activity-event',
  observedAt: sourceProof.generatedAt,
}));

const parentBlockRule = {
  ruleId: sourcePolicy.ruleIds[0],
  target: {
    targetId: `${sourceRow.primaryCategory}-target`,
    targetType: 'category',
    targetValue: sourceRow.primaryCategory,
  },
  action: 'block',
  scheduleId: null,
  priority: 100,
  reasonCode: 'parent-explicit-screen-category-block',
  createdBy: {
    actorId: 'screen-ai-confidence-proof-parent',
    role: 'parent',
  },
  enabled: true,
  effectiveFrom: null,
  effectiveUntil: null,
};

const sourcePolicyDecision = {
  schemaVersion: 'v0.6',
  decisionId: sourcePolicy.decisionId,
  action: sourcePolicy.action,
  reasonCodes: sourcePolicy.reasonCodes,
  evidenceReferences: sourceEvidenceReferences,
  ruleIds: sourcePolicy.ruleIds,
  localAiResultId: sourcePolicy.localAiResultId,
  dryRun: true,
  enforcementHandoffState: sourceProof.policy.enforcementHandoffState,
  expiresAt: null,
};

const localAiResult = {
  schemaVersion: 'v0.6',
  resultId: sourcePolicy.localAiResultId,
  requestId: `${sourceRow.queueJobId}-local-ai-request`,
  action: sourcePolicy.action,
  confidence: sourceRow.confidence,
  unknownState: 'none',
  degradedState: 'none',
  reasonCodes: sourcePolicy.reasonCodes,
  explanationReference: `${sourceRow.rowId}-screen-confidence-explanation`,
  evidenceReferences: sourceEvidenceReferences,
  parentRuleReferences: sourcePolicy.ruleIds,
  memoryReferences: [],
  graphReferences: [],
  modelRuntime: {
    runtimeReferenceId: sourceRow.modelRuntimeRef,
    providerId: `${sourceRow.providerKind}-provider`,
    modelId: sourceRow.modelId,
    modelReference: `${sourceRow.modelId}-local-reference`,
    privacyMode: 'local-only',
    adapterBoundary: 'local-adapter-ready',
    executionState: 'dry-run-ready',
    providerSource: 'os-capability-probe',
    loadState: 'loaded',
    capabilityFlags: ['classification', 'safety-decision'],
    resourceClass: 'cpu',
    degradedState: 'none',
    lastCheckedAt: sourceProof.generatedAt,
    unavailableReason: null,
  },
  promptVersion: sourceRow.promptOrTemplateVersion,
  expiresAt: null,
};

const claimBoundaries = {
  remoteAiUsed: false,
  apiAiUsed: false,
  policyAuthorityClaimed: false,
  enforcementClaimed: false,
  rawEvidenceEmbedded: false,
  modelQualityClaimed: false,
};

const proof = buildScreenAiConfidencePolicyProof({
  schemaVersion: 'v0.6',
  proofId: 'screen-ai-confidence-policy-proof',
  evaluatedAt: sourceProof.generatedAt,
  localAiResult,
  parentRule: parentBlockRule,
  sourcePolicyDecision,
  minimumConfidence: 0.7,
  claimBoundaries,
});

const lowConfidenceProof = buildScreenAiConfidencePolicyProof({
  schemaVersion: 'v0.6',
  proofId: 'screen-ai-confidence-policy-proof-low-confidence',
  evaluatedAt: sourceProof.generatedAt,
  localAiResult: {
    ...localAiResult,
    confidence: 0.31,
    unknownState: 'low-confidence',
    parentRuleReferences: ['screen-ai-confidence-allow-rule'],
  },
  parentRule: {
    ...parentBlockRule,
    ruleId: 'screen-ai-confidence-allow-rule',
    action: 'allow',
    reasonCode: 'parent-explicit-screen-category-allow',
  },
  sourcePolicyDecision: {
    ...sourcePolicyDecision,
    ruleIds: ['screen-ai-confidence-allow-rule'],
  },
  minimumConfidence: 0.7,
  claimBoundaries,
});

const proofSummary = {
  proof: 'screen-ai-confidence-policy-proof',
  proofTier: 'P3_CONTRACT_REAL_SOURCE_REPLAY',
  generatedAt: new Date().toISOString(),
  sourceProof: 'output/screen-ai-pipeline-proof/service-winrt-ocr-policy/proof-summary.json',
  sourceLiveSurface: sourceProof.sourceLiveSurface,
  sourceAnalysisRow: {
    rowId: sourceRow.rowId,
    providerKind: sourceRow.providerKind,
    primaryCategory: sourceRow.primaryCategory,
    confidence: sourceRow.confidence,
    imageDeletionState: sourceRow.imageDeletionState,
    rawImageRetained: sourceRow.rawImageRetained,
    custodyState: sourceRow.custodyState,
  },
  trustedConfidencePolicyProof: proof,
  lowConfidenceFallbackProof: lowConfidenceProof,
  assertions: {
    sourceUsedRealLivePublicPixels: sourceProof.assertions.sourceUsedLivePublicBrowserPixels === true,
    sourceRanWindowsWinRtOcr: sourceProof.assertions.sourceRanWindowsWinRtOcr === true,
    sourceTempImageDeleted: sourceProof.assertions.sourceTempImageDeleted === true,
    sourceRawImageNotRetained: sourceProof.assertions.sourceRawImageNotRetained === true,
    stricterParentRulePreserved: proof.policyDecision.action === 'block',
    lowConfidenceCannotAllow: lowConfidenceProof.policyDecision.action === 'unknown',
    dryRunOnly: proof.policyDecision.dryRun === true && lowConfidenceProof.policyDecision.dryRun === true,
    enforcementNotClaimed:
      proof.policyDecision.enforcementHandoffState === 'disabled' &&
      lowConfidenceProof.policyDecision.enforcementHandoffState === 'disabled',
  },
  nonClaims: [
    'This proof replays the existing real service WinRT OCR policy artifact through a confidence/degraded policy contract.',
    'It proves deterministic policy handling of confidence and stricter parent rules; it does not rerun live capture, download or run a production model, prove model quality, or claim enforcement.',
    'Raw screenshots remain deleted and are not embedded in this proof output.',
  ],
};

await mkdir(outputDir, { recursive: true });
await mkdir(testResultDir, { recursive: true });
await writeFile(resolve(outputDir, 'proof-summary.json'), `${JSON.stringify(proofSummary, null, 2)}\n`);
await writeFile(resolve(testResultDir, 'proof.json'), `${JSON.stringify(proofSummary, null, 2)}\n`);

console.log(`screen-ai-confidence-policy-proof-ok:${resolve(outputDir, 'proof-summary.json')}`);
