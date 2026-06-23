import {
  SocialAlertReportParentSurfaceReadModelSnapshotSchema,
  type SocialAlertReportParentSurfaceReadModelRow,
  type SocialAlertReportParentSurfaceReadModelSnapshot,
} from '@ocentra-parent/schema-domain/agent-social-alert-report-parent-surface-read-model';
import { type DisplayText, decodeDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { decodePortalDetailValue, type PortalDetailValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { PortalDetails } from './details';

export type SocialAlertReportParentSurfacePanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText | PortalDetailValue;
};

export type SocialAlertReportParentSurfacePanelRow = {
  readonly key: PortalDetailValue;
  readonly title: DisplayText;
  readonly details: readonly SocialAlertReportParentSurfacePanelDetail[];
};

export type SocialAlertReportParentSurfacePanelIntent = {
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly summary: PortalDetailValue;
  readonly productClaim: DisplayText;
  readonly details: readonly SocialAlertReportParentSurfacePanelDetail[];
  readonly rows: readonly SocialAlertReportParentSurfacePanelRow[];
};

const Values = {
  EmptyRows: '0 parent surface rows',
  NotReported: 'not reported',
  RefsSeparator: ', ',
  RowsSuffix: ' parent surface rows',
} as const;

const Copy = {
  Body: decodeDisplayText(
    'Service-backed parent-surface status shows provider and preference handoff state without rendering notification, preference, history, or delivery UI.'
  ),
  ProductClaim: decodeDisplayText(
    'Parent-surface status projection only; notification UI delivery, provider delivery, receipt ingestion, final policy execution, and enforcement remain unclaimed.'
  ),
  Title: decodeDisplayText('Social parent surface status'),
} as const;

const RowTitles = {
  Manual: decodeDisplayText('Parent surface manual action required'),
  Unavailable: decodeDisplayText('Parent surface unavailable'),
} as const;

export function createSocialAlertReportParentSurfacePanelIntent(
  snapshotInput: unknown
): SocialAlertReportParentSurfacePanelIntent {
  const parsed = SocialAlertReportParentSurfaceReadModelSnapshotSchema.safeParse(snapshotInput);
  if (!parsed.success) {
    return emptyIntent();
  }
  return populatedIntent(parsed.data);
}

function populatedIntent(
  snapshot: SocialAlertReportParentSurfaceReadModelSnapshot
): SocialAlertReportParentSurfacePanelIntent {
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

function emptyIntent(): SocialAlertReportParentSurfacePanelIntent {
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

function rowIntent(row: SocialAlertReportParentSurfaceReadModelRow): SocialAlertReportParentSurfacePanelRow {
  return {
    key: detailValue(row.surfaceRowId),
    title: row.parentSurfaceStatus === 'manual-action-required' ? RowTitles.Manual : RowTitles.Unavailable,
    details: [
      detail(PortalDetails.Status, row.parentSurfaceStatus),
      detail(PortalDetails.Capability, row.preferenceVisibility),
      detail(PortalDetails.ReasonCodes, refsValue(row.manualProofRequirements)),
      detail(PortalDetails.EvidenceReferences, refsValue(row.drillInRefs)),
      detail(PortalDetails.InterventionAuditId, refsValue(row.auditRefs)),
      detail(PortalDetails.ProductClaim, Copy.ProductClaim),
    ],
  };
}

function detail(label: DisplayText, value: unknown): SocialAlertReportParentSurfacePanelDetail {
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
