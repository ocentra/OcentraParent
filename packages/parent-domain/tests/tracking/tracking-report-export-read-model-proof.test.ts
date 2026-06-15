import { describe, expect, it } from 'vitest';
import {
  TrackingReportExportReadModelPacketSchema,
  buildTrackingReportExportReadModelProof,
  type TrackingReportExportReadModelKind,
  type TrackingReportExportReadModelPacket,
  type TrackingReportExportReadModelProof,
} from '../../src/tracking-report-export-read-model-proof';

const GeneratedAt = '2026-06-06T20:50:00.000Z';

describe('tracking report export read-model proof', () => {
  it('builds redacted export packets without raw location or product overclaims', () => {
    const proof = buildTrackingReportExportReadModelProof(GeneratedAt);

    expect(proof.proofMode).toBe('tracking-report-export-read-model-proof');
    expect(proof.packets.map((packet) => packet.packetKind)).toEqual([
      'redacted-report-export-packet',
      'retention-audit-export-packet',
      'family-dashboard-summary-packet',
      'policy-drill-in-export-packet',
    ]);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.productClaims.rawLocationPayloadClaimed).toBe(false);

    for (const packet of proof.packets) {
      expectPacketRefs(packet);
      expectNoProductClaims(packet);
    }
  });

  it('keeps the retention audit packet scoped to parent-owned local export custody', () => {
    const retentionPacket = packetFor(
      buildTrackingReportExportReadModelProof(GeneratedAt),
      'retention-audit-export-packet'
    );

    expect(retentionPacket.custodyScope).toBe('parent-owned-local-export');
    expect(retentionPacket.reasonCodes).toContain('tracking-retention-audit-export-read-model-ready');
    expect(retentionPacket.auditRefs).toContain('tracking-report-export-audit-retention-audit');
  });

  it('rejects packets missing any upstream proof family or evidence refs', () => {
    const reportPacket = packetFor(
      buildTrackingReportExportReadModelProof(GeneratedAt),
      'redacted-report-export-packet'
    );

    expect(TrackingReportExportReadModelPacketSchema.safeParse({ ...reportPacket, sourceProofRefs: [] }).success).toBe(
      false
    );
    expect(
      TrackingReportExportReadModelPacketSchema.safeParse({ ...reportPacket, serviceReadModelProofRefs: [] }).success
    ).toBe(false);
    expect(
      TrackingReportExportReadModelPacketSchema.safeParse({ ...reportPacket, reportConsumerProofRefs: [] }).success
    ).toBe(false);
    expect(
      TrackingReportExportReadModelPacketSchema.safeParse({ ...reportPacket, dashboardRollupProofRefs: [] }).success
    ).toBe(false);
    expect(
      TrackingReportExportReadModelPacketSchema.safeParse({ ...reportPacket, retentionSettingsProofRefs: [] }).success
    ).toBe(false);
    expect(
      TrackingReportExportReadModelPacketSchema.safeParse({ ...reportPacket, evidenceReferences: [] }).success
    ).toBe(false);
  });

  it('rejects impossible redaction counts and wrong retention custody', () => {
    const proof = buildTrackingReportExportReadModelProof(GeneratedAt);
    const reportPacket = packetFor(proof, 'redacted-report-export-packet');
    const retentionPacket = packetFor(proof, 'retention-audit-export-packet');

    expect(
      TrackingReportExportReadModelPacketSchema.safeParse({
        ...reportPacket,
        redactedEvidenceRefCount: reportPacket.exportedRowCount + 1,
      }).success
    ).toBe(false);
    expect(
      TrackingReportExportReadModelPacketSchema.safeParse({
        ...retentionPacket,
        custodyScope: 'parent-owned-redacted-report',
      }).success
    ).toBe(false);
  });
});

function packetFor(
  proof: TrackingReportExportReadModelProof,
  packetKind: TrackingReportExportReadModelKind
): TrackingReportExportReadModelPacket {
  const packet = proof.packets.find((entry) => entry.packetKind === packetKind);
  if (packet === undefined) {
    throw new Error(`Missing tracking report export packet: ${packetKind}`);
  }
  return packet;
}

function expectPacketRefs(packet: TrackingReportExportReadModelPacket): void {
  expect(packet.packetState).toBe('export-read-model-ready');
  expect(packet.requiredProofTier).toBe('P2_HOSTED_CI');
  expect(packet.currentProofTier).toBe('P2_HOSTED_CI');
  expect(packet.sourceProofRefs.length).toBeGreaterThan(0);
  expect(packet.serviceReadModelProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json'
  );
  expect(packet.reportConsumerProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json'
  );
  expect(packet.dashboardRollupProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/23-family-dashboard-rollup-proof.json'
  );
  expect(packet.retentionSettingsProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json'
  );
  expect(packet.evidenceReferences.length).toBeGreaterThan(0);
  expect(packet.redactionState).toBe('evidence-refs-only');
  expect(packet.remoteSyncEnabled).toBe(false);
  expect(packet.remoteAiEnabled).toBe(false);
}

function expectNoProductClaims(packet: TrackingReportExportReadModelPacket): void {
  expect(packet.reportExportReadModelClaimed).toBe(true);
  expect(packet.rawLocationPayloadClaimed).toBe(false);
  expect(packet.portalUiClaimed).toBe(false);
  expect(packet.serviceMutationClaimed).toBe(false);
  expect(packet.platformRuntimeClaimed).toBe(false);
  expect(packet.childDeviceDeliveryClaimed).toBe(false);
  expect(packet.providerDeliveryClaimed).toBe(false);
  expect(packet.notificationReceiptClaimed).toBe(false);
  expect(packet.physicalDeviceClaimed).toBe(false);
  expect(packet.authorityClaimed).toBe(false);
  expect(packet.productClaimReady).toBe(false);
}
