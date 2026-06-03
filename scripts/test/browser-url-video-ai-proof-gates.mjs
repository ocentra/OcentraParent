import { existsSync } from 'node:fs';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', 'ai-25-proof-gates-fixtures-tests-rollout');
const resultDirectory = join(root, 'test-results', 'browser-url-video-ai-proof-gates');
const proofRoot = join(root, 'output', 'browser-plan-proof');

const partialRows = new Set([19, 20, 21, 22, 23, 24]);
const requiredProofFiles = ['00-source-snapshot.md', '08-security-negative-proof.md', '10-validation-commands.log'];
const rolloutGuardTexts = [
  'AI cannot enforce directly.',
  'Remote AI cannot run by default.',
  'Memory cannot drive block without evidence refs.',
  'Unknown/timeout must become explicit parent fallback.',
  'Do not block because AI said so.',
];

if (!existsSync(proofRoot)) {
  throw new Error(`Missing browser proof root: ${relativePath(proofRoot)}`);
}

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  const docs = await loadDocs();
  const proofDirectories = await aiProofDirectories();
  const rows = await Promise.all(expectedRows().map((row) => validateAiRow(row, proofDirectories, docs)));
  const failures = rows.flatMap((row) => row.failures);
  const guardFailures = validateRolloutGuards(docs.plan);
  const manifest = manifestFor(rows, [...failures, ...guardFailures]);

  if (manifest.failures.length > 0) {
    throw new Error(`Browser URL/video AI proof gate failed:\n${manifest.failures.join('\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const markdownPath = join(outputDirectory, '01-ai-proof-gate.md');
  await writeFile(proofPath, `${JSON.stringify(manifest, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(manifest)}\n`);

  console.log('browser-url-video-ai-proof-gates-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
  console.log(`complete=${manifest.summary.completeRows} partial=${manifest.summary.partialRows}`);
}

async function loadDocs() {
  return {
    checklist: await readText('docs/plans/browser-plan/implementation-checklist.md'),
    plan: await readText('docs/plans/browser-plan/v0-5-browser-url-video-ai-intelligence-plan.md'),
    feature: await readText('docs/features/browser-web-control.md'),
    expectation: await readText('docs/expectations/browser-evidence.md'),
  };
}

async function aiProofDirectories() {
  const entries = await readdir(proofRoot, { withFileTypes: true });
  return new Map(
    entries
      .filter((entry) => entry.isDirectory() && /^ai-\d\d-/.test(entry.name))
      .map((entry) => [Number(entry.name.slice(3, 5)), entry.name])
  );
}

function expectedRows() {
  return Array.from({ length: 24 }, (_, index) => {
    const rowNumber = index + 1;
    return {
      rowNumber,
      rowId: `AI-${String(rowNumber).padStart(2, '0')}`,
      expectedStatus: partialRows.has(rowNumber) ? '[~]' : '[x]',
      expectedState: partialRows.has(rowNumber) ? 'partial-manual-required' : 'contract-proof-present',
      requiresUiMarker: rowNumber !== 1,
    };
  });
}

async function validateAiRow(row, proofDirectories, docs) {
  const failures = [];
  const proofDirectory = proofDirectories.get(row.rowNumber);
  if (!proofDirectory) {
    failures.push(`${row.rowId} missing proof directory`);
    return { ...row, proofDirectory: null, proofFiles: [], failures };
  }

  const proofFiles = await readdir(join(proofRoot, proofDirectory));
  failures.push(...validateChecklist(row, proofDirectory, docs.checklist));
  failures.push(...validateProofFiles(row, proofFiles));
  failures.push(...validatePlanMention(row, docs.plan));

  return {
    ...row,
    proofDirectory,
    proofFiles: proofFiles.sort(),
    state: row.expectedState,
    failures,
  };
}

function validateChecklist(row, proofDirectory, checklist) {
  const rowText = checklistRowText(row.rowId, checklist);
  const failures = [];
  if (!rowText) {
    return [`${row.rowId} missing implementation-checklist row`];
  }
  if (!rowText.includes(row.expectedStatus)) {
    failures.push(`${row.rowId} checklist status is not ${row.expectedStatus}`);
  }
  if (!rowText.includes('codex-d')) {
    failures.push(`${row.rowId} checklist owner is not codex-d`);
  }
  if (!rowText.includes(`output/browser-plan-proof/${proofDirectory}/`)) {
    failures.push(`${row.rowId} checklist does not reference its proof directory`);
  }
  return failures;
}

function validateProofFiles(row, proofFiles) {
  const failures = [];
  for (const requiredFile of requiredProofFiles) {
    if (!proofFiles.includes(requiredFile)) {
      failures.push(`${row.rowId} proof is missing ${requiredFile}`);
    }
  }
  if (!proofFiles.some((file) => /^01-.*proof\.(md|log)$/.test(file))) {
    failures.push(`${row.rowId} proof is missing a 01-* proof artifact`);
  }
  if (row.requiresUiMarker && !proofFiles.includes('ui-not-applicable.md')) {
    failures.push(`${row.rowId} proof is missing ui-not-applicable.md`);
  }
  return failures;
}

function validatePlanMention(row, plan) {
  return plan.includes(row.rowId) ? [] : [`${row.rowId} missing from URL/video AI plan checkpoint`];
}

function validateRolloutGuards(plan) {
  return rolloutGuardTexts
    .filter((guardText) => !plan.includes(guardText))
    .map((guardText) => `rollout guard missing: ${guardText}`);
}

function manifestFor(rows, failures) {
  return {
    schemaVersion: 1,
    proofMode: 'browser-url-video-ai-proof-gates',
    generatedAt: new Date().toISOString(),
    proofRoot: relativePath(proofRoot),
    rows,
    summary: {
      totalRows: rows.length,
      completeRows: rows.filter((row) => row.expectedState === 'contract-proof-present').length,
      partialRows: rows.filter((row) => row.expectedState === 'partial-manual-required').length,
      failures: failures.length,
      rolloutState: 'partial-manual-required',
      productClaimed: false,
    },
    noClaimGuards: rolloutGuardTexts,
    failures,
  };
}

function markdownFor(manifest) {
  const rows = manifest.rows
    .map((row) => `| ${row.rowId} | ${row.state} | \`${row.proofDirectory}\` | ${row.proofFiles.length} |`)
    .join('\n');
  return [
    '# AI-25 URL/Video AI Proof Gate',
    '',
    `Generated: ${manifest.generatedAt}`,
    '',
    `Rows checked: ${manifest.summary.totalRows}`,
    `Contract-proof rows: ${manifest.summary.completeRows}`,
    `Partial/manual-required rows: ${manifest.summary.partialRows}`,
    `Product claimed: ${manifest.summary.productClaimed}`,
    '',
    '| Row | State | Proof Directory | Files |',
    '| --- | --- | --- | --- |',
    rows,
    '',
    'Rollout state: partial/manual-required. The gate proves proof-pack coverage',
    'for AI-01 through AI-24, not runtime model execution, UI delivery, policy',
    'authority, enforcement, or product completion.',
  ].join('\n');
}

function checklistRowText(rowId, checklist) {
  return checklist.split(/\r?\n/).find((line) => line.startsWith(`| ${rowId} |`));
}

async function readText(path) {
  return readFile(join(root, path), 'utf8');
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
