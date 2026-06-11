import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'ai-plan-proof', 'household-ai-provider-claim-lease-proof');
const testResultRoot = resolve(repoRoot, 'test-results', 'household-ai-provider-claim-lease-proof');
const proofPath = join(outputRoot, 'proof-summary.json');
const validationLogPath = join(outputRoot, 'validation-commands.log');
const testResultPath = join(testResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'household-ai-provider-claim-lease-proof',
  ])
);

const proofModule = await import(
  pathToFileURL(resolve(repoRoot, 'packages', 'parent-domain', 'dist', 'household-ai-provider-claim-lease-proof.js'))
);

const proof = proofModule.HouseholdAiProviderClaimLeaseProofSchema.parse({
  ...proofModule.HouseholdAiProviderClaimLeaseProof,
  generatedAt,
});

const proofSummary = {
  status: 'ok',
  proofKind: 'household-ai-provider-claim-lease-proof',
  generatedAt,
  proof: relativePath(proofPath),
  sourceContracts: ['packages/parent-domain/src/household-ai-provider-claim-lease-proof.ts'],
  job: {
    jobId: proof.jobId,
    workKind: proof.workKind,
    childAgentId: proof.childAgentId,
    custodyRef: proof.custodyRef,
    redactedPayloadRef: proof.redactedPayloadRef,
    maxAttempts: proof.maxAttempts,
    leaseTtlMs: proof.leaseTtlMs,
  },
  leaseStates: proof.leaseAttempts.map((row) => ({
    state: row.state,
    providerId: row.providerId,
    acceptedClaim: row.acceptedClaim,
    activeLeaseCountAfterDecision: row.activeLeaseCountAfterDecision,
    rejectionReason: row.rejectionReason,
  })),
  messageStates: proof.messageReceipts.map((row) => ({
    state: row.state,
    messageId: row.messageId,
    sideEffectApplied: row.sideEffectApplied,
  })),
  validationSummary: proof.validationSummary,
  assertions: {
    oneLeasePerJob:
      proof.validationSummary.maxActiveLeaseCount === 1 &&
      proof.leaseAttempts.every((row) => row.activeLeaseCountAfterDecision <= 1),
    duplicateClaimRejected: proof.validationSummary.duplicateClaimRejectedCount === 1,
    leaseExpiryRequeued: proof.validationSummary.expiredRequeueCount === 1,
    maxAttemptDeadLettered: proof.validationSummary.deadLetterCount === 1,
    duplicateMessageIdempotent:
      proof.validationSummary.duplicateMessageIgnoredCount === 1 &&
      proof.messageReceipts.some((row) => row.state === 'duplicate-ignored' && row.sideEffectApplied === false),
    noRuntimePolicyEnforcementOrRawTransferClaims: Object.values(proof.claimBoundaries).every(
      (value) => value === false
    ),
  },
  claimBoundaries: proof.claimBoundaries,
  nonClaims: [
    'This proof does not execute over a physical household LAN.',
    'This proof does not execute a model or prove model quality.',
    'This proof does not grant provider policy authority or dispatch enforcement.',
    'This proof does not transfer raw screenshots, retain raw screen pixels, or use remote/API AI.',
    'This proof does not add a package export while packages/parent-domain/package.json is locked by another lane.',
  ],
};

mkdirSync(outputRoot, { recursive: true });
mkdirSync(testResultRoot, { recursive: true });
writeFileSync(proofPath, `${JSON.stringify(proofSummary, null, 2)}\n`);
writeFileSync(
  validationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- household-ai-provider-claim-lease-proof',
  ].join('\n') + '\n'
);
writeFileSync(testResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(proofPath) }, null, 2)}\n`);
console.log(`household-ai-provider-claim-lease-proof-ok:${relativePath(proofPath)}`);

function relativePath(filePath) {
  return relative(repoRoot, filePath).replaceAll('\\', '/');
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: repoRoot, stdio: 'inherit' });
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
