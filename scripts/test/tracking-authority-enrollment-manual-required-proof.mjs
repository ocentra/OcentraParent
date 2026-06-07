import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-authority-enrollment-manual-required-proof';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const output31 = path.join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '31-platform-extension-checklists-and-proof-routing'
);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(output31, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-authority-enrollment-manual-required-proof',
  ]);

  const proofModule = await importDist('tracking-authority-enrollment-manual-required-proof.js');
  const generatedAt = '2026-06-07T18:05:00.000Z';
  const readModel = proofModule.buildTrackingAuthorityEnrollmentManualRequiredProof(generatedAt);
  const proof = buildProof({ generatedAt, readModel });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-authority-enrollment-manual-required-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

function buildProof({ generatedAt, readModel }) {
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    workpackId: '31-platform-extension-checklists-and-proof-routing',
    companionWorkpackId: '33-proof-gates-fixtures-rollout-and-pr-gate',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P0_CONTRACT',
    status: 'authority_required',
    readModel,
    summary: {
      rowCount: readModel.rows.length,
      authorityRequiredRows: readModel.rows.filter((row) => row.state === 'authority-required').length,
      manualRequiredRows: readModel.rows.filter((row) => row.state === 'manual-required').length,
      authorityEnrollmentClaimedRows: readModel.rows.filter((row) => row.authorityEnrollmentClaimed).length,
      physicalDeviceClaimedRows: readModel.rows.filter((row) => row.physicalDeviceClaimed).length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    proofLabels: [
      'tracking-authority.android-device-owner-required',
      'tracking-authority.android-managed-profile-required',
      'tracking-authority.ios-family-controls-entitlement-required',
      'tracking-authority.ios-app-review-required',
      'tracking-authority.desktop-managed-policy-manual-required',
      'tracking-authority.no-hard-control-runtime-claim',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Authority enrollment and hard-control behavior require enrolled devices, capability grants, screenshots/log bundles, and parent-visible status rows. This CI proof names the required evidence and keeps authority, hard-control runtime, physical-device, provider delivery, production worker, and product-ready tracking claims false.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 5, 'expected five authority proof rows');
  assert.equal(proof.summary.authorityRequiredRows, 4, 'expected four authority-required rows');
  assert.equal(proof.summary.manualRequiredRows, 1, 'expected one manual-required row');
  assert.equal(proof.summary.authorityEnrollmentClaimedRows, 0, 'no authority enrollment claims');
  assert.equal(proof.summary.physicalDeviceClaimedRows, 0, 'no physical-device claims');
  assert.equal(proof.summary.productReadyRows, 0, 'no product-ready rows');
  assert.deepEqual(Object.values(proof.productClaims), [false, false, false, false, false, false]);
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(output31, '21-authority-enrollment-manual-required-proof.json'), proof);
  await writeJson(path.join(output33, '48-authority-enrollment-manual-required-proof.json'), proof);
  await writeFile(path.join(output31, '21-authority-enrollment-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
  await writeFile(
    path.join(output31, '21-authority-enrollment-validation-commands.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
}

function sourceSnapshot(proof) {
  return [
    '# WP31 Tracking Authority Enrollment Manual-Required Proof',
    '',
    `- Branch: ${proof.branch}`,
    `- Commit: ${proof.commit}`,
    `- Evidence: ${relativePath(path.join(resultDir, 'proof.json'))}`,
    `- Status: ${proof.status}`,
    '',
    '## Required Evidence Rows',
    '',
    ...proof.readModel.rows.map(
      (row) => `- ${row.enrollmentMode}: ${row.state}; required refs: ${row.requiredEvidenceRefs.join(', ')}`
    ),
    '',
    '## Non-Claims',
    '',
    '- No authority enrollment is claimed.',
    '- No hard-control runtime is claimed.',
    '- No physical-device behavior is claimed.',
    '- No provider delivery, production worker, or product-ready tracking is claimed.',
    '',
  ].join('\n');
}

function importDist(name) {
  return import(pathToFileURL(path.join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function run(command, args) {
  commands.push({ command: [command, ...args].join(' ') });
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

async function writeJson(filePath, value) {
  await mkdir(path.dirname(filePath), { recursive: true });
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}
