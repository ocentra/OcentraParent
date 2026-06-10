import {
  SocialParentNotificationDeliveryReadModelSnapshotSchema,
  type SocialParentNotificationDeliveryReadModelRow,
  type SocialParentNotificationDeliveryReadModelSnapshot,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { type DisplayText, decodeDisplayText } from '@ocentra-parent/text-domain/contracts';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalDetails } from './details';

export type SocialParentNotificationDeliveryPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText | PortalDetailValue;
};

export type SocialParentNotificationDeliveryPanelRow = {
  readonly key: PortalDetailValue;
  readonly title: DisplayText;
  readonly details: readonly SocialParentNotificationDeliveryPanelDetail[];
};

export type SocialParentNotificationDeliveryPanelIntent = {
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly summary: PortalDetailValue;
  readonly productClaim: DisplayText;
  readonly details: readonly SocialParentNotificationDeliveryPanelDetail[];
  readonly rows: readonly SocialParentNotificationDeliveryPanelRow[];
};

const Values = {
  EmptyRows: '0 parent notification readiness rows',
  NotReported: 'not reported',
  RefsSeparator: ', ',
  RowsSuffix: ' parent notification readiness rows',
} as const;

const Copy = {
  Body: decodeDisplayText(
    'Service-backed readiness projection shows parent-owned report status and manual gaps without claiming notification UI delivery, provider delivery, final policy execution, or enforcement.'
  ),
  ProductClaim: decodeDisplayText(
    'Parent report readiness projection only; parent notification UI delivery, external runtime report delivery, provider delivery, final policy execution, and enforcement remain unclaimed.'
  ),
  Title: decodeDisplayText('Social parent notification delivery readiness'),
} as const;

const RowTitles = {
  ManualRequired: decodeDisplayText('Parent notification manual proof required'),
  Ready: decodeDisplayText('Parent report status ready'),
  Unavailable: decodeDisplayText('Parent notification delivery unavailable'),
} as const;

export function createSocialParentNotificationDeliveryPanelIntent(
  snapshotInput: unknown
): SocialParentNotificationDeliveryPanelIntent {
  const parsed = SocialParentNotificationDeliveryReadModelSnapshotSchema.safeParse(snapshotInput);
  if (!parsed.success) {
    return emptyIntent();
  }
  return populatedIntent(parsed.data);
}

function populatedIntent(
  snapshot: SocialParentNotificationDeliveryReadModelSnapshot
): SocialParentNotificationDeliveryPanelIntent {
  return {
    title: Copy.Title,
    body: Copy.Body,
    summary: detailValue(String(snapshot.rows.length) + Values.RowsSuffix),
    productClaim: Copy.ProductClaim,
    details: [
      detail(PortalDetails.RowsReturned, String(snapshot.rows.length)),
      detail(PortalDetails.GeneratedAt, snapshot.generatedAt),
      detail(PortalDetails.ProductClaim, Copy.ProductClaim),
    ],
    rows: snapshot.rows.map(rowIntent),
  };
}

function emptyIntent(): SocialParentNotificationDeliveryPanelIntent {
  return {
    title: Copy.Title,
    body: Copy.Body,
    summary: detailValue(Values.EmptyRows),
    productClaim: Copy.ProductClaim,
    details: [
      detail(PortalDetails.RowsReturned, '0'),
      detail(PortalDetails.Status, Values.NotReported),
      detail(PortalDetails.ProductClaim, Copy.ProductClaim),
    ],
    rows: [],
  };
}

function rowIntent(row: SocialParentNotificationDeliveryReadModelRow): SocialParentNotificationDeliveryPanelRow {
  return {
    key: detailValue(row.notificationDeliveryReadinessRowId),
    title: rowTitle(row),
    details: [
      detail(PortalDetails.Status, row.notificationDeliveryReadinessState),
      detail(PortalDetails.Capability, row.reportDeliveryExecutionState),
      detail(PortalDetails.ReasonCodes, refsValue(row.manualProofRequirements)),
      detail(PortalDetails.EvidenceReferences, refsValue(row.sourceEvidenceRefs)),
      detail(PortalDetails.PolicyEvaluation, refsValue(row.sourcePolicyRefs)),
      detail(PortalDetails.InterventionAuditId, refsValue(row.sourceAuditRefs)),
      detail(PortalDetails.ProductClaim, Copy.ProductClaim),
    ],
  };
}

function rowTitle(row: SocialParentNotificationDeliveryReadModelRow): DisplayText {
  if (row.notificationDeliveryReadinessState === 'parent-report-status-ready') {
    return RowTitles.Ready;
  }
  if (row.notificationDeliveryReadinessState === 'unavailable') {
    return RowTitles.Unavailable;
  }
  return RowTitles.ManualRequired;
}

function detail(label: DisplayText, value: unknown): SocialParentNotificationDeliveryPanelDetail {
  return {
    label,
    value: detailValue(value),
  };
}

function refsValue(values: readonly unknown[]): PortalDetailValue {
  const refs = values.map((value) => String(value).trim()).filter((value) => value.length > 0);
  return detailValue(refs.length > 0 ? refs.join(Values.RefsSeparator) : Values.NotReported);
}

function detailValue(value: unknown): PortalDetailValue {
  const text = typeof value === 'string' && value.trim().length > 0 ? value : Values.NotReported;
  return decodePortalDetailValue(text);
}
