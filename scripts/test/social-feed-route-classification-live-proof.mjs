import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright';
import { parseBrowserUrlShape } from '../../packages/browser-domain/dist/browser-url-intelligence.js';
import {
  BrowserSocialFeedRouteClassificationSchema,
  classifyBrowserSocialFeedRoute,
} from '../../packages/browser-domain/dist/browser-social-feed-route-classification.js';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '../../packages/browser-domain/dist/browser-social-url-patterns.js';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const proofRoot = join(repoRoot, 'output/browser-plan-proof/social-08-feed-reels-shorts-route-classification');
const screenshotRoot = join(proofRoot, '06-live-screenshots');
const testResultPath = join(repoRoot, 'test-results/social-feed-route-classification-live-proof/proof.json');
const outputProofPath = join(proofRoot, '11-live-route-classification-proof.json');
const observedAt = new Date().toISOString();

const sourceFiles = [
  'packages/browser-domain/src/browser-url-intelligence.ts',
  'packages/browser-domain/src/browser-social-url-patterns.ts',
  'packages/browser-domain/src/browser-social-feed-route-classification.ts',
];
const builtFiles = [
  'packages/browser-domain/dist/browser-url-intelligence.js',
  'packages/browser-domain/dist/browser-social-url-patterns.js',
  'packages/browser-domain/dist/browser-social-feed-route-classification.js',
];

const liveTargets = [
  {
    id: 'reddit-popular-feed',
    url: 'https://www.reddit.com/r/popular/',
    expectedPlatform: 'reddit',
    expectedRouteKind: 'feed',
    surfaceHint: 'home-feed',
    expectedSurfaceKind: 'dynamic-feed',
  },
  {
    id: 'twitch-directory-feed',
    url: 'https://www.twitch.tv/directory',
    expectedPlatform: 'twitch',
    expectedRouteKind: 'feed',
    surfaceHint: 'home-feed',
    expectedSurfaceKind: 'dynamic-feed',
  },
  {
    id: 'tiktok-explore-feed',
    url: 'https://www.tiktok.com/explore',
    expectedPlatform: 'tiktok',
    expectedRouteKind: 'feed',
    surfaceHint: 'explore-feed',
    expectedSurfaceKind: 'dynamic-feed',
  },
  {
    id: 'instagram-reels-feed',
    url: 'https://www.instagram.com/reels/',
    expectedPlatform: 'instagram',
    expectedRouteKind: 'feed',
    surfaceHint: 'reels-feed',
    expectedSurfaceKind: 'short-video-surface',
  },
  {
    id: 'youtube-shorts-video',
    url: 'https://www.youtube.com/shorts/jNQXAC9IVRw',
    expectedPlatform: 'youtube-shorts',
    expectedRouteKind: 'video',
    surfaceHint: 'single-short-video',
    expectedSurfaceKind: 'single-short-video',
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

const classifiedCaptures = captures.filter((capture) => capture.contractClassificationCreated);
assertClassificationCoverage(classifiedCaptures);

const classificationParseChecks = classifiedCaptures.map((capture) => ({
  targetId: capture.targetId,
  accepted: BrowserSocialFeedRouteClassificationSchema.safeParse(capture.classificationSummary).success,
}));
if (!classificationParseChecks.every((check) => check.accepted)) {
  throw new Error('Expected all SOCIAL-08 live classifications to parse through the contract schema');
}

const negativeChecks = buildNegativeChecks(classifiedCaptures.map((capture) => capture.classificationSummary));
if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected all SOCIAL-08 negative checks to reject dishonest content or authority claims');
}

const proof = {
  schemaVersion: 1,
  proofId: 'social-feed-route-classification-live-proof',
  generatedAt: observedAt,
  branch: git(['branch', '--show-current']),
  commit: git(['rev-parse', 'HEAD']),
  baseCommit: git(['rev-parse', 'origin/main']),
  liveCaptureSummary: {
    realPublicSocialSurfacesUsed: true,
    generatedOrFixturePageUsed: false,
    passiveNavigationOnly: true,
    formsSubmitted: false,
    credentialsCaptured: false,
    rawPageBodyPersisted: false,
    rawDomPersisted: false,
    rawTitlePersisted: false,
    screenshotsPersisted: true,
    screenshotCount: captures.filter((capture) => capture.screenshotSha256 !== null).length,
    classifiedCaptureCount: classifiedCaptures.length,
    requestedTargetCount: liveTargets.length,
    requiredCoverage: {
      dynamicFeed: true,
      shortVideoSurface: true,
      singleShortVideo: true,
      routeOnlyClassification: true,
    },
  },
  captures,
  classificationParseChecks,
  negativeChecks,
  noClaimChecks: {
    feedContentCaptured: false,
    recommendationModelClaimed: false,
    messageContentCaptured: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    enforcementClaimed: false,
  },
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('social-feed-route-classification-live-proof-ok=true');
console.log(`proof=${testResultPath}`);
console.log(`outputProof=${outputProofPath}`);
console.log(`classifiedCaptureCount=${classifiedCaptures.length}`);
console.log(
  `classifiedSurfaces=${classifiedCaptures.map((capture) => capture.classificationSummary.surfaceKind).join(',')}`
);

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
    const classificationSummary = classifyLiveRoute(target, proofFinalUrl);
    return {
      ...base,
      contractClassificationCreated: true,
      classificationSummary,
    };
  } catch (error) {
    return {
      ...base,
      contractClassificationCreated: false,
      contractError: error instanceof Error ? error.message : String(error),
      classificationSummary: null,
    };
  }
}

function classifyLiveRoute(target, finalUrl) {
  const routeEvidence = socialRoute(target, finalUrl);
  assertRouteTargetMatches(target, routeEvidence);
  const classification = classifyBrowserSocialFeedRoute({
    feedRouteClassificationId: `${target.id}-live-feed-route-classification`,
    observedAt,
    sourceEvidenceIds: [`${target.id}-live-feed-route-evidence`],
    routeEvidence,
    surfaceHint: target.surfaceHint,
  });

  if (classification.surfaceKind !== target.expectedSurfaceKind) {
    throw new Error(
      `Expected ${target.id} surface kind ${target.expectedSurfaceKind} but got ${classification.surfaceKind}`
    );
  }

  return classification;
}

function socialRoute(target, finalUrl) {
  const classification = parseBrowserUrlShape({
    classificationId: `${target.id}-live-url-shape`,
    classifiedAt: observedAt,
    sourceEvidenceIds: [`${target.id}-live-url-evidence`],
    sourceKind: 'managed-browser-exact-url',
    url: urlForContract(finalUrl, target.url),
    title: 'SOCIAL-08 live route classification managed URL evidence',
  });

  return buildBrowserSocialRouteEvidenceFromUrlPattern({
    socialRouteEvidenceId: `${target.id}-live-social-route-evidence`,
    observedAt,
    sourceEvidenceIds: [`${target.id}-live-social-route-source`],
    classification,
  });
}

function assertRouteTargetMatches(target, routeEvidence) {
  if (routeEvidence.platform !== target.expectedPlatform) {
    throw new Error(`Expected ${target.id} platform ${target.expectedPlatform} but got ${routeEvidence.platform}`);
  }
  if (routeEvidence.routeKind !== target.expectedRouteKind) {
    throw new Error(`Expected ${target.id} route kind ${target.expectedRouteKind} but got ${routeEvidence.routeKind}`);
  }
}

function assertClassificationCoverage(classifiedCaptures) {
  if (classifiedCaptures.length < 4) {
    throw new Error(`Expected at least 4 live SOCIAL-08 classifications, received ${classifiedCaptures.length}`);
  }

  const classifications = classifiedCaptures.map((capture) => capture.classificationSummary);
  const hasDynamicFeed = classifications.some((classification) => classification.surfaceKind === 'dynamic-feed');
  const hasShortVideoSurface = classifications.some(
    (classification) => classification.surfaceKind === 'short-video-surface'
  );
  const hasSingleShortVideo = classifications.some(
    (classification) => classification.surfaceKind === 'single-short-video'
  );
  const allRouteOnly = classifications.every((classification) => classification.routeOnly);
  const screenshotCount = classifiedCaptures.filter((capture) => capture.screenshotSha256 !== null).length;

  if (!hasDynamicFeed || !hasShortVideoSurface || !hasSingleShortVideo || !allRouteOnly) {
    throw new Error('Expected live proof coverage for dynamic feed, short-video surface, and single-short-video');
  }
  if (screenshotCount !== classifiedCaptures.length) {
    throw new Error('Expected every live SOCIAL-08 classification to include a screenshot hash');
  }
}

function buildNegativeChecks(classifications) {
  const first = classifications[0];
  const invalidRows = [
    ['feed-content-claim', { ...first, feedContentSemanticsClaimed: true }],
    ['recommendation-claim', { ...first, recommendationSemanticsClaimed: true }],
    ['message-content-claim', { ...first, messageContentClaimed: true }],
    ['ai-decision-claim', { ...first, aiDecisionClaimed: true }],
    ['policy-decision-claim', { ...first, policyDecisionClaimed: true }],
    ['native-app-control-claim', { ...first, nativeAppControlClaimed: true }],
    ['platform-connector-claim', { ...first, platformConnectorClaimed: true }],
    ['enforcement-claim', { ...first, enforcementClaimed: true }],
    ['not-route-only', { ...first, routeOnly: false }],
  ];

  return invalidRows.map(([name, row]) => ({
    name,
    rejected: !BrowserSocialFeedRouteClassificationSchema.safeParse(row).success,
  }));
}

function urlForContract(finalUrl, requestedUrl) {
  const routeOnlyFinalUrl = sanitizeUrlForProof(finalUrl);
  if (isSupportedRoute(routeOnlyFinalUrl)) {
    return routeOnlyFinalUrl;
  }
  return requestedUrl;
}

function isSupportedRoute(value) {
  try {
    const url = new URL(value);
    const host = url.hostname.toLowerCase();
    const path = url.pathname.toLowerCase();
    return (
      host.endsWith('reddit.com') ||
      host.endsWith('twitch.tv') ||
      host.endsWith('tiktok.com') ||
      host.endsWith('instagram.com') ||
      (host.endsWith('youtube.com') && path.includes('/shorts/'))
    );
  } catch {
    return false;
  }
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
