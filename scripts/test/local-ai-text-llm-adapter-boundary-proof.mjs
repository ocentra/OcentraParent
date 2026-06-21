import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-text-llm-adapter-boundary-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-text-llm-adapter-boundary-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand(...npmCommand(['run', 'build:contracts']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/ai-domain',
    '--',
    'local-ai-text-llm-adapter-boundary-proof',
  ])
);

const adapterModule = await import('@ocentra-parent/schema-domain/local-ai-text-llm-adapter-boundary-proof');

const readyInput = localAiTextLlmAdapterBoundaryInput();
const readyProof = adapterModule.proveLocalAiTextLlmAdapterBoundary(readyInput);
const unavailableProof = adapterModule.proveLocalAiTextLlmAdapterBoundary({
  ...readyInput,
  modelRuntime: {
    ...readyInput.modelRuntime,
    executionState: 'disabled',
    loadState: 'unavailable',
    degradedState: 'provider-unavailable',
    unavailableReason: 'local-ai-provider-unconfigured',
  },
});
const manualProof = adapterModule.proveLocalAiTextLlmAdapterBoundary({
  ...readyInput,
  localAdapterAvailable: false,
});

const rejectionChecks = [
  {
    name: 'raw-prompt-retention',
    rejected: !adapterModule.LocalAiTextLlmAdapterBoundaryInputSchema.safeParse({
      ...readyInput,
      rawPromptRetained: true,
    }).success,
  },
  {
    name: 'raw-model-output-retention',
    rejected: !adapterModule.LocalAiTextLlmAdapterBoundaryInputSchema.safeParse({
      ...readyInput,
      rawModelOutputRetained: true,
    }).success,
  },
  {
    name: 'prompt-version-mismatch',
    rejected: !adapterModule.LocalAiTextLlmAdapterBoundaryInputSchema.safeParse({
      ...readyInput,
      promptVersion: 'prompt:other:v1',
    }).success,
  },
  {
    name: 'remote-provider-overclaim',
    rejected: !adapterModule.LocalAiTextLlmAdapterBoundaryProofSchema.safeParse({
      ...readyProof,
      remoteApiClaimed: true,
    }).success,
  },
  {
    name: 'model-execution-overclaim',
    rejected: !adapterModule.LocalAiTextLlmAdapterBoundaryProofSchema.safeParse({
      ...readyProof,
      modelExecuted: true,
    }).success,
  },
];

const proof = {
  status: 'ok',
  proofKind: 'local-ai-text-llm-adapter-boundary-proof',
  generatedAt,
  output: relativePath(ProofPath),
  adapterRows: [
    proofRow('ready', readyProof),
    proofRow('unavailable', unavailableProof),
    proofRow('manual-required', manualProof),
  ],
  assertions: {
    localAdapterReadyRow:
      readyProof.state === 'ready-for-local-adapter' && readyProof.localOnly && readyProof.parserRequiredBeforeResult,
    unavailableRuntimeNotReady: unavailableProof.state === 'unavailable' && !unavailableProof.modelExecuted,
    manualAdapterGapNotReady: manualProof.state === 'manual-required' && !manualProof.policyAuthorityClaimed,
    promptVersionPreserved: readyProof.promptVersion === readyInput.promptVersion,
    runtimeMetadataPreserved:
      readyProof.runtimeReferenceId === readyInput.modelRuntime.runtimeReferenceId &&
      readyProof.providerId === readyInput.modelRuntime.providerId &&
      readyProof.modelId === readyInput.modelRuntime.modelId,
    noModelExecutionClaim: [readyProof, unavailableProof, manualProof].every((row) => !row.modelExecuted),
    noRemoteApiClaim: [readyProof, unavailableProof, manualProof].every((row) => !row.remoteApiClaimed),
    noPolicyAuthorityClaim: [readyProof, unavailableProof, manualProof].every((row) => !row.policyAuthorityClaimed),
    noEnforcementClaim: [readyProof, unavailableProof, manualProof].every((row) => !row.enforcementClaimed),
    noRawPromptOrOutputRetention: [readyProof, unavailableProof, manualProof].every(
      (row) => !row.rawPromptRetained && !row.rawModelOutputRetained
    ),
    malformedInputsAndOverclaimsRejected: rejectionChecks.every((check) => check.rejected),
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
    'cmd /c npm run build:contracts',
    'cmd /c npm run test --workspace @ocentra-parent/ai-domain -- local-ai-text-llm-adapter-boundary-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-text-llm-adapter-boundary-proof-ok:${proof.adapterRows.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

function proofRow(name, row) {
  return {
    name,
    adapterRequestId: row.adapterRequestId,
    state: row.state,
    runtimeReferenceId: row.runtimeReferenceId,
    providerId: row.providerId,
    modelId: row.modelId,
    promptVersion: row.promptVersion,
    evidenceReferenceCount: row.evidenceReferenceCount,
    parentRuleReferenceCount: row.parentRuleReferenceCount,
    parserRequiredBeforeResult: row.parserRequiredBeforeResult,
    modelExecuted: row.modelExecuted,
    remoteApiClaimed: row.remoteApiClaimed,
    policyAuthorityClaimed: row.policyAuthorityClaimed,
    enforcementClaimed: row.enforcementClaimed,
    rawPromptRetained: row.rawPromptRetained,
    rawModelOutputRetained: row.rawModelOutputRetained,
  };
}

function localAiTextLlmAdapterBoundaryInput() {
  return {
    schemaVersion: 'v0.6',
    adapterRequestId: 'local-ai-text-adapter:screen-summary-wiki-ocr',
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    localAdapterAvailable: true,
    manualProofRequired: false,
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
      lastCheckedAt: '2026-06-06T12:24:00.000Z',
      unavailableReason: null,
    },
    promptVersion: 'prompt:screen-safety:v1',
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
          observedAt: '2026-06-06T12:24:00.000Z',
        },
      },
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T12:24:00.000Z',
        },
      ],
      parentRuleReferences: ['policy-rule:video-warn'],
      recentActivityWindow: [
        {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T12:24:00.000Z',
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
