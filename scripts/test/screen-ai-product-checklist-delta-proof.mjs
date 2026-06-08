import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputDir = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'product-checklist-delta');
const proofPath = join(outputDir, 'proof-summary.json');
const deltaPath = join(outputDir, 'product-capability-checklist-delta.md');
const commandsPath = join(outputDir, '10-validation-commands.log');

const sourceArtifacts = {
  productChecklist: 'docs/product-capability-checklist.md',
  finalProductPath: 'output/screen-ai-pipeline-proof/final-product-path/proof-summary.json',
  finalAdapterAudit: 'output/screen-ai-pipeline-proof/final-adapter-dependency-audit/proof-summary.json',
  adapterBlockerLedger: 'output/screen-ai-pipeline-proof/adapter-blocker-ledger/proof-summary.json',
  linuxHostExecution: 'output/screen-ai-pipeline-proof/linux-host-adapter-execution/proof-summary.json',
  aiPlanClosure: 'output/ai-plan-proof/local-ai-plan-closure-audit/proof-summary.json',
  screenPlanClosure: 'output/screen-plan-proof/screen-plan-closure-audit/proof-summary.json',
};

const failures = [];
const productChecklist = readText(sourceArtifacts.productChecklist);
const finalProductPath = readJson(sourceArtifacts.finalProductPath);
const finalAdapterAudit = readJson(sourceArtifacts.finalAdapterAudit);
const adapterBlockerLedger = readJson(sourceArtifacts.adapterBlockerLedger);
const linuxHostExecution = readJson(sourceArtifacts.linuxHostExecution);
const aiPlanClosure = readJson(sourceArtifacts.aiPlanClosure);
const screenPlanClosure = readJson(sourceArtifacts.screenPlanClosure);

const currentRows = {
  localScreenEvidenceSummaries: findTableRow(productChecklist, 'Local screen evidence summaries'),
  childSafetyAiDecision: findTableRow(productChecklist, 'Child-safety AI decision'),
};

assert(Boolean(currentRows.localScreenEvidenceSummaries), 'missing Local screen evidence summaries row');
assert(Boolean(currentRows.childSafetyAiDecision), 'missing Child-safety AI decision row');
assert(finalProductPath.closure?.screenAndAiPrerequisitesStacked === true, 'final path does not stack screen and AI');
assert(
  finalProductPath.closure?.singleRuntimeSessionRerun === true,
  'final path does not preserve the fresh service OCR rerun proof'
);
assert(finalProductPath.closure?.finalPipelineProductComplete === false, 'final path claims product complete');
assert(
  finalProductPath.closure?.finalPipelineProductCompleteBlockedByAdapterGate === true,
  'final path does not keep adapter gate blocked'
);
assert(finalAdapterAudit.closure?.broadBrowserNetworkMobileProductComplete === false, 'adapter audit overclaims');
assert(finalAdapterAudit.closure?.blockedAdapterRows === 5, 'adapter audit blocker count changed');
assert(finalAdapterAudit.closure?.custodyArtifactRows === 3, 'adapter audit custody count changed');
assert(finalAdapterAudit.closure?.linuxHostExecutionRows === 1, 'adapter audit lost Linux execution row');
assert(adapterBlockerLedger.closure?.blockerRows === 5, 'adapter blocker ledger row count changed');
assert(
  adapterBlockerLedger.closure?.linuxWsl2HostExecutionNoLongerBlocked === true,
  'adapter blocker ledger still treats Linux execution as blocked'
);
assert(
  linuxHostExecution.status === 'linux-host-adapter-execution-proved-wsl2',
  'Linux execution proof status changed'
);
assert(
  linuxHostExecution.closure?.linuxWsl2HostMutationExecuted === true,
  'Linux execution proof did not mutate WSL2 host target'
);
assert(
  linuxHostExecution.closure?.linuxWsl2RollbackExecuted === true,
  'Linux execution proof did not roll back WSL2 host target'
);
assert(
  linuxHostExecution.closure?.nativeLinuxDesktopProductReady === false,
  'Linux execution proof overclaims native desktop readiness'
);
assert(aiPlanClosure.closure?.finalProductCompleteDeferredToPipeline === true, 'AI closure does not defer final path');
assert(screenPlanClosure.assertions?.noProductCompleteClaim === true, 'screen closure claims product complete');
assert(
  screenPlanClosure.assertions?.serviceCadenceRuntimeProved === true,
  'screen closure lost service cadence runtime proof'
);
assert(
  screenPlanClosure.assertions?.serviceDisabledSuppressionProved === true,
  'screen closure lost disabled suppression proof'
);
assert(
  screenPlanClosure.assertions?.serviceForegroundWatcherProved === true,
  'screen closure lost foreground watcher proof'
);
assert(
  screenPlanClosure.assertions?.serviceEncryptedQueueExpiryDeletionProved === true,
  'screen closure lost encrypted queue expiry deletion proof'
);
assert(
  screenPlanClosure.assertions?.deleteFailedVisibilityProved === true,
  'screen closure lost delete-failed visibility proof'
);

const desiredRows = {
  localScreenEvidenceSummaries: [
    '| Local screen evidence summaries | in progress | [screen evidence](expectations/screen-evidence.md), [roadmap V0.5.3](expectations/roadmap-v0-5-3-local-screen-evidence-analysis-queue.md) | Screen evidence settings, real Windows capture/scope/trigger proofs, local capture adapter, encrypted queue custody, deletion/retention custody, service cadence runtime, service foreground runtime, service disabled no-capture/no-AI suppression, service local-adapter/native-game analysis, service retention sweeper runtime, portal-chain, live-operator/live-external URL artifact gate, optional retention/live-view preflight, managed-browser CDP page/viewport/crop capture proof, Windows owned-process action/block dispatch proofs, service WinRT OCR redaction policy proof, Tesseract runtime extraction and fallback OCR evidence, small guided VLM provider/readiness proof, screen-plan closure audit, AI-plan closure audit, final product path artifact gate with fresh service OCR source rerun evidence, final adapter dependency audit, adapter blocker ledger proof, upstream adapter prerequisite bridge, Linux host adapter custody artifact, WSL2 Linux host adapter execution proof, Android mobile-control custody artifact, and iOS mobile-control custody artifact under `output/screen-ai-pipeline-proof/`, `output/screen-plan-proof/`, and `output/ai-plan-proof/`. The final product path artifact validates retained real-run artifacts across live/operator trigger rows, local AI rows, policy dry-runs, parent explanation snapshots, deletion rows, Windows action handoff, portal/read-model, service-backed Activity Screen read model, service cadence/foreground/disabled runtime evidence, service encrypted queue expiry deletion, delete-failed visibility, service WinRT OCR policy source rerun, retention custody, protected-surface non-claims, and screen/AI closure audits; the adapter audits prove Windows owned-process time-limit/block adapters and the reversible WSL2 Linux iptables apply/rollback proof execute today while broad/browser/network/mobile adapter classes remain blocked without product-complete claim upgrades and Android/iOS custody artifacts record non-executed apply/rollback/audit state. | Parent-facing settings and retention UI, optional raw-retention runtime enablement, live-view transport/relay/cache, platform permission proof, privacy/legal approval, current PP-OCRv5 runtime error resolution, production OCR/VLM provider quality/resource proof, live macOS/native Linux Wayland/physical Android/iOS parity, managed browser URL-trigger ownership, authenticated-account social variants, managed-browser trigger producer ownership, broad installed-app apply/rollback/audit, host network/domain apply/rollback/audit, managed active-tab exact URL apply/rollback/audit, Android device-owner/managed-profile/UsageStats/Accessibility/VPN-DNS execution after the custody artifact, iOS Family Controls/DeviceActivity/Network Extension execution after the custody artifact, native Linux desktop/Wayland/PipeWire product parity beyond the WSL2 host proof, product-complete reports, and final product-complete adapter execution remain. |',
  ].join('\n'),
  childSafetyAiDecision: [
    '| Child-safety AI decision | in progress | [AI](expectations/ai.md), [policy](expectations/policy.md) | Dry-run local AI policy evaluator, evidence context builder, AI-plan closure audit, final product path artifacts proving real captured screen evidence becomes meaningful categories and dry-run policy decisions, and policy-only enforcement consumption that uses decision refs instead of raw AI output or raw screen pixels. Controlled captured-screen analysis, live operator artifacts, service WinRT OCR captured-pixel analysis plus source service OCR proof rerun, stored-evidence input routing, deterministic/text contracts, memory/graph refs, provider runtime/scheduler, and policy-only consumption are stacked without remote/API AI, raw prompt retention, raw image retention by default, model-quality claims, or AI-owned enforcement claims. The WSL2 Linux host execution artifact proves screen-derived reversible iptables apply/rollback/audit state without upgrading native Linux desktop product readiness; Android and iOS custody artifacts record screen-derived non-executed apply/rollback/audit state for their blocked adapter classes without upgrading execution claims. | Model quality, confidence calibration, authenticated-account social/video variants, physical mobile execution, cross-platform OCR/VLM quality, and full enforcement handoff remain blocked until the broad installed-app, host network/domain, managed active-tab, Android, iOS, and native Linux desktop parity artifacts exist; the WSL2 Linux host proof does not claim native Linux Wayland/PipeWire product rollout, the Android custody artifact does not execute Device Owner/managed-profile/UsageStats/Accessibility/VPN-DNS control, and the iOS custody artifact does not execute Family Controls/DeviceActivity/Network Extension control. |',
  ].join('\n'),
};

for (const [name, row] of Object.entries(desiredRows)) {
  assert(row.includes('| in progress |'), `${name} delta must remain in progress`);
  assert(!row.includes('| done |'), `${name} delta must not mark done`);
  assert(!row.includes('product-complete adapter execution complete'), `${name} delta overclaims adapter execution`);
}

if (failures.length > 0) {
  throw new Error(
    `Screen AI product checklist delta proof failed:\n${failures.map((failure) => `- ${failure}`).join('\n')}`
  );
}

const generatedAt = new Date().toISOString();
const deltaMarkdown = [
  '# Screen AI Product Checklist Delta',
  '',
  `Generated: ${generatedAt}`,
  '',
  'Do not apply this while another lane owns `docs/product-capability-checklist.md`.',
  '',
  '## Replace Row: Local screen evidence summaries',
  '',
  '```markdown',
  desiredRows.localScreenEvidenceSummaries,
  '```',
  '',
  '## Replace Row: Child-safety AI decision',
  '',
  '```markdown',
  desiredRows.childSafetyAiDecision,
  '```',
  '',
].join('\n');

const proof = {
  status: 'doc-delta-ready-product-checklist-locked',
  proofKind: 'screen-ai-product-checklist-delta-proof',
  generatedAt,
  sourceArtifacts,
  currentRows,
  desiredRows,
  deltaMarkdown: relativePath(deltaPath),
  closure: {
    productChecklistEdited: false,
    productChecklistLockRespected: true,
    localScreenEvidenceRowDeltaReady: true,
    childSafetyAiDecisionRowDeltaReady: true,
    finalPathScreenAndAiPrerequisitesStacked: true,
    finalPathFreshServiceRerunProved: true,
    finalPipelineProductCompleteClaimed: false,
    broadBrowserNetworkMobileProductComplete: false,
    blockedAdapterRows: finalAdapterAudit.closure.blockedAdapterRows,
    custodyArtifactRows: finalAdapterAudit.closure.custodyArtifactRows,
    linuxHostExecutionRows: finalAdapterAudit.closure.linuxHostExecutionRows,
    linuxWsl2HostExecutionProved: true,
    nativeLinuxDesktopProductReady: false,
    aiPlanDefersFinalProductCompleteToPipeline: true,
    serviceCadenceRuntimeProved: true,
    serviceDisabledSuppressionProved: true,
    serviceForegroundWatcherProved: true,
    serviceEncryptedQueueExpiryDeletionProved: true,
    deleteFailedVisibilityProved: true,
  },
  nonClaims: [
    'This proof does not edit docs/product-capability-checklist.md while another lane owns that lock.',
    'This proof does not claim broad installed-app, host network/domain, managed active-tab, Android, iOS, or native Linux desktop product-complete adapter execution.',
    'This proof does not mark Local screen evidence summaries or Child-safety AI decision done.',
  ],
};

mkdirSync(outputDir, { recursive: true });
writeFileSync(deltaPath, `${deltaMarkdown}\n`);
writeFileSync(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(commandsPath, validationCommands());
console.log(`screen-ai-product-checklist-delta-proof-ok:${relativePath(proofPath)}`);

function findTableRow(text, label) {
  return text.split(/\r?\n/).find((line) => line.startsWith(`| ${label} |`));
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
    'node --check scripts/test/screen-ai-product-checklist-delta-proof.mjs',
    'node scripts/test/screen-ai-product-checklist-delta-proof.mjs',
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
