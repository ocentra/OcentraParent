import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { spawn } from 'node:child_process';

const repoRoot = process.cwd();
const proofMode = 'tamper-uninstall-artifact-status-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tamper-uninstall-artifact-status',
  ]);

  const packageJson = JSON.parse(await readRepoFile('packages/parent-domain/package.json'));
  const contractModule = await import('@ocentra-parent/parent-domain/tamper-uninstall-artifact-status');
  const readModelModule = await import('@ocentra-parent/parent-domain/tamper-uninstall-artifact-status-read-model');
  const readModel = contractModule.TamperUninstallArtifactStatusReadModelSchema.parse(
    readModelModule.TamperUninstallArtifactStatusReadModel
  );
  const surfaces = readModel.entries.map((entry) => entry.surface);
  const nonClaims = readModel.entries.every((entry) => noClaimFieldsAreFalse(entry));
  const adminRemoval = readModel.entries.find((entry) => entry.surface === 'admin-removal-flow');

  assertPackageExport(packageJson);
  assertExactCoverage(surfaces);
  assertManualStates(readModel);
  assertAdminRemovalFlow(adminRemoval);
  if (!nonClaims) {
    throw new Error('Tamper uninstall artifact status proof contains claim upgrades.');
  }

  proofLabels.push(
    'tamper-uninstall-artifact-status.package-export',
    'tamper-uninstall-artifact-status.surface-coverage',
    'tamper-uninstall-artifact-status.manual-artifact-boundaries',
    'tamper-uninstall-artifact-status.admin-removal-flow',
    'tamper-uninstall-artifact-status.no-anti-tamper-claims'
  );

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode,
    commands,
    proofLabels,
    evidence: {
      contract: 'packages/parent-domain/src/tamper-uninstall-artifact-status.ts',
      contractTest: 'packages/parent-domain/tests/tamper-uninstall-artifact-status.test.ts',
      packageExports: ['./tamper-uninstall-artifact-status', './tamper-uninstall-artifact-status-read-model'],
      output: relativePath(proofPath),
    },
    summary: {
      entryCount: readModel.entries.length,
      surfaces,
      manualRequiredCount: readModel.entries.filter((entry) => entry.artifactState === 'manual-required').length,
      deviceProofRequiredCount: readModel.entries.filter((entry) => entry.artifactState === 'device-proof-required')
        .length,
      adminRemovalDocumented: adminRemoval?.parentVisibleStatus,
    },
    nonClaims: [
      'uninstall detection artifact capture',
      'anti-tamper resistance',
      'stealth or hidden persistence',
      'privilege escalation',
      'admin removal blocking',
      'notification provider delivery',
      'raw child data custody',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`tamper-uninstall-artifact-status-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relativePath(proofPath)}`);
}

function assertPackageExport(packageJson) {
  const exportEntry = packageJson.exports?.['./tamper-uninstall-artifact-status'];
  const readModelExportEntry = packageJson.exports?.['./tamper-uninstall-artifact-status-read-model'];
  if (
    exportEntry?.import !== './dist/tamper-uninstall-artifact-status.js' ||
    exportEntry?.types !== './dist/tamper-uninstall-artifact-status.d.ts'
  ) {
    throw new Error('Missing parent-domain tamper uninstall artifact status export.');
  }
  if (
    readModelExportEntry?.import !== './dist/tamper-uninstall-artifact-status-read-model.js' ||
    readModelExportEntry?.types !== './dist/tamper-uninstall-artifact-status-read-model.d.ts'
  ) {
    throw new Error('Missing parent-domain tamper uninstall artifact status read model export.');
  }
}

function assertExactCoverage(surfaces) {
  const expected = [
    'windows-service-stop',
    'windows-package-uninstall',
    'linux-service-package',
    'macos-launchd-package',
    'android-package-removed',
    'android-device-owner-managed-profile',
    'ios-family-controls-device-activity',
    'admin-removal-flow',
  ];
  if (JSON.stringify(surfaces) !== JSON.stringify(expected)) {
    throw new Error(`Unexpected tamper uninstall artifact surface coverage: ${surfaces.join(',')}`);
  }
}

function assertManualStates(readModel) {
  const manual = readModel.entries.filter((entry) => entry.artifactState === 'manual-required');
  const device = readModel.entries.filter((entry) => entry.artifactState === 'device-proof-required');
  if (manual.length !== 4 || device.length !== 3) {
    throw new Error('Expected four desktop manual artifact rows and three mobile device-proof rows.');
  }
}

function assertAdminRemovalFlow(adminRemoval) {
  if (
    adminRemoval?.artifactState !== 'documented-admin-removal' ||
    adminRemoval.adminRemovalFlowRefs[0] !== 'documented-parent-admin-removal-flow-ref' ||
    adminRemoval.adminRemovalBlockingClaimed
  ) {
    throw new Error('Admin removal flow row is missing documented non-blocking status.');
  }
}

function noClaimFieldsAreFalse(entry) {
  return [
    entry.uninstallDetectionClaimed,
    entry.tamperResistanceClaimed,
    entry.stealthPersistenceClaimed,
    entry.privilegeEscalationClaimed,
    entry.adminRemovalBlockingClaimed,
    entry.providerDeliveryClaimed,
    entry.rawChildDataIncluded,
  ].every((value) => value === false);
}

async function readRepoFile(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function runNpm(args) {
  await runCommand('cmd', ['/c', 'npm', ...args]);
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

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
