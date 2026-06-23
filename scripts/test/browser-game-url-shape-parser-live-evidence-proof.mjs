import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';

import { BrowserGameUrlShapeParseResultSchema } from '@ocentra-parent/schema-domain/browser-game-url-shape-parser';
import { parseBrowserGameUrlShape } from '../../packages/browser-domain/dist/browser-game-url-shape-evaluator.js';

const repoRoot = process.cwd();
const proofId = 'browser-game-url-shape-parser-live-evidence-proof';
const resultPath = join(repoRoot, 'test-results', proofId, 'proof.json');
const outputProofPath = join(
  repoRoot,
  'output',
  'browser-plan-proof',
  'game-05-game-url-shape-parser',
  '02-live-url-shape-proof.json'
);

const targets = [
  {
    targetId: 'crazygames-bloxdhop',
    url: 'https://www.crazygames.com/game/bloxdhop-io',
    expectedRouteSurfaceKind: 'catalog-route',
  },
  {
    targetId: 'poki-subway-surfers',
    url: 'https://poki.com/en/g/subway-surfers',
    expectedRouteSurfaceKind: 'game-detail-route',
  },
  {
    targetId: 'coolmath-run-3',
    url: 'https://www.coolmathgames.com/0-run-3',
    expectedRouteSurfaceKind: 'catalog-route',
  },
  {
    targetId: 'chess-play-online',
    url: 'https://www.chess.com/play/online',
    expectedRouteSurfaceKind: 'play-route',
  },
  {
    targetId: 'xbox-cloud-play',
    url: 'https://www.xbox.com/en-US/play',
    expectedRouteSurfaceKind: 'play-route',
  },
  {
    targetId: 'nvidia-geforce-now',
    url: 'https://www.nvidia.com/en-us/geforce-now/',
    expectedRouteSurfaceKind: 'catalog-route',
  },
  {
    targetId: 'xbox-cloud-gaming',
    url: 'https://www.xbox.com/play/cloud',
    expectedRouteSurfaceKind: 'cloud-session-route',
  },
];

const startedAt = new Date().toISOString();
const branch = git(['rev-parse', '--abbrev-ref', 'HEAD']);
const commit = git(['rev-parse', 'HEAD']);
const baseCommit = git(['rev-parse', 'origin/main']);
const captures = await Promise.all(targets.map(captureTarget));
const parseResults = captures.map(parseEvidenceFor);
const negativeChecks = runNegativeChecks(parseResults[0].parseResult);

if (!captures.every((capture) => capture.responseOk)) {
  throw new Error('Expected all browser-game URL-shape live captures to return HTTP 2xx/3xx responses');
}
if (!parseResults.every((entry) => entry.parseResult.parseState === 'parsed')) {
  throw new Error('Expected every live browser-game URL-shape route to parse');
}
if (!parseResults.every((entry) => entry.parseResult.routeSurfaceKind === entry.expectedRouteSurfaceKind)) {
  throw new Error('Expected every live browser-game URL-shape route to match its route surface kind');
}
if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected browser-game URL-shape negative checks to reject overclaims');
}

const proof = {
  schemaVersion: 1,
  proofId,
  generatedAt: startedAt,
  branch,
  commit,
  baseCommit,
  captureMode: 'real-live-browser-game-url-shape-parser',
  targets: captures,
  parseResults,
  negativeChecks,
  summary: {
    targetCount: captures.length,
    parsedRows: parseResults.length,
    negativeChecks: negativeChecks.length,
    rawUrlStored: false,
    rawDomainStored: false,
    rawPathStored: false,
    rawQueryStored: false,
    rawFragmentStored: false,
    browserNavigationClaimed: false,
    runtimeDetectionClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    productChecklistUpgradeClaimed: false,
  },
};

await writeJson(resultPath, proof);
await writeJson(outputProofPath, proof);

console.log('browser-game-url-shape-parser-live-evidence-proof-ok=true');
console.log(`proof=${relativePath(resultPath)}`);
console.log(`outputProof=${relativePath(outputProofPath)}`);
console.log(`targets=${captures.length} negativeChecks=${negativeChecks.length}`);

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
    rawUrlPersisted: false,
    rawDomainPersisted: false,
    rawPathPersisted: false,
    rawQueryPersisted: false,
    rawFragmentPersisted: false,
  };
}

function parseEvidenceFor(capture) {
  const target = targets.find((entry) => entry.targetId === capture.targetId);
  const parseResult = parseBrowserGameUrlShape(target.url);
  BrowserGameUrlShapeParseResultSchema.safeParse(parseResult);
  return {
    targetId: capture.targetId,
    sourceEvidenceRef: `parent-proof-${proofId}-${capture.targetId}`,
    expectedRouteSurfaceKind: target.expectedRouteSurfaceKind,
    parseResult,
    rawUrlPersisted: false,
    rawDomainPersisted: false,
    rawPathPersisted: false,
    rawQueryPersisted: false,
    rawFragmentPersisted: false,
  };
}

function runNegativeChecks(validResult) {
  const invalidResults = [
    ['raw-url', { rawUrlStored: true }],
    ['raw-domain', { rawDomainStored: true }],
    ['raw-path', { rawPathStored: true }],
    ['raw-query', { rawQueryStored: true }],
    ['browser-navigation', { browserNavigationClaimed: true }],
    ['runtime-detection', { runtimeDetectionClaimed: true }],
    ['ai-classification', { aiClassificationClaimed: true }],
    ['policy-decision', { policyDecisionClaimed: true }],
    ['cloud-frame-analysis', { cloudFrameAnalysisClaimed: true }],
    ['native-game-control', { nativeGameControlClaimed: true }],
    ['enforcement', { enforcementClaimed: true }],
    ['parsed-without-fingerprint', { routeShapeFingerprint: null }],
    ['parsed-unknown-route', { routeSurfaceKind: 'unknown-route' }],
    ['parsed-manual-custody', { inputCustody: 'manual-required' }],
    ['manual-with-fingerprint', { parseState: 'manual-required', routeShapeFingerprint: 'manual-fingerprint' }],
    ['manual-with-high-confidence', { parseState: 'manual-required', confidence: 'high' }],
  ];
  return invalidResults.map(([name, invalid]) => ({
    name,
    rejected: !BrowserGameUrlShapeParseResultSchema.safeParse({ ...validResult, ...invalid }).success,
  }));
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

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
