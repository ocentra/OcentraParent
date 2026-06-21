import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseBrowserUrlShape } from '../../packages/browser-domain/dist/browser-url-intelligence.js';
import { BrowserSocialRouteEvidenceSchema } from '@ocentra-parent/schema-domain/browser-social-platform-route-schemas';
import {
  buildBrowserSocialRouteEvidenceFromUrlPattern,
  matchBrowserSocialUrlPattern,
} from '@ocentra-parent/schema-domain/browser-social-url-patterns';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const proofRoot = join(repoRoot, 'output/browser-plan-proof/social-03-social-url-pattern-library');
const testResultPath = join(repoRoot, 'test-results/social-url-pattern-live-evidence-proof/proof.json');
const outputProofPath = join(proofRoot, '11-live-url-pattern-proof.json');
const observedAt = new Date().toISOString();

const sourceProofs = {
  feedRouteClassification: readJson('test-results/social-feed-route-classification-live-proof/proof.json'),
  accountCreationGate: readJson('test-results/social-account-creation-live-proof/proof.json'),
};

mkdirSync(proofRoot, { recursive: true });
mkdirSync(dirname(testResultPath), { recursive: true });

assertLiveCaptureProof(sourceProofs.feedRouteClassification, 'social-feed-route-classification-live-proof');
assertLiveCaptureProof(sourceProofs.accountCreationGate, 'social-account-creation-live-proof');

const liveUrlRows = [
  ...feedRoutePatternRows(sourceProofs.feedRouteClassification),
  ...accountRoutePatternRows(sourceProofs.accountCreationGate),
];

const parseChecks = liveUrlRows.map((row) => {
  const classification = managedClassification(row);
  const match = matchBrowserSocialUrlPattern(classification);
  const evidence = buildBrowserSocialRouteEvidenceFromUrlPattern({
    socialRouteEvidenceId: row.socialRouteEvidenceId,
    observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    classification,
  });
  return {
    targetId: row.targetId,
    requestedUrlSha256: row.requestedUrlSha256,
    finalUrlSha256: row.finalUrlSha256,
    patternId: match?.patternId ?? null,
    platform: evidence.platform,
    routeKind: evidence.routeKind,
    urlShapeTargetKind: evidence.urlShapeTargetKind,
    exactManagedBrowserRouteEvidence: evidence.exactManagedBrowserRouteEvidence,
    accountIdentityClaimed: evidence.accountIdentityClaimed,
    messageContentClaimed: evidence.messageContentClaimed,
    feedContentSemanticsClaimed: evidence.feedContentSemanticsClaimed,
    aiDecisionClaimed: evidence.aiDecisionClaimed,
    policyDecisionClaimed: evidence.policyDecisionClaimed,
    enforcementClaimed: evidence.enforcementClaimed,
    nativeAppControlClaimed: evidence.nativeAppControlClaimed,
    platformConnectorClaimed: evidence.platformConnectorClaimed,
    accepted: true,
  };
});

if (!parseChecks.every((check) => check.patternId !== null && check.accepted)) {
  throw new Error('Expected every SOCIAL-03 live URL row to match a social URL pattern');
}

const negativeChecks = [
  rejects('unmanaged-browser-url-rejected', {
    ...managedClassification(liveUrlRows[0]),
    sourceKind: 'unmanaged-browser-process',
  }),
  rejects('fake-social-domain-rejected', {
    ...managedClassification(liveUrlRows[1]),
    url: 'https://instagram.example.test/accounts/emailsignup/',
    domain: null,
  }),
  rejects('null-url-rejected', {
    ...managedClassification(liveUrlRows[2]),
    url: null,
  }),
  rejects('raw-feed-semantics-promotion-rejected', managedClassification(liveUrlRows[3]), {
    feedContentSemanticsClaimed: true,
  }),
  rejects('policy-decision-promotion-rejected', managedClassification(liveUrlRows[4]), {
    policyDecisionClaimed: true,
  }),
  rejects('enforcement-promotion-rejected', managedClassification(liveUrlRows[5]), {
    enforcementClaimed: true,
  }),
];

if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected SOCIAL-03 unmanaged, fake-domain, and overclaim checks to reject');
}

const proof = {
  schemaVersion: 1,
  proofId: 'social-url-pattern-live-evidence-proof',
  generatedAt: observedAt,
  branch: git(['branch', '--show-current']),
  commit: git(['rev-parse', 'HEAD']),
  baseCommit: git(['rev-parse', 'origin/main']),
  sourceProofs: {
    feedRouteClassification: sourceProofSummary(sourceProofs.feedRouteClassification),
    accountCreationGate: sourceProofSummary(sourceProofs.accountCreationGate),
  },
  liveEvidenceSummary: {
    realPublicSocialSurfacesUsed: true,
    generatedOrFixturePageUsed: false,
    passiveNavigationOnly: true,
    rawPageBodyPersisted: false,
    rawDomPersisted: false,
    rawTitlePersisted: false,
    rawMessageOrFeedContentPersisted: false,
    screenshotsPersistedInSourceProofs: true,
    urlPatternRows: liveUrlRows.length,
    sourceProofCaptureCount:
      sourceProofs.feedRouteClassification.captures.length + sourceProofs.accountCreationGate.captures.length,
    accountIdentityClaimed: false,
    messageContentClaimed: false,
    feedContentSemanticsClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  },
  liveUrlRows: liveUrlRows.map(redactedLiveUrlRow),
  parseChecks,
  negativeChecks,
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('social-url-pattern-live-evidence-proof-ok=true');
console.log(`proof=${relativePath(testResultPath)}`);
console.log(`outputProof=${relativePath(outputProofPath)}`);
console.log(`rows=${liveUrlRows.length} negativeChecks=${negativeChecks.length}`);

function feedRoutePatternRows(proof) {
  return proof.captures
    .filter((capture) => capture.contractClassificationCreated && capture.classificationSummary !== null)
    .map((capture) =>
      liveUrlRow(capture, {
        sourceEvidenceIds: capture.classificationSummary.sourceEvidenceIds,
        socialRouteEvidenceId: `social-pattern-route-${capture.targetId}`,
      })
    );
}

function accountRoutePatternRows(proof) {
  return proof.captures
    .filter((capture) => capture.contractPlanCreated && capture.planSummary !== null)
    .map((capture) =>
      liveUrlRow(capture, {
        sourceEvidenceIds: capture.planSummary.sourceEvidenceIds,
        socialRouteEvidenceId: `social-pattern-route-${capture.targetId}`,
      })
    );
}

function liveUrlRow(capture, { sourceEvidenceIds, socialRouteEvidenceId }) {
  const url = capture.finalUrl ?? capture.requestedUrl;
  if (typeof url !== 'string' || url.length === 0) {
    throw new Error(`Expected live capture ${capture.targetId} to include a final or requested URL`);
  }
  if (capture.rawPageBodyPersisted || capture.rawDomPersisted || capture.rawTitlePersisted) {
    throw new Error(`Expected live capture ${capture.targetId} to keep raw page/title data out of proof`);
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
    sourceEvidenceIds,
    socialRouteEvidenceId,
  };
}

function managedClassification(row) {
  return parseBrowserUrlShape({
    classificationId: `social-url-pattern-live-classification-${row.targetId}`,
    classifiedAt: observedAt,
    sourceEvidenceIds: row.sourceEvidenceIds,
    sourceKind: 'managed-browser-exact-url',
    url: row.url,
    title: null,
  });
}

function rejects(label, classification, evidencePatch = {}) {
  let rejected = false;
  let reason = null;
  try {
    const match = matchBrowserSocialUrlPattern(classification);
    if (match === null) {
      rejected = true;
      reason = 'pattern-match-rejected';
    } else {
      const evidence = buildBrowserSocialRouteEvidenceFromUrlPattern({
        socialRouteEvidenceId: `dishonest-social-url-pattern-${label}`,
        observedAt,
        sourceEvidenceIds: ['dishonest-social-url-pattern-source'],
        classification,
      });
      const mutated = BrowserSocialRouteEvidenceSchema.safeParse({
        ...evidence,
        ...evidencePatch,
      });
      rejected = !mutated.success;
      reason = mutated.success ? 'accepted' : 'route-evidence-schema-rejected';
    }
  } catch (error) {
    rejected = true;
    reason = error instanceof Error ? error.message : String(error);
  }
  return { label, rejected, reason };
}

function redactedLiveUrlRow(row) {
  return {
    targetId: row.targetId,
    requestedUrlSha256: row.requestedUrlSha256,
    finalUrlSha256: row.finalUrlSha256,
    responseStatus: row.responseStatus,
    screenshotPath: row.screenshotPath,
    screenshotSha256: row.screenshotSha256,
    screenshotBytes: row.screenshotBytes,
    sourceEvidenceIds: row.sourceEvidenceIds,
    socialRouteEvidenceId: row.socialRouteEvidenceId,
  };
}

function assertLiveCaptureProof(proof, proofId) {
  assertProofId(proof, proofId);
  if (!proof.liveCaptureSummary?.realPublicSocialSurfacesUsed) {
    throw new Error(`Expected ${proofId} to use real public social surfaces`);
  }
  if (proof.liveCaptureSummary?.generatedOrFixturePageUsed) {
    throw new Error(`Expected ${proofId} not to use generated or fixture pages`);
  }
  if (!Array.isArray(proof.captures) || proof.captures.length === 0) {
    throw new Error(`Expected ${proofId} to include capture rows`);
  }
}

function assertProofId(proof, proofId) {
  if (proof.proofId !== proofId) {
    throw new Error(`Expected proof ${proofId}, got ${proof.proofId}`);
  }
}

function sourceProofSummary(proof) {
  return {
    proofId: proof.proofId,
    generatedAt: proof.generatedAt,
    branch: proof.branch,
    commit: proof.commit,
    baseCommit: proof.baseCommit,
    captureCount: proof.captures?.length ?? 0,
  };
}

function readJson(relativePathFromRoot) {
  return JSON.parse(readFileSync(join(repoRoot, relativePathFromRoot), 'utf8'));
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function shaRef(value) {
  return typeof value === 'string' && value.length > 0 ? `source-ref:${value.length}` : null;
}
