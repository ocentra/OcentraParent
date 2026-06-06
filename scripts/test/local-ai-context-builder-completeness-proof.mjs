import { execFileSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'ai-plan-proof', 'local-ai-context-builder-completeness-proof');
const testResultsDir = join(repoRoot, 'test-results', 'local-ai-context-builder-completeness-proof');
const proofSummaryPath = join(outputDir, 'proof-summary.json');
const validationCommandsPath = join(outputDir, 'validation-commands.log');
const testResultPath = join(testResultsDir, 'proof.json');
const observedAt = '2026-06-06T08:58:00.000Z';

async function main() {
  rmSync(outputDir, { recursive: true, force: true });
  rmSync(testResultsDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });
  mkdirSync(testResultsDir, { recursive: true });

  const commands = [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-context-builder-completeness-proof',
  ];
  runCommand(commands[0]);
  runCommand(commands[1]);

  const contract = await import(
    pathToFileURL(join(repoRoot, 'packages/parent-domain/dist/local-ai-context-builder-completeness-proof.js')).href
  );
  const proof = contract.buildLocalAiContextBuilderCompletenessProof(buildProofInput());
  const proofSummary = {
    status: 'ok',
    proofKind: proof.proofKind,
    generatedAt: new Date().toISOString(),
    proof: relativePath(proofSummaryPath),
    testResult: relativePath(testResultPath),
    validationCommands: relativePath(validationCommandsPath),
    contextStates: {
      ready: proof.readyResult.state,
      partial: proof.partialResult.state,
      forbiddenCustody: proof.forbiddenCustodyResult.state,
      unallowedCustody: proof.unallowedCustodyResult.state,
      unavailableRuntime: proof.unavailableRuntimeResult.state,
    },
    summary: proof.summary,
    claimBoundaries: proof.claimBoundaries,
    sourceArtifacts: [
      'packages/parent-domain/src/local-ai-context.ts',
      'packages/parent-domain/src/local-ai-context-builder.ts',
      'packages/parent-domain/src/local-ai-context-builder-completeness-proof.ts',
    ],
    claimsProved: [
      'The real local AI context builder returns a ready local-only context preserving browser, app/game, network-flow, screen-summary, parent-rule, runtime, memory, graph, and prompt refs.',
      'The builder returns typed partial state when required evidence is missing and typed rejected states for forbidden hosted custody and unallowed custody.',
      'Unavailable local runtime degrades the context explicitly without remote/API fallback or policy/enforcement authority.',
    ],
    nonClaims: [
      'This proof does not execute a model or prove production model quality.',
      'This proof does not render a portal UI or dispatch enforcement.',
      'This proof does not use remote/API AI or retain raw prompts or raw evidence.',
      'This proof does not update the product capability checklist because that shared file is owned by another active lane.',
    ],
  };

  writeJson(proofSummaryPath, proofSummary);
  writeJson(testResultPath, proof);
  writeFileSync(validationCommandsPath, `${commands.join('\n')}\n`);
  console.log(`local-ai-context-builder-completeness-proof-ok:${relativePath(proofSummaryPath)}`);
}

function buildProofInput() {
  return {
    schemaVersion: 'v0.6',
    readyInput: contextInput(),
    partialInput: contextInput({
      evidenceReferences: [evidenceReference('screen-ref-context-builder', 'screen-summary')],
    }),
    forbiddenCustodyInput: contextInput({
      evidenceReferences: [evidenceReference('hosted-ref-context-builder', 'browser', 'ocentra-hosted-non-activity')],
    }),
    unallowedCustodyInput: contextInput({
      evidenceReferences: [evidenceReference('export-ref-context-builder', 'browser', 'parent-owned-export')],
    }),
    unavailableRuntimeInput: contextInput({ runtimeReferences: [] }),
    claimBoundaries: {
      modelExecutionClaimed: false,
      modelQualityClaimed: false,
      policyAuthorityClaimed: false,
      enforcementClaimed: false,
      portalUiClaimed: false,
      remoteApiAiUsed: false,
      rawPromptRetained: false,
      rawEvidenceRetained: false,
    },
  };
}

const childProfile = { childProfileId: 'child-context-builder-complete', displayName: 'Sam' };
const device = {
  deviceId: 'device-context-builder-complete',
  childProfileId: 'child-context-builder-complete',
  label: 'Sam Windows PC',
  platform: 'windows',
};
const sourceEvidence = {
  evidenceReferenceId: 'context-builder-complete-source',
  kind: 'journal-event',
  observedAt,
};
const parentRuleContextReference = {
  parentRuleRefId: 'context-builder-parent-rule-context',
  policyVersion: 'context-builder-policy-v1',
  family: { familyId: 'family-context-builder-complete' },
  childProfile,
  device,
  rule: {
    ruleId: 'context-builder-rule',
    target: {
      targetId: 'context-builder-target',
      targetType: 'category',
      targetValue: 'screen-safety',
    },
    action: 'warn',
    scheduleId: null,
    priority: 10,
    reasonCode: 'parent-rule-browser-safety',
    createdBy: { actorId: 'parent-context-builder', role: 'parent' },
    enabled: true,
    effectiveFrom: null,
    effectiveUntil: null,
  },
  targetEvidenceRefs: ['screen-ref-context-builder'],
  custody: 'parent-device-cache',
  updatedAt: observedAt,
  expiresAt: null,
};
const request = {
  schemaVersion: 'v0.6',
  requestId: 'context-builder-complete-request',
  requestedAt: '2026-06-06T08:58:02.000Z',
  childProfile,
  device,
  requestedEvaluationKind: 'mixed-context',
  requiredEvidenceKinds: ['browser', 'app-game', 'network-flow', 'screen-summary'],
  parentRuleContextReferences: [parentRuleContextReference],
  modelTaskRequirements: ['safety-decision'],
  allowedCustody: ['child-device-query-store', 'child-device-journal'],
  promptVersion: 'context-builder-prompt-v1',
};

function contextInput(overrides = {}) {
  return {
    contextId: 'context-builder-complete-context',
    request,
    evidenceReferences: [
      evidenceReference('browser-ref-context-builder', 'browser'),
      evidenceReference('app-game-ref-context-builder', 'app-game'),
      evidenceReference('network-ref-context-builder', 'network-flow'),
      evidenceReference('screen-ref-context-builder', 'screen-summary'),
    ],
    runtimeReferences: [
      {
        runtimeReferenceId: 'runtime-context-builder-complete',
        providerId: 'local-provider-context-builder',
        modelId: 'local-model-context-builder',
        modelReference: 'artifact:local-context-builder-model',
        privacyMode: 'local-only',
        adapterBoundary: 'local-adapter-ready',
        executionState: 'dry-run-ready',
        providerSource: 'local-model-cache',
        loadState: 'loaded',
        capabilityFlags: ['safety-decision', 'classification'],
        resourceClass: 'cpu',
        degradedState: 'none',
        lastCheckedAt: observedAt,
        unavailableReason: null,
      },
    ],
    memoryReferences: [
      {
        memoryReferenceId: 'context-builder-complete-memory',
        kind: 'recent-activity',
        sourceEvidenceReferences: [sourceEvidence],
        sourcePolicyVersion: 'context-builder-policy-v1',
        generatedAt: observedAt,
        confidence: 0.82,
        derivedIndexVersion: 'context-builder-memory-index-v1',
      },
    ],
    graphReferences: [
      {
        graphReferenceId: 'context-builder-complete-graph',
        kind: 'graph-edge',
        sourceEvidenceReferences: [sourceEvidence],
        sourcePolicyVersion: 'context-builder-policy-v1',
        generatedAt: observedAt,
        confidence: 0.79,
        derivedIndexVersion: 'context-builder-graph-index-v1',
      },
    ],
    ...overrides,
  };
}

function evidenceReference(evidenceRefId, evidenceKind, custody = 'child-device-query-store') {
  return {
    evidenceRefId,
    evidence: {
      evidenceReferenceId: `stored-${evidenceRefId}`,
      kind: 'query-store-summary',
      observedAt,
    },
    evidenceKind,
    sourceSchemaVersion: 'v0.6',
    observedAt,
    ingestedAt: '2026-06-06T08:58:01.000Z',
    freshUntil: '2026-06-06T09:08:00.000Z',
    sourceId: `source-${evidenceRefId}`,
    adapterId: `adapter-${evidenceRefId}`,
    device,
    childProfile,
    custody,
    retentionState: custody === 'ocentra-hosted-non-activity' ? 'unavailable' : 'local',
    confidence: 0.86,
    confidenceKind: 'classifier',
    capabilityStatus: custody === 'unavailable' ? 'unavailable' : 'available',
    degradedReasons: custody === 'unavailable' ? ['custody-unavailable'] : [],
    unknownReasons: [],
    sourceEvidenceReferences: [sourceEvidence],
  };
}

function runCommand(command) {
  execFileSync('cmd', ['/c', command], { stdio: 'inherit' });
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

await main();
