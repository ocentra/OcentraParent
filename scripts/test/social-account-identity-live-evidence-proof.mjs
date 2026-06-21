import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  BrowserSocialAccountIdentityRegistryEntrySchema,
  buildUnverifiedSocialAccountIdentityContextFromFlow,
} from '../../packages/schema-domain/dist/browser-social-account-identity-registry.js';
import { buildBrowserSocialAccountFlowEvidenceFromRoute } from '../../packages/schema-domain/dist/browser-social-account-flow-schemas.js';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '@ocentra-parent/schema-domain/browser-social-url-patterns';
import { parseBrowserUrlShape } from '../../packages/browser-domain/dist/browser-url-intelligence.js';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const proofRoot = join(repoRoot, 'output/browser-plan-proof/social-06-social-account-identity-registry');
const testResultPath = join(repoRoot, 'test-results/social-account-identity-live-evidence-proof/proof.json');
const outputProofPath = join(proofRoot, '11-live-identity-proof.json');
const observedAt = new Date().toISOString();

const sourceProof = readJson('test-results/social-account-creation-live-proof/proof.json');

mkdirSync(proofRoot, { recursive: true });
mkdirSync(dirname(testResultPath), { recursive: true });

assertLiveAccountCaptureProof(sourceProof);

const liveIdentityRows = sourceProof.captures
  .filter((capture) => capture.contractPlanCreated && capture.planSummary !== null)
  .map(liveIdentityRow);

const parseChecks = liveIdentityRows.map((row) => {
  const identityEntry = buildUnverifiedSocialAccountIdentityContextFromFlow({
    registryEntryId: row.registryEntryId,
    accountIdentityRef: row.accountIdentityRef,
    observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    accountFlowEvidence: buildAccountFlowEvidence(row),
  });
  return {
    targetId: row.targetId,
    requestedUrlSha256: row.requestedUrlSha256,
    finalUrlSha256: row.finalUrlSha256,
    platform: identityEntry.platform,
    sourceKind: identityEntry.sourceKind,
    identityState: identityEntry.identityState,
    socialRouteEvidenceId: identityEntry.socialRouteEvidenceId,
    accountFlowEvidenceId: identityEntry.accountFlowEvidenceId,
    parentAssertionRef: identityEntry.parentAssertionRef,
    handleHashRef: identityEntry.handleHashRef,
    displayNameHashRef: identityEntry.displayNameHashRef,
    platformAccountIdHashRef: identityEntry.platformAccountIdHashRef,
    rawHandleCaptured: identityEntry.rawHandleCaptured,
    rawDisplayNameCaptured: identityEntry.rawDisplayNameCaptured,
    rawPlatformAccountIdCaptured: identityEntry.rawPlatformAccountIdCaptured,
    credentialCaptured: identityEntry.credentialCaptured,
    identityVerifiedByPlatform: identityEntry.identityVerifiedByPlatform,
    parentDeclaredIdentity: identityEntry.parentDeclaredIdentity,
    childDeclaredIdentity: identityEntry.childDeclaredIdentity,
    accountCreationClaimed: identityEntry.accountCreationClaimed,
    loginSuccessClaimed: identityEntry.loginSuccessClaimed,
    connectorAuthorizationClaimed: identityEntry.connectorAuthorizationClaimed,
    aiDecisionClaimed: identityEntry.aiDecisionClaimed,
    policyDecisionClaimed: identityEntry.policyDecisionClaimed,
    enforcementClaimed: identityEntry.enforcementClaimed,
    nativeAppControlClaimed: identityEntry.nativeAppControlClaimed,
    accepted: true,
  };
});

if (!parseChecks.every((check) => check.accepted)) {
  throw new Error('Expected every SOCIAL-06 live identity row to parse as unverified route-context identity');
}

const negativeChecks = [
  rejectsMutation('raw-handle-capture-rejected', liveIdentityRows[0], { rawHandleCaptured: true }),
  rejectsMutation('raw-display-name-capture-rejected', liveIdentityRows[1], { rawDisplayNameCaptured: true }),
  rejectsMutation('raw-platform-account-id-capture-rejected', liveIdentityRows[2], {
    rawPlatformAccountIdCaptured: true,
  }),
  rejectsMutation('credential-capture-rejected', liveIdentityRows[3], { credentialCaptured: true }),
  rejectsMutation('platform-verification-rejected', liveIdentityRows[0], { identityVerifiedByPlatform: true }),
  rejectsMutation('parent-declared-without-parent-hash-rejected', liveIdentityRows[1], {
    parentDeclaredIdentity: true,
  }),
  rejectsMutation('child-declared-identity-rejected', liveIdentityRows[2], { childDeclaredIdentity: true }),
  rejectsMutation('account-creation-claim-rejected', liveIdentityRows[3], { accountCreationClaimed: true }),
  rejectsMutation('login-success-claim-rejected', liveIdentityRows[0], { loginSuccessClaimed: true }),
  rejectsMutation('connector-authorization-claim-rejected', liveIdentityRows[1], {
    connectorAuthorizationClaimed: true,
  }),
  rejectsMutation('policy-decision-claim-rejected', liveIdentityRows[2], { policyDecisionClaimed: true }),
  rejectsMutation('native-app-control-claim-rejected', liveIdentityRows[3], { nativeAppControlClaimed: true }),
  rejectsMutation('enforcement-claim-rejected', liveIdentityRows[0], { enforcementClaimed: true }),
  rejectsMutation('missing-account-flow-ref-rejected', liveIdentityRows[1], { accountFlowEvidenceId: null }),
];

if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected SOCIAL-06 raw identity, authority, and inconsistent-state checks to reject');
}

const proof = {
  schemaVersion: 1,
  proofId: 'social-account-identity-live-evidence-proof',
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
    routeContextIdentityOnly: true,
    identityRows: liveIdentityRows.length,
    rawHandleCaptured: false,
    rawDisplayNameCaptured: false,
    rawPlatformAccountIdCaptured: false,
    credentialCaptured: false,
    identityVerifiedByPlatform: false,
    parentDeclaredIdentity: false,
    childDeclaredIdentity: false,
    accountCreationClaimed: false,
    loginSuccessClaimed: false,
    connectorAuthorizationClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
  },
  liveIdentityRows: liveIdentityRows.map(redactedLiveIdentityRow),
  parseChecks,
  negativeChecks,
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('social-account-identity-live-evidence-proof-ok=true');
console.log(`proof=${relativePath(testResultPath)}`);
console.log(`outputProof=${relativePath(outputProofPath)}`);
console.log(`rows=${liveIdentityRows.length} negativeChecks=${negativeChecks.length}`);

function liveIdentityRow(capture) {
  const url = capture.finalUrl ?? capture.requestedUrl;
  if (typeof url !== 'string' || url.length === 0) {
    throw new Error(`Expected live identity capture ${capture.targetId} to include a final or requested URL`);
  }
  if (capture.rawPageBodyPersisted || capture.rawDomPersisted || capture.fieldValuesCaptured) {
    throw new Error(`Expected live identity capture ${capture.targetId} to keep raw page/form data out of proof`);
  }
  if (capture.credentialsCaptured || capture.formSubmitted) {
    throw new Error(`Expected live identity capture ${capture.targetId} to avoid credentials and form submission`);
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
    sourceEvidenceIds: capture.planSummary.sourceEvidenceIds,
    registryEntryId: `social-identity-live-entry-${capture.targetId}`,
    accountIdentityRef: `social-identity-live-ref-${capture.targetId}`,
    accountFlowEvidenceId: capture.planSummary.accountFlowEvidenceId,
    socialRouteEvidenceId: capture.planSummary.socialRouteEvidenceId,
    expectedPlatform: capture.planSummary.platform,
    expectedAccountFlowKind: capture.planSummary.accountFlowKind,
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
        classificationId: `social-identity-live-classification-${row.targetId}`,
        classifiedAt: observedAt,
        sourceEvidenceIds: row.sourceEvidenceIds,
        sourceKind: 'managed-browser-exact-url',
        url: row.url,
        title: null,
      }),
    }),
  });
}

function rejectsMutation(label, row, evidencePatch) {
  const evidence = buildUnverifiedSocialAccountIdentityContextFromFlow({
    registryEntryId: `dishonest-social-identity-entry-${label}`,
    accountIdentityRef: `dishonest-social-identity-ref-${label}`,
    observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    accountFlowEvidence: buildAccountFlowEvidence(row),
  });
  const mutated = BrowserSocialAccountIdentityRegistryEntrySchema.safeParse({
    ...evidence,
    ...evidencePatch,
  });
  return {
    label,
    rejected: !mutated.success,
    reason: mutated.success ? 'accepted' : 'identity-registry-schema-rejected',
  };
}

function redactedLiveIdentityRow(row) {
  return {
    targetId: row.targetId,
    requestedUrlSha256: row.requestedUrlSha256,
    finalUrlSha256: row.finalUrlSha256,
    responseStatus: row.responseStatus,
    screenshotPath: row.screenshotPath,
    screenshotSha256: row.screenshotSha256,
    screenshotBytes: row.screenshotBytes,
    sourceEvidenceIds: row.sourceEvidenceIds,
    registryEntryId: row.registryEntryId,
    accountIdentityRef: row.accountIdentityRef,
    accountFlowEvidenceId: row.accountFlowEvidenceId,
    socialRouteEvidenceId: row.socialRouteEvidenceId,
    expectedPlatform: row.expectedPlatform,
    expectedAccountFlowKind: row.expectedAccountFlowKind,
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
    throw new Error('SOCIAL-06 cannot use source proof with form values, credentials, submission, or account creation');
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
