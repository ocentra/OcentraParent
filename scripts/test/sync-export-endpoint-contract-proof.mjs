import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'sync-export-endpoint-contract-proof';
const outputDir = join(repoRoot, 'output', 'data-custody-storage-plan-proof', '03-parent-owned-cloud-sync');
const proofPath = join(outputDir, `${proofMode}.json`);
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
      'tests/unit/sync-export.test.ts',
    ])
  );

  const packageExport = await assertPackageExport();
  const contract = await assertBuiltContract();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/endpoint-domain/src/constants/sync-export.ts',
      contractTest: 'packages/endpoint-domain/tests/unit/sync-export.test.ts',
      builtModule: 'packages/endpoint-domain/dist/constants/sync-export.js',
      packageExport,
      output: relativePath(proofPath),
    },
    routeContracts: contract.routeContracts,
    connectorStatusContracts: contract.connectorStatusContracts,
    nonClaims: [
      'connector OAuth',
      'upload/download runtime',
      'Ocentra-hosted family data custody',
      'account/subscription backend',
      'portal UI',
    ],
    knownGaps: [
      'parent-owned storage connector implementation',
      'real export bundle generation',
      'real sync cursor and retry queue execution',
      'delete/retention controls',
      'product checklist row update by primary-owned lock',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`sync-export-endpoint-contract-proof-ok:${relativePath(proofPath)}`);
}

async function assertPackageExport() {
  const exportedModule = await import('@ocentra-parent/endpoint-domain/constants/sync-export');
  assert.equal(
    exportedModule.ParentOwnedSyncExportBoundaryState.RouteContract,
    'defined'
  );
  return '@ocentra-parent/endpoint-domain/constants/sync-export';
}

async function assertBuiltContract() {
  const modulePath = pathToFileURL(
    join(repoRoot, 'packages', 'endpoint-domain', 'dist', 'constants', 'sync-export.js')
  );
  const module = await import(modulePath.href);

  assert.deepEqual(module.ParentOwnedSyncExportApiPath, {
    ExportManifest: '/api/v1/sync-export/export-manifest',
    ExportStatus: '/api/v1/sync-export/export-status',
    SyncCursor: '/api/v1/sync-export/sync-cursor',
    SyncBatchStatus: '/api/v1/sync-export/sync-batch-status',
    ImportPreview: '/api/v1/sync-export/import-preview',
    DeleteStatus: '/api/v1/sync-export/delete-status',
  });
  assert.deepEqual(module.RemoteConnectorStatusApiPath, {
    StatusSummary: '/api/v1/remote-connectors/status',
    ProviderStatus: '/api/v1/remote-connectors/provider-status',
    RevocationStatus: '/api/v1/remote-connectors/revocation-status',
    ReportCompileStatus: '/api/v1/remote-connectors/report-compile-status',
  });
  assert.deepEqual(module.ParentOwnedSyncExportBoundaryState, {
    RouteContract: 'defined',
    TransferRuntime: 'not-implemented',
    ConnectorOAuth: 'not-implemented',
    OcentraHostedFamilyDataCustody: 'not-supported',
    AccountSubscriptionBackend: 'not-implemented',
    PortalUi: 'not-implemented',
  });

  return {
    routeContracts: Object.keys(module.ParentOwnedSyncExportApiPath),
    connectorStatusContracts: Object.keys(module.RemoteConnectorStatusApiPath),
  };
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
