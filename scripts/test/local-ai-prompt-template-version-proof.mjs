import { execFileSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'ai-plan-proof', 'local-ai-prompt-template-version-proof');
const testResultsDir = join(repoRoot, 'test-results', 'local-ai-prompt-template-version-proof');
const proofSummaryPath = join(outputDir, 'proof-summary.json');
const validationCommandsPath = join(outputDir, 'validation-commands.log');
const testResultPath = join(testResultsDir, 'proof.json');
const observedAt = '2026-06-06T09:18:00.000Z';
const promptVersion = 'local-ai-safety-template-v1';

const childProfile = { childProfileId: 'child-prompt-template', displayName: 'Sam' };
const device = {
  deviceId: 'device-prompt-template',
  childProfileId: 'child-prompt-template',
  label: 'Sam Windows PC',
  platform: 'windows',
};
const evidenceReference = {
  evidenceReferenceId: 'prompt-template-source-evidence',
  kind: 'journal-event',
  observedAt,
};
const runtimeStatus = {
  runtimeReferenceId: 'runtime-prompt-template',
  providerId: 'local-provider-prompt-template',
  modelId: 'local-model-prompt-template',
  modelReference: 'artifact:local_prompt_template_model',
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
};

await main();

async function main() {
  rmSync(outputDir, { recursive: true, force: true });
  rmSync(testResultsDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });
  mkdirSync(testResultsDir, { recursive: true });

  const commands = [
    'cmd /c npm run build:contracts',
    'cmd /c npm run test --workspace @ocentra-parent/ai-domain -- local-ai-prompt-template-version-proof',
  ];
  runCommand(commands[0]);
  runCommand(commands[1]);

  const contract = await import('@ocentra-parent/schema-domain/local-ai-prompt-template-version-proof');
  const proof = contract.buildLocalAiPromptTemplateVersionProof(buildProofInput());
  const proofSummary = {
    status: 'ok',
    proofKind: proof.proofKind,
    generatedAt: new Date().toISOString(),
    proof: relativePath(proofSummaryPath),
    testResult: relativePath(testResultPath),
    validationCommands: relativePath(validationCommandsPath),
    promptVersion: proof.evaluationInput.modelRequest.promptVersion,
    selectedBindings: proof.selectedTemplateRows.map((row) => row.inputBinding),
    summary: proof.summary,
    claimBoundaries: proof.claimBoundaries,
    sourceArtifacts: [
      'packages/schema-domain/src/ai-primitives.ts',
      'packages/schema-domain/src/local-ai.ts',
      'packages/schema-domain/src/local-ai-context-builder.ts',
      'packages/schema-domain/src/local-ai-context-result.ts',
      'packages/schema-domain/src/local-ai-prompt-template-version-proof.ts',
    ],
    claimsProved: [
      'The prompt/template version is schema-bound and reconciled across context-builder request, local AI evaluation input, and safety result.',
      'Selected prompt/template rows preserve provider, model, task, input binding, and output schema refs for context-builder, evaluation-input, and safety-result boundaries.',
      'Prompt/template proof rejects raw prompt retention, raw model output retention, remote/API AI, policy authority, enforcement, portal UI, model execution, and production model-quality claims.',
    ],
    nonClaims: [
      'This proof does not execute a model or prove production model quality.',
      'This proof does not render a portal UI or dispatch enforcement.',
      'This proof does not store raw prompt text or raw model output.',
      'This proof does not update the product capability checklist because this lane did not lock that shared file.',
    ],
  };

  writeJson(proofSummaryPath, proofSummary);
  writeJson(testResultPath, proof);
  writeFileSync(validationCommandsPath, `${commands.join('\n')}\n`);
  console.log(`local-ai-prompt-template-version-proof-ok:${relativePath(proofSummaryPath)}`);
}

function buildProofInput() {
  return {
    schemaVersion: 'v0.6',
    contextRequest: {
      schemaVersion: 'v0.6',
      requestId: 'prompt-template-request',
      requestedAt: observedAt,
      childProfile,
      device,
      requestedEvaluationKind: 'mixed-context',
      requiredEvidenceKinds: ['screen-summary'],
      parentRuleContextReferences: [],
      modelTaskRequirements: ['safety-decision'],
      allowedCustody: ['child-device-query-store'],
      promptVersion,
    },
    evaluationInput: {
      schemaVersion: 'v0.6',
      requestId: 'prompt-template-request',
      childProfile,
      device,
      currentObservation: { contextKind: 'recent-activity', evidence: evidenceReference },
      evidenceReferences: [evidenceReference],
      parentRuleReferences: ['prompt-template-policy-rule'],
      recentActivityWindow: [evidenceReference],
      memoryReferences: [],
      graphReferences: [],
      modelRequest: {
        providerId: runtimeStatus.providerId,
        modelId: runtimeStatus.modelId,
        promptVersion,
      },
    },
    safetyResult: {
      schemaVersion: 'v0.6',
      resultId: 'prompt-template-result',
      requestId: 'prompt-template-request',
      action: 'warn',
      confidence: 0.66,
      unknownState: 'none',
      degradedState: 'none',
      reasonCodes: ['local-ai-text-dry-run-candidate'],
      explanationReference: 'prompt-template-explanation',
      evidenceReferences: [evidenceReference],
      parentRuleReferences: ['prompt-template-policy-rule'],
      memoryReferences: [],
      graphReferences: [],
      modelRuntime: runtimeStatus,
      promptVersion,
      expiresAt: null,
    },
    templateRows: [
      templateRow('context-builder'),
      templateRow('evaluation-input'),
      templateRow('safety-result'),
      { ...templateRow('context-builder'), templateRef: 'prompt-template:inactive-old', active: false },
    ],
    claimBoundaries: {
      modelExecutionClaimed: false,
      modelQualityClaimed: false,
      policyAuthorityClaimed: false,
      enforcementClaimed: false,
      portalUiClaimed: false,
      remoteApiAiUsed: false,
      rawPromptRetained: false,
      rawModelOutputRetained: false,
    },
  };
}

function templateRow(inputBinding) {
  return {
    templateRef: `prompt-template:${inputBinding}`,
    promptVersion,
    providerId: runtimeStatus.providerId,
    modelId: runtimeStatus.modelId,
    task: 'safety-decision',
    inputBinding,
    outputSchemaRef: `schema:${inputBinding}`,
    active: true,
    rawPromptRetained: false,
    rawModelOutputRetained: false,
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
