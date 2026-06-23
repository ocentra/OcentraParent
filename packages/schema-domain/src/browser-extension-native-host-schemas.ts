import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  ActivityDeviceIdSchema,
  ActivitySourceIdSchema,
  ActivityTimestampSchema,
} from '@ocentra-parent/schema-domain/evidence-primitives';
import { BrowserManagedSessionIdSchema, BrowserOriginSchema, BrowserProfileIdSchema } from './browser-schemas';

export const BrowserExtensionNativeHostSchemaVersion = 1;
export const BrowserNativeHostMaxMessageLengthBytes = 1_048_576;

export const BrowserExtensionIdSchema = withParser(brandedNonEmptyStringSchema('BrowserExtensionId'));
export const BrowserNativeHostIdSchema = withParser(brandedNonEmptyStringSchema('BrowserNativeHostId'));

export const BrowserExtensionInstallStateSchema = withParser(
  Schema.Literal(
    'not-installed',
    'installed-enabled',
    'installed-disabled',
    'permission-required',
    'native-host-missing',
    'heartbeat-stale',
    'error'
  )
);
export const BrowserExtensionMinimumPermissionStateSchema = withParser(
  Schema.Literal('minimum-url-title-tab', 'permission-required', 'runtime-signal-proof-required')
);
export const BrowserNativeHostMessageStateSchema = withParser(
  Schema.Literal(
    'origin-validated',
    'origin-invalid',
    'schema-invalid',
    'length-invalid',
    'stale-heartbeat',
    'native-host-missing'
  )
);
export const BrowserExtensionRuntimeSignalSchema = withParser(
  Schema.Literal('active-tab', 'canvas', 'webgl', 'fullscreen', 'pointer-lock', 'gamepad')
);
export const BrowserExtensionManagedProfileBindingSchema = withParser(
  Schema.Literal('managed-profile-bound', 'missing-managed-session', 'unmanaged-personal-profile', 'default-profile')
);
export const BrowserExtensionHeartbeatStateSchema = withParser(Schema.Literal('fresh', 'stale', 'missing'));

const BrowserExtensionNativeHostBoundaryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserExtensionNativeHostSchemaVersion),
  checkedAt: ActivityTimestampSchema,
  sourceId: ActivitySourceIdSchema,
  deviceId: ActivityDeviceIdSchema,
  managedBrowserSessionId: Schema.Union(BrowserManagedSessionIdSchema, Schema.Null),
  profileId: Schema.Union(BrowserProfileIdSchema, Schema.Null),
  extensionId: BrowserExtensionIdSchema,
  nativeHostId: BrowserNativeHostIdSchema,
  extensionInstallState: BrowserExtensionInstallStateSchema,
  minimumPermissionState: BrowserExtensionMinimumPermissionStateSchema,
  nativeHostMessageState: BrowserNativeHostMessageStateSchema,
  profileBinding: BrowserExtensionManagedProfileBindingSchema,
  reportedOrigin: BrowserOriginSchema,
  allowedOrigin: BrowserOriginSchema,
  nativeHostMessageLengthBytes: Schema.Number,
  serviceWorkerHeartbeatState: BrowserExtensionHeartbeatStateSchema,
  heartbeatAgeMs: Schema.Number,
  heartbeatStaleAfterMs: Schema.Number,
  runtimeSignals: Schema.Array(BrowserExtensionRuntimeSignalSchema),
  nativeHostOriginValidated: Schema.Boolean,
  nativeHostSchemaValidated: Schema.Boolean,
  managedProfileOnly: Schema.Boolean,
  personalProfileCaptureClaimed: Schema.Boolean,
  unmanagedProfileCaptureClaimed: Schema.Boolean,
  runtimeSignalCaptureClaimed: Schema.Boolean,
});

type BrowserExtensionNativeHostBoundaryCandidate = Infer<typeof BrowserExtensionNativeHostBoundaryBaseSchema>;

export const BrowserExtensionNativeHostBoundarySchema = withParser(
  BrowserExtensionNativeHostBoundaryBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserExtensionNativeHostBoundaryIsHonest(value) ||
        'Expected extension/native-host boundary to stay managed-profile-only, origin-validated, schema-validated, heartbeat-bound, and no-claim for personal profiles or runtime signals'
    )
  )
);

export type BrowserExtensionId = typeof BrowserExtensionIdSchema.Type;
export type BrowserNativeHostId = typeof BrowserNativeHostIdSchema.Type;
export type BrowserExtensionInstallState = Infer<typeof BrowserExtensionInstallStateSchema>;
export type BrowserExtensionMinimumPermissionState = Infer<typeof BrowserExtensionMinimumPermissionStateSchema>;
export type BrowserNativeHostMessageState = Infer<typeof BrowserNativeHostMessageStateSchema>;
export type BrowserExtensionRuntimeSignal = Infer<typeof BrowserExtensionRuntimeSignalSchema>;
export type BrowserExtensionManagedProfileBinding = Infer<typeof BrowserExtensionManagedProfileBindingSchema>;
export type BrowserExtensionHeartbeatState = Infer<typeof BrowserExtensionHeartbeatStateSchema>;
export type BrowserExtensionNativeHostBoundary = Infer<typeof BrowserExtensionNativeHostBoundarySchema>;

function browserExtensionNativeHostBoundaryIsHonest(value: BrowserExtensionNativeHostBoundaryCandidate): boolean {
  if (
    !value.managedProfileOnly ||
    value.personalProfileCaptureClaimed ||
    value.unmanagedProfileCaptureClaimed ||
    value.runtimeSignalCaptureClaimed
  ) {
    return false;
  }
  if (
    value.profileBinding !== 'managed-profile-bound' ||
    value.managedBrowserSessionId === null ||
    value.profileId === null
  ) {
    return false;
  }
  return (
    nativeHostMessageStateIsConsistent(value) && heartbeatStateIsConsistent(value) && runtimeSignalsStayManual(value)
  );
}

function nativeHostMessageStateIsConsistent(value: BrowserExtensionNativeHostBoundaryCandidate): boolean {
  switch (value.nativeHostMessageState) {
    case 'origin-validated':
      return (
        value.nativeHostOriginValidated &&
        value.nativeHostSchemaValidated &&
        value.reportedOrigin === value.allowedOrigin &&
        messageLengthIsValid(value.nativeHostMessageLengthBytes)
      );
    case 'origin-invalid':
      return !value.nativeHostOriginValidated || value.reportedOrigin !== value.allowedOrigin;
    case 'schema-invalid':
      return !value.nativeHostSchemaValidated;
    case 'length-invalid':
      return !messageLengthIsValid(value.nativeHostMessageLengthBytes);
    case 'stale-heartbeat':
      return value.serviceWorkerHeartbeatState === 'stale';
    case 'native-host-missing':
      return value.extensionInstallState === 'native-host-missing';
  }
}

function heartbeatStateIsConsistent(value: BrowserExtensionNativeHostBoundaryCandidate): boolean {
  if (value.serviceWorkerHeartbeatState === 'fresh') {
    return value.heartbeatAgeMs <= value.heartbeatStaleAfterMs;
  }
  if (value.serviceWorkerHeartbeatState === 'stale') {
    return value.heartbeatAgeMs > value.heartbeatStaleAfterMs;
  }
  return value.nativeHostMessageState === 'native-host-missing';
}

function runtimeSignalsStayManual(value: BrowserExtensionNativeHostBoundaryCandidate): boolean {
  return value.runtimeSignals.length === 0 || value.minimumPermissionState === 'runtime-signal-proof-required';
}

function messageLengthIsValid(value: number): boolean {
  return value > 0 && value <= BrowserNativeHostMaxMessageLengthBytes;
}
