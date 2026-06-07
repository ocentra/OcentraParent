import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '106-child-ux-runtime-audit-handoff';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-child-ux-runtime-audit-handoff-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-07T05:20:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-child-ux-runtime-audit-handoff',
  'app-game-child-facing-ux',
]);

const contract = await importDist('app-game-child-ux-runtime-audit-handoff.js');
const childUxRules = await importDist('app-game-child-facing-ux-rules.js');
const childUxContract = await importDist('app-game-child-facing-ux.js');
const refs = await importDist('reference-primitives.js');
const handoff = contract.buildAppGameChildUxRuntimeAuditHandoff(
  handoffOptions(refs),
  childUxCards(refs, childUxRules, childUxContract)
);
const proof = {
  proofMode: 'app-game-child-ux-runtime-audit-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(handoff),
  nonClaims: pickNonClaims(handoff),
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-child-ux-runtime-audit-handoff.ts',
    rules: 'packages/parent-domain/src/app-game-child-ux-runtime-audit-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-child-ux-runtime-audit-handoff.test.ts',
    harness: 'scripts/test/app-game-child-ux-runtime-audit-handoff-proof.mjs',
    evidence: 'test-results/app-game-child-ux-runtime-audit-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  handoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), handoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP106');
await writeProofPack(appProofDir, proof, 'app WP106');

console.log('app-game-child-ux-runtime-audit-handoff-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-child-ux-runtime-audit-handoff-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function handoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    childUxRuntimeAuditHandoffId: 'app-game-child-ux-runtime-audit-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-child-facing-ux',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    runtimeAuditProofRefs: ['future-app-game-child-ux-runtime-audit-persistence-proof'],
  };
}

function childUxCards(refs, childUxRules, childUxContract) {
  const device = {
    deviceId: 'device-child-runtime-audit-windows',
    childProfileId: 'child-runtime-audit-profile',
    label: 'Study PC',
    platform: refs.ParentPlatform.Windows,
  };
  const evidenceReference = {
    evidenceReferenceId: 'evidence-child-runtime-audit-policy',
    kind: refs.ParentEvidenceReferenceKind.PolicyDecision,
    observedAt: timestamp,
  };
  const approvalRequestRef = {
    actionReferenceId: 'approval-request-child-runtime-audit',
    actor: {
      actorId: 'child-device-local-agent',
      role: 'system',
    },
    policyVersion: 'policy-child-runtime-audit-v1',
    createdAt: timestamp,
  };
  const readyNativeAppCard = childUxContract.AppGameChildUxCardSchema.parse({
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    childUxStateId: 'child-ux-runtime-audit-ready-app',
    device,
    target: {
      targetKind: childUxRules.AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-study-game',
      childSafeDisplayLabelToken: childUxRules.AppGameChildUxCopyToken.LimitReachedTitle,
    },
    surfaceState: childUxRules.AppGameChildUxSurfaceState.TimeLimitReached,
    capabilityState: childUxRules.AppGameChildUxCapabilityState.Supported,
    claimState: childUxRules.AppGameChildUxClaimState.LimitReached,
    explanationSource: childUxRules.AppGameChildUxExplanationSource.ParentRule,
    titleToken: childUxRules.AppGameChildUxCopyToken.LimitReachedTitle,
    bodyToken: childUxRules.AppGameChildUxCopyToken.LimitReachedBody,
    primaryAction: childUxRules.AppGameChildUxPrimaryAction.RequestMoreTime,
    primaryActionToken: childUxRules.AppGameChildUxCopyToken.RequestMoreTimeAction,
    evidenceReferences: [evidenceReference],
    childReasonReferences: ['child-reason-runtime-audit-limit-reached'],
    childStatusReferences: ['child-status-runtime-audit-limit-reached'],
    approvalRequestRef,
    privateDiagnosticReferences: [],
    adapterActionRef: null,
  });
  return [
    readyNativeAppCard,
    childUxContract.AppGameChildUxCardSchema.parse({
      ...readyNativeAppCard,
      childUxStateId: 'child-ux-runtime-audit-missing-reason-game',
      target: {
        targetKind: childUxRules.AppGameChildUxTargetKind.NativeGame,
        targetRef: 'target-native-game-study',
        childSafeDisplayLabelToken: childUxRules.AppGameChildUxCopyToken.FamilyRuleTitle,
      },
      surfaceState: childUxRules.AppGameChildUxSurfaceState.FamilyRuleWarning,
      claimState: childUxRules.AppGameChildUxClaimState.WarningOnly,
      titleToken: childUxRules.AppGameChildUxCopyToken.FamilyRuleTitle,
      bodyToken: childUxRules.AppGameChildUxCopyToken.FamilyRuleBody,
      primaryAction: childUxRules.AppGameChildUxPrimaryAction.Dismiss,
      primaryActionToken: childUxRules.AppGameChildUxCopyToken.DismissAction,
      childReasonReferences: [],
      approvalRequestRef: null,
    }),
    childUxContract.AppGameChildUxCardSchema.parse({
      ...readyNativeAppCard,
      childUxStateId: 'child-ux-runtime-audit-missing-status-unknown-app',
      target: {
        targetKind: childUxRules.AppGameChildUxTargetKind.UnknownApp,
        targetRef: 'target-unknown-app-review',
        childSafeDisplayLabelToken: childUxRules.AppGameChildUxCopyToken.FamilyRuleTitle,
      },
      surfaceState: childUxRules.AppGameChildUxSurfaceState.FamilyRuleWarning,
      claimState: childUxRules.AppGameChildUxClaimState.WarningOnly,
      titleToken: childUxRules.AppGameChildUxCopyToken.FamilyRuleTitle,
      bodyToken: childUxRules.AppGameChildUxCopyToken.FamilyRuleBody,
      primaryAction: childUxRules.AppGameChildUxPrimaryAction.Dismiss,
      primaryActionToken: childUxRules.AppGameChildUxCopyToken.DismissAction,
      childReasonReferences: ['child-reason-runtime-audit-new-app'],
      childStatusReferences: [],
      approvalRequestRef: null,
    }),
    childUxContract.AppGameChildUxCardSchema.parse({
      ...readyNativeAppCard,
      childUxStateId: 'child-ux-runtime-audit-manual-launcher',
      target: {
        targetKind: childUxRules.AppGameChildUxTargetKind.LauncherGameCandidate,
        targetRef: 'target-launcher-game-candidate',
        childSafeDisplayLabelToken: childUxRules.AppGameChildUxCopyToken.ManualRequiredTitle,
      },
      surfaceState: childUxRules.AppGameChildUxSurfaceState.ManualRequired,
      capabilityState: childUxRules.AppGameChildUxCapabilityState.ManualRequired,
      claimState: childUxRules.AppGameChildUxClaimState.ManualRequired,
      explanationSource: childUxRules.AppGameChildUxExplanationSource.Capability,
      titleToken: childUxRules.AppGameChildUxCopyToken.ManualRequiredTitle,
      bodyToken: childUxRules.AppGameChildUxCopyToken.ManualRequiredBody,
      primaryAction: childUxRules.AppGameChildUxPrimaryAction.TryLater,
      primaryActionToken: childUxRules.AppGameChildUxCopyToken.TryLaterAction,
      approvalRequestRef: null,
    }),
  ];
}

function summarize(handoff) {
  return {
    nativeAppRowCount: handoff.nativeAppRowCount,
    nativeGameRowCount: handoff.nativeGameRowCount,
    runtimeAuditReadyCount: handoff.runtimeAuditReadyCount,
    blockedMissingChildReasonCount: handoff.blockedMissingChildReasonCount,
    blockedMissingChildStatusCount: handoff.blockedMissingChildStatusCount,
    manualRequiredNoAdapterCount: handoff.manualRequiredNoAdapterCount,
  };
}

function pickNonClaims(handoff) {
  return {
    childRuntimeDelivered: handoff.childRuntimeDelivered,
    childRequestUiRendered: handoff.childRequestUiRendered,
    childStatusRuntimePersisted: handoff.childStatusRuntimePersisted,
    runtimeAuditPersisted: handoff.runtimeAuditPersisted,
    adapterDispatchClaimed: handoff.adapterDispatchClaimed,
    platformEnforcementClaimed: handoff.platformEnforcementClaimed,
    privateDiagnosticsExposed: handoff.privateDiagnosticsExposed,
  };
}

function assertProof(proof) {
  if (proof.summary.runtimeAuditReadyCount !== 1) {
    throw new Error('Expected one child UX row to be ready for future runtime audit proof');
  }
  if (
    proof.summary.blockedMissingChildReasonCount !== 1 ||
    proof.summary.blockedMissingChildStatusCount !== 1 ||
    proof.summary.manualRequiredNoAdapterCount !== 1
  ) {
    throw new Error('Expected missing-reason, missing-status, and manual-required/no-adapter rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected child UX runtime audit handoff to avoid child runtime delivery, request UI rendering, status persistence, audit persistence, adapter dispatch, platform enforcement, and private diagnostics'
    );
  }
}

async function writeProofPack(path, proof, label) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      `# ${label} Source Snapshot`,
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Source: packages/parent-domain/src/app-game-child-ux-runtime-audit-handoff.ts',
      '- Test: packages/parent-domain/tests/app-game-child-ux-runtime-audit-handoff.test.ts',
      '',
    ].join('\n')
  );
  await writeFile(
    join(path, '10-validation-commands.log'),
    commands
      .map((command) => `${command.command}\nexit=${command.status}\n${`${command.stdout}${command.stderr}`.trimEnd()}`)
      .join('\n')
  );
  await writeJson(join(path, 'proof.json'), proof);
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

function run(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push({
    command: [command, ...args].join(' '),
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
  });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}
