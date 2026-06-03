import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-approval-contract-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/app-install-purchase-approval.test.ts',
  ]);

  const proofModule = await loadContractProofModule();
  const parsedReadModel = proofModule.AppInstallPurchaseApprovalContractProofReadModel;
  const supportStateCounts = proofModule.summarizeAppInstallPurchaseApprovalSupportStates(
    parsedReadModel.platformSupportMatrix
  );
  assert.equal(supportStateCounts.supported > 0, true);
  assert.equal(supportStateCounts['manual-required'] > 0, true);
  assert.equal(supportStateCounts.unavailable > 0, true);
  assert.deepEqual(
    parsedReadModel.approvalDecisions.map((decision) => decision.decisionAction),
    ['approve', 'deny', 'time-box', 'review-needed']
  );

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'app-install-purchase-approval-contract-proof',
    commands,
    evidence: {
      contract: 'packages/parent-domain/src/app-install-purchase-approval.ts',
      contractTest: 'packages/parent-domain/tests/app-install-purchase-approval.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      output: relative(repoRoot, proofPath),
    },
    requestKinds: [
      parsedReadModel.installRequest.requestKind,
      parsedReadModel.purchaseRequest.requestKind,
      parsedReadModel.subscriptionRequest.requestKind,
    ],
    approvalDecisionActions: parsedReadModel.approvalDecisions.map((decision) => decision.decisionAction),
    supportStateCounts,
    platformSupportMatrix: parsedReadModel.platformSupportMatrix.map((row) => ({
      platform: row.platform,
      storeSurface: row.storeSurface,
      contractRequestState: row.contractRequestState,
      storeMetadataState: row.storeMetadataState,
      installInterceptionState: row.installInterceptionState,
      purchaseInterceptionState: row.purchaseInterceptionState,
      subscriptionInterceptionState: row.subscriptionInterceptionState,
      childPendingState: row.childPendingState,
      approvalDeliveryState: row.approvalDeliveryState,
      proofRequirement: row.proofRequirement,
      claimBoundary: row.claimBoundary,
    })),
    nonClaims: parsedReadModel.nonClaims,
    claimBoundaries: {
      storeIntegrationClaim: parsedReadModel.storeIntegrationClaim,
      billingEntitlementClaim: parsedReadModel.billingEntitlementClaim,
      portalUiClaim: parsedReadModel.portalUiClaim,
      platformAdapterClaim: parsedReadModel.platformAdapterClaim,
      interceptionClaim: parsedReadModel.interceptionClaim,
      runtimeBlockingSeparation: parsedReadModel.runtimeBlockingSeparation,
    },
    knownGaps: proofModule.AppInstallPurchaseApprovalProofKnownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`app-install-purchase-approval-contract-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadContractProofModule() {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-install-purchase-approval-proof.js');
  return import(pathToFileURL(modulePath).href);
}

async function gitHead() {
  const output = await commandOutput('git', ['rev-parse', 'HEAD']);
  return output.trim();
}

async function commandOutput(command, args) {
  const chunks = [];
  const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
  child.stdout.on('data', (chunk) => chunks.push(chunk));
  child.stderr.on('data', (chunk) => chunks.push(chunk));
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  const output = Buffer.concat(chunks).toString('utf8');
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${exitCode}\n${output}`);
  }
  return output;
}

async function runCommand(command, args) {
  const startedAt = new Date().toISOString();
  const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit' });
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  commands.push({ command: `${command} ${args.join(' ')}`, startedAt, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${exitCode}`);
  }
}
