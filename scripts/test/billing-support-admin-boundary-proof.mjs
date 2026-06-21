import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'billing-support-admin-boundary-proof';
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
      'tests/billing-support-admin-boundary.test.ts',
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
      contract: 'packages/schema-domain/src/billing-support-admin-boundary.ts',
      values: 'packages/schema-domain/src/billing-support-admin-boundary-values.ts',
      proofModel: 'packages/schema-domain/src/billing-support-admin-boundary-proof.ts',
      contractTest: 'packages/parent-domain/tests/billing-support-admin-boundary.test.ts',
      packageExport,
      documentation,
      output: relativePath(proofPath),
    },
    actions: contract.actions,
    runtimeStates: contract.runtimeStates,
    nonClaims: [
      'Stripe SDK',
      'provider secrets',
      'billing provider contact execution',
      'account backend admin runtime',
      'entitlement admin override runtime',
      'refund/credit runtime',
      'portal admin UI',
      'support backend upload',
      'child activity custody',
    ],
    knownGaps: [
      'billing provider contact runtime',
      'account backend admin runtime',
      'entitlement admin override runtime',
      'refund and credit issuance runtime',
      'portal admin UI',
      'production support backend upload',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`billing-support-admin-boundary-proof-ok:${relativePath(proofPath)}`);
}

async function assertBuiltContract() {
  const proofModulePath = pathToFileURL(
    join(repoRoot, 'packages', 'schema-domain', 'dist', 'billing-support-admin-boundary-proof.js')
  );
  const boundaryModulePath = pathToFileURL(
    join(repoRoot, 'packages', 'schema-domain', 'dist', 'billing-support-admin-boundary.js')
  );
  const proofModule = await import(proofModulePath.href);
  const boundaryModule = await import(boundaryModulePath.href);
  const proof = proofModule.BillingSupportAdminBoundaryProofReadModel;

  assert.equal(proof.schemaVersion, proofMode);
  assert.equal(typeof boundaryModule.decodeBillingSupportAdminBoundaryProof, 'function');
  assert.ok(boundaryModule.BillingSupportAdminBoundaryProofSchema);
  assert.deepEqual(summarizeValues(proof.rows.map((row) => row.action)), {
    'support-case-triage': 1,
    'account-status-review': 1,
    'billing-escalation-request': 1,
    'provider-contact-manual-required': 1,
    'entitlement-admin-override-manual-required': 1,
    'refund-credit-manual-required': 1,
  });
  assert.deepEqual(summarizeValues(proof.rows.map((row) => row.runtimeState)), {
    'read-only-local-proof': 2,
    'manual-required': 2,
    'not-implemented': 2,
  });
  assert.equal(proof.providerContactClaim, 'not-executed');
  assert.equal(proof.backendUploadClaim, 'not-executed');
  assert.equal(proof.childActivityCustodyClaim, 'not-included');

  return {
    actions: proof.rows.map((row) => row.action),
    runtimeStates: proof.rows.map((row) => row.runtimeState),
  };
}

async function assertPublicPackageExport() {
  const module = await import('@ocentra-parent/schema-domain/billing-support-admin-boundary');
  assert.equal(typeof module.decodeBillingSupportAdminBoundaryProof, 'function');
  assert.ok(module.BillingSupportAdminBoundaryProofSchema);
  return '@ocentra-parent/schema-domain/billing-support-admin-boundary';
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

function summarizeValues(values) {
  return Object.fromEntries(values.reduce((counts, value) => counts.set(value, (counts.get(value) ?? 0) + 1), new Map()));
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
