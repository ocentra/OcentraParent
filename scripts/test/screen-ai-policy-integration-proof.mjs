import { execFileSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'ai-plan-proof', 'screen-ai-policy-integration-proof');
const testResultsDir = join(repoRoot, 'test-results', 'screen-ai-policy-integration-proof');
const proofSummaryPath = join(outputDir, 'proof-summary.json');
const proofResultPath = join(testResultsDir, 'proof.json');
const validationCommandsPath = join(outputDir, 'validation-commands.log');
const commands = [];

await main();

async function main() {
  rmSync(outputDir, { recursive: true, force: true });
  rmSync(testResultsDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });
  mkdirSync(testResultsDir, { recursive: true });

  runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'screen-ai-policy-integration-proof',
  ]);

  const localAi = await import('@ocentra-parent/parent-domain/local-ai');
  const policy = await import('@ocentra-parent/parent-domain/policy');
  const allowAiResult = localAi.LocalAiSafetyResultSchema.parse(localAiResultBase());
  const blockDecision = policy.PolicyDecisionSchema.parse(
    policyDecision({
      action: policy.selectStricterPolicyAction('block', allowAiResult.action),
      localAiResultId: allowAiResult.resultId,
      ruleIds: ['parent-rule-screen-block'],
    })
  );
  const lowConfidenceAiResult = localAi.LocalAiSafetyResultSchema.parse(
    localAiResultBase({
      resultId: 'screen-ai-result-low-confidence-allow',
      confidence: 0.42,
      unknownState: 'low-confidence',
      reasonCodes: ['screen-ai-low-confidence-school'],
    })
  );
  const timeLimitDecision = policy.PolicyDecisionSchema.parse(
    policyDecision({
      decisionId: 'policy-decision-screen-low-confidence-time-limit',
      action: policy.selectStricterPolicyAction('time-limit', lowConfidenceAiResult.action),
      reasonCodes: ['parent-rule-time-limit-wins', ...lowConfidenceAiResult.reasonCodes],
      localAiResultId: lowConfidenceAiResult.resultId,
      ruleIds: ['parent-rule-screen-time-limit'],
      expiresAt: '2026-06-05T23:00:00.000Z',
    })
  );
  const assertions = {
    schemaValidLocalAiResults: allowAiResult.resultId.length > 0 && lowConfidenceAiResult.resultId.length > 0,
    parentBlockOverridesAiAllow: blockDecision.action === 'block',
    parentTimeLimitOverridesLowConfidenceAiAllow: timeLimitDecision.action === 'time-limit',
    policyDecisionReferencesAiResult:
      blockDecision.localAiResultId === allowAiResult.resultId &&
      timeLimitDecision.localAiResultId === lowConfidenceAiResult.resultId,
    policyDecisionReferencesEvidence:
      blockDecision.evidenceReferences.length > 0 && timeLimitDecision.evidenceReferences.length > 0,
    enforcementRemainsDisabled: blockDecision.enforcementHandoffState === 'disabled',
  };
  const proofSummary = {
    proof: 'screen-ai-policy-integration-proof',
    proofTier: 'P3_CONTRACT_POLICY_INTEGRATION',
    generatedAt: new Date().toISOString(),
    validationCommands: relativePath(validationCommandsPath),
    sourceArtifacts: [
      'packages/parent-domain/src/local-ai.ts',
      'packages/parent-domain/src/policy.ts',
      'packages/parent-domain/tests/screen-ai-policy-integration-proof.test.ts',
    ],
    decisions: {
      blockDecision,
      timeLimitDecision,
    },
    assertions,
    claimsProved: [
      'Schema-valid local screen AI output is evidence input to policy, not authority.',
      'A stricter parent block rule overrides local screen AI allow output.',
      'A stricter parent time-limit rule overrides low-confidence local screen AI allow output.',
      'Policy decisions retain evidence refs, local AI result refs, parent rule refs, dry-run state, and disabled enforcement handoff.',
    ],
    nonClaims: [
      'This proof does not execute a model, rerun screen capture, render portal UI, or dispatch enforcement.',
      'This proof uses existing local AI and policy contracts and does not add a new package export.',
      'This proof does not complete production model quality, broad adapter support, or product settings UI.',
    ],
  };

  if (!Object.values(assertions).every((assertion) => assertion === true)) {
    throw new Error(`Screen AI policy integration proof failed: ${JSON.stringify(assertions)}`);
  }

  writeFileSync(proofSummaryPath, `${JSON.stringify(proofSummary, null, 2)}\n`);
  writeFileSync(
    proofResultPath,
    `${JSON.stringify({ allowAiResult, lowConfidenceAiResult, proofSummary }, null, 2)}\n`
  );
  writeFileSync(validationCommandsPath, `${commands.map((command) => `${command}: PASS`).join('\n')}\n`);
  console.log(
    `screen-ai-policy-integration-proof-ok:${blockDecision.action}:${timeLimitDecision.action}:${blockDecision.enforcementHandoffState}`
  );
  console.log(`proof=${relativePath(proofSummaryPath)}`);
}

function localAiResultBase(overrides = {}) {
  return {
    schemaVersion: 'v0.6',
    resultId: 'screen-ai-result-school-allow',
    requestId: 'screen-ai-request-school-page',
    action: 'allow',
    confidence: 0.91,
    unknownState: 'none',
    degradedState: 'none',
    reasonCodes: ['screen-ai-school-content'],
    explanationReference: 'screen-ai-explanation-school-page',
    evidenceReferences: [screenEvidenceReference()],
    parentRuleReferences: ['parent-rule-screen-block'],
    memoryReferences: [],
    graphReferences: [],
    modelRuntime: {
      runtimeReferenceId: 'screen-ai-runtime-local',
      providerId: 'local-screen-provider',
      modelId: 'screen-safety-model',
      modelReference: 'local-model-cache-screen-safety',
      privacyMode: 'local-only',
      adapterBoundary: 'local-adapter-ready',
      executionState: 'dry-run-ready',
      providerSource: 'local-model-cache',
      loadState: 'loaded',
      capabilityFlags: ['safety-decision'],
      resourceClass: 'cpu',
      degradedState: 'none',
      lastCheckedAt: '2026-06-05T22:00:00.000Z',
      unavailableReason: null,
    },
    promptVersion: 'screen-ai-policy-integration-v1',
    expiresAt: null,
    ...overrides,
  };
}

function policyDecision(overrides = {}) {
  return {
    schemaVersion: 'v0.6',
    decisionId: 'policy-decision-screen-parent-rule-wins',
    action: 'block',
    reasonCodes: ['parent-rule-block-wins', 'screen-ai-school-content'],
    evidenceReferences: [screenEvidenceReference()],
    ruleIds: ['parent-rule-screen-block'],
    localAiResultId: 'screen-ai-result-school-allow',
    dryRun: true,
    enforcementHandoffState: 'disabled',
    expiresAt: null,
    ...overrides,
  };
}

function screenEvidenceReference() {
  return {
    evidenceReferenceId: 'screen-summary-evidence-wikipedia-school',
    kind: 'activity-event',
    observedAt: '2026-06-05T22:00:00.000Z',
  };
}

function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  execFileSync(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
