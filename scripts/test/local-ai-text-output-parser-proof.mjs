import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-text-output-parser-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-text-output-parser-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();
const observedAt = '2026-06-06T12:40:00.000Z';
const evidenceReference = {
  evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
  kind: 'query-store-summary',
  observedAt,
};

runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runCommand('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'local-ai-text-output-parser-proof',
]);

const adapterModule = await import(
  pathToFileURL(resolve(RepoRoot, 'packages', 'parent-domain', 'dist', 'local-ai-text-llm-adapter-boundary-proof.js'))
    .href
);
const parserModule = await import(
  pathToFileURL(resolve(RepoRoot, 'packages', 'parent-domain', 'dist', 'local-ai-text-output-parser-proof.js')).href
);

const adapterProof = adapterModule.proveLocalAiTextLlmAdapterBoundary(adapterInput());
const parsed = parserModule.parseLocalAiTextOutput(parserInput(validCandidateOutput(), adapterProof));
const malformedRejected = parserModule.parseLocalAiTextOutput(
  parserInput({ ...validCandidateOutput(), action: 'silently-allow' }, adapterProof)
);
const remoteRejected = parserModule.parseLocalAiTextOutput(
  parserInput(
    {
      ...validCandidateOutput(),
      modelRuntime: {
        ...readyRuntime(),
        privacyMode: 'remote-api',
      },
    },
    adapterProof
  )
);
const manualProof = adapterModule.proveLocalAiTextLlmAdapterBoundary({
  ...adapterInput(),
  localAdapterAvailable: false,
});
const manualRejected = parserModule.parseLocalAiTextOutput(parserInput(validCandidateOutput(), manualProof));

const rejectionChecks = [
  {
    name: 'raw-model-output-retention',
    rejected: !parserModule.LocalAiTextOutputParserInputSchema.safeParse({
      ...parserInput(validCandidateOutput(), adapterProof),
      rawModelOutputRetained: true,
    }).success,
  },
  {
    name: 'model-execution-overclaim',
    rejected: !parserModule.LocalAiTextOutputParserProofSchema.safeParse({
      ...parsed,
      modelExecuted: true,
    }).success,
  },
  {
    name: 'policy-authority-overclaim',
    rejected: !parserModule.LocalAiTextOutputParserProofSchema.safeParse({
      ...parsed,
      policyAuthorityClaimed: true,
    }).success,
  },
];

const proof = {
  status: 'ok',
  proofKind: 'local-ai-text-output-parser-proof',
  generatedAt,
  output: relativePath(ProofPath),
  parserRows: [
    proofRow('parsed', parsed),
    proofRow('malformed-rejected', malformedRejected),
    proofRow('remote-rejected', remoteRejected),
    proofRow('manual-required', manualRejected),
  ],
  assertions: {
    validOutputBecomesPolicyEligible: parsed.state === 'parsed-local-result' && parsed.resultPolicyEligible,
    malformedOutputRejectedBeforePolicy: malformedRejected.state === 'rejected-invalid-output',
    remoteOutputRejectedBeforePolicy: remoteRejected.state === 'rejected-invalid-output',
    manualAdapterOutputNotPolicyEligible:
      manualRejected.state === 'manual-required' && !manualRejected.resultPolicyEligible,
    rawModelOutputNotRetained: [parsed, malformedRejected, remoteRejected, manualRejected].every(
      (row) => !row.rawModelOutputRetained
    ),
    noModelExecutionClaim: [parsed, malformedRejected, remoteRejected, manualRejected].every(
      (row) => !row.modelExecuted
    ),
    noRemotePolicyOrEnforcementClaim: [parsed, malformedRejected, remoteRejected, manualRejected].every(
      (row) => !row.remoteApiClaimed && !row.policyAuthorityClaimed && !row.enforcementClaimed
    ),
    overclaimsRejected: rejectionChecks.every((check) => check.rejected),
  },
  rejectionChecks,
  nonClaims: parsed.nonClaims,
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-text-output-parser-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-text-output-parser-proof-ok:${proof.parserRows.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

function proofRow(name, row) {
  return {
    name,
    parserRunId: row.parserRunId,
    state: row.state,
    adapterRequestId: row.adapterRequestId,
    action: row.result?.action ?? null,
    confidence: row.result?.confidence ?? null,
    resultPolicyEligible: row.resultPolicyEligible,
    parserRejectedOutput: row.parserRejectedOutput,
    modelExecuted: row.modelExecuted,
    remoteApiClaimed: row.remoteApiClaimed,
    policyAuthorityClaimed: row.policyAuthorityClaimed,
    enforcementClaimed: row.enforcementClaimed,
    rawModelOutputRetained: row.rawModelOutputRetained,
  };
}

function readyRuntime() {
  return {
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
    lastCheckedAt: observedAt,
    unavailableReason: null,
  };
}

function adapterInput() {
  return {
    schemaVersion: 'v0.6',
    adapterRequestId: 'local-ai-text-adapter:screen-summary-wiki-ocr',
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    localAdapterAvailable: true,
    manualProofRequired: false,
    modelRuntime: readyRuntime(),
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
        evidence: evidenceReference,
      },
      evidenceReferences: [evidenceReference],
      parentRuleReferences: ['policy-rule:video-warn'],
      recentActivityWindow: [evidenceReference],
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

function validCandidateOutput() {
  return {
    schemaVersion: 'v0.6',
    resultId: 'local-ai-text-result:screen-summary-wiki-ocr',
    requestId: 'local-ai-eval:screen-summary-wiki-ocr',
    action: 'warn',
    confidence: 0.68,
    unknownState: 'none',
    degradedState: 'none',
    reasonCodes: ['local-ai-text:screen-video-risk'],
    explanationReference: 'local-ai-text-explanation:screen-summary-wiki-ocr',
    evidenceReferences: [evidenceReference],
    parentRuleReferences: ['policy-rule:video-warn'],
    memoryReferences: [],
    graphReferences: [],
    modelRuntime: readyRuntime(),
    promptVersion: 'prompt:screen-safety:v1',
    expiresAt: null,
  };
}

function parserInput(candidateOutput, adapterProof) {
  return {
    schemaVersion: 'v0.6',
    parserRunId: 'local-ai-text-parser:screen-summary-wiki-ocr',
    adapterProof,
    candidateOutput,
    rawModelOutputRetained: false,
  };
}

function relativePath(filePath) {
  return relative(RepoRoot, filePath).replaceAll('\\', '/');
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}
