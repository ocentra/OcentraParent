import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', 'social-policy-schedule-time-budget-proof');
const resultDirectory = join(root, 'test-results', 'social-policy-schedule-time-budget-proof');

const requiredFiles = [
  'packages/schema-domain/src/social-policy-compiler-values.ts',
  'packages/schema-domain/src/social-policy-compiler.ts',
  'packages/browser-domain/tests/unit/social-policy-compiler.test.ts',
];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  const values = await readText('packages/schema-domain/src/social-policy-compiler-values.ts');
  const compiler = await readText('packages/schema-domain/src/social-policy-compiler.ts');
  const test = await readText('packages/browser-domain/tests/unit/social-policy-compiler.test.ts');
  const featureDoc = await readText('docs/features/social-video-control.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md');

  const checks = [
    checkFilesExist(),
    checkIncludes(values, 'SocialPolicyTimeBudgetRefsSchema', 'time budget refs schema'),
    checkIncludes(values, 'SocialParentPolicyScheduleStateSchema', 'schedule state schema'),
    checkIncludes(values, 'SocialParentPolicyTimeBudgetStateSchema', 'time budget state schema'),
    checkIncludes(compiler, 'timeBudgetContextRefs', 'compiler propagates time budget refs'),
    checkIncludes(compiler, "value.scheduleState !== 'manual-required'", 'contract-only schedule state guard'),
    checkIncludes(compiler, "value.timeBudgetState !== 'unavailable'", 'contract-only time budget guard'),
    checkIncludes(test, 'scheduleContextRefs: []', 'missing schedule rejection test'),
    checkIncludes(test, 'timeBudgetContextRefs: []', 'missing time budget rejection test'),
    checkIncludes(featureDoc, 'social-policy-schedule-time-budget-proof', 'feature doc proof note'),
    checkIncludes(workpackReadme, 'social-policy-schedule-time-budget-proof', 'workpack README proof note'),
  ].flat();

  const failures = checks.filter((check) => !check.pass).map((check) => check.label);
  const proof = {
    schemaVersion: 1,
    proofMode: 'social-policy-schedule-time-budget-proof',
    generatedAt: new Date().toISOString(),
    files: requiredFiles,
    outputDirectory: relativePath(outputDirectory),
    checks,
    summary: {
      proofClaim: 'social policy compiler requires explicit schedule and time-budget context',
      scheduleIntegration: 'contract-proof-present',
      timeBudgetIntegration: 'contract-proof-present',
      finalPolicyDecision: 'not-claimed',
      runtimeGate: 'not-claimed',
      enforcement: 'not-claimed',
      failures: failures.length,
    },
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social policy schedule/time-budget proof failed:\n${failures.join('\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const markdownPath = join(outputDirectory, '01-social-policy-schedule-time-budget-proof.md');
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(proof)}\n`);

  console.log('social-policy-schedule-time-budget-proof-ok=true');
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
    '# Social Policy Schedule/Time-Budget Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Proof claim: ${proof.summary.proofClaim}`,
    `Schedule integration: ${proof.summary.scheduleIntegration}`,
    `Time-budget integration: ${proof.summary.timeBudgetIntegration}`,
    `Final policy decision: ${proof.summary.finalPolicyDecision}`,
    `Runtime gate: ${proof.summary.runtimeGate}`,
    `Enforcement: ${proof.summary.enforcement}`,
    '',
    'This proof strengthens the SOCIAL-12 parent policy compiler contract so',
    'contract-only social decision candidates must cite explicit schedule and',
    'time-budget context refs and states. Manual-required or unavailable',
    'schedule/time-budget states remain non-final fallbacks.',
  ].join('\n');
}

async function readText(path) {
  return readFile(join(root, path), 'utf8');
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
