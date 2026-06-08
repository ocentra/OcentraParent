import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputDir = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'adapter-dependency-handoff');
const proofPath = join(outputDir, 'proof-summary.json');
const handoffPath = join(outputDir, 'adapter-dependency-handoff.json');
const markdownPath = join(outputDir, 'adapter-dependency-handoff.md');
const commandsPath = join(outputDir, '10-validation-commands.log');

const sourceArtifacts = {
  finalAdapterDependencyAudit: 'output/screen-ai-pipeline-proof/final-adapter-dependency-audit/proof-summary.json',
  adapterBlockerLedger: 'output/screen-ai-pipeline-proof/adapter-blocker-ledger/proof-summary.json',
  finalProductPath: 'output/screen-ai-pipeline-proof/final-product-path/proof-summary.json',
  upstreamAdapterPrerequisiteBridge:
    'output/screen-ai-pipeline-proof/upstream-adapter-prerequisite-bridge/proof-summary.json',
  screenAiPipelineChecklist: 'docs/plans/screen-ai-pipeline-plan/implementation-checklist.md',
};

const expectedHandoff = {
  'screen-ai-broad-installed-app-manual-required': {
    owningLane: 'codex-c',
    owningDomain: 'app-game/enforcement adapter layer',
    currentUpstreamContextFiles: [
      'output/app-game-plan-proof/23-broad-blocking-proof-gates/03-runtime-evidence.json',
      'test-results/app-install-purchase-package-source-adapter-execution-proof/proof.json',
      'test-results/app-install-purchase-provider-store-api-execution-proof/proof.json',
      'test-results/app-install-purchase-external-runtime-writer-transport-execution-proof/proof.json',
    ],
    expectedProofFile:
      'output/app-game-plan-proof/screen-derived-broad-installed-app-apply-rollback-audit/proof-summary.json',
    expectedContractShape: {
      sourcePolicyDecisionRef: 'screen-derived block/time-limit policy decision id',
      sourceActivityEvidenceRef: 'screen analysis/activity evidence ref',
      applyResultRef: 'real broad installed-app adapter apply result',
      rollbackOrExpiryRef: 'rollback or expiry result for the same target',
      auditRef: 'durable adapter audit/custody ref',
      rawImageRetained: false,
      rawImageDeletedBeforeAdapter: true,
      appBlockingClaimed: true,
      finalAdapterCompletionClaimed: true,
    },
    unblocksFinalRows: [
      'screen-ai-pipeline-plan: Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.',
      'product-capability-checklist: Local screen evidence summaries',
      'product-capability-checklist: Child-safety AI decision',
    ],
  },
  'screen-ai-host-network-domain-manual-required': {
    owningLane: 'E-D',
    owningDomain: 'network/domain enforcement adapter layer',
    expectedProofFile:
      'output/network-plan-proof/screen-derived-host-network-domain-apply-rollback-audit/proof-summary.json',
    expectedContractShape: {
      sourcePolicyDecisionRef: 'screen-derived network/domain policy decision id',
      sourceNetworkEvidenceRef: 'host/domain/IP evidence ref',
      applyResultRef: 'real DNS/filter/firewall apply result',
      rollbackOrExpiryRef: 'rollback or expiry result for the same rule',
      auditRef: 'durable network adapter audit/custody ref',
      rawImageRetained: false,
      rawImageDeletedBeforeAdapter: true,
      finalAdapterCompletionClaimed: true,
    },
    unblocksFinalRows: [
      'screen-ai-pipeline-plan: Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.',
      'product-capability-checklist: Local screen evidence summaries',
      'product-capability-checklist: Child-safety AI decision',
    ],
  },
  'screen-ai-managed-active-tab-not-claimed': {
    owningLane: 'codex-d',
    owningDomain: 'browser managed-control adapter layer',
    expectedProofFile:
      'output/browser-plan-proof/screen-derived-managed-active-tab-apply-rollback-audit/proof-summary.json',
    expectedContractShape: {
      sourcePolicyDecisionRef: 'screen-derived browser policy decision id',
      sourceBrowserEvidenceRef: 'managed active-tab URL/evidence ref',
      applyResultRef: 'real exact active-tab adapter apply result',
      rollbackOrExpiryRef: 'tab/action rollback or expiry result',
      auditRef: 'durable browser adapter audit/custody ref',
      rawImageRetained: false,
      rawImageDeletedBeforeAdapter: true,
      finalAdapterCompletionClaimed: true,
    },
    unblocksFinalRows: [
      'screen-ai-pipeline-plan: Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.',
      'product-capability-checklist: Local screen evidence summaries',
      'product-capability-checklist: Child-safety AI decision',
    ],
  },
  'screen-ai-android-mobile-control-manual-required': {
    owningLane: 'primary/mobile-child-agent-sequencing',
    owningDomain: 'Android child-agent Device Owner or managed-profile adapter layer',
    expectedProofFile:
      'output/mobile-plan-proof/screen-derived-android-mobile-control-apply-rollback-audit/proof-summary.json',
    expectedContractShape: {
      sourcePolicyDecisionRef: 'screen-derived mobile policy decision id',
      sourceMobileEvidenceRef: 'Android child-agent/device evidence ref',
      applyResultRef: 'real Device Owner, managed-profile, UsageStats, Accessibility, or VPN/DNS apply result',
      rollbackOrExpiryRef: 'rollback or expiry result for the same mobile control',
      auditRef: 'durable Android adapter audit/custody ref',
      rawImageRetained: false,
      rawImageDeletedBeforeAdapter: true,
      finalAdapterCompletionClaimed: true,
    },
    unblocksFinalRows: [
      'screen-ai-pipeline-plan: Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.',
      'product-capability-checklist: Local screen evidence summaries',
      'product-capability-checklist: Child-safety AI decision',
    ],
  },
  'screen-ai-ios-mobile-control-manual-required': {
    owningLane: 'primary/mobile-child-agent-sequencing',
    owningDomain: 'iOS Family Controls and DeviceActivity adapter layer',
    expectedProofFile:
      'output/mobile-plan-proof/screen-derived-ios-mobile-control-apply-rollback-audit/proof-summary.json',
    expectedContractShape: {
      sourcePolicyDecisionRef: 'screen-derived mobile policy decision id',
      sourceMobileEvidenceRef: 'iOS child-agent/device evidence ref',
      applyResultRef: 'real Family Controls, DeviceActivity, or Network Extension apply result',
      rollbackOrExpiryRef: 'rollback or expiry result for the same mobile control',
      auditRef: 'durable iOS adapter audit/custody ref',
      rawImageRetained: false,
      rawImageDeletedBeforeAdapter: true,
      finalAdapterCompletionClaimed: true,
    },
    unblocksFinalRows: [
      'screen-ai-pipeline-plan: Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.',
      'product-capability-checklist: Local screen evidence summaries',
      'product-capability-checklist: Child-safety AI decision',
    ],
  },
};

const failures = [];
const finalAdapterAudit = readJson(sourceArtifacts.finalAdapterDependencyAudit);
const adapterBlockerLedger = readJson(sourceArtifacts.adapterBlockerLedger);
const finalProductPath = readJson(sourceArtifacts.finalProductPath);
const upstreamAdapterPrerequisiteBridge = readJson(sourceArtifacts.upstreamAdapterPrerequisiteBridge);
const checklist = readText(sourceArtifacts.screenAiPipelineChecklist);

assert(finalAdapterAudit.status === 'blocked-by-upstream-adapter-artifacts', 'final adapter audit is not blocked');
assert(finalAdapterAudit.closure?.blockedAdapterRows === 5, 'final adapter audit blocker count changed');
assert(finalAdapterAudit.closure?.linuxHostExecutionRows === 1, 'final adapter audit lost Linux execution row');
assert(adapterBlockerLedger.status === 'blocked-but-actionable', 'adapter blocker ledger is not actionable');
assert(adapterBlockerLedger.closure?.blockerRows === 5, 'adapter blocker ledger blocker count changed');
assert(finalProductPath.closure?.finalPipelineProductComplete === false, 'final product path claims complete');
assert(
  upstreamAdapterPrerequisiteBridge.closure?.appInstallPackageSourceExecutionPresent === true,
  'upstream bridge must consume current app-install package-source execution context'
);
assert(
  upstreamAdapterPrerequisiteBridge.closure?.appInstallExternalWriterTransportStillBlocked === true,
  'upstream bridge must keep app-install external writer transport blocked'
);
assert(
  upstreamAdapterPrerequisiteBridge.closure?.appInstallAppBlockingClaimed === false,
  'upstream bridge unexpectedly claims app-install app blocking'
);
assert(
  checklist.includes(
    '- [ ] Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.'
  ),
  'final screen-ai adapter row is not open'
);

const handoffRows = finalAdapterAudit.blockedRows.map((blockedRow) => {
  const ledgerRow = adapterBlockerLedger.rows.find((row) => row.rowId === blockedRow.rowId);
  const expected = expectedHandoff[blockedRow.rowId];
  assert(Boolean(ledgerRow), `missing ledger row ${blockedRow.rowId}`);
  assert(Boolean(expected), `missing expected handoff ${blockedRow.rowId}`);
  assert(blockedRow.adapterExecutionProofArtifact === null, `${blockedRow.rowId} unexpectedly has execution proof`);
  assert(blockedRow.rawImageRetained === false, `${blockedRow.rowId} retains raw image`);
  assert(blockedRow.rawImageDeletedBeforeAdapter === true, `${blockedRow.rowId} does not delete image before adapter`);
  assert(blockedRow.claimFlagsAllFalse === true, `${blockedRow.rowId} has upgraded claim flags`);
  assert(ledgerRow.claimUpgradeAllowed === false, `${blockedRow.rowId} allows claim upgrade`);
  return {
    rowId: blockedRow.rowId,
    adapterClass: ledgerRow.adapterClass,
    readinessState: blockedRow.readinessState,
    actionExecutionState: blockedRow.actionExecutionState,
    currentMissingArtifact: blockedRow.missingArtifact,
    owningLane: expected.owningLane,
    owningDomain: expected.owningDomain,
    currentUpstreamContextFiles: expected.currentUpstreamContextFiles ?? [],
    expectedProofFile: expected.expectedProofFile,
    expectedContractShape: expected.expectedContractShape,
    unblocksFinalRows: expected.unblocksFinalRows,
    claimGate: ledgerRow.claimGate,
  };
});

assert(handoffRows.length === 5, 'handoff row count changed');

for (const row of handoffRows) {
  assert(row.expectedContractShape.rawImageRetained === false, `${row.rowId} expected shape retains raw image`);
  assert(
    row.expectedContractShape.rawImageDeletedBeforeAdapter === true,
    `${row.rowId} expected shape does not require deletion before adapter`
  );
  assert(
    row.expectedContractShape.finalAdapterCompletionClaimed === true,
    `${row.rowId} expected shape does not declare final adapter completion for completed upstream proof`
  );
  if (row.rowId === 'screen-ai-broad-installed-app-manual-required') {
    assert(
      row.expectedContractShape.appBlockingClaimed === true,
      `${row.rowId} expected shape must require explicit app blocking claim`
    );
    assert(
      row.currentUpstreamContextFiles.length === 4,
      `${row.rowId} must name the current upstream app/game and app-install context artifacts`
    );
  }
}

if (failures.length > 0) {
  throw new Error(
    `Screen AI adapter dependency handoff proof failed:\n${failures.map((failure) => `- ${failure}`).join('\n')}`
  );
}

const generatedAt = new Date().toISOString();
const handoff = {
  schemaVersion: 'v0.6',
  generatedAt,
  sourceArtifacts,
  rows: handoffRows,
  claimBoundary:
    'These rows are remaining dependency handoff requirements only. They do not prove execution until the expected proof files exist and satisfy the expected contract shape.',
};
const proof = {
  status: 'adapter-dependency-handoff-ready-upstream-execution-required',
  proofKind: 'screen-ai-adapter-dependency-handoff-proof',
  generatedAt,
  sourceArtifacts,
  handoff: relativePath(handoffPath),
  handoffMarkdown: relativePath(markdownPath),
  closure: {
    dependencyRowsMapped: handoffRows.length,
    linuxWsl2HostExecutionAlreadyProved: true,
    owningLanesMapped: true,
    expectedProofFilesMapped: true,
    expectedContractShapesMapped: true,
    upstreamAppInstallContextMappedWithoutClaimUpgrade: true,
    finalScreenAiAdapterRowStillOpen: true,
    productChecklistEdited: false,
    productCompleteClaimed: false,
    rawImageRetainedByExpectedContracts: false,
  },
  nonClaims: [
    'This proof does not implement broad installed-app, host network/domain, managed active-tab, Android, iOS, or native Linux desktop product-complete adapters.',
    'This proof does not edit docs/product-capability-checklist.md while another lane owns the file.',
    'This proof does not close the final screen-AI adapter row or mark product-complete execution.',
  ],
};

mkdirSync(outputDir, { recursive: true });
writeFileSync(handoffPath, `${JSON.stringify(handoff, null, 2)}\n`);
writeFileSync(markdownPath, markdown(handoffRows, generatedAt));
writeFileSync(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(commandsPath, validationCommands());
console.log(`screen-ai-adapter-dependency-handoff-proof-ok:${relativePath(proofPath)}`);

function markdown(rows, generatedAt) {
  const blocks = rows.map((row) =>
    [
      `## ${row.rowId}`,
      '',
      `- adapter class: ${row.adapterClass}`,
      `- owner: ${row.owningLane} (${row.owningDomain})`,
      `- expected proof: \`${row.expectedProofFile}\``,
      `- current upstream context: ${row.currentUpstreamContextFiles.map((path) => `\`${path}\``).join(', ') || 'none'}`,
      `- missing now: ${row.currentMissingArtifact}`,
      `- unblocks: ${row.unblocksFinalRows.join('; ')}`,
      '',
      '```json',
      JSON.stringify(row.expectedContractShape, null, 2),
      '```',
      '',
    ].join('\n')
  );
  return [`# Screen AI Adapter Dependency Handoff`, '', `Generated: ${generatedAt}`, '', ...blocks].join('\n');
}

function readJson(path) {
  return JSON.parse(readText(path));
}

function readText(path) {
  const absolute = resolve(repoRoot, path);
  assert(existsSync(absolute), `missing source artifact ${path}`);
  return readFileSync(absolute, 'utf8');
}

function validationCommands() {
  return [
    'node --check scripts/test/screen-ai-adapter-dependency-handoff-proof.mjs',
    'node scripts/test/screen-ai-adapter-dependency-handoff-proof.mjs',
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
