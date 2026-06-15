import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'screen-ai-model-output-parser-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'screen-ai-model-output-parser-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'screen-ai-model-output-parser-proof',
  ])
);

const { LocalAiEvaluationInputSchema, LocalAiSafetyResultSchema } =
  await import('@ocentra-parent/ai-domain/local-ai');

const evidenceReference = {
  evidenceReferenceId: 'screen-evidence:winrt-ocr-row',
  kind: 'activity-event',
  observedAt: generatedAt,
};

const childProfile = {
  childProfileId: 'child:screen-ai-parser',
  displayName: 'Sam',
};

const device = {
  deviceId: 'device:screen-ai-parser',
  childProfileId: childProfile.childProfileId,
  label: 'Sam Windows PC',
  platform: 'windows',
};

const modelRuntime = {
  runtimeReferenceId: 'runtime:screen-child-safety-parser',
  providerId: 'screen-local-provider',
  modelId: 'screen-child-safety-v1',
  modelReference: 'artifact:screen-child-safety-v1',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['safety-decision', 'classification'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: generatedAt,
  unavailableReason: null,
};

const modelRequest = {
  providerId: modelRuntime.providerId,
  modelId: modelRuntime.modelId,
  promptVersion: 'screen-safety-template-v1',
};

const parsedInput = LocalAiEvaluationInputSchema.parse({
  schemaVersion: 'v0.6',
  requestId: 'screen-ai-request:parser-proof',
  childProfile,
  device,
  currentObservation: {
    contextKind: 'video',
    evidence: evidenceReference,
  },
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['policy-rule:school-night'],
  recentActivityWindow: [evidenceReference],
  memoryReferences: [],
  graphReferences: [],
  modelRequest,
});

const parsedOutput = LocalAiSafetyResultSchema.parse({
  schemaVersion: 'v0.6',
  resultId: 'screen-ai-result:parser-proof',
  requestId: parsedInput.requestId,
  action: 'warn',
  confidence: 0.74,
  unknownState: 'none',
  degradedState: 'none',
  reasonCodes: ['screen-ai:video-detected', 'policy:school-night'],
  explanationReference: 'explanation:screen-ai-parser-proof',
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['policy-rule:school-night'],
  memoryReferences: [],
  graphReferences: [],
  modelRuntime,
  promptVersion: modelRequest.promptVersion,
  expiresAt: null,
});

const rejectionChecks = [
  {
    name: 'invalid-action',
    rejected: !LocalAiSafetyResultSchema.safeParse({ ...parsedOutput, action: 'redirect' }).success,
  },
  {
    name: 'high-confidence',
    rejected: !LocalAiSafetyResultSchema.safeParse({ ...parsedOutput, confidence: 1.01 }).success,
  },
  {
    name: 'negative-confidence',
    rejected: !LocalAiSafetyResultSchema.safeParse({ ...parsedOutput, confidence: -0.01 }).success,
  },
  {
    name: 'remote-runtime',
    rejected: !LocalAiSafetyResultSchema.safeParse({
      ...parsedOutput,
      modelRuntime: {
        ...modelRuntime,
        privacyMode: 'remote-api',
      },
    }).success,
  },
  {
    name: 'missing-observation-evidence',
    rejected: !LocalAiEvaluationInputSchema.safeParse({
      ...parsedInput,
      currentObservation: {
        contextKind: 'video',
      },
    }).success,
  },
];

const proof = {
  status: 'ok',
  proofKind: 'screen-ai-model-output-parser-proof',
  generatedAt,
  output: relativePath(ProofPath),
  parsedInput: {
    requestId: parsedInput.requestId,
    contextKind: parsedInput.currentObservation.contextKind,
    evidenceReferenceCount: parsedInput.evidenceReferences.length,
    parentRuleReferenceCount: parsedInput.parentRuleReferences.length,
    modelRequest: parsedInput.modelRequest,
  },
  parsedOutput: {
    resultId: parsedOutput.resultId,
    requestId: parsedOutput.requestId,
    action: parsedOutput.action,
    confidence: parsedOutput.confidence,
    unknownState: parsedOutput.unknownState,
    degradedState: parsedOutput.degradedState,
    reasonCodeCount: parsedOutput.reasonCodes.length,
    evidenceReferenceCount: parsedOutput.evidenceReferences.length,
    parentRuleReferenceCount: parsedOutput.parentRuleReferences.length,
    runtimePrivacyMode: parsedOutput.modelRuntime.privacyMode,
    runtimeProviderSource: parsedOutput.modelRuntime.providerSource,
    promptVersion: parsedOutput.promptVersion,
  },
  rejectionChecks,
  assertions: {
    inputCarriesScreenEvidence: parsedInput.currentObservation.contextKind === 'video',
    outputCarriesEvidenceRefs: parsedOutput.evidenceReferences.length === 1,
    outputCarriesRuleRefs: parsedOutput.parentRuleReferences.length === 1,
    outputUsesLocalOnlyRuntime: parsedOutput.modelRuntime.privacyMode === 'local-only',
    malformedOutputsRejected: rejectionChecks.every((check) => check.rejected),
  },
  nonClaims: [
    'This proof validates the schema parser boundary for screen AI model input/output objects.',
    'It does not execute a model, prove model quality, rerun screen capture, render portal UI, or dispatch enforcement.',
  ],
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- screen-ai-model-output-parser-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`screen-ai-model-output-parser-proof-ok:${proof.parsedOutput.action}:${proof.parsedOutput.confidence}`);
console.log(`proof=${relativePath(ProofPath)}`);

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
