import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-child-runtime-transport-receipt-boundary-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '208-app-game-child-runtime-transport-receipt-boundary'
);
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });

  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-child-facing-ux-child-runtime-transport-receipt-boundary',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

  const writerModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-child-facing-ux-child-device-runtime-writer.js')
    ).href
  );
  const boundaryModule = await import(
    pathToFileURL(
      join(
        repoRoot,
        'packages',
        'parent-domain',
        'dist',
        'app-game-child-facing-ux-child-runtime-transport-receipt-boundary.js'
      )
    ).href
  );
  const sourceWriter = writerModule.AppGameChildDeviceRuntimeWriterReadModelSchema.parse(runtimeWriterFixture());
  const readModel = boundaryModule.buildAppGameChildRuntimeTransportReceiptBoundaryReadModel(
    {
      generatedAt: '2026-06-08T23:05:00Z',
      boundaryId: 'app-game-child-runtime-transport-receipt-boundary-proof',
      receiptContractRefs: [
        'child-runtime-delivery-receipt-contract-ref',
        'child-runtime-delivery-receipt-storage-ref',
      ],
    },
    sourceWriter
  );
  const summary = boundaryModule.summarizeAppGameChildRuntimeTransportReceiptBoundary(readModel);

  assertEqual(summary.transportRequiredCount, 2, 'transport required count');
  assertEqual(summary.manualRequiredCount, 1, 'manual required count');
  assertEqual(summary.unavailableCount, 1, 'unavailable count');
  assertEqual(readModel.runtimeTransportExecuted, false, 'runtime transport execution claim');
  assertEqual(readModel.runtimeReceiptIngested, false, 'runtime receipt ingestion claim');
  assertEqual(readModel.platformDeliveryChannelClaimed, false, 'platform delivery channel claim');

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    readModel,
    summary,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-child-facing-ux-child-runtime-transport-receipt-boundary.ts',
      contractTest:
        'packages/parent-domain/tests/app-game-child-facing-ux-child-runtime-transport-receipt-boundary.test.ts',
      sourceRuntimeWriterContract:
        'packages/parent-domain/src/app-game-child-facing-ux-child-device-runtime-writer.ts',
      sourceRuntimeWriterProof: 'test-results/app-game-child-device-runtime-writer-proof/proof.json',
    },
    claimsProved: [
      'Writer-envelope-ready rows can become child-runtime-transport-required receipt boundary rows',
      'Manual-required and unavailable writer rows remain blocked before runtime transport execution',
      'Receipt contract refs are represented without claiming receipt ingestion',
      'Runtime transport execution, receipt ingestion, provider delivery, platform delivery channel, adapter dispatch, platform enforcement, and raw private source rows remain unclaimed',
    ],
    claimsNotProved: [
      'Child runtime transport execution',
      'Child runtime receipt ingestion',
      'Provider delivery execution',
      'Platform delivery channel execution',
      'Adapter dispatch or platform enforcement',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(
    join(appGameProofDir, '00-source-snapshot.md'),
    [
      '# WP208 app/game child runtime transport receipt boundary source snapshot',
      '',
      '- Branch: codex/app-game-control-product-completion',
      '- Commit: uncommitted full-goal batch, validated by harness before final checkpoint commit',
      '- Parent read model: packages/parent-domain/src/app-game-child-facing-ux-child-runtime-transport-receipt-boundary.ts',
      '- Source writer model: packages/parent-domain/src/app-game-child-facing-ux-child-device-runtime-writer.ts',
      '',
      'Evidence:',
      '- Writer-envelope-ready rows become child-runtime-transport-required boundary rows.',
      '- Manual-required and unavailable rows remain non-executable.',
      '- Runtime transport execution, receipt ingestion, provider delivery, adapter dispatch, and platform enforcement stay unclaimed.',
      '',
    ].join('\n')
  );
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-child-runtime-transport-receipt-boundary-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${commandLine} exited with ${code}`))));
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

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function runtimeWriterFixture() {
  return {
    schemaVersion: 'v0.6',
    runtimeWriterId: 'app-game-child-device-runtime-writer-proof',
    generatedAt: '2026-06-08T23:05:00Z',
    family: { familyId: 'family-child-runtime-transport-receipt' },
    sourceDeliveryReadinessId: 'app-game-child-device-delivery-readiness-proof',
    rows: [
      writerRow('limit-reached', 'writer-envelope-ready'),
      writerRow('request-submitted', 'writer-envelope-ready'),
      writerRow('manual-required', 'manual-required'),
      writerRow('unavailable', 'unavailable'),
    ],
    writerEnvelopeReadyCount: 2,
    manualRequiredCount: 1,
    unavailableCount: 1,
    nonClaims: [
      'no-runtime-writer-execution',
      'no-child-runtime-transport',
      'no-child-runtime-receipt-ingestion',
      'no-provider-delivery-execution',
      'no-platform-delivery-channel',
      'no-adapter-dispatch',
      'no-platform-enforcement',
      'no-raw-private-source-rows',
    ],
    runtimeWriterExecuted: false,
    childRuntimeTransportAttached: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  };
}

function writerRow(suffix, writerEnvelopeState) {
  return {
    runtimeWriterRowId: `app-game-child-device-runtime-writer-${suffix}`,
    sourceDeliveryReadinessRowId: `app-game-child-device-delivery-readiness-${suffix}`,
    sourceDeliveryReadinessStatus:
      writerEnvelopeState === 'writer-envelope-ready' ? 'child-transport-required' : writerEnvelopeState,
    writerEnvelopeState,
    childDeliveryTargetRefs:
      writerEnvelopeState === 'writer-envelope-ready'
        ? [
            'child-runtime-transport-contract-ref',
            'child-runtime-receipt-contract-ref',
            'child-device-local-agent-route-ref',
          ]
        : [writerEnvelopeState === 'unavailable' ? 'source-unavailable' : 'manual-proof-required'],
    runtimeWriterAuditRefs: [`app-game-child-device-runtime-writer-audit-${suffix}`],
    runtimeWriterExecuted: false,
    childRuntimeTransportAttached: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  };
}
