import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', 'social-applied-schedule-time-budget-proof');
const resultDirectory = join(root, 'test-results', 'social-applied-schedule-time-budget-proof');

const requiredFiles = [
  'packages/parent-domain/src/social-applied-schedule-time-budget-proof.ts',
  'packages/parent-domain/tests/social-applied-schedule-time-budget-proof.test.ts',
  'scripts/test/social-applied-schedule-time-budget-proof.mjs',
];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  const packageJson = await readText('packages/parent-domain/package.json');
  const featureDoc = await readText('docs/features/social-video-control.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md');
  const contract = await readText('packages/parent-domain/src/social-applied-schedule-time-budget-proof.ts');
  const test = await readText('packages/parent-domain/tests/social-applied-schedule-time-budget-proof.test.ts');
  const proofModule = await import('../../packages/parent-domain/dist/social-applied-schedule-time-budget-proof.js');

  const readModel = proofModule.SocialAppliedScheduleTimeBudgetProofReadModel;
  const summary = proofModule.summarizeSocialAppliedScheduleTimeBudgetProof(readModel);
  const checks = [
    checkFilesExist(),
    checkIncludes(packageJson, './social-applied-schedule-time-budget-proof', 'parent-domain package export'),
    checkIncludes(featureDoc, 'social-applied-schedule-time-budget-proof', 'social/video feature proof note'),
    checkIncludes(workpackReadme, 'social-applied-schedule-time-budget-proof', 'social workpack README proof note'),
    checkIncludes(contract, 'runtimeScheduleAppliedClaimed: Schema.Literal(false)', 'runtime schedule guard'),
    checkIncludes(contract, 'runtimeTimeBudgetAppliedClaimed: Schema.Literal(false)', 'runtime time-budget guard'),
    checkIncludes(contract, 'browserRuntimeGateExecutedClaimed: Schema.Literal(false)', 'browser runtime gate guard'),
    checkIncludes(contract, 'finalPolicyDecisionClaimed: Schema.Literal(false)', 'final policy guard'),
    checkIncludes(contract, 'enforcementClaimed: Schema.Literal(false)', 'enforcement guard'),
    checkIncludes(test, 'runtimeScheduleAppliedClaimed: true', 'runtime schedule rejection test'),
    checkIncludes(test, "evaluatedScheduleState: 'inside-allowed-window'", 'mismatched schedule rejection test'),
  ].flat();

  const failures = checks.filter((check) => !check.pass).map((check) => check.label);
  const proof = {
    schemaVersion: 1,
    proofMode: 'social-applied-schedule-time-budget-proof',
    generatedAt: new Date().toISOString(),
    files: requiredFiles,
    outputDirectory: relativePath(outputDirectory),
    checks,
    summary,
    rows: readModel.rows.map((row) => ({
      appliedScheduleTimeBudgetRowId: row.appliedScheduleTimeBudgetRowId,
      sourceDecisionCandidateId: row.sourceDecisionCandidateId,
      applicationState: row.applicationState,
      evaluatedScheduleState: row.evaluatedScheduleState,
      evaluatedTimeBudgetState: row.evaluatedTimeBudgetState,
      parentOwnedScheduleWindowEvaluated: row.parentOwnedScheduleWindowEvaluated,
      parentOwnedTimeBudgetEvaluated: row.parentOwnedTimeBudgetEvaluated,
      runtimeScheduleAppliedClaimed: row.runtimeScheduleAppliedClaimed,
      runtimeTimeBudgetAppliedClaimed: row.runtimeTimeBudgetAppliedClaimed,
      browserRuntimeGateExecutedClaimed: row.browserRuntimeGateExecutedClaimed,
      finalPolicyDecisionClaimed: row.finalPolicyDecisionClaimed,
      enforcementClaimed: row.enforcementClaimed,
    })),
    nonClaims: readModel.nonClaims,
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social applied schedule/time-budget proof failed:\\n${failures.join('\\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const markdownPath = join(outputDirectory, '01-social-applied-schedule-time-budget-proof.md');
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(proof)}\n`);

  console.log('social-applied-schedule-time-budget-proof-ok=true');
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
    '# Social Applied Schedule Time-Budget Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Rows: ${proof.summary.totalRows}`,
    `Parent-owned application evaluated rows: ${proof.summary.parentOwnedApplicationEvaluatedRows}`,
    `Manual-required rows: ${proof.summary.manualRequiredRows}`,
    `Runtime schedule applied claimed: ${proof.summary.runtimeScheduleAppliedClaimed}`,
    `Runtime time-budget applied claimed: ${proof.summary.runtimeTimeBudgetAppliedClaimed}`,
    `Browser runtime gate executed claimed: ${proof.summary.browserRuntimeGateExecutedClaimed}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
    '',
    'This proof consumes SOCIAL-12 compiler candidates and proves a',
    'parent-owned schedule/time-budget application-readiness read model. It can',
    'cite evaluated schedule and budget refs plus a runtime handoff ref while',
    'preserving explicit non-claims for runtime-applied schedules, runtime',
    'time-budget application, browser gate execution, final policy execution,',
    'and enforcement.',
  ].join('\n');
}

async function readText(path) {
  return readFile(join(root, path), 'utf8');
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
