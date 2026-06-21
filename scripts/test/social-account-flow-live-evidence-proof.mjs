import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  buildBrowserSocialAccountFlowEvidenceFromRoute,
  BrowserSocialAccountFlowEvidenceSchema,
} from '../../packages/schema-domain/dist/browser-social-account-flow-schemas.js';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '@ocentra-parent/schema-domain/browser-social-url-patterns';
import { parseBrowserUrlShape } from '../../packages/browser-domain/dist/browser-url-intelligence.js';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const proofRoot = join(repoRoot, 'output/browser-plan-proof/social-04-account-flow-evidence-contracts');
const testResultPath = join(repoRoot, 'test-results/social-account-flow-live-evidence-proof/proof.json');
const outputProofPath = join(proofRoot, '11-live-account-flow-proof.json');
const observedAt = new Date().toISOString();

const sourceProof = readJson('test-results/social-account-creation-live-proof/proof.json');

mkdirSync(proofRoot, { recursive: true });
mkdirSync(dirname(testResultPath), { recursive: true });

assertLiveAccountCaptureProof(sourceProof);

const liveAccountRows = sourceProof.captures
  .filter((capture) => capture.contractPlanCreated && capture.planSummary !== null)
  .map(liveAccountRow);

const parseChecks = liveAccountRows.map((row) => {
  const routeEvidence = buildRouteEvidence(row);
  const accountFlowEvidence = buildBrowserSocialAccountFlowEvidenceFromRoute({
    accountFlowEvidenceId: row.accountFlowEvidenceId,
    observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    routeEvidence,
  });
  return {
    targetId: row.targetId,
    requestedUrlSha256: row.requestedUrlSha256,
    finalUrlSha256: row.finalUrlSha256,
    platform: accountFlowEvidence.platform,
    routeKind: accountFlowEvidence.routeKind,
    accountFlowKind: accountFlowEvidence.accountFlowKind,
    evidenceState: accountFlowEvidence.evidenceState,
    exactManagedBrowserRouteEvidence: accountFlowEvidence.exactManagedBrowserRouteEvidence,
    manualRequired: accountFlowEvidence.manualRequired,
    accountIdentityRef: accountFlowEvidence.accountIdentityRef,
    parentApprovalRequestRef: accountFlowEvidence.parentApprovalRequestRef,
    accountIdentityClaimed: accountFlowEvidence.accountIdentityClaimed,
    credentialCaptured: accountFlowEvidence.credentialCaptured,
    formFieldValuesCaptured: accountFlowEvidence.formFieldValuesCaptured,
    formSubmissionClaimed: accountFlowEvidence.formSubmissionClaimed,
    accountCreationCompletedClaimed: accountFlowEvidence.accountCreationCompletedClaimed,
    loginSuccessClaimed: accountFlowEvidence.loginSuccessClaimed,
    accountSwitchCompletedClaimed: accountFlowEvidence.accountSwitchCompletedClaimed,
    parentApprovalDecisionClaimed: accountFlowEvidence.parentApprovalDecisionClaimed,
    aiDecisionClaimed: accountFlowEvidence.aiDecisionClaimed,
    policyDecisionClaimed: accountFlowEvidence.policyDecisionClaimed,
    enforcementClaimed: accountFlowEvidence.enforcementClaimed,
    nativeAppControlClaimed: accountFlowEvidence.nativeAppControlClaimed,
    platformConnectorClaimed: accountFlowEvidence.platformConnectorClaimed,
    accepted: true,
  };
});

if (!parseChecks.every((check) => check.accepted)) {
  throw new Error('Expected every SOCIAL-04 live account row to parse as account-flow evidence');
}

const negativeChecks = [
  rejectsBuildFromNonAccountRoute('feed-route-rejected'),
  rejectsMutation('account-identity-ref-rejected', liveAccountRows[0], { accountIdentityRef: 'raw-account-ref' }),
  rejectsMutation('parent-approval-ref-rejected', liveAccountRows[1], {
    parentApprovalRequestRef: 'approval-request-not-created-by-this-contract',
  }),
  rejectsMutation('credential-capture-rejected', liveAccountRows[2], { credentialCaptured: true }),
  rejectsMutation('form-field-values-rejected', liveAccountRows[3], { formFieldValuesCaptured: true }),
  rejectsMutation('form-submission-rejected', liveAccountRows[0], { formSubmissionClaimed: true }),
  rejectsMutation('account-created-claim-rejected', liveAccountRows[1], { accountCreationCompletedClaimed: true }),
  rejectsMutation('login-success-claim-rejected', liveAccountRows[2], { loginSuccessClaimed: true }),
  rejectsMutation('policy-decision-claim-rejected', liveAccountRows[3], { policyDecisionClaimed: true }),
  rejectsMutation('enforcement-claim-rejected', liveAccountRows[0], { enforcementClaimed: true }),
];

if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected SOCIAL-04 non-account and overclaim checks to reject');
}

const proof = {
  schemaVersion: 1,
  proofId: 'social-account-flow-live-evidence-proof',
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
    accountFlowRows: liveAccountRows.length,
    accountIdentityClaimed: false,
    parentApprovalDecisionClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  },
  liveAccountRows: liveAccountRows.map(redactedLiveAccountRow),
  parseChecks,
  negativeChecks,
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('social-account-flow-live-evidence-proof-ok=true');
console.log(`proof=${relativePath(testResultPath)}`);
console.log(`outputProof=${relativePath(outputProofPath)}`);
console.log(`rows=${liveAccountRows.length} negativeChecks=${negativeChecks.length}`);

function liveAccountRow(capture) {
  const url = capture.finalUrl ?? capture.requestedUrl;
  if (typeof url !== 'string' || url.length === 0) {
    throw new Error(`Expected live account capture ${capture.targetId} to include a final or requested URL`);
  }
  if (capture.rawPageBodyPersisted || capture.rawDomPersisted || capture.fieldValuesCaptured) {
    throw new Error(`Expected live account capture ${capture.targetId} to keep raw page/form data out of proof`);
  }
  if (capture.credentialsCaptured || capture.formSubmitted) {
    throw new Error(`Expected live account capture ${capture.targetId} to avoid credentials and form submission`);
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
    socialRouteEvidenceId: capture.planSummary.socialRouteEvidenceId,
    expectedPlatform: capture.planSummary.platform,
    expectedAccountFlowKind: capture.planSummary.accountFlowKind,
  };
}

function buildRouteEvidence(row) {
  return buildBrowserSocialRouteEvidenceFromUrlPattern({
    socialRouteEvidenceId: row.socialRouteEvidenceId,
    observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    classification: parseManagedUrl(row),
  });
}

function parseManagedUrl(row) {
  return parseBrowserUrlShape({
    classificationId: `social-account-flow-live-classification-${row.targetId}`,
    classifiedAt: observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    sourceKind: 'managed-browser-exact-url',
    url: row.url,
    title: null,
  });
}

function rejectsBuildFromNonAccountRoute(label) {
  let rejected = false;
  let reason = null;
  try {
    buildBrowserSocialAccountFlowEvidenceFromRoute({
      accountFlowEvidenceId: `dishonest-social-account-flow-${label}`,
      observedAt,
      sourceEvidenceIds: ['dishonest-social-account-flow-source'],
      routeEvidence: buildBrowserSocialRouteEvidenceFromUrlPattern({
        socialRouteEvidenceId: `dishonest-social-route-${label}`,
        observedAt,
        sourceEvidenceIds: ['dishonest-social-route-source'],
        classification: parseBrowserUrlShape({
          classificationId: `dishonest-social-url-shape-${label}`,
          classifiedAt: observedAt,
          sourceEvidenceIds: ['dishonest-social-url-shape-source'],
          sourceKind: 'managed-browser-exact-url',
          url: 'https://x.com/home',
          title: null,
        }),
      }),
    });
  } catch (error) {
    rejected = true;
    reason = error instanceof Error ? error.message : String(error);
  }
  return { label, rejected, reason: reason ?? 'accepted' };
}

function rejectsMutation(label, row, evidencePatch) {
  const evidence = buildBrowserSocialAccountFlowEvidenceFromRoute({
    accountFlowEvidenceId: `dishonest-social-account-flow-${label}`,
    observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    routeEvidence: buildRouteEvidence(row),
  });
  const mutated = BrowserSocialAccountFlowEvidenceSchema.safeParse({
    ...evidence,
    ...evidencePatch,
  });
  return {
    label,
    rejected: !mutated.success,
    reason: mutated.success ? 'accepted' : 'account-flow-schema-rejected',
  };
}

function redactedLiveAccountRow(row) {
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
    socialRouteEvidenceId: row.socialRouteEvidenceId,
    expectedPlatform: row.expectedPlatform,
    expectedAccountFlowKind: row.expectedAccountFlowKind,
  };
}

function assertLiveAccountCaptureProof(proof) {
  assertProofId(proof, 'social-account-creation-live-proof');
  if (!proof.liveCaptureSummary?.realPublicSocialSurfacesUsed) {
    throw new Error('Expected SOCIAL-13 source proof to use real public social surfaces');
  }
  if (proof.liveCaptureSummary?.generatedOrFixturePageUsed) {
    throw new Error('SOCIAL-04 live proof cannot use generated or fixture pages as source evidence');
  }
  if (!proof.liveCaptureSummary?.passiveNavigationOnly) {
    throw new Error('Expected SOCIAL-13 source proof to be passive navigation only');
  }
  if (
    proof.liveCaptureSummary?.formValuesCaptured ||
    proof.liveCaptureSummary?.credentialsCaptured ||
    proof.liveCaptureSummary?.formsSubmitted ||
    proof.liveCaptureSummary?.accountCreated
  ) {
    throw new Error('SOCIAL-04 cannot use account source proof with form values, credentials, submission, or creation');
  }
  if (proof.liveCaptureSummary?.rawPageBodyPersisted || proof.liveCaptureSummary?.rawDomPersisted) {
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

function assertProofId(proof, expectedProofId) {
  if (proof?.proofId !== expectedProofId) {
    throw new Error(`Expected proofId ${expectedProofId}`);
  }
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
