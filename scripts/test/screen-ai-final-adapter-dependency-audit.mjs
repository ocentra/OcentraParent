import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputDir = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'final-adapter-dependency-audit');
const proofPath = join(outputDir, 'proof-summary.json');
const snapshotPath = join(outputDir, '00-source-snapshot.md');
const commandsPath = join(outputDir, '10-validation-commands.log');

const sourcePaths = {
  adapterReadiness: 'output/screen-ai-pipeline-proof/adapter-readiness/proof-summary.json',
  adapterReadinessReadModel: 'output/screen-ai-pipeline-proof/adapter-readiness/read-model.json',
  finalProductPath: 'output/screen-ai-pipeline-proof/final-product-path/proof-summary.json',
  checklist: 'docs/plans/screen-ai-pipeline-plan/implementation-checklist.md',
};

const requiredBlockedRows = [
  {
    rowId: 'screen-ai-broad-installed-app-manual-required',
    boundary: 'broad-installed-app-blocking',
    expectedState: 'manual-required',
    missingArtifact:
      'broad installed-app apply, rollback, and audit custody proof from a screen-derived block decision',
  },
  {
    rowId: 'screen-ai-host-network-domain-manual-required',
    boundary: 'host-network-domain-blocking',
    expectedState: 'manual-required',
    missingArtifact:
      'host DNS/filter apply, rollback, and audit custody proof from a screen-derived network/domain decision',
  },
  {
    rowId: 'screen-ai-managed-active-tab-not-claimed',
    boundary: 'managed-exact-active-tab-enforcement',
    expectedState: 'not-claimed',
    missingArtifact:
      'managed active-tab exact URL apply, rollback, and audit custody proof from a screen-derived browser decision',
  },
  {
    rowId: 'screen-ai-android-mobile-control-manual-required',
    boundary: 'android-mobile-control-adapter',
    expectedState: 'manual-required',
    missingArtifact: 'Android device-owner/managed-profile control proof from a screen-derived mobile decision',
  },
  {
    rowId: 'screen-ai-ios-mobile-control-manual-required',
    boundary: 'ios-mobile-control-adapter',
    expectedState: 'manual-required',
    missingArtifact: 'iOS Family Controls/DeviceActivity control proof from a screen-derived mobile decision',
  },
  {
    rowId: 'screen-ai-linux-host-adapter-unavailable',
    boundary: 'linux-host-adapter',
    expectedState: 'unavailable',
    missingArtifact: 'Linux host adapter apply, rollback, and audit custody proof from a screen-derived decision',
  },
];

const failures = [];
const adapterReadiness = readJson(sourcePaths.adapterReadiness);
const adapterReadinessReadModel = readJson(sourcePaths.adapterReadinessReadModel);
const finalProductPath = readJson(sourcePaths.finalProductPath);
const checklist = readText(sourcePaths.checklist);

const rowById = new Map(adapterReadinessReadModel.rows.map((row) => [row.rowId, row]));
const blockedRows = requiredBlockedRows.map((requirement) => auditBlockedRow(requirement));
const executedRows = adapterReadinessReadModel.rows.filter((row) => row.actionExecutionState === 'executed');
const openChecklistRowPresent = checklist.includes(
  '- [ ] Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.'
);

assert(adapterReadiness.proof === 'screen-ai-adapter-readiness-proof', 'adapter readiness proof id mismatch');
assert(finalProductPath.status === 'ok', 'final product path artifact gate is not ok');
assert(finalProductPath.closure?.actionDispatchProven === true, 'Windows action dispatch is not proved');
assert(finalProductPath.closure?.portalReadModelProven === true, 'portal/read-model path is not proved');
assert(finalProductPath.closure?.retentionCustodyProven === true, 'retention/deletion custody is not proved');
assert(adapterReadiness.summary?.executedRows === 2, 'expected exactly two executed Windows owned-process rows');
assert(adapterReadiness.summary?.skippedRows === 6, 'expected six non-product-complete adapter rows');
assert(adapterReadiness.summary?.claimUpgradeRows === 0, 'adapter readiness proof contains claim upgrades');
assert(
  executedRows.every((row) => row.platform === 'windows'),
  'non-Windows row executed unexpectedly'
);
assert(
  openChecklistRowPresent,
  'product-complete adapter checklist row should remain open until upstream artifacts exist'
);

if (failures.length > 0) {
  throw new Error(
    `Screen AI final adapter dependency audit failed:\n${failures.map((failure) => `- ${failure}`).join('\n')}`
  );
}

const proof = {
  status: 'blocked-by-upstream-adapter-artifacts',
  proofKind: 'screen-ai-final-adapter-dependency-audit',
  generatedAt: new Date().toISOString(),
  sourceArtifacts: sourcePaths,
  closure: {
    windowsOwnedProcessAdaptersProved: true,
    finalPathArtifactGateStillValid: true,
    portalReadModelAndDeletionStillProved: true,
    broadBrowserNetworkMobileProductComplete: false,
    openChecklistRowRetained: true,
    executedAdapterRows: executedRows.length,
    blockedAdapterRows: blockedRows.length,
    claimUpgradeRows: adapterReadiness.summary.claimUpgradeRows,
  },
  blockedRows,
  nextRequiredArtifacts: blockedRows.map((row) => ({
    rowId: row.rowId,
    boundary: row.boundary,
    missingArtifact: row.missingArtifact,
  })),
  nonClaims: [
    'This audit does not implement broad installed-app blocking, host network/domain blocking, managed active-tab enforcement, Android/iOS mobile control, or Linux host control.',
    'This audit does not close the product-complete adapter checklist row; it verifies that the row remains open until upstream execution artifacts exist.',
    'This audit reuses retained proof artifacts and does not rerun live capture, local AI inference, portal rendering, or adapter execution.',
  ],
};

writeOutputs(proof);
console.log(`screen-ai-final-adapter-dependency-audit-ok:${relativePath(proofPath)}`);

function auditBlockedRow(requirement) {
  const row = rowById.get(requirement.rowId);
  assert(Boolean(row), `missing adapter readiness row ${requirement.rowId}`);
  if (!row) {
    return { ...requirement, present: false };
  }

  const rowAudit = {
    rowId: row.rowId,
    boundary: requirement.boundary,
    readinessState: row.readinessState,
    actionExecutionState: row.actionExecutionState,
    adapterExecutionProofArtifact: row.adapterExecutionProofArtifact,
    rawImageRetained: row.rawImageRetained,
    rawImageDeletedBeforeAdapter: row.rawImageDeletedBeforeAdapter,
    claimFlagsAllFalse: Object.values(row.claimFlags).every((flag) => flag === false),
    missingArtifact: requirement.missingArtifact,
  };

  assert(row.readinessState === requirement.expectedState, `${requirement.rowId} readiness state changed`);
  assert(row.actionExecutionState === 'skipped', `${requirement.rowId} unexpectedly executed`);
  assert(row.adapterExecutionProofArtifact === null, `${requirement.rowId} unexpectedly has adapter proof`);
  assert(row.rawImageRetained === false, `${requirement.rowId} retained raw image`);
  assert(row.rawImageDeletedBeforeAdapter === true, `${requirement.rowId} lacks deleted-image custody`);
  assert(rowAudit.claimFlagsAllFalse, `${requirement.rowId} contains claim flag upgrade`);

  return rowAudit;
}

function readJson(path) {
  return JSON.parse(readText(path));
}

function readText(path) {
  const absolute = resolve(repoRoot, path);
  assert(existsSync(absolute), `missing artifact ${path}`);
  return readFileSync(absolute, 'utf8');
}

function writeOutputs(proof) {
  mkdirSync(outputDir, { recursive: true });
  writeFileSync(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  writeFileSync(snapshotPath, snapshot(proof));
  writeFileSync(commandsPath, validationCommands());
}

function snapshot(proof) {
  const sources = Object.entries(sourcePaths)
    .map(([name, path]) => `- ${name}: \`${path}\``)
    .join('\n');
  const blockers = proof.blockedRows
    .map((row) => `- ${row.rowId}: ${row.readinessState}, ${row.missingArtifact}`)
    .join('\n');
  return `# Screen AI Final Adapter Dependency Audit\n\nGenerated: ${proof.generatedAt}\n\n## Source Artifacts\n\n${sources}\n\n## Blocked Adapter Rows\n\n${blockers}\n\n## Closure\n\n\`\`\`json\n${JSON.stringify(proof.closure, null, 2)}\n\`\`\`\n`;
}

function validationCommands() {
  return [
    'node --check scripts/test/screen-ai-final-adapter-dependency-audit.mjs',
    'node scripts/test/screen-ai-final-adapter-dependency-audit.mjs',
    'git diff --check',
    'npm run lanes:guard',
    'npm run hub:guard',
    '',
  ].join('\n');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function assert(condition, message) {
  if (!condition) {
    failures.push(message);
  }
}
