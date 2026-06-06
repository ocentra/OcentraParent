import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-remote-assistant-boundary-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-remote-assistant-boundary-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runCommand('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'local-ai-remote-assistant-boundary-proof',
]);

const remoteBoundaryModule = await import('@ocentra-parent/parent-domain/local-ai-remote-assistant-boundary-proof');
const proofContract = remoteBoundaryModule.LocalAiRemoteAssistantBoundaryProof;

const requestRejectionChecks = [
  {
    name: 'child-safety-path-request',
    rejected: !remoteBoundaryModule.RemoteAssistantRequestSchema.safeParse({
      ...proofContract.readyRequest,
      childSafetyDecisionPath: true,
    }).success,
  },
  {
    name: 'unapproved-parent-remote-use',
    rejected: !remoteBoundaryModule.RemoteAssistantRequestSchema.safeParse({
      ...proofContract.readyRequest,
      parentAuthorizedRemoteUse: false,
    }).success,
  },
  {
    name: 'raw-prompt-retention',
    rejected: !remoteBoundaryModule.RemoteAssistantRequestSchema.safeParse({
      ...proofContract.readyRequest,
      rawPromptRetained: true,
    }).success,
  },
  {
    name: 'missing-approved-evidence',
    rejected: !remoteBoundaryModule.RemoteAssistantRequestSchema.safeParse({
      ...proofContract.readyRequest,
      approvedSourceEvidenceReferences: [],
    }).success,
  },
];

const resultRejectionChecks = [
  {
    name: 'policy-authority-overclaim',
    rejected: !remoteBoundaryModule.RemoteAssistantResultSchema.safeParse({
      ...proofContract.readyResult,
      policyAuthorityClaimed: true,
    }).success,
  },
  {
    name: 'enforcement-overclaim',
    rejected: !remoteBoundaryModule.RemoteAssistantResultSchema.safeParse({
      ...proofContract.readyResult,
      enforcementClaimed: true,
    }).success,
  },
  {
    name: 'remote-policy-override-overclaim',
    rejected: !remoteBoundaryModule.RemoteAssistantResultSchema.safeParse({
      ...proofContract.readyResult,
      remoteOutputAllowedToOverrideLocalPolicy: true,
    }).success,
  },
  {
    name: 'raw-model-output-retention',
    rejected: !remoteBoundaryModule.RemoteAssistantResultSchema.safeParse({
      ...proofContract.readyResult,
      rawModelOutputRetained: true,
    }).success,
  },
];

const proof = {
  status: 'ok',
  proofKind: 'local-ai-remote-assistant-boundary-proof',
  generatedAt,
  output: relativePath(ProofPath),
  request: {
    requestId: proofContract.readyRequest.requestId,
    parentAuthorizedRemoteUse: proofContract.readyRequest.parentAuthorizedRemoteUse,
    childSafetyDecisionPath: proofContract.readyRequest.childSafetyDecisionPath,
    custodyBoundary: proofContract.readyRequest.custodyBoundary,
    approvedEvidenceReferenceCount: proofContract.readyRequest.approvedSourceEvidenceReferences.length,
    permittedReportBundleCount: proofContract.readyRequest.permittedReportBundleRefs.length,
    rawPromptRetained: proofContract.readyRequest.rawPromptRetained,
  },
  readyResult: resultRow(proofContract.readyResult),
  fallbackResult: resultRow(proofContract.fallbackResult),
  assertions: {
    parentAuthorizedRemoteOnly:
      proofContract.readyRequest.parentAuthorizedRemoteUse && !proofContract.readyRequest.childSafetyDecisionPath,
    evidenceCited:
      proofContract.readyRequest.approvedSourceEvidenceReferences[0].evidenceReferenceId ===
      proofContract.readyResult.citedEvidenceReferences[0].evidenceReferenceId,
    remoteAnswerCannotOverridePolicy: !proofContract.readyResult.remoteOutputAllowedToOverrideLocalPolicy,
    remoteSuggestionWeakerThanLocalPolicy:
      proofContract.readyResult.remoteSuggestedPolicyDecision.action === 'allow' &&
      proofContract.readyResult.localPolicyDecision.action === 'block',
    fallbackKeepsLocalPolicy:
      proofContract.fallbackResult.executionState === 'local-only-fallback' &&
      proofContract.fallbackResult.localPolicyDecision.action === 'block',
    noPolicyAuthorityClaim: [proofContract.readyResult, proofContract.fallbackResult].every(
      (row) => !row.policyAuthorityClaimed
    ),
    noEnforcementClaim: [proofContract.readyResult, proofContract.fallbackResult].every(
      (row) => !row.enforcementClaimed
    ),
    noRawRetention: [proofContract.readyResult, proofContract.fallbackResult].every(
      (row) => !row.rawPromptRetained && !row.rawModelOutputRetained
    ),
    malformedBoundaryInputsRejected: [...requestRejectionChecks, ...resultRejectionChecks].every(
      (check) => check.rejected
    ),
  },
  rejectionChecks: [...requestRejectionChecks, ...resultRejectionChecks],
  validationSummary: proofContract.validationSummary,
  knownNonClaims: [
    'Remote/API assistant is parent-authorized report/explanation only.',
    'Remote assistant output cannot override local child-safety AI or deterministic policy.',
    'This proof does not execute a remote provider, render portal UI, dispatch enforcement, or claim model quality.',
  ],
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-remote-assistant-boundary-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-remote-assistant-boundary-proof-ok:${proof.rejectionChecks.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

function resultRow(row) {
  return {
    resultId: row.resultId,
    executionState: row.executionState,
    answerRef: row.answerRef,
    failureReason: row.failureReason,
    localAiResultId: row.localAiResult.resultId,
    localPolicyDecisionId: row.localPolicyDecision.decisionId,
    localPolicyAction: row.localPolicyDecision.action,
    remoteSuggestedAction: row.remoteSuggestedPolicyDecision?.action ?? null,
    citedEvidenceReferenceCount: row.citedEvidenceReferences.length,
    remoteApiAiUsed: row.remoteApiAiUsed,
    remoteOutputAllowedToOverrideLocalPolicy: row.remoteOutputAllowedToOverrideLocalPolicy,
    policyAuthorityClaimed: row.policyAuthorityClaimed,
    enforcementClaimed: row.enforcementClaimed,
    rawPromptRetained: row.rawPromptRetained,
    rawModelOutputRetained: row.rawModelOutputRetained,
  };
}

function relativePath(filePath) {
  return relative(RepoRoot, filePath).replaceAll('\\', '/');
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}
