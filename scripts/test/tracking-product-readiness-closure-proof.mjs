import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-product-readiness-closure-proof';
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const namedProofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const resultRoot = join(repoRoot, 'test-results', proofMode);
const generatedAt = '2026-06-07T16:30:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

const sourceProofs = [
  sourceProof('pre-device-gate', 'output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json'),
  sourceProof('android-emulator-proof', 'test-results/tracking-plan-android-emulator-proof/proof.json'),
  sourceProof('ios-simulator-proof', 'test-results/tracking-plan-ios-simulator-proof/proof.json'),
  sourceProof(
    'ios-privacy-disclosure-release-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/47-ios-privacy-disclosure-release-proof.json'
  ),
  sourceProof('wsl-local-replay', 'output/tracking-plan-proof/wsl-local-replay/proof.json'),
  sourceProof('hosted-ui-artifact-inventory', 'test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json'),
  sourceProof(
    'android-system-geofence-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/44-android-system-geofence-blocker-proof.json'
  ),
  sourceProof(
    'notification-receipt-boundary',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/22-notification-receipt-boundary-proof.json'
  ),
  sourceProof(
    'notification-preference-preflight',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/24-notification-preference-preflight-proof.json'
  ),
  sourceProof(
    'notification-preference-status-handoff',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/54-notification-preference-status-handoff-proof.json'
  ),
  sourceProof(
    'notification-local-outbox-readiness',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/42-notification-local-outbox-readiness-proof.json'
  ),
  sourceProof(
    'authority-enrollment-manual-required',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/48-authority-enrollment-manual-required-proof.json'
  ),
  sourceProof(
    'authority-runtime-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/55-authority-runtime-readiness-blocker-proof.json'
  ),
  sourceProof(
    'child-runtime-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/50-child-runtime-artifact-gate-proof.json'
  ),
  sourceProof(
    'physical-device-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/49-physical-device-artifact-gate-proof.json'
  ),
  sourceProof(
    'provider-delivery-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/51-provider-delivery-artifact-gate-proof.json'
  ),
  sourceProof(
    'provider-runtime-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/52-provider-runtime-readiness-blocker-proof.json'
  ),
  sourceProof(
    'escalation-runtime-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/53-escalation-runtime-readiness-blocker-proof.json'
  ),
  sourceProof(
    'child-runtime-product-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/45-child-runtime-product-readiness-blocker-proof.json'
  ),
  sourceProof(
    'full-product-ui-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/56-full-product-ui-readiness-blocker-proof.json'
  ),
  sourceProof(
    'full-product-ui-runtime-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/59-full-product-ui-runtime-artifact-gate-proof.json'
  ),
  sourceProof(
    'production-durable-workers-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/57-production-durable-workers-readiness-blocker-proof.json'
  ),
  sourceProof(
    'production-worker-runtime-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/58-production-worker-runtime-artifact-gate-proof.json'
  ),
  sourceProof(
    'retention-product-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/43-retention-product-readiness-proof.json'
  ),
  sourceProof(
    'retention-runtime-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/60-retention-runtime-artifact-gate-proof.json'
  ),
];

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(output33, { recursive: true });
  await mkdir(namedProofRoot, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-product-readiness-closure-proof',
  ]);

  await assertSourceProofsExist();
  const proof = await buildProof();
  assertProof(proof);
  await writeProofArtifacts(proof);

  console.log('tracking-product-readiness-closure-proof-ok');
  console.log('evidence=test-results/tracking-product-readiness-closure-proof/proof.json');
}

async function buildProof() {
  const proofModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-product-readiness-closure-proof.js'))
      .href
  );
  return {
    ...proofModule.buildTrackingProductReadinessClosureProof(generatedAt, sourceProofs),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    artifactPaths: {
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/46-product-readiness-closure-proof.json',
      evidence: 'test-results/tracking-product-readiness-closure-proof/proof.json',
      namedProofRoot: 'output/tracking-plan-proof/tracking-product-readiness-closure-proof/proof.json',
    },
  };
}

function assertProof(proof) {
  const [row] = proof.rows;
  if (!row || !proof.proofClaims.remainingProductBlockersEnumerated) {
    throw new Error(`Tracking product readiness closure proof is empty: ${JSON.stringify(proof)}`);
  }
  if (
    row.physicalAndroidBackgroundClaimed ||
    row.physicalIosBackgroundClaimed ||
    row.authorityClaimed ||
    row.productionWorkersClaimed ||
    row.productReadyClaimed
  ) {
    throw new Error(`Tracking product readiness closure overclaimed product readiness: ${JSON.stringify(row)}`);
  }
}

async function writeProofArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(resultRoot, 'product-readiness-closure-read-model.json'), proof.rows);
  await writeJson(join(output33, '46-product-readiness-closure-proof.json'), proof);
  await writeJson(join(namedProofRoot, 'proof.json'), proof);
  await writeFile(join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot(proof));
  await writeFile(join(namedProofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(join(namedProofRoot, '16-validation-commands.log'), validationLog());
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Product Readiness Closure Source Snapshot',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.baseCommitAtGeneration}`,
    '- requiredProofTier: P3_LOCAL_DEV_MACHINE',
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    '- status: proved',
    '- proves local/CI proof accounting is closed for current tracking continuation scope',
    '- does not prove retention product settings, physical-device, authority, provider-delivery, production, or product-ready tracking behavior',
    '- proof module: packages/parent-domain/src/tracking-product-readiness-closure-proof.ts',
    '- proof tests: packages/parent-domain/tests/tracking-product-readiness-closure-proof.test.ts',
    '- proof harness: scripts/test/tracking-product-readiness-closure-proof.mjs',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=33-proof-gates-fixtures-rollout-and-pr-gate',
    'Closure rows cite existing local/CI proof refs and enumerate remaining product blockers.',
    'Rows do not claim writable retention product settings, platform retention enforcement, Android/iOS physical background behavior, authority enrollment, provider delivery/receipt runtime, production workers, actual child-device runtime, or product readiness.',
    '',
  ].join('\n');
}

async function assertSourceProofsExist() {
  for (const source of sourceProofs) {
    const contents = await readFile(join(repoRoot, source.proofRef), 'utf8');
    const parsed = JSON.parse(contents);
    source.status = statusFrom(parsed);
    source.proofTier = proofTierFrom(parsed, source.proofTier);
  }
}

function sourceProof(coverageTag, proofRef) {
  return {
    coverageTag,
    proofRef,
    status: 'proved',
    proofTier: 'P3_LOCAL_DEV_MACHINE',
  };
}

function statusFrom(parsed) {
  if (typeof parsed.status === 'string' && parsed.status.length > 0) return parsed.status;
  if (parsed.proofClaims || parsed.productClaims || parsed.generatedAt) return 'proved';
  return 'present';
}

function proofTierFrom(parsed, fallback) {
  if (typeof parsed.currentProofTier === 'string' && parsed.currentProofTier.length > 0) {
    return parsed.currentProofTier;
  }
  if (typeof parsed.requiredProofTier === 'string' && parsed.requiredProofTier.length > 0) {
    return parsed.requiredProofTier;
  }
  return fallback;
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  commands.push({
    command: printable,
    status: result.status,
    stdout: result.stdout.trim(),
    stderr: result.stderr.trim(),
  });
  if (result.status !== 0) {
    throw new Error(`${printable} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function validationLog() {
  return `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`;
}

function gitOutput(args) {
  const result = spawnSync('git', args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  if (result.status !== 0) return '';
  return result.stdout.trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}
