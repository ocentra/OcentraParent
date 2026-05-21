import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import { ActivityDeviceIdSchema, ActivitySourceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  ScreenCaptureReasonSchema,
  ScreenCaptureScopeSchema,
  ScreenDeletionStateSchema,
  ScreenImageFormatSchema,
  ScreenQueueStatusSchema,
} from './screen-evidence-states';
import {
  ScreenEvidenceAdapterIdSchema,
  ScreenEvidenceDeletionProofRefSchema,
  ScreenEvidenceEncryptedImageRefSchema,
  ScreenEvidenceImageDigestSchema,
  ScreenEvidenceLocalUserRefSchema,
  ScreenEvidenceParentSettingRefSchema,
  ScreenEvidenceQueueJobIdSchema,
  ScreenEvidenceReasonSchema,
  ScreenEvidenceRetryCountSchema,
  ScreenEvidenceSchemaVersion,
  ScreenEvidenceSettingVersionSchema,
} from './screen-evidence-primitives';

const RequiredTrue = Schema.Literal(true);

export const ScreenAnalysisQueueJobSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenEvidenceSchemaVersion),
    queueJobId: ScreenEvidenceQueueJobIdSchema,
    createdAt: ActivityTimestampSchema,
    notBefore: ActivityTimestampSchema,
    expiresAt: ActivityTimestampSchema,
    lastAttemptAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    captureReason: ScreenCaptureReasonSchema,
    captureScope: ScreenCaptureScopeSchema,
    sourceId: ActivitySourceIdSchema,
    adapterId: ScreenEvidenceAdapterIdSchema,
    deviceRef: ActivityDeviceIdSchema,
    localUserRef: ScreenEvidenceLocalUserRefSchema,
    parentSettingRef: ScreenEvidenceParentSettingRefSchema,
    settingVersion: ScreenEvidenceSettingVersionSchema,
    relatedEvidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
    encryptedImageRef: ScreenEvidenceEncryptedImageRefSchema,
    imageDigest: ScreenEvidenceImageDigestSchema,
    imageByteSize: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    imageFormat: ScreenImageFormatSchema,
    status: ScreenQueueStatusSchema,
    attemptCount: ScreenEvidenceRetryCountSchema,
    maxRetryCount: ScreenEvidenceRetryCountSchema,
    failureReason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
    unavailableReason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
    deletionRequired: RequiredTrue,
    deletedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    deletionStatus: ScreenDeletionStateSchema,
    deletionProofRef: Schema.Union(ScreenEvidenceDeletionProofRefSchema, Schema.Null),
    custodyState: Schema.Literal('child-device-temp-queue'),
  })
);

export type ScreenAnalysisQueueJob = Infer<typeof ScreenAnalysisQueueJobSchema>;
