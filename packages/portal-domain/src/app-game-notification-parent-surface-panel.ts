import {
  AppGameNotificationParentSurfaceIntentReadModelSchema,
  type AppGameNotificationParentSurfaceIntentReadModel,
  type AppGameNotificationParentSurfaceIntentRow,
} from '@ocentra-parent/app-game-domain/app-game-notification-parent-surface-intent';
import { type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalDetails } from './details';

export type AppGameNotificationParentSurfaceDetail = {
  readonly label: DisplayText;
  readonly value: PortalDetailValue | DisplayText;
};

export type AppGameNotificationParentSurfacePanelRow = {
  readonly key: PortalDetailValue;
  readonly title: PortalDetailValue;
  readonly details: readonly AppGameNotificationParentSurfaceDetail[];
};

export type AppGameNotificationParentSurfacePanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly state: PortalDetailValue;
  readonly summary: PortalDetailValue;
  readonly productClaim: DisplayText;
  readonly metrics: readonly AppGameNotificationParentSurfaceDetail[];
  readonly rows: readonly AppGameNotificationParentSurfacePanelRow[];
  readonly emptyMessage: DisplayText;
};

const AppGameNotificationParentSurfacePanelValues = {
  EmptyRowCount: '0',
  EmptyRowsSummary: '0 parent-surface intent rows',
  EmptyState: 'empty',
  NotReported: 'not reported',
  ProviderSeparator: ' / ',
  ReadyState: 'ready',
  RefsSeparator: ', ',
  RowsSummarySuffix: ' parent-surface intent rows',
  ServiceEventNotReported: 'service event not reported',
  UnavailableState: 'unavailable',
  ManualActionSuffix: ' manual action',
  HistoryRowsSuffix: ' history rows',
  PreferenceSetupSuffix: ' preference setup',
} as const;

export function createAppGameNotificationParentSurfacePanelIntent(
  readModelInput: unknown
): AppGameNotificationParentSurfacePanelIntent {
  const parsed = AppGameNotificationParentSurfaceIntentReadModelSchema.safeParse(readModelInput);
  if (!parsed.success) {
    return emptyPanelIntent();
  }

  return populatedPanelIntent(parsed.data);
}

function populatedPanelIntent(
  readModel: AppGameNotificationParentSurfaceIntentReadModel
): AppGameNotificationParentSurfacePanelIntent {
  return {
    eyebrow: resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurface),
    title: resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurface),
    body: resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurfaceBody),
    state: detailValue(
      readModel.rows.length > 0
        ? AppGameNotificationParentSurfacePanelValues.ReadyState
        : AppGameNotificationParentSurfacePanelValues.EmptyState
    ),
    summary: detailValue(String(readModel.rows.length) + AppGameNotificationParentSurfacePanelValues.RowsSummarySuffix),
    productClaim: resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurfaceNoRuntimeClaim),
    metrics: [
      detail(PortalDetails.RowsReturned, String(readModel.rows.length)),
      detail(
        PortalDetails.Status,
        String(readModel.manualActionRequiredCount) + AppGameNotificationParentSurfacePanelValues.ManualActionSuffix
      ),
      detail(
        PortalDetails.HistoryVisibility,
        String(readModel.historyVisibleCount) + AppGameNotificationParentSurfacePanelValues.HistoryRowsSuffix
      ),
      detail(
        PortalDetails.Capability,
        String(readModel.preferenceSetupRequiredCount) +
          AppGameNotificationParentSurfacePanelValues.PreferenceSetupSuffix
      ),
      detail(PortalDetails.RuntimeReference, readModel.intentId),
      detail(PortalDetails.GeneratedAt, readModel.generatedAt),
    ],
    rows: readModel.rows.map(panelRow),
    emptyMessage: resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurfaceNoData),
  };
}

function emptyPanelIntent(): AppGameNotificationParentSurfacePanelIntent {
  return {
    eyebrow: resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurface),
    title: resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurface),
    body: resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurfaceBody),
    state: detailValue(AppGameNotificationParentSurfacePanelValues.UnavailableState),
    summary: detailValue(AppGameNotificationParentSurfacePanelValues.EmptyRowsSummary),
    productClaim: resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurfaceNoRuntimeClaim),
    metrics: [
      detail(PortalDetails.RowsReturned, AppGameNotificationParentSurfacePanelValues.EmptyRowCount),
      detail(PortalDetails.Status, AppGameNotificationParentSurfacePanelValues.NotReported),
      detail(PortalDetails.RuntimeReference, AppGameNotificationParentSurfacePanelValues.ServiceEventNotReported),
    ],
    rows: [],
    emptyMessage: resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurfaceNoData),
  };
}

function panelRow(row: AppGameNotificationParentSurfaceIntentRow): AppGameNotificationParentSurfacePanelRow {
  return {
    key: detailValue(row.surfaceRowId),
    title: detailValue(row.surfaceRowId),
    details: [
      detail(PortalDetails.Status, row.parentSurfaceStatus),
      detail(
        PortalDetails.Provider,
        row.providerChannel + AppGameNotificationParentSurfacePanelValues.ProviderSeparator + row.providerStatus
      ),
      detail(PortalDetails.HistoryVisibility, row.historyVisibility),
      detail(PortalDetails.Capability, row.preferenceVisibility),
      detail(PortalDetails.EvidenceReferences, refsValue(row.drillInRefs)),
      detail(PortalDetails.RuntimeReference, refsValue([row.sourceSchedulerEntryRef, row.sourceOutboxRecordRef])),
      detail(PortalDetails.MissingProof, refsValue(row.manualProofRequirements)),
      detail(
        PortalDetails.ProductClaim,
        resolvePortalDevText(PortalDevTextToken.AppGameNotificationParentSurfaceNoRuntimeClaim)
      ),
    ],
  };
}

function detail(label: DisplayText, value: unknown): AppGameNotificationParentSurfaceDetail {
  return {
    label,
    value: detailValue(value),
  };
}

function refsValue(values: readonly (string | null)[]): PortalDetailValue {
  const refs = values.filter((value): value is string => typeof value === 'string' && value.trim().length > 0);
  return detailValue(
    refs.length > 0
      ? refs.join(AppGameNotificationParentSurfacePanelValues.RefsSeparator)
      : AppGameNotificationParentSurfacePanelValues.NotReported
  );
}

function detailValue(value: unknown): PortalDetailValue {
  const text =
    typeof value === 'string' && value.trim().length > 0
      ? value
      : AppGameNotificationParentSurfacePanelValues.NotReported;
  return decodePortalDetailValue(text);
}
