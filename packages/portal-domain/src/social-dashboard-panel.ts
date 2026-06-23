import { SocialDashboardUxTextToken, resolveSocialDashboardUxText } from '@ocentra-parent/schema-domain/text-social-ux';
import {
  SocialDashboardUxSnapshotSchema,
  type SocialDashboardPanel,
  type SocialDashboardUxSnapshot,
} from '@ocentra-parent/schema-domain/social-dashboard-ux';
import { type DisplayText, decodeDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { decodePortalDetailValue, type PortalDetailValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { PortalDetails } from './details';

export type SocialDashboardPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText | PortalDetailValue;
};

export type SocialDashboardPanelRow = {
  readonly key: PortalDetailValue;
  readonly title: DisplayText;
  readonly details: readonly SocialDashboardPanelDetail[];
};

export type SocialDashboardPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly state: PortalDetailValue;
  readonly summary: PortalDetailValue;
  readonly productClaim: DisplayText;
  readonly metrics: readonly SocialDashboardPanelDetail[];
  readonly rows: readonly SocialDashboardPanelRow[];
  readonly emptyMessage: DisplayText;
};

const SocialDashboardPanelValues = {
  EmptyRowCount: '0',
  EmptyRowsSummary: '0 social dashboard rows',
  EmptyState: 'unavailable',
  NotReported: 'not reported',
  ReadyState: 'ready',
  RefsSeparator: ', ',
  RowsSummarySuffix: ' social dashboard rows',
} as const;

const SocialDashboardText = {
  Body: decodeDisplayText(
    'Schema-backed social rows show parent-review and manual-required status only; runtime fetch, connector, native app, policy execution, and enforcement remain unclaimed.'
  ),
  Empty: decodeDisplayText('No social dashboard snapshot has been reported yet.'),
  ProductClaim: decodeDisplayText(
    'Rendered parent surface only; social runtime data fetch, notifications, connector authorization, native app control, policy execution, and enforcement remain unclaimed.'
  ),
} as const;

export function createSocialDashboardPanelIntent(snapshotInput: unknown): SocialDashboardPanelIntent {
  const parsed = SocialDashboardUxSnapshotSchema.safeParse(snapshotInput);
  if (!parsed.success) {
    return emptyPanelIntent();
  }
  return populatedPanelIntent(parsed.data);
}

function populatedPanelIntent(snapshot: SocialDashboardUxSnapshot): SocialDashboardPanelIntent {
  return {
    eyebrow: resolveSocialDashboardUxText(SocialDashboardUxTextToken.Title),
    title: resolveSocialDashboardUxText(SocialDashboardUxTextToken.Title),
    body: SocialDashboardText.Body,
    state: detailValue(
      snapshot.panels.length > 0 ? SocialDashboardPanelValues.ReadyState : SocialDashboardPanelValues.EmptyState
    ),
    summary: detailValue(String(snapshot.panels.length) + SocialDashboardPanelValues.RowsSummarySuffix),
    productClaim: SocialDashboardText.ProductClaim,
    metrics: [
      detail(PortalDetails.RowsReturned, String(snapshot.panels.length)),
      detail(PortalDetails.GeneratedAt, snapshot.generatedAt),
      detail(PortalDetails.ProductClaim, SocialDashboardText.ProductClaim),
    ],
    rows: snapshot.panels.map(panelRow),
    emptyMessage: SocialDashboardText.Empty,
  };
}

function emptyPanelIntent(): SocialDashboardPanelIntent {
  return {
    eyebrow: resolveSocialDashboardUxText(SocialDashboardUxTextToken.Title),
    title: resolveSocialDashboardUxText(SocialDashboardUxTextToken.Title),
    body: SocialDashboardText.Body,
    state: detailValue(SocialDashboardPanelValues.EmptyState),
    summary: detailValue(SocialDashboardPanelValues.EmptyRowsSummary),
    productClaim: SocialDashboardText.ProductClaim,
    metrics: [
      detail(PortalDetails.RowsReturned, SocialDashboardPanelValues.EmptyRowCount),
      detail(PortalDetails.Status, SocialDashboardPanelValues.NotReported),
      detail(PortalDetails.ProductClaim, SocialDashboardText.ProductClaim),
    ],
    rows: [],
    emptyMessage: SocialDashboardText.Empty,
  };
}

function panelRow(panel: SocialDashboardPanel): SocialDashboardPanelRow {
  return {
    key: detailValue(panel.panelId),
    title: panelTitle(panel),
    details: [
      detail(PortalDetails.Status, panelStatus(panel.status)),
      detail(PortalDetails.Capability, panel.primaryAction),
      detail(PortalDetails.EvidenceReferences, refsValue(panel.sourceEvidenceRefs)),
      detail(PortalDetails.Reason, refsValue(panel.reasons)),
      detail(PortalDetails.ProductClaim, SocialDashboardText.ProductClaim),
    ],
  };
}

function panelTitle(panel: SocialDashboardPanel): DisplayText {
  if (panel.panelKind === 'account-approval-queue') {
    return resolveSocialDashboardUxText(SocialDashboardUxTextToken.AccountApprovals);
  }
  if (panel.panelKind === 'feed-video-gates') {
    return resolveSocialDashboardUxText(SocialDashboardUxTextToken.FeedVideoGates);
  }
  if (panel.panelKind === 'native-app-capability') {
    return resolveSocialDashboardUxText(SocialDashboardUxTextToken.NativeAppCapability);
  }
  if (panel.panelKind === 'connector-boundaries') {
    return resolveSocialDashboardUxText(SocialDashboardUxTextToken.ConnectorBoundaries);
  }
  if (panel.panelKind === 'decision-memory') {
    return resolveSocialDashboardUxText(SocialDashboardUxTextToken.DecisionMemory);
  }
  if (panel.panelKind === 'settings-custody') {
    return resolveSocialDashboardUxText(SocialDashboardUxTextToken.SettingsCustody);
  }
  return resolveSocialDashboardUxText(SocialDashboardUxTextToken.ManualRequiredGaps);
}

function panelStatus(status: SocialDashboardPanel['status']): DisplayText {
  if (status === 'ready-for-review') {
    return resolveSocialDashboardUxText(SocialDashboardUxTextToken.ReadyForReviewStatus);
  }
  if (status === 'manual-required') {
    return resolveSocialDashboardUxText(SocialDashboardUxTextToken.ManualRequiredStatus);
  }
  if (status === 'contract-only') {
    return resolveSocialDashboardUxText(SocialDashboardUxTextToken.ContractOnlyStatus);
  }
  return resolveSocialDashboardUxText(SocialDashboardUxTextToken.UnavailableStatus);
}

function detail(label: DisplayText, value: unknown): SocialDashboardPanelDetail {
  return {
    label,
    value: detailValue(value),
  };
}

function refsValue(values: readonly unknown[]): PortalDetailValue {
  const refs = values.map((value) => String(value).trim()).filter((value) => value.length > 0);
  return detailValue(
    refs.length > 0 ? refs.join(SocialDashboardPanelValues.RefsSeparator) : SocialDashboardPanelValues.NotReported
  );
}

function detailValue(value: unknown): PortalDetailValue {
  const text = typeof value === 'string' && value.trim().length > 0 ? value : SocialDashboardPanelValues.NotReported;
  return decodePortalDetailValue(text);
}
