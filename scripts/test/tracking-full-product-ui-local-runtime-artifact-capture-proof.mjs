import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { copyFile, mkdir, readFile, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-full-product-ui-local-runtime-artifact-capture-proof';
const generatedAt = '2026-06-08T04:35:00.000Z';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const focusedProofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const productRuntimeRoot = path.join(repoRoot, 'output', 'tracking-plan-proof', 'product-parent-child-ui-runtime');
const output30 = path.join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const retentionWritableExecutionProofRef =
  'output/tracking-plan-proof/tracking-retention-product-settings-writable-execution-proof/proof.json';
const childRuntimeArtifactGateProofRef =
  'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/50-child-runtime-artifact-gate-proof.json';

const sourceProofRefs = [
  'test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json',
  'test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json',
  retentionWritableExecutionProofRef,
  childRuntimeArtifactGateProofRef,
];

const copiedScreenshotCaptures = [
  {
    artifactId: 'parent-overview-runtime-ui',
    sourceArtifactRef:
      'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-parent-overview-shell.png',
    outputArtifactRef: 'output/tracking-plan-proof/product-parent-child-ui-runtime/01-parent-overview-runtime.png',
  },
  {
    artifactId: 'parent-device-detail-runtime-ui',
    sourceArtifactRef:
      'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-parent-devices-shell.png',
    outputArtifactRef: 'output/tracking-plan-proof/product-parent-child-ui-runtime/02-parent-device-detail-runtime.png',
  },
  {
    artifactId: 'parent-notification-history-preferences-runtime',
    sourceArtifactRef:
      'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-notification-parent-surface.png',
    outputArtifactRef:
      'output/tracking-plan-proof/product-parent-child-ui-runtime/03-parent-notification-history-preferences-runtime.png',
  },
];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(productRuntimeRoot, { recursive: true });
  await mkdir(output30, { recursive: true });
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
    'tracking-full-product-ui-local-runtime-artifact-capture-proof',
  ]);

  const proofModule = await importDist('tracking-full-product-ui-local-runtime-artifact-capture-proof.js');
  const closureEvidenceInput = await readClosureEvidenceInput();
  const captures = [
    ...(await Promise.all(copiedScreenshotCaptures.map(copyScreenshotArtifact))),
    await writeAccessibilityArtifact(),
    await writeEndToEndTraceArtifact(closureEvidenceInput),
  ];
  const readModel = proofModule.buildTrackingFullProductUiLocalRuntimeArtifactCaptureProof(
    generatedAt,
    sourceProofRefs,
    captures,
    closureEvidenceInput
  );
  const proof = buildProof(readModel);

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-full-product-ui-local-runtime-artifact-capture-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function copyScreenshotArtifact(capture) {
  const sourcePath = path.join(repoRoot, capture.sourceArtifactRef);
  const outputPath = path.join(repoRoot, capture.outputArtifactRef);
  await mkdir(path.dirname(outputPath), { recursive: true });
  await copyFile(sourcePath, outputPath);
  const sourceStats = await stat(sourcePath);
  const outputStats = await stat(outputPath);
  const dimensions = pngDimensions(await readFile(outputPath), capture.outputArtifactRef);

  return {
    artifactId: capture.artifactId,
    sourceArtifactRef: capture.sourceArtifactRef,
    outputArtifactRef: capture.outputArtifactRef,
    sourceBytes: sourceStats.size,
    outputBytes: outputStats.size,
    width: dimensions.width,
    height: dimensions.height,
  };
}

async function writeAccessibilityArtifact() {
  const sourceArtifactRef = 'test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json';
  const outputArtifactRef =
    'output/tracking-plan-proof/product-parent-child-ui-runtime/08-cross-surface-accessibility-report.json';
  const sourcePath = path.join(repoRoot, sourceArtifactRef);
  const outputPath = path.join(repoRoot, outputArtifactRef);
  const sourceSummary = JSON.parse(await readFile(sourcePath, 'utf8'));
  const report = {
    schemaVersion: 1,
    reportMode: 'tracking-full-product-ui-local-cross-surface-accessibility',
    generatedAt,
    sourceArtifactRef,
    route: sourceSummary.route,
    assertions: sourceSummary.assertions,
    layoutBoxes: sourceSummary.summary?.layoutBoxes ?? [],
    nonClaims: {
      renderedChildDeviceRuntimeUiClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      productionProductUiClaimed: false,
      productClaimReady: false,
    },
  };
  await mkdir(path.dirname(outputPath), { recursive: true });
  await writeJson(outputPath, report);
  const sourceStats = await stat(sourcePath);
  const outputStats = await stat(outputPath);

  return {
    artifactId: 'cross-surface-accessibility-report',
    sourceArtifactRef,
    outputArtifactRef,
    sourceBytes: sourceStats.size,
    outputBytes: outputStats.size,
  };
}

async function readClosureEvidenceInput() {
  const childRuntimeProof = await readJson(path.join(repoRoot, childRuntimeArtifactGateProofRef));
  return {
    retentionWritableExecutionProofRef,
    retentionWritableExecutionProof: await readJson(path.join(repoRoot, retentionWritableExecutionProofRef)),
    childRuntimeArtifactGateProofRef,
    childRuntimeArtifactGateProof: childRuntimeProof.readModel ?? childRuntimeProof,
  };
}

async function writeEndToEndTraceArtifact(closureEvidenceInput) {
  const sourceArtifactRef = 'test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json';
  const outputArtifactRef =
    'output/tracking-plan-proof/product-parent-child-ui-runtime/09-product-ui-end-to-end-trace.json';
  const sourcePath = path.join(repoRoot, sourceArtifactRef);
  const outputPath = path.join(repoRoot, outputArtifactRef);
  const inventoryProof = JSON.parse(await readFile(sourcePath, 'utf8'));
  const retentionProof = closureEvidenceInput.retentionWritableExecutionProof;
  const childRuntimeProof = closureEvidenceInput.childRuntimeArtifactGateProof;
  const [childRuntimeRow] = childRuntimeProof.rows;
  assert.ok(childRuntimeRow, 'child runtime artifact gate proof needs a row');
  const trace = {
    schemaVersion: 1,
    traceMode: 'tracking-full-product-ui-local-end-to-end-trace',
    generatedAt,
    sourceArtifactRef,
    hostedInventoryStatus: inventoryProof.status,
    presentLocalProductArtifacts: [
      'output/tracking-plan-proof/product-parent-child-ui-runtime/01-parent-overview-runtime.png',
      'output/tracking-plan-proof/product-parent-child-ui-runtime/02-parent-device-detail-runtime.png',
      'output/tracking-plan-proof/product-parent-child-ui-runtime/03-parent-notification-history-preferences-runtime.png',
      'output/tracking-plan-proof/product-parent-child-ui-runtime/08-cross-surface-accessibility-report.json',
    ],
    localRuntimeClosureEvidence: {
      retentionWritableExecutionProofRef,
      retentionWritableExecutionRows: retentionProof.rows.length,
      retentionWritableExecutionDerivations: retentionProof.derivationMatrix.length,
      retentionWritableExecutionArtifactRefs: retentionProof.rows.map((row) => row.outputArtifactRef),
      childRuntimeArtifactGateProofRef,
      childRuntimeRequiredArtifacts: childRuntimeRow.requiredArtifacts,
      childRuntimeMissingArtifacts: childRuntimeRow.missingArtifacts,
    },
    stillMissingRuntimeArtifacts: [
      'output/tracking-plan-proof/product-parent-child-ui-runtime/04-retention-settings-production-write-result.png',
      'output/tracking-plan-proof/product-parent-child-ui-runtime/05-child-device-rendered-check-in-runtime.png',
      'output/tracking-plan-proof/product-parent-child-ui-runtime/06-child-device-rendered-location-consent-runtime.png',
      'output/tracking-plan-proof/product-parent-child-ui-runtime/07-child-device-safe-help-response-runtime.png',
    ],
    proofBoundary: {
      currentProofTier: 'P2_HOSTED_CI',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      localParentUiTraceOnly: true,
      fullProductUiRuntimeClaimed: false,
      childDeviceRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      productionProductUiClaimed: false,
      productClaimReady: false,
    },
  };
  await mkdir(path.dirname(outputPath), { recursive: true });
  await writeJson(outputPath, trace);
  const sourceStats = await stat(sourcePath);
  const outputStats = await stat(outputPath);

  return {
    artifactId: 'product-ui-end-to-end-trace',
    sourceArtifactRef,
    outputArtifactRef,
    sourceBytes: sourceStats.size,
    outputBytes: outputStats.size,
  };
}

function buildProof(readModel) {
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    workpackIds: ['30-parent-and-child-ui-ux-surfaces', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P2_HOSTED_CI',
    status: 'partial_local_artifacts_captured',
    sourceProofRefs,
    readModel,
    summary: {
      localArtifactCount: readModel.localArtifactCount,
      screenshotArtifactCount: readModel.rows.filter((row) => row.outputArtifactRef.endsWith('.png')).length,
      jsonArtifactCount: readModel.rows.filter((row) => row.outputArtifactRef.endsWith('.json')).length,
      retentionWritableExecutionRowCount: readModel.closureEvidence.retentionWritableExecutionRowCount,
      retentionWritableExecutionDerivationCount: readModel.closureEvidence.retentionWritableExecutionDerivationCount,
      childRuntimeMissingArtifactCount: readModel.closureEvidence.childRuntimeMissingArtifactCount,
      fullProductUiRuntimeClaimedRows: readModel.rows.filter((row) => row.fullProductUiRuntimeClaimed).length,
      childDeviceRuntimeClaimedRows: readModel.rows.filter((row) => row.childDeviceRuntimeClaimed).length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    proofLabels: [
      'tracking-full-product-ui-local-runtime.parent-overview-artifact',
      'tracking-full-product-ui-local-runtime.parent-device-detail-artifact',
      'tracking-full-product-ui-local-runtime.parent-notification-history-preferences-artifact',
      'tracking-full-product-ui-local-runtime.cross-surface-accessibility-artifact',
      'tracking-full-product-ui-local-runtime.local-end-to-end-trace-artifact',
      'tracking-full-product-ui-local-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'This proof only captures locally hosted parent overview/device shell screenshots, notification history/preferences screenshot, cross-surface accessibility metadata, and a local end-to-end trace into the product UI artifact root. Retention settings production write result, child-device rendered runtime UI, physical-device UI, authority-gated UI, provider-delivery UI, and production product UI still require real artifacts before product UI can be claimed.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.localArtifactCount, 5, 'expected five local parent UI artifacts');
  assert.equal(proof.summary.screenshotArtifactCount, 3, 'expected overview, device, and notification screenshots');
  assert.equal(proof.summary.jsonArtifactCount, 2, 'expected accessibility report and local trace artifacts');
  assert.equal(proof.summary.retentionWritableExecutionRowCount, 1, 'expected retention writable execution row');
  assert.equal(proof.summary.retentionWritableExecutionDerivationCount, 1, 'expected retention derivation matrix row');
  assert.equal(proof.summary.childRuntimeMissingArtifactCount, 10, 'expected child runtime hard-gap artifacts');
  assert.equal(
    proof.readModel.closureEvidence.retentionLocalProductSettingsWritableExecutionObserved,
    true,
    'expected local retention writable execution evidence'
  );
  assert.equal(proof.summary.fullProductUiRuntimeClaimedRows, 0, 'no full product UI runtime claims');
  assert.equal(proof.summary.childDeviceRuntimeClaimedRows, 0, 'no child runtime UI claims');
  assert.equal(proof.summary.productReadyRows, 0, 'no product-ready rows');
  assert.equal(proof.productClaims.fullProductUiRuntimeClaimed, false, 'no full product UI runtime claim');
  assert.equal(proof.productClaims.childDeviceRuntimeClaimed, false, 'no child-device runtime claim');
  assert.equal(proof.productClaims.productClaimReady, false, 'no product-ready claim');
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(focusedProofDir, 'proof.json'), proof);
  await writeJson(path.join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeFile(
    path.join(focusedProofDir, '00-source-snapshot.md'),
    [
      '# Tracking Full Product UI Local Runtime Artifact Capture Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- localArtifactCount: ${proof.summary.localArtifactCount}`,
      `- retentionWritableExecutionRowCount: ${proof.summary.retentionWritableExecutionRowCount}`,
      `- childRuntimeMissingArtifactCount: ${proof.summary.childRuntimeMissingArtifactCount}`,
      '- source: hosted parent overview/devices shell screenshots, hosted notification parent-surface screenshot, hosted tracking accessibility summary, and hosted artifact inventory proof.',
      '- closure evidence: consumes retention writable execution derivation proof and child runtime artifact gate proof without upgrading runtime claims.',
      '- boundary: local parent-side artifact capture and trace only; retention production write-result UI, child-device runtime, physical-device, authority, provider, production, and product-ready claims remain false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(path.join(output30, '33-full-product-ui-local-runtime-artifact-capture-proof.json'), proof);
  await writeJson(path.join(output33, '66-full-product-ui-local-runtime-artifact-capture-proof.json'), proof);
  await writeFile(
    path.join(output33, '66-full-product-ui-local-runtime-artifact-capture-validation-commands.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
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

async function writeJson(filePath, value) {
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

async function readJson(filePath) {
  return JSON.parse(await readFile(filePath, 'utf8'));
}

function pngDimensions(buffer, relativePath) {
  const signature = '89504e470d0a1a0a';
  if (buffer.subarray(0, 8).toString('hex') !== signature) {
    throw new Error(`Screenshot is not a PNG: ${relativePath}`);
  }
  return {
    width: buffer.readUInt32BE(16),
    height: buffer.readUInt32BE(20),
  };
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}
