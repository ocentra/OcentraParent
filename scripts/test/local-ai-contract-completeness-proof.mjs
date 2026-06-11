import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-contract-completeness-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-contract-completeness-proof');
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
    'local-ai-contract-completeness-proof',
  ])
);

const proofModule = await import(
  pathToFileURL(resolve(RepoRoot, 'packages', 'parent-domain', 'dist', 'local-ai-contract-completeness-proof.js'))
);

const proof = proofModule.LocalAiContractCompletenessProofSchema.parse({
  ...proofModule.LocalAiContractCompletenessProof,
  generatedAt,
});

const proofSummary = {
  status: 'ok',
  proofKind: 'local-ai-contract-completeness-proof',
  generatedAt,
  proof: relativePath(ProofPath),
  sourceContracts: [
    'packages/parent-domain/src/local-ai.ts',
    'packages/parent-domain/src/local-ai-runtime.ts',
    'packages/parent-domain/src/local-ai-provider-scheduler.ts',
    'packages/parent-domain/src/local-ai-references.ts',
  ],
  provedContractKinds: proof.provedContractKinds,
  validationSummary: proof.validationSummary,
  request: {
    requestId: proof.evaluationInput.requestId,
    currentObservationKind: proof.evaluationInput.currentObservation.contextKind,
    evidenceReferenceCount: proof.evaluationInput.evidenceReferences.length,
    parentRuleReferenceCount: proof.evaluationInput.parentRuleReferences.length,
    memoryReferenceCount: proof.evaluationInput.memoryReferences.length,
    graphReferenceCount: proof.evaluationInput.graphReferences.length,
    promptVersion: proof.evaluationInput.modelRequest.promptVersion,
  },
  result: {
    resultId: proof.safetyResult.resultId,
    requestId: proof.safetyResult.requestId,
    action: proof.safetyResult.action,
    confidence: proof.safetyResult.confidence,
    unknownState: proof.safetyResult.unknownState,
    degradedState: proof.safetyResult.degradedState,
    evidenceReferenceCount: proof.safetyResult.evidenceReferences.length,
    parentRuleReferenceCount: proof.safetyResult.parentRuleReferences.length,
    promptVersion: proof.safetyResult.promptVersion,
  },
  provider: {
    providerId: proof.providerCapability.providerId,
    runtimeReferenceId: proof.runtimeStatus.runtimeReferenceId,
    privacyMode: proof.runtimeStatus.privacyMode,
    adapterBoundary: proof.runtimeStatus.adapterBoundary,
    executionState: proof.runtimeStatus.executionState,
    providerSource: proof.runtimeStatus.providerSource,
    capabilityFlags: proof.providerCapability.supportedTasks,
    selectedRuntimeReferenceId: proof.routeDecision.selectedRuntimeReferenceId,
    jobClass: proof.routeDecision.jobClass,
    jobStatus: proof.routeDecision.jobStatus,
    queuePosition: proof.routeDecision.queuePosition,
    duplicateRuntimeBlocked: proof.routeDecision.duplicateRuntimeBlocked,
  },
  assertions: {
    allContractKindsPresent: proof.provedContractKinds.length === 5,
    resultMatchesInputRequest: proof.safetyResult.requestId === proof.evaluationInput.requestId,
    routeTargetsRuntime: proof.routeDecision.selectedRuntimeReferenceId === proof.runtimeStatus.runtimeReferenceId,
    queueTargetsRuntime: proof.queueStatus.runtimeReferenceId === proof.runtimeStatus.runtimeReferenceId,
    providerCapabilityTargetsRuntime: proof.providerCapability.providerId === proof.runtimeStatus.providerId,
    evidenceAndRulesCited:
      proof.evaluationInput.evidenceReferences.length > 0 &&
      proof.evaluationInput.parentRuleReferences.length > 0 &&
      proof.safetyResult.evidenceReferences.length > 0 &&
      proof.safetyResult.parentRuleReferences.length > 0,
    memoryAndGraphCited:
      proof.safetyResult.memoryReferences.length > 0 && proof.safetyResult.graphReferences.length > 0,
    localOnlyRuntime: proof.safetyResult.modelRuntime.privacyMode === 'local-only',
    noRuntimeUiEnforcementOverclaims: Object.values(proof.claimBoundaries).every((value) => value === false),
  },
  claimBoundaries: proof.claimBoundaries,
  nonClaims: [
    'This proof does not execute a model or prove production model quality.',
    'This proof does not render portal UI or make a parent-facing UX claim.',
    'This proof does not make policy decisions or dispatch enforcement.',
    'This proof does not use remote/API AI, retain raw prompts, or retain raw evidence.',
  ],
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proofSummary, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-contract-completeness-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-contract-completeness-proof-ok:${proof.provedContractKinds.length}`);
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
