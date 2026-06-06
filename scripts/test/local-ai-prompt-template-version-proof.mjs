import { execFileSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'ai-plan-proof', 'local-ai-prompt-template-version-proof');
const testResultsDir = join(repoRoot, 'test-results', 'local-ai-prompt-template-version-proof');
const proofSummaryPath = join(outputDir, 'proof-summary.json');
const validationCommandsPath = join(outputDir, 'validation-commands.log');
const testResultPath = join(testResultsDir, 'proof.json');

await main();

async function main() {
  rmSync(outputDir, { recursive: true, force: true });
  rmSync(testResultsDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });
  mkdirSync(testResultsDir, { recursive: true });

  const commands = [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-prompt-template-version-proof',
  ];
  runCommand(commands[0]);
  runCommand(commands[1]);

  const contract = await import(
    pathToFileURL(join(repoRoot, 'packages/parent-domain/dist/local-ai-prompt-template-version-proof.js')).href
  );
  const proof = contract.buildLocalAiPromptTemplateVersionProof(buildProofInput());
  const proofSummary = {
    status: 'ok',
    proofKind: proof.proofKind,
    generatedAt: new Date().toISOString(),
    proof: relativePath(proofSummaryPath),
    testResult: relativePath(testResultPath),
    validationCommands: relativePath(validationCommandsPath),
    promptVersion: proof.promptRecord.promptVersion,
    summary: proof.summary,
    claimBoundaries: proof.claimBoundaries,
    sourceArtifacts: [
      'packages/parent-domain/src/local-ai-primitives.ts',
      'packages/parent-domain/src/local-ai-runtime.ts',
      'packages/parent-domain/src/local-ai.ts',
      'packages/parent-domain/src/local-ai-prompt-template-version-proof.ts',
    ],
    claimsProved: [
      'Local AI prompt/template versions must be active, local-only, and compatible with the selected model/runtime before use.',
      'Prompt/template version records must cite evidence and parent-rule refs used by the local AI request and result.',
      'Raw prompt/template text, remote/API AI, policy authority, UI, enforcement, and model-quality claims are rejected.',
    ],
    nonClaims: [
      'This proof does not execute a model or claim production model quality.',
      'This proof does not render a prompt/template surface in the portal.',
      'This proof does not make policy decisions or dispatch enforcement.',
      'This proof does not create a production prompt registry storage runtime.',
    ],
  };

  writeJson(proofSummaryPath, proofSummary);
  writeJson(testResultPath, proof);
  writeFileSync(validationCommandsPath, `${commands.join('\n')}\n`);
  console.log(`local-ai-prompt-template-version-proof-ok:${relativePath(proofSummaryPath)}`);
}

function buildProofInput() {
  const childProfile = { childProfileId: 'child-prompt-template-proof', displayName: 'Sam' };
  const device = {
    deviceId: 'device-prompt-template-proof',
    childProfileId: 'child-prompt-template-proof',
    label: 'Sam Windows PC',
    platform: 'windows',
  };
  const evidence = {
    evidenceReferenceId: 'evidence-prompt-template-proof',
    kind: 'query-store-summary',
    observedAt: '2026-06-06T08:00:00.000Z',
  };
  const parentRule = 'rule-prompt-template-proof';
  const modelRequest = {
    providerId: 'provider-local-prompt-template-proof',
    modelId: 'model-local-prompt-template-proof',
    promptVersion: 'prompt-template-screen-safety-v1',
  };
  const runtimeStatus = {
    runtimeReferenceId: 'runtime-local-prompt-template-proof',
    providerId: modelRequest.providerId,
    modelId: modelRequest.modelId,
    modelReference: 'model-ref-local-prompt-template-proof',
    privacyMode: 'local-only',
    adapterBoundary: 'local-adapter-ready',
    executionState: 'dry-run-ready',
    providerSource: 'local-model-cache',
    loadState: 'loaded',
    capabilityFlags: ['classification', 'safety-decision'],
    resourceClass: 'cpu',
    degradedState: 'none',
    lastCheckedAt: '2026-06-06T08:01:00.000Z',
    unavailableReason: null,
  };
  const evaluationInput = {
    schemaVersion: 'v0.6',
    requestId: 'request-prompt-template-proof',
    childProfile,
    device,
    currentObservation: {
      contextKind: 'page',
      evidence,
    },
    evidenceReferences: [evidence],
    parentRuleReferences: [parentRule],
    recentActivityWindow: [evidence],
    memoryReferences: [],
    graphReferences: [],
    modelRequest,
  };
  return {
    schemaVersion: 'v0.6',
    evaluationInput,
    safetyResult: {
      schemaVersion: 'v0.6',
      resultId: 'result-prompt-template-proof',
      requestId: evaluationInput.requestId,
      action: 'warn',
      confidence: 0.82,
      unknownState: 'none',
      degradedState: 'none',
      reasonCodes: ['screen-category-video'],
      explanationReference: 'explanation-prompt-template-proof',
      evidenceReferences: [evidence],
      parentRuleReferences: [parentRule],
      memoryReferences: [],
      graphReferences: [],
      modelRuntime: runtimeStatus,
      promptVersion: modelRequest.promptVersion,
      expiresAt: null,
    },
    runtimeStatus,
    promptRecords: [
      {
        schemaVersion: 'v0.6',
        promptVersion: modelRequest.promptVersion,
        lifecycleState: 'active',
        compatibleModelIds: [modelRequest.modelId],
        compatibleRuntimeRefs: [runtimeStatus.runtimeReferenceId],
        taskRequirements: ['classification', 'safety-decision'],
        evidenceReferences: [evidence],
        parentRuleReferences: [parentRule],
        generatedAt: '2026-06-06T07:55:00.000Z',
        validFrom: '2026-06-06T07:55:00.000Z',
        validUntil: null,
        supersededByPromptVersion: null,
        rawPromptRetained: false,
        rawTemplateTextRetained: false,
        remoteAiRequired: false,
      },
    ],
    claimBoundaries: {
      remoteAiUsed: false,
      apiAiUsed: false,
      rawPromptRetained: false,
      rawTemplateTextRetained: false,
      modelQualityClaimed: false,
      policyAuthorityClaimed: false,
      enforcementClaimed: false,
      uiClaimed: false,
    },
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
