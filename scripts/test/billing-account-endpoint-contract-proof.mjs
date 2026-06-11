import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'billing-account-endpoint-contract-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/endpoint-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/endpoint-domain',
      '--',
      'tests/billing-account.test.ts',
    ])
  );

  const packageExport = await assertPackageExport();
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
      contract: 'packages/endpoint-domain/src/constants/billing-account.ts',
      contractTest: 'packages/endpoint-domain/tests/billing-account.test.ts',
      packageExport,
      documentation,
      output: relativePath(proofPath),
    },
    billingAccountContracts: contract.billingAccountContracts,
    accountDistributionContracts: contract.accountDistributionContracts,
    nonClaims: [
      'Stripe SDK',
      'billing provider backend',
      'account backend',
      'download or updater runtime',
      'portal UI',
      'child activity custody',
    ],
    knownGaps: [
      'billing provider integration',
      'account/subscription backend implementation',
      'signed entitlement snapshot runtime',
      'download/update handler implementation',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`billing-account-endpoint-contract-proof-ok:${relativePath(proofPath)}`);
}

async function assertPackageExport() {
  const packageJson = JSON.parse(await readRepoFile('packages/endpoint-domain/package.json'));
  assert.deepEqual(packageJson.exports['./constants/billing-account'], {
    import: './dist/constants/billing-account.js',
    types: './dist/constants/billing-account.d.ts',
  });
  return 'packages/endpoint-domain/package.json#exports[./constants/billing-account]';
}

async function assertBuiltContract() {
  const modulePath = pathToFileURL(
    join(repoRoot, 'packages', 'endpoint-domain', 'dist', 'constants', 'billing-account.js')
  );
  const module = await import(modulePath.href);

  assert.deepEqual(module.BillingAccountApiPath, {
    AccountStatus: '/api/v1/billing-account/account-status',
    PlanEntitlementSnapshot: '/api/v1/billing-account/plan-entitlement-snapshot',
    SubscriptionStatus: '/api/v1/billing-account/subscription-status',
    DeviceLimitDecision: '/api/v1/billing-account/device-limit-decision',
  });
  assert.deepEqual(module.AccountDistributionApiPath, {
    DownloadSurface: '/api/v1/account-distribution/download-surface',
    UpdateStatus: '/api/v1/account-distribution/update-status',
    ReleaseStatus: '/api/v1/account-distribution/release-status',
  });
  assert.deepEqual(module.BillingAccountBoundaryState, {
    RouteContract: 'defined',
    StripeSdk: 'not-included',
    BillingProviderBackend: 'not-implemented',
    AccountBackend: 'not-implemented',
    ChildActivityCustody: 'not-supported',
    PortalUi: 'not-implemented',
  });

  return {
    billingAccountContracts: Object.keys(module.BillingAccountApiPath),
    accountDistributionContracts: Object.keys(module.AccountDistributionApiPath),
  };
}

async function assertDocumentationProof() {
  const productionDistribution = await readRepoFile('docs/features/production-distribution-support.md');
  const billing = await readRepoFile('docs/expectations/billing.md');
  const cloud = await readRepoFile('docs/expectations/cloud.md');
  const endpointReadme = await readRepoFile('packages/endpoint-domain/README.md');
  assertIncludes(productionDistribution, proofMode, 'production distribution feature proof note');
  assertIncludes(billing, proofMode, 'billing expectation proof note');
  assertIncludes(cloud, proofMode, 'cloud expectation proof note');
  assertIncludes(endpointReadme, proofMode, 'endpoint-domain README proof note');
  return [
    'docs/features/production-distribution-support.md',
    'docs/expectations/billing.md',
    'docs/expectations/cloud.md',
    'packages/endpoint-domain/README.md',
  ];
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
