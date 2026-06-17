import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const lanDomainRoot = join(repoRoot, 'packages', 'lan-domain');
const outputDir = join(repoRoot, 'output', 'lan-plan-proof', '01-lan-b1-proof-regeneration');
const sourceMatrixProofPath = join(outputDir, '01-lan-source-matrix-plan-completion-proof.json');
const signedRelayProofPath = join(outputDir, '02-lan-signed-discovery-relay-spine-proof.json');
const productionDiscoveryProofPath = join(outputDir, '03-production-discovery-household-proof.json');
const proofPath = join(outputDir, '04-household-lan-proof-readiness.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npx', 'vitest', 'run', 'tests/unit/v0-9-household-lan-pairing-proof.test.ts'], lanDomainRoot);

  const sourceMatrixProof = await readJson(sourceMatrixProofPath);
  const signedRelayProof = await readJson(signedRelayProofPath);
  const productionDiscoveryProof = await readJson(productionDiscoveryProofPath);

  assertArrayIncludes(sourceMatrixProof.claimsNotProved, 'Physical two-device household LAN readiness.', 'source matrix non-claim');
  assertArrayIncludes(signedRelayProof.manualProofRequired, 'signed-child-agent-hello', 'signed relay manual hello');
  assertArrayIncludes(signedRelayProof.manualProofRequired, 'signed-child-agent-heartbeat', 'signed relay manual heartbeat');
  assertArrayIncludes(productionDiscoveryProof.claimsNotProved, 'Physical household LAN product readiness across two real devices.', 'production discovery non-claim');

  const manualRequiredBoundaries = [
    'two-physical-household-hosts',
    'household-router-reachability',
    'os-firewall-or-local-network-permission',
    'physical-origin-allowlist',
    'physical-pairing-revocation-rejection',
    'physical-stale-offline-selected-device',
    'real-mobile-controller-package',
    'real-mobile-observer-package',
    'signed-child-agent-hello',
    'signed-child-agent-heartbeat',
    'real-lan-ai-provider-host',
  ];

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'v0-9-household-lan-proof-readiness',
    ownerBoundary: 'packages/lan-domain',
    commands,
    artifactPath: relativePath(proofPath),
    consumedProofs: {
      sourceMatrix: relativePath(sourceMatrixProofPath),
      signedDiscoveryRelay: relativePath(signedRelayProofPath),
      productionDiscoveryHousehold: relativePath(productionDiscoveryProofPath),
    },
    productReadinessDecision: 'not-ready-for-product-ready-household-lan-claim',
    localMechanicalProofStates: {
      sourceMatrix: 'ci-mechanical-proof',
      signedDiscoveryRelay: 'ci-mechanical-proof',
      productionDiscoveryHousehold: 'ci-mechanical-proof',
    },
    manualRequiredBoundaries,
    notImplementedBoundaries: ['cloud-relay-separate-proof', 'remote-control-not-implemented'],
    claimsProvedByThisGate: [
      'Current lan-domain proofs distinguish local mechanical proof from physical household readiness.',
      'Signed artifact, router, firewall, stale or offline, and mobile-package boundaries remain explicit.',
      'Cloud relay and remote control remain non-claims for the B1 slice.',
    ],
    claimsNotProvedByThisGate: [
      'Physical household LAN readiness on two real devices.',
      'Router or firewall behavior outside local contract proof.',
      'Real Android or iOS controller or observer authority behavior.',
      'Cloud relay routing or remote-control implementation.',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('v0-9-household-lan-proof-readiness-ok');
  console.log(`evidence=${proofPath}`);
}

function assertArrayIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}.`);
  }
}

async function runCommand(commandName, args, cwd) {
  commands.push(`${relativePath(cwd)} :: ${[commandName, ...args].join(' ')}`);
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${commandName} ${args.join(' ')} exited with ${code}.`));
    });
    child.once('error', reject);
  });
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed.'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
