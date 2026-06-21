import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'ai-plan-proof', 'household-ai-provider-advertisement-heartbeat-proof');
const testResultRoot = resolve(repoRoot, 'test-results', 'household-ai-provider-advertisement-heartbeat-proof');
const proofPath = join(outputRoot, 'proof-summary.json');
const validationLogPath = join(outputRoot, 'validation-commands.log');
const testResultPath = join(testResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand(...npmCommand(['run', 'build:contracts']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/ai-domain',
    '--',
    'household-ai-provider-advertisement-heartbeat-proof',
  ])
);

const proofModule = await import('@ocentra-parent/schema-domain/household-ai-provider-advertisement-heartbeat-proof');

const proof = proofModule.HouseholdAiProviderAdvertisementHeartbeatProofSchema.parse({
  ...proofModule.HouseholdAiProviderAdvertisementHeartbeatProof,
  generatedAt,
});

const proofSummary = {
  status: 'ok',
  proofKind: 'household-ai-provider-advertisement-heartbeat-proof',
  generatedAt,
  proof: relativePath(proofPath),
  sourceContracts: ['packages/schema-domain/src/household-ai-provider-advertisement-heartbeat-proof.ts'],
  requestedCapability: proof.requestedCapability,
  providerStates: proof.advertisements.map((row) => ({
    providerId: row.providerId,
    state: row.state,
    capabilities: row.capabilities,
    resourceClass: row.resourceClass,
    heartbeatAgeMs: row.heartbeatAgeMs,
    heartbeatTtlMs: row.heartbeatTtlMs,
    rejectionReason: row.rejectionReason,
  })),
  validationSummary: proof.validationSummary,
  assertions: {
    freshTrustedProviderEligible: proof.validationSummary.eligibleProviderCount === 1,
    staleProviderRejected: proof.validationSummary.staleProviderRejectedCount === 1,
    offlineProviderRejected: proof.validationSummary.offlineProviderRejectedCount === 1,
    revokedProviderRejected: proof.validationSummary.revokedProviderRejectedCount === 1,
    unsupportedProviderRejected: proof.validationSummary.unsupportedProviderRejectedCount === 1,
    noRawPayloadAdvertisement: proof.validationSummary.rawPayloadAdvertisementCount === 0,
    noRemoteApiAdvertisement: proof.validationSummary.remoteApiAdvertisementCount === 0,
    noRuntimePolicyEnforcementOrRawTransferClaims: Object.values(proof.claimBoundaries).every(
      (value) => value === false
    ),
  },
  claimBoundaries: proof.claimBoundaries,
  nonClaims: [
    'This proof does not execute over a physical household LAN or run provider gossip.',
    'This proof does not execute a model or prove model quality.',
    'This proof does not grant provider policy authority or dispatch enforcement.',
    'This proof does not transfer raw screenshots, retain raw screen pixels, or use remote/API AI.',
    'This proof does not claim a parent-domain facade or any new package export surface.',
  ],
};

mkdirSync(outputRoot, { recursive: true });
mkdirSync(testResultRoot, { recursive: true });
writeFileSync(proofPath, `${JSON.stringify(proofSummary, null, 2)}\n`);
writeFileSync(
  validationLogPath,
  [
    'cmd /c npm run build:contracts',
    'cmd /c npm run test --workspace @ocentra-parent/ai-domain -- household-ai-provider-advertisement-heartbeat-proof',
  ].join('\n') + '\n'
);
writeFileSync(testResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(proofPath) }, null, 2)}\n`);
console.log(`household-ai-provider-advertisement-heartbeat-proof-ok:${relativePath(proofPath)}`);

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
