import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', 'social-24-rollout-manual-required-labels');
const resultDirectory = join(root, 'test-results', 'social-platform-account-feed-rollout-gate');
const checklistPath = 'docs/plans/browser-plan/implementation-checklist.md';

const rolloutGuards = [
  {
    docKey: 'plan',
    text: 'SOCIAL rollout state: partial/manual-required.',
  },
  {
    docKey: 'readme',
    text: 'SOCIAL rollout state: partial/manual-required.',
  },
  {
    docKey: 'feature',
    text: 'Product completion remains unclaimed;',
  },
  {
    docKey: 'browserFeature',
    text: 'Product checklist upgrade is not claimed.',
  },
  {
    docKey: 'expectation',
    text: 'Rollout/manual-required gates may label rows as partial/manual-required only.',
  },
];

if (!existsSync(join(root, checklistPath))) {
  throw new Error(`Missing checklist: ${checklistPath}`);
}

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  const docs = await loadDocs();
  const rows = expectedRows().map((row) => validateChecklistRow(row, docs.checklist));
  const guardFailures = validateRolloutGuards(docs);
  const failures = [...rows.flatMap((row) => row.failures), ...guardFailures];
  const manifest = manifestFor(rows, failures);

  if (manifest.failures.length > 0) {
    throw new Error(`Social platform account/feed rollout gate failed:\n${manifest.failures.join('\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const markdownPath = join(outputDirectory, '01-rollout-manual-required-labels.md');
  await writeFile(proofPath, `${JSON.stringify(manifest, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(manifest)}\n`);

  console.log('social-platform-account-feed-rollout-gate-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
  console.log(`complete=${manifest.summary.completeRows} partial=${manifest.summary.partialRows}`);
}

async function loadDocs() {
  return {
    checklist: await readText(checklistPath),
    plan: await readText('docs/plans/browser-plan/v0-5-social-platform-account-feed-gating-plan.md'),
    readme: await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md'),
    feature: await readText('docs/features/social-video-control.md'),
    browserFeature: await readText('docs/features/browser-web-control.md'),
    expectation: await readText('docs/expectations/social-video-control.md'),
  };
}

function expectedRows() {
  const completeRows = new Set([1, 13, 14, 15, 20, 21, 22]);
  return Array.from({ length: 23 }, (_, index) => {
    const rowNumber = index + 1;
    const complete = completeRows.has(rowNumber);
    return {
      rowNumber,
      rowId: `SOCIAL-${String(rowNumber).padStart(2, '0')}`,
      expectedStatus: complete ? '[x]' : '[~]',
      expectedState: complete ? 'proof-present' : 'partial-manual-required',
    };
  });
}

function validateChecklistRow(row, checklist) {
  const rowText = checklistRowText(row.rowId, checklist);
  const failures = [];
  if (!rowText) {
    failures.push(`${row.rowId} missing checklist row`);
    return { ...row, failures };
  }
  if (!rowText.includes(row.expectedStatus)) {
    failures.push(`${row.rowId} status is not ${row.expectedStatus}`);
  }
  if (!rowText.includes('codex-d')) {
    failures.push(`${row.rowId} owner is not codex-d`);
  }
  return {
    ...row,
    state: row.expectedState,
    failures,
  };
}

function validateRolloutGuards(docs) {
  return rolloutGuards
    .filter((guard) => !docs[guard.docKey].includes(guard.text))
    .map((guard) => `${guard.docKey} missing rollout guard: ${guard.text}`);
}

function manifestFor(rows, failures) {
  return {
    schemaVersion: 1,
    proofMode: 'social-platform-account-feed-rollout-gate',
    generatedAt: new Date().toISOString(),
    rows,
    summary: {
      totalRows: rows.length,
      completeRows: rows.filter((row) => row.expectedState === 'proof-present').length,
      partialRows: rows.filter((row) => row.expectedState === 'partial-manual-required').length,
      failures: failures.length,
      rolloutState: 'partial/manual-required',
      productClaimed: false,
    },
    guardTexts: rolloutGuards.map((guard) => guard.text),
    noClaimLabels: [
      'rendered-proof-bundle-social-ui-present',
      'service-backed-dashboard-and-explanation-read-model-proof-present',
      'social-alert-report-intent-proof-present',
      'social-schedule-time-budget-compiler-proof-present',
      'social-parent-sensitivity-settings-proof-present',
      'provider-report-delivery-not-claimed',
      'applied-schedule-time-budget-not-claimed',
      'connector-native-runtime-not-claimed',
      'final-policy-execution-not-claimed',
      'enforcement-not-claimed',
      'product-checklist-upgrade-not-claimed',
    ],
    failures,
  };
}

function markdownFor(manifest) {
  const rows = manifest.rows.map((row) => `| ${row.rowId} | ${row.state} | ${row.expectedStatus} |`).join('\n');
  return [
    '# SOCIAL-24 Rollout Manual-Required Labels',
    '',
    `Generated: ${manifest.generatedAt}`,
    '',
    `Rows checked: ${manifest.summary.totalRows}`,
    `Proof-present rows: ${manifest.summary.completeRows}`,
    `Partial/manual-required rows: ${manifest.summary.partialRows}`,
    `Rollout state: ${manifest.summary.rolloutState}`,
    `Product claimed: ${manifest.summary.productClaimed}`,
    '',
    '| Row | State | Checklist Status |',
    '| --- | --- | --- |',
    rows,
    '',
    'SOCIAL rollout state: partial/manual-required.',
    'Product checklist upgrade is not claimed.',
    'Rendered proof-bundle social UI exists for dashboard, child intervention,',
    'and parent explanation states. Service-backed dashboard and explanation',
    'read-model delivery is present. Ref-only social alert/report intent proof',
    'is present. Schedule/time-budget compiler proof and parent sensitivity',
    'settings proof are present; connector/native runtime, provider/report',
    'delivery, applied schedules/budgets, final policy execution, and',
    'enforcement remain unclaimed.',
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
