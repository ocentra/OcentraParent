import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OcrProofRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'screen-winrt-ocr-worker');
const OcrProofPath = join(OcrProofRoot, 'proof-summary.json');
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'screen-summary-parent-explanation');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ObservedAtFallback = '2026-06-05T10:43:30.710Z';
const ChildProfile = { childProfileId: 'screen-summary-parent-explanation-child', displayName: 'Sam' };
const Device = {
  deviceId: 'screen-summary-parent-explanation-windows-device',
  childProfileId: ChildProfile.childProfileId,
  label: 'Sam Windows PC',
  platform: 'windows',
};
const ClaimBoundaries = {
  rawImageRetained: false,
  remoteAiUsed: false,
  apiAiUsed: false,
  policyAuthorityClaimed: false,
  enforcementClaimed: false,
  portalRuntimeClaimed: false,
};

runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));

const { buildLocalAiEvidenceContext } = await import('@ocentra-parent/ai-domain/local-ai-context-builder');
const { buildScreenSummaryParentExplanation } = await importDist('local-ai-screen-summary-parent-explanation.js');
const ocrProof = JSON.parse(readFileSync(OcrProofPath, 'utf8'));
const scenarios = ocrProof.proof.scenarios;
const rows = scenarios.map((scenario) => explanationRowForScenario(scenario));
const failures = rows.filter(rowFailsValidation).map((row) => `${row.ocrResultId} explanation row failed invariants`);

if (failures.length > 0) {
  throw new Error(`Screen summary parent explanation proof failed:\n${failures.join('\n')}`);
}

const proof = {
  status: 'ok',
  proofKind: 'screen-summary-parent-explanation-proof',
  generatedAt: new Date().toISOString(),
  sourceProof: relativePath(OcrProofPath),
  output: relativePath(ProofPath),
  rows,
  summary: {
    sourceScenarioCount: scenarios.length,
    readyExplanationCount: rows.filter((row) => row.readiness === 'ready-for-parent-audit').length,
    screenSummaryRefCount: rows.reduce((count, row) => count + row.screenSummaryRefs.length, 0),
    localOnly: true,
    remoteAiUsed: false,
    apiAiUsed: false,
    rawImageRetained: false,
    enforcementClaimed: false,
    failures: failures.length,
  },
  nonClaims: [
    'This proof replays already-captured WinRT OCR proof artifacts; it does not create new screen captures.',
    'This proof does not claim production OCR/VLM quality, portal UI, remote/API AI, policy authority, or final enforcement.',
  ],
};

mkdirSync(OutputRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(`screen-summary-parent-explanation-proof-ok:${ProofPath}`);

function explanationRowForScenario(scenario) {
  const policyDecision = policyDecisionForScenario(scenario);
  const contextResult = buildLocalAiEvidenceContext(localAiContextInput(scenario, policyDecision));
  const explanation = buildScreenSummaryParentExplanation({
    schemaVersion: 'v0.6',
    explanationId: `${scenario.ocrResultId}-parent-explanation`,
    generatedAt: scenario.analyzedAt ?? ObservedAtFallback,
    contextResult,
    policyDecision,
    claimBoundaries: ClaimBoundaries,
  });
  return {
    ocrResultId: scenario.ocrResultId,
    sourceQueueJobId: scenario.queueJobId,
    primaryCategory: scenario.primaryCategory,
    imageDigest: scenario.imageDigest,
    sourceImageDeletionState: scenario.imageDeletionState,
    sourceCustodyState: scenario.custodyState,
    sourceRawImageRetained: scenario.rawImageRetained,
    contextState: contextResult.state,
    readiness: explanation.readiness,
    screenSummaryRefs: explanation.screenSummaryRefs,
    auditEvidenceReferences: explanation.auditEvidenceReferences.map((reference) => reference.evidenceReferenceId),
    policyDecisionRef: explanation.policyDecisionRef,
    policyAction: explanation.policyAction,
    policyReasonCodes: explanation.policyReasonCodes,
    policyDryRun: explanation.policyDryRun,
    enforcementHandoffState: explanation.enforcementHandoffState,
    parentRuleRefs: explanation.parentRuleRefs,
    localModelRuntimeRefs: explanation.localModelRuntimeRefs,
    custodyLabels: explanation.custodyLabels,
    deletionReasons: explanation.deletionReasons,
    explanationReasons: explanation.explanationReasons,
    claimBoundaries: explanation.claimBoundaries,
  };
}

function localAiContextInput(scenario, policyDecision) {
  const observedAt = scenario.analyzedAt ?? ObservedAtFallback;
  const screenRefId = `${scenario.ocrResultId}-screen-summary-ref`;
  return {
    contextId: `${scenario.ocrResultId}-parent-explanation-context`,
    request: {
      schemaVersion: 'v0.6',
      requestId: `${scenario.ocrResultId}-parent-explanation-request`,
      requestedAt: observedAt,
      childProfile: ChildProfile,
      device: Device,
      requestedEvaluationKind: 'screen-summary',
      requiredEvidenceKinds: ['screen-summary'],
      parentRuleContextReferences: [parentRuleContext(scenario, policyDecision, screenRefId, observedAt)],
      modelTaskRequirements: ['classification', 'safety-decision'],
      allowedCustody: ['child-device-query-store'],
      promptVersion: 'screen-summary-parent-explanation-v1',
    },
    evidenceReferences: [screenSummaryEvidence(scenario, screenRefId, observedAt)],
    runtimeReferences: [runtimeStatus(scenario, observedAt)],
    memoryReferences: [],
    graphReferences: [],
  };
}

function parentRuleContext(scenario, policyDecision, screenRefId, observedAt) {
  return {
    parentRuleRefId: `${scenario.ocrResultId}-parent-explanation-rule-context`,
    policyVersion: 'screen-summary-parent-explanation-policy-v1',
    family: { familyId: 'screen-summary-parent-explanation-family' },
    childProfile: ChildProfile,
    device: Device,
    rule: {
      ruleId: policyDecision.ruleIds[0],
      target: {
        targetId: `${scenario.primaryCategory}-screen-summary-parent-explanation-target`,
        targetType: 'category',
        targetValue: scenario.primaryCategory,
      },
      action: policyDecision.action,
      scheduleId: null,
      priority: 10,
      reasonCode: policyDecision.reasonCodes[0],
      createdBy: { actorId: 'parent-1', role: 'parent' },
      enabled: true,
      effectiveFrom: null,
      effectiveUntil: null,
    },
    targetEvidenceRefs: [screenRefId],
    custody: 'parent-device-cache',
    updatedAt: observedAt,
    expiresAt: null,
  };
}

function screenSummaryEvidence(scenario, screenRefId, observedAt) {
  return {
    evidenceRefId: screenRefId,
    evidence: {
      evidenceReferenceId: `${scenario.ocrResultId}-query-store-summary`,
      kind: 'query-store-summary',
      observedAt,
    },
    evidenceKind: 'screen-summary',
    sourceSchemaVersion: 'v0.6',
    observedAt,
    ingestedAt: observedAt,
    freshUntil: null,
    sourceId: scenario.ocrResultId,
    adapterId: scenario.modelRuntimeRef,
    device: Device,
    childProfile: ChildProfile,
    custody: scenario.custodyState,
    retentionState: scenario.imageDeletionState === 'deleted' ? 'deleted-source' : 'temporary',
    confidence: scenario.confidence,
    confidenceKind: 'classifier',
    capabilityStatus: scenario.capabilityStatus === 'ready' ? 'available' : 'degraded',
    degradedReasons:
      scenario.imageDeletionState === 'deleted' ? ['screen-image-deleted'] : ['screen-deletion-unconfirmed'],
    unknownReasons: scenario.primaryCategory === 'unknown' ? ['missing-evidence'] : [],
    sourceEvidenceReferences: scenario.sourceEvidenceRefs.map((reference) => ({
      evidenceReferenceId: reference.evidenceId,
      kind: 'journal-event',
      observedAt,
    })),
  };
}

function runtimeStatus(scenario, observedAt) {
  return {
    runtimeReferenceId: `${scenario.modelRuntimeRef}-parent-explanation`,
    providerId: scenario.ocrEngine,
    modelId: scenario.modelId,
    modelReference: scenario.modelRuntimeRef,
    privacyMode: 'local-only',
    adapterBoundary: 'local-adapter-ready',
    executionState: 'dry-run-ready',
    providerSource: 'local-model-cache',
    loadState: 'loaded',
    capabilityFlags: ['classification', 'safety-decision'],
    resourceClass: 'cpu',
    degradedState: 'none',
    lastCheckedAt: observedAt,
    unavailableReason: null,
  };
}

function policyDecisionForScenario(scenario) {
  const scenarioSlug = scenario.ocrResultId.replace('screen-winrt-ocr-result-', '');
  return JSON.parse(readFileSync(join(OcrProofRoot, scenarioSlug, '07-policy-decision.json'), 'utf8'));
}

function rowFailsValidation(row) {
  return (
    row.contextState !== 'ready' ||
    row.readiness !== 'ready-for-parent-audit' ||
    row.sourceImageDeletionState !== 'deleted' ||
    row.sourceRawImageRetained !== false ||
    row.sourceCustodyState !== 'child-device-query-store' ||
    row.screenSummaryRefs.length !== 1 ||
    row.auditEvidenceReferences.length === 0 ||
    row.parentRuleRefs.length === 0 ||
    row.policyReasonCodes.length === 0 ||
    !row.custodyLabels.includes('child-device-query-store') ||
    !row.deletionReasons.includes('screen-image-deleted') ||
    !row.policyDryRun ||
    row.enforcementHandoffState === 'handed-off' ||
    Object.values(row.claimBoundaries).some((claim) => claim !== false)
  );
}

async function importDist(fileName) {
  return import(pathToFileURL(join(RepoRoot, 'packages', 'parent-domain', 'dist', fileName)).href);
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}

function relativePath(path) {
  return relative(RepoRoot, path).replaceAll('\\', '/');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
