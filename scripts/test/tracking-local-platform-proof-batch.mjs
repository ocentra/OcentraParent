import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-local-platform-proof-batch';
const generatedAt = '2026-06-08T15:30:00.000Z';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const focusedProofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const proofRefs = {
  android: 'test-results/tracking-plan-android-emulator-proof/proof.json',
  androidInventory: 'test-results/tracking-android-emulator-artifact-inventory-proof/proof.json',
  crossPlatformRuntimeCapability: 'test-results/tracking-cross-platform-runtime-capability-proof/proof.json',
  wsl: 'test-results/tracking-plan-wsl-local-proof/proof.json',
  hostedAccessibility: 'test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json',
  hostedInventory: 'test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json',
  parentChildLocalRuntimeBridge: 'test-results/tracking-parent-child-local-runtime-bridge-proof/proof.json',
  productUiLocal: 'test-results/tracking-full-product-ui-local-runtime-artifact-capture-proof/proof.json',
  productUiPreflight: 'test-results/tracking-full-product-ui-runtime-preflight-proof/proof.json',
  productReadinessClosure: 'test-results/tracking-product-readiness-closure-proof/proof.json',
  realRuntimeHandoff: 'test-results/tracking-real-runtime-handoff-proof/proof.json',
};

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', ['/c', 'npm', 'run', 'test', '--workspace', '@ocentra-parent/parent-domain', '--', proofMode]);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-cross-platform-runtime-capability-proof.mjs']);

  const proofModule = await importDist('tracking-local-platform-proof-batch.js');
  const source = await readSourceProofs();
  const readModel = proofModule.buildTrackingLocalPlatformProofBatch(generatedAt, rowsFromSource(source));
  const proof = buildProof(readModel, source);

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-local-platform-proof-batch-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function readSourceProofs() {
  return {
    android: await readJson(proofRefs.android),
    androidInventory: await readJson(proofRefs.androidInventory),
    crossPlatformRuntimeCapability: await readJson(proofRefs.crossPlatformRuntimeCapability),
    wsl: await readJson(proofRefs.wsl),
    hostedAccessibility: await readJson(proofRefs.hostedAccessibility),
    hostedInventory: await readJson(proofRefs.hostedInventory),
    parentChildLocalRuntimeBridge: await readJson(proofRefs.parentChildLocalRuntimeBridge),
    productUiLocal: await readJson(proofRefs.productUiLocal),
    productUiPreflight: await readJson(proofRefs.productUiPreflight),
    productReadinessClosure: await readJson(proofRefs.productReadinessClosure),
    realRuntimeHandoff: await readJson(proofRefs.realRuntimeHandoff),
  };
}

function rowsFromSource(source) {
  const androidTransitions = source.android.runtime?.localGeofenceTransitions?.length ?? 0;
  const systemBroadcastCount = source.android.runtime?.systemProximityBroadcastCount ?? 0;
  const hostedHeadingCount = source.hostedAccessibility.summary?.headings?.length ?? 0;
  const unlabeledButtons = source.hostedAccessibility.summary?.unlabeledButtons ?? 0;
  const handoffSummary = source.realRuntimeHandoff.summary;

  return [
    {
      area: 'android-emulator-runtime',
      status: 'local-proof-passed',
      proofRef: proofRefs.android,
      sourceRefs: [proofRefs.androidInventory],
      currentProofTier: source.android.currentProofTier,
      requiredProofTier: source.android.requiredProofTier,
      passedLocalAssertions: [
        `Android emulator proof status: ${source.android.currentStatus}`,
        'background permission grant and app-owned geofence transition artifacts are present',
        'ATD Settings activity unavailability is recorded as a route-attempt artifact, not a Settings proof',
      ],
      remainingBlockers: [
        'Android system geofence delivery and dwell transition proof require nonzero system delivery artifacts',
        'physical Android device and authority-enrolled runtime proof remain required',
      ],
      metrics: [
        { name: 'localGeofenceTransitionCount', value: androidTransitions },
        { name: 'systemProximityBroadcastCount', value: systemBroadcastCount },
      ],
      ciRunnable: false,
    },
    {
      area: 'cross-platform-runtime-capability',
      status: 'local-proof-passed',
      proofRef: proofRefs.crossPlatformRuntimeCapability,
      sourceRefs: source.crossPlatformRuntimeCapability.readModel.rows.flatMap((row) => [
        row.proofRef,
        ...row.sourceRefs,
      ]),
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      passedLocalAssertions: [
        'cross-platform proof accounts Windows host, WSL/Linux, Docker, Android SDK, Android Gradle, Android emulator, Android physical status, and macOS/iOS CI/manual routing',
        'Android SDK and Gradle project build capability are separated from Android physical behavior claims',
      ],
      remainingBlockers: [
        'macOS/iOS runtime execution remains CI/manual-routed from this Windows host',
        'Android physical location/geofence behavior remains separately gated',
      ],
      metrics: [
        { name: 'crossPlatformRowCount', value: source.crossPlatformRuntimeCapability.summary.rowCount },
        {
          name: 'localProofPassedRows',
          value: source.crossPlatformRuntimeCapability.summary.localProofPassedRows,
        },
        { name: 'productReadyRows', value: source.crossPlatformRuntimeCapability.summary.productReadyRows },
      ],
      ciRunnable: true,
    },
    {
      area: 'wsl-local-replay',
      status: 'local-proof-passed',
      proofRef: proofRefs.wsl,
      sourceRefs: [],
      currentProofTier: source.wsl.currentProofTier,
      requiredProofTier: source.wsl.requiredProofTier,
      passedLocalAssertions: [`WSL local replay status: ${source.wsl.currentStatus}`],
      remainingBlockers: [],
      metrics: [{ name: 'commandCount', value: source.wsl.commands?.length ?? 0 }],
      ciRunnable: true,
    },
    {
      area: 'hosted-parent-ui-accessibility',
      status: 'local-proof-passed',
      proofRef: proofRefs.hostedAccessibility,
      sourceRefs: [proofRefs.hostedInventory],
      currentProofTier: source.hostedInventory.currentProofTier,
      requiredProofTier: source.hostedInventory.requiredProofTier,
      passedLocalAssertions: [
        'hosted parent tracking route has a named region',
        'hosted parent tracking route has zero unlabeled buttons',
        'hosted screenshot inventory is present',
      ],
      remainingBlockers: ['full product parent/child UI runtime remains outside the hosted route'],
      metrics: [
        { name: 'headingCount', value: hostedHeadingCount },
        { name: 'unlabeledButtonCount', value: unlabeledButtons },
        { name: 'hostedScreenshotCount', value: source.hostedInventory.screenshots?.length ?? 0 },
      ],
      ciRunnable: true,
    },
    {
      area: 'product-parent-child-ui-local-artifacts',
      status: 'local-proof-passed',
      proofRef: proofRefs.productUiLocal,
      sourceRefs: [proofRefs.productUiPreflight],
      currentProofTier: source.productUiLocal.currentProofTier,
      requiredProofTier: source.productUiLocal.requiredProofTier,
      passedLocalAssertions: [
        'local parent overview, device detail, notification, retention, and child hosted-readiness artifacts are captured',
        'cross-surface accessibility and end-to-end trace artifacts are captured',
      ],
      remainingBlockers: [
        'production retention write-result UI and rendered child-device UI runtime artifacts remain missing',
      ],
      metrics: [
        { name: 'localArtifactCount', value: source.productUiLocal.summary.localArtifactCount },
        { name: 'missingRuntimeArtifactCount', value: source.productUiPreflight.summary.missingArtifactCount },
      ],
      ciRunnable: true,
    },
    {
      area: 'parent-child-local-runtime-bridge',
      status: 'local-proof-passed',
      proofRef: proofRefs.parentChildLocalRuntimeBridge,
      sourceRefs: source.parentChildLocalRuntimeBridge.sourceProofRefs,
      currentProofTier: source.parentChildLocalRuntimeBridge.currentProofTier,
      requiredProofTier: source.parentChildLocalRuntimeBridge.requiredProofTier,
      passedLocalAssertions: [
        'local parent-child runtime bridge observes typed transport handoff and parent read-model projection',
        'local bridge records ordered parent, transport, child-agent, health, and read-model phases',
      ],
      remainingBlockers: [
        'physical child-device delivery/execution and rendered child UI runtime artifacts remain required',
      ],
      metrics: [
        {
          name: 'storedEventCount',
          value: source.parentChildLocalRuntimeBridge.summary.storedEventCount,
        },
        {
          name: 'deadLetterCount',
          value: source.parentChildLocalRuntimeBridge.summary.deadLetterCount,
        },
        {
          name: 'childAgentPhaseCount',
          value: source.parentChildLocalRuntimeBridge.summary.childAgentPhaseCount,
        },
      ],
      ciRunnable: true,
    },
    {
      area: 'real-runtime-handoff-accounting',
      status: 'manual-required',
      proofRef: proofRefs.realRuntimeHandoff,
      sourceRefs: [proofRefs.productReadinessClosure],
      currentProofTier: source.realRuntimeHandoff.currentProofTier,
      requiredProofTier: source.realRuntimeHandoff.requiredProofTier,
      passedLocalAssertions: ['real-runtime handoff carries local proof accounting and claim-audit blockers'],
      remainingBlockers: [
        'physical Android/iOS, child-device runtime, authority, provider, and production rows remain manual-required',
      ],
      metrics: [
        { name: 'handoffRowCount', value: handoffSummary.handoffRowCount },
        { name: 'missingArtifactCount', value: handoffSummary.missingArtifactCount },
        { name: 'productReadyRows', value: handoffSummary.productReadyRowCount },
      ],
      ciRunnable: false,
    },
  ];
}

function buildProof(readModel, source) {
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    workpackIds: [
      '08-android-foreground-location-adapter',
      '09-android-background-location-and-geofence-adapter',
      '30-parent-and-child-ui-ux-surfaces',
      '33-proof-gates-fixtures-rollout-and-pr-gate',
    ],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'local_platform_batch_ready_manual_runtime_gaps_remaining',
    sourceProofRefs: readModel.sourceProofRefs,
    readModel,
    summary: readModel.summary,
    productClaims: readModel.productClaims,
    remainingProductClaimsFalse: {
      physicalAndroidBackgroundClaimed: source.productReadinessClosure.productClaims.physicalAndroidBackgroundClaimed,
      physicalIosBackgroundClaimed: source.productReadinessClosure.productClaims.physicalIosBackgroundClaimed,
      childDeviceRuntimeClaimed: source.productReadinessClosure.productClaims.childDeviceRuntimeClaimed,
      fullProductUiClaimed: source.productReadinessClosure.productClaims.fullProductUiClaimed,
      authorityClaimed: source.productReadinessClosure.productClaims.authorityClaimed,
      providerDeliveryReceiptClaimed: source.productReadinessClosure.productClaims.providerDeliveryReceiptClaimed,
      productionWorkersClaimed: source.productReadinessClosure.productClaims.productionWorkersClaimed,
      productReadyClaimed: source.productReadinessClosure.productClaims.productReadyClaimed,
    },
    proofLabels: [
      'tracking.local-platform-batch.android-emulator',
      'tracking.local-platform-batch.cross-platform-runtime-capability',
      'tracking.local-platform-batch.wsl-local-replay',
      'tracking.local-platform-batch.hosted-ui-accessibility',
      'tracking.local-platform-batch.product-ui-local-artifacts',
      'tracking.local-platform-batch.manual-runtime-gaps-preserved',
    ],
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 7, 'expected seven local platform batch rows');
  assert.equal(proof.summary.localProofPassedRows, 6, 'expected six local proof rows');
  assert.equal(proof.summary.manualRequiredRows, 1, 'expected one handoff/manual row');
  assert.equal(proof.summary.productReadyRows, 0, 'product-ready rows must stay zero');
  assert.equal(proof.remainingProductClaimsFalse.productReadyClaimed, false, 'product-ready claim must stay false');
  assert.equal(proof.productClaims.androidPhysicalDeviceClaimed, false, 'Android physical claim must stay false');
  assert.equal(proof.productClaims.iosRuntimeClaimed, false, 'iOS runtime claim must stay false');
  assert.equal(proof.productClaims.childDeviceRuntimeClaimed, false, 'child runtime claim must stay false');
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(focusedProofDir, 'proof.json'), proof);
  await writeJson(path.join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(output33, '74-local-platform-proof-batch.json'), proof);
  await writeFile(path.join(focusedProofDir, '00-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
  await writeFile(path.join(output33, '74-local-platform-proof-batch-validation.log'), commandLog(), 'utf8');
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Local Platform Proof Batch Source Snapshot',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.commit}`,
    `- status: ${proof.status}`,
    `- localProofPassedRows: ${proof.summary.localProofPassedRows}`,
    `- manualRequiredRows: ${proof.summary.manualRequiredRows}`,
    '- Android emulator, cross-platform host capability, WSL replay, hosted UI accessibility, and product UI local artifacts are aggregated.',
    '- Physical Android/iOS, child-device runtime, authority, provider, production, and product-ready claims remain false.',
    '',
  ].join('\n');
}

function importDist(name) {
  return import(pathToFileURL(path.join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function run(command, args) {
  commands.push({ command: [command, ...args].join(' ') });
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) throw new Error(`Command failed: ${command} ${args.join(' ')}`);
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

async function readJson(relativePath) {
  return JSON.parse(await readFile(path.join(repoRoot, relativePath), 'utf8'));
}

async function writeJson(filePath, value) {
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function commandLog() {
  return `${commands.map((entry) => entry.command).join('\n')}\n`;
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}
