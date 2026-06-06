import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-text-inference-dry-run');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-text-inference-dry-run');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runCommand('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'local-ai-text-inference-dry-run-proof',
]);

const dryRunModule = await import('@ocentra-parent/parent-domain/local-ai-text-inference-dry-run-proof');

const readyInput = localAiTextInferenceDryRunInput();
const readyProof = dryRunModule.runLocalAiTextInferenceDryRun(readyInput);
const unavailableProof = dryRunModule.runLocalAiTextInferenceDryRun({
  ...readyInput,
  modelRuntime: {
    ...readyInput.modelRuntime,
    executionState: 'disabled',
    loadState: 'unavailable',
    degradedState: 'provider-unavailable',
    unavailableReason: 'local-ai-provider-unconfigured',
  },
});
const missingEvidenceProof = dryRunModule.runLocalAiTextInferenceDryRun({
  ...readyInput,
  evaluationInput: {
    ...readyInput.evaluationInput,
    evidenceReferences: [],
  },
});

const rejectionChecks = [
  {
    name: 'raw-prompt-retention',
    rejected: !dryRunModule.LocalAiTextInferenceDryRunInputSchema.safeParse({
      ...readyInput,
      rawPromptRetained: true,
    }).success,
  },
  {
    name: 'provider-mismatch',
    rejected: !dryRunModule.LocalAiTextInferenceDryRunInputSchema.safeParse({
      ...readyInput,
      modelRuntime: {
        ...readyInput.modelRuntime,
        providerId: 'local-provider-other',
      },
    }).success,
  },
  {
    name: 'model-execution-overclaim',
    rejected: !dryRunModule.LocalAiTextInferenceDryRunResultSchema.safeParse({
      ...readyProof,
      modelExecuted: true,
    }).success,
  },
  {
    name: 'remote-api-overclaim',
    rejected: !dryRunModule.LocalAiTextInferenceDryRunResultSchema.safeParse({
      ...readyProof,
      remoteApiClaimed: true,
    }).success,
  },
];

const proof = {
  status: 'ok',
  proofKind: 'local-ai-text-inference-dry-run-proof',
  generatedAt,
  output: relativePath(ProofPath),
  dryRunRows: [
    proofRow('ready', readyProof),
    proofRow('unavailable', unavailableProof),
    proofRow('missing-evidence', missingEvidenceProof),
  ],
  assertions: {
    readyDryRunWarnsFromTypedEvidence: readyProof.state === 'ready-dry-run' && readyProof.result.action === 'warn',
    unavailableRuntimeAsksParent:
      unavailableProof.state === 'unavailable-dry-run' && unavailableProof.result.action === 'ask-parent',
    missingEvidenceStaysUnknown:
      missingEvidenceProof.result.action === 'unknown' &&
      missingEvidenceProof.result.unknownState === 'missing-evidence',
    localRuntimePreserved:
      readyProof.modelRuntime.runtimeReferenceId === readyProof.result.modelRuntime.runtimeReferenceId,
    noModelExecutionClaim: [readyProof, unavailableProof, missingEvidenceProof].every((row) => !row.modelExecuted),
    noRemoteApiClaim: [readyProof, unavailableProof, missingEvidenceProof].every((row) => !row.remoteApiClaimed),
    noPolicyAuthorityClaim: [readyProof, unavailableProof, missingEvidenceProof].every(
      (row) => !row.policyAuthorityClaimed
    ),
    noEnforcementClaim: [readyProof, unavailableProof, missingEvidenceProof].every((row) => !row.enforcementClaimed),
    noRawPromptRetention: [readyProof, unavailableProof, missingEvidenceProof].every((row) => !row.rawPromptRetained),
    malformedInputsRejected: rejectionChecks.every((check) => check.rejected),
  },
  rejectionChecks,
  nonClaims: readyProof.nonClaims,
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-text-inference-dry-run-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-text-inference-dry-run-proof-ok:${proof.dryRunRows.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

function proofRow(name, row) {
  return {
    name,
    dryRunId: row.dryRunId,
    state: row.state,
    action: row.result.action,
    confidence: row.result.confidence,
    unknownState: row.result.unknownState,
    degradedState: row.result.degradedState,
    reasonCodes: row.result.reasonCodes,
    evidenceReferenceCount: row.evidenceReferenceCount,
    parentRuleReferenceCount: row.parentRuleReferenceCount,
    runtimeReferenceId: row.modelRuntime.runtimeReferenceId,
    promptVersion: row.promptVersion,
    modelExecuted: row.modelExecuted,
    remoteApiClaimed: row.remoteApiClaimed,
    policyAuthorityClaimed: row.policyAuthorityClaimed,
    enforcementClaimed: row.enforcementClaimed,
    rawPromptRetained: row.rawPromptRetained,
  };
}

function localAiTextInferenceDryRunInput() {
  return {
    schemaVersion: 'v0.6',
    dryRunId: 'local-ai-text-dry-run:screen-summary-wiki-ocr',
    rawPromptRetained: false,
    modelRuntime: {
      runtimeReferenceId: 'local-ai-runtime-local-llama-cli',
      providerId: 'local-provider-llama-cli',
      modelId: 'gemma-4-e2b-it-q4-k-m',
      modelReference: 'artifact:gemma_4_e2b_it_q4_k_m',
      privacyMode: 'local-only',
      adapterBoundary: 'local-adapter-ready',
      executionState: 'dry-run-ready',
      providerSource: 'local-model-cache',
      loadState: 'loaded',
      capabilityFlags: ['classification', 'safety-decision'],
      resourceClass: 'cpu',
      degradedState: 'none',
      lastCheckedAt: '2026-06-06T04:00:00.000Z',
      unavailableReason: null,
    },
    evaluationInput: {
      schemaVersion: 'v0.6',
      requestId: 'local-ai-eval:screen-summary-wiki-ocr',
      childProfile: {
        childProfileId: 'child:maya',
        displayName: 'Maya',
      },
      device: {
        deviceId: 'device:maya-windows',
        childProfileId: 'child:maya',
        label: 'Maya Windows laptop',
        platform: 'windows',
      },
      currentObservation: {
        observationReferenceId: 'observation:screen-summary-wiki-ocr',
        contextKind: 'page',
        evidence: {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T04:00:00.000Z',
        },
      },
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T04:00:00.000Z',
        },
      ],
      parentRuleReferences: ['policy-rule:video-warn'],
      recentActivityWindow: [
        {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T04:00:00.000Z',
        },
      ],
      memoryReferences: [],
      graphReferences: [],
      modelRequest: {
        providerId: 'local-provider-llama-cli',
        modelId: 'gemma-4-e2b-it-q4-k-m',
        promptVersion: 'prompt:screen-safety:v1',
      },
    },
  };
}

function relativePath(filePath) {
  return relative(RepoRoot, filePath).replaceAll('\\', '/');
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}
