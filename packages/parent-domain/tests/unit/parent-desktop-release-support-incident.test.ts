import { describe, expect, it } from 'vitest';
import { ParentDesktopReleaseSupportReadModelSchema } from '@ocentra-parent/schema-domain/parent-desktop-release-support';
import { RuntimeReadModel } from './parent-desktop-release-support-fixtures';

describe('parent desktop release support incident handoff contracts', () => {
  registerAcceptedIncidentHandoffTest();
  registerConsentGuardrailTest();
  registerSupportBundleManifestGuardrailTest();
  registerDiagnosticReferenceGuardrailTest();
  registerManualProductionSupportStateGuardrailTest();
});

function registerAcceptedIncidentHandoffTest(): void {
  it('accepts parent consent, safe bundle disclosure, and manual production support states', () => {
    const parsed = ParentDesktopReleaseSupportReadModelSchema.parse(RuntimeReadModel);

    expect(parsed.supportIncidentHandoff.parentConsent).toEqual({
      consentState: 'parent-approved',
      capturedBy: 'manual-export-action',
      disclosureState: 'shown-before-export',
      parentActor: 'parent manually exported support bundle after disclosure',
      consentRecordedAt: '2026-06-02T05:45:00.000Z',
      revocationState: 'manual-required',
    });
    expect(parsed.supportIncidentHandoff.supportBundleManifest.includedDataClasses).toEqual([
      'release-version',
      'commit-id',
      'platform-family',
      'package-runtime-state',
      'service-health-state',
      'route-state',
      'capability-state',
      'degraded-state',
      'redaction-summary',
      'manual-proof-reference',
      'incident-status',
    ]);
    expect(parsed.supportIncidentHandoff.manualProductionSupportStates.supportBackendUploadState).toBe(
      'not-implemented'
    );
  });
}

function registerConsentGuardrailTest(): void {
  it('rejects handoff without explicit parent consent and disclosure', () => {
    const missingConsent = withIncidentConsent({ consentState: 'missing' });
    const hiddenDisclosure = withIncidentConsent({ disclosureState: 'not-shown' });

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(missingConsent).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(hiddenDisclosure).success).toBe(false);
  });
}

function registerSupportBundleManifestGuardrailTest(): void {
  it('rejects manifests that include child evidence, raw URLs, screenshots, or SQLite data', () => {
    const childActivityFlag = withSupportBundleManifest({ containsChildActivity: true });
    const rawUrlFlag = withSupportBundleManifest({ containsRawUrls: true });
    const screenshotFlag = withSupportBundleManifest({ containsScreenshots: true });
    const sqliteFlag = withSupportBundleManifest({ containsSqliteSnapshots: true });
    const rawUrlDataClass = withSupportBundleManifest({
      includedDataClasses: [
        ...RuntimeReadModel.supportIncidentHandoff.supportBundleManifest.includedDataClasses,
        'raw-urls',
      ],
    });

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(childActivityFlag).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(rawUrlFlag).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(screenshotFlag).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(sqliteFlag).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(rawUrlDataClass).success).toBe(false);
  });
}

function registerDiagnosticReferenceGuardrailTest(): void {
  it('rejects diagnostic references that point to private paths, command logs, or sensitive data', () => {
    const privatePath = withIncidentDiagnosticReference('proof-json', {
      reference: 'C:/Users/parent/AppData/support-proof.json',
    });
    const commandReference = withIncidentDiagnosticReference('manual-runbook', {
      reference: 'test-results/manual-platform-proof/command-log.json',
    });
    const sensitiveReference = withIncidentDiagnosticReference('redaction-summary', { includesSensitiveData: true });

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(privatePath).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(commandReference).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(sensitiveReference).success).toBe(false);
  });
}

function registerManualProductionSupportStateGuardrailTest(): void {
  it('rejects backend upload, billing, account, or remote production support overclaims', () => {
    const backendUploadPreview = withManualProductionSupportStates({ supportBackendUploadState: 'preview-only' });
    const accountLookupPreview = withManualProductionSupportStates({ accountLookupState: 'preview-only' });
    const remoteControlPreview = withManualProductionSupportStates({ remoteControlState: 'preview-only' });
    const missingNonClaim = withManualProductionSupportStates({
      nonClaims: ['support incident proof keeps production support manual-required'],
    });

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(backendUploadPreview).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(accountLookupPreview).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(remoteControlPreview).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(missingNonClaim).success).toBe(false);
  });
}

function withIncidentConsent(patch: object) {
  return withIncidentHandoff({
    parentConsent: { ...RuntimeReadModel.supportIncidentHandoff.parentConsent, ...patch },
  });
}

function withSupportBundleManifest(patch: object) {
  return withIncidentHandoff({
    supportBundleManifest: { ...RuntimeReadModel.supportIncidentHandoff.supportBundleManifest, ...patch },
  });
}

function withIncidentDiagnosticReference(kind: string, patch: object) {
  return withIncidentHandoff({
    diagnosticReferences: RuntimeReadModel.supportIncidentHandoff.diagnosticReferences.map((entry) =>
      entry.kind === kind ? { ...entry, ...patch } : entry
    ),
  });
}

function withManualProductionSupportStates(patch: object) {
  return withIncidentHandoff({
    manualProductionSupportStates: {
      ...RuntimeReadModel.supportIncidentHandoff.manualProductionSupportStates,
      ...patch,
    },
  });
}

function withIncidentHandoff(patch: object) {
  return {
    ...RuntimeReadModel,
    supportIncidentHandoff: { ...RuntimeReadModel.supportIncidentHandoff, ...patch },
  };
}
