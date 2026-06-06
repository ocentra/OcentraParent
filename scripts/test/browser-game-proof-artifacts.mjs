import { existsSync } from 'node:fs';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const proofRoot = join(root, 'output', 'browser-plan-proof');
const outputDirectory = join(proofRoot, 'game-22-tests-fixtures-playwright-manual-proof');
const resultDirectory = join(root, 'test-results', 'browser-game-proof-artifacts');
const requiredProofFiles = ['00-source-snapshot.md', '08-security-negative-proof.md', '10-validation-commands.log'];

if (!existsSync(proofRoot)) {
  throw new Error(`Missing browser proof root: ${relativePath(proofRoot)}`);
}

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  const docs = await loadDocs();
  const proofDirectories = await gameProofDirectories();
  const rows = await Promise.all(expectedRows().map((row) => validateGameRow(row, proofDirectories, docs)));
  const failures = rows.flatMap((row) => row.failures);
  const manifest = manifestFor(rows, failures);

  if (manifest.failures.length > 0) {
    throw new Error(`Browser-game proof artifact gate failed:\n${manifest.failures.join('\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const markdownPath = join(outputDirectory, '01-browser-game-proof-artifact-manifest.md');
  await writeFile(proofPath, `${JSON.stringify(manifest, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(manifest)}\n`);

  console.log('browser-game-proof-artifacts-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
  console.log(`complete=${manifest.summary.completeRows} partial=${manifest.summary.partialRows}`);
  console.log(`playwright=${manifest.summary.playwrightState}`);
}

async function loadDocs() {
  return {
    checklist: await readText('docs/plans/browser-plan/implementation-checklist.md'),
    plan: await readText('docs/plans/browser-plan/v0-5-browser-games-cloud-gaming-gating-plan.md'),
    readme: await readText('docs/plans/browser-plan/browser-games-cloud-gaming/readme.md'),
    feature: await readText('docs/features/browser-web-control.md'),
    expectation: await readText('docs/expectations/browser-evidence.md'),
  };
}

async function gameProofDirectories() {
  const entries = await readdir(proofRoot, { withFileTypes: true });
  return new Map(
    entries
      .filter((entry) => entry.isDirectory() && /^game-\d\d-/.test(entry.name))
      .map((entry) => [Number(entry.name.slice(5, 7)), entry.name])
  );
}

function expectedRows() {
  return Array.from({ length: 21 }, (_, index) => {
    const rowNumber = index + 1;
    const isComplete = rowNumber <= 6;
    return {
      rowNumber,
      rowId: `GAME-${String(rowNumber).padStart(2, '0')}`,
      expectedStatus: isComplete ? '[x]' : '[~]',
      expectedState: isComplete
        ? rowNumber === 1
          ? 'scaffold-proof-present'
          : rowNumber === 2
            ? 'live-route-proof-present'
            : rowNumber === 3
              ? 'live-portal-pattern-proof-present'
              : rowNumber === 4
                ? 'live-cloud-pattern-proof-present'
                : rowNumber === 5
                  ? 'live-url-shape-proof-present'
                  : 'live-runtime-signal-shape-proof-present'
        : 'partial-manual-required',
    };
  });
}

async function validateGameRow(row, proofDirectories, docs) {
  const failures = [];
  const proofDirectory = proofDirectories.get(row.rowNumber);
  if (!proofDirectory) {
    failures.push(`${row.rowId} missing proof directory`);
    return { ...row, proofDirectory: null, proofFiles: [], failures };
  }

  const proofFiles = await readdir(join(proofRoot, proofDirectory));
  failures.push(...validateChecklist(row, proofDirectory, docs.checklist));
  failures.push(...validateProofFiles(row, proofFiles));
  failures.push(...validateDocs(row, proofDirectory, docs));

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
  if (!proofFiles.includes('ui-not-applicable.md')) {
    failures.push(`${row.rowId} proof is missing ui-not-applicable.md`);
  }
  return failures;
}

function validateDocs(row, proofDirectory, docs) {
  const failures = [];
  if (!docs.plan.includes(row.rowId)) {
    failures.push(`${row.rowId} missing from browser-game gating plan`);
  }
  if (!docs.readme.includes(`output/browser-plan-proof/${proofDirectory}/`)) {
    failures.push(`${row.rowId} proof directory missing from browser-game README`);
  }
  if (row.rowNumber >= 2 && !docs.feature.includes(row.rowId)) {
    failures.push(`${row.rowId} missing from browser-web-control feature doc`);
  }
  if (!docs.expectation.includes('Browser-game proof artifact gates may verify')) {
    failures.push(`${row.rowId} expectation proof artifact boundary missing`);
  }
  return failures;
}

function manifestFor(rows, failures) {
  return {
    schemaVersion: 1,
    proofMode: 'browser-game-proof-artifacts',
    generatedAt: new Date().toISOString(),
    proofRoot: relativePath(proofRoot),
    rows,
    summary: {
      totalRows: rows.length,
      completeRows: rows.filter((row) => row.expectedStatus === '[x]').length,
      partialRows: rows.filter((row) => row.expectedState === 'partial-manual-required').length,
      failures: failures.length,
      playwrightState: 'manual-required-no-rendered-browser-game-ui',
      productClaimed: false,
    },
    manualProofBoundary: {
      screenshots: 'not-applicable-contract-only',
      playwright: 'manual-required-no-rendered-browser-game-ui',
      liveRouteEvidence: 'game-02-live-route-proof-present',
      livePortalPatternEvidence: 'game-03-live-portal-pattern-proof-present',
      liveCloudPatternEvidence: 'game-04-live-cloud-pattern-proof-present',
      liveUrlShapeEvidence: 'game-05-live-url-shape-proof-present',
      liveRuntimeSignalEvidence: 'game-06-live-runtime-signal-shape-proof-present',
      renderedUi: 'not-claimed',
      cloudStreamedFrameAnalysis: 'not-claimed',
      nativeGameControl: 'not-claimed',
      enforcement: 'not-claimed',
      productChecklistUpgrade: 'not-claimed',
    },
    failures,
  };
}

function markdownFor(manifest) {
  const rows = manifest.rows
    .map((row) => `| ${row.rowId} | ${row.state} | \`${row.proofDirectory}\` | ${row.proofFiles.length} |`)
    .join('\n');
  return [
    '# GAME-22 Browser-Game Proof Artifact Manifest',
    '',
    `Generated: ${manifest.generatedAt}`,
    '',
    `Rows checked: ${manifest.summary.totalRows}`,
    `Proof-present rows: ${manifest.summary.completeRows}`,
    `Partial/manual-required rows: ${manifest.summary.partialRows}`,
    `Playwright state: ${manifest.summary.playwrightState}`,
    `Product claimed: ${manifest.summary.productClaimed}`,
    '',
    '| Row | State | Proof Directory | Files |',
    '| --- | --- | --- | --- |',
    rows,
    '',
    'GAME-22 proves proof-pack coverage for GAME-01 through GAME-21.',
    'GAME-02 live route proof is present for real public browser-game and',
    'cloud-gaming route surfaces with ref-only/hash-only custody.',
    'GAME-03 live portal pattern library proof is present for real public',
    'browser-game portal and archive surfaces with ref-only/hash-only custody.',
    'GAME-04 live cloud pattern library proof is present for real public',
    'cloud-gaming and cloud-PC surfaces with ref-only/hash-only custody.',
    'GAME-05 live URL-shape parser proof is present for real public',
    'browser-game and cloud-gaming route surfaces with ref-only/hash-only custody.',
    'GAME-06 live runtime signal shape proof is present for real public',
    'browser-game and cloud-gaming pages through Playwright with ref-only/hash-only custody.',
    'It does not prove rendered browser-game UI, Playwright screenshots,',
    'runtime browser-game detection, cloud-streamed frame analysis, native',
    'game control, final policy execution, enforcement, or product checklist',
    'completion.',
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
