import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, stat, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-ios-simulator-artifact-inventory-proof';
const resultRoot = join(repoRoot, 'test-results', proofMode);
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const wp11Root = join(repoRoot, 'output', 'tracking-plan-proof', '11-ios-core-location-foreground-adapter');
const wp12Root = join(repoRoot, 'output', 'tracking-plan-proof', '12-ios-background-region-significant-change-adapter');
const wp31Root = join(repoRoot, 'output', 'tracking-plan-proof', '31-platform-extension-checklists-and-proof-routing');
const wp33Root = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const generatedAt = '2026-06-08T11:00:00.000Z';
const sourceIosSimulatorProofRef = 'test-results/tracking-plan-ios-simulator-proof/proof.json';
const iosManualRequiredProofRef = 'test-results/tracking-ios-location-manual-required-proof/proof.json';
const iosPrivacyReleaseProofRef = 'test-results/tracking-ios-privacy-disclosure-release-proof/proof.json';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await rm(proofRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(proofRoot, { recursive: true });
  await mkdir(wp11Root, { recursive: true });
  await mkdir(wp12Root, { recursive: true });
  await mkdir(wp31Root, { recursive: true });
  await mkdir(wp33Root, { recursive: true });

  run('cmd', [
    '/c',
    'npm',
    'exec',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'vitest',
    'run',
    'tests/contract/tracking-ios-simulator-artifact-inventory-proof.test.ts',
  ]);

  const proof = await buildProof();
  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-ios-simulator-artifact-inventory-proof-ok');
  console.log('evidence=test-results/tracking-ios-simulator-artifact-inventory-proof/proof.json');
}

async function buildProof() {
  const proofModule = await importSource(
    'packages/schema-domain/src/tracking-ios-simulator-artifact-inventory-proof.ts'
  );
  const iosSimulatorProof = await readJson(sourceIosSimulatorProofRef);
  const iosManualRequiredProof = await readJson(iosManualRequiredProofRef);
  const iosPrivacyReleaseProof = await readJson(iosPrivacyReleaseProofRef);
  const readModel = proofModule.buildTrackingIosSimulatorArtifactInventoryProof(generatedAt, {
    sourceIosSimulatorProofRef,
    iosSimulatorProofStatus: iosSimulatorProof.simulatorExecution.currentStatus,
    iosSimulatorCurrentProofTier: iosSimulatorProof.simulatorExecution.currentProofTier,
    hostPlatform: iosSimulatorProof.simulatorExecution.host.platform,
    hostArch: iosSimulatorProof.simulatorExecution.host.arch,
    canRunXcodeSimulator: iosSimulatorProof.simulatorExecution.host.canRunXcodeSimulator,
    iosManualRequiredRowCount: iosManualRequiredProof.summary.rows,
    iosRequiredRuntimeArtifactCount: iosManualRequiredProof.summary.requiredRuntimeArtifactRefs,
    iosPresentRuntimeArtifactCount: iosManualRequiredProof.summary.presentRuntimeArtifactRefs,
    iosMissingRuntimeArtifactCount: iosManualRequiredProof.summary.missingRuntimeArtifactRefs,
    privacyReleaseGateRowCount: iosPrivacyReleaseProof.summary.rows,
    privacyReleaseBlockedCount: iosPrivacyReleaseProof.summary.releaseBlockedCount,
    artifactRows: await artifactRows(proofModule.RequiredTrackingIosSimulatorArtifactRefs),
  });

  return {
    ...readModel,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    sourceProofs: {
      iosSimulator: {
        proofRef: sourceIosSimulatorProofRef,
        currentStatus: iosSimulatorProof.simulatorExecution.currentStatus,
        currentProofTier: iosSimulatorProof.simulatorExecution.currentProofTier,
        host: iosSimulatorProof.simulatorExecution.host,
        manualRequiredReason: iosSimulatorProof.simulatorExecution.manualRequiredReason,
      },
      iosManualRequired: {
        proofRef: iosManualRequiredProofRef,
        summary: iosManualRequiredProof.summary,
      },
      iosPrivacyRelease: {
        proofRef: iosPrivacyReleaseProofRef,
        summary: iosPrivacyReleaseProof.summary,
      },
    },
    artifactPaths: {
      wp11: 'output/tracking-plan-proof/11-ios-core-location-foreground-adapter/21-ios-simulator-artifact-inventory-proof.json',
      wp12: 'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/21-ios-simulator-artifact-inventory-proof.json',
      wp31: 'output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/21-ios-simulator-artifact-inventory-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/69-ios-simulator-artifact-inventory-proof.json',
      evidence: 'test-results/tracking-ios-simulator-artifact-inventory-proof/proof.json',
      namedProofRoot: 'output/tracking-plan-proof/tracking-ios-simulator-artifact-inventory-proof/proof.json',
    },
  };
}

async function artifactRows(requiredArtifactRefs) {
  const rows = [];
  for (const artifactRef of requiredArtifactRefs) {
    const artifactStat = await stat(join(repoRoot, artifactRef)).catch(() => undefined);
    rows.push({
      artifactRef,
      category: categoryFor(artifactRef),
      required: true,
      present: artifactStat !== undefined && artifactStat.isFile(),
      byteSize: artifactStat?.size ?? 0,
    });
  }
  return rows;
}

function assertProof(proof) {
  if (proof.summary.missingArtifactCount !== 0 || !proof.summary.simulatorArtifactInventoryComplete) {
    throw new Error(`iOS simulator artifact inventory has missing artifacts: ${JSON.stringify(proof.summary)}`);
  }
  if (
    proof.summary.simulatorPackageArtifactCount < 4 ||
    proof.summary.locationManualRequiredArtifactCount < 3 ||
    proof.summary.privacyDisclosureArtifactCount < 2
  ) {
    throw new Error(`iOS simulator artifact inventory lost expected proof rows: ${JSON.stringify(proof.summary)}`);
  }
  if (proof.summary.iosManualRequiredRowCount !== 7 || proof.summary.iosMissingRuntimeArtifactCount !== 9) {
    throw new Error(
      `iOS simulator artifact inventory lost manual-required accounting: ${JSON.stringify(proof.summary)}`
    );
  }
  if (
    proof.productClaims.coreLocationRuntimeClaimed ||
    proof.productClaims.backgroundRegionRuntimeClaimed ||
    proof.productClaims.physicalDeviceProofClaimed ||
    proof.productClaims.productClaimReady
  ) {
    throw new Error(
      `iOS simulator artifact inventory overclaimed product readiness: ${JSON.stringify(proof.productClaims)}`
    );
  }
}

async function writeArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(resultRoot, 'artifact-inventory-read-model.json'), proof.rows);
  await writeJson(join(proofRoot, 'proof.json'), proof);
  await writeJson(join(wp11Root, '21-ios-simulator-artifact-inventory-proof.json'), proof);
  await writeJson(join(wp12Root, '21-ios-simulator-artifact-inventory-proof.json'), proof);
  await writeJson(join(wp31Root, '21-ios-simulator-artifact-inventory-proof.json'), proof);
  await writeJson(join(wp33Root, '69-ios-simulator-artifact-inventory-proof.json'), proof);
  await writeFile(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot(proof));
  await writeFile(join(proofRoot, '16-validation-commands.log'), validationLog());
}

function sourceSnapshot(proof) {
  return [
    '# Tracking iOS Simulator Artifact Inventory Source Snapshot',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.baseCommitAtGeneration}`,
    '- requiredProofTier: P4_PHYSICAL_DEVICE',
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    '- status: ios-simulator-local-artifacts-present-physical-device-required',
    `- sourceIosSimulatorProofRef: ${sourceIosSimulatorProofRef}`,
    `- requiredArtifactCount: ${proof.summary.requiredArtifactCount}`,
    `- presentArtifactCount: ${proof.summary.presentArtifactCount}`,
    `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
    `- simulatorPackageArtifactCount: ${proof.summary.simulatorPackageArtifactCount}`,
    `- locationManualRequiredArtifactCount: ${proof.summary.locationManualRequiredArtifactCount}`,
    `- privacyDisclosureArtifactCount: ${proof.summary.privacyDisclosureArtifactCount}`,
    `- platformProofArtifactCount: ${proof.summary.platformProofArtifactCount}`,
    `- validationLogArtifactCount: ${proof.summary.validationLogArtifactCount}`,
    `- iosManualRequiredRowCount: ${proof.summary.iosManualRequiredRowCount}`,
    `- iosMissingRuntimeArtifactCount: ${proof.summary.iosMissingRuntimeArtifactCount}`,
    '- does not prove iOS Core Location runtime, Always authorization, region delivery, physical-device behavior, authority enrollment, provider delivery, production runtime, or product readiness',
    '- proof module: packages/schema-domain/src/tracking-ios-simulator-artifact-inventory-proof.ts',
    '- proof tests: packages/tracking-domain/tests/contract/tracking-ios-simulator-artifact-inventory-proof.test.ts',
    '- proof harness: scripts/test/tracking-ios-simulator-artifact-inventory-proof.mjs',
    '',
  ].join('\n');
}

function categoryFor(artifactRef) {
  if (artifactRef.includes('18-ios-simulator-proof') || artifactRef.includes('tracking-plan-ios-simulator-proof')) {
    return 'simulator-package-proof';
  }
  if (artifactRef.includes('19-ios-location-manual-required') || artifactRef.includes('tracking-ios-location-manual')) {
    return 'location-manual-required-proof';
  }
  if (artifactRef.includes('20-ios-privacy-disclosure') || artifactRef.includes('tracking-ios-privacy')) {
    return 'privacy-disclosure-proof';
  }
  if (artifactRef.includes('validation-commands')) return 'validation-log';
  return 'platform-proof';
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  commands.push({
    command: printable,
    status: result.status,
    stdout: result.stdout.trim(),
    stderr: result.stderr.trim(),
  });
  if (result.status !== 0) {
    throw new Error(`${printable} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function validationLog() {
  return `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`;
}

async function importSource(relativePath) {
  return tsImport(pathToFileURL(join(repoRoot, relativePath)).href, import.meta.url);
}

async function readJson(path) {
  return JSON.parse(await readFile(join(repoRoot, path), 'utf8'));
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function gitOutput(args) {
  const result = spawnSync('git', args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  if (result.status !== 0) return '';
  return result.stdout.trim();
}
