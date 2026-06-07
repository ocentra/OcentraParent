import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';

import {
  BrowserGameHiddenAnalysisLoaderRequestSchema,
  BrowserGameHiddenAnalysisLoaderResultSchema,
  BrowserGameHiddenAnalysisProfileDesignSchema,
  planBrowserGameHiddenAnalysisProfileSafety,
} from '../../packages/parent-domain/dist/browser-game-hidden-analysis-profile-safety.js';

const repoRoot = process.cwd();
const proofId = 'browser-game-hidden-analysis-profile-safety-live-evidence-proof';
const resultPath = join(repoRoot, 'test-results', proofId, 'proof.json');
const outputProofPath = join(
  repoRoot,
  'output',
  'browser-plan-proof',
  'game-08-hidden-analysis-profile-safety',
  '02-live-profile-safety-shape-proof.json'
);

const targets = [
  {
    targetId: 'poki-subway-surfers',
    url: 'https://poki.com/en/g/subway-surfers',
    profileKind: 'metadata-only-profile',
    capabilityState: 'available',
  },
  {
    targetId: 'coolmath-run-3',
    url: 'https://www.coolmathgames.com/0-run-3',
    profileKind: 'educational-game-review-profile',
    capabilityState: 'available',
  },
  {
    targetId: 'chess-play-online',
    url: 'https://www.chess.com/play/online',
    profileKind: 'isolated-managed-profile',
    capabilityState: 'available',
  },
  {
    targetId: 'playstation-plus-games',
    url: 'https://www.playstation.com/en-us/ps-plus/games/',
    profileKind: 'cloud-session-review-profile',
    capabilityState: 'manual-required',
  },
  {
    targetId: 'xbox-cloud-play',
    url: 'https://www.xbox.com/en-US/play',
    profileKind: 'cloud-session-review-profile',
    capabilityState: 'profile-proof-missing',
  },
];

const startedAt = new Date().toISOString();
const branch = git(['rev-parse', '--abbrev-ref', 'HEAD']);
const commit = git(['rev-parse', 'HEAD']);
const baseCommit = git(['rev-parse', 'origin/main']);
const captures = await Promise.all(targets.map(captureTarget));
const profileRows = captures.flatMap((capture) => profileRowsFor(capture));
const loaderRequests = captures.map(loaderRequestFor);
const loaderResults = loaderRequests.map((request) => planBrowserGameHiddenAnalysisProfileSafety(request));
const proofBackedResults = captures.map(proofBackedResultFor);
const negativeChecks = runNegativeChecks(profileRows[0], loaderRequests[0], proofBackedResults[0]);

if (!captures.every((capture) => capture.responseOk)) {
  throw new Error('Expected all hidden analysis profile safety public captures to return HTTP 2xx/3xx responses');
}
if (!profileRows.every((row) => BrowserGameHiddenAnalysisProfileDesignSchema.safeParse(row).success)) {
  throw new Error('Expected every hidden analysis profile design row to parse');
}
if (!loaderRequests.every((request) => BrowserGameHiddenAnalysisLoaderRequestSchema.safeParse(request).success)) {
  throw new Error('Expected every hidden analysis loader request to parse');
}
if (!loaderResults.every((result) => BrowserGameHiddenAnalysisLoaderResultSchema.safeParse(result).success)) {
  throw new Error('Expected every hidden analysis planned loader result to parse');
}
if (!proofBackedResults.every((result) => BrowserGameHiddenAnalysisLoaderResultSchema.safeParse(result).success)) {
  throw new Error('Expected every proof-backed hidden analysis loader result to parse');
}
if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected hidden analysis profile safety negative checks to reject overclaims');
}

const proof = {
  schemaVersion: 1,
  proofId,
  generatedAt: startedAt,
  branch,
  commit,
  baseCommit,
  captureMode: 'real-public-browser-game-hidden-analysis-profile-safety-shapes',
  targets: captures,
  profileRows,
  loaderRequests,
  loaderResults,
  proofBackedResults,
  negativeChecks,
  summary: {
    targetCount: captures.length,
    profileRows: profileRows.length,
    loaderRequests: loaderRequests.length,
    loaderResults: loaderResults.length,
    proofBackedResults: proofBackedResults.length,
    negativeChecks: negativeChecks.length,
    ocentraOwnedProfileRequired: true,
    separateFromChildVisibleProfileRequired: true,
    boundedRetentionRequired: true,
    childCookiesUsed: false,
    childSessionTokensUsed: false,
    sharedChildStorageUsed: false,
    rawUrlStored: false,
    rawPageBodyStored: false,
    rawGamePayloadStored: false,
    rawScreenFrameStored: false,
    browserInstrumentationClaimed: false,
    hiddenNativeControlClaimed: false,
    aiClassificationClaimed: false,
    finalPolicyDecisionClaimed: false,
    uiRenderedClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    productChecklistUpgradeClaimed: false,
  },
};

await writeJson(resultPath, proof);
await writeJson(outputProofPath, proof);

console.log('browser-game-hidden-analysis-profile-safety-live-evidence-proof-ok=true');
console.log(`proof=${relativePath(resultPath)}`);
console.log(`outputProof=${relativePath(outputProofPath)}`);
console.log(
  `targets=${captures.length} profileRows=${profileRows.length} loaderResults=${
    loaderResults.length + proofBackedResults.length
  } negativeChecks=${negativeChecks.length}`
);

async function captureTarget(target) {
  const inputUrl = new URL(target.url);
  const response = await fetch(target.url, {
    redirect: 'follow',
    headers: {
      'user-agent': 'Mozilla/5.0 OcentraParentBrowserGameProof/1.0',
      accept: 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
    },
  });
  const body = Buffer.from(await response.arrayBuffer());
  const finalUrl = new URL(response.url);
  return {
    targetId: target.targetId,
    status: response.status,
    responseOk: response.status >= 200 && response.status < 400,
    contentType: response.headers.get('content-type') ?? 'unknown',
    contentLength: body.length,
    bodySha256: sha256(body),
    inputOriginSha256: sha256(inputUrl.origin),
    inputPathSha256: sha256(inputUrl.pathname),
    finalOriginSha256: sha256(finalUrl.origin),
    finalPathSha256: sha256(finalUrl.pathname),
    profileKind: target.profileKind,
    capabilityState: target.capabilityState,
    livePageLoadedByChildProfile: false,
    livePageLoadedByHiddenProfile: false,
    childCookiesOrSessionPersisted: false,
    rawUrlPersisted: false,
    rawPageBodyPersisted: false,
    rawGamePayloadPersisted: false,
    rawScreenFramePersisted: false,
  };
}

function profileRowsFor(capture) {
  return [queuedProfileDesignFor(capture), proofBackedProfileDesignFor(capture)];
}

function queuedProfileDesignFor(capture) {
  return profileDesignFor(capture, {
    profileDesignId: `hidden-profile-design-${capture.targetId}`,
    profileKind: capture.profileKind,
    profileFingerprint: `hidden-profile-fingerprint-${sha256(`${capture.targetId}:queued`).slice(0, 32)}`,
    state: 'queued',
    confidence: 'medium',
    loaderProofRef: null,
    summaryRef: null,
    reasonCodes: ['ocentra-owned-profile', 'separate-from-child-profile', 'bounded-retention'],
  });
}

function proofBackedProfileDesignFor(capture) {
  return profileDesignFor(capture, {
    profileDesignId: `hidden-profile-design-proof-${capture.targetId}`,
    profileKind: capture.profileKind === 'isolated-managed-profile' ? 'metadata-only-profile' : capture.profileKind,
    profileFingerprint: `hidden-profile-fingerprint-${sha256(`${capture.targetId}:proof-backed`).slice(0, 32)}`,
    state: 'metadata-only',
    confidence: 'medium',
    loaderProofRef: `hidden-loader-proof-${capture.targetId}`,
    summaryRef: `hidden-analysis-summary-${capture.targetId}`,
    reasonCodes: ['loader-proof-required'],
  });
}

function profileDesignFor(capture, overrides) {
  return {
    schemaVersion: 'browser-game-hidden-analysis-profile-safety-contract',
    profileDesignId: 'hidden-profile-design-live-proof',
    designedAt: startedAt,
    profileKind: 'metadata-only-profile',
    profileFingerprint: 'hidden-profile-fingerprint-live-proof',
    sourceEvidenceRefs: [`parent-proof-${proofId}-${capture.targetId}`],
    state: 'queued',
    confidence: 'medium',
    loaderProofRef: null,
    summaryRef: null,
    reasonCodes: ['ocentra-owned-profile', 'separate-from-child-profile', 'bounded-retention'],
    retentionTtlSeconds: 900,
    maxStructuredSummaryBytes: 2048,
    safety: safeProfileFlags(),
    rawUrlStored: false,
    rawPageBodyStored: false,
    rawGamePayloadStored: false,
    rawScreenFrameStored: false,
    childCookiesOrSessionUsed: false,
    browserInstrumentationClaimed: false,
    hiddenNativeControlClaimed: false,
    aiClassificationClaimed: false,
    finalPolicyDecisionClaimed: false,
    uiRenderedClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function loaderRequestFor(capture) {
  const queuedDesign = queuedProfileDesignFor(capture);
  const capabilityState = capture.capabilityState === 'available' ? 'available' : capture.capabilityState;
  const profileDesign =
    capabilityState === 'available'
      ? queuedDesign
      : {
          ...queuedDesign,
          profileKind: 'manual-required',
          state: capabilityState === 'profile-proof-missing' ? 'profile-proof-missing' : 'manual-required',
          confidence: 'low',
          reasonCodes: [capabilityState === 'profile-proof-missing' ? 'profile-proof-missing' : 'manual-required'],
        };
  return {
    schemaVersion: 'browser-game-hidden-analysis-profile-safety-contract',
    loaderRequestId: `hidden-loader-request-${capture.targetId}`,
    requestedAt: startedAt,
    profileDesign,
    capabilityState,
    policyAllowsHiddenAnalysis: true,
  };
}

function proofBackedResultFor(capture) {
  return {
    schemaVersion: 'browser-game-hidden-analysis-profile-safety-contract',
    loaderResultId: `hidden-loader-result-proof-${capture.targetId}`,
    loaderRequestId: `hidden-loader-request-${capture.targetId}`,
    producedAt: startedAt,
    profileDesignId: `hidden-profile-design-proof-${capture.targetId}`,
    sourceEvidenceRefs: [`parent-proof-${proofId}-${capture.targetId}`],
    state: 'metadata-only',
    confidence: 'medium',
    loaderProofRef: `hidden-loader-proof-${capture.targetId}`,
    summaryRef: `hidden-analysis-summary-${capture.targetId}`,
    reasonCodes: ['loader-proof-required'],
    loadedByHiddenAdapter: true,
    metadataOnly: true,
    rawUrlStored: false,
    rawPageBodyCaptured: false,
    rawGamePayloadCaptured: false,
    rawScreenFrameCaptured: false,
    childCookiesOrSessionUsed: false,
    browserInstrumentationClaimed: false,
    hiddenNativeControlClaimed: false,
    aiClassificationClaimed: false,
    finalPolicyDecisionClaimed: false,
    uiRenderedClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
  };
}

function safeProfileFlags() {
  return {
    ocentraOwnedProfile: true,
    separateFromChildVisibleProfile: true,
    usesChildCookies: false,
    usesChildSessionTokens: false,
    sharesStorageWithChildProfile: false,
    allowsAutoplayAudio: false,
    allowsDownloads: false,
    allowsFormSubmit: false,
    claimsCaptchaAutomation: false,
    claimsLoginBypass: false,
    retainsRawPageBody: false,
    retainsRawGamePayload: false,
    retainsRawScreenFrame: false,
    boundedRetention: true,
  };
}

function runNegativeChecks(validProfile, validRequest, validResult) {
  const invalidProfileSafety = [
    ['uses-child-cookies', { safety: { ...validProfile.safety, usesChildCookies: true } }],
    ['uses-child-session-tokens', { safety: { ...validProfile.safety, usesChildSessionTokens: true } }],
    ['shares-child-storage', { safety: { ...validProfile.safety, sharesStorageWithChildProfile: true } }],
    ['allows-autoplay-audio', { safety: { ...validProfile.safety, allowsAutoplayAudio: true } }],
    ['allows-downloads', { safety: { ...validProfile.safety, allowsDownloads: true } }],
    ['allows-form-submit', { safety: { ...validProfile.safety, allowsFormSubmit: true } }],
    ['claims-captcha-automation', { safety: { ...validProfile.safety, claimsCaptchaAutomation: true } }],
    ['claims-login-bypass', { safety: { ...validProfile.safety, claimsLoginBypass: true } }],
    ['retains-raw-page-body', { safety: { ...validProfile.safety, retainsRawPageBody: true } }],
    ['retains-raw-game-payload', { safety: { ...validProfile.safety, retainsRawGamePayload: true } }],
    ['retains-raw-screen-frame', { safety: { ...validProfile.safety, retainsRawScreenFrame: true } }],
  ];
  const invalidProfileClaims = unsafeProfileClaimFields().map((field) => [
    `profile-${kebab(field)}`,
    { [field]: true },
  ]);
  const invalidResultClaims = unsafeResultClaimFields().map((field) => [`result-${kebab(field)}`, { [field]: true }]);

  return [
    ...invalidProfileSafety.map(([name, invalid]) => negativeProfileCheck(name, validProfile, invalid)),
    ...invalidProfileClaims.map(([name, invalid]) => negativeProfileCheck(name, validProfile, invalid)),
    ...invalidResultClaims.map(([name, invalid]) => negativeResultCheck(name, validResult, invalid)),
    negativeProfileCheck('proof-backed-profile-missing-loader-proof', proofBackedProfileDesignFor(targets[0]), {
      loaderProofRef: null,
    }),
    negativeResultCheck('proof-backed-result-missing-summary-ref', validResult, { summaryRef: null }),
    negativeRequestCheck('available-request-manual-profile', validRequest, {
      capabilityState: 'available',
      profileDesign: {
        ...validRequest.profileDesign,
        profileKind: 'manual-required',
        state: 'manual-required',
        confidence: 'low',
        reasonCodes: ['manual-required'],
      },
    }),
  ];
}

function negativeProfileCheck(name, validProfile, invalid) {
  return {
    name,
    rejected: !BrowserGameHiddenAnalysisProfileDesignSchema.safeParse({ ...validProfile, ...invalid }).success,
  };
}

function negativeRequestCheck(name, validRequest, invalid) {
  return {
    name,
    rejected: !BrowserGameHiddenAnalysisLoaderRequestSchema.safeParse({ ...validRequest, ...invalid }).success,
  };
}

function negativeResultCheck(name, validResult, invalid) {
  return {
    name,
    rejected: !BrowserGameHiddenAnalysisLoaderResultSchema.safeParse({ ...validResult, ...invalid }).success,
  };
}

function unsafeProfileClaimFields() {
  return [
    'rawUrlStored',
    'rawPageBodyStored',
    'rawGamePayloadStored',
    'rawScreenFrameStored',
    'childCookiesOrSessionUsed',
    'browserInstrumentationClaimed',
    'hiddenNativeControlClaimed',
    'aiClassificationClaimed',
    'finalPolicyDecisionClaimed',
    'uiRenderedClaimed',
    'cloudFrameAnalysisClaimed',
    'nativeGameControlClaimed',
    'enforcementClaimed',
  ];
}

function unsafeResultClaimFields() {
  return [
    'rawUrlStored',
    'rawPageBodyCaptured',
    'rawGamePayloadCaptured',
    'rawScreenFrameCaptured',
    'childCookiesOrSessionUsed',
    'browserInstrumentationClaimed',
    'hiddenNativeControlClaimed',
    'aiClassificationClaimed',
    'finalPolicyDecisionClaimed',
    'uiRenderedClaimed',
    'cloudFrameAnalysisClaimed',
    'nativeGameControlClaimed',
    'enforcementClaimed',
  ];
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function kebab(value) {
  return value.replace(/[A-Z]/g, (letter) => `-${letter.toLowerCase()}`);
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
