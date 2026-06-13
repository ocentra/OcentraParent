import { createHash } from 'node:crypto';
import { mkdirSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = fileURLToPath(new URL('../..', import.meta.url));

const proofPath = join(repoRoot, 'test-results/dynamic-social-live-url-proof/proof.json');
const outputProofPath = join(
  repoRoot,
  'output/browser-plan-proof/ai-23-dynamic-feed-social-url-handling/11-live-dynamic-social-url-proof.json'
);

const sourceFiles = [
  'packages/browser-domain/src/browser-url-intelligence-schemas.ts',
  'packages/browser-domain/src/browser-url-intelligence.ts',
];

const builtFiles = [
  'packages/browser-domain/dist/browser-url-intelligence-schemas.js',
  'packages/browser-domain/dist/browser-url-intelligence.js',
];

const liveRouteCases = [
  {
    label: 'instagram-explore-feed',
    url: 'https://www.instagram.com/explore/',
    expectedHost: 'www.instagram.com',
    expectedPlatform: 'instagram',
    expectedTargetKind: 'social-feed',
    expectedReasonCodes: ['dynamic-feed', 'parsed-social-route'],
  },
  {
    label: 'instagram-public-reel',
    url: 'https://www.instagram.com/reel/Cu8Z9Z1sW4Z/',
    expectedHost: 'www.instagram.com',
    expectedPlatform: 'instagram',
    expectedTargetKind: 'short-video',
    expectedPostId: 'Cu8Z9Z1sW4Z',
    expectedReasonCodes: ['parsed-post-id', 'parsed-social-route'],
  },
  {
    label: 'instagram-direct-inbox',
    url: 'https://www.instagram.com/direct/inbox/',
    expectedHost: 'www.instagram.com',
    expectedPlatform: 'instagram',
    expectedTargetKind: 'social-messaging',
    expectedReasonCodes: ['parsed-social-route'],
  },
  {
    label: 'instagram-create-post',
    url: 'https://www.instagram.com/create/select/',
    expectedHost: 'www.instagram.com',
    expectedPlatform: 'instagram',
    expectedTargetKind: 'social-upload-post',
    expectedReasonCodes: ['parsed-social-route'],
  },
  {
    label: 'tiktok-for-you-feed',
    url: 'https://www.tiktok.com/foryou',
    expectedHost: 'www.tiktok.com',
    expectedPlatform: 'tiktok',
    expectedTargetKind: 'social-feed',
    expectedReasonCodes: ['dynamic-feed', 'parsed-social-route'],
  },
  {
    label: 'tiktok-live-route',
    url: 'https://www.tiktok.com/@tiktok/live',
    expectedHost: 'www.tiktok.com',
    expectedPlatform: 'tiktok',
    expectedTargetKind: 'social-livestream',
    expectedReasonCodes: ['parsed-social-route'],
  },
  {
    label: 'facebook-watch-feed',
    url: 'https://www.facebook.com/watch/',
    expectedHost: 'www.facebook.com',
    expectedPlatform: 'facebook',
    expectedTargetKind: 'social-feed',
    expectedReasonCodes: ['dynamic-feed', 'parsed-social-route'],
  },
  {
    label: 'facebook-live-route',
    url: 'https://www.facebook.com/live/',
    expectedHost: 'www.facebook.com',
    expectedPlatform: 'facebook',
    expectedTargetKind: 'social-livestream',
    expectedReasonCodes: ['parsed-social-route'],
  },
  {
    label: 'twitch-following-feed',
    url: 'https://www.twitch.tv/directory/following',
    expectedHost: 'www.twitch.tv',
    expectedPlatform: 'twitch',
    expectedTargetKind: 'social-feed',
    expectedReasonCodes: ['dynamic-feed', 'parsed-social-route'],
  },
  {
    label: 'x-home-feed',
    url: 'https://x.com/home',
    expectedHost: 'x.com',
    expectedPlatform: 'x-twitter',
    expectedTargetKind: 'social-feed',
    expectedReasonCodes: ['dynamic-feed', 'parsed-social-route'],
  },
  {
    label: 'x-search-route',
    url: 'https://x.com/search?q=fractions',
    expectedHost: 'x.com',
    expectedPlatform: 'x-twitter',
    expectedTargetKind: 'search',
    expectedQuery: 'fractions',
    expectedReasonCodes: ['parsed-search-query', 'parsed-social-route'],
  },
  {
    label: 'x-public-status-route',
    url: 'https://x.com/OpenAI/status/1790000000000000000',
    expectedHost: 'x.com',
    expectedPlatform: 'x-twitter',
    expectedTargetKind: 'social-post',
    expectedPostId: '1790000000000000000',
    expectedReasonCodes: ['parsed-post-id', 'parsed-social-route'],
  },
  {
    label: 'reddit-root-feed',
    url: 'https://www.reddit.com/',
    expectedHost: 'www.reddit.com',
    expectedPlatform: 'reddit',
    expectedTargetKind: 'social-feed',
    expectedReasonCodes: ['dynamic-feed', 'parsed-social-route'],
  },
  {
    label: 'reddit-community-forum',
    url: 'https://www.reddit.com/r/Parenting/',
    expectedHost: 'www.reddit.com',
    expectedPlatform: 'reddit',
    expectedTargetKind: 'forum',
    expectedChannelId: 'Parenting',
    expectedReasonCodes: ['parsed-channel-id', 'parsed-social-route'],
  },
  {
    label: 'reddit-public-comment-route',
    url: 'https://www.reddit.com/r/Parenting/comments/1d00000/example/',
    expectedHost: 'www.reddit.com',
    expectedPlatform: 'reddit',
    expectedTargetKind: 'social-post',
    expectedPostId: '1d00000',
    expectedReasonCodes: ['parsed-post-id', 'parsed-social-route'],
  },
  {
    label: 'discord-channel-route',
    url: 'https://discord.com/channels/@me',
    expectedHost: 'discord.com',
    expectedPlatform: 'discord',
    expectedTargetKind: 'social-messaging',
    expectedReasonCodes: ['parsed-social-route'],
  },
];

assertBuiltContractsAreFresh();

const { parseBrowserUrlShape } = await import(
  pathToFileURL(join(repoRoot, 'packages/browser-domain/dist/browser-url-intelligence.js')).href
);
const {
  BrowserUrlIntelligenceMemoryHitSchema,
  BrowserUrlIntelligenceMemorySchemaVersion,
  BrowserUrlShapeClassificationResultSchema,
  BrowserUrlShapeSchemaVersion,
} = await import(
  pathToFileURL(join(repoRoot, 'packages/browser-domain/dist/browser-url-intelligence-schemas.js')).href
);

const fetchedRoutes = await Promise.all(liveRouteCases.map(fetchLiveRoute));
const routeProofs = liveRouteCases.map((routeCase, index) => {
  const liveResponse = fetchedRoutes[index];
  const classification = parseManagedUrl(routeCase, liveResponse);
  assertRouteMatches(routeCase, classification);
  assertNoClaimFlags(classification);
  return {
    label: routeCase.label,
    liveResponse: liveResponseSummary(routeCase, liveResponse),
    classification: classificationSummary(classification),
  };
});

const negativeChecks = runNegativeChecks();

const proof = {
  proofName: 'dynamic-social-live-url-proof',
  generatedAt: new Date().toISOString(),
  generatedOrFixturePageUsed: false,
  liveNetworkEvidenceUsed: true,
  persistedRawHtml: false,
  persistedRawTitle: false,
  persistedRawDescription: false,
  routeCount: routeProofs.length,
  routeProofs,
  negativeChecks,
  noClaimChecks: {
    accountIdentityClaimed: false,
    feedContentSemanticsClaimed: false,
    messageContentClaimed: false,
    uploadMonitoringClaimed: false,
    livestreamContentClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    policyAuthorityClaimed: false,
    nativeAppControlClaimed: false,
    connectorAccessClaimed: false,
    enforcementClaimed: false,
  },
};

writeJson(proofPath, proof);
writeJson(outputProofPath, proof);

console.log('dynamic-social-live-url-proof-ok=true');
console.log(`proof=${proofPath}`);
console.log(`outputProof=${outputProofPath}`);
console.log(`routeCount=${routeProofs.length}`);
console.log(`platforms=${[...new Set(routeProofs.map((route) => route.classification.platform))].join(',')}`);

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

async function fetchLiveRoute(routeCase) {
  const response = await fetch(routeCase.url, {
    redirect: 'manual',
    signal: AbortSignal.timeout(15000),
    headers: {
      accept: 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
      'user-agent': 'Mozilla/5.0 OcentraParentProof/1.0',
    },
  });
  const body = await response.text();
  if (response.status < 200 || response.status >= 500) {
    throw new Error(`Expected a live non-5xx response for ${routeCase.label} but got ${response.status}`);
  }
  const host = new URL(routeCase.url).host;
  if (host !== routeCase.expectedHost) {
    throw new Error(`Expected ${routeCase.label} host ${routeCase.expectedHost} but got ${host}`);
  }
  return {
    url: routeCase.url,
    status: response.status,
    contentType: response.headers.get('content-type') ?? null,
    location: response.headers.get('location'),
    body,
  };
}

function parseManagedUrl(routeCase, liveResponse) {
  return parseBrowserUrlShape({
    classificationId: `url-shape-live-${routeCase.label}`,
    classifiedAt: new Date().toISOString(),
    sourceEvidenceIds: [`browser-evidence-live-${routeCase.label}`],
    sourceKind: 'managed-browser-exact-url',
    url: routeCase.url,
    title: extractTitle(liveResponse.body),
  });
}

function runNegativeChecks() {
  const unmanaged = parseBrowserUrlShape({
    classificationId: 'url-shape-live-unmanaged-social-feed',
    classifiedAt: new Date().toISOString(),
    sourceEvidenceIds: ['browser-evidence-live-unmanaged-social'],
    sourceKind: 'unmanaged-browser-process',
    url: 'https://www.instagram.com/explore/',
    title: 'Ignored unmanaged social route',
  });
  const socialPostWithoutPostId = BrowserUrlShapeClassificationResultSchema.safeParse({
    schemaVersion: BrowserUrlShapeSchemaVersion,
    classificationId: 'url-shape-invalid-social-post-without-id',
    classifiedAt: new Date().toISOString(),
    sourceEvidenceIds: ['browser-evidence-invalid-social-post'],
    sourceKind: 'managed-browser-exact-url',
    url: 'https://x.com/OpenAI/status/',
    domain: 'x.com',
    title: null,
    targetKind: 'social-post',
    platform: 'x-twitter',
    platformIds: emptyPlatformIds(),
    confidence: 'high',
    reasonCodes: ['parsed-url', 'parsed-post-id', 'parsed-social-route'],
    exactUrlEvidence: true,
    contentSemanticsClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
  });
  const staleDynamicFeedMemory = BrowserUrlIntelligenceMemoryHitSchema.safeParse({
    ...memoryHit(),
    hitState: 'stale',
    staleReason: 'dynamic-feed-ttl',
    canDrivePolicyInput: false,
  });
  const staleDynamicFeedPolicyInputClaim = BrowserUrlIntelligenceMemoryHitSchema.safeParse({
    ...memoryHit(),
    hitState: 'stale',
    staleReason: 'dynamic-feed-ttl',
    canDrivePolicyInput: true,
  });
  if (unmanaged.targetKind !== 'unknown' || unmanaged.exactUrlEvidence !== false) {
    throw new Error('Expected unmanaged social route to remain unknown and non-exact');
  }
  if (socialPostWithoutPostId.success) {
    throw new Error('Expected social-post without post id to be rejected');
  }
  if (!staleDynamicFeedMemory.success) {
    throw new Error('Expected stale dynamic-feed memory row without policy authority to be accepted');
  }
  if (staleDynamicFeedPolicyInputClaim.success) {
    throw new Error('Expected stale dynamic-feed memory row claiming policy input to be rejected');
  }
  return {
    unmanagedSocialRouteRejectedAsExact: true,
    socialPostWithoutPostIdRejected: true,
    staleDynamicFeedMemoryCannotDrivePolicyInput: true,
    staleDynamicFeedPolicyInputClaimRejected: true,
  };
}

function assertRouteMatches(routeCase, classification) {
  assertField(routeCase, classification, 'platform', routeCase.expectedPlatform);
  assertField(routeCase, classification, 'targetKind', routeCase.expectedTargetKind);
  if (!classification.exactUrlEvidence) {
    throw new Error(`Expected exact URL evidence for ${routeCase.label}`);
  }
  if (routeCase.expectedPostId !== undefined) {
    assertPlatformId(routeCase, classification, 'postId', routeCase.expectedPostId);
  }
  if (routeCase.expectedChannelId !== undefined) {
    assertPlatformId(routeCase, classification, 'channelId', routeCase.expectedChannelId);
  }
  if (routeCase.expectedQuery !== undefined) {
    assertPlatformId(routeCase, classification, 'query', routeCase.expectedQuery);
  }
  for (const reasonCode of routeCase.expectedReasonCodes) {
    if (!classification.reasonCodes.includes(reasonCode)) {
      throw new Error(`Expected ${routeCase.label} to include reason ${reasonCode}`);
    }
  }
}

function assertField(routeCase, classification, field, expected) {
  if (classification[field] !== expected) {
    throw new Error(`Expected ${routeCase.label} ${field}=${expected} but got ${classification[field]}`);
  }
}

function assertPlatformId(routeCase, classification, field, expected) {
  if (classification.platformIds[field] !== expected) {
    throw new Error(`Expected ${routeCase.label} platformIds.${field}=${expected}`);
  }
}

function assertNoClaimFlags(classification) {
  if (
    classification.contentSemanticsClaimed ||
    classification.aiDecisionClaimed ||
    classification.policyDecisionClaimed
  ) {
    throw new Error(`Unexpected content, AI, or policy claim in ${classification.classificationId}`);
  }
}

function liveResponseSummary(routeCase, liveResponse) {
  const parsed = new URL(routeCase.url);
  const location = liveResponse.location === null ? null : new URL(liveResponse.location, routeCase.url);
  return {
    host: parsed.host,
    pathHash: sha256(parsed.pathname),
    queryHash: parsed.search.length > 0 ? sha256(parsed.search) : null,
    status: liveResponse.status,
    contentType: liveResponse.contentType,
    byteLength: Buffer.byteLength(liveResponse.body),
    bodyHash: liveResponse.body.length > 0 ? sha256(liveResponse.body) : null,
    titleHash: hashOrNull(extractTitle(liveResponse.body)),
    titleLength: extractTitle(liveResponse.body)?.length ?? 0,
    redirectHost: location?.host ?? null,
    redirectPathHash: location === null ? null : sha256(location.pathname),
  };
}

function classificationSummary(classification) {
  return {
    classificationId: classification.classificationId,
    sourceKind: classification.sourceKind,
    platform: classification.platform,
    targetKind: classification.targetKind,
    exactUrlEvidence: classification.exactUrlEvidence,
    confidence: classification.confidence,
    reasonCodes: classification.reasonCodes,
    platformIds: classification.platformIds,
    contentSemanticsClaimed: classification.contentSemanticsClaimed,
    aiDecisionClaimed: classification.aiDecisionClaimed,
    policyDecisionClaimed: classification.policyDecisionClaimed,
  };
}

function extractTitle(html) {
  return decodeEntities(html.match(/<title[^>]*>([^<]+)/iu)?.[1]?.trim() ?? null);
}

function decodeEntities(value) {
  if (value === null) {
    return null;
  }
  return value
    .replace(/&amp;/giu, '&')
    .replace(/&lt;/giu, '<')
    .replace(/&gt;/giu, '>')
    .replace(/&quot;/giu, '"')
    .replace(/&#39;/giu, "'");
}

function memoryHit() {
  return {
    schemaVersion: BrowserUrlIntelligenceMemorySchemaVersion,
    memoryHitId: 'memory-hit-live-social-feed',
    lookedUpAt: new Date().toISOString(),
    key: {
      keyKind: 'domain-path-hash',
      keyValue: 'instagram:explore',
    },
    hitState: 'hit',
    decisionKind: 'known-allowed',
    sourceEvidenceIds: ['browser-evidence-live-instagram-explore-feed'],
    analysisRef: 'ai-analysis-live-social-feed',
    parentActionRef: null,
    policyVersionRef: 'policy-version-browser-ai-23',
    expiresAt: new Date(Date.now() + 60_000).toISOString(),
    staleReason: null,
    canDrivePolicyInput: true,
    canDirectlyEnforce: false,
  };
}

function emptyPlatformIds() {
  return {
    videoId: null,
    channelId: null,
    playlistId: null,
    postId: null,
    query: null,
  };
}

function hashOrNull(value) {
  return value === null ? null : sha256(value);
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function writeJson(path, value) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
