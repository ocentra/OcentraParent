import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', 'game-24-rollout-manual-required-labels');
const resultDirectory = join(root, 'test-results', 'browser-game-rollout-gate');
const checklistPath = 'docs/plans/browser-plan/implementation-checklist.md';

const rolloutGuards = [
  {
    docKey: 'plan',
    text: 'GAME rollout state: partial/manual-required.',
  },
  {
    docKey: 'readme',
    text: 'GAME rollout state: partial/manual-required.',
  },
  {
    docKey: 'browserFeature',
    text: 'Browser-game/cloud-gaming GAME-24 now labels the game track partial/manual-required through the rollout gate.',
  },
  {
    docKey: 'expectation',
    text: 'Browser-game rollout gates may label rows as complete, partial/manual-required, or open/manual-required only.',
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
    throw new Error(`Browser-game rollout gate failed:\n${manifest.failures.join('\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const markdownPath = join(outputDirectory, '01-rollout-manual-required-labels.md');
  await writeFile(proofPath, `${JSON.stringify(manifest, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(manifest)}\n`);

  console.log('browser-game-rollout-gate-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
  console.log(
    `complete=${manifest.summary.completeRows} partial=${manifest.summary.partialRows} open=${manifest.summary.openRows}`
  );
}

async function loadDocs() {
  return {
    checklist: await readText(checklistPath),
    plan: await readText('docs/plans/browser-plan/v0-5-browser-games-cloud-gaming-gating-plan.md'),
    readme: await readText('docs/plans/browser-plan/browser-games-cloud-gaming/readme.md'),
    browserFeature: await readText('docs/features/browser-web-control.md'),
    expectation: await readText('docs/expectations/browser-evidence.md'),
  };
}

function expectedRows() {
  return Array.from({ length: 24 }, (_, index) => {
    const rowNumber = index + 1;
    const rowId = `GAME-${String(rowNumber).padStart(2, '0')}`;
    if (rowNumber === 1) {
      return {
        rowNumber,
        rowId,
        expectedStatus: '[x]',
        expectedOwner: 'codex-d',
        expectedState: 'scaffold-proof-present',
      };
    }
    if ([2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 23, 24].includes(rowNumber)) {
      return {
        rowNumber,
        rowId,
        expectedStatus: '[~]',
        expectedOwner: 'codex-d',
        expectedState: 'partial-manual-required',
      };
    }
    return {
      rowNumber,
      rowId,
      expectedStatus: '[ ]',
      expectedOwner: '',
      expectedState: 'open-manual-required',
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
  if (row.expectedOwner && !rowText.includes(row.expectedOwner)) {
    failures.push(`${row.rowId} owner is not ${row.expectedOwner}`);
  }
  return {
    ...row,
    state: row.expectedState,
    failures,
  };
}

function validateRolloutGuards(docs) {
  return rolloutGuards
    .filter((guard) => !normalizedText(docs[guard.docKey]).includes(normalizedText(guard.text)))
    .map((guard) => `${guard.docKey} missing rollout guard: ${guard.text}`);
}

function normalizedText(text) {
  return text.replace(/\s+/g, ' ').trim();
}

function manifestFor(rows, failures) {
  return {
    schemaVersion: 1,
    proofMode: 'browser-game-rollout-gate',
    generatedAt: new Date().toISOString(),
    rows,
    summary: {
      totalRows: rows.length,
      completeRows: rows.filter((row) => row.expectedState === 'scaffold-proof-present').length,
      partialRows: rows.filter((row) => row.expectedState === 'partial-manual-required').length,
      openRows: rows.filter((row) => row.expectedState === 'open-manual-required').length,
      failures: failures.length,
      rolloutState: 'partial/manual-required',
      productClaimed: false,
    },
    guardTexts: rolloutGuards.map((guard) => guard.text),
    noClaimLabels: [
      'browser-game-route-contracts-open',
      'runtime-signal-proof-manual-required',
      'metadata-ai-memory-proof-manual-required',
      'child-parent-ui-not-claimed',
      'cloud-streamed-frame-analysis-not-claimed',
      'native-game-control-not-claimed',
      'enforcement-not-claimed',
      'product-checklist-upgrade-not-claimed',
    ],
    failures,
  };
}

function markdownFor(manifest) {
  const rows = manifest.rows.map((row) => `| ${row.rowId} | ${row.state} | ${row.expectedStatus} |`).join('\n');
  return [
    '# GAME-24 Rollout Manual-Required Labels',
    '',
    `Generated: ${manifest.generatedAt}`,
    '',
    `Rows checked: ${manifest.summary.totalRows}`,
    `Scaffold-proof rows: ${manifest.summary.completeRows}`,
    `Partial/manual-required rows: ${manifest.summary.partialRows}`,
    `Open/manual-required rows: ${manifest.summary.openRows}`,
    `Rollout state: ${manifest.summary.rolloutState}`,
    `Product claimed: ${manifest.summary.productClaimed}`,
    '',
    '| Row | State | Checklist Status |',
    '| --- | --- | --- |',
    rows,
    '',
    'GAME rollout state: partial/manual-required.',
    'Product checklist upgrade is not claimed.',
    'Browser-game route contracts, runtime signals, metadata, AI, memory,',
    'child/parent UI, cloud-streamed frame analysis, native game control,',
    'and enforcement remain open or manual-required until separate proof exists.',
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
