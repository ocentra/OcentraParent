import { spawn } from 'node:child_process';
import { createCipheriv, createHash, randomBytes } from 'node:crypto';
import { existsSync } from 'node:fs';
import { mkdir, mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { chromium } from 'playwright';

const proofRoot = join(process.cwd(), 'output', 'screen-plan-proof', '33-managed-browser-cdp-screenshot-capture-path');
const proofSummaryPath = join(proofRoot, 'proof-summary.json');
const validationLogPath = join(proofRoot, '14-validation-commands.log');
const liveUrlCandidates = [
  process.env.OCENTRA_SCREEN_CDP_LIVE_URL,
  'https://example.com/',
  'https://1.1.1.1/',
  'http://1.1.1.1/',
].filter(Boolean);
const successfulCommands = [];

await runPackageCommand([
  'exec',
  '--workspace',
  '@ocentra-parent/activity-domain',
  '--',
  'vitest',
  'run',
  'tests/screen-managed-browser-cdp-capture.test.ts',
]);
await runPackageCommand(['run', 'build', '--workspace', '@ocentra-parent/activity-domain']);
await runManagedBrowserCdpProof();

console.log(`screen-managed-browser-cdp-capture-proof-ok:${proofSummaryPath}`);

async function runManagedBrowserCdpProof() {
  const {
    ScreenAnalysisQueueJobSchema,
    ScreenManagedBrowserCdpCaptureSchemaVersion,
    ScreenManagedBrowserCdpMaxPixels,
    ScreenManagedBrowserCdpScreenshotArtifactSchema,
    ScreenManagedBrowserCdpScreenshotRequestSchema,
  } = await import('@ocentra-parent/screen-domain/screen-evidence');

  const browser = await chromium.launch({
    headless: true,
    args: ['--ignore-certificate-errors'],
  });
  const captures = [];
  const queueProofs = [];
  const deletionProofs = [];
  let readiness = null;

  try {
    const context = await browser.newContext({
      ignoreHTTPSErrors: true,
      viewport: {
        width: 1280,
        height: 720,
      },
      deviceScaleFactor: 1,
    });
    const { page, loadedUrl } = await openFirstReachableLivePage(context, liveUrlCandidates);
    await page.waitForLoadState('networkidle', { timeout: 10_000 }).catch(() => undefined);

    readiness = await buildPageReadiness(page, loadedUrl);
    const targetId = await findManagedBrowserTargetId(browser, page);
    const cdp = await context.newCDPSession(page);
    await cdp.send('Page.enable');
    const layoutMetrics = await cdp.send('Page.getLayoutMetrics');

    for (const plan of buildCapturePlans(layoutMetrics)) {
      const request = ScreenManagedBrowserCdpScreenshotRequestSchema.parse(
        buildRequest({
          plan,
          targetId,
          readiness,
          schemaVersion: ScreenManagedBrowserCdpCaptureSchemaVersion,
          maxPixelCount: ScreenManagedBrowserCdpMaxPixels,
        })
      );
      const cdpResult = await cdp.send('Page.captureScreenshot', cdpParamsFor(request));
      const pngBytes = Buffer.from(cdpResult.data, 'base64');
      const pngInfo = parsePngInfo(pngBytes);
      const imageDigest = `sha256:${sha256(pngBytes)}`;
      const queue = await encryptThroughTempQueue(plan.captureMode, pngBytes);
      const now = new Date().toISOString();
      const artifact = ScreenManagedBrowserCdpScreenshotArtifactSchema.parse({
        schemaVersion: ScreenManagedBrowserCdpCaptureSchemaVersion,
        captureId: `managed-browser-cdp-${plan.captureMode}-capture`,
        requestId: request.requestId,
        capturedAt: now,
        targetId,
        cdpMethod: 'Page.captureScreenshot',
        captureMode: plan.captureMode,
        fromSurface: true,
        captureBeyondViewport: request.captureBeyondViewport,
        captureScope: 'managedBrowserWindow',
        imageWidth: pngInfo.width,
        imageHeight: pngInfo.height,
        imagePixelCount: pngInfo.width * pngInfo.height,
        imageByteSize: pngBytes.byteLength,
        imageFormat: 'png',
        imageDigest,
        urlEvidenceRef: request.urlEvidenceRef,
        titleEvidenceRef: request.titleEvidenceRef,
        screenshotEvidenceRef: evidenceRef(`managed-browser-cdp-${plan.captureMode}-screenshot`, imageDigest),
        queueJobId: `managed-browser-cdp-${plan.captureMode}-queue`,
        encryptedImageRef: `managed-browser-cdp-${plan.captureMode}-encrypted-temp-image`,
        custodyState: 'child-device-temp-queue',
        deletionRequired: true,
        deletionStatus: 'deleted',
        deletionProofRef: `managed-browser-cdp-${plan.captureMode}-delete-proof`,
        rawTempPathRedacted: true,
        rawImageRetained: false,
        liveScreencastStarted: false,
        desktopCaptureAttempted: false,
        remoteUploadAllowed: false,
      });
      const queueJob = ScreenAnalysisQueueJobSchema.parse(buildQueueJob({ artifact, request, now }));
      captures.push(redactCapture({ request, artifact, queueJob, plan }));
      queueProofs.push(queue);
      deletionProofs.push({
        captureMode: plan.captureMode,
        rawTempDeleted: queue.rawTempDeleted,
        encryptedTempDeleted: queue.encryptedTempDeleted,
        rawImageRetained: false,
        deletionProofRef: artifact.deletionProofRef,
      });
    }

    await cdp.detach();
    await context.close();
  } finally {
    await browser.close();
  }

  await writeProofFiles({ captures, queueProofs, deletionProofs, readiness });
}

async function writeProofFiles({ captures, queueProofs, deletionProofs, readiness }) {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(join(proofRoot, '10-ui-snapshots'), { recursive: true });
  await writeFile(
    join(proofRoot, '00-source-snapshot.md'),
    [
      '# WP33 Managed Browser CDP Capture Source Snapshot',
      '',
      `- Live URL host: ${readiness.host}`,
      `- Final URL hash: ${readiness.finalUrlHash}`,
      `- Title hash: ${readiness.titleHash}`,
      '- CDP method reference: https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-captureScreenshot',
      '- CDP target reference: https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-getTargets',
      '- Playwright CDP session was used only to drive the managed page target.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(proofRoot, '03-cdp-capture-proof.json'), { readiness, captures });
  await writeJson(join(proofRoot, '05-queue-encryption-proof.json'), {
    algorithm: 'aes-256-gcm',
    queueProofs,
    keyMaterialRetained: false,
  });
  await writeJson(join(proofRoot, '08-deletion-proof.json'), {
    deletionProofs,
    allRawDeleted: deletionProofs.every((proof) => proof.rawTempDeleted && !proof.rawImageRetained),
    allEncryptedTempDeleted: deletionProofs.every((proof) => proof.encryptedTempDeleted),
  });
  await writeFile(
    join(proofRoot, '10-ui-snapshots', 'README.md'),
    [
      '# Snapshot Boundary',
      '',
      'This WP33 proof captures real page pixels through CDP, but retained artifacts do not store raw screenshot images.',
      'The inspectable proof is the redacted capture metadata, dimensions, hashes, queue proof, and deletion proof.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(validationLogPath, `${successfulCommands.join('\n')}\n`, 'utf8');
  await writeJson(proofSummaryPath, {
    proofGeneratedAt: new Date().toISOString(),
    proofTopic: 'screen-managed-browser-cdp-screenshot-capture-path',
    workpack: 'docs/plans/screen-plan/workpacks/33-managed-browser-cdp-screenshot-capture-path.md',
    sourceDocs: [
      'https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-captureScreenshot',
      'https://chromedevtools.github.io/devtools-protocol/tot/Target/#method-getTargets',
    ],
    livePageReadiness: readiness,
    claimsProven: [
      'Chromium CDP Page.captureScreenshot captures real page pixels for page, viewport, and crop modes',
      'each capture is tied to a managed browser page target id and URL/title evidence refs',
      'capture requests reject desktop capture, full-screen display capture, live screencast, raw retention, and remote upload defaults',
      'captured image bytes are encrypted through a temporary queue handoff and then deleted',
      'retained proof artifacts contain hashes, dimensions, custody, queue, and deletion state instead of raw screenshots',
    ],
    nonClaims: [
      'this proof does not claim managed-browser production URL-trigger ownership',
      'this proof does not claim OCR/VLM quality, policy action, enforcement, live view, or raw screenshot retention mode',
      'this proof uses a public live page for CDP path proof; final product-complete matrix still needs the full operator/live scenario set',
    ],
    validationCommands: successfulCommands,
    captureSummary: {
      totalModes: captures.length,
      modes: captures.map((capture) => capture.captureMode),
      allManagedBrowserWindow: captures.every((capture) => capture.captureScope === 'managedBrowserWindow'),
      allDeleted: deletionProofs.every((proof) => proof.rawTempDeleted && proof.encryptedTempDeleted),
      anyDesktopCapture: captures.some((capture) => capture.desktopCaptureAttempted),
      anyLiveScreencast: captures.some((capture) => capture.liveScreencastStarted),
      anyRemoteUpload: captures.some((capture) => capture.remoteUploadAllowed),
    },
  });
}

async function openFirstReachableLivePage(context, candidates) {
  const failures = [];
  for (const candidate of candidates) {
    const page = await context.newPage();
    try {
      await page.goto(candidate, { waitUntil: 'domcontentloaded', timeout: 45_000 });
      await page.waitForLoadState('networkidle', { timeout: 10_000 }).catch(() => undefined);
      const currentUrl = page.url();
      if (currentUrl.startsWith('chrome-error://')) {
        throw new Error(`Chromium loaded an error page for ${candidate}`);
      }
      return {
        loadedUrl: candidate,
        page,
      };
    } catch (error) {
      failures.push(`${candidate}: ${error.message}`);
      await page.close().catch(() => undefined);
    }
  }
  throw new Error(`No live URL candidate loaded for managed-browser CDP proof\n${failures.join('\n')}`);
}

async function buildPageReadiness(page, loadedUrl) {
  const finalUrl = page.url();
  const host = new URL(finalUrl).hostname;
  const title = await page.title();
  const visibleText = await page.evaluate(() => document.body?.innerText ?? '');
  if (visibleText.trim().length < 10) {
    throw new Error('Managed-browser CDP proof refuses blank or near-blank live pages');
  }
  return {
    host,
    requestedLiveUrlHash: `sha256:${sha256(loadedUrl)}`,
    finalUrlHash: `sha256:${sha256(finalUrl)}`,
    finalUrlLength: finalUrl.length,
    titleHash: `sha256:${sha256(title)}`,
    titleLength: title.length,
    visibleTextHash: `sha256:${sha256(visibleText)}`,
    visibleTextLength: visibleText.length,
  };
}

async function findManagedBrowserTargetId(browser, page) {
  const session = await browser.newBrowserCDPSession();
  try {
    const targets = await session.send('Target.getTargets');
    const finalUrl = page.url();
    const title = await page.title();
    const target =
      targets.targetInfos.find((info) => info.type === 'page' && info.url === finalUrl) ??
      targets.targetInfos.find((info) => info.type === 'page' && info.title === title);
    if (!target?.targetId) {
      throw new Error('Unable to locate managed browser page target id through CDP Target.getTargets');
    }
    return target.targetId;
  } finally {
    await session.detach();
  }
}

function buildCapturePlans(layoutMetrics) {
  const viewportWidth = Math.max(1, Math.floor(layoutMetrics.cssVisualViewport?.clientWidth ?? 1280));
  const viewportHeight = Math.max(1, Math.floor(layoutMetrics.cssVisualViewport?.clientHeight ?? 720));
  const contentWidth = Math.min(1280, Math.max(1, Math.ceil(layoutMetrics.cssContentSize?.width ?? viewportWidth)));
  const contentHeight = Math.min(
    Math.floor(4_000_000 / contentWidth),
    4096,
    Math.max(1, Math.ceil(layoutMetrics.cssContentSize?.height ?? viewportHeight))
  );
  return [
    {
      captureMode: 'page',
      captureBeyondViewport: true,
      clip: {
        x: 0,
        y: 0,
        width: contentWidth,
        height: contentHeight,
        scale: 1,
      },
      estimatedPixelCount: contentWidth * contentHeight,
      viewport: {
        width: viewportWidth,
        height: viewportHeight,
        deviceScaleFactor: 1,
      },
    },
    {
      captureMode: 'viewport',
      captureBeyondViewport: false,
      clip: null,
      estimatedPixelCount: viewportWidth * viewportHeight,
      viewport: {
        width: viewportWidth,
        height: viewportHeight,
        deviceScaleFactor: 1,
      },
    },
    {
      captureMode: 'crop',
      captureBeyondViewport: false,
      clip: {
        x: 0,
        y: 0,
        width: Math.min(480, viewportWidth),
        height: Math.min(320, viewportHeight),
        scale: 1,
      },
      estimatedPixelCount: Math.min(480, viewportWidth) * Math.min(320, viewportHeight),
      viewport: {
        width: viewportWidth,
        height: viewportHeight,
        deviceScaleFactor: 1,
      },
    },
  ];
}

function buildRequest({ plan, targetId, readiness, schemaVersion, maxPixelCount }) {
  const urlEvidenceRef = evidenceRef('managed-browser-cdp-url-ref', readiness.finalUrlHash);
  const titleEvidenceRef = evidenceRef('managed-browser-cdp-title-ref', readiness.titleHash);
  return {
    schemaVersion,
    requestId: `managed-browser-cdp-${plan.captureMode}-request`,
    requestedAt: new Date().toISOString(),
    deviceRef: 'windows-child-device',
    targetId,
    targetType: 'page',
    captureMode: plan.captureMode,
    captureReason: 'managedBrowserUrlChange',
    captureScope: 'managedBrowserWindow',
    cdpMethod: 'Page.captureScreenshot',
    fromSurface: true,
    captureBeyondViewport: plan.captureBeyondViewport,
    viewport: plan.viewport,
    clip: plan.clip,
    estimatedPixelCount: plan.estimatedPixelCount,
    maxPixelCount,
    urlEvidenceRef,
    titleEvidenceRef,
    relatedEvidenceRefs: [urlEvidenceRef, titleEvidenceRef],
    parentAllowsManagedBrowserCapture: true,
    parentAllowsLiveScreencast: false,
    allowDesktopCapture: false,
    allowFullScreenCapture: false,
    rawScreenshotRetentionAllowed: false,
    remoteUploadAllowed: false,
  };
}

function cdpParamsFor(request) {
  const params = {
    format: 'png',
    fromSurface: request.fromSurface,
    captureBeyondViewport: request.captureBeyondViewport,
  };
  if (request.clip !== null) {
    params.clip = request.clip;
  }
  return params;
}

function buildQueueJob({ artifact, request, now }) {
  const expiresAt = new Date(Date.parse(now) + 300_000).toISOString();
  return {
    schemaVersion: 1,
    queueJobId: artifact.queueJobId,
    createdAt: now,
    notBefore: now,
    expiresAt,
    lastAttemptAt: now,
    captureReason: request.captureReason,
    captureScope: request.captureScope,
    sourceId: 'managed-browser-cdp-target-source',
    adapterId: 'managed-browser-cdp',
    deviceRef: request.deviceRef,
    localUserRef: 'local-child-user',
    parentSettingRef: 'screen-summary-managed-browser-cdp-proof-setting',
    settingVersion: 1,
    relatedEvidenceRefs: request.relatedEvidenceRefs,
    encryptedImageRef: artifact.encryptedImageRef,
    imageDigest: artifact.imageDigest,
    imageByteSize: artifact.imageByteSize,
    imageFormat: artifact.imageFormat,
    status: 'deleted',
    attemptCount: 1,
    maxRetryCount: 1,
    failureReason: null,
    unavailableReason: null,
    deletionRequired: true,
    deletedAt: now,
    deletionStatus: 'deleted',
    deletionProofRef: artifact.deletionProofRef,
    custodyState: 'child-device-temp-queue',
  };
}

function redactCapture({ request, artifact, queueJob, plan }) {
  return {
    captureMode: plan.captureMode,
    requestId: request.requestId,
    captureId: artifact.captureId,
    targetIdHash: `sha256:${sha256(request.targetId)}`,
    cdpMethod: artifact.cdpMethod,
    captureScope: artifact.captureScope,
    captureBeyondViewport: request.captureBeyondViewport,
    clip: request.clip,
    viewport: request.viewport,
    imageWidth: artifact.imageWidth,
    imageHeight: artifact.imageHeight,
    imagePixelCount: artifact.imagePixelCount,
    imageByteSize: artifact.imageByteSize,
    imageDigest: artifact.imageDigest,
    queueJobId: queueJob.queueJobId,
    custodyState: queueJob.custodyState,
    deletionStatus: queueJob.deletionStatus,
    rawImageRetained: artifact.rawImageRetained,
    liveScreencastStarted: artifact.liveScreencastStarted,
    desktopCaptureAttempted: artifact.desktopCaptureAttempted,
    remoteUploadAllowed: artifact.remoteUploadAllowed,
  };
}

async function encryptThroughTempQueue(captureMode, pngBytes) {
  const tempRoot = await mkdtemp(join(tmpdir(), 'ocentra-screen-cdp-'));
  const rawPath = join(tempRoot, `${captureMode}.raw.png`);
  const encryptedPath = join(tempRoot, `${captureMode}.queue.enc`);
  await writeFile(rawPath, pngBytes);
  const iv = randomBytes(12);
  const key = randomBytes(32);
  const cipher = createCipheriv('aes-256-gcm', key, iv);
  const encrypted = Buffer.concat([cipher.update(pngBytes), cipher.final()]);
  const authTag = cipher.getAuthTag();
  const encryptedEnvelope = Buffer.concat([iv, authTag, encrypted]);
  await writeFile(encryptedPath, encryptedEnvelope);
  const proof = {
    captureMode,
    algorithm: 'aes-256-gcm',
    encryptedByteSize: encryptedEnvelope.byteLength,
    encryptedDigest: `sha256:${sha256(encryptedEnvelope)}`,
    rawTempPathHash: `sha256:${sha256(rawPath)}`,
    encryptedTempPathHash: `sha256:${sha256(encryptedPath)}`,
    keyMaterialRetained: false,
  };
  await rm(rawPath, { force: true });
  await rm(encryptedPath, { force: true });
  await rm(tempRoot, { recursive: true, force: true });
  return {
    ...proof,
    rawTempDeleted: !existsSync(rawPath),
    encryptedTempDeleted: !existsSync(encryptedPath),
  };
}

function parsePngInfo(pngBytes) {
  if (
    pngBytes.length < 24 ||
    pngBytes[0] !== 0x89 ||
    pngBytes[1] !== 0x50 ||
    pngBytes[2] !== 0x4e ||
    pngBytes[3] !== 0x47
  ) {
    throw new Error('Expected Page.captureScreenshot to return PNG bytes');
  }
  return {
    width: pngBytes.readUInt32BE(16),
    height: pngBytes.readUInt32BE(20),
  };
}

function evidenceRef(evidenceId, digest) {
  return {
    evidenceId,
    kind: 'local-db-row',
    digest,
    uri: null,
  };
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function runPackageCommand(args) {
  if (process.platform === 'win32') {
    return runCommand(...npmCommand([...args]));
  }
  return runCommand('npm', args);
}

function runCommand(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: process.cwd(),
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    const output = collectOutput(child);
    child.on('error', reject);
    child.on('exit', (code) => {
      const commandLine = `${command} ${args.join(' ')}`;
      if (code === 0) {
        successfulCommands.push(commandLine);
        resolve();
        return;
      }
      reject(new Error(`${commandLine} failed with ${code}\n${output()}`));
    });
  });
}

function collectOutput(child) {
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
  return () => chunks.join('');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
