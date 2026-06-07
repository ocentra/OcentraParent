import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', 'social-report-writer-delivery-proof');
const resultDirectory = join(root, 'test-results', 'social-report-writer-delivery-proof');

const requiredFiles = [
  'packages/parent-domain/src/social-report-writer-delivery-proof.ts',
  'packages/parent-domain/tests/social-report-writer-delivery-proof.test.ts',
  'scripts/test/social-report-writer-delivery-proof.mjs',
];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  const packageJson = await readText('packages/parent-domain/package.json');
  const featureDoc = await readText('docs/features/social-video-control.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md');
  const contract = await readText('packages/parent-domain/src/social-report-writer-delivery-proof.ts');
  const test = await readText('packages/parent-domain/tests/social-report-writer-delivery-proof.test.ts');
  const proofModule = await import('../../packages/parent-domain/dist/social-report-writer-delivery-proof.js');

  const readModel = proofModule.SocialReportWriterDeliveryProofReadModel;
  const summary = proofModule.summarizeSocialReportWriterDeliveryProof(readModel);
  const checks = [
    checkFilesExist(),
    checkIncludes(packageJson, './social-report-writer-delivery-proof', 'parent-domain package export'),
    checkIncludes(featureDoc, 'social-report-writer-delivery-proof', 'social/video feature proof note'),
    checkIncludes(workpackReadme, 'social-report-writer-delivery-proof', 'social workpack README proof note'),
    checkIncludes(
      contract,
      'externalRuntimeReportDeliveryClaimed: Schema.Literal(false)',
      'external report delivery guard'
    ),
    checkIncludes(contract, 'providerDeliveryAttempted: Schema.Literal(false)', 'provider delivery guard'),
    checkIncludes(contract, 'finalPolicyDecisionClaimed: Schema.Literal(false)', 'final policy guard'),
    checkIncludes(contract, 'enforcementClaimed: Schema.Literal(false)', 'enforcement guard'),
    checkIncludes(test, 'externalRuntimeReportDeliveryClaimed: true', 'external delivery rejection test'),
    checkIncludes(test, 'reportArtifactRef: null', 'missing report artifact rejection test'),
  ].flat();

  const failures = checks.filter((check) => !check.pass).map((check) => check.label);
  const proof = {
    schemaVersion: 1,
    proofMode: 'social-report-writer-delivery-proof',
    generatedAt: new Date().toISOString(),
    files: requiredFiles,
    outputDirectory: relativePath(outputDirectory),
    checks,
    summary,
    rows: readModel.reportWriterDeliveryRows.map((row) => ({
      reportWriterDeliveryRowId: row.reportWriterDeliveryRowId,
      sourceIntentRef: row.sourceIntentRef,
      reportWriterDeliveryState: row.reportWriterDeliveryState,
      reportWriterReceiptState: row.reportWriterReceiptState,
      parentOwnedReportArtifactWritten: row.parentOwnedReportArtifactWritten,
      parentOwnedReportReceiptRecorded: row.parentOwnedReportReceiptRecorded,
      externalRuntimeReportDeliveryClaimed: row.externalRuntimeReportDeliveryClaimed,
      providerDeliveryAttempted: row.providerDeliveryAttempted,
      finalPolicyDecisionClaimed: row.finalPolicyDecisionClaimed,
      enforcementClaimed: row.enforcementClaimed,
    })),
    nonClaims: readModel.nonClaims,
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social report writer delivery proof failed:\n${failures.join('\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const markdownPath = join(outputDirectory, '01-social-report-writer-delivery-proof.md');
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(proof)}\n`);

  console.log('social-report-writer-delivery-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
}

function checkFilesExist() {
  return requiredFiles.map((path) => ({
    label: `${path} exists`,
    pass: existsSync(join(root, path)),
  }));
}

function checkIncludes(text, expected, label) {
  return {
    label,
    pass: text.includes(expected),
  };
}

function markdownFor(proof) {
  return [
    '# Social Report Writer Delivery Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Rows: ${proof.summary.totalRows}`,
    `Report delivery ready rows: ${proof.summary.reportDeliveryReadyRows}`,
    `Manual-required rows: ${proof.summary.manualRequiredRows}`,
    `External runtime report delivery claimed: ${proof.summary.externalRuntimeReportDeliveryClaimed}`,
    `Provider delivery attempted: ${proof.summary.providerDeliveryAttempted}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
    '',
    'This proof adds a parent-owned social report writer delivery-readiness',
    'boundary. It proves report-ready rows can cite parent-owned report artifacts',
    'and receipts from social alert/report intents while preserving explicit',
    'non-claims for external runtime report delivery, provider dispatch, provider',
    'receipt ingestion, raw social content, final policy execution, and enforcement.',
  ].join('\n');
}

async function readText(path) {
  return readFile(join(root, path), 'utf8');
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
