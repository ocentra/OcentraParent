import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import { ActivityDeviceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  ScreenCaptureReasonSchema,
  ScreenCaptureScopeSchema,
  ScreenDeletionStateSchema,
  ScreenEvidenceCustodyStateSchema,
  ScreenImageFormatSchema,
} from './screen-evidence-states';
import {
  ScreenEvidenceDeletionProofRefSchema,
  ScreenEvidenceEncryptedImageRefSchema,
  ScreenEvidenceImageDigestSchema,
  ScreenEvidenceQueueJobIdSchema,
} from './screen-evidence-primitives';
import {
  ScreenManagedBrowserCdpCaptureIdSchema,
  ScreenManagedBrowserCdpCaptureModeSchema,
  ScreenManagedBrowserCdpCaptureRequestIdSchema,
  ScreenManagedBrowserCdpCaptureSchemaVersion,
  ScreenManagedBrowserCdpMaxDimension,
  ScreenManagedBrowserCdpMaxPixels,
  ScreenManagedBrowserCdpMethodSchema,
  ScreenManagedBrowserCdpTargetIdSchema,
  ScreenManagedBrowserCdpTargetTypeSchema,
} from './screen-managed-browser-cdp-capture-values';

export * from './screen-managed-browser-cdp-capture-values';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const PositiveInteger = Schema.Number.pipe(Schema.int(), Schema.positive());
const NonNegativeNumber = Schema.Number.pipe(Schema.nonNegative());
const BoundedDimension = PositiveInteger.pipe(
  Schema.filter((value) => value <= ScreenManagedBrowserCdpMaxDimension || 'Expected CDP capture dimension cap')
);
const BoundedPixelCount = PositiveInteger.pipe(
  Schema.filter((value) => value <= ScreenManagedBrowserCdpMaxPixels || 'Expected CDP capture pixel cap')
);
const EvidenceRefsSchema = Schema.Array(ActivityEvidenceRefSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one managed-browser evidence reference')
);

const CdpViewportSchema = Schema.Struct({
  width: BoundedDimension,
  height: BoundedDimension,
  deviceScaleFactor: PositiveInteger,
});

const CdpClipSchema = Schema.Struct({
  x: NonNegativeNumber,
  y: NonNegativeNumber,
  width: BoundedDimension,
  height: BoundedDimension,
  scale: PositiveInteger,
});

const ScreenManagedBrowserCdpScreenshotRequestBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenManagedBrowserCdpCaptureSchemaVersion),
  requestId: ScreenManagedBrowserCdpCaptureRequestIdSchema,
  requestedAt: ActivityTimestampSchema,
  deviceRef: ActivityDeviceIdSchema,
  targetId: ScreenManagedBrowserCdpTargetIdSchema,
  targetType: ScreenManagedBrowserCdpTargetTypeSchema,
  captureMode: ScreenManagedBrowserCdpCaptureModeSchema,
  captureReason: ScreenCaptureReasonSchema,
  captureScope: Schema.Literal('managedBrowserWindow'),
  cdpMethod: ScreenManagedBrowserCdpMethodSchema,
  fromSurface: RequiredTrue,
  captureBeyondViewport: Schema.Boolean,
  viewport: CdpViewportSchema,
  clip: Schema.Union(CdpClipSchema, Schema.Null),
  estimatedPixelCount: BoundedPixelCount,
  maxPixelCount: Schema.Literal(ScreenManagedBrowserCdpMaxPixels),
  urlEvidenceRef: ActivityEvidenceRefSchema,
  titleEvidenceRef: ActivityEvidenceRefSchema,
  relatedEvidenceRefs: EvidenceRefsSchema,
  parentAllowsManagedBrowserCapture: RequiredTrue,
  parentAllowsLiveScreencast: RequiredFalse,
  allowDesktopCapture: RequiredFalse,
  allowFullScreenCapture: RequiredFalse,
  rawScreenshotRetentionAllowed: RequiredFalse,
  remoteUploadAllowed: RequiredFalse,
});

export const ScreenManagedBrowserCdpScreenshotRequestSchema = withParser(
  ScreenManagedBrowserCdpScreenshotRequestBaseSchema.pipe(
    Schema.filter(
      (value) => requestModeIsConsistent(value) || 'Expected CDP capture mode, clip, and pixel bounds to agree'
    )
  )
);

const ScreenManagedBrowserCdpScreenshotArtifactBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenManagedBrowserCdpCaptureSchemaVersion),
  captureId: ScreenManagedBrowserCdpCaptureIdSchema,
  requestId: ScreenManagedBrowserCdpCaptureRequestIdSchema,
  capturedAt: ActivityTimestampSchema,
  targetId: ScreenManagedBrowserCdpTargetIdSchema,
  cdpMethod: ScreenManagedBrowserCdpMethodSchema,
  captureMode: ScreenManagedBrowserCdpCaptureModeSchema,
  fromSurface: RequiredTrue,
  captureBeyondViewport: Schema.Boolean,
  captureScope: ScreenCaptureScopeSchema,
  imageWidth: BoundedDimension,
  imageHeight: BoundedDimension,
  imagePixelCount: BoundedPixelCount,
  imageByteSize: PositiveInteger,
  imageFormat: ScreenImageFormatSchema,
  imageDigest: ScreenEvidenceImageDigestSchema,
  urlEvidenceRef: ActivityEvidenceRefSchema,
  titleEvidenceRef: ActivityEvidenceRefSchema,
  screenshotEvidenceRef: ActivityEvidenceRefSchema,
  queueJobId: ScreenEvidenceQueueJobIdSchema,
  encryptedImageRef: ScreenEvidenceEncryptedImageRefSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  deletionRequired: RequiredTrue,
  deletionStatus: ScreenDeletionStateSchema,
  deletionProofRef: ScreenEvidenceDeletionProofRefSchema,
  rawTempPathRedacted: RequiredTrue,
  rawImageRetained: RequiredFalse,
  liveScreencastStarted: RequiredFalse,
  desktopCaptureAttempted: RequiredFalse,
  remoteUploadAllowed: RequiredFalse,
});

export const ScreenManagedBrowserCdpScreenshotArtifactSchema = withParser(
  ScreenManagedBrowserCdpScreenshotArtifactBaseSchema.pipe(
    Schema.filter(
      (value) =>
        artifactIsConsistent(value) ||
        'Expected managed-browser CDP screenshot artifact to stay page-scoped, queued, and deleted'
    )
  )
);

export type ScreenManagedBrowserCdpScreenshotRequest = Infer<typeof ScreenManagedBrowserCdpScreenshotRequestSchema>;
export type ScreenManagedBrowserCdpScreenshotArtifact = Infer<typeof ScreenManagedBrowserCdpScreenshotArtifactSchema>;

export const decodeScreenManagedBrowserCdpScreenshotRequest = Schema.decodeUnknownSync(
  ScreenManagedBrowserCdpScreenshotRequestSchema
);
export const decodeScreenManagedBrowserCdpScreenshotArtifact = Schema.decodeUnknownSync(
  ScreenManagedBrowserCdpScreenshotArtifactSchema
);

function requestModeIsConsistent(value: Infer<typeof ScreenManagedBrowserCdpScreenshotRequestBaseSchema>) {
  if (value.estimatedPixelCount > value.maxPixelCount) {
    return false;
  }
  if (!value.fromSurface || value.cdpMethod !== 'Page.captureScreenshot') {
    return false;
  }
  if (value.captureMode === 'page') {
    return (
      value.captureBeyondViewport && value.clip !== null && value.clip.width * value.clip.height <= value.maxPixelCount
    );
  }
  if (value.captureMode === 'crop') {
    return (
      !value.captureBeyondViewport && value.clip !== null && value.clip.width * value.clip.height <= value.maxPixelCount
    );
  }
  return (
    !value.captureBeyondViewport &&
    value.clip === null &&
    value.viewport.width * value.viewport.height <= value.maxPixelCount
  );
}

function artifactIsConsistent(value: Infer<typeof ScreenManagedBrowserCdpScreenshotArtifactBaseSchema>) {
  return (
    value.fromSurface &&
    (value.captureMode === 'page') === value.captureBeyondViewport &&
    value.captureScope === 'managedBrowserWindow' &&
    value.custodyState === 'child-device-temp-queue' &&
    value.deletionStatus === 'deleted' &&
    value.imagePixelCount === value.imageWidth * value.imageHeight &&
    !value.rawImageRetained &&
    !value.liveScreencastStarted &&
    !value.desktopCaptureAttempted &&
    !value.remoteUploadAllowed
  );
}
