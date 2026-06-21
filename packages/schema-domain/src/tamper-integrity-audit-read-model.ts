import {
  TamperIntegrityAuditEntrySchema,
  TamperIntegrityAuditReadModelSchema,
  TamperIntegrityAuditRequiredPayloadFields,
  type TamperIntegrityAuditEntry,
  type TamperIntegrityAuditHeartbeatState,
  type TamperIntegrityAuditPermissionState,
  type TamperIntegrityAuditServicePresenceState,
  type TamperIntegrityAuditSeverity,
  type TamperIntegrityAuditSignalKind,
  type TamperIntegrityAuditTamperState,
  type TamperIntegrityAuditUninstallState,
} from './tamper-integrity-audit';

type TamperIntegrityAuditEntryInput = {
  auditEntryId: string;
  signalKind: TamperIntegrityAuditSignalKind;
  heartbeatState?: TamperIntegrityAuditHeartbeatState;
  permissionState?: TamperIntegrityAuditPermissionState;
  servicePresenceState?: TamperIntegrityAuditServicePresenceState;
  uninstallState?: TamperIntegrityAuditUninstallState;
  tamperState?: TamperIntegrityAuditTamperState;
  severity: TamperIntegrityAuditSeverity;
  adminRemovalFlowRefs?: readonly string[];
  manualProofRequirements?: readonly string[];
};

const generatedAt = '2026-06-03T08:51:27.513Z';
const AdminRemovalFlowRef = 'documented-parent-admin-removal-flow-ref';

export const TamperIntegrityAuditReadModel = TamperIntegrityAuditReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'tamper-integrity-audit-contract-proof',
  generatedAt,
  sourceContractRefs: [
    'enforcement-integrity-tamper-feature-doc',
    'tamper-uninstall-protection-expectation',
    'v0-8-enforcement-integrity-runtime-audit-proof',
    'v0-8-integrity-alert-status-bridge-proof',
  ],
  entries: [
    tamperIntegrityAuditEntry({
      auditEntryId: 'tamper-audit-heartbeat-stale',
      signalKind: 'heartbeat-stale',
      heartbeatState: 'stale',
      severity: 'warning',
    }),
    tamperIntegrityAuditEntry({
      auditEntryId: 'tamper-audit-heartbeat-offline',
      signalKind: 'heartbeat-offline',
      heartbeatState: 'offline',
      servicePresenceState: 'offline',
      severity: 'critical',
    }),
    tamperIntegrityAuditEntry({
      auditEntryId: 'tamper-audit-permission-loss',
      signalKind: 'permission-loss',
      permissionState: 'permission-lost',
      severity: 'critical',
    }),
    tamperIntegrityAuditEntry({
      auditEntryId: 'tamper-audit-service-stopped',
      signalKind: 'service-stopped',
      servicePresenceState: 'stopped',
      severity: 'critical',
    }),
    tamperIntegrityAuditEntry({
      auditEntryId: 'tamper-audit-agent-removed',
      signalKind: 'agent-removed',
      servicePresenceState: 'removed',
      severity: 'critical',
    }),
    tamperIntegrityAuditEntry({
      auditEntryId: 'tamper-audit-uninstall-detected',
      signalKind: 'uninstall-detected',
      uninstallState: 'detected',
      severity: 'critical',
      adminRemovalFlowRefs: [AdminRemovalFlowRef],
      manualProofRequirements: ['platform uninstall artifact before removal detection can be claimed'],
    }),
    tamperIntegrityAuditEntry({
      auditEntryId: 'tamper-audit-manual-required',
      signalKind: 'tamper-manual-required',
      tamperState: 'manual-required',
      severity: 'manual-required',
      manualProofRequirements: ['security product review before anti-tamper behavior can be claimed'],
    }),
    tamperIntegrityAuditEntry({
      auditEntryId: 'tamper-audit-admin-removal-flow',
      signalKind: 'admin-removal-flow',
      severity: 'info',
      adminRemovalFlowRefs: [AdminRemovalFlowRef],
    }),
  ],
});

function tamperIntegrityAuditEntry(input: TamperIntegrityAuditEntryInput): TamperIntegrityAuditEntry {
  return TamperIntegrityAuditEntrySchema.parse({
    schemaVersion: 1,
    heartbeatState: 'not-applicable',
    permissionState: 'not-applicable',
    servicePresenceState: 'not-applicable',
    uninstallState: 'not-applicable',
    tamperState: 'not-applicable',
    payloadRedactionState: 'minimal-operational-fields-only',
    auditRefs: ['tamper-integrity-audit-entry-ref'],
    integrityRefs: ['agent-integrity-status-ref'],
    parentAlertRefs: ['parent-visible-integrity-alert-ref'],
    authenticatedDrillInRefs: ['authenticated-integrity-drill-in-ref'],
    adminRemovalFlowRefs: [],
    manualProofRequirements: [],
    nonClaims: [
      'no stealth behavior is claimed by this audit contract',
      'no privilege escalation is claimed by this audit contract',
      'no notification provider delivery is claimed by this audit contract',
      'no admin removal blocking is claimed by this audit contract',
    ],
    redactionSafePayloadFields: [...TamperIntegrityAuditRequiredPayloadFields],
    providerDeliveryClaimed: false,
    stealthBehaviorClaimed: false,
    privilegeEscalationClaimed: false,
    hiddenPersistenceClaimed: false,
    blocksAdminRemovalClaimed: false,
    rawChildDataIncluded: false,
    rawEvidencePayloadIncluded: false,
    rawUrlsIncluded: false,
    screenshotsIncluded: false,
    commandLinesIncluded: false,
    privatePathsIncluded: false,
    messageContentsIncluded: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
