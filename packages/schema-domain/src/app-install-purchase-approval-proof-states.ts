const SchemaVersion = 'app-install-purchase-approval-contract-proof';
const ContractBoundary =
  'contract proof only; no platform adapter no store integration no portal runtime no child-device delivery';

export const AppInstallPurchaseApprovalReportRefs = {
  RequestAudit: 'app-install-purchase-request-audit-report-ref',
  DecisionAudit: 'app-install-purchase-decision-audit-report-ref',
  ChildFacing: 'app-install-purchase-child-facing-report-ref',
  PlatformLimitation: 'app-install-purchase-platform-limitation-report-ref',
} as const;

const ChildFacingStateInputs = [
  {
    childStateId: 'child-state-install-pending-parent-review',
    requestId: 'install-request-proof-1',
    requestKind: 'install',
    childVisibleStatus: 'pending-parent-review-visible',
    sourceApprovalState: {
      state: 'pending-parent-review',
      expiryState: 'not-expiring',
      expiresAt: null,
      reviewReason: null,
    },
    auditSource: 'request',
  },
  {
    childStateId: 'child-state-install-approved',
    requestId: 'install-request-proof-1',
    requestKind: 'install',
    childVisibleStatus: 'approved-visible',
    sourceApprovalState: {
      state: 'approved',
      expiryState: 'not-expiring',
      expiresAt: null,
      reviewReason: null,
    },
    auditSource: 'decision',
  },
  {
    childStateId: 'child-state-purchase-denied',
    requestId: 'purchase-request-proof-1',
    requestKind: 'purchase',
    childVisibleStatus: 'denied-visible',
    sourceApprovalState: {
      state: 'denied',
      expiryState: 'not-expiring',
      expiresAt: null,
      reviewReason: null,
    },
    auditSource: 'decision',
  },
  {
    childStateId: 'child-state-subscription-time-box',
    requestId: 'subscription-request-proof-1',
    requestKind: 'subscription',
    childVisibleStatus: 'time-box-visible',
    sourceApprovalState: {
      state: 'time-box-active',
      expiryState: 'time-box-active',
      expiresAt: '2026-06-10T07:10:00.000Z',
      reviewReason: null,
    },
    auditSource: 'decision',
  },
  {
    childStateId: 'child-state-purchase-review-needed',
    requestId: 'purchase-request-proof-1',
    requestKind: 'purchase',
    childVisibleStatus: 'review-needed-visible',
    sourceApprovalState: {
      state: 'review-needed',
      expiryState: 'review-needed',
      expiresAt: null,
      reviewReason: 'age rating changed',
    },
    auditSource: 'decision',
  },
] as const;

export function appInstallPurchaseApprovalChildFacingStates(input: {
  readonly requestAuditEvent: unknown;
  readonly decisionAuditEvent: unknown;
}) {
  return ChildFacingStateInputs.map(({ auditSource, ...stateInput }) =>
    childState({
      ...stateInput,
      auditEventRefs: [auditSource === 'request' ? input.requestAuditEvent : input.decisionAuditEvent],
    })
  );
}

export function appInstallPurchaseApprovalAuditReportIntegration(input: {
  readonly requestAuditEvent: unknown;
  readonly decisionAuditEvent: unknown;
}) {
  return [
    auditReportRow(
      'request-audit-history',
      'contract-only',
      [input.requestAuditEvent],
      [AppInstallPurchaseApprovalReportRefs.RequestAudit]
    ),
    auditReportRow(
      'parent-decision-audit-history',
      'contract-only',
      [input.decisionAuditEvent],
      [AppInstallPurchaseApprovalReportRefs.DecisionAudit]
    ),
    auditReportRow(
      'child-facing-state-report',
      'manual-required',
      [input.requestAuditEvent, input.decisionAuditEvent],
      [AppInstallPurchaseApprovalReportRefs.ChildFacing]
    ),
    auditReportRow(
      'platform-limitation-report',
      'manual-required',
      [input.requestAuditEvent],
      [AppInstallPurchaseApprovalReportRefs.PlatformLimitation]
    ),
  ] as const;
}

function childState(input: {
  readonly childStateId: string;
  readonly requestId: string;
  readonly requestKind: 'install' | 'purchase' | 'subscription';
  readonly childVisibleStatus:
    | 'pending-parent-review-visible'
    | 'approved-visible'
    | 'denied-visible'
    | 'time-box-visible'
    | 'review-needed-visible';
  readonly sourceApprovalState: {
    readonly state: 'pending-parent-review' | 'approved' | 'denied' | 'time-box-active' | 'review-needed';
    readonly expiryState: 'not-expiring' | 'time-box-active' | 'review-needed';
    readonly expiresAt: string | null;
    readonly reviewReason: string | null;
  };
  readonly auditEventRefs: readonly unknown[];
}) {
  return {
    schemaVersion: SchemaVersion,
    platform: 'android',
    deliveryState: 'manual-required',
    deliveryRequirement: 'real child-device agent delivery proof before child-visible status can be claimed',
    reportRefs: [AppInstallPurchaseApprovalReportRefs.ChildFacing],
    claimBoundary: ContractBoundary,
    ...input,
  } as const;
}

function auditReportRow(
  surface:
    | 'request-audit-history'
    | 'parent-decision-audit-history'
    | 'child-facing-state-report'
    | 'platform-limitation-report',
  integrationState: 'contract-only' | 'manual-required',
  auditEventRefs: readonly unknown[],
  reportRefs: readonly string[]
) {
  return {
    schemaVersion: SchemaVersion,
    surface,
    integrationState,
    auditEventRefs,
    reportRefs,
    proofRequirement: 'contract status only; report UI and runtime report delivery need separate proof',
    claimBoundary: 'contract proof only; no portal runtime no platform adapter no store integration',
  } as const;
}
