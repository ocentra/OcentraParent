import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const root = process.cwd();
const proofRoot = join(root, 'output', 'browser-plan-proof', '05-cross-platform-inventory-matrix');
const sourceProofPath = join(proofRoot, '15-android-owned-browser-shell-proof.json');
const manifestPath = join(proofRoot, '16-android-owned-shell-runtime-proof.json');
const resultDirectory = join(root, 'test-results', 'browser-platform-android-owned-shell-runtime-proof');
const proofPath = join(resultDirectory, 'proof.json');

await main();

async function main() {
  buildWorkspace('@ocentra-parent/schema-domain');
  buildWorkspace('@ocentra-parent/browser-domain');
  runFocusedTest();

  if (!existsSync(sourceProofPath)) {
    throw new Error(`Missing Android owned-shell source proof: ${relativePath(sourceProofPath)}`);
  }

  const sourceProof = JSON.parse(await readFile(sourceProofPath, 'utf8'));
  const runtimeModule = await import(
    pathToFileURL(join(root, 'packages', 'activity-domain', 'dist', 'browser-android-owned-shell-runtime.js')).href
  );
  const readModel = runtimeModule.buildBrowserAndroidOwnedShellRuntimeReadModel(sourceProof);
  const failures = validateReadModel(readModel);
  const proof = {
    schemaVersion: 1,
    proofMode: 'browser-platform-android-owned-shell-runtime-proof',
    generatedAt: new Date().toISOString(),
    branch: git(['branch', '--show-current']),
    commit: git(['rev-parse', 'HEAD']),
    baseCommit: git(['rev-parse', 'origin/main']),
    sourceProof: relativePath(sourceProofPath),
    source: 'packages/browser-domain/src/browser-android-owned-shell-runtime.ts',
    test: 'packages/browser-domain/tests/unit/browser-android-owned-shell-runtime.test.ts',
    summary: {
      rows: readModel.rows.length,
      physicalVisibleRows: readModel.physicalVisibleRows,
      manualRequiredRows: readModel.manualRequiredRows,
      productClaimed: readModel.productClaimed,
      physicalDeviceOwnerClaimed: readModel.rows.some((row) => row.physicalDeviceOwnerClaimed),
      physicalBrowserRoleRoutingClaimed: readModel.rows.some((row) => row.physicalBrowserRoleRoutingClaimed),
      exactUrlPolicyClaimed: readModel.rows.some((row) => row.exactUrlPolicyClaimed),
      knownActiveTabProofClaimed: readModel.rows.some((row) => row.knownActiveTabProofClaimed),
      enforcementClaimed: readModel.rows.some((row) => row.enforcementClaimed),
      failures: failures.length,
    },
    readModel,
    noClaimBoundaries: {
      physicalDeviceOwnerClaimed: false,
      physicalBrowserRoleRoutingClaimed: false,
      exactUrlPolicyClaimed: false,
      knownActiveTabProofClaimed: false,
      vpnDnsBrowserProofClaimed: false,
      usageStatsRouteProofClaimed: false,
      accessibilityRouteProofClaimed: false,
      finalPolicyExecutionClaimed: false,
      enforcementClaimed: false,
      productClaimed: false,
    },
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Android owned-shell runtime proof failed:\n${failures.join('\n')}`);
  }

  await mkdir(proofRoot, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(manifestPath, `${JSON.stringify(proof, null, 2)}\n`);

  console.log('browser-platform-android-owned-shell-runtime-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(manifestPath)}`);
  console.log(
    `physicalVisibleRows=${proof.summary.physicalVisibleRows} manualRequiredRows=${proof.summary.manualRequiredRows}`
  );
}

function validateReadModel(readModel) {
  const failures = [];
  const physicalVisibleRows = readModel.rows.filter((row) => row.runtimeState === 'physical-visible-owned-shell');
  const manualRequiredRows = readModel.rows.filter((row) => row.runtimeState === 'manual-required');
  if (physicalVisibleRows.length !== 1) {
    failures.push(`expected exactly one physical visible owned-shell row, received ${physicalVisibleRows.length}`);
  }
  if (manualRequiredRows.length !== 1) {
    failures.push(`expected exactly one manual-required no-claim row, received ${manualRequiredRows.length}`);
  }
  if (readModel.productClaimed !== false) {
    failures.push('Android owned-shell runtime read model claimed product completion');
  }
  for (const row of readModel.rows) {
    if (
      row.exactUrlPolicyClaimed ||
      row.knownActiveTabProofClaimed ||
      row.physicalDeviceOwnerClaimed ||
      row.physicalBrowserRoleRoutingClaimed ||
      row.vpnDnsBrowserProofClaimed ||
      row.usageStatsRouteProofClaimed ||
      row.accessibilityRouteProofClaimed ||
      row.finalPolicyExecutionClaimed ||
      row.enforcementClaimed
    ) {
      failures.push(`dishonest Android owned-shell runtime claim in row ${row.reasonCode}`);
    }
  }
  return failures;
}

function buildWorkspace(workspace) {
  execFileSync(...npmCommand(['run', 'build', '--workspace', workspace]), {
    cwd: root,
    stdio: 'inherit',
  });
}

function runFocusedTest() {
  execFileSync(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/browser-domain',
      '--',
      'browser-android-owned-shell-runtime.test.ts',
    ]),
    {
      cwd: root,
      stdio: 'inherit',
    }
  );
}

function git(args) {
  return execFileSync('git', args, { cwd: root, encoding: 'utf8' }).trim();
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
