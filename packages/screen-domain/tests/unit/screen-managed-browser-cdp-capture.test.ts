import { describe, expect, it } from 'vitest';
import {
  ScreenManagedBrowserCdpScreenshotArtifactSchema,
  ScreenManagedBrowserCdpScreenshotRequestSchema,
} from '@ocentra-parent/schema-domain/screen-managed-browser-cdp-capture';
import { ScreenManagedBrowserCdpMaxPixels } from '@ocentra-parent/schema-domain/screen-managed-browser-cdp-capture-values';

describe('screen managed browser CDP screenshot capture contracts', () => {
  it('accepts managed page, viewport, and crop screenshot requests tied to a target', acceptsCdpModes);
  it('accepts deleted encrypted temp queue screenshot artifacts', acceptsDeletedQueueArtifacts);
  it('rejects desktop, fullscreen, live screencast, raw retention, and remote upload flags', rejectsUnsafeFlags);
  it('rejects inconsistent crop and pixel-bound requests', rejectsInconsistentCropAndPixelBounds);
  it('rejects artifacts that are not managed-browser temp-queue deleted screenshots', rejectsUnsafeArtifacts);
});

function acceptsCdpModes() {
  const page = request({ captureMode: 'page' });
  const viewport = request({
    captureMode: 'viewport',
    captureBeyondViewport: false,
    clip: null,
  });
  const crop = request({
    captureMode: 'crop',
    captureBeyondViewport: false,
    clip: {
      x: 12,
      y: 24,
      width: 320,
      height: 240,
      scale: 1,
    },
    estimatedPixelCount: 76_800,
  });

  expect(ScreenManagedBrowserCdpScreenshotRequestSchema.parse(page).captureMode).toBe('page');
  expect(ScreenManagedBrowserCdpScreenshotRequestSchema.parse(viewport).captureMode).toBe('viewport');
  expect(ScreenManagedBrowserCdpScreenshotRequestSchema.parse(crop).clip?.width).toBe(320);
}

function acceptsDeletedQueueArtifacts() {
  const parsed = ScreenManagedBrowserCdpScreenshotArtifactSchema.parse(artifact());

  expect(parsed.cdpMethod).toBe('Page.captureScreenshot');
  expect(parsed.captureScope).toBe('managedBrowserWindow');
  expect(parsed.custodyState).toBe('child-device-temp-queue');
  expect(parsed.deletionRequired).toBe(true);
  expect(parsed.deletionStatus).toBe('deleted');
  expect(parsed.rawImageRetained).toBe(false);
  expect(parsed.liveScreencastStarted).toBe(false);
  expect(parsed.desktopCaptureAttempted).toBe(false);
  expect(parsed.remoteUploadAllowed).toBe(false);
}

function rejectsUnsafeFlags() {
  const safe = request();

  expect(
    ScreenManagedBrowserCdpScreenshotRequestSchema.safeParse({
      ...safe,
      allowDesktopCapture: true,
    }).success
  ).toBe(false);
  expect(
    ScreenManagedBrowserCdpScreenshotRequestSchema.safeParse({
      ...safe,
      allowFullScreenCapture: true,
    }).success
  ).toBe(false);
  expect(
    ScreenManagedBrowserCdpScreenshotRequestSchema.safeParse({
      ...safe,
      parentAllowsLiveScreencast: true,
    }).success
  ).toBe(false);
  expect(
    ScreenManagedBrowserCdpScreenshotRequestSchema.safeParse({
      ...safe,
      rawScreenshotRetentionAllowed: true,
    }).success
  ).toBe(false);
  expect(
    ScreenManagedBrowserCdpScreenshotRequestSchema.safeParse({
      ...safe,
      remoteUploadAllowed: true,
    }).success
  ).toBe(false);
}

function rejectsInconsistentCropAndPixelBounds() {
  expect(
    ScreenManagedBrowserCdpScreenshotRequestSchema.safeParse({
      ...request({ captureMode: 'crop' }),
      clip: null,
    }).success
  ).toBe(false);
  expect(
    ScreenManagedBrowserCdpScreenshotRequestSchema.safeParse({
      ...request(),
      estimatedPixelCount: ScreenManagedBrowserCdpMaxPixels + 1,
    }).success
  ).toBe(false);
  expect(
    ScreenManagedBrowserCdpScreenshotRequestSchema.safeParse({
      ...request({ captureMode: 'viewport' }),
      clip: {
        x: 0,
        y: 0,
        width: 100,
        height: 100,
        scale: 1,
      },
    }).success
  ).toBe(false);
}

function rejectsUnsafeArtifacts() {
  const safe = artifact();

  expect(
    ScreenManagedBrowserCdpScreenshotArtifactSchema.safeParse({
      ...safe,
      captureScope: 'activeWindow',
    }).success
  ).toBe(false);
  expect(
    ScreenManagedBrowserCdpScreenshotArtifactSchema.safeParse({
      ...safe,
      custodyState: 'parent-device-cache',
    }).success
  ).toBe(false);
  expect(
    ScreenManagedBrowserCdpScreenshotArtifactSchema.safeParse({
      ...safe,
      deletionStatus: 'deletePending',
    }).success
  ).toBe(false);
  expect(
    ScreenManagedBrowserCdpScreenshotArtifactSchema.safeParse({
      ...safe,
      rawImageRetained: true,
    }).success
  ).toBe(false);
}

function request(overrides = {}) {
  return {
    schemaVersion: 1,
    requestId: 'managed-browser-cdp-capture-request',
    requestedAt: '2026-06-05T06:50:00.000Z',
    deviceRef: 'windows-child-device',
    targetId: 'managed-browser-target-page-1',
    targetType: 'page',
    captureMode: 'page',
    captureReason: 'managedBrowserUrlChange',
    captureScope: 'managedBrowserWindow',
    cdpMethod: 'Page.captureScreenshot',
    fromSurface: true,
    captureBeyondViewport: true,
    viewport: {
      width: 1280,
      height: 720,
      deviceScaleFactor: 1,
    },
    clip: {
      x: 0,
      y: 0,
      width: 1280,
      height: 720,
      scale: 1,
    },
    estimatedPixelCount: 921_600,
    maxPixelCount: ScreenManagedBrowserCdpMaxPixels,
    urlEvidenceRef: evidenceRef('managed-browser-url-ref'),
    titleEvidenceRef: evidenceRef('managed-browser-title-ref'),
    relatedEvidenceRefs: [evidenceRef('managed-browser-url-ref'), evidenceRef('managed-browser-title-ref')],
    parentAllowsManagedBrowserCapture: true,
    parentAllowsLiveScreencast: false,
    allowDesktopCapture: false,
    allowFullScreenCapture: false,
    rawScreenshotRetentionAllowed: false,
    remoteUploadAllowed: false,
    ...overrides,
  };
}

function artifact(overrides = {}) {
  return {
    schemaVersion: 1,
    captureId: 'managed-browser-cdp-capture-page',
    requestId: 'managed-browser-cdp-capture-request',
    capturedAt: '2026-06-05T06:50:01.000Z',
    targetId: 'managed-browser-target-page-1',
    cdpMethod: 'Page.captureScreenshot',
    captureMode: 'page',
    fromSurface: true,
    captureBeyondViewport: true,
    captureScope: 'managedBrowserWindow',
    imageWidth: 1280,
    imageHeight: 720,
    imagePixelCount: 921_600,
    imageByteSize: 64_000,
    imageFormat: 'png',
    imageDigest: 'sha256-managed-browser-cdp-capture-page',
    urlEvidenceRef: evidenceRef('managed-browser-url-ref'),
    titleEvidenceRef: evidenceRef('managed-browser-title-ref'),
    screenshotEvidenceRef: evidenceRef('managed-browser-screenshot-ref'),
    queueJobId: 'managed-browser-cdp-queue-job',
    encryptedImageRef: 'encrypted-managed-browser-cdp-image-ref',
    custodyState: 'child-device-temp-queue',
    deletionRequired: true,
    deletionStatus: 'deleted',
    deletionProofRef: 'managed-browser-cdp-delete-proof-ref',
    rawTempPathRedacted: true,
    rawImageRetained: false,
    liveScreencastStarted: false,
    desktopCaptureAttempted: false,
    remoteUploadAllowed: false,
    ...overrides,
  };
}

function evidenceRef(evidenceId: string) {
  return {
    evidenceId,
    kind: 'local-db-row',
    digest: `${evidenceId}-digest`,
    uri: null,
  };
}
