import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'final-adapter-dependency-audit');
const proofPath = join(outputDir, 'proof-summary.json');
const snapshotPath = join(outputDir, '00-source-snapshot.md');
const commandsPath = join(outputDir, '10-validation-commands.log');

const sourcePaths = {
  adapterReadiness: 'output/screen-ai-pipeline-proof/adapter-readiness/proof-summary.json',
  adapterReadinessReadModel: 'output/screen-ai-pipeline-proof/adapter-readiness/read-model.json',
  finalProductPath: 'output/screen-ai-pipeline-proof/final-product-path/proof-summary.json',
  linuxHostCustody: 'output/screen-ai-pipeline-proof/linux-host-adapter-custody/proof-summary.json',
  linuxHostExecution: 'output/screen-ai-pipeline-proof/linux-host-adapter-execution/proof-summary.json',
  androidMobileCustody: 'output/screen-ai-pipeline-proof/android-mobile-control-custody/proof-summary.json',
  iosMobileCustody: 'output/screen-ai-pipeline-proof/ios-mobile-control-custody/proof-summary.json',
  adapterDependencyHandoff: 'output/screen-ai-pipeline-proof/adapter-dependency-handoff/proof-summary.json',
  adapterDependencyHandoffRows:
    'output/screen-ai-pipeline-proof/adapter-dependency-handoff/adapter-dependency-handoff.json',
  screenAiAdapterReadinessContract: 'packages/schema-domain/dist/screen-ai-adapter-readiness-proof.js',
  screenAiAdapterReadinessContractSource: 'packages/schema-domain/src/screen-ai-adapter-readiness-proof.ts',
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
];

const failures = [];
const adapterReadiness = readJson(sourcePaths.adapterReadiness);
const adapterReadinessReadModel = readJson(sourcePaths.adapterReadinessReadModel);
const finalProductPath = readJson(sourcePaths.finalProductPath);
const linuxHostCustody = readJson(sourcePaths.linuxHostCustody);
const linuxHostExecution = readJson(sourcePaths.linuxHostExecution);
const androidMobileCustody = readJson(sourcePaths.androidMobileCustody);
const iosMobileCustody = readJson(sourcePaths.iosMobileCustody);
const adapterDependencyHandoff = readJson(sourcePaths.adapterDependencyHandoff);
const adapterDependencyHandoffRows = readJson(sourcePaths.adapterDependencyHandoffRows);
const checklist = readText(sourcePaths.checklist);
const screenAiAdapterContracts = await importScreenAiAdapterContracts();
const parsedAdapterReadinessReadModel =
  screenAiAdapterContracts.ScreenAiAdapterReadinessReadModelSchema.parse(adapterReadinessReadModel);
const availableCompletionArtifacts = loadAvailableCompletionArtifacts(adapterDependencyHandoffRows.rows);
const finalAdapterCompletionGate = screenAiAdapterContracts.screenAiFinalAdapterCompletionGate(
  parsedAdapterReadinessReadModel,
  availableCompletionArtifacts
);

const rowById = new Map(adapterReadinessReadModel.rows.map((row) => [row.rowId, row]));
const blockedRows = requiredBlockedRows.map((requirement) => auditBlockedRow(requirement));
const custodyRows = [
  auditCustodyArtifact({
    rowId: 'screen-ai-linux-host-adapter-unavailable',
    proof: linuxHostCustody,
    expectedStatus: 'linux-host-custody-artifact-written-final-execution-blocked',
    closureKey: 'linuxHostApplyCustodyRecorded',
    executionKey: 'linuxHostApplyExecuted',
    artifactPath: sourcePaths.linuxHostCustody,
  }),
  auditCustodyArtifact({
    rowId: 'screen-ai-android-mobile-control-manual-required',
    proof: androidMobileCustody,
    expectedStatus: 'android-mobile-control-custody-artifact-written-final-execution-blocked',
    closureKey: 'androidMobileApplyCustodyRecorded',
    executionKey: 'androidMobileApplyExecuted',
    artifactPath: sourcePaths.androidMobileCustody,
  }),
  auditCustodyArtifact({
    rowId: 'screen-ai-ios-mobile-control-manual-required',
    proof: iosMobileCustody,
    expectedStatus: 'ios-mobile-control-custody-artifact-written-final-execution-blocked',
    closureKey: 'iosMobileApplyCustodyRecorded',
    executionKey: 'iosMobileApplyExecuted',
    artifactPath: sourcePaths.iosMobileCustody,
  }),
];
const handoffRows = auditDependencyHandoff();
const linuxExecutionRow = auditLinuxExecutionArtifact();
const executedRows = adapterReadinessReadModel.rows.filter((row) => row.actionExecutionState === 'executed');
const openChecklistRowPresent = checklist.includes(
  '- [ ] Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.'
);

assert(adapterReadiness.proof === 'screen-ai-adapter-readiness-proof', 'adapter readiness proof id mismatch');
assert(finalProductPath.status === 'ok', 'final product path artifact gate is not ok');
assert(finalProductPath.closure?.actionDispatchProven === true, 'Windows action dispatch is not proved');
assert(finalProductPath.closure?.portalReadModelProven === true, 'portal/read-model path is not proved');
assert(finalProductPath.closure?.retentionCustodyProven === true, 'retention/deletion custody is not proved');
assert(
  finalProductPath.closure?.singleRuntimeSessionRerun === true,
  'final product path did not retain the fresh service OCR rerun proof'
);
assert(adapterReadiness.summary?.executedRows === 2, 'expected exactly two executed Windows owned-process rows');
assert(adapterReadiness.summary?.skippedRows === 6, 'expected six non-product-complete adapter rows');
assert(adapterReadiness.summary?.claimUpgradeRows === 0, 'adapter readiness proof contains claim upgrades');
assert(finalAdapterCompletionGate.completed === false, 'final adapter completion gate closed unexpectedly');
assert(finalAdapterCompletionGate.requiredRows === 5, 'final adapter completion gate required row count changed');
assert(
  finalAdapterCompletionGate.missingRows.length > 0,
  'final adapter completion gate should still require upstream completion artifacts'
);
assert(
  executedRows.every((row) => row.platform === 'windows'),
  'non-Windows row executed unexpectedly'
);
assert(
  openChecklistRowPresent,
  'product-complete adapter checklist row should remain open until upstream artifacts exist'
);
assert(
  checklist.includes('`output/screen-ai-pipeline-proof/linux-host-adapter-custody/proof-summary.json`'),
  'screen-ai checklist must cite Linux custody artifact'
);
assert(
  checklist.includes('`output/screen-ai-pipeline-proof/android-mobile-control-custody/proof-summary.json`'),
  'screen-ai checklist must cite Android custody artifact'
);
assert(
  checklist.includes('`output/screen-ai-pipeline-proof/ios-mobile-control-custody/proof-summary.json`'),
  'screen-ai checklist must cite iOS custody artifact'
);
assert(
  checklist.includes('`output/screen-ai-pipeline-proof/adapter-dependency-handoff/proof-summary.json`'),
  'screen-ai checklist must cite adapter dependency handoff artifact'
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
    finalPathFreshServiceRerunProved: true,
    portalReadModelAndDeletionStillProved: true,
    broadBrowserNetworkMobileProductComplete: false,
    openChecklistRowRetained: true,
    executedAdapterRows: executedRows.length,
    linuxHostExecutionRows: 1,
    blockedAdapterRows: blockedRows.length,
    custodyArtifactRows: custodyRows.length,
    dependencyHandoffRows: handoffRows.length,
    finalAdapterCompletionGateSatisfied: finalAdapterCompletionGate.completed,
    finalAdapterCompletionGateCompletedRows: finalAdapterCompletionGate.completedRows,
    finalAdapterCompletionGateMissingRows: finalAdapterCompletionGate.missingRows.length,
    finalAdapterCompletionGateInvalidRows: finalAdapterCompletionGate.invalidRows.length,
    claimUpgradeRows: adapterReadiness.summary.claimUpgradeRows,
  },
  blockedRows,
  linuxExecutionRow,
  custodyRows,
  dependencyHandoffRows: handoffRows,
  finalAdapterCompletionGate,
  nextRequiredArtifacts: blockedRows.map((row) => ({
    rowId: row.rowId,
    boundary: row.boundary,
    custodyArtifact: row.custodyArtifact ?? null,
    missingArtifact: row.missingArtifact,
    handoffOwner: handoffRows.find((handoffRow) => handoffRow.rowId === row.rowId)?.owningLane ?? null,
    expectedProofFile: handoffRows.find((handoffRow) => handoffRow.rowId === row.rowId)?.expectedProofFile ?? null,
  })),
  nonClaims: [
    'This audit does not implement broad installed-app blocking, host network/domain blocking, managed active-tab enforcement, or Android/iOS mobile control.',
    'This audit consumes the WSL2 Linux host execution artifact but does not close the product-complete adapter checklist row; it verifies that the row remains open until the remaining upstream execution artifacts exist.',
    'This audit consumes the final product-path proof after its service OCR policy source rerun assertion, but does not rerun live capture, local AI inference, portal rendering, or adapter execution itself.',
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

function auditCustodyArtifact({ rowId, proof, expectedStatus, closureKey, executionKey, artifactPath }) {
  assert(proof.status === expectedStatus, `${rowId} custody proof status changed`);
  assert(proof.closure?.screenDerivedBlockDecisionPreserved === true, `${rowId} lost screen-derived decision custody`);
  assert(proof.closure?.[closureKey] === true, `${rowId} apply custody is not recorded`);
  assert(proof.closure?.[executionKey] === false, `${rowId} execution unexpectedly claimed`);
  assert(proof.closure?.finalAdapterCompletionClaimed === false, `${rowId} claimed final adapter completion`);
  assert(proof.closure?.productCompleteAdapterRowStillOpen === true, `${rowId} closed product-complete row`);

  const blockedRow = blockedRows.find((row) => row.rowId === rowId);
  if (blockedRow) {
    blockedRow.custodyArtifact = artifactPath;
    blockedRow.custodyRecorded = true;
  }

  return {
    rowId,
    artifactPath,
    status: proof.status,
    screenDerivedBlockDecisionPreserved: proof.closure?.screenDerivedBlockDecisionPreserved === true,
    applyCustodyRecorded: proof.closure?.[closureKey] === true,
    executionClaimed: proof.closure?.[executionKey] === true,
    finalAdapterCompletionClaimed: proof.closure?.finalAdapterCompletionClaimed === true,
    productCompleteAdapterRowStillOpen: proof.closure?.productCompleteAdapterRowStillOpen === true,
  };
}

function auditLinuxExecutionArtifact() {
  assert(
    linuxHostExecution.status === 'linux-host-adapter-execution-proved-wsl2',
    'Linux host execution proof status changed'
  );
  assert(
    linuxHostExecution.closure?.screenDerivedBlockDecisionPreserved === true,
    'Linux execution lost screen-derived decision custody'
  );
  assert(
    linuxHostExecution.closure?.rawImageDeletedBeforeAdapter === true,
    'Linux execution lost deleted-image custody'
  );
  assert(linuxHostExecution.closure?.rawImageRetained === false, 'Linux execution retained raw image');
  assert(linuxHostExecution.closure?.linuxWsl2HostMutationExecuted === true, 'Linux WSL2 host mutation not executed');
  assert(linuxHostExecution.closure?.linuxWsl2RollbackExecuted === true, 'Linux WSL2 rollback not clean');
  assert(linuxHostExecution.closure?.linuxExecutionAuditRecorded === true, 'Linux execution audit not recorded');
  assert(linuxHostExecution.closure?.finalAdapterCompletionClaimed === false, 'Linux execution claimed completion');
  assert(
    linuxHostExecution.closure?.nativeLinuxDesktopProductReady === false,
    'Linux execution overclaimed desktop readiness'
  );

  return {
    rowId: 'screen-ai-linux-host-adapter-unavailable',
    artifactPath: sourcePaths.linuxHostExecution,
    status: linuxHostExecution.status,
    screenDerivedBlockDecisionPreserved: true,
    rawImageDeletedBeforeAdapter: true,
    executionClaimed: true,
    rollbackExecuted: true,
    finalAdapterCompletionClaimed: false,
    nativeLinuxDesktopProductReady: false,
  };
}

function auditDependencyHandoff() {
  assert(
    adapterDependencyHandoff.status === 'adapter-dependency-handoff-ready-upstream-execution-required',
    'adapter dependency handoff status changed'
  );
  assert(
    adapterDependencyHandoff.closure?.dependencyRowsMapped === requiredBlockedRows.length,
    'adapter dependency handoff row count changed'
  );
  assert(
    adapterDependencyHandoff.closure?.linuxWsl2HostExecutionAlreadyProved === true,
    'adapter dependency handoff lost completed Linux execution marker'
  );
  assert(
    adapterDependencyHandoff.closure?.finalScreenAiAdapterRowStillOpen === true,
    'adapter dependency handoff closed final adapter row'
  );
  assert(
    adapterDependencyHandoff.closure?.productChecklistEdited === false,
    'adapter dependency handoff edited product checklist'
  );
  assert(
    adapterDependencyHandoff.closure?.productCompleteClaimed === false,
    'adapter dependency handoff claims product complete'
  );
  assert(
    adapterDependencyHandoff.closure?.upstreamAppInstallContextMappedWithoutClaimUpgrade === true,
    'adapter dependency handoff must map app-install context without upgrading screen adapter claims'
  );
  assert(
    adapterDependencyHandoffRows.claimBoundary?.includes('dependency handoff requirements only'),
    'adapter dependency handoff lost claim boundary'
  );

  const rows = adapterDependencyHandoffRows.rows ?? [];
  assert(rows.length === requiredBlockedRows.length, 'adapter dependency handoff rows length mismatch');

  for (const requirement of requiredBlockedRows) {
    const row = rows.find((candidate) => candidate.rowId === requirement.rowId);
    assert(Boolean(row), `missing adapter dependency handoff row ${requirement.rowId}`);
    if (!row) {
      continue;
    }
    assert(Boolean(row.owningLane), `${requirement.rowId} handoff missing owning lane`);
    assert(Boolean(row.owningDomain), `${requirement.rowId} handoff missing owning domain`);
    assert(Boolean(row.expectedProofFile), `${requirement.rowId} handoff missing expected proof file`);
    assert(
      row.expectedContractShape?.rawImageRetained === false,
      `${requirement.rowId} handoff expected contract retains raw image`
    );
    assert(
      row.expectedContractShape?.rawImageDeletedBeforeAdapter === true,
      `${requirement.rowId} handoff expected contract does not require deletion before adapter`
    );
    assert(
      row.expectedContractShape?.finalAdapterCompletionClaimed === true,
      `${requirement.rowId} handoff expected contract cannot close completion when proof exists`
    );
    if (requirement.rowId === 'screen-ai-broad-installed-app-manual-required') {
      assert(
        row.expectedContractShape?.appBlockingClaimed === true,
        `${requirement.rowId} handoff must require explicit app blocking claim`
      );
      assert(
        row.currentUpstreamContextFiles?.length === 4,
        `${requirement.rowId} handoff must map app/game and app-install context artifacts`
      );
    }
    assert(
      (row.unblocksFinalRows ?? []).some((value) => value.startsWith('screen-ai-pipeline-plan:')),
      `${requirement.rowId} handoff does not name screen-ai pipeline row`
    );
  }

  return rows.map((row) => ({
    rowId: row.rowId,
    adapterClass: row.adapterClass,
    owningLane: row.owningLane,
    owningDomain: row.owningDomain,
    expectedProofFile: row.expectedProofFile,
    expectedContractShape: row.expectedContractShape,
  }));
}

async function importScreenAiAdapterContracts() {
  const contractPath = resolve(repoRoot, sourcePaths.screenAiAdapterReadinessContract);
  assert(existsSync(contractPath), 'screen AI adapter readiness contract must be built before this audit runs');
  return import(pathToFileURL(contractPath).href);
}

function loadAvailableCompletionArtifacts(handoffRows) {
  return handoffRows
    .map((row) => row.expectedProofFile)
    .filter((path) => typeof path === 'string' && existsSync(resolve(repoRoot, path)))
    .map((path) => screenAiAdapterContracts.ScreenAiAdapterCompletionArtifactSchema.parse(readJson(path)));
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
  const custody = proof.custodyRows
    .map((row) => `- ${row.rowId}: ${row.status}, executionClaimed=${row.executionClaimed}`)
    .join('\n');
  const handoff = proof.dependencyHandoffRows
    .map((row) => `- ${row.rowId}: ${row.owningLane}, ${row.expectedProofFile}`)
    .join('\n');
  return `# Screen AI Final Adapter Dependency Audit\n\nGenerated: ${proof.generatedAt}\n\n## Source Artifacts\n\n${sources}\n\n## Blocked Adapter Rows\n\n${blockers}\n\n## Custody Artifacts\n\n${custody}\n\n## Dependency Handoff Rows\n\n${handoff}\n\n## Closure\n\n\`\`\`json\n${JSON.stringify(proof.closure, null, 2)}\n\`\`\`\n`;
}

function validationCommands() {
  return [
    'npm run build --workspace @ocentra-parent/schema-domain',
    'npm run build --workspace @ocentra-parent/enforcement-domain',
    'npm run test --workspace @ocentra-parent/screen-domain -- screen-ai-adapter-readiness-proof.test.ts',
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
