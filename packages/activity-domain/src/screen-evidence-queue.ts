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

const isTimestampBefore = (before: string, after: string) => Date.parse(before) < Date.parse(after);
const isTimestampAtOrBefore = (before: string, after: string) => Date.parse(before) <= Date.parse(after);

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
  }).pipe(
    Schema.filter(
      (value) =>
        (isTimestampBefore(value.createdAt, value.expiresAt) &&
          isTimestampAtOrBefore(value.notBefore, value.expiresAt)) ||
        'Expected screen evidence queue TTL to expire after creation and not-before timestamps'
    ),
    Schema.filter(
      (value) =>
        value.attemptCount <= value.maxRetryCount ||
        'Expected screen evidence queue attempts to stay within the configured retry bound'
    ),
    Schema.filter(
      (value) =>
        value.status !== 'deleted' ||
        (value.deletionStatus === 'deleted' && value.deletedAt !== null && value.deletionProofRef !== null) ||
        'Expected deleted screen evidence queue jobs to carry deletion timestamp and proof'
    ),
    Schema.filter(
      (value) =>
        value.deletionStatus !== 'deleted' ||
        value.status === 'deleted' ||
        'Expected deleted screen evidence queue custody to match a deleted queue job status'
    ),
    Schema.filter(
      (value) =>
        value.status !== 'expired' ||
        (value.deletionStatus === 'expiredDeleted' && value.deletedAt !== null && value.deletionProofRef !== null) ||
        'Expected expired screen evidence queue jobs to prove expired image deletion'
    ),
    Schema.filter(
      (value) =>
        value.deletionStatus !== 'expiredDeleted' ||
        value.status === 'expired' ||
        'Expected expired-deleted screen evidence queue custody to match an expired queue job status'
    ),
    Schema.filter(
      (value) =>
        value.deletionStatus !== 'deleteFailed' ||
        (value.status === 'failed' && value.deletedAt === null && value.deletionProofRef === null) ||
        'Expected delete-failed screen evidence queue jobs to remain failed without deletion proof'
    )
  )
);

export type ScreenAnalysisQueueJob = Infer<typeof ScreenAnalysisQueueJobSchema>;
