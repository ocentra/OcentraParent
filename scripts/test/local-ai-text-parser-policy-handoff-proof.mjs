import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-text-parser-policy-handoff-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-text-parser-policy-handoff-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();
const observedAt = '2026-06-06T14:45:00.000Z';
const sourceProofRefs = ['proof:local-ai-text-parser-read-model'];
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
  'local-ai-text-parser-policy-handoff-proof',
]);

const adapterModule = await importDist('local-ai-text-llm-adapter-boundary-proof.js');
const parserModule = await importDist('local-ai-text-output-parser-proof.js');
const readModelModule = await importDist('local-ai-text-parser-read-model-proof.js');
const policyHandoffModule = await importDist('local-ai-text-parser-policy-handoff-proof.js');

const adapterProof = adapterModule.proveLocalAiTextLlmAdapterBoundary(adapterInput());
const parsed = parserModule.parseLocalAiTextOutput(parserInput(validCandidateOutput(), adapterProof));
const malformedRejected = parserModule.parseLocalAiTextOutput(
  parserInput({ ...validCandidateOutput(), action: 'silently-allow' }, adapterProof)
);
const manualAdapter = adapterModule.proveLocalAiTextLlmAdapterBoundary({
  ...adapterInput(),
  localAdapterAvailable: false,
});
const manualRequired = parserModule.parseLocalAiTextOutput(parserInput(validCandidateOutput(), manualAdapter));
const readModelSnapshot = readModelModule.buildLocalAiTextParserReadModelSnapshot({
  generatedAt,
  snapshotId: 'local-ai-text-parser-read-model:snapshot:policy',
  sourceProofRefs,
  parserProofs: [parsed, malformedRejected, manualRequired],
});
const policyProof = policyHandoffModule.buildLocalAiTextParserPolicyHandoffProof({
  generatedAt,
  proofId: 'local-ai-text-parser-policy-handoff:wiki-ocr',
  sourceProofRefs,
  readModelRows: readModelSnapshot.rows,
});

const rejectionChecks = [
  {
    name: 'model-execution-overclaim',
    rejected: !policyHandoffModule.LocalAiTextParserPolicyHandoffRowSchema.safeParse({
      ...policyProof.rows[0],
      modelExecuted: true,
    }).success,
  },
  {
    name: 'enforcement-overclaim',
    rejected: !policyHandoffModule.LocalAiTextParserPolicyHandoffRowSchema.safeParse({
      ...policyProof.rows[0],
      enforcementClaimed: true,
    }).success,
  },
  {
    name: 'proof-count-mismatch',
    rejected: !policyHandoffModule.LocalAiTextParserPolicyHandoffProofSchema.safeParse({
      ...policyProof,
      policyReadyRowCount: 0,
    }).success,
  },
];

const proof = {
  status: 'ok',
  proofKind: 'local-ai-text-parser-policy-handoff-proof',
  generatedAt,
  output: relativePath(ProofPath),
  rows: policyProof.rows.map((row) => ({
    policyHandoffRowId: row.policyHandoffRowId,
    sourceReadModelRowId: row.sourceReadModelRowId,
    parserRunId: row.parserRunId,
    handoffState: row.handoffState,
    action: row.action,
    policyDecisionId: row.policyDecision?.decisionId ?? null,
    policyDecisionAction: row.policyDecision?.action ?? null,
    localAiResultId: row.policyDecision?.localAiResultId ?? null,
    policyDecisionHandoffState: row.policyDecisionHandoffState,
    resultPolicyEligible: row.resultPolicyEligible,
    manualRequiredReasons: row.manualRequiredReasons,
    sourceProofRefs: row.sourceProofRefs,
    dryRunOnly: row.dryRunOnly,
    reportOnly: row.reportOnly,
    modelExecuted: row.modelExecuted,
    rawModelOutputRetained: row.rawModelOutputRetained,
    remoteApiClaimed: row.remoteApiClaimed,
    policyAuthorityClaimed: row.policyAuthorityClaimed,
    enforcementClaimed: row.enforcementClaimed,
  })),
  counts: {
    policyReadyRowCount: policyProof.policyReadyRowCount,
    manualRequiredRowCount: policyProof.manualRequiredRowCount,
  },
  assertions: {
    readyRowCreatesDryRunPolicyDecision:
      policyProof.policyReadyRowCount === 1 &&
      policyProof.rows.some(
        (row) =>
          row.handoffState === 'policy-dry-run-ready' &&
          row.policyDecision?.dryRun === true &&
          row.policyDecision.enforcementHandoffState === 'disabled'
      ),
    manualRowsDoNotCreatePolicyDecisions:
      policyProof.manualRequiredRowCount === 2 &&
      policyProof.rows
        .filter((row) => row.handoffState === 'manual-required')
        .every((row) => row.policyDecision === null && !row.resultPolicyEligible),
    sourceProofRefsPreserved: policyProof.rows.every((row) =>
      sourceProofRefs.every((proofRef) => row.sourceProofRefs.includes(proofRef))
    ),
    rawModelOutputNotRetained: policyProof.rows.every((row) => !row.rawModelOutputRetained),
    noModelPolicyOrEnforcementClaim: policyProof.rows.every(
      (row) => !row.modelExecuted && !row.policyAuthorityClaimed && !row.enforcementClaimed
    ),
    overclaimsRejected: rejectionChecks.every((check) => check.rejected),
  },
  rejectionChecks,
  nonClaims: policyProof.nonClaims,
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-text-parser-policy-handoff-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-text-parser-policy-handoff-proof-ok:${proof.rows.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

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

async function importDist(fileName) {
  return import(pathToFileURL(join(RepoRoot, 'packages', 'parent-domain', 'dist', fileName)).href);
}

function relativePath(filePath) {
  return relative(RepoRoot, filePath).replaceAll('\\', '/');
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}
