import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const proofMode = 'app-game-child-device-runtime-writer-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '207-app-game-child-device-runtime-writer');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/app-game-domain',
      '--',
      '--run',
      'tests/unit/app-game-child-facing-ux-child-device-runtime-writer.test.ts',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']));

  const schemaReadinessModule =
    await import('@ocentra-parent/schema-domain/app-game-child-facing-ux-child-device-delivery-readiness');
  commands.push('node import @ocentra-parent/schema-domain/app-game-child-facing-ux-child-device-delivery-readiness');
  const schemaWriterModule =
    await import('@ocentra-parent/schema-domain/app-game-child-facing-ux-child-device-runtime-writer');
  commands.push('node import @ocentra-parent/schema-domain/app-game-child-facing-ux-child-device-runtime-writer');
  if (!('AppGameChildDeviceDeliveryReadinessReadModelSchema' in schemaReadinessModule)) {
    throw new Error('Missing AppGameChildDeviceDeliveryReadinessReadModelSchema export from schema-domain');
  }
  if (!('AppGameChildDeviceRuntimeWriterReadModelSchema' in schemaWriterModule)) {
    throw new Error('Missing AppGameChildDeviceRuntimeWriterReadModelSchema export from schema-domain');
  }

  const readinessModule = schemaReadinessModule;
  const writerModule = schemaWriterModule;
  const sourceReadiness =
    readinessModule.AppGameChildDeviceDeliveryReadinessReadModelSchema.parse(deliveryReadinessFixture());
  const readModel = writerModule.buildAppGameChildDeviceRuntimeWriterReadModel(
    {
      generatedAt: '2026-06-08T22:45:00Z',
      runtimeWriterId: 'app-game-child-device-runtime-writer-proof',
    },
    sourceReadiness
  );
  const summary = writerModule.summarizeAppGameChildDeviceRuntimeWriter(readModel);

  assertEqual(summary.writerEnvelopeReadyCount, 2, 'writer envelope ready count');
  assertEqual(summary.manualRequiredCount, 1, 'manual required count');
  assertEqual(summary.unavailableCount, 1, 'unavailable count');
  assertEqual(readModel.runtimeWriterExecuted, false, 'runtime writer execution claim');
  assertEqual(readModel.childRuntimeTransportAttached, false, 'child runtime transport claim');
  assertEqual(readModel.childRuntimeReceiptIngested, false, 'child receipt claim');

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    readModel,
    summary,
    evidence: {
      readinessOwner: 'packages/schema-domain/src/app-game-child-facing-ux-child-device-delivery-readiness.ts',
      writerOwner: 'packages/schema-domain/src/app-game-child-facing-ux-child-device-runtime-writer.ts',
      consumerTest: 'packages/app-game-domain/tests/unit/app-game-child-facing-ux-child-device-runtime-writer.test.ts',
    },
    claimsProved: [
      'schema-domain owns the child-device delivery readiness and child-device runtime writer contract surfaces',
      'Transport-required child-device delivery readiness rows can become runtime writer envelopes',
      'Manual-required and unavailable readiness rows remain blocked or unavailable',
      'Runtime writer rows carry only parent-safe target and audit references',
      'Runtime writer execution, child runtime transport, receipt ingestion, provider delivery, platform delivery channel, adapter dispatch, platform enforcement, and raw private source rows remain unclaimed',
    ],
    claimsNotProved: [
      'Runtime writer process execution',
      'Child runtime transport attachment',
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
      '# WP207 app/game child-device runtime writer source snapshot',
      '',
      '- Branch: codex/app-game-control-product-completion',
      '- Commit: uncommitted full-goal batch, validated by harness before final checkpoint commit',
      '- Readiness owner: packages/schema-domain/src/app-game-child-facing-ux-child-device-delivery-readiness.ts',
      '- Writer owner: packages/schema-domain/src/app-game-child-facing-ux-child-device-runtime-writer.ts',
      '',
      'Evidence:',
      '- schema-domain exports the child-device delivery readiness and child-device runtime writer owners.',
      '- Transport-required readiness rows become writer-envelope-ready rows.',
      '- Manual-required and unavailable rows remain non-executable.',
      '- Runtime execution, child transport, receipts, provider delivery, adapter dispatch, and platform enforcement stay unclaimed.',
      '',
    ].join('\n')
  );
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-child-device-runtime-writer-proof-ok');
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

function deliveryReadinessFixture() {
  return {
    schemaVersion: 'v0.6',
    readinessId: 'app-game-child-device-delivery-readiness-proof',
    generatedAt: '2026-06-08T22:45:00Z',
    family: { familyId: 'family-child-delivery-runtime-writer' },
    sourceProviderStatusHandoffId: 'app-game-child-delivery-readiness-provider-status-handoff',
    rows: [
      readinessRow('limit-reached', 'child-transport-required'),
      readinessRow('request-submitted', 'child-transport-required'),
      readinessRow('manual-required', 'manual-required'),
      readinessRow('unavailable', 'unavailable'),
    ],
    transportRequiredCount: 2,
    manualRequiredCount: 1,
    unavailableCount: 1,
    nonClaims: [
      'no-child-runtime-transport',
      'no-child-runtime-receipt-ingestion',
      'no-provider-delivery-execution',
      'no-platform-delivery-channel',
      'no-adapter-dispatch',
      'no-platform-enforcement',
      'no-raw-private-source-rows',
    ],
    childRuntimeTransportClaimed: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  };
}

function readinessRow(suffix, deliveryReadinessStatus) {
  return {
    deliveryReadinessRowId: `app-game-child-device-delivery-readiness-${suffix}`,
    sourceProviderStatusHandoffRowId: `app-game-child-ux-provider-status-handoff-${suffix}`,
    sourceProviderStatus: deliveryReadinessStatus === 'unavailable' ? 'unavailable' : 'manual-required',
    sourceOutboxRecordRef:
      deliveryReadinessStatus === 'child-transport-required' ? `app-game-child-ux-local-outbox-${suffix}` : null,
    sourceSchedulerEntryRef:
      deliveryReadinessStatus === 'child-transport-required'
        ? `app-game-child-ux-local-outbox-scheduler-${suffix}`
        : null,
    deliveryReadinessStatus,
    requiredTransportRefs:
      deliveryReadinessStatus === 'child-transport-required'
        ? [
            'child-runtime-transport-contract-ref',
            'child-runtime-receipt-contract-ref',
            'child-device-local-agent-route-ref',
          ]
        : ['manual-proof-required'],
    openGaps: openGapsFor(deliveryReadinessStatus),
    childRuntimeTransportClaimed: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  };
}

function openGapsFor(status) {
  if (status === 'unavailable') {
    return ['source-unavailable', 'child-runtime-transport-not-attached'];
  }
  if (status === 'manual-required') {
    return ['manual-proof-required', 'child-runtime-transport-not-attached'];
  }
  return [
    'child-runtime-transport-not-attached',
    'child-runtime-receipt-not-ingested',
    'provider-delivery-not-executed',
    'platform-delivery-channel-not-proved',
  ];
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
