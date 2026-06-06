import { mkdir, readFile, rm, stat, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const outputDir = join(repoRoot, 'test-results', 'tracking-hosted-ui-artifact-inventory-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-hosted-ui-artifact-inventory-proof');
const wp30ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const generatedAt = '2026-06-06T14:40:00.000Z';

const requiredScreenshots = [
  '11-ui-snapshots/hosted-policy-tracking-live-summary.png',
  '11-ui-snapshots/hosted-policy-tracking-live-summary-mobile.png',
  '11-ui-snapshots/hosted-policy-tracking-citation-detail.png',
  '11-ui-snapshots/hosted-policy-tracking-evidence-drawer.png',
  '11-ui-snapshots/hosted-policy-tracking-child-check-in.png',
  '11-ui-snapshots/hosted-policy-tracking-child-runtime-ui.png',
  '11-ui-snapshots/hosted-policy-tracking-family-dashboard-rollup.png',
  '11-ui-snapshots/hosted-policy-tracking-retention-settings.png',
];

const requiredAssertions = [
  'named-region',
  'visible-heading',
  'service-backed-row-citation-visible',
  'service-data-coverage-visible',
  'service-backed-citation-detail-screenshot',
  'service-backed-evidence-drawer-screenshot',
  'family-dashboard-rollup-screenshot',
  'retention-settings-screenshot',
  'child-check-in-screenshot',
  'child-runtime-ui-screenshot',
  'desktop-screenshot',
  'mobile-screenshot',
  'no-product-claim-visible',
  'child-device-delivery-not-claimed',
  'no-unlabeled-buttons',
];

await rm(outputDir, { recursive: true, force: true });
await rm(proofDir, { recursive: true, force: true });
await mkdir(outputDir, { recursive: true });
await mkdir(proofDir, { recursive: true });
await mkdir(wp33ProofDir, { recursive: true });

const proof = await buildProof();

assertProof(proof);
await writeJson(join(outputDir, 'proof.json'), proof);
await writeJson(join(proofDir, 'proof.json'), proof);
await writeFile(join(proofDir, '00-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
await writeFile(join(proofDir, '16-validation-commands.log'), validationLog(proof), 'utf8');
await writeJson(join(wp30ProofDir, '21-hosted-ui-artifact-inventory-proof.json'), proof);
await writeJson(join(wp33ProofDir, '28-hosted-ui-artifact-inventory-proof.json'), proof);

console.log('tracking-hosted-ui-artifact-inventory-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-hosted-ui-artifact-inventory-proof', 'proof.json')}`);

async function buildProof() {
  const hostedProof = await readJson(
    'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-hosted-ui-proof.json'
  );
  const evidenceDrawerProof = await readJson(
    'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/20-evidence-drawer-hosted-ui-proof.json'
  );
  const accessibilitySummary = await readJson('test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json');
  const screenshots = await Promise.all(requiredScreenshots.map(readScreenshot));

  return {
    schemaVersion: 1,
    proofMode: 'tracking-hosted-ui-artifact-inventory-proof',
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    currentStatus: 'proved',
    productClaimReady: false,
    sourceProofs: {
      hostedProof: proofSummary(hostedProof),
      evidenceDrawerProof: proofSummary(evidenceDrawerProof),
      accessibilitySummary: accessibilitySummarySummary(accessibilitySummary),
    },
    screenshots,
    requiredAssertions,
    proofPaths: {
      evidence: 'test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json',
      workpack30Proof:
        'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/21-hosted-ui-artifact-inventory-proof.json',
      wp33Proof:
        'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/28-hosted-ui-artifact-inventory-proof.json',
      accessibilitySummary: 'test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json',
    },
    nonClaims: {
      fullParentChildUiClaimed: false,
      childDeviceRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryClaimed: false,
      productionProofClaimed: false,
      productReadyTrackingClaimed: false,
    },
  };
}

async function readScreenshot(relativePath) {
  const absolutePath = join(wp30ProofDir, relativePath);
  const buffer = await readFile(absolutePath);
  const stats = await stat(absolutePath);
  const dimensions = pngDimensions(buffer, relativePath);

  return {
    path: `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/${relativePath}`,
    bytes: stats.size,
    width: dimensions.width,
    height: dimensions.height,
  };
}

function assertProof(proof) {
  if (proof.screenshots.length !== requiredScreenshots.length) {
    throw new Error(`Unexpected screenshot count: ${proof.screenshots.length}`);
  }
  const badScreenshot = proof.screenshots.find((screenshot) => screenshot.bytes <= 1024 || screenshot.width <= 0);
  if (badScreenshot) {
    throw new Error(`Invalid screenshot artifact: ${JSON.stringify(badScreenshot)}`);
  }
  if (proof.sourceProofs.hostedProof.productClaimReady !== false) {
    throw new Error('Hosted proof must keep productClaimReady=false.');
  }
  if (proof.sourceProofs.evidenceDrawerProof.productClaimReady !== false) {
    throw new Error('Evidence drawer proof must keep productClaimReady=false.');
  }
  const missingAssertions = requiredAssertions.filter(
    (assertion) => !proof.sourceProofs.accessibilitySummary.assertions.includes(assertion)
  );
  if (missingAssertions.length > 0) {
    throw new Error(`Missing hosted accessibility assertions: ${missingAssertions.join(', ')}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Hosted UI inventory proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

function proofSummary(proof) {
  return {
    proofMode: proof.proofMode,
    currentProofTier: proof.currentProofTier,
    currentStatus: proof.currentStatus,
    productClaimReady: proof.productClaimReady,
  };
}

function accessibilitySummarySummary(summary) {
  return {
    route: summary.route,
    assertions: summary.assertions,
    hasNamedRegion: summary.summary?.hasNamedRegion === true,
    headingCount: summary.summary?.headings?.length ?? 0,
    labelCount: summary.summary?.labels?.length ?? 0,
  };
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

function sourceSnapshot(proof) {
  return [
    '# Tracking Hosted UI Artifact Inventory Proof Source Snapshot',
    '',
    `- Branch: ${proof.branch}`,
    `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
    '- Source proof: existing hosted UI Playwright proof artifacts and accessibility summary.',
    '- Scope: verify stored hosted screenshots, evidence drawer proof, and accessibility assertions for WP30/WP33 handoff.',
    '- Boundary: inventory proof only; child-device runtime, physical-device proof, authority, provider delivery, full parent/child UI beyond the hosted route, production proof, and product-ready tracking remain unclaimed.',
    '',
  ].join('\n');
}

function validationLog(proof) {
  return [
    '$ node scripts/test/tracking-hosted-ui-artifact-inventory-proof.mjs',
    'tracking-hosted-ui-artifact-inventory-proof-ok',
    `evidence=${proof.proofPaths.evidence}`,
  ].join('\n');
}

async function readJson(relativePath) {
  return JSON.parse(await readFile(join(repoRoot, relativePath), 'utf8'));
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}
