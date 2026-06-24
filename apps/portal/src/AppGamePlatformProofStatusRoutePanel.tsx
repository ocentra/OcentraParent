import type { ReactElement } from 'react';
import type {
  AppGamePlatformProofStatusReadModel,
  AppGamePlatformProofStatusRow,
} from '@ocentra-parent/schema-domain/app-game-platform-proof-status';
import { type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { isPortalAppGameParentSurfaceRoute } from '@ocentra-parent/portal-domain/routes';
import type { PortalRenderActions } from './portal-actions';
import {
  createAppGamePlatformProofStatusPanelIntent,
  type AppGamePlatformProofStatusPanelDetail,
  type AppGamePlatformProofStatusPanelIntent,
  type AppGamePlatformProofStatusPanelRow,
} from '@ocentra-parent/portal-domain/app-game-platform-proof-status-panel';

type PlatformProofStatusPanelReadModel = Exclude<
  Parameters<typeof createAppGamePlatformProofStatusPanelIntent>[0],
  null
>;
type AppGamePlatformProofStatusRouteReadModelResult =
  | {
      readonly ok: true;
      readonly value: AppGamePlatformProofStatusReadModel;
    }
  | {
      readonly ok: false;
    };

export function shouldRenderAppGamePlatformProofStatusRoute(route: PortalRouteValue): boolean {
  return isPortalAppGameParentSurfaceRoute(route);
}

export function AppGamePlatformProofStatusRoutePanel({
  actions,
  commandEnabled,
  readModelResult,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly readModelResult: AppGamePlatformProofStatusRouteReadModelResult | null;
}): ReactElement {
  const readModel =
    readModelResult !== null && readModelResult.ok
      ? normalizeAppGamePlatformProofStatusReadModel(readModelResult.value)
      : null;
  const intent = createAppGamePlatformProofStatusPanelIntent(readModel);
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.GetActivityAppGamePlatformProofStatusReadModel)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{intent.eyebrow}</p>
          <h2>{intent.title}</h2>
          <p>{intent.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => void actions.refreshRouteSnapshot?.()}
          >
            {resolvePortalDevText(PortalDevTextToken.GetActivityAppGamePlatformProofStatusReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGamePlatformProofStatusSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <AppGamePlatformProofStatusEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row, index) => (
              <AppGamePlatformProofStatusRowCard key={`${String(row.title)}:${index}`} row={row} />
            ))
          )}
        </div>
      </div>
    </section>
  );
}

function AppGamePlatformProofStatusSummaryCard({
  intent,
}: {
  readonly intent: AppGamePlatformProofStatusPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{PortalDetails.PlatformState}</h2>
      <AppGamePlatformProofStatusDetails details={intent.summaryDetails} />
    </article>
  );
}

function AppGamePlatformProofStatusEmptyCard({
  intent,
}: {
  readonly intent: AppGamePlatformProofStatusPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{intent.loadState}</h2>
      <p>{intent.emptyMessage}</p>
      <AppGamePlatformProofStatusDetails
        details={[
          {
            label: PortalDetails.ProductClaim,
            value: intent.productClaim,
          },
        ]}
      />
    </article>
  );
}

function AppGamePlatformProofStatusRowCard({
  row,
}: {
  readonly row: AppGamePlatformProofStatusPanelRow;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{row.title}</h2>
      <AppGamePlatformProofStatusDetails details={row.details} />
    </article>
  );
}

function AppGamePlatformProofStatusDetails({
  details,
}: {
  readonly details: readonly AppGamePlatformProofStatusPanelDetail[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <AppGamePlatformProofStatusDetail
          key={`${String(detail.label)}:${index}`}
          label={detail.label}
          value={detail.value}
        />
      ))}
    </dl>
  );
}

function AppGamePlatformProofStatusDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: PortalDisplayText;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

export function normalizeAppGamePlatformProofStatusReadModel(
  readModel: AppGamePlatformProofStatusReadModel
): PlatformProofStatusPanelReadModel {
  const rows = readModel.rows.map(normalizeAppGamePlatformProofStatusRow);
  return {
    generatedAt: readModel.generatedAt,
    returned: readModel.platformProofObservedCount,
    hostVisibleCount: rows.filter((row) => row.hostCapabilityState === 'available').length,
    hostNotDetectedCount: rows.filter((row) => row.hostCapabilityState === 'not-detected').length,
    localRuntimeNotApplicableCount: rows.filter((row) => row.hostCapabilityState === 'not-applicable').length,
    enforcementReadyCount: readModel.enforcementReadyCount,
    openGapCount: readModel.openGapCount,
    rows,
  };
}

function normalizeAppGamePlatformProofStatusRow(
  row: AppGamePlatformProofStatusRow
): PlatformProofStatusPanelReadModel['rows'][number] {
  return {
    platform: row.platform,
    proofState: row.proofState,
    authorityState: row.authorityState,
    hostCapabilityState: deriveHostCapabilityState(row),
    hostCapabilityEvidenceRefs: row.packageVisibilityCount > 0 || row.runtimeVisibilityCount > 0 ? row.proofRefs : [],
    hostCapabilityProbeRefs: [],
    adapterDispatchClaimed: row.adapterDispatchClaimed,
    broadInstalledAppBlockingClaimed: row.broadBlockingClaimed,
    platformEnforcementClaimed: row.platformEnforcementClaimed,
    providerDeliveryClaimed: false,
    childDeliveryClaimed: row.childDeliveryClaimed,
    privateDiagnosticsClaimed: row.auditProofAttached,
    proofRefs: row.proofRefs,
    openGaps: row.openGaps,
  };
}

function deriveHostCapabilityState(
  row: AppGamePlatformProofStatusRow
): 'available' | 'not-detected' | 'not-applicable' {
  if (row.proofState === 'apple-ci-artifacts-required') {
    return 'not-applicable';
  }
  if (row.packageVisibilityCount > 0 || row.runtimeVisibilityCount > 0) {
    return 'available';
  }
  return 'not-detected';
}
