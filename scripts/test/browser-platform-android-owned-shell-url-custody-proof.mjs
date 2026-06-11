import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const root = process.cwd();
const proofRoot = join(root, 'output', 'browser-plan-proof', '05-cross-platform-inventory-matrix');
const sourceProofPath = join(proofRoot, '15-android-owned-browser-shell-proof.json');
const manifestPath = join(proofRoot, '17-android-owned-shell-url-custody-proof.json');
const resultDirectory = join(root, 'test-results', 'browser-platform-android-owned-shell-url-custody-proof');
const proofPath = join(resultDirectory, 'proof.json');

await main();

async function main() {
  buildWorkspace('@ocentra-parent/schema-domain');
  buildWorkspace('@ocentra-parent/activity-domain');
  runFocusedTest();

  if (!existsSync(sourceProofPath)) {
    throw new Error(`Missing Android owned-shell source proof: ${relativePath(sourceProofPath)}`);
  }

  const sourceProof = JSON.parse(await readFile(sourceProofPath, 'utf8'));
  const custodyModule = await import(
    pathToFileURL(join(root, 'packages', 'activity-domain', 'dist', 'browser-android-owned-shell-url-custody.js')).href
  );
  const readModel = custodyModule.buildBrowserAndroidOwnedShellUrlCustodyReadModel(sourceProof);
  const failures = validateReadModel(readModel);
  const proof = {
    schemaVersion: 1,
    proofMode: 'browser-platform-android-owned-shell-url-custody-proof',
    generatedAt: new Date().toISOString(),
    branch: git(['branch', '--show-current']),
    commit: git(['rev-parse', 'HEAD']),
    baseCommit: git(['rev-parse', 'origin/main']),
    sourceProof: relativePath(sourceProofPath),
    source: 'packages/activity-domain/src/browser-android-owned-shell-url-custody.ts',
    test: 'packages/activity-domain/tests/browser-android-owned-shell-url-custody.test.ts',
    summary: {
      rows: readModel.rows.length,
      physicalRequestedUrlRefRows: readModel.physicalRequestedUrlRefRows,
      manualRequiredRows: readModel.manualRequiredRows,
      exactActiveTabClaimed: readModel.exactActiveTabClaimed,
      policyExecutionClaimed: readModel.policyExecutionClaimed,
      enforcementClaimed: readModel.enforcementClaimed,
      productClaimed: readModel.productClaimed,
      rawUrlPersisted: readModel.rows.some((row) => row.rawUrlPersisted),
      physicalDeviceOwnerClaimed: readModel.rows.some((row) => row.physicalDeviceOwnerClaimed),
      physicalBrowserRoleRoutingClaimed: readModel.rows.some((row) => row.physicalBrowserRoleRoutingClaimed),
      failures: failures.length,
    },
    readModel,
    noClaimBoundaries: {
      rawUrlPersisted: false,
      physicalDeviceOwnerClaimed: false,
      physicalBrowserRoleRoutingClaimed: false,
      exactActiveTabClaimed: false,
      policyExecutionClaimed: false,
      vpnDnsBrowserProofClaimed: false,
      usageStatsRouteProofClaimed: false,
      accessibilityRouteProofClaimed: false,
      enforcementClaimed: false,
      productClaimed: false,
    },
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Android owned-shell URL custody proof failed:\n${failures.join('\n')}`);
  }

  await mkdir(proofRoot, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(manifestPath, `${JSON.stringify(proof, null, 2)}\n`);

  console.log('browser-platform-android-owned-shell-url-custody-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(manifestPath)}`);
  console.log(
    `physicalRequestedUrlRefRows=${proof.summary.physicalRequestedUrlRefRows} manualRequiredRows=${proof.summary.manualRequiredRows}`
  );
}

function validateReadModel(readModel) {
  const failures = [];
  if (readModel.physicalRequestedUrlRefRows !== 1) {
    failures.push(`expected one physical requested-URL ref row, received ${readModel.physicalRequestedUrlRefRows}`);
  }
  if (readModel.manualRequiredRows !== 1) {
    failures.push(`expected one manual-required row, received ${readModel.manualRequiredRows}`);
  }
  if (readModel.exactActiveTabClaimed || readModel.policyExecutionClaimed || readModel.enforcementClaimed) {
    failures.push('Android owned-shell URL custody read model claimed active-tab, policy execution, or enforcement');
  }
  for (const row of readModel.rows) {
    if (
      row.rawUrlPersisted ||
      row.physicalDeviceOwnerClaimed ||
      row.physicalBrowserRoleRoutingClaimed ||
      row.knownActiveTabProofClaimed ||
      row.vpnDnsBrowserProofClaimed ||
      row.usageStatsRouteProofClaimed ||
      row.accessibilityRouteProofClaimed ||
      row.finalPolicyExecutionClaimed ||
      row.enforcementClaimed
    ) {
      failures.push(`dishonest Android owned-shell URL custody claim in row ${row.reasonCode}`);
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
      '@ocentra-parent/activity-domain',
      '--',
      'browser-android-owned-shell-url-custody.test.ts',
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
