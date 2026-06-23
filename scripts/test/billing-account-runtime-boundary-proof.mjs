import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'billing-account-runtime-boundary-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'tests/billing-account-runtime-boundary.test.ts',
    ]),
    {
      OCENTRA_PARENT_DOMAIN_TEST_SKIP_PROOF_CHAIN: '1',
    }
  );

  const contract = await assertBuiltContract();
  const packageExport = await assertPublicPackageExport();
  const documentation = await assertDocumentationProof();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/schema-domain/src/billing-account-runtime-boundary.ts',
      values: 'packages/schema-domain/src/billing-account-runtime-boundary-values.ts',
      proofModel: 'packages/schema-domain/src/billing-account-runtime-boundary-proof.ts',
      contractTest: 'packages/parent-domain/tests/billing-account-runtime-boundary.test.ts',
      packageExport,
      documentation,
      output: relativePath(proofPath),
    },
    accountStatuses: contract.accountStatuses,
    runtimeOperations: contract.runtimeOperations,
    entitlementSigningState: contract.entitlementSigningState,
    nonClaims: [
      'Stripe SDK',
      'provider secrets',
      'billing provider runtime',
      'account backend',
      'entitlement signing runtime',
      'portal UI',
      'child activity custody',
    ],
    knownGaps: [
      'account backend runtime',
      'billing provider runtime',
      'provider secret custody',
      'entitlement signing delivery runtime',
      'portal billing UI',
      'child-device entitlement consumption beyond signed local snapshots',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`billing-account-runtime-boundary-proof-ok:${relativePath(proofPath)}`);
}

async function assertBuiltContract() {
  const proofModulePath = pathToFileURL(
    join(repoRoot, 'packages', 'schema-domain', 'dist', 'billing-account-runtime-boundary-proof.js')
  );
  const boundaryModulePath = pathToFileURL(
    join(repoRoot, 'packages', 'schema-domain', 'dist', 'billing-account-runtime-boundary.js')
  );
  const proofModule = await import(proofModulePath.href);
  const boundaryModule = await import(boundaryModulePath.href);
  const proof = proofModule.BillingAccountRuntimeBoundaryProofReadModel;

  assert.equal(proof.schemaVersion, proofMode);
  assert.equal(typeof boundaryModule.decodeBillingAccountRuntimeBoundaryProof, 'function');
  assert.ok(boundaryModule.BillingAccountRuntimeBoundaryProofSchema);
  assert.deepEqual(
    summarizeValues(
      proof.accountStatusRows.map((row) => row.accountStatus),
      ['trialing', 'active', 'past-due', 'backend-unavailable', 'provider-unavailable', 'manual-review']
    ),
    {
      trialing: 0,
      active: 1,
      'past-due': 1,
      'backend-unavailable': 1,
      'provider-unavailable': 1,
      'manual-review': 1,
    }
  );
  assert.deepEqual(summarizeValues(proof.runtimeOperations.map((row) => row.operation)), {
    'account-status-read': 1,
    'subscription-status-read': 1,
    'entitlement-snapshot-read': 1,
    'device-limit-decision-read': 1,
    'download-status-read': 1,
    'provider-webhook-sync': 1,
  });
  assert.equal(proof.entitlementSigningBoundary.signingState, 'manual-required');
  assert.deepEqual(
    proof.nonClaims,
    [
      'no-stripe-sdk',
      'no-provider-secrets',
      'no-billing-provider-runtime',
      'no-account-backend',
      'no-entitlement-signing-runtime',
      'no-portal-ui',
      'no-child-activity-custody',
    ],
    'expected runtime boundary non-claims to remain explicit'
  );
  assert.equal(proof.childDeviceConsumptionClaim, 'signed-snapshot-consumption-contract');

  return {
    accountStatuses: proof.accountStatusRows.map((row) => row.accountStatus),
    runtimeOperations: proof.runtimeOperations.map((row) => row.operation),
    entitlementSigningState: proof.entitlementSigningBoundary.signingState,
  };
}

async function assertPublicPackageExport() {
  const module = await import('@ocentra-parent/schema-domain/billing-account-runtime-boundary');
  assert.equal(typeof module.decodeBillingAccountRuntimeBoundaryProof, 'function');
  assert.ok(module.BillingAccountRuntimeBoundaryProofSchema);
  return '@ocentra-parent/schema-domain/billing-account-runtime-boundary';
}

async function assertDocumentationProof() {
  const productionDistribution = await readRepoFile('docs/features/production-distribution-support.md');
  const billing = await readRepoFile('docs/expectations/billing.md');
  assertIncludes(productionDistribution, proofMode, 'production distribution feature proof note');
  assertIncludes(billing, proofMode, 'billing expectation proof note');
  return ['docs/features/production-distribution-support.md', 'docs/expectations/billing.md'];
}

async function readRepoFile(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function runCommand(commandName, args, env = {}) {
  commands.push([commandName, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, {
      cwd: repoRoot,
      env: { ...process.env, ...env },
      stdio: 'inherit',
      windowsHide: true,
    });
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

function summarizeValues(values, expectedValues = []) {
  const counts = new Map(expectedValues.map((value) => [value, 0]));
  for (const value of values) {
    counts.set(value, (counts.get(value) ?? 0) + 1);
  }
  return Object.fromEntries(counts);
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
