import { createHash } from 'node:crypto';
import { mkdirSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = fileURLToPath(new URL('../..', import.meta.url));

const vimeoPageUrl = 'https://vimeo.com/76979871';
const vimeoPlayerUrl = 'https://player.vimeo.com/video/76979871';
const genericVideoObjectUrl = 'https://www.ted.com/talks/sir_ken_robinson_do_schools_kill_creativity';

const proofPath = join(repoRoot, 'test-results/vimeo-generic-live-metadata-proof/proof.json');
const outputProofPath = join(
  repoRoot,
  'output/browser-plan-proof/ai-22-vimeo-generic-video-parser/11-live-vimeo-generic-metadata-proof.json'
);

const sourceFiles = [
  'packages/browser-domain/src/browser-url-intelligence.ts',
  'packages/schema-domain/src/browser-video-metadata.ts',
];

const builtFiles = [
  'packages/browser-domain/dist/browser-url-intelligence.js',
  'packages/schema-domain/dist/browser-video-metadata.js',
];

assertBuiltContractsAreFresh();

const { parseBrowserUrlShape } = await import(
  pathToFileURL(join(repoRoot, 'packages/browser-domain/dist/browser-url-intelligence.js')).href
);
const { buildVideoMetadataEvidence } = await import(
  pathToFileURL(join(repoRoot, 'packages/schema-domain/dist/browser-video-metadata.js')).href
);

const [vimeoPage, vimeoPlayer, genericVideoPage] = await Promise.all([
  fetchLiveHtml(vimeoPageUrl),
  fetchLiveHtml(vimeoPlayerUrl),
  fetchLiveHtml(genericVideoObjectUrl),
]);

const vimeoVideoObject = requireVideoObject(vimeoPlayer.html, vimeoPlayerUrl);
const genericVideoObject = requireVideoObject(genericVideoPage.html, genericVideoObjectUrl);

const vimeoPageClassification = parseManagedUrl({
  classificationId: 'url-shape-live-vimeo-page-76979871',
  sourceEvidenceId: 'browser-evidence-live-vimeo-page',
  url: vimeoPageUrl,
  title: extractTitle(vimeoPage.html),
});
const vimeoPlayerClassification = parseManagedUrl({
  classificationId: 'url-shape-live-vimeo-player-76979871',
  sourceEvidenceId: 'browser-evidence-live-vimeo-player',
  url: vimeoPlayerUrl,
  title: extractTitle(vimeoPlayer.html),
});
const genericClassification = parseManagedUrl({
  classificationId: 'url-shape-live-generic-video-object-ted',
  sourceEvidenceId: 'browser-evidence-live-generic-video-object',
  url: genericVideoObjectUrl,
  title: extractTitle(genericVideoPage.html),
});

const vimeoEvidence = buildVideoMetadataEvidence({
  metadataEvidenceId: 'metadata-evidence-live-vimeo-76979871',
  collectedAt: new Date().toISOString(),
  sourceEvidenceIds: ['browser-evidence-live-vimeo-player'],
  classification: vimeoPlayerClassification,
  sourceKind: 'platform-page-metadata',
  sourceRef: 'live-vimeo-player-jsonld',
  browserTitle: extractTitle(vimeoPlayer.html),
  openGraphTitle: textOrNull(vimeoVideoObject.name),
  openGraphDescription: textOrNull(vimeoVideoObject.description),
  platformVideoIdOverride: null,
  channelName: authorName(vimeoVideoObject.author),
  thumbnailUrl: urlOrNull(firstValue(vimeoVideoObject.thumbnailUrl)),
  thumbnailHashRef: hashRef('thumbnail', firstValue(vimeoVideoObject.thumbnailUrl)),
  durationSeconds: durationSeconds(vimeoVideoObject.duration),
  publishDate: timestampOrNull(vimeoVideoObject.uploadDate),
  captionsAvailable: null,
  transcriptAvailable: null,
  platformCategory: null,
  platformRating: null,
  restrictedSignal: null,
  degradedReasons: [],
});

const genericEvidence = buildVideoMetadataEvidence({
  metadataEvidenceId: 'metadata-evidence-live-generic-video-object-ted',
  collectedAt: new Date().toISOString(),
  sourceEvidenceIds: ['browser-evidence-live-generic-video-object'],
  classification: genericClassification,
  sourceKind: 'schema-org-video-object',
  sourceRef: 'live-generic-schema-org-video-object',
  browserTitle: extractTitle(genericVideoPage.html),
  openGraphTitle: textOrNull(genericVideoObject.name),
  openGraphDescription: textOrNull(genericVideoObject.description),
  platformVideoIdOverride: hashRef('generic-video-object', genericVideoObject.embedUrl ?? genericVideoObject.url),
  channelName: authorName(genericVideoObject.author),
  thumbnailUrl: urlOrNull(firstValue(genericVideoObject.thumbnailUrl)),
  thumbnailHashRef: hashRef('thumbnail', firstValue(genericVideoObject.thumbnailUrl)),
  durationSeconds: durationSeconds(genericVideoObject.duration),
  publishDate: timestampOrNull(genericVideoObject.uploadDate),
  captionsAvailable: null,
  transcriptAvailable: null,
  platformCategory: null,
  platformRating: null,
  restrictedSignal: null,
  degradedReasons: [],
});

const negativeChecks = runNegativeChecks(genericClassification);
assertNoClaimFlags(vimeoEvidence);
assertNoClaimFlags(genericEvidence);

const proof = {
  proofName: 'vimeo-generic-live-metadata-proof',
  generatedAt: new Date().toISOString(),
  generatedOrFixturePageUsed: false,
  liveNetworkEvidenceUsed: true,
  persistedRawHtml: false,
  persistedRawTitle: false,
  persistedRawDescription: false,
  sources: {
    vimeoPage: sourceSummary(vimeoPage, vimeoPage.html, { expectedHost: 'vimeo.com' }),
    vimeoPlayer: sourceSummary(vimeoPlayer, vimeoPlayer.html, { expectedHost: 'player.vimeo.com' }),
    genericVideoObjectPage: sourceSummary(genericVideoPage, genericVideoPage.html, { expectedHost: 'www.ted.com' }),
  },
  parserEvidence: {
    vimeoPage: classificationSummary(vimeoPageClassification),
    vimeoPlayer: classificationSummary(vimeoPlayerClassification),
    genericVideoObjectPage: classificationSummary(genericClassification),
  },
  metadataEvidence: {
    vimeo: metadataSummary(vimeoEvidence),
    genericVideoObject: metadataSummary(genericEvidence),
  },
  extractedVideoObject: {
    vimeo: videoObjectSummary(vimeoVideoObject),
    genericVideoObject: videoObjectSummary(genericVideoObject),
  },
  negativeChecks,
  noClaimChecks: {
    pageBodyCaptured: false,
    transcriptTextCaptured: false,
    contentSemanticsClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    policyAuthorityClaimed: false,
    hiddenPageLoadClaimed: false,
    enforcementClaimed: false,
  },
};

writeJson(proofPath, proof);
writeJson(outputProofPath, proof);

console.log('vimeo-generic-live-metadata-proof-ok=true');
console.log(`proof=${proofPath}`);
console.log(`outputProof=${outputProofPath}`);
console.log(`vimeoVideoId=${vimeoEvidence.fields.platformVideoId}`);
console.log(`genericMetadataState=${genericEvidence.metadataState}`);

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

async function fetchLiveHtml(url) {
  const response = await fetch(url, {
    headers: {
      accept: 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
      'user-agent': 'Mozilla/5.0 OcentraParentProof/1.0',
    },
  });
  const html = await response.text();
  if (!response.ok) {
    throw new Error(`Expected live HTTP success for ${host(url)} but got ${response.status}`);
  }
  if (html.length < 1024) {
    throw new Error(`Expected live HTML body for ${host(url)} to be substantial`);
  }
  return {
    url,
    status: response.status,
    contentType: response.headers.get('content-type') ?? null,
    html,
  };
}

function parseManagedUrl({ classificationId, sourceEvidenceId, url, title }) {
  return parseBrowserUrlShape({
    classificationId,
    classifiedAt: new Date().toISOString(),
    sourceEvidenceIds: [sourceEvidenceId],
    sourceKind: 'managed-browser-exact-url',
    url,
    title,
  });
}

function extractTitle(html) {
  return decodeEntities(html.match(/<title[^>]*>([^<]+)/iu)?.[1]?.trim() ?? null);
}

function requireVideoObject(html, url) {
  const candidates = [];
  for (const block of jsonLdBlocks(html)) {
    try {
      collectVideoObjects(JSON.parse(block), candidates);
    } catch {
      continue;
    }
  }
  const selected = candidates.find((candidate) => textOrNull(candidate.name) !== null) ?? candidates[0];
  if (selected === undefined) {
    throw new Error(`Expected live VideoObject JSON-LD for ${host(url)}`);
  }
  return selected;
}

function jsonLdBlocks(html) {
  return [...html.matchAll(/<script[^>]+type=["']application\/ld\+json["'][^>]*>([\s\S]*?)<\/script>/giu)].map(
    (match) => match[1].trim()
  );
}

function collectVideoObjects(value, output) {
  if (value === null || typeof value !== 'object') {
    return;
  }
  if (Array.isArray(value)) {
    for (const item of value) {
      collectVideoObjects(item, output);
    }
    return;
  }

  const typeValue = value['@type'];
  const types = Array.isArray(typeValue) ? typeValue : [typeValue];
  if (types.includes('VideoObject')) {
    output.push(value);
  }
  for (const child of Object.values(value)) {
    collectVideoObjects(child, output);
  }
}

function runNegativeChecks(genericClassification) {
  const checks = [];
  expectThrows('generic-open-graph-only-rejected', () =>
    buildVideoMetadataEvidence({
      metadataEvidenceId: 'metadata-evidence-live-generic-open-graph-negative',
      collectedAt: new Date().toISOString(),
      sourceEvidenceIds: ['browser-evidence-live-generic-video-object'],
      classification: genericClassification,
      sourceKind: 'open-graph',
      sourceRef: 'live-generic-open-graph-negative',
      browserTitle: 'OpenGraph-only title',
      openGraphTitle: 'OpenGraph-only title',
      openGraphDescription: 'OpenGraph-only description',
      platformVideoIdOverride: null,
      channelName: null,
      thumbnailUrl: null,
      thumbnailHashRef: null,
      durationSeconds: null,
      publishDate: null,
      captionsAvailable: null,
      transcriptAvailable: null,
      platformCategory: null,
      platformRating: null,
      restrictedSignal: null,
      degradedReasons: [],
    })
  );
  checks.push('generic-open-graph-only-rejected');

  expectThrows('unmanaged-vimeo-exact-url-rejected', () =>
    buildVideoMetadataEvidence({
      metadataEvidenceId: 'metadata-evidence-live-vimeo-unmanaged-negative',
      collectedAt: new Date().toISOString(),
      sourceEvidenceIds: ['browser-evidence-live-vimeo-unmanaged-negative'],
      classification: parseBrowserUrlShape({
        classificationId: 'url-shape-live-vimeo-unmanaged-negative',
        classifiedAt: new Date().toISOString(),
        sourceEvidenceIds: ['browser-evidence-live-vimeo-unmanaged-negative'],
        sourceKind: 'unmanaged-browser-process',
        url: vimeoPageUrl,
        title: 'Ignored unmanaged title',
      }),
      sourceKind: 'platform-page-metadata',
      sourceRef: 'live-vimeo-unmanaged-negative',
      browserTitle: 'Ignored unmanaged title',
      openGraphTitle: 'Ignored unmanaged title',
      openGraphDescription: null,
      platformVideoIdOverride: null,
      channelName: null,
      thumbnailUrl: null,
      thumbnailHashRef: null,
      durationSeconds: null,
      publishDate: null,
      captionsAvailable: null,
      transcriptAvailable: null,
      platformCategory: null,
      platformRating: null,
      restrictedSignal: null,
      degradedReasons: [],
    })
  );
  checks.push('unmanaged-vimeo-exact-url-rejected');
  return checks;
}

function expectThrows(label, fn) {
  try {
    fn();
  } catch {
    return;
  }
  throw new Error(`Expected negative check to throw: ${label}`);
}

function assertNoClaimFlags(evidence) {
  const flags = [
    'pageBodyCaptured',
    'transcriptTextCaptured',
    'contentSemanticsClaimed',
    'aiDecisionClaimed',
    'policyDecisionClaimed',
    'policyAuthorityClaimed',
  ];
  for (const flag of flags) {
    if (evidence[flag] !== false) {
      throw new Error(`Expected ${flag} to remain false`);
    }
  }
}

function sourceSummary(source, html, { expectedHost }) {
  const parsed = new URL(source.url);
  if (parsed.host !== expectedHost) {
    throw new Error(`Expected ${expectedHost}, got ${parsed.host}`);
  }
  return {
    host: parsed.host,
    pathHash: hash(parsed.pathname),
    status: source.status,
    contentType: source.contentType,
    byteLength: Buffer.byteLength(html, 'utf8'),
    hasVideoObjectJsonLd: /VideoObject/iu.test(html) && /application\/ld\+json/iu.test(html),
    hasOpenGraphVideo: /og:video/iu.test(html),
    titleHash: hash(extractTitle(html)),
    titleLength: textLength(extractTitle(html)),
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

function metadataSummary(evidence) {
  return {
    metadataEvidenceId: evidence.metadataEvidenceId,
    sourceKind: evidence.source.sourceKind,
    metadataState: evidence.metadataState,
    platformVideoId: evidence.fields.platformVideoId,
    schemaOrgType: evidence.fields.schemaOrgType,
    titleHash: hash(evidence.fields.openGraphTitle ?? evidence.fields.browserTitle),
    titleLength: textLength(evidence.fields.openGraphTitle ?? evidence.fields.browserTitle),
    descriptionHash: hash(evidence.fields.openGraphDescription),
    descriptionLength: textLength(evidence.fields.openGraphDescription),
    channelNameHash: hash(evidence.fields.channelName),
    channelNameLength: textLength(evidence.fields.channelName),
    thumbnailUrlHost: evidence.fields.thumbnailUrl === null ? null : host(evidence.fields.thumbnailUrl),
    thumbnailHashRefPresent: evidence.fields.thumbnailHashRef !== null,
    durationSeconds: evidence.fields.durationSeconds,
    publishDatePresent: evidence.fields.publishDate !== null,
    degradedReasons: evidence.degradedReasons,
    pageBodyCaptured: evidence.pageBodyCaptured,
    transcriptTextCaptured: evidence.transcriptTextCaptured,
    contentSemanticsClaimed: evidence.contentSemanticsClaimed,
    aiDecisionClaimed: evidence.aiDecisionClaimed,
    policyDecisionClaimed: evidence.policyDecisionClaimed,
    policyAuthorityClaimed: evidence.policyAuthorityClaimed,
    canDriveAiInput: evidence.canDriveAiInput,
  };
}

function videoObjectSummary(videoObject) {
  return {
    type: Array.isArray(videoObject['@type']) ? videoObject['@type'].join(',') : (videoObject['@type'] ?? null),
    nameHash: hash(textOrNull(videoObject.name)),
    nameLength: textLength(textOrNull(videoObject.name)),
    descriptionHash: hash(textOrNull(videoObject.description)),
    descriptionLength: textLength(textOrNull(videoObject.description)),
    durationSeconds: durationSeconds(videoObject.duration),
    uploadDatePresent: timestampOrNull(videoObject.uploadDate) !== null,
    thumbnailHost: firstValue(videoObject.thumbnailUrl) === null ? null : host(firstValue(videoObject.thumbnailUrl)),
    authorHash: hash(authorName(videoObject.author)),
    authorLength: textLength(authorName(videoObject.author)),
  };
}

function textOrNull(value) {
  return typeof value === 'string' && value.trim().length > 0 ? decodeEntities(value.trim()) : null;
}

function urlOrNull(value) {
  const text = textOrNull(value);
  if (text === null) {
    return null;
  }
  return new URL(text).toString();
}

function firstValue(value) {
  if (Array.isArray(value)) {
    return textOrNull(value[0]);
  }
  return textOrNull(value);
}

function timestampOrNull(value) {
  const text = textOrNull(value);
  if (text === null) {
    return null;
  }
  const date = new Date(text);
  return Number.isNaN(date.getTime()) ? null : date.toISOString();
}

function durationSeconds(value) {
  const text = textOrNull(value);
  if (text === null) {
    return null;
  }
  const match = text.match(/^P(?:(\d+)D)?T?(?:(\d+)H)?(?:(\d+)M)?(?:(\d+)S)?$/u);
  if (match === null) {
    return null;
  }
  const [, days, hours, minutes, seconds] = match;
  return Number(days ?? 0) * 86400 + Number(hours ?? 0) * 3600 + Number(minutes ?? 0) * 60 + Number(seconds ?? 0);
}

function authorName(value) {
  if (Array.isArray(value)) {
    return authorName(value[0]);
  }
  if (value !== null && typeof value === 'object') {
    return textOrNull(value.name);
  }
  return textOrNull(value);
}

function hashRef(prefix, value) {
  const text = textOrNull(value);
  return text === null ? null : `${prefix}-${hash(text).slice(0, 16)}`;
}

function hash(value) {
  if (value === null || value === undefined) {
    return null;
  }
  return createHash('sha256').update(String(value)).digest('hex');
}

function textLength(value) {
  return value === null || value === undefined ? 0 : String(value).length;
}

function host(url) {
  return new URL(url).host;
}

function decodeEntities(value) {
  if (value === null) {
    return null;
  }
  return value
    .replace(/&amp;/giu, '&')
    .replace(/&quot;/giu, '"')
    .replace(/&#39;/giu, "'")
    .replace(/&lt;/giu, '<')
    .replace(/&gt;/giu, '>');
}

function writeJson(path, value) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
