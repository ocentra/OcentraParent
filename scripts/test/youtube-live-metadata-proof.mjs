import { createHash } from 'node:crypto';
import { existsSync } from 'node:fs';
import { mkdir, stat, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const root = process.cwd();
const sourceUrl = 'https://www.youtube.com/watch?v=dQw4w9WgXcQ';
const sourceVideoId = 'dQw4w9WgXcQ';
const oembedUrl = `https://www.youtube.com/oembed?url=${encodeURIComponent(sourceUrl)}&format=json`;
const userAgent = 'OcentraParentLiveProof/1.0 (+https://ocentra.local/browser-plan)';
const resultDirectory = join(root, 'test-results', 'youtube-live-metadata-proof');
const proofDirectory = join(root, 'output', 'browser-plan-proof', 'ai-21-youtube-parser-metadata-adapter');
const resultProofPath = join(resultDirectory, 'proof.json');
const browserPlanProofPath = join(proofDirectory, '11-live-youtube-metadata-proof.json');

await main();

async function main() {
  await assertProofBuildsAreFresh();
  const { parseBrowserUrlShape } = await importDistModule('packages/browser-domain/dist/browser-url-intelligence.js');
  const { buildYouTubeMetadataEvidence } = await importDistModule(
    'packages/schema-domain/dist/browser-youtube-metadata.js'
  );

  await mkdir(resultDirectory, { recursive: true });
  await mkdir(proofDirectory, { recursive: true });

  const collectedAt = new Date().toISOString();
  const [oembedCapture, pageCapture] = await Promise.all([fetchYouTubeOEmbed(), fetchYouTubeWatchPage()]);
  const pageTitle = pageCapture.metadata.openGraphTitle ?? oembedCapture.payload.title;
  const classification = parseBrowserUrlShape({
    classificationId: `url-shape-youtube-live-${sourceVideoId}`,
    classifiedAt: collectedAt,
    sourceEvidenceIds: [`browser-evidence-youtube-live-${sourceVideoId}`],
    sourceKind: 'managed-browser-exact-url',
    url: sourceUrl,
    title: pageTitle,
  });
  const metadataEvidence = buildYouTubeMetadataEvidence({
    metadataEvidenceId: `metadata-evidence-youtube-live-${sourceVideoId}`,
    collectedAt,
    sourceEvidenceIds: [`browser-evidence-youtube-live-${sourceVideoId}`],
    classification,
    sourceRef: 'youtube-live-oembed-watch-page-metadata',
    browserTitle: pageTitle,
    openGraphTitle: pageCapture.metadata.openGraphTitle ?? oembedCapture.payload.title,
    openGraphDescription: pageCapture.metadata.openGraphDescription,
    channelName: oembedCapture.payload.authorName,
    thumbnailUrl: oembedCapture.payload.thumbnailUrl,
    thumbnailHashRef: redactedText(oembedCapture.payload.thumbnailUrl).sha256,
    durationSeconds: null,
    publishDate: null,
    captionsAvailable: null,
    transcriptAvailable: null,
    platformCategory: null,
    platformRating: null,
    restrictedSignal: null,
    degradedReasons: [],
  });
  const proof = proofFor({ collectedAt, oembedCapture, pageCapture, classification, metadataEvidence });

  await writeFile(resultProofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(browserPlanProofPath, `${JSON.stringify(proof, null, 2)}\n`);

  console.log('youtube-live-metadata-proof-ok=true');
  console.log(`proof=${relativePath(resultProofPath)}`);
  console.log(`browserPlanProof=${relativePath(browserPlanProofPath)}`);
  console.log(`videoId=${proof.classification.platformIds.videoId}`);
  console.log(`metadataState=${proof.metadataEvidence.metadataState}`);
}

async function assertProofBuildsAreFresh() {
  const pairs = [
    [
      'packages/browser-domain/src/browser-url-intelligence.ts',
      'packages/browser-domain/dist/browser-url-intelligence.js',
    ],
    [
      'packages/schema-domain/src/browser-youtube-metadata.ts',
      'packages/schema-domain/dist/browser-youtube-metadata.js',
    ],
  ];
  for (const [sourcePath, distPath] of pairs) {
    const sourceFullPath = join(root, sourcePath);
    const distFullPath = join(root, distPath);
    if (!existsSync(distFullPath)) {
      throw new Error(`Missing built proof dependency ${distPath}; run cmd /c npm run build:contracts first`);
    }
    const [sourceInfo, distInfo] = await Promise.all([stat(sourceFullPath), stat(distFullPath)]);
    if (distInfo.mtimeMs < sourceInfo.mtimeMs) {
      throw new Error(`Stale built proof dependency ${distPath}; run cmd /c npm run build:contracts first`);
    }
  }
}

async function importDistModule(relativePath) {
  const modulePath = join(root, relativePath);
  return import(pathToFileURL(modulePath).href);
}

async function fetchYouTubeOEmbed() {
  const response = await fetchWithTimeout(oembedUrl, {
    accept: 'application/json',
    timeoutMs: 30000,
  });
  const text = await response.text();
  const payload = parseOEmbedPayload(text);
  return {
    sourceKind: 'youtube-oembed-live',
    url: oembedUrl,
    status: response.status,
    ok: response.ok,
    contentType: response.headers.get('content-type'),
    responseBytes: Buffer.byteLength(text),
    payload,
  };
}

async function fetchYouTubeWatchPage() {
  const response = await fetchWithTimeout(sourceUrl, {
    accept: 'text/html,application/xhtml+xml',
    timeoutMs: 30000,
  });
  const html = await response.text();
  const metadata = {
    openGraphTitle: metaContent(html, 'og:title'),
    openGraphDescription: metaContent(html, 'og:description'),
    openGraphImage: metaContent(html, 'og:image'),
    twitterTitle: metaContent(html, 'twitter:title'),
    itempropDuration: metaContent(html, 'duration'),
    itempropDatePublished: metaContent(html, 'datePublished'),
  };
  const markers = {
    openGraphTitle: metadata.openGraphTitle !== null,
    openGraphDescription: metadata.openGraphDescription !== null,
    openGraphImage: metadata.openGraphImage !== null,
    twitterTitle: metadata.twitterTitle !== null,
    ytInitialPlayerResponse: html.includes('ytInitialPlayerResponse'),
    watchEndpoint: html.includes('watchEndpoint'),
  };
  if (!response.ok || !markers.openGraphTitle) {
    throw new Error(
      `YouTube watch page metadata capture failed: status=${response.status} ogTitle=${markers.openGraphTitle}`
    );
  }
  return {
    sourceKind: 'youtube-watch-page-live',
    url: sourceUrl,
    status: response.status,
    ok: response.ok,
    contentType: response.headers.get('content-type'),
    responseBytes: Buffer.byteLength(html),
    metadata,
    markers,
  };
}

async function fetchWithTimeout(url, options) {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), options.timeoutMs);
  timeout.unref?.();
  try {
    return await fetch(url, {
      headers: {
        accept: options.accept,
        'user-agent': userAgent,
      },
      signal: controller.signal,
    });
  } finally {
    clearTimeout(timeout);
  }
}

function parseOEmbedPayload(text) {
  const parsed = JSON.parse(text);
  const payload = {
    title: requiredString(parsed.title, 'title'),
    authorName: requiredString(parsed.author_name, 'author_name'),
    thumbnailUrl: requiredString(parsed.thumbnail_url, 'thumbnail_url'),
    providerName: requiredString(parsed.provider_name, 'provider_name'),
    type: requiredString(parsed.type, 'type'),
    width: requiredNumber(parsed.width, 'width'),
    height: requiredNumber(parsed.height, 'height'),
  };
  if (payload.providerName !== 'YouTube' || payload.type !== 'video') {
    throw new Error(`Unexpected YouTube oEmbed provider/type: ${payload.providerName}/${payload.type}`);
  }
  return payload;
}

function proofFor({ collectedAt, oembedCapture, pageCapture, classification, metadataEvidence }) {
  return {
    schemaVersion: 1,
    proofMode: 'youtube-live-metadata-proof',
    generatedAt: collectedAt,
    sourceSurface: {
      platform: 'youtube',
      liveWatchUrl: sourceUrl,
      liveOEmbedUrl: oembedUrl,
      publicVideoId: sourceVideoId,
      generatedFixture: false,
      localFixture: false,
    },
    liveCapture: {
      oembed: {
        sourceKind: oembedCapture.sourceKind,
        status: oembedCapture.status,
        ok: oembedCapture.ok,
        contentType: oembedCapture.contentType,
        responseBytes: oembedCapture.responseBytes,
        title: redactedText(oembedCapture.payload.title),
        authorName: redactedText(oembedCapture.payload.authorName),
        thumbnailUrl: redactedUrl(oembedCapture.payload.thumbnailUrl),
        providerName: oembedCapture.payload.providerName,
        type: oembedCapture.payload.type,
        dimensions: {
          width: oembedCapture.payload.width,
          height: oembedCapture.payload.height,
        },
      },
      watchPage: {
        sourceKind: pageCapture.sourceKind,
        status: pageCapture.status,
        ok: pageCapture.ok,
        contentType: pageCapture.contentType,
        responseBytes: pageCapture.responseBytes,
        markers: pageCapture.markers,
        openGraphTitle: redactedText(pageCapture.metadata.openGraphTitle),
        openGraphDescription: redactedText(pageCapture.metadata.openGraphDescription),
        openGraphImage: redactedUrl(pageCapture.metadata.openGraphImage),
        twitterTitle: redactedText(pageCapture.metadata.twitterTitle),
        itempropDuration: redactedText(pageCapture.metadata.itempropDuration),
        itempropDatePublished: redactedText(pageCapture.metadata.itempropDatePublished),
      },
    },
    classification: {
      classificationId: classification.classificationId,
      sourceKind: classification.sourceKind,
      exactUrlEvidence: classification.exactUrlEvidence,
      targetKind: classification.targetKind,
      platform: classification.platform,
      platformIds: {
        videoId: classification.platformIds.videoId,
        channelId: classification.platformIds.channelId,
        playlistId: classification.platformIds.playlistId,
      },
      confidence: classification.confidence,
      reasonCodes: classification.reasonCodes,
      contentSemanticsClaimed: classification.contentSemanticsClaimed,
      aiDecisionClaimed: classification.aiDecisionClaimed,
      policyDecisionClaimed: classification.policyDecisionClaimed,
    },
    metadataEvidence: {
      metadataEvidenceId: metadataEvidence.metadataEvidenceId,
      metadataState: metadataEvidence.metadataState,
      sourceKind: metadataEvidence.source.sourceKind,
      sourceRef: metadataEvidence.source.sourceRef,
      urlShapeClassificationId: metadataEvidence.urlShapeClassificationId,
      fieldsPresent: fieldsPresent(metadataEvidence.fields),
      fieldHashes: {
        browserTitle: redactedText(metadataEvidence.fields.browserTitle),
        openGraphTitle: redactedText(metadataEvidence.fields.openGraphTitle),
        openGraphDescription: redactedText(metadataEvidence.fields.openGraphDescription),
        channelName: redactedText(metadataEvidence.fields.channelName),
        thumbnailUrl: redactedUrl(metadataEvidence.fields.thumbnailUrl),
      },
      platformVideoId: metadataEvidence.fields.platformVideoId,
      schemaOrgType: metadataEvidence.fields.schemaOrgType,
      degradedReasons: metadataEvidence.degradedReasons,
      canDriveAiInput: metadataEvidence.canDriveAiInput,
      noClaimFlags: {
        pageBodyCaptured: metadataEvidence.pageBodyCaptured,
        transcriptTextCaptured: metadataEvidence.transcriptTextCaptured,
        contentSemanticsClaimed: metadataEvidence.contentSemanticsClaimed,
        aiDecisionClaimed: metadataEvidence.aiDecisionClaimed,
        policyDecisionClaimed: metadataEvidence.policyDecisionClaimed,
        policyAuthorityClaimed: metadataEvidence.policyAuthorityClaimed,
      },
    },
    persistedBoundary: {
      rawWatchPageHtmlPersisted: false,
      rawPageBodyPersisted: false,
      rawTranscriptTextPersisted: false,
      rawTitleOrDescriptionPersistedInProof: false,
      cookiesTokensLocalStorageCaptured: false,
      generatedOrFabricatedProofPageUsed: false,
      liveSurfaceRequired: true,
    },
  };
}

function fieldsPresent(fields) {
  return Object.fromEntries(Object.entries(fields).map(([key, value]) => [key, value !== null]));
}

function metaContent(html, key) {
  const tags = html.match(/<meta\b[^>]*>/gi) ?? [];
  for (const tag of tags) {
    const property = attributeValue(tag, 'property');
    const name = attributeValue(tag, 'name');
    const itemprop = attributeValue(tag, 'itemprop');
    if (property === key || name === key || itemprop === key) {
      return decodeHtml(attributeValue(tag, 'content'));
    }
  }
  return null;
}

function attributeValue(tag, attributeName) {
  const quoted = new RegExp(`${attributeName}\\s*=\\s*["']([^"']*)["']`, 'i').exec(tag);
  return quoted?.[1] ?? null;
}

function decodeHtml(value) {
  if (value === null) {
    return null;
  }
  return value
    .replaceAll('&quot;', '"')
    .replaceAll('&#39;', "'")
    .replaceAll('&lt;', '<')
    .replaceAll('&gt;', '>')
    .replaceAll('&amp;', '&');
}

function redactedText(value) {
  if (typeof value !== 'string' || value.length === 0) {
    return {
      present: false,
      length: 0,
      sha256: null,
    };
  }
  return {
    present: true,
    length: value.length,
    sha256: `sha256:${sha256(value)}`,
  };
}

function redactedUrl(value) {
  if (typeof value !== 'string' || value.length === 0) {
    return {
      present: false,
      host: null,
      length: 0,
      sha256: null,
    };
  }
  return {
    present: true,
    host: urlHost(value),
    length: value.length,
    sha256: `sha256:${sha256(value)}`,
  };
}

function urlHost(value) {
  try {
    return new URL(value).host;
  } catch {
    return null;
  }
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function requiredString(value, fieldName) {
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`Expected non-empty YouTube oEmbed ${fieldName}`);
  }
  return value;
}

function requiredNumber(value, fieldName) {
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    throw new Error(`Expected numeric YouTube oEmbed ${fieldName}`);
  }
  return value;
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
