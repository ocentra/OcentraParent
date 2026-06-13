import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { buildBrowserSocialAccountFlowEvidenceFromRoute } from '../../packages/browser-domain/dist/browser-social-account-flow-schemas.js';
import {
  BrowserSocialFormShapeEvidenceSchema,
  detectBrowserSocialFormShape,
} from '../../packages/browser-domain/dist/browser-social-form-shape-detector.js';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '../../packages/browser-domain/dist/browser-social-url-patterns.js';
import { parseBrowserUrlShape } from '../../packages/browser-domain/dist/browser-url-intelligence.js';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const proofRoot = join(repoRoot, 'output/browser-plan-proof/social-05-managed-dom-form-shape-detector');
const testResultPath = join(repoRoot, 'test-results/social-form-shape-live-evidence-proof/proof.json');
const outputProofPath = join(proofRoot, '11-live-form-shape-proof.json');
const observedAt = new Date().toISOString();

const sourceProof = readJson('test-results/social-account-creation-live-proof/proof.json');

mkdirSync(proofRoot, { recursive: true });
mkdirSync(dirname(testResultPath), { recursive: true });

assertLiveAccountCaptureProof(sourceProof);

const liveFormRows = sourceProof.captures
  .filter((capture) => capture.contractPlanCreated && capture.planSummary !== null)
  .map(liveFormRow);

const parseChecks = liveFormRows.map((row) => {
  const formShapeEvidence = detectBrowserSocialFormShape({
    formShapeEvidenceId: row.formShapeEvidenceId,
    observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    accountFlowEvidence: buildAccountFlowEvidence(row),
    controls: row.sanitizedControlKinds.map((controlKind) => ({ controlKind, valueCaptured: false })),
  });
  return {
    targetId: row.targetId,
    requestedUrlSha256: row.requestedUrlSha256,
    finalUrlSha256: row.finalUrlSha256,
    platform: formShapeEvidence.platform,
    accountFlowKind: formShapeEvidence.accountFlowKind,
    formShapeKind: formShapeEvidence.formShapeKind,
    detectionState: formShapeEvidence.detectionState,
    matchedControlKinds: formShapeEvidence.matchedControlKinds,
    missingControlKinds: formShapeEvidence.missingControlKinds,
    manualRequired: formShapeEvidence.manualRequired,
    rawDomCaptured: formShapeEvidence.rawDomCaptured,
    fieldValuesCaptured: formShapeEvidence.fieldValuesCaptured,
    credentialCaptured: formShapeEvidence.credentialCaptured,
    formSubmissionClaimed: formShapeEvidence.formSubmissionClaimed,
    accountIdentityClaimed: formShapeEvidence.accountIdentityClaimed,
    parentApprovalDecisionClaimed: formShapeEvidence.parentApprovalDecisionClaimed,
    aiDecisionClaimed: formShapeEvidence.aiDecisionClaimed,
    policyDecisionClaimed: formShapeEvidence.policyDecisionClaimed,
    enforcementClaimed: formShapeEvidence.enforcementClaimed,
    nativeAppControlClaimed: formShapeEvidence.nativeAppControlClaimed,
    platformConnectorClaimed: formShapeEvidence.platformConnectorClaimed,
    accepted: true,
  };
});

if (!parseChecks.every((check) => check.accepted)) {
  throw new Error('Expected every SOCIAL-05 live form row to parse as form-shape evidence');
}

const negativeChecks = [
  rejectsWeakControls('missing-required-controls-rejected', liveFormRows[0]),
  rejectsCapturedControlValue('captured-control-value-rejected', liveFormRows[1]),
  rejectsMutation('raw-dom-capture-rejected', liveFormRows[0], { rawDomCaptured: true }),
  rejectsMutation('field-values-rejected', liveFormRows[1], { fieldValuesCaptured: true }),
  rejectsMutation('credential-capture-rejected', liveFormRows[2], { credentialCaptured: true }),
  rejectsMutation('form-submission-rejected', liveFormRows[3], { formSubmissionClaimed: true }),
  rejectsMutation('account-identity-claim-rejected', liveFormRows[0], { accountIdentityClaimed: true }),
  rejectsMutation('parent-approval-decision-rejected', liveFormRows[1], { parentApprovalDecisionClaimed: true }),
  rejectsMutation('policy-decision-claim-rejected', liveFormRows[2], { policyDecisionClaimed: true }),
  rejectsMutation('enforcement-claim-rejected', liveFormRows[3], { enforcementClaimed: true }),
];

if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected SOCIAL-05 weak-control, captured-value, and overclaim checks to reject');
}

const proof = {
  schemaVersion: 1,
  proofId: 'social-form-shape-live-evidence-proof',
  generatedAt: observedAt,
  branch: git(['branch', '--show-current']),
  commit: git(['rev-parse', 'HEAD']),
  baseCommit: git(['rev-parse', 'origin/main']),
  sourceProof: sourceProofSummary(sourceProof),
  liveEvidenceSummary: {
    realPublicSocialSurfacesUsed: true,
    generatedOrFixturePageUsed: false,
    passiveNavigationOnly: true,
    rawPageBodyPersisted: false,
    rawDomPersisted: false,
    rawTitlePersisted: false,
    formValuesCaptured: false,
    credentialsCaptured: false,
    formsSubmitted: false,
    accountCreated: false,
    screenshotsPersistedInSourceProof: true,
    sanitizedControlHintsOnly: true,
    formShapeRows: liveFormRows.length,
    rawDomCaptured: false,
    fieldValuesCaptured: false,
    credentialCaptured: false,
    formSubmissionClaimed: false,
    accountIdentityClaimed: false,
    parentApprovalDecisionClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  },
  liveFormRows: liveFormRows.map(redactedLiveFormRow),
  parseChecks,
  negativeChecks,
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('social-form-shape-live-evidence-proof-ok=true');
console.log(`proof=${relativePath(testResultPath)}`);
console.log(`outputProof=${relativePath(outputProofPath)}`);
console.log(`rows=${liveFormRows.length} negativeChecks=${negativeChecks.length}`);

function liveFormRow(capture) {
  const url = capture.finalUrl ?? capture.requestedUrl;
  if (typeof url !== 'string' || url.length === 0) {
    throw new Error(`Expected live form capture ${capture.targetId} to include a final or requested URL`);
  }
  if (capture.rawPageBodyPersisted || capture.rawDomPersisted || capture.fieldValuesCaptured) {
    throw new Error(`Expected live form capture ${capture.targetId} to keep raw page/form data out of proof`);
  }
  if (capture.credentialsCaptured || capture.formSubmitted) {
    throw new Error(`Expected live form capture ${capture.targetId} to avoid credentials and form submission`);
  }
  return {
    targetId: capture.targetId,
    url,
    requestedUrlSha256: capture.requestedUrlSha256 ?? shaRef(capture.requestedUrl),
    finalUrlSha256: capture.finalUrlSha256,
    responseStatus: capture.responseStatus,
    screenshotPath: capture.screenshotPath,
    screenshotSha256: capture.screenshotSha256,
    screenshotBytes: capture.screenshotBytes,
    sanitizedControlKinds: capture.sanitizedControlKinds,
    sourceEvidenceIds: capture.planSummary.sourceEvidenceIds,
    accountFlowEvidenceId: capture.planSummary.accountFlowEvidenceId,
    formShapeEvidenceId: capture.planSummary.formShapeEvidenceId,
    socialRouteEvidenceId: capture.planSummary.socialRouteEvidenceId,
    expectedPlatform: capture.planSummary.platform,
    expectedAccountFlowKind: capture.planSummary.accountFlowKind,
    expectedFormShapeKind: capture.planSummary.formShapeKind,
  };
}

function buildAccountFlowEvidence(row) {
  return buildBrowserSocialAccountFlowEvidenceFromRoute({
    accountFlowEvidenceId: row.accountFlowEvidenceId,
    observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    routeEvidence: buildBrowserSocialRouteEvidenceFromUrlPattern({
      socialRouteEvidenceId: row.socialRouteEvidenceId,
      observedAt,
      sourceEvidenceIds: row.sourceEvidenceIds,
      classification: parseBrowserUrlShape({
        classificationId: `social-form-shape-live-classification-${row.targetId}`,
        classifiedAt: observedAt,
        sourceEvidenceIds: row.sourceEvidenceIds,
        sourceKind: 'managed-browser-exact-url',
        url: row.url,
        title: null,
      }),
    }),
  });
}

function rejectsWeakControls(label, row) {
  return rejectsDetectorInput(label, row, [{ controlKind: 'email-input', valueCaptured: false }]);
}

function rejectsCapturedControlValue(label, row) {
  return rejectsDetectorInput(
    label,
    row,
    row.sanitizedControlKinds.map((controlKind, index) => ({
      controlKind,
      valueCaptured: index === 0,
    }))
  );
}

function rejectsDetectorInput(label, row, controls) {
  let rejected = false;
  let reason = null;
  try {
    detectBrowserSocialFormShape({
      formShapeEvidenceId: `dishonest-social-form-shape-${label}`,
      observedAt,
      sourceEvidenceIds: row.sourceEvidenceIds,
      accountFlowEvidence: buildAccountFlowEvidence(row),
      controls,
    });
  } catch (error) {
    rejected = true;
    reason = error instanceof Error ? error.message : String(error);
  }
  return { label, rejected, reason: reason ?? 'accepted' };
}

function rejectsMutation(label, row, evidencePatch) {
  const evidence = detectBrowserSocialFormShape({
    formShapeEvidenceId: `dishonest-social-form-shape-${label}`,
    observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    accountFlowEvidence: buildAccountFlowEvidence(row),
    controls: row.sanitizedControlKinds.map((controlKind) => ({ controlKind, valueCaptured: false })),
  });
  const mutated = BrowserSocialFormShapeEvidenceSchema.safeParse({
    ...evidence,
    ...evidencePatch,
  });
  return {
    label,
    rejected: !mutated.success,
    reason: mutated.success ? 'accepted' : 'form-shape-schema-rejected',
  };
}

function redactedLiveFormRow(row) {
  return {
    targetId: row.targetId,
    requestedUrlSha256: row.requestedUrlSha256,
    finalUrlSha256: row.finalUrlSha256,
    responseStatus: row.responseStatus,
    screenshotPath: row.screenshotPath,
    screenshotSha256: row.screenshotSha256,
    screenshotBytes: row.screenshotBytes,
    sanitizedControlKinds: row.sanitizedControlKinds,
    sourceEvidenceIds: row.sourceEvidenceIds,
    accountFlowEvidenceId: row.accountFlowEvidenceId,
    formShapeEvidenceId: row.formShapeEvidenceId,
    socialRouteEvidenceId: row.socialRouteEvidenceId,
    expectedPlatform: row.expectedPlatform,
    expectedAccountFlowKind: row.expectedAccountFlowKind,
    expectedFormShapeKind: row.expectedFormShapeKind,
  };
}

function assertLiveAccountCaptureProof(proof) {
  if (proof?.proofId !== 'social-account-creation-live-proof') {
    throw new Error('Expected proofId social-account-creation-live-proof');
  }
  const summary = proof.liveCaptureSummary;
  if (!summary?.realPublicSocialSurfacesUsed || summary.generatedOrFixturePageUsed || !summary.passiveNavigationOnly) {
    throw new Error('Expected SOCIAL-13 source proof to be passive real-public-social capture');
  }
  if (summary.formValuesCaptured || summary.credentialsCaptured || summary.formsSubmitted || summary.accountCreated) {
    throw new Error('SOCIAL-05 cannot use source proof with form values, credentials, submission, or creation');
  }
  if (summary.rawPageBodyPersisted || summary.rawDomPersisted) {
    throw new Error('Expected SOCIAL-13 source proof to avoid raw page and DOM persistence');
  }
}

function sourceProofSummary(proof) {
  return {
    proofId: proof.proofId,
    generatedAt: proof.generatedAt,
    branch: proof.branch,
    commit: proof.commit,
    captureCount: proof.captures.length,
    realPublicSocialSurfacesUsed: proof.liveCaptureSummary.realPublicSocialSurfacesUsed,
    generatedOrFixturePageUsed: proof.liveCaptureSummary.generatedOrFixturePageUsed,
    passiveNavigationOnly: proof.liveCaptureSummary.passiveNavigationOnly,
    formValuesCaptured: proof.liveCaptureSummary.formValuesCaptured,
    credentialsCaptured: proof.liveCaptureSummary.credentialsCaptured,
    formsSubmitted: proof.liveCaptureSummary.formsSubmitted,
    accountCreated: proof.liveCaptureSummary.accountCreated,
  };
}

function readJson(path) {
  return JSON.parse(readFileSync(join(repoRoot, path), 'utf8'));
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

function shaRef(value) {
  if (typeof value !== 'string') {
    return null;
  }
  return execFileSync('git', ['hash-object', '--stdin'], {
    cwd: repoRoot,
    encoding: 'utf8',
    input: value,
  }).trim();
}
