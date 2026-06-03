import { describe, expect, it } from 'vitest';
import {
  BrowserExtensionNativeHostBoundarySchema,
  BrowserExtensionNativeHostSchemaVersion,
  BrowserNativeHostMaxMessageLengthBytes,
} from '../src/browser';

describe('browser extension native-host boundary contracts', () => {
  it('accepts managed-profile-only origin and schema validated native-host state', acceptsManagedNativeHostState);
  it('rejects personal-profile or unmanaged capture claims', rejectsPersonalOrUnmanagedCaptureClaims);
  it('rejects origin, schema, and length drift', rejectsNativeHostMessageDrift);
  it('keeps runtime signals manual-required until separately proved', keepsRuntimeSignalsManualRequired);
  it(
    'represents stale heartbeat and missing native-host states without capture claims',
    representsStaleAndMissingNativeHostStates
  );
});

function acceptsManagedNativeHostState() {
  const parsed = BrowserExtensionNativeHostBoundarySchema.safeParse(managedNativeHostBoundary());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.profileBinding).toBe('managed-profile-bound');
    expect(parsed.data.nativeHostMessageState).toBe('origin-validated');
    expect(parsed.data.personalProfileCaptureClaimed).toBe(false);
  }
}

function rejectsPersonalOrUnmanagedCaptureClaims() {
  expect(
    BrowserExtensionNativeHostBoundarySchema.safeParse({
      ...managedNativeHostBoundary(),
      profileBinding: 'unmanaged-personal-profile',
    }).success
  ).toBe(false);
  expect(
    BrowserExtensionNativeHostBoundarySchema.safeParse({
      ...managedNativeHostBoundary(),
      personalProfileCaptureClaimed: true,
    }).success
  ).toBe(false);
  expect(
    BrowserExtensionNativeHostBoundarySchema.safeParse({
      ...managedNativeHostBoundary(),
      unmanagedProfileCaptureClaimed: true,
    }).success
  ).toBe(false);
}

function rejectsNativeHostMessageDrift() {
  expect(
    BrowserExtensionNativeHostBoundarySchema.safeParse({
      ...managedNativeHostBoundary(),
      reportedOrigin: 'chrome-extension://other-extension',
    }).success
  ).toBe(false);
  expect(
    BrowserExtensionNativeHostBoundarySchema.safeParse({
      ...managedNativeHostBoundary(),
      nativeHostMessageState: 'schema-invalid',
      nativeHostSchemaValidated: true,
    }).success
  ).toBe(false);
  expect(
    BrowserExtensionNativeHostBoundarySchema.safeParse({
      ...managedNativeHostBoundary(),
      nativeHostMessageState: 'length-invalid',
      nativeHostMessageLengthBytes: BrowserNativeHostMaxMessageLengthBytes,
    }).success
  ).toBe(false);
}

function keepsRuntimeSignalsManualRequired() {
  const parsed = BrowserExtensionNativeHostBoundarySchema.safeParse({
    ...managedNativeHostBoundary(),
    minimumPermissionState: 'runtime-signal-proof-required',
    runtimeSignals: ['active-tab', 'canvas', 'webgl', 'fullscreen', 'pointer-lock', 'gamepad'],
  });

  expect(parsed.success).toBe(true);
  expect(
    BrowserExtensionNativeHostBoundarySchema.safeParse({
      ...managedNativeHostBoundary(),
      runtimeSignals: ['gamepad'],
    }).success
  ).toBe(false);
  expect(
    BrowserExtensionNativeHostBoundarySchema.safeParse({
      ...managedNativeHostBoundary(),
      minimumPermissionState: 'runtime-signal-proof-required',
      runtimeSignals: ['gamepad'],
      runtimeSignalCaptureClaimed: true,
    }).success
  ).toBe(false);
}

function representsStaleAndMissingNativeHostStates() {
  expect(
    BrowserExtensionNativeHostBoundarySchema.safeParse({
      ...managedNativeHostBoundary(),
      nativeHostMessageState: 'stale-heartbeat',
      extensionInstallState: 'heartbeat-stale',
      serviceWorkerHeartbeatState: 'stale',
      heartbeatAgeMs: 90_000,
    }).success
  ).toBe(true);
  expect(
    BrowserExtensionNativeHostBoundarySchema.safeParse({
      ...managedNativeHostBoundary(),
      nativeHostMessageState: 'native-host-missing',
      extensionInstallState: 'native-host-missing',
      serviceWorkerHeartbeatState: 'missing',
    }).success
  ).toBe(true);
}

function managedNativeHostBoundary() {
  return {
    schemaVersion: BrowserExtensionNativeHostSchemaVersion,
    checkedAt: '2026-06-03T01:25:00Z',
    sourceId: 'managed-extension-native-host-boundary',
    deviceId: 'local-dev-agent',
    managedBrowserSessionId: 'managed-browser-session-1',
    profileId: 'managed-profile-child',
    extensionId: 'ocentra-managed-extension',
    nativeHostId: 'ocentra-managed-native-host',
    extensionInstallState: 'installed-enabled',
    minimumPermissionState: 'minimum-url-title-tab',
    nativeHostMessageState: 'origin-validated',
    profileBinding: 'managed-profile-bound',
    reportedOrigin: 'chrome-extension://ocentra-managed-extension',
    allowedOrigin: 'chrome-extension://ocentra-managed-extension',
    nativeHostMessageLengthBytes: 512,
    serviceWorkerHeartbeatState: 'fresh',
    heartbeatAgeMs: 500,
    heartbeatStaleAfterMs: 30_000,
    runtimeSignals: [],
    nativeHostOriginValidated: true,
    nativeHostSchemaValidated: true,
    managedProfileOnly: true,
    personalProfileCaptureClaimed: false,
    unmanagedProfileCaptureClaimed: false,
    runtimeSignalCaptureClaimed: false,
  };
}
