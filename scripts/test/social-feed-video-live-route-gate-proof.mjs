import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright';
import { parseBrowserUrlShape } from '../../packages/browser-domain/dist/browser-url-intelligence.js';
import { classifyBrowserSocialFeedRoute } from '../../packages/schema-domain/dist/browser-social-feed-route-classification.js';
import {
  BrowserSocialFeedVideoRouteGatePlanSchema,
  planBrowserSocialFeedVideoRouteGate,
} from '@ocentra-parent/schema-domain/browser-social-feed-video-route-gate';
import { extractBrowserSocialVideoMetadata } from '../../packages/schema-domain/dist/browser-social-video-metadata.js';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '@ocentra-parent/schema-domain/browser-social-url-patterns';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const proofRoot = join(repoRoot, 'output/browser-plan-proof/social-14-managed-browser-feed-short-video-gate');
const screenshotRoot = join(proofRoot, '06-live-screenshots');
const testResultPath = join(repoRoot, 'test-results/social-feed-video-live-route-gate-proof/proof.json');
const outputProofPath = join(proofRoot, '11-live-route-gate-proof.json');
const observedAt = new Date().toISOString();

const sourceFiles = [
  'packages/browser-domain/src/browser-url-intelligence.ts',
  'packages/schema-domain/src/browser-social-url-patterns.ts',
  'packages/schema-domain/src/browser-social-feed-route-classification.ts',
  'packages/schema-domain/src/browser-social-video-metadata.ts',
  'packages/schema-domain/src/browser-social-feed-video-route-gate-values.ts',
  'packages/schema-domain/src/browser-social-feed-video-route-gate-guards.ts',
  'packages/schema-domain/src/browser-social-feed-video-route-gate.ts',
];

const builtFiles = [
  'packages/browser-domain/dist/browser-url-intelligence.js',
  'packages/schema-domain/dist/browser-social-url-patterns.js',
  'packages/schema-domain/dist/browser-social-feed-route-classification.js',
  'packages/schema-domain/dist/browser-social-video-metadata.js',
  'packages/schema-domain/dist/browser-social-feed-video-route-gate-values.js',
  'packages/schema-domain/dist/browser-social-feed-video-route-gate-guards.js',
  'packages/schema-domain/dist/browser-social-feed-video-route-gate.js',
];

const liveTargets = [
  {
    id: 'reddit-popular-feed',
    url: 'https://www.reddit.com/r/popular/',
    expectedPlatform: 'reddit',
    expectedRouteKind: 'feed',
    surfaceHint: 'home-feed',
    routeGateAction: 'warn-route-candidate',
    parentApprovalRequired: false,
    policyDecisionCandidateRef: 'social-policy-candidate-reddit-popular-warn',
    parentApprovalRequestRef: null,
    timeLimitCandidateRef: null,
    reasons: ['dynamic-feed-route', 'parent-policy-match'],
  },
  {
    id: 'twitch-directory-feed',
    url: 'https://www.twitch.tv/directory',
    expectedPlatform: 'twitch',
    expectedRouteKind: 'feed',
    surfaceHint: 'home-feed',
    routeGateAction: 'limit-route-candidate',
    parentApprovalRequired: false,
    policyDecisionCandidateRef: 'social-policy-candidate-twitch-directory-limit',
    parentApprovalRequestRef: null,
    timeLimitCandidateRef: 'social-time-limit-candidate-twitch-directory',
    reasons: ['dynamic-feed-route', 'schedule-limit-candidate', 'parent-policy-match'],
  },
  {
    id: 'tiktok-explore-feed',
    url: 'https://www.tiktok.com/explore',
    expectedPlatform: 'tiktok',
    expectedRouteKind: 'feed',
    surfaceHint: 'explore-feed',
    routeGateAction: 'parent-review-candidate',
    parentApprovalRequired: true,
    policyDecisionCandidateRef: null,
    parentApprovalRequestRef: 'social-parent-review-candidate-tiktok-explore',
    timeLimitCandidateRef: null,
    reasons: ['dynamic-feed-route', 'parent-review-required'],
  },
  {
    id: 'instagram-reels-feed',
    url: 'https://www.instagram.com/reels/',
    expectedPlatform: 'instagram',
    expectedRouteKind: 'feed',
    surfaceHint: 'reels-feed',
    routeGateAction: 'block-route-candidate',
    parentApprovalRequired: false,
    policyDecisionCandidateRef: 'social-policy-candidate-instagram-reels-block',
    parentApprovalRequestRef: null,
    timeLimitCandidateRef: null,
    reasons: ['short-video-route', 'policy-block-candidate', 'parent-policy-match'],
  },
  {
    id: 'youtube-shorts-video',
    url: 'https://www.youtube.com/shorts/jNQXAC9IVRw',
    expectedPlatform: 'youtube-shorts',
    expectedRouteKind: 'video',
    surfaceHint: 'single-short-video',
    routeGateAction: 'parent-review-candidate',
    parentApprovalRequired: true,
    policyDecisionCandidateRef: null,
    parentApprovalRequestRef: 'social-parent-review-candidate-youtube-shorts',
    timeLimitCandidateRef: null,
    reasons: ['single-video-route', 'parent-review-required'],
  },
  {
    id: 'vimeo-public-video',
    url: 'https://vimeo.com/76979871',
    expectedPlatform: 'vimeo',
    expectedRouteKind: 'video',
    surfaceHint: null,
    routeGateAction: 'parent-review-candidate',
    parentApprovalRequired: true,
    policyDecisionCandidateRef: null,
    parentApprovalRequestRef: 'social-parent-review-candidate-vimeo-video',
    timeLimitCandidateRef: null,
    reasons: ['single-video-route', 'parent-review-required'],
  },
];

assertBuiltContractsAreFresh();
mkdirSync(screenshotRoot, { recursive: true });

const browser = await chromium.launch({ headless: true });
const captures = [];
try {
  for (const target of liveTargets) {
    captures.push(await captureLiveTarget(browser, target));
  }
} finally {
  await browser.close();
}

const plannedCaptures = captures.filter((capture) => capture.contractPlanCreated);
assertPlannedCaptureCoverage(plannedCaptures);

const planParseChecks = plannedCaptures.map((capture) => ({
  targetId: capture.targetId,
  accepted: BrowserSocialFeedVideoRouteGatePlanSchema.safeParse(capture.planSummary).success,
}));
if (!planParseChecks.every((check) => check.accepted)) {
  throw new Error('Expected all SOCIAL-14 live route-gate plans to parse through the contract schema');
}

const negativeChecks = buildNegativeChecks(plannedCaptures.map((capture) => capture.planSummary));
if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected all SOCIAL-14 negative checks to reject dishonest runtime/action claims');
}

const proof = {
  schemaVersion: 1,
  proofId: 'social-feed-video-live-route-gate-proof',
  generatedAt: observedAt,
  branch: git(['branch', '--show-current']),
  commit: git(['rev-parse', 'HEAD']),
  baseCommit: git(['rev-parse', 'origin/main']),
  liveCaptureSummary: {
    realPublicSocialVideoSurfacesUsed: true,
    generatedOrFixturePageUsed: false,
    passiveNavigationOnly: true,
    formsSubmitted: false,
    credentialsCaptured: false,
    rawPageBodyPersisted: false,
    rawDomPersisted: false,
    rawTitlePersisted: false,
    screenshotsPersisted: true,
    screenshotCount: captures.filter((capture) => capture.screenshotSha256 !== null).length,
    plannedCaptureCount: plannedCaptures.length,
    requestedTargetCount: liveTargets.length,
    requiredCoverage: {
      dynamicFeedRoute: true,
      shortVideoSurfaceRoute: true,
      socialVideoRoute: true,
      parentReviewCandidate: true,
      timeLimitCandidate: true,
      blockCandidate: true,
    },
  },
  captures,
  planParseChecks,
  plannedGateSummaries: plannedCaptures.map((capture) => capture.planSummary),
  negativeChecks,
  noClaimChecks: {
    browserNavigationBlockedClaimed: false,
    browserRedirectClaimed: false,
    cssDomHiddenClaimed: false,
    tabClosedClaimed: false,
    timeLimitAppliedClaimed: false,
    childUiRenderedClaimed: false,
    parentUiNotifiedClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    feedContentCaptured: false,
    videoContentCaptured: false,
    recommendationModelClaimed: false,
  },
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('social-feed-video-live-route-gate-proof-ok=true');
console.log(`proof=${testResultPath}`);
console.log(`outputProof=${outputProofPath}`);
console.log(`plannedCaptureCount=${plannedCaptures.length}`);
console.log(`plannedPlatforms=${plannedCaptures.map((capture) => capture.planSummary.platform).join(',')}`);

async function captureLiveTarget(browserInstance, target) {
  const page = await browserInstance.newPage({
    viewport: { width: 1280, height: 900 },
    userAgent:
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125 Safari/537.36',
  });

  let responseStatus = null;
  let responseUrl = null;
  let navigationError = null;
  try {
    const response = await page.goto(target.url, { waitUntil: 'domcontentloaded', timeout: 30_000 });
    responseStatus = response?.status() ?? null;
    responseUrl = response?.url() ?? null;
    await page.waitForLoadState('networkidle', { timeout: 8_000 }).catch(() => undefined);
  } catch (error) {
    navigationError = error instanceof Error ? error.message : String(error);
  }

  const finalUrl = page.url();
  const proofFinalUrl = sanitizeUrlForProof(finalUrl);
  const proofRequestedUrl = sanitizeUrlForProof(target.url);
  const title = await page.title().catch(() => '');
  const screenshotPath = join(screenshotRoot, `${target.id}.png`);
  await page.screenshot({ path: screenshotPath, fullPage: false }).catch(() => undefined);
  await page.close();

  const base = {
    targetId: target.id,
    requestedUrl: proofRequestedUrl,
    finalUrl: proofFinalUrl,
    finalUrlSha256: sha256(proofFinalUrl),
    finalUrlQueryOrHashRemoved: finalUrl !== proofFinalUrl,
    responseStatus,
    responseUrl: responseUrl === null ? null : sanitizeUrlForProof(responseUrl),
    navigationError,
    titleLength: title.length,
    titleSha256: sha256(title),
    screenshotPath: relative(repoRoot, screenshotPath).replaceAll('\\', '/'),
    screenshotSha256: existsSync(screenshotPath) ? sha256File(screenshotPath) : null,
    screenshotBytes: existsSync(screenshotPath) ? statSync(screenshotPath).size : 0,
    rawPageBodyPersisted: false,
    rawDomPersisted: false,
    rawTitlePersisted: false,
    formSubmitted: false,
    credentialsCaptured: false,
  };

  try {
    const planSummary = buildGatePlanFromLiveCapture(target, {
      contractUrl: proofFinalUrl,
      title,
      screenshotSha256: base.screenshotSha256,
    });
    return {
      ...base,
      contractPlanCreated: true,
      planSummary,
    };
  } catch (error) {
    return {
      ...base,
      contractPlanCreated: false,
      contractError: error instanceof Error ? error.message : String(error),
      planSummary: null,
    };
  }
}

function buildGatePlanFromLiveCapture(target, capture) {
  const routeEvidence = socialRoute(target, capture.contractUrl);
  assertRouteTargetMatches(target, routeEvidence);

  const feedRouteClassification =
    target.surfaceHint === null
      ? null
      : classifyBrowserSocialFeedRoute({
          feedRouteClassificationId: `${target.id}-feed-route-classification`,
          observedAt,
          sourceEvidenceIds: [`${target.id}-live-feed-route-evidence`],
          routeEvidence,
          surfaceHint: target.surfaceHint,
        });

  const videoMetadataEvidence =
    routeEvidence.routeKind === 'video' || target.surfaceHint === null
      ? extractBrowserSocialVideoMetadata({
          metadataEvidenceId: `${target.id}-video-metadata-evidence`,
          collectedAt: observedAt,
          sourceEvidenceIds: [`${target.id}-live-video-metadata-evidence`],
          routeEvidence,
          sourceKind: 'platform-page-metadata',
          titleRef: hashRef('title', capture.title),
          descriptionRef: null,
          authorHashRef: null,
          thumbnailHashRef: capture.screenshotSha256 === null ? null : `screenshot-sha256-${capture.screenshotSha256}`,
          durationSeconds: null,
          publishedAt: null,
          categoryRef: null,
          restrictionSignalRef: null,
        })
      : null;

  return planBrowserSocialFeedVideoRouteGate({
    gatePlanId: `${target.id}-route-gate-plan`,
    plannedAt: observedAt,
    sourceEvidenceIds: [`${target.id}-live-route-gate-evidence`],
    feedRouteClassification,
    videoMetadataEvidence,
    policyDecisionCandidateRef: target.policyDecisionCandidateRef,
    parentApprovalRequestRef: target.parentApprovalRequestRef,
    timeLimitCandidateRef: target.timeLimitCandidateRef,
    routeGateAction: target.routeGateAction,
    parentApprovalRequired: target.parentApprovalRequired,
    reasons: routeGateReasons(target, videoMetadataEvidence),
  });
}

function socialRoute(target, url) {
  const classification = parseBrowserUrlShape({
    classificationId: `${target.id}-url-shape`,
    classifiedAt: observedAt,
    sourceEvidenceIds: [`${target.id}-live-url-evidence`],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'SOCIAL-14 live route gate managed URL evidence',
  });

  return buildBrowserSocialRouteEvidenceFromUrlPattern({
    socialRouteEvidenceId: `${target.id}-social-route-evidence`,
    observedAt,
    sourceEvidenceIds: [`${target.id}-live-social-route-evidence`],
    classification,
  });
}

function routeGateReasons(target, videoMetadataEvidence) {
  const reasons = [...target.reasons];
  if (videoMetadataEvidence !== null) {
    reasons.push(videoMetadataEvidence.metadataState === 'available' ? 'metadata-available' : 'metadata-partial');
  }
  return [...new Set(reasons)];
}

function assertRouteTargetMatches(target, routeEvidence) {
  if (routeEvidence.platform !== target.expectedPlatform) {
    throw new Error(`Expected ${target.id} platform ${target.expectedPlatform} but got ${routeEvidence.platform}`);
  }
  if (routeEvidence.routeKind !== target.expectedRouteKind) {
    throw new Error(`Expected ${target.id} route kind ${target.expectedRouteKind} but got ${routeEvidence.routeKind}`);
  }
}

function assertPlannedCaptureCoverage(plannedCaptures) {
  if (plannedCaptures.length < 4) {
    throw new Error(`Expected at least 4 live SOCIAL-14 route-gate plans, received ${plannedCaptures.length}`);
  }

  const plans = plannedCaptures.map((capture) => capture.planSummary);
  const hasDynamicFeed = plans.some((plan) => plan.routeGateTargetKind === 'social-feed-route');
  const hasShortVideo = plans.some((plan) => plan.routeGateTargetKind === 'social-short-video-route');
  const hasSocialVideo = plans.some((plan) => plan.routeGateTargetKind === 'social-video-route');
  const hasParentReview = plans.some((plan) => plan.routeGateAction === 'parent-review-candidate');
  const hasTimeLimit = plans.some((plan) => plan.routeGateAction === 'limit-route-candidate');
  const hasBlock = plans.some((plan) => plan.routeGateAction === 'block-route-candidate');
  const screenshotCount = plannedCaptures.filter((capture) => capture.screenshotSha256 !== null).length;

  if (!hasDynamicFeed || !hasShortVideo || !hasSocialVideo || !hasParentReview || !hasTimeLimit || !hasBlock) {
    throw new Error(
      'Expected live proof coverage for dynamic feed, short-video, video, parent-review, limit, and block'
    );
  }
  if (screenshotCount !== plannedCaptures.length) {
    throw new Error('Expected every planned live route-gate capture to include a screenshot hash');
  }
}

function buildNegativeChecks(plans) {
  const first = plans[0];
  const limit = plans.find((plan) => plan.routeGateAction === 'limit-route-candidate') ?? first;
  const parentReview = plans.find((plan) => plan.routeGateAction === 'parent-review-candidate') ?? first;

  const invalidRows = [
    ['browser-navigation-block-claim', { ...first, browserNavigationBlockedClaimed: true }],
    ['css-dom-hide-claim', { ...first, cssDomHiddenClaimed: true }],
    ['time-limit-applied-claim', { ...limit, timeLimitAppliedClaimed: true }],
    ['parent-ui-notified-claim', { ...parentReview, parentUiNotifiedClaimed: true }],
    ['policy-decision-claim', { ...first, policyDecisionClaimed: true }],
    ['enforcement-claim', { ...first, enforcementClaimed: true }],
    ['feed-content-capture-claim', { ...first, feedContentCaptured: true }],
    ['recommendation-model-claim', { ...first, recommendationModelClaimed: true }],
    ['limit-without-time-limit-ref', { ...limit, timeLimitCandidateRef: null }],
    [
      'parent-review-without-request-ref',
      { ...parentReview, parentApprovalRequired: true, parentApprovalRequestRef: null },
    ],
  ];

  return invalidRows.map(([name, row]) => ({
    name,
    rejected: !BrowserSocialFeedVideoRouteGatePlanSchema.safeParse(row).success,
  }));
}

function assertBuiltContractsAreFresh() {
  const newestSourceMtime = Math.max(...sourceFiles.map((file) => statSync(join(repoRoot, file)).mtimeMs));
  for (const builtFile of builtFiles) {
    const builtPath = join(repoRoot, builtFile);
    const builtMtime = statSync(builtPath).mtimeMs;
    if (builtMtime < newestSourceMtime) {
      throw new Error(`Build output is stale: ${builtFile}. Run cmd /c npm run build:contracts first.`);
    }
  }
}

function sanitizeUrlForProof(value) {
  try {
    const url = new URL(value);
    url.username = '';
    url.password = '';
    url.search = '';
    url.hash = '';
    return url.toString();
  } catch {
    return value;
  }
}

function hashRef(label, value) {
  const normalized = value.trim();
  if (normalized.length === 0) {
    return null;
  }
  return `${label}-sha256-${sha256(normalized).slice(0, 24)}`;
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function sha256File(path) {
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

function writeJson(path, value) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}
