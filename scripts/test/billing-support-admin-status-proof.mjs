import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const proofMode = 'billing-support-admin-status-proof';
const testResultsDir = join(repoRoot, 'test-results', proofMode);
const outputDir = join(repoRoot, 'output', proofMode);
const proofPath = join(testResultsDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const commands = [];

await main();

async function main() {
  await mkdir(testResultsDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'tests/billing-support-admin-status-proof.test.ts',
    ]),
    {
      OCENTRA_PARENT_DOMAIN_TEST_SKIP_PROOF_CHAIN: '1',
    }
  );

  const contract = await assertPublicPackageExport();
  const documentation = await assertDocumentationProof();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/parent-domain/src/billing-support-admin-status-proof.ts',
      values: 'packages/parent-domain/src/billing-support-admin-status-values.ts',
      contractTest: 'packages/parent-domain/tests/billing-support-admin-status-proof.test.ts',
      packageExport: '@ocentra-parent/parent-domain/billing-support-admin-status-proof',
      documentation,
      proof: relativePath(proofPath),
      summary: relativePath(summaryPath),
    },
    statusRows: contract.statusRows,
    runtimeStates: contract.runtimeStates,
    nonClaims: [
      'Stripe SDK',
      'provider secrets',
      'billing provider contact execution',
      'account lookup execution',
      'entitlement admin override runtime',
      'refund and credit runtime',
      'portal admin UI',
      'support backend upload',
      'child activity custody',
    ],
    knownGaps: [
      'billing provider contact runtime',
      'account lookup execution and account backend admin runtime',
      'entitlement admin override runtime',
      'refund and credit issuance runtime',
      'portal admin UI and support backend upload',
      'production billing support execution',
    ],
  };
  const summary = {
    proofMode,
    commit,
    statusRows: contract.statusRows,
    runtimeStates: contract.runtimeStates,
    nonClaims: proof.nonClaims,
    knownGaps: proof.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`billing-support-admin-status-proof-ok:${relativePath(proofPath)}`);
}

async function assertPublicPackageExport() {
  const module = await import('@ocentra-parent/parent-domain/billing-support-admin-status-proof');
  const proof = module.BillingSupportAdminStatusProofReadModel;

  assert.equal(proof.schemaVersion, proofMode);
  assert.equal(typeof module.decodeBillingSupportAdminStatusProof, 'function');
  assert.deepEqual(module.summarizeBillingSupportAdminStatusRows(proof.rows), {
    'case-triage-visible': 1,
    'account-review-visible': 1,
    'billing-escalation-visible': 1,
    'provider-contact-manual-required': 1,
    'entitlement-override-manual-required': 1,
    'refund-credit-manual-required': 1,
    'resolution-update-ready': 1,
  });
  assert.deepEqual(module.summarizeBillingSupportAdminStatusRuntimeStates(proof.rows), {
    'source-contract-ready': 3,
    'manual-required': 2,
    'not-implemented': 2,
  });
  assert.equal(proof.providerClaim, 'not-executed');
  assert.equal(proof.portalAdminUiClaim, 'not-implemented');
  assert.equal(proof.childActivityCustodyClaim, 'not-supported');

  return {
    statusRows: proof.rows.map((row) => row.statusRow),
    runtimeStates: proof.rows.map((row) => row.runtimeState),
  };
}

async function assertDocumentationProof() {
  const productionDistribution = await readRepoFile('docs/features/production-distribution-support.md');
  const billing = await readRepoFile('docs/expectations/billing.md');
  const readme = await readRepoFile('packages/parent-domain/README.md');
  assertIncludes(productionDistribution, proofMode, 'production distribution feature proof note');
  assertIncludes(billing, proofMode, 'billing expectation proof note');
  assertIncludes(readme, proofMode, 'parent-domain README proof note');
  return [
    'docs/features/production-distribution-support.md',
    'docs/expectations/billing.md',
    'packages/parent-domain/README.md',
  ];
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
