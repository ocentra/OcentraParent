import type { ReactElement } from 'react';
import {
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/schema-domain/portal-contracts';
import { decodeDisplayText, type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  networkEvidenceDrawerSummary,
  type NetworkEvidenceDrawerSummary,
} from '@ocentra-parent/portal-domain/network-evidence-drawer';
import {
  isPortalInlineNetworkEvidenceDrawerRoute,
  isPortalNetworkEvidenceDrawerRoute,
} from '@ocentra-parent/portal-domain/routes';
import type { PortalLiveActivityState } from './live-activity-state';

export function shouldRenderNetworkEvidenceDrawerRoute(route: PortalRouteValue): boolean {
  return isPortalNetworkEvidenceDrawerRoute(route);
}

export function NetworkEvidenceDrawerRoutePanel({
  liveActivity,
  route,
}: {
  readonly liveActivity: PortalLiveActivityState;
  readonly route: PortalRouteValue;
}): ReactElement {
  const summary = networkEvidenceDrawerSummary(liveActivity.networkFlowReadModel, {
    networkFlowEventPayload: liveActivity.networkFlowEvent?.payload ?? null,
    policyPreviewReadModel: liveActivity.policyPreviewReadModel,
    networkRuntimeEventChainStream: liveActivity.networkRuntimeEventChainStream,
  });
  const lanSourceMatrix = projectLanDiscoverySourceMatrixViewModel(liveActivity.lanAddDeviceReadModel);
  const inlineOnActivityRoute = isPortalInlineNetworkEvidenceDrawerRoute(route);
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.NetworkFlow)}
      className={PortalDom.Classes.TrackingStatusOverlay}
      style={inlineOnActivityRoute ? inlineActivityRoutePanelStyle : undefined}
    >
      <div
        className={PortalDom.Classes.TrackingStatusOverlayContent}
        style={inlineOnActivityRoute ? inlineActivityRoutePanelContentStyle : undefined}
      >
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{resolvePortalDevText(PortalDevTextToken.NetworkFlow)}</p>
          <h2>{resolvePortalDevText(PortalDevTextToken.NetworkFlow)}</h2>
          {liveActivity.networkFlowReadModel === null ? (
            <p>{resolvePortalDevText(PortalDevTextToken.NoNetworkFlow)}</p>
          ) : null}
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <NetworkEvidenceDrawerCard summary={summary} />
          <NetworkEvidenceUnsupportedClaimCard summary={summary} />
          <NetworkEvidenceLanSourceMatrixCard matrix={lanSourceMatrix} />
        </div>
      </div>
    </section>
  );
}

function NetworkEvidenceDrawerCard({ summary }: { readonly summary: NetworkEvidenceDrawerSummary }): ReactElement {
  return (
    <article className={networkEvidenceDrawerCardClassName()}>
      <h2>{resolvePortalDevText(PortalDevTextToken.NetworkFlow)}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <NetworkEvidenceDrawerDetail label={PortalDetails.EventId} value={summary.evidenceId} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.LastObserved} value={summary.observedAt} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.FirstObserved} value={summary.firstSeenAt} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.LastChecked} value={summary.lastSeenAt} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Device} value={summary.deviceRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Profile} value={summary.childProfileRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Source} value={summary.sourceAdapter} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Capability} value={summary.sourceQuality} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.PlatformState} value={summary.platformState} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ReadModelRows} value={summary.readModelRows} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Destination} value={summary.remoteEndpoint} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.NetworkProtocol} value={summary.protocolCandidate} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.TcpState} value={summary.applicationProtocolCandidate} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Process} value={summary.processRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Domain} value={summary.domainEvidenceRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Connections} value={summary.byteSummary} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.EvidenceReferences} value={summary.evidenceReferences} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ReasonCodes} value={summary.uncertaintyReasonCodes} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Custody} value={summary.custody} />
      </dl>
    </article>
  );
}

function NetworkEvidenceUnsupportedClaimCard({
  summary,
}: {
  readonly summary: NetworkEvidenceDrawerSummary;
}): ReactElement {
  return (
    <article className={networkEvidenceDrawerCardClassName()}>
      <h2>{PortalDetails.MissingProof}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <NetworkEvidenceDrawerDetail label={PortalDetails.BrowserEvidence} value={summary.browserRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.ExactUrlClaim} value={summary.exactUrlClaim} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.AnalyzerAlerts} value={summary.analyzerAlertRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.DetectionResults} value={summary.detectionResultRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.LocalAiResult} value={summary.aiAuditRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.RiskBudget} value={summary.riskBudgetRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.PolicyPreview} value={summary.policyDecisionRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.EnforcementHandoff} value={summary.interventionResultRef} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.DeletedEvidence} value={summary.retentionState} />
        <NetworkEvidenceDrawerDetail
          label={PortalDetails.DeletedEvidenceReferences}
          value={summary.deletedEvidenceReferences}
        />
        <NetworkEvidenceDrawerDetail label={PortalDetails.PerformanceState} value={summary.degradedState} />
        <NetworkEvidenceDrawerDetail label={PortalDetails.Level} value={summary.evidenceGrade} />
      </dl>
    </article>
  );
}

function NetworkEvidenceLanSourceMatrixCard({
  matrix,
}: {
  readonly matrix: LanDiscoverySourceMatrixViewModel | null;
}): ReactElement {
  return (
    <article className={networkEvidenceDrawerCardClassName()}>
      <h2>{LAN_SOURCE_MATRIX_TEXT.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <NetworkEvidenceDrawerDetail label={PortalDetails.GeneratedAt} value={detailFromValue(matrix?.generatedAt)} />
        <NetworkEvidenceDrawerDetail label={LAN_SOURCE_MATRIX_TEXT.matrixCoverage} value={detailFromValue(matrix?.rowSummary)} />
        <NetworkEvidenceDrawerDetail label={LAN_SOURCE_MATRIX_TEXT.statusMix} value={detailFromValue(matrix?.statusSummary)} />
        <NetworkEvidenceDrawerDetail
          label={LAN_SOURCE_MATRIX_TEXT.currentScanSources}
          value={detailFromValue(matrix?.currentSourceSummary)}
        />
        <NetworkEvidenceDrawerDetail
          label={LAN_SOURCE_MATRIX_TEXT.restartPersistedSources}
          value={detailFromValue(matrix?.persistedSourceSummary)}
        />
        <NetworkEvidenceDrawerDetail label={LAN_SOURCE_MATRIX_TEXT.weakFenceSummary} value={detailFromValue(matrix?.fenceSummary)} />
        <NetworkEvidenceDrawerDetail label={LAN_SOURCE_MATRIX_TEXT.claimsProved} value={detailFromValue(matrix?.claimsProved)} />
        <NetworkEvidenceDrawerDetail label={LAN_SOURCE_MATRIX_TEXT.claimsNotProved} value={detailFromValue(matrix?.claimsNotProved)} />
      </dl>
      <NetworkEvidenceDrawerRowSection title={LAN_SOURCE_MATRIX_TEXT.workpacks} rows={matrix?.workpackRows ?? []} />
      <NetworkEvidenceDrawerRowSection
        title={LAN_SOURCE_MATRIX_TEXT.implementedSources}
        rows={matrix?.implementedSourceRows ?? []}
      />
      <NetworkEvidenceDrawerRowSection
        title={LAN_SOURCE_MATRIX_TEXT.weakSourceFences}
        rows={matrix?.weakSourceRows ?? []}
      />
    </article>
  );
}

function NetworkEvidenceDrawerRowSection({
  title,
  rows,
}: {
  readonly title: PortalDisplayText;
  readonly rows: readonly LanDiscoverySourceMatrixDisplayRow[];
}): ReactElement {
  return (
    <div style={networkEvidenceDrawerSectionStyle}>
      <p className={PortalDom.Classes.ProductEyebrow}>{title}</p>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {rows.length === 0 ? (
          <NetworkEvidenceDrawerDetail label={PortalDetails.Status} value={detailFromValue(undefined)} />
        ) : (
          rows.map((row) => (
            <NetworkEvidenceDrawerDetail key={`${row.label}-${row.value}`} label={detailLabel(row.label)} value={detailFromValue(row.value)} />
          ))
        )}
      </dl>
    </div>
  );
}

function NetworkEvidenceDrawerDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: PortalDetailValue;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function networkEvidenceDrawerCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}

export type LanDiscoverySourceMatrixViewModel = {
  readonly generatedAt: string | null;
  readonly rowSummary: string;
  readonly statusSummary: string;
  readonly currentSourceSummary: string;
  readonly persistedSourceSummary: string;
  readonly fenceSummary: string;
  readonly claimsProved: string;
  readonly claimsNotProved: string;
  readonly workpackRows: readonly LanDiscoverySourceMatrixDisplayRow[];
  readonly implementedSourceRows: readonly LanDiscoverySourceMatrixDisplayRow[];
  readonly weakSourceRows: readonly LanDiscoverySourceMatrixDisplayRow[];
};

type LanDiscoverySourceMatrixDisplayRow = {
  readonly label: string;
  readonly value: string;
};

type LanSourceMatrixReadModel = {
  readonly scanSummary: Pick<
    NonNullable<PortalLiveActivityState['lanAddDeviceReadModel']>['scanSummary'],
    'sourceLabels'
  >;
  readonly lanDiscoverySourceMatrix: LanDiscoverySourceMatrixRecord | null;
};

type PortalLanDiscoverySourceMatrix = NonNullable<
  NonNullable<PortalLiveActivityState['lanAddDeviceReadModel']>['lanDiscoverySourceMatrix']
>;
type PortalLanDiscoverySourceMatrixWorkpackRow = PortalLanDiscoverySourceMatrix['workpackRows'][number];
type PortalLanDiscoverySourceMatrixSourceRow = PortalLanDiscoverySourceMatrix['sourceRows'][number];

type LanDiscoverySourceMatrixRecord = {
  readonly generatedAt: string | null;
  readonly workpackRows: readonly LanDiscoverySourceMatrixWorkpackRow[];
  readonly sourceRows: readonly LanDiscoverySourceMatrixSourceRow[];
  readonly claimsProved: readonly string[];
  readonly claimsNotProved: readonly string[];
};
type LanDiscoverySourceMatrixWorkpackRow = Pick<
  PortalLanDiscoverySourceMatrixWorkpackRow,
  | 'workpackId'
  | 'title'
  | 'discoveryState'
  | 'proofState'
  | 'runtimeOwner'
  | 'status'
  | 'readModelVisible'
  | 'requiredArtifactSummary'
>;
type LanDiscoverySourceMatrixSourceRow = Pick<
  PortalLanDiscoverySourceMatrixSourceRow,
  | 'source'
  | 'workpackId'
  | 'status'
  | 'authority'
  | 'runtimePath'
  | 'uiSurface'
  | 'canConfirmChildAgent'
  | 'canAssignChildProfile'
  | 'canControlRoute'
  | 'requiresSelectedInterface'
  | 'persistsAcrossRestart'
  | 'evidenceLabel'
  | 'requiredArtifactSummary'
>;

export function projectLanDiscoverySourceMatrixViewModel(
  lanAddDeviceReadModel: LanSourceMatrixReadModel | null
): LanDiscoverySourceMatrixViewModel | null {
  if (lanAddDeviceReadModel === null || lanAddDeviceReadModel.lanDiscoverySourceMatrix === null) {
    return null;
  }
  const matrix = lanAddDeviceReadModel.lanDiscoverySourceMatrix;
  const visibleWorkpacks = matrix.workpackRows.filter((row) => row.readModelVisible);
  const implementedSourceRows = matrix.sourceRows.filter((row) => row.status === 'implemented');
  const weakSourceRows = matrix.sourceRows.filter((row) => WEAK_SOURCE_AUTHORITIES.has(row.authority));
  const restartPersistedSources = matrix.sourceRows.filter((row) => row.persistsAcrossRestart);
  const routeControlSources = matrix.sourceRows.filter((row) => row.canControlRoute);
  const statusCounts = countSourceRowsByStatus(matrix.sourceRows);

  return {
    generatedAt: matrix.generatedAt,
    rowSummary: joinTexts([`${visibleWorkpacks.length} workpacks`, `${matrix.sourceRows.length} sources`]),
    statusSummary: joinTexts([
      `${statusCounts.implemented} implemented`,
      `${statusCounts.partial} partial`,
      `${statusCounts.manualRequired} manual required`,
      `${statusCounts.notImplemented} not implemented`,
    ]),
    currentSourceSummary: joinTexts(lanAddDeviceReadModel.scanSummary.sourceLabels),
    persistedSourceSummary: joinTexts(
      restartPersistedSources.map((row) => `${row.source} (WP ${row.workpackId})`)
    ),
    fenceSummary: joinTexts([
      `${weakSourceRows.length} weak-source fences`,
      `${routeControlSources.length} route-control sources`,
    ]),
    claimsProved: joinTexts(matrix.claimsProved),
    claimsNotProved: joinTexts(matrix.claimsNotProved),
    workpackRows: visibleWorkpacks.map(projectWorkpackDisplayRow),
    implementedSourceRows: implementedSourceRows.map(projectImplementedSourceDisplayRow),
    weakSourceRows: weakSourceRows.map(projectWeakSourceDisplayRow),
  };
}

function projectWorkpackDisplayRow(row: LanDiscoverySourceMatrixWorkpackRow): LanDiscoverySourceMatrixDisplayRow {
  return {
    label: `WP ${row.workpackId}`,
    value: joinTexts([
      row.status,
      row.discoveryState,
      row.proofState,
      row.runtimeOwner,
      row.title,
      row.requiredArtifactSummary,
    ]),
  };
}

function projectImplementedSourceDisplayRow(row: LanDiscoverySourceMatrixSourceRow): LanDiscoverySourceMatrixDisplayRow {
  return {
    label: `${row.source} (WP ${row.workpackId})`,
    value: joinTexts([
      row.authority,
      row.runtimePath,
      row.uiSurface,
      booleanStateLabel(row.canControlRoute, 'route-control', 'no-route-control'),
      booleanStateLabel(row.canConfirmChildAgent, 'child-confirmed', 'child-not-confirmed'),
      booleanStateLabel(row.persistsAcrossRestart, 'restart-persisted', 'volatile'),
      row.evidenceLabel,
      row.requiredArtifactSummary,
    ]),
  };
}

function projectWeakSourceDisplayRow(row: LanDiscoverySourceMatrixSourceRow): LanDiscoverySourceMatrixDisplayRow {
  return {
    label: `${row.source} (WP ${row.workpackId})`,
    value: joinTexts([
      row.status,
      row.authority,
      row.runtimePath,
      booleanStateLabel(row.canControlRoute, 'route-control', 'no-route-control'),
      booleanStateLabel(row.canAssignChildProfile, 'profile-assignable', 'profile-not-assignable'),
      booleanStateLabel(row.requiresSelectedInterface, 'selected-interface', 'interface-not-required'),
      booleanStateLabel(row.persistsAcrossRestart, 'restart-persisted', 'volatile'),
      row.evidenceLabel,
      row.requiredArtifactSummary,
    ]),
  };
}

function countSourceRowsByStatus(
  rows: readonly LanDiscoverySourceMatrixSourceRow[]
): Record<'implemented' | 'partial' | 'manualRequired' | 'notImplemented', number> {
  return rows.reduce(
    (counts, row) => {
      if (row.status === 'implemented') {
        counts.implemented += 1;
      } else if (row.status === 'partial') {
        counts.partial += 1;
      } else if (row.status === 'manual-required') {
        counts.manualRequired += 1;
      } else if (row.status === 'not-implemented') {
        counts.notImplemented += 1;
      }
      return counts;
    },
    {
      implemented: 0,
      partial: 0,
      manualRequired: 0,
      notImplemented: 0,
    }
  );
}

function booleanStateLabel(value: boolean, truthy: string, falsy: string): string {
  return value ? truthy : falsy;
}

function detailLabel(label: string): PortalDisplayText {
  return decodeDisplayText(label);
}

function detailFromValue(value: string | null | undefined): PortalDetailValue {
  if (value === null || value === undefined || value.length === 0) {
    return decodePortalDetailValue(notReportedText());
  }
  return decodePortalDetailValue(value);
}

function joinTexts(values: readonly (string | null | undefined)[]): string {
  const normalized = values.filter((value): value is string => value !== null && value !== undefined && value.length > 0);
  if (normalized.length === 0) {
    return notReportedText();
  }
  return normalized.join(' | ');
}

function notReportedText(): string {
  return resolvePortalDevText(PortalDevTextToken.NotReported);
}

const LAN_SOURCE_MATRIX_TEXT = {
  title: decodeDisplayText('LAN source matrix'),
  matrixCoverage: decodeDisplayText('Matrix coverage'),
  statusMix: decodeDisplayText('Status mix'),
  currentScanSources: decodeDisplayText('Current scan sources'),
  restartPersistedSources: decodeDisplayText('Restart-persisted sources'),
  weakFenceSummary: decodeDisplayText('Weak-source fence summary'),
  claimsProved: decodeDisplayText('Claims proved'),
  claimsNotProved: decodeDisplayText('Claims not proved'),
  workpacks: decodeDisplayText('Workpack rows'),
  implementedSources: decodeDisplayText('Implemented source proof'),
  weakSourceFences: decodeDisplayText('Weak-source fences'),
} as const;

const WEAK_SOURCE_AUTHORITIES = new Set(['weak-identity', 'name-only', 'presence-only', 'no-child-confirmation']);

const inlineActivityRoutePanelStyle = {
  position: 'relative',
  inset: 'auto',
  zIndex: 'auto',
  marginTop: '16px',
  overflow: 'visible',
} as const;

const inlineActivityRoutePanelContentStyle = {
  height: 'auto',
} as const;

const networkEvidenceDrawerSectionStyle = {
  marginTop: '16px',
} as const;
