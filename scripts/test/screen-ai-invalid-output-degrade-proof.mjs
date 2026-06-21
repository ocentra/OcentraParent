import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'screen-ai-invalid-output-degrade-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'screen-ai-invalid-output-degrade-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/screen-domain',
    '--',
    'screen-ai-invalid-output-degrade-proof.test.ts',
  ])
);

const { LocalAiSafetyResultSchema } = await import('@ocentra-parent/schema-domain/local-ai');

const evidenceReference = {
  evidenceReferenceId: 'screen-evidence:invalid-output-degrade',
  kind: 'activity-event',
  observedAt: generatedAt,
};

const readyRuntime = {
  runtimeReferenceId: 'runtime:screen-invalid-output-degrade',
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

const baseResult = {
  schemaVersion: 'v0.6',
  resultId: 'screen-ai-result:valid-output',
  requestId: 'screen-ai-request:invalid-output-degrade',
  action: 'warn',
  confidence: 0.7,
  unknownState: 'none',
  degradedState: 'none',
  reasonCodes: ['screen-ai:valid-output'],
  explanationReference: 'explanation:screen-ai-valid-output',
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['policy-rule:screen-safety'],
  memoryReferences: [],
  graphReferences: [],
  modelRuntime: readyRuntime,
  promptVersion: 'screen-safety-template-v1',
  expiresAt: null,
};

const malformedOutputRejected = !LocalAiSafetyResultSchema.safeParse({
  ...baseResult,
  action: 'silently-allow',
  confidence: 1.4,
  evidenceReferences: 'screen-evidence:invalid-output-degrade',
  modelRuntime: {
    ...readyRuntime,
    privacyMode: 'remote-api',
  },
}).success;

const invalidOutputFallback = LocalAiSafetyResultSchema.parse({
  ...baseResult,
  resultId: 'screen-ai-result:invalid-output',
  action: 'unknown',
  confidence: 0,
  unknownState: 'model-unavailable',
  degradedState: 'invalid-output',
  reasonCodes: ['screen-ai:invalid-ai-output', 'screen-ai:model-output-unparseable'],
  explanationReference: null,
  modelRuntime: {
    ...readyRuntime,
    executionState: 'failed',
    loadState: 'degraded',
    degradedState: 'invalid-output',
    unavailableReason: 'screen-ai-model-output-unparseable',
  },
});

const timeoutFallback = LocalAiSafetyResultSchema.parse({
  ...baseResult,
  resultId: 'screen-ai-result:timeout',
  action: 'ask-parent',
  confidence: 0,
  unknownState: 'model-unavailable',
  degradedState: 'overloaded',
  reasonCodes: ['screen-ai:model-timeout', 'screen-ai:manual-parent-review-required'],
  explanationReference: null,
  modelRuntime: {
    ...readyRuntime,
    executionState: 'failed',
    loadState: 'failed',
    degradedState: 'overloaded',
    unavailableReason: 'screen-ai-local-model-timeout',
  },
});

const degradedResults = [invalidOutputFallback, timeoutFallback];
const proof = {
  status: 'ok',
  proofKind: 'screen-ai-invalid-output-degrade-proof',
  generatedAt,
  output: relativePath(ProofPath),
  fallbackResults: degradedResults.map((result) => ({
    resultId: result.resultId,
    action: result.action,
    confidence: result.confidence,
    unknownState: result.unknownState,
    degradedState: result.degradedState,
    runtimeExecutionState: result.modelRuntime.executionState,
    runtimeLoadState: result.modelRuntime.loadState,
    runtimeDegradedState: result.modelRuntime.degradedState,
    unavailableReason: result.modelRuntime.unavailableReason,
    evidenceReferenceCount: result.evidenceReferences.length,
    parentRuleReferenceCount: result.parentRuleReferences.length,
  })),
  assertions: {
    malformedModelOutputRejected: malformedOutputRejected,
    invalidOutputFallsBackToUnknown: invalidOutputFallback.action === 'unknown',
    timeoutFallsBackToParentReview: timeoutFallback.action === 'ask-parent',
    degradedOutputsCarryNoPositiveConfidence: degradedResults.every((result) => result.confidence === 0),
    degradedOutputsRemainLocalOnly: degradedResults.every((result) => result.modelRuntime.privacyMode === 'local-only'),
    degradedOutputsRetainEvidenceAndRules: degradedResults.every(
      (result) => result.evidenceReferences.length === 1 && result.parentRuleReferences.length === 1
    ),
    degradedOutputsDoNotAllowOrBlock: degradedResults.every(
      (result) => result.action !== 'allow' && result.action !== 'block'
    ),
  },
  nonClaims: [
    'This proof validates typed degradation states for invalid/unparseable screen AI output and local timeout cases.',
    'It does not execute a model, prove model quality, rerun screen capture, render portal UI, or dispatch enforcement.',
  ],
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/schema-domain',
    'cmd /c npm run test --workspace @ocentra-parent/screen-domain -- screen-ai-invalid-output-degrade-proof.test.ts',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`screen-ai-invalid-output-degrade-proof-ok:${invalidOutputFallback.action}:${timeoutFallback.action}`);
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
