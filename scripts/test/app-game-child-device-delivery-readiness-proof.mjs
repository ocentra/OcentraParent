import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import { spawn } from 'node:child_process';

const repoRoot = process.cwd();
const proofMode = 'app-game-child-device-delivery-readiness-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '206-app-game-child-device-delivery-readiness');
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
    'app-game-child-facing-ux-child-device-delivery-readiness',
    'app-game-child-facing-ux-local-outbox-provider-status-handoff',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

  const module = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-child-facing-ux-child-device-delivery-readiness.js')
    ).href
  );
  const childUxModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-child-facing-ux.js')).href
  );
  const childUxRulesModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-child-facing-ux-rules.js')).href
  );
  const childUxHandoffModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-child-facing-ux-handoff.js')).href
  );
  const childUxLocalHandoffModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-child-facing-ux-local-handoff.js')).href
  );
  const childUxOutboxModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-child-facing-ux-local-outbox-bridge.js')
    ).href
  );
  const childUxSchedulerModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-child-facing-ux-local-outbox-scheduler-bridge.js')
    ).href
  );
  const childUxProviderPreflightModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-child-facing-ux-local-outbox-provider-preflight.js')
    ).href
  );
  const providerStatusModule = await import(
    pathToFileURL(
      join(
        repoRoot,
        'packages',
        'parent-domain',
        'dist',
        'app-game-child-facing-ux-local-outbox-provider-status-handoff.js'
      )
    ).href
  );

  const sourceReadModel = buildProviderStatusReadModel({
    childUxModule,
    childUxRulesModule,
    childUxHandoffModule,
    childUxLocalHandoffModule,
    childUxOutboxModule,
    childUxSchedulerModule,
    childUxProviderPreflightModule,
    providerStatusModule,
  });
  const readModel = module.buildAppGameChildDeviceDeliveryReadinessReadModel(
    {
      generatedAt: '2026-06-08T22:15:00Z',
      readinessId: 'app-game-child-device-delivery-readiness-proof',
      requiredTransportRefs: [
        'child-runtime-transport-contract-ref',
        'child-runtime-receipt-contract-ref',
        'child-device-local-agent-route-ref',
      ],
    },
    sourceReadModel
  );
  const summary = module.summarizeAppGameChildDeviceDeliveryReadiness(readModel);

  assertEqual(summary.transportRequiredCount, 2, 'transport required count');
  assertEqual(summary.manualRequiredCount, 0, 'manual required count');
  assertEqual(summary.unavailableCount, 1, 'unavailable count');
  assertEqual(readModel.childRuntimeTransportClaimed, false, 'child runtime transport claim');
  assertEqual(readModel.providerDeliveryExecuted, false, 'provider delivery execution claim');
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
      contract: 'packages/parent-domain/src/app-game-child-facing-ux-child-device-delivery-readiness.ts',
      contractTest: 'packages/parent-domain/tests/app-game-child-facing-ux-child-device-delivery-readiness.test.ts',
      providerStatusContract:
        'packages/parent-domain/src/app-game-child-facing-ux-local-outbox-provider-status-handoff.ts',
      providerStatusTest:
        'packages/parent-domain/tests/app-game-child-facing-ux-local-outbox-provider-status-handoff.test.ts',
    },
    claimsProved: [
      'Scheduled app/game child UX provider-status rows are promoted only to child-transport-required readiness rows',
      'Manual-required and unavailable source rows stay out of transport-required readiness',
      'The read model carries parent-safe transport references without raw child payload rows',
      'Child runtime transport, receipt ingestion, provider delivery execution, platform delivery channel, adapter dispatch, and platform enforcement remain unclaimed',
    ],
    claimsNotProved: [
      'Child runtime transport attachment',
      'Child runtime delivery receipt ingestion',
      'Provider delivery execution',
      'Platform push, overlay, notification, or OS-level delivery channel execution',
      'Adapter dispatch or platform enforcement',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(
    join(appGameProofDir, '00-source-snapshot.md'),
    [
      '# WP206 app/game child-device delivery readiness source snapshot',
      '',
      '- Branch: codex/app-game-control-product-completion',
      '- Commit: uncommitted full-goal batch, validated by harness before final checkpoint commit',
      '- Parent read model: packages/parent-domain/src/app-game-child-facing-ux-child-device-delivery-readiness.ts',
      '- Source read model: packages/parent-domain/src/app-game-child-facing-ux-local-outbox-provider-status-handoff.ts',
      '',
      'Evidence:',
      '- Scheduled provider-status rows become child-transport-required readiness rows.',
      '- Manual-required and unavailable source rows remain blocked or unavailable.',
      '- Runtime child transport, receipts, provider delivery execution, adapter dispatch, and platform enforcement stay unclaimed.',
      '',
    ].join('\n')
  );
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-child-device-delivery-readiness-proof-ok');
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

function buildProviderStatusReadModel(modules) {
  const rules = modules.childUxRulesModule;
  const timestamp = '2026-06-08T22:15:00Z';
  const baseCard = {
    schemaVersion: 'v0.6',
    childUxStateId: 'child-ux-limit-reached-child-delivery-readiness',
    device: {
      deviceId: 'device-child-delivery-readiness',
      childProfileId: 'child-profile-child-delivery-readiness',
      label: 'Study PC',
      platform: 'windows',
    },
    target: {
      targetKind: rules.AppGameChildUxTargetKind.NativeGame,
      targetRef: 'target-native-game-child-delivery-readiness',
      childSafeDisplayLabelToken: rules.AppGameChildUxCopyToken.LimitReachedTitle,
    },
    surfaceState: rules.AppGameChildUxSurfaceState.TimeLimitReached,
    capabilityState: rules.AppGameChildUxCapabilityState.Supported,
    claimState: rules.AppGameChildUxClaimState.LimitReached,
    explanationSource: rules.AppGameChildUxExplanationSource.ParentRule,
    titleToken: rules.AppGameChildUxCopyToken.LimitReachedTitle,
    bodyToken: rules.AppGameChildUxCopyToken.LimitReachedBody,
    primaryAction: rules.AppGameChildUxPrimaryAction.RequestMoreTime,
    primaryActionToken: rules.AppGameChildUxCopyToken.RequestMoreTimeAction,
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-child-delivery-readiness',
        kind: 'policy-decision',
        observedAt: timestamp,
      },
    ],
    childReasonReferences: ['child-reason-limit-reached-child-delivery-readiness'],
    childStatusReferences: ['child-status-limit-reached-child-delivery-readiness'],
    approvalRequestRef: {
      actionReferenceId: 'approval-request-child-delivery-readiness',
      actor: {
        actorId: 'child-device-local-agent',
        role: 'system',
      },
      policyVersion: 'policy-child-delivery-readiness-v1',
      createdAt: timestamp,
    },
    privateDiagnosticReferences: [],
    adapterActionRef: null,
  };
  const nativeAppCard = {
    ...baseCard,
    childUxStateId: 'child-ux-native-app-request-submitted-child-delivery-readiness',
    target: {
      targetKind: rules.AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-delivery-readiness',
      childSafeDisplayLabelToken: rules.AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: rules.AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: rules.AppGameChildUxClaimState.RequestSubmitted,
    titleToken: rules.AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: rules.AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: rules.AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: rules.AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-child-delivery-readiness'],
    childStatusReferences: ['child-status-request-submitted-child-delivery-readiness'],
    approvalRequestRef: null,
  };
  const unavailableCard = {
    ...baseCard,
    childUxStateId: 'child-ux-unavailable-child-delivery-readiness',
    surfaceState: rules.AppGameChildUxSurfaceState.Unavailable,
    capabilityState: rules.AppGameChildUxCapabilityState.Unavailable,
    claimState: rules.AppGameChildUxClaimState.Unavailable,
    titleToken: rules.AppGameChildUxCopyToken.UnavailableTitle,
    bodyToken: rules.AppGameChildUxCopyToken.UnavailableBody,
    primaryAction: rules.AppGameChildUxPrimaryAction.TryLater,
    primaryActionToken: rules.AppGameChildUxCopyToken.TryLaterAction,
    approvalRequestRef: null,
  };

  const cards = [baseCard, nativeAppCard, unavailableCard].map((card) =>
    modules.childUxModule.AppGameChildUxCardSchema.parse(card)
  );
  const handoff = modules.childUxHandoffModule.buildAppGameChildUxHandoffReadModel(
    {
      generatedAt: timestamp,
      handoffId: 'app-game-child-delivery-readiness-handoff',
      localHandoffRootRef: 'child-device-delivery-readiness-handoff-root',
    },
    cards
  );
  const artifacts = modules.childUxLocalHandoffModule.buildAppGameChildUxLocalHandoffArtifactReadModel(
    {
      generatedAt: timestamp,
      localArtifactRootRef: 'child-device-delivery-readiness-artifact-root',
      localArtifactFileRef: 'child-device-delivery-readiness-artifact-jsonl',
    },
    handoff
  );
  const outbox = modules.childUxOutboxModule.buildAppGameChildUxLocalOutboxBridgeReadModel(
    {
      family: { familyId: 'family-child-delivery-readiness' },
      parentAction: {
        actionReferenceId: 'parent-action-child-delivery-readiness',
        actor: {
          actorId: 'parent-child-delivery-readiness',
          role: 'parent',
        },
        policyVersion: 'policy-child-delivery-readiness-v1',
        createdAt: timestamp,
      },
      generatedAt: timestamp,
      bridgeId: 'app-game-child-delivery-readiness-source-bridge-proof',
      outboxRootRef: 'parent-owned-child-delivery-readiness-root',
      outboxFileRef: 'parent-owned-child-delivery-readiness-jsonl-ref',
      localDataPathRef: 'parent-owned-child-delivery-readiness-data-path-ref',
    },
    artifacts
  );
  const scheduler = modules.childUxSchedulerModule.buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel(
    {
      generatedAt: timestamp,
      schedulerBridgeId: 'app-game-child-delivery-readiness-scheduler-bridge-proof',
      schedulerArtifactRootRef: 'parent-owned-child-delivery-readiness-scheduler-root-ref',
      schedulerArtifactRef: 'parent-owned-child-delivery-readiness-scheduler-jsonl-ref',
      schedulerNowAt: timestamp,
    },
    outbox
  );
  const preflight = modules.childUxProviderPreflightModule.buildAppGameChildUxLocalOutboxProviderPreflightReadModel(
    {
      generatedAt: timestamp,
      providerPreflightId: 'app-game-child-delivery-readiness-provider-preflight-proof',
      sourceContractRefs: [
        'app-game-child-ux-local-outbox-scheduler-bridge',
        'notification-local-outbox-scheduler-proof',
        'notification-provider-adapter-boundary-required',
      ],
    },
    scheduler
  );
  return modules.providerStatusModule.buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel(
    {
      generatedAt: timestamp,
      handoffId: 'app-game-child-delivery-readiness-provider-status-handoff',
      sourceContractRefs: [
        'app-game-child-ux-local-outbox-provider-preflight',
        'v0-8-notification-provider-status-boundary',
      ],
    },
    preflight
  );
}
