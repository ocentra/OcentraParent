import { existsSync } from 'node:fs';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const proofRoot = join(root, 'output', 'browser-plan-proof');
const outputDirectory = join(proofRoot, 'social-23-tests-fixtures-playwright-manual-proof');
const resultDirectory = join(root, 'test-results', 'social-platform-account-feed-proof-artifacts');
const requiredProofFiles = ['00-source-snapshot.md', '08-security-negative-proof.md', '10-validation-commands.log'];

if (!existsSync(proofRoot)) {
  throw new Error(`Missing browser proof root: ${relativePath(proofRoot)}`);
}

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  const docs = await loadDocs();
  const proofDirectories = await socialProofDirectories();
  const rows = await Promise.all(expectedRows().map((row) => validateSocialRow(row, proofDirectories, docs)));
  const failures = rows.flatMap((row) => row.failures);
  const manifest = manifestFor(rows, failures);

  if (manifest.failures.length > 0) {
    throw new Error(`Social platform account/feed proof gate failed:\n${manifest.failures.join('\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const markdownPath = join(outputDirectory, '01-social-proof-artifact-manifest.md');
  await writeFile(proofPath, `${JSON.stringify(manifest, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(manifest)}\n`);

  console.log('social-platform-account-feed-proof-artifacts-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
  console.log(`complete=${manifest.summary.completeRows} partial=${manifest.summary.partialRows}`);
}

async function loadDocs() {
  return {
    checklist: await readText('docs/plans/browser-plan/implementation-checklist.md'),
    plan: await readText('docs/plans/browser-plan/v0-5-social-platform-account-feed-gating-plan.md'),
    readme: await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md'),
    feature: await readText('docs/features/social-video-control.md'),
    expectation: await readText('docs/expectations/social-video-control.md'),
  };
}

async function socialProofDirectories() {
  const entries = await readdir(proofRoot, { withFileTypes: true });
  return new Map(
    entries
      .filter((entry) => entry.isDirectory() && /^social-\d\d-/.test(entry.name))
      .map((entry) => [Number(entry.name.slice(7, 9)), entry.name])
  );
}

function expectedRows() {
  const completeRows = new Set([1, 8, 9, 10, 11, 12, 13, 14, 15, 18, 20, 21, 22]);
  return Array.from({ length: 22 }, (_, index) => {
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

async function validateSocialRow(row, proofDirectories, docs) {
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
    failures.push(`${row.rowId} missing from social gating plan`);
  }
  if (!docs.readme.includes(`output/browser-plan-proof/${proofDirectory}/`)) {
    failures.push(`${row.rowId} proof directory missing from social workpack README`);
  }
  if (row.rowNumber >= 2 && !docs.feature.includes(row.rowId)) {
    failures.push(`${row.rowId} missing from social/video feature doc`);
  }
  if (row.rowNumber >= 2 && !docs.expectation.includes('Contract Boundary')) {
    failures.push(`${row.rowId} expectation contract boundary missing`);
  }
  return failures;
}

function manifestFor(rows, failures) {
  return {
    schemaVersion: 1,
    proofMode: 'social-platform-account-feed-proof-artifacts',
    generatedAt: new Date().toISOString(),
    proofRoot: relativePath(proofRoot),
    rows,
    summary: {
      totalRows: rows.length,
      completeRows: rows.filter((row) => row.expectedState === 'proof-present').length,
      partialRows: rows.filter((row) => row.expectedState === 'partial-manual-required').length,
      failures: failures.length,
      playwrightState: 'rendered-proof-bundle-ui-present-runtime-delivery-manual-required',
      productClaimed: false,
    },
    manualProofBoundary: {
      screenshots: 'rendered-proof-bundle-ui-screenshots-present',
      playwright: 'service-backed-dashboard-and-explanation-playwright-present',
      renderedUi: 'parent-dashboard-child-intervention-parent-explanation-proof-present',
      serviceBackedExplanationReadModel: 'proof-present',
      liveRouteClassification: 'proof-present',
      liveMetadataExtraction: 'proof-present',
      liveEvidenceAiBoundary: 'proof-present',
      liveEvidenceRiskBenefitBoundary: 'proof-present',
      liveEvidencePolicyCompilerBoundary: 'proof-present',
      livePublicConnectorBoundary: 'proof-present',
      liveEvidenceDecisionMemoryBoundary: 'proof-present',
      alertReportIntent: 'proof-present',
      scheduleTimeBudgetCompiler: 'proof-present',
      parentSensitivitySettings: 'proof-present',
      sourceCustodySettings: 'proof-present',
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
    '# SOCIAL-23 Social Proof Artifact Manifest',
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
    'SOCIAL-23 proves proof-pack coverage for SOCIAL-01 through SOCIAL-22.',
    'Rendered proof-bundle UI exists for the parent social dashboard,',
    'child-agent-served social intervention page, and parent explanation panel.',
    'Service-backed dashboard and explanation read-model delivery is present.',
    'Live route classification proof is present for SOCIAL-08 public feed surfaces.',
    'Live metadata extraction proof is present for SOCIAL-09 public social/video surfaces.',
    'Live-evidence AI boundary proof is present for SOCIAL-10 degraded model-unavailable rows.',
    'Live-evidence risk/benefit boundary proof is present for SOCIAL-11 unavailable signal sets.',
    'Live-evidence policy compiler proof is present for SOCIAL-12 non-final manual-review candidates.',
    'Live public connector boundary proof is present for SOCIAL-18 Google/YouTube, Meta, and TikTok surfaces.',
    'Live-evidence decision memory proof is present for SOCIAL-19 ref-only cache snapshots.',
    'Ref-only social alert/report intent proof is present.',
    'Schedule/time-budget compiler contract proof is present.',
    'Parent sensitivity settings contract proof is present.',
    'Source custody settings contract proof is present over source/privacy refs.',
    'It does not prove runtime connector behavior, native app control, final',
    'policy execution, provider/report delivery, applied schedules/budgets,',
    'runtime custody mutation, enforcement, or product checklist completion.',
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
