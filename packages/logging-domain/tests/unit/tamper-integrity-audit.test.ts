import { describe, expect, it } from 'vitest';
import {
  TamperIntegrityAuditEntrySchema,
  TamperIntegrityAuditReadModelSchema,
  TamperIntegrityAuditRequiredPayloadFields,
} from '../../src/tamper-integrity-audit';
import { TamperIntegrityAuditReadModel } from '../../src/tamper-integrity-audit-read-model';

describe('tamper integrity audit logging contract', () => {
  it(
    'covers heartbeat stale offline permission stopped removed uninstall tamper and admin-removal rows',
    assertSignalCoverage
  );
  it(
    'keeps payload fields redaction-safe and excludes raw child activity evidence fields',
    assertRedactedPayloadFields
  );
  it(
    'keeps anti-tamper provider delivery stealth privilege and admin-blocking as explicit non-claims',
    assertNonClaims
  );
  it('requires uninstall manual proof and documented admin removal flow references', assertManualProofReferences);
  it('rejects claim upgrades sensitive fields missing refs and incoherent state rows', assertInvalidRowsRejected);
});

function assertSignalCoverage() {
  const readModel = TamperIntegrityAuditReadModelSchema.parse(TamperIntegrityAuditReadModel);
  expect(readModel.readModelId).toBe('tamper-integrity-audit-contract-proof');
  expect(readModel.entries).toHaveLength(8);
  expect(countBy(readModel.entries.map((entry) => entry.signalKind))).toEqual({
    'heartbeat-stale': 1,
    'heartbeat-offline': 1,
    'permission-loss': 1,
    'service-stopped': 1,
    'agent-removed': 1,
    'uninstall-detected': 1,
    'tamper-manual-required': 1,
    'admin-removal-flow': 1,
  });
  expect(entryFor('tamper-audit-heartbeat-stale').heartbeatState).toBe('stale');
  expect(entryFor('tamper-audit-heartbeat-offline').heartbeatState).toBe('offline');
  expect(entryFor('tamper-audit-permission-loss').permissionState).toBe('permission-lost');
  expect(entryFor('tamper-audit-service-stopped').servicePresenceState).toBe('stopped');
  expect(entryFor('tamper-audit-agent-removed').servicePresenceState).toBe('removed');
}

function assertRedactedPayloadFields() {
  for (const entry of TamperIntegrityAuditReadModel.entries) {
    expect(entry.payloadRedactionState).toBe('minimal-operational-fields-only');
    expect(entry.redactionSafePayloadFields).toEqual([...TamperIntegrityAuditRequiredPayloadFields]);
    expect(entry.rawChildDataIncluded).toBe(false);
    expect(entry.rawEvidencePayloadIncluded).toBe(false);
    expect(entry.rawUrlsIncluded).toBe(false);
    expect(entry.screenshotsIncluded).toBe(false);
    expect(entry.commandLinesIncluded).toBe(false);
    expect(entry.privatePathsIncluded).toBe(false);
    expect(entry.messageContentsIncluded).toBe(false);
    expect(entry.authenticatedDrillInRefs).toEqual(['authenticated-integrity-drill-in-ref']);
  }
}

function assertNonClaims() {
  for (const entry of TamperIntegrityAuditReadModel.entries) {
    expect(entry.providerDeliveryClaimed).toBe(false);
    expect(entry.stealthBehaviorClaimed).toBe(false);
    expect(entry.privilegeEscalationClaimed).toBe(false);
    expect(entry.hiddenPersistenceClaimed).toBe(false);
    expect(entry.blocksAdminRemovalClaimed).toBe(false);
    expect(entry.nonClaims).toEqual([
      'no stealth behavior is claimed by this audit contract',
      'no privilege escalation is claimed by this audit contract',
      'no notification provider delivery is claimed by this audit contract',
      'no admin removal blocking is claimed by this audit contract',
    ]);
  }
}

function assertManualProofReferences() {
  expect(entryFor('tamper-audit-uninstall-detected').uninstallState).toBe('detected');
  expect(entryFor('tamper-audit-uninstall-detected').adminRemovalFlowRefs).toEqual([
    'documented-parent-admin-removal-flow-ref',
  ]);
  expect(entryFor('tamper-audit-uninstall-detected').manualProofRequirements).toEqual([
    'platform uninstall artifact before removal detection can be claimed',
  ]);
  expect(entryFor('tamper-audit-admin-removal-flow').adminRemovalFlowRefs).toEqual([
    'documented-parent-admin-removal-flow-ref',
  ]);
  expect(entryFor('tamper-audit-manual-required').manualProofRequirements).toEqual([
    'security product review before anti-tamper behavior can be claimed',
  ]);
}

function assertInvalidRowsRejected() {
  const stale = entryFor('tamper-audit-heartbeat-stale');
  const uninstall = entryFor('tamper-audit-uninstall-detected');
  const tamper = entryFor('tamper-audit-manual-required');

  for (const invalidEntry of [
    { ...stale, auditEntryId: 'invalid-provider-delivery', providerDeliveryClaimed: true },
    { ...stale, auditEntryId: 'invalid-stealth', stealthBehaviorClaimed: true },
    { ...stale, auditEntryId: 'invalid-privilege', privilegeEscalationClaimed: true },
    { ...stale, auditEntryId: 'invalid-admin-block', blocksAdminRemovalClaimed: true },
    { ...stale, auditEntryId: 'invalid-raw-child-data', rawChildDataIncluded: true },
    { ...stale, auditEntryId: 'invalid-raw-url', rawUrlsIncluded: true },
    { ...stale, auditEntryId: 'invalid-message-content', messageContentsIncluded: true },
    {
      ...stale,
      auditEntryId: 'invalid-duplicate-payload-field',
      redactionSafePayloadFields: ['audit-entry-ref', 'audit-entry-ref'],
    },
    { ...stale, auditEntryId: 'invalid-missing-alert-ref', parentAlertRefs: [] },
    { ...stale, auditEntryId: 'invalid-stale-coherence', heartbeatState: 'fresh' },
    { ...uninstall, auditEntryId: 'invalid-uninstall-no-admin-flow', adminRemovalFlowRefs: [] },
    { ...uninstall, auditEntryId: 'invalid-uninstall-no-manual-proof', manualProofRequirements: [] },
    { ...tamper, auditEntryId: 'invalid-tamper-no-proof', manualProofRequirements: [] },
    {
      ...tamper,
      auditEntryId: 'invalid-missing-non-claim',
      nonClaims: ['no stealth behavior is claimed by this audit contract'],
    },
  ]) {
    expect(() => TamperIntegrityAuditEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(auditEntryId: string) {
  const entry = TamperIntegrityAuditReadModel.entries.find((candidate) => candidate.auditEntryId === auditEntryId);
  if (entry === undefined) {
    throw new Error(`Missing tamper integrity audit entry: ${auditEntryId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
