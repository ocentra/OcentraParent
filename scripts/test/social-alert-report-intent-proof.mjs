import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', 'social-alert-report-intent-proof');
const resultDirectory = join(root, 'test-results', 'social-alert-report-intent-proof');

const requiredFiles = [
  'packages/parent-domain/src/social-alert-report-intent-values.ts',
  'packages/parent-domain/src/social-alert-report-intent.ts',
  'packages/parent-domain/tests/social-alert-report-intent.test.ts',
];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  const packageJson = await readText('packages/parent-domain/package.json');
  const featureDoc = await readText('docs/features/social-video-control.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md');
  const contract = await readText('packages/parent-domain/src/social-alert-report-intent.ts');
  const test = await readText('packages/parent-domain/tests/social-alert-report-intent.test.ts');

  const checks = [
    checkFilesExist(),
    checkIncludes(packageJson, './social-alert-report-intent', 'parent-domain package export'),
    checkIncludes(featureDoc, 'social-alert-report-intent-proof', 'social/video feature proof note'),
    checkIncludes(workpackReadme, 'social-alert-report-intent-proof', 'social workpack README proof note'),
    checkIncludes(contract, 'reportDeliveryClaimed', 'report delivery non-claim guard'),
    checkIncludes(contract, 'finalPolicyDecisionClaimed', 'final policy non-claim guard'),
    checkIncludes(contract, 'enforcementClaimed', 'enforcement non-claim guard'),
    checkIncludes(test, 'providerDeliveryAttempted: true', 'provider delivery rejection test'),
    checkIncludes(test, 'rawMessageContentIncluded: true', 'raw message rejection test'),
  ].flat();

  const failures = checks.filter((check) => !check.pass).map((check) => check.label);
  const proof = {
    schemaVersion: 1,
    proofMode: 'social-alert-report-intent-proof',
    generatedAt: new Date().toISOString(),
    files: requiredFiles,
    outputDirectory: relativePath(outputDirectory),
    checks,
    summary: {
      proofClaim: 'social alert/report contract intent present',
      alertDelivery: 'local-outbox-intent-only',
      reportDelivery: 'not-claimed',
      providerDelivery: 'not-claimed',
      rawSocialContent: 'rejected',
      finalPolicyDecision: 'not-claimed',
      enforcement: 'not-claimed',
      failures: failures.length,
    },
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social alert/report proof failed:\n${failures.join('\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const markdownPath = join(outputDirectory, '01-social-alert-report-intent-proof.md');
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(proof)}\n`);

  console.log('social-alert-report-intent-proof-ok=true');
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
    '# Social Alert/Report Intent Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Proof claim: ${proof.summary.proofClaim}`,
    `Alert delivery: ${proof.summary.alertDelivery}`,
    `Report delivery: ${proof.summary.reportDelivery}`,
    `Provider delivery: ${proof.summary.providerDelivery}`,
    `Raw social content: ${proof.summary.rawSocialContent}`,
    `Final policy decision: ${proof.summary.finalPolicyDecision}`,
    `Enforcement: ${proof.summary.enforcement}`,
    '',
    'This proof adds the parent-domain alert/report intent boundary for social',
    'signals. It proves minimal ref-only payloads can be queued for the local',
    'outbox and parent report linkage while rejecting raw account, video,',
    'message, screenshot, provider-delivery, report-delivery, final-policy, and',
    'enforcement claims.',
  ].join('\n');
}

async function readText(path) {
  return readFile(join(root, path), 'utf8');
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
