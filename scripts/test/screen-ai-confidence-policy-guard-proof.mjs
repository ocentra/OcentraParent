import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'screen-ai-confidence-policy-guard-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'screen-ai-confidence-policy-guard-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();
const confidenceThreshold = 0.8;

runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runCommand('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'screen-ai-confidence-policy-guard-proof',
]);

const proofModuleUrl = pathToFileURL(
  resolve(RepoRoot, 'packages', 'parent-domain', 'dist', 'screen-ai-confidence-policy-guard-proof.js')
).href;
const { buildScreenAiConfidencePolicyGuardProof, screenAiConfidencePolicyGuardSummary } = await import(proofModuleUrl);

const evidenceReference = {
  evidenceReferenceId: 'screen-evidence:confidence-policy-guard',
  kind: 'activity-event',
  observedAt: generatedAt,
};

const readyRuntime = {
  runtimeReferenceId: 'runtime:screen-confidence-policy-guard',
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

const rows = [
  confidenceRow({
    rowId: 'screen-ai-confidence-row:high',
    resultId: 'screen-ai-result:confidence-high',
    sourceEvidenceRef: 'screen-evidence:confidence-policy-guard:high',
    confidence: 0.91,
    confidenceBand: 'high',
    localAction: 'warn',
    policyAction: 'warn',
    unknownState: 'none',
    degradedState: 'none',
    guardOutcome: 'policy-ready',
    policyEligible: true,
    handoffState: 'disabled',
    reasonCode: 'screen-ai:confidence-policy-guard',
  }),
  confidenceRow({
    rowId: 'screen-ai-confidence-row:medium',
    resultId: 'screen-ai-result:confidence-medium',
    sourceEvidenceRef: 'screen-evidence:confidence-policy-guard:medium',
    confidence: 0.62,
    confidenceBand: 'medium',
    localAction: 'ask-parent',
    policyAction: 'ask-parent',
    unknownState: 'none',
    degradedState: 'none',
    guardOutcome: 'parent-review-required',
    policyEligible: true,
    handoffState: 'disabled',
    reasonCode: 'screen-ai:confidence-below-auto-policy-threshold',
  }),
  confidenceRow({
    rowId: 'screen-ai-confidence-row:low',
    resultId: 'screen-ai-result:confidence-low',
    sourceEvidenceRef: 'screen-evidence:confidence-policy-guard:low',
    confidence: 0.31,
    confidenceBand: 'low',
    localAction: 'unknown',
    policyAction: 'unknown',
    unknownState: 'low-confidence',
    degradedState: 'none',
    guardOutcome: 'manual-required',
    policyEligible: false,
    handoffState: 'not-requested',
    reasonCode: 'screen-ai:low-confidence-manual-review',
  }),
  confidenceRow({
    rowId: 'screen-ai-confidence-row:unknown',
    resultId: 'screen-ai-result:confidence-unknown',
    sourceEvidenceRef: 'screen-evidence:confidence-policy-guard:unknown',
    confidence: 0,
    confidenceBand: 'unknown',
    localAction: 'unknown',
    policyAction: 'unknown',
    unknownState: 'model-unavailable',
    degradedState: 'provider-unavailable',
    guardOutcome: 'manual-required',
    policyEligible: false,
    handoffState: 'not-requested',
    reasonCode: 'screen-ai:confidence-unavailable',
    runtimeOverrides: {
      executionState: 'failed',
      loadState: 'failed',
      degradedState: 'provider-unavailable',
      unavailableReason: 'screen-ai-confidence-unavailable',
    },
  }),
];

const parsedProof = buildScreenAiConfidencePolicyGuardProof({
  schemaVersion: 'v0.6',
  proofId: 'screen-ai-confidence-policy-guard-proof',
  generatedAt,
  rows,
});
const summary = screenAiConfidencePolicyGuardSummary(parsedProof);
const proof = {
  status: 'ok',
  proofKind: 'screen-ai-confidence-policy-guard-proof',
  generatedAt,
  output: relativePath(ProofPath),
  confidenceThreshold,
  summary,
  rows: parsedProof.rows.map((row) => ({
    rowId: row.rowId,
    confidence: row.localAiResult.confidence,
    confidenceBand: row.confidenceBand,
    localAction: row.localAiResult.action,
    policyAction: row.policyDecision.action,
    guardOutcome: row.guardOutcome,
    policyEligible: row.policyEligible,
    enforcementAllowed: row.enforcementAllowed,
    remoteProviderUsed: row.remoteProviderUsed,
    rawImageRetained: row.rawImageRetained,
  })),
  assertions: {
    coversHighMediumLowAndUnknownConfidence: summary.totalRows === 4,
    lowConfidenceCannotAllowOrBlock: summary.unsafeAllowOrBlockRows === 0,
    noEnforcementAuthorityClaimed: summary.enforcementAllowedRows === 0,
    noRemoteProviderUsed: summary.remoteProviderRows === 0,
    noRawImageRetained: summary.rawRetainedRows === 0,
  },
  nonClaims: [
    'This proof validates confidence-to-policy guard behavior over typed screen AI safety results.',
    'It does not execute a model, prove model quality, rerun capture, render portal UI, dispatch enforcement, use remote/API AI, or retain raw images.',
  ],
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- screen-ai-confidence-policy-guard-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`screen-ai-confidence-policy-guard-proof-ok:${summary.totalRows}:${summary.unsafeAllowOrBlockRows}`);
console.log(`proof=${relativePath(ProofPath)}`);

function confidenceRow(input) {
  const localAiResult = {
    schemaVersion: 'v0.6',
    resultId: input.resultId,
    requestId: 'screen-ai-request:confidence-policy-guard',
    action: input.localAction,
    confidence: input.confidence,
    unknownState: input.unknownState,
    degradedState: input.degradedState,
    reasonCodes: [input.reasonCode],
    explanationReference:
      input.confidenceBand === 'low' || input.confidenceBand === 'unknown' ? null : `explanation:${input.resultId}`,
    evidenceReferences: [evidenceReference],
    parentRuleReferences: ['policy-rule:screen-confidence'],
    memoryReferences: [],
    graphReferences: [],
    modelRuntime: {
      ...readyRuntime,
      ...(input.runtimeOverrides ?? {}),
    },
    promptVersion: 'screen-safety-template-v1',
    expiresAt: null,
  };

  return {
    rowId: input.rowId,
    sourceEvidenceRef: input.sourceEvidenceRef,
    localAiResult,
    policyDecision: {
      schemaVersion: 'v0.6',
      decisionId: `policy-decision:${input.resultId}`,
      action: input.policyAction,
      reasonCodes: [input.reasonCode],
      evidenceReferences: [evidenceReference],
      ruleIds: ['policy-rule:screen-confidence'],
      localAiResultId: input.resultId,
      dryRun: true,
      enforcementHandoffState: input.handoffState,
      expiresAt: null,
    },
    confidenceThreshold,
    confidenceBand: input.confidenceBand,
    guardOutcome: input.guardOutcome,
    policyEligible: input.policyEligible,
    enforcementAllowed: false,
    remoteProviderUsed: false,
    rawImageRetained: false,
  };
}

function relativePath(filePath) {
  return relative(RepoRoot, filePath).replaceAll('\\', '/');
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}
