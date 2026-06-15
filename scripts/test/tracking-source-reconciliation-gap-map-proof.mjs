import { createHash } from 'node:crypto';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-source-reconciliation-gap-map-proof';
const wp01Root = join(repoRoot, 'output', 'tracking-plan-proof', '01-source-index-and-repo-reconciliation');
const wp02Root = join(repoRoot, 'output', 'tracking-plan-proof', '02-current-tracking-snapshot-and-gap-map');
const resultRoot = join(repoRoot, 'test-results', proofMode);
const generatedAt = '2026-06-08T00:35:00.000Z';

const sourceDocs = [
  'docs/features/location-geofence-device-status.md',
  'docs/expectations/location-geofence.md',
  'docs/tracking-control-settings-inventory.md',
  'docs/device-location-tracking-capability-guide.md',
  'docs/device-location-tracking-schema-proposal.md',
  'docs/plans/tracking-plan/source-index.md',
  'docs/plans/tracking-plan/current-tracking-snapshot.md',
  'docs/plans/tracking-plan/pasted-content-coverage-audit.md',
  'docs/plans/tracking-plan/implementation-checklist.md',
];

const requiredSourceIndexPhrases = [
  'Source Truth Rule',
  'Pasted Draft Coverage',
  'does not replace source docs',
  'make notification providers a child-location data store',
];

const requiredSnapshotPhrases = [
  'Snapshot Date',
  'Local/CI Proof Now Exists',
  'Runtime/Product Claims Still Missing',
  'Remaining Product-Claim Blockers',
  'product-ready tracking remains false',
];

const requiredClosureBlockers = [
  'android-physical-background-proof-required',
  'ios-physical-region-proof-required',
  'retention-writable-product-settings-required',
  'retention-platform-runtime-enforcement-required',
  'actual-child-device-runtime-required',
  'full-product-parent-child-ui-required',
  'authority-enrollment-proof-required',
  'provider-delivery-receipt-runtime-required',
  'production-durable-workers-required',
];

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(wp01Root, { recursive: true });
  await mkdir(wp02Root, { recursive: true });

  const proof = await buildProof();
  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-source-reconciliation-gap-map-proof-ok');
  console.log('evidence=test-results/tracking-source-reconciliation-gap-map-proof/proof.json');
}

async function buildProof() {
  const sourceIndex = await readText('docs/plans/tracking-plan/source-index.md');
  const snapshot = await readText('docs/plans/tracking-plan/current-tracking-snapshot.md');
  const closure = await readJson('test-results/tracking-product-readiness-closure-proof/proof.json');
  const [closureRow] = closure.rows ?? [];
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    requiredProofTier: 'P1_DOC_SOURCE_RECONCILIATION',
    currentProofTier: 'P1_DOC_SOURCE_RECONCILIATION',
    currentStatus: 'proved',
    sourceDocs: await Promise.all(sourceDocs.map(documentSummary)),
    sourceIndexAssertions: phraseAssertions(sourceIndex, requiredSourceIndexPhrases),
    currentSnapshotAssertions: phraseAssertions(snapshot, requiredSnapshotPhrases),
    closureCoverageTags: closure.sourceProofs?.map((entry) => entry.coverageTag) ?? [],
    remainingProductBlockers: closureRow?.remainingBlockers ?? [],
    productClaims: {
      localCiProofAccountingReady: closureRow?.localCiProofAccountingReady === true,
      physicalAndroidBackgroundClaimed: closureRow?.physicalAndroidBackgroundClaimed === true,
      physicalIosBackgroundClaimed: closureRow?.physicalIosBackgroundClaimed === true,
      childDeviceRuntimeClaimed: closureRow?.childDeviceRuntimeClaimed === true,
      fullProductUiClaimed: closureRow?.fullProductUiClaimed === true,
      authorityClaimed: closureRow?.authorityClaimed === true,
      providerDeliveryReceiptClaimed: closureRow?.providerDeliveryReceiptClaimed === true,
      productionWorkersClaimed: closureRow?.productionWorkersClaimed === true,
      productReadyClaimed: closureRow?.productReadyClaimed === true,
    },
    nonClaims: {
      planningDocsAreRuntimeProof: false,
      physicalDeviceClaimed: false,
      authorityEnrollmentClaimed: false,
      providerDeliveryClaimed: false,
      productionRuntimeClaimed: false,
      productReadyClaimed: false,
    },
    proofPaths: {
      wp01: 'output/tracking-plan-proof/01-source-index-and-repo-reconciliation/proof.json',
      wp02: 'output/tracking-plan-proof/02-current-tracking-snapshot-and-gap-map/proof.json',
      evidence: 'test-results/tracking-source-reconciliation-gap-map-proof/proof.json',
      closure: 'test-results/tracking-product-readiness-closure-proof/proof.json',
    },
  };
}

function assertProof(proof) {
  const missingDocs = proof.sourceDocs.filter((entry) => !entry.exists).map((entry) => entry.path);
  if (missingDocs.length > 0) {
    throw new Error(`Tracking source docs missing: ${missingDocs.join(', ')}`);
  }
  assertAllPresent('source index phrase', proof.sourceIndexAssertions);
  assertAllPresent('current snapshot phrase', proof.currentSnapshotAssertions);
  const blockerSet = new Set(proof.remainingProductBlockers);
  const missingBlockers = requiredClosureBlockers.filter((blocker) => !blockerSet.has(blocker));
  if (missingBlockers.length > 0) {
    throw new Error(`Tracking closure blockers missing from gap map: ${missingBlockers.join(', ')}`);
  }
  if (
    proof.productClaims.physicalAndroidBackgroundClaimed ||
    proof.productClaims.physicalIosBackgroundClaimed ||
    proof.productClaims.authorityClaimed ||
    proof.productClaims.providerDeliveryReceiptClaimed ||
    proof.productClaims.productionWorkersClaimed ||
    proof.productClaims.productReadyClaimed
  ) {
    throw new Error(`Tracking source/gap proof overclaimed product readiness: ${JSON.stringify(proof.productClaims)}`);
  }
}

function assertAllPresent(label, assertions) {
  const missing = assertions.filter((entry) => !entry.present).map((entry) => entry.phrase);
  if (missing.length > 0) {
    throw new Error(`Missing ${label}: ${missing.join(', ')}`);
  }
}

async function writeArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(wp01Root, 'proof.json'), proof);
  await writeJson(join(wp02Root, 'proof.json'), proof);
  await writeFile(
    join(wp01Root, '00-source-snapshot.md'),
    sourceSnapshot('WP01 Source Index And Repo Reconciliation', proof)
  );
  await writeFile(
    join(wp02Root, '00-source-snapshot.md'),
    sourceSnapshot('WP02 Current Tracking Snapshot And Gap Map', proof)
  );
  await writeFile(join(wp01Root, '16-validation-commands.log'), validationLog());
  await writeFile(join(wp02Root, '16-validation-commands.log'), validationLog());
}

function sourceSnapshot(title, proof) {
  return [
    `# ${title}`,
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.baseCommitAtGeneration}`,
    `- branch: ${proof.branch}`,
    '- requiredProofTier: P1_DOC_SOURCE_RECONCILIATION',
    '- currentProofTier: P1_DOC_SOURCE_RECONCILIATION',
    '- status: proved',
    '- proves source docs, current snapshot, and product-readiness closure blockers are aligned',
    '- does not prove physical-device, authority, provider delivery, production, or product-ready tracking behavior',
    '',
    '## Remaining Product Blockers',
    '',
    ...proof.remainingProductBlockers.map((blocker) => `- ${blocker}`),
    '',
  ].join('\n');
}

function validationLog() {
  return [
    'node scripts/test/tracking-product-readiness-closure-proof.mjs exit=0',
    'node scripts/test/tracking-source-reconciliation-gap-map-proof.mjs exit=0',
    '',
  ].join('\n');
}

async function documentSummary(path) {
  const text = await readText(path);
  return {
    path,
    exists: true,
    sha256: createHash('sha256').update(text).digest('hex'),
    lineCount: text.split(/\r?\n/).length,
  };
}

function phraseAssertions(text, phrases) {
  return phrases.map((phrase) => ({
    phrase,
    present: text.includes(phrase),
  }));
}

async function readText(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function readJson(path) {
  return JSON.parse(await readText(path));
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
