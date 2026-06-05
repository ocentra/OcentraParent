import type { ActivityScreenReadModel } from '@ocentra-parent/activity-domain/activity-surface';
import type { ActivitySurfaceAdapterResult } from '@ocentra-parent/agent-protocol-domain/activity-surface-adapter';
import {
  PortalDetails,
  PortalDom,
  PortalFormatting,
  PortalRoute,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import type { ReactElement } from 'react';
import { detailFromValue, eventStatus, notReported } from './event-detail-values';
import type { PortalLiveActivityState } from './live-activity-state';

export function shouldRenderScreenParentExplanationRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.Activity || route === PortalRoute.ScreenAnalysis;
}

export function ScreenParentExplanationRoutePanel({
  liveActivity,
}: {
  readonly liveActivity: PortalLiveActivityState;
}): ReactElement {
  const readModel = screenReadModel(liveActivity.activityScreenReadModel);
  const row = readModel?.rows[0] ?? null;
  return (
    <section
      aria-label={PortalText.Resolve(PortalTextToken.ScreenAnalysis)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{PortalText.Resolve(PortalTextToken.ScreenAnalysis)}</p>
          <h2>{PortalText.Resolve(PortalTextToken.ScreenAnalysis)}</h2>
          {row === null ? <p>{PortalText.Resolve(PortalTextToken.NoActivityStatus)}</p> : null}
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <ScreenParentExplanationSummaryCard liveActivity={liveActivity} readModel={readModel} />
          <ScreenParentExplanationRowCard readModel={readModel} />
        </div>
      </div>
    </section>
  );
}

function ScreenParentExplanationSummaryCard({
  liveActivity,
  readModel,
}: {
  readonly liveActivity: PortalLiveActivityState;
  readonly readModel: ActivityScreenReadModel | null;
}): ReactElement {
  return (
    <article className={screenParentExplanationCardClassName()}>
      <h2>{PortalDetails.Status}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenParentExplanationDetail
          label={PortalDetails.Status}
          value={eventStatus(liveActivity.activityScreenReadModelEvent)}
        />
        <ScreenParentExplanationDetail label={PortalDetails.LoadState} value={detailFromValue(readModel?.state)} />
        <ScreenParentExplanationDetail
          label={PortalDetails.RowsReturned}
          value={detailFromValue(readModel?.rows.length)}
        />
        <ScreenParentExplanationDetail
          label={PortalDetails.GeneratedAt}
          value={detailFromValue(readModel?.generatedAt)}
        />
        <ScreenParentExplanationDetail
          label={PortalDetails.EventId}
          value={detailFromValue(liveActivity.activityScreenReadModelEvent?.eventId)}
        />
      </dl>
    </article>
  );
}

function ScreenParentExplanationRowCard({
  readModel,
}: {
  readonly readModel: ActivityScreenReadModel | null;
}): ReactElement {
  const row = readModel?.rows[0] ?? null;
  return (
    <article className={screenParentExplanationCardClassName()}>
      <h2>{PortalDetails.LocalAiResult}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenParentExplanationDetail label={PortalDetails.EntryId} value={detailFromValue(row?.rowId)} />
        <ScreenParentExplanationDetail label={PortalDetails.Subject} value={detailFromValue(row?.label)} />
        <ScreenParentExplanationDetail label={PortalDetails.Device} value={detailFromValue(row?.deviceId)} />
        <ScreenParentExplanationDetail label={PortalDetails.ActivityKind} value={detailFromValue(row?.captureReason)} />
        <ScreenParentExplanationDetail label={PortalDetails.TargetType} value={detailFromValue(row?.captureScope)} />
        <ScreenParentExplanationDetail label={PortalDetails.Provider} value={detailFromValue(row?.providerKind)} />
        <ScreenParentExplanationDetail label={PortalDetails.Model} value={detailFromValue(row?.modelId)} />
        <ScreenParentExplanationDetail
          label={PortalDetails.Version}
          value={detailFromValue(row?.promptOrTemplateVersion)}
        />
        <ScreenParentExplanationDetail label={PortalDetails.Level} value={detailFromValue(row?.confidence)} />
        <ScreenParentExplanationDetail
          label={PortalDetails.ProductClaim}
          value={detailFromValue(row?.primaryCategory)}
        />
        <ScreenParentExplanationDetail
          label={PortalDetails.DecisionId}
          value={detailFromValue(row?.policyDecisionRef)}
        />
        <ScreenParentExplanationDetail
          label={PortalDetails.DecisionAction}
          value={detailFromValue(row?.policyAction)}
        />
        <ScreenParentExplanationDetail label={PortalDetails.RuleIds} value={detailList(row?.parentRuleRefs)} />
        <ScreenParentExplanationDetail label={PortalDetails.ReasonCodes} value={detailList(row?.policyReasonCodes)} />
        <ScreenParentExplanationDetail
          label={PortalDetails.LocalAiResult}
          value={detailList(row?.parentExplanationRefs)}
        />
        <ScreenParentExplanationDetail label={PortalDetails.Reason} value={detailList(row?.explanationReasons)} />
        <ScreenParentExplanationDetail
          label={PortalDetails.RuntimeReference}
          value={detailList(row?.localModelRuntimeRefs)}
        />
        <ScreenParentExplanationDetail
          label={PortalDetails.DeletedEvidence}
          value={detailFromValue(row?.imageDeletionState)}
        />
        <ScreenParentExplanationDetail label={PortalDetails.Custody} value={detailFromValue(row?.custodyState)} />
        <ScreenParentExplanationDetail label={PortalDetails.DeletedEvidence} value={detailList(row?.deletionReasons)} />
        <ScreenParentExplanationDetail
          label={PortalDetails.PolicyPreview}
          value={detailFromValue(row?.policyEligible)}
        />
        <ScreenParentExplanationDetail
          label={PortalDetails.EvidenceReferences}
          value={detailFromValue(row?.evidence.length)}
        />
        <ScreenParentExplanationDetail label={PortalDetails.Source} value={detailFromValue(row?.queueJobId)} />
        <ScreenParentExplanationDetail
          label={PortalDetails.HistoryVisibility}
          value={detailFromValue(row?.rawImageRetained)}
        />
      </dl>
    </article>
  );
}

function screenReadModel(result: ActivitySurfaceAdapterResult<unknown> | null): ActivityScreenReadModel | null {
  if (result?.ok !== true) {
    return null;
  }
  return result.value as ActivityScreenReadModel;
}

function detailList(values: readonly string[] | undefined): PortalDetailValue {
  if (values === undefined || values.length === 0) {
    return notReported();
  }
  return decodePortalDetailValue(values.join(PortalFormatting.EventDetailSeparator));
}

function ScreenParentExplanationDetail({
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

function screenParentExplanationCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
