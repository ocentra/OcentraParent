import { execFileSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'ai-plan-proof', 'screen-ai-memory-graph-source-guard-proof');
const testResultsDir = join(repoRoot, 'test-results', 'screen-ai-memory-graph-source-guard-proof');
const proofSummaryPath = join(outputDir, 'proof-summary.json');
const testResultPath = join(testResultsDir, 'proof.json');
const observedAt = '2026-06-05T17:15:00.000Z';

await main();

async function main() {
  rmSync(outputDir, { recursive: true, force: true });
  rmSync(testResultsDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });
  mkdirSync(testResultsDir, { recursive: true });
  execFileSync(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']), {
    stdio: 'inherit',
  });

  const contract = await import(
    pathToFileURL(join(repoRoot, 'packages/parent-domain/dist/screen-ai-memory-graph-source-guard-proof.js')).href
  );
  const proof = contract.buildScreenAiMemoryGraphSourceGuardProof(buildProofInput());
  const proofSummary = {
    status: 'ok',
    proofKind: proof.proofKind,
    generatedAt: new Date().toISOString(),
    proof: proofSummaryRelativePath(),
    testResult: relativePath(testResultPath),
    contextState: proof.contextResult.state,
    sourceGuardSummary: proof.sourceGuardSummary,
    claimBoundaries: proof.claimBoundaries,
    sourceArtifacts: [
      'packages/parent-domain/src/local-ai-context-builder.ts',
      'packages/parent-domain/src/local-ai-context-selection.ts',
      'packages/parent-domain/src/local-ai-references.ts',
    ],
    claimsProved: [
      'Screen AI recent memory and graph references must cite selected stored screen evidence before model input.',
      'Uncited or ungrounded derived memory/graph context is rejected instead of silently entering a local AI decision.',
      'The proof path keeps local-only/no-remote/no-policy-authority/no-enforcement boundaries explicit.',
    ],
    nonClaims: [
      'This proof does not execute a production VLM/OCR model or claim model quality.',
      'This proof does not persist a SQLite AI journal or render a portal screen.',
      'This proof does not enforce; policy remains the authority for later action paths.',
    ],
  };

  writeJson(proofSummaryPath, proofSummary);
  writeJson(testResultPath, proof);
  console.log(`screen-ai-memory-graph-source-guard-proof-ok:${relativePath(proofSummaryPath)}`);
}

function buildProofInput() {
  const childProfile = { childProfileId: 'child-screen-ai-memory', displayName: 'Sam' };
  const device = {
    deviceId: 'screen-ai-memory-device',
    childProfileId: 'child-screen-ai-memory',
    label: 'Sam Windows PC',
    platform: 'windows',
  };
  const rawCaptureAuditEvidence = {
    evidenceReferenceId: 'screen-ai-memory-raw-capture-deleted',
    kind: 'journal-event',
    observedAt,
  };
  return {
    schemaVersion: 'v0.6',
    contextInput: {
      contextId: 'screen-ai-memory-context',
      request: {
        schemaVersion: 'v0.6',
        requestId: 'screen-ai-memory-request',
        requestedAt: '2026-06-05T17:15:03.000Z',
        childProfile,
        device,
        requestedEvaluationKind: 'screen-summary',
        requiredEvidenceKinds: ['screen-summary'],
        parentRuleContextReferences: [parentRuleContextReference(childProfile, device)],
        modelTaskRequirements: ['classification', 'safety-decision'],
        allowedCustody: ['child-device-query-store'],
        promptVersion: 'screen-ai-memory-prompt-v1',
      },
      evidenceReferences: [screenEvidenceReference(childProfile, device, rawCaptureAuditEvidence)],
      runtimeReferences: [runtimeReference()],
      memoryReferences: [memoryReference(rawCaptureAuditEvidence)],
      graphReferences: [graphReference(rawCaptureAuditEvidence)],
    },
    claimBoundaries: {
      remoteAiUsed: false,
      apiAiUsed: false,
      policyAuthorityClaimed: false,
      enforcementClaimed: false,
      uncitedMemoryAllowed: false,
      uncitedGraphAllowed: false,
      rawEvidenceEmbedded: false,
    },
  };
}

function screenEvidenceReference(childProfile, device, sourceEvidence) {
  return {
    evidenceRefId: 'screen-ai-memory-context-ref',
    evidence: {
      evidenceReferenceId: 'screen-ai-memory-stored-screen-summary',
      kind: 'query-store-summary',
      observedAt,
    },
    evidenceKind: 'screen-summary',
    sourceSchemaVersion: 'v0.6',
    observedAt,
    ingestedAt: '2026-06-05T17:15:02.000Z',
    freshUntil: '2026-06-05T17:20:00.000Z',
    sourceId: 'screen-ai-memory-screen-service',
    adapterId: 'screen-ai-memory-winrt-ocr',
    device,
    childProfile,
    custody: 'child-device-query-store',
    retentionState: 'local',
    confidence: 0.9,
    confidenceKind: 'model',
    capabilityStatus: 'available',
    degradedReasons: [],
    unknownReasons: [],
    sourceEvidenceReferences: [sourceEvidence],
  };
}

function parentRuleContextReference(childProfile, device) {
  return {
    parentRuleRefId: 'screen-ai-memory-parent-rule-context',
    policyVersion: 'screen-ai-memory-policy-v1',
    family: { familyId: 'screen-ai-memory-family' },
    childProfile,
    device,
    rule: {
      ruleId: 'screen-ai-memory-rule',
      target: {
        targetId: 'screen-ai-memory-target',
        targetType: 'category',
        targetValue: 'screen-safety',
      },
      action: 'warn',
      scheduleId: null,
      priority: 10,
      reasonCode: 'screen-ai-memory-cited-context',
      createdBy: { actorId: 'screen-ai-memory-parent', role: 'parent' },
      enabled: true,
      effectiveFrom: null,
      effectiveUntil: null,
    },
    targetEvidenceRefs: ['screen-ai-memory-context-ref'],
    custody: 'child-device-query-store',
    updatedAt: observedAt,
    expiresAt: null,
  };
}

function runtimeReference() {
  return {
    runtimeReferenceId: 'screen-ai-memory-runtime',
    providerId: 'screen-ai-memory-provider',
    modelId: 'screen-ai-memory-model',
    modelReference: 'screen-ai-memory-model-local-cache',
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

function memoryReference(sourceEvidence) {
  return {
    memoryReferenceId: 'screen-ai-memory-recent-activity',
    kind: 'recent-activity',
    sourceEvidenceReferences: [sourceEvidence],
    sourcePolicyVersion: null,
    generatedAt: observedAt,
    confidence: 0.82,
    derivedIndexVersion: 'screen-ai-memory-index-v1',
  };
}

function graphReference(sourceEvidence) {
  return {
    graphReferenceId: 'screen-ai-memory-graph-edge',
    kind: 'graph-edge',
    sourceEvidenceReferences: [sourceEvidence],
    sourcePolicyVersion: null,
    generatedAt: observedAt,
    confidence: 0.78,
    derivedIndexVersion: 'screen-ai-graph-index-v1',
  };
}

function proofSummaryRelativePath() {
  return relativePath(proofSummaryPath);
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
