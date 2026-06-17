import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofMode = 'tracking-provider-delivery-artifact-gate-proof';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const output26 = path.join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(output26, { recursive: true });
  await mkdir(output33, { recursive: true });

  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-provider-delivery-artifact-gate-proof.test.ts',
  ]);

  const proofModule = await tsImport(
    pathToFileURL(
      path.join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-provider-delivery-artifact-gate-proof.ts')
    ).href,
    import.meta.url
  );
  const generatedAt = '2026-06-07T19:20:00.000Z';
  const inventory = {
    presentArtifacts: await presentArtifactsForRoot(
      path.join(repoRoot, proofModule.RequiredTrackingProviderDeliveryArtifactPlan.proofRoot),
      proofModule.RequiredTrackingProviderDeliveryArtifactPlan.requiredArtifacts
    ),
  };
  const readModel = proofModule.buildTrackingProviderDeliveryArtifactGateProof(generatedAt, inventory);
  const proof = buildProof({ generatedAt, readModel });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-provider-delivery-artifact-gate-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function presentArtifactsForRoot(rootPath, requiredArtifacts) {
  const present = [];
  for (const artifact of requiredArtifacts) {
    const artifactPath = path.join(rootPath, artifact);
    if (await pathExists(artifactPath)) {
      present.push(artifact);
    }
  }
  return present;
}

async function pathExists(filePath) {
  try {
    await stat(filePath);
    return true;
  } catch (error) {
    if (error?.code === 'ENOENT') return false;
    throw error;
  }
}

function buildProof({ generatedAt, readModel }) {
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    workpackIds: ['26-alert-severity-and-notification-model', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_MANUAL_PROVIDER_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: readModel.rows.every((row) => row.providerDeliveryArtifactSetComplete)
      ? 'artifact_set_present'
      : 'manual_required',
    readModel,
    summary: {
      rowCount: readModel.rows.length,
      completeRows: readModel.rows.filter((row) => row.providerDeliveryArtifactSetComplete).length,
      manualRequiredRows: readModel.rows.filter((row) => !row.providerDeliveryArtifactSetComplete).length,
      missingArtifactCount: readModel.rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      providerDeliveryRuntimeClaimedRows: readModel.rows.filter((row) => row.providerDeliveryRuntimeClaimed).length,
      webhookReceiptIngestionRuntimeClaimedRows: readModel.rows.filter(
        (row) => row.webhookReceiptIngestionRuntimeClaimed
      ).length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    proofLabels: [
      'tracking-provider-delivery.artifact-gate',
      'tracking-provider-delivery.receipt-runtime-artifact-requirements',
      'tracking-provider-delivery.no-runtime-claim-from-file-presence',
      'tracking-provider-delivery.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Actual notification provider delivery and receipt ingestion still require a manual provider-runtime run with redacted config, credential-presence attestation, minimal payload snapshot, provider attempt/response, receipt webhook event, receipt ingestion result, retry/quiet-hours worker log, parent notification UI screenshot, and result summary. This gate validates required artifact presence only and keeps provider delivery, receipt ingestion, credentials, adapter dispatch, production outbox storage, child-device delivery, physical-device, authority, and product-ready claims false.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 1, 'expected one provider delivery artifact gate row');
  assert.equal(proof.summary.providerDeliveryRuntimeClaimedRows, 0, 'no provider delivery runtime claims');
  assert.equal(proof.summary.webhookReceiptIngestionRuntimeClaimedRows, 0, 'no webhook receipt runtime claims');
  assert.equal(proof.summary.productReadyRows, 0, 'no product-ready rows');
  assert.deepEqual(Object.values(proof.productClaims), [
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
  ]);
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(output26, '29-provider-delivery-artifact-gate-proof.json'), proof);
  await writeJson(path.join(output33, '51-provider-delivery-artifact-gate-proof.json'), proof);
  await writeFile(
    path.join(output33, '51-provider-delivery-artifact-gate-validation-commands.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
}

function run(command, args) {
  commands.push({ command: [command, ...args].join(' ') });
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

async function writeJson(filePath, value) {
  await mkdir(path.dirname(filePath), { recursive: true });
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}
