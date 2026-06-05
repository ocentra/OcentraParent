import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'production-release-public-status-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
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
    'tests/production-release-public-status-proof.test.ts',
  ]);

  const contract = await assertBuiltContract();
  const documentation = await assertDocumentationProof();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/parent-domain/src/production-release-public-status-proof.ts',
      contractTest: 'packages/parent-domain/tests/production-release-public-status-proof.test.ts',
      documentation,
      output: relativePath(proofPath),
    },
    surfaces: contract.surfaces,
    manualProofGaps: contract.manualProofGaps,
    nonClaims: contract.nonClaims,
    knownGaps: [
      'family.ocentra.ca public runtime',
      'account backend runtime',
      'billing provider runtime',
      'production publishing promotion',
      'signing notarization store proof',
      'updater execution',
      'support backend upload',
      'child activity custody exclusion',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)}`);
}

async function assertBuiltContract() {
  const proofModulePath = pathToFileURL(
    join(repoRoot, 'packages', 'parent-domain', 'dist', 'production-release-public-status-proof.js')
  );
  const proofModule = await import(proofModulePath.href);
  const readModel = proofModule.ProductionReleasePublicStatusProofReadModel;
  const parsed = proofModule.ProductionReleasePublicStatusProofSchema.parse(readModel);

  assert.deepEqual(proofModule.summarizeProductionReleasePublicStatusSurfaces(parsed.surfaces), {
    'public-download': 1,
    'release-status': 1,
    'update-status': 1,
    'account-status': 1,
    'subscription-status': 1,
    'support-status': 1,
  });
  assert.equal(parsed.publicHostState, 'not-implemented');
  assert.equal(parsed.productionPublishingState, 'production-promotion-required');
  assert.equal(parsed.childActivityCustodyClaim, 'not-implemented');
  assert.equal(parsed.publicSupportRuntimeClaim, 'not-implemented');
  assert.deepEqual(parsed.nonClaims, [
    'no-public-website-runtime',
    'no-account-backend',
    'no-billing-provider-runtime',
    'no-production-publishing',
    'no-signing-store-proof',
    'no-updater-execution',
    'no-support-backend-upload',
    'no-child-activity-custody',
  ]);

  return {
    surfaces: parsed.surfaces.map((row) => ({
      surface: row.surface,
      routeContractState: row.routeContractState,
      backendRuntimeState: row.backendRuntimeState,
      parentVisibleState: row.parentVisibleState,
      allowedDataClasses: row.allowedDataClasses,
    })),
    manualProofGaps: parsed.manualProofGaps.map((row) => ({
      gapId: row.gapId,
      state: row.state,
    })),
    nonClaims: parsed.nonClaims,
  };
}

async function assertDocumentationProof() {
  const docs = [
    'docs/features/production-distribution-support.md',
    'docs/expectations/release-installer.md',
    'docs/expectations/platform-deliverables.md',
    'docs/expectations/cloud.md',
    'docs/expectations/billing.md',
  ];
  for (const path of docs) {
    assertIncludes(await readRepoFile(path), proofMode, `${path} proof note`);
  }
  return docs;
}

async function readRepoFile(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function runCommand(commandName, args) {
  commands.push([commandName, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) =>
      code === 0 ? resolve() : reject(new Error(`${commandName} ${args.join(' ')} exited with ${code}`))
    );
    child.once('error', reject);
  });
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function assertIncludes(value, expected, label) {
  if (!value.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
