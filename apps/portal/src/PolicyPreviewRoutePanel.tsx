import { useState, type ReactElement } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  isParentPolicyPreviewRoute,
  ParentUiActionPayloadField,
  type ParentPolicyPreviewPanelCardSnapshot,
  type ParentPolicyPreviewPanelDetailSnapshot,
  type ParentPolicyPreviewPanelSnapshot,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';

export function shouldRenderPolicyPreviewRoute(route: ParentRouteId): boolean {
  return isParentPolicyPreviewRoute(route);
}

export function PolicyPreviewRoutePanel({
  actions,
  commandEnabled,
  panel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly panel: ParentPolicyPreviewPanelSnapshot | null;
}): ReactElement {
  if (panel === null) {
    return <></>;
  }
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.PolicyPreview)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{PortalDetails.PolicyPreview}</p>
          <h2>{panel.title}</h2>
          <p>{panel.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => void actions.refreshRouteSnapshot?.()}
          >
            {resolvePortalDevText(PortalDevTextToken.GetPolicyPreviewReadModel)}
          </button>
        </header>
        <PolicyPreviewAuthoringSurface actions={actions} commandEnabled={commandEnabled} authoring={panel.authoring} />
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <PolicyPreviewSummaryCard panel={panel} />
          {panel.cards.length === 0 ? (
            <PolicyPreviewEmptyCard panel={panel} />
          ) : (
            panel.cards.map((card, index) => <PolicyPreviewCard key={`${String(card.title)}:${index}`} card={card} />)
          )}
        </div>
      </div>
    </section>
  );
}

function PolicyPreviewAuthoringSurface({
  actions,
  authoring,
  commandEnabled,
}: {
  readonly actions: PortalRenderActions;
  readonly authoring: ParentPolicyPreviewPanelSnapshot['authoring'];
  readonly commandEnabled: boolean;
}): ReactElement | null {
  if (authoring === null || authoring === undefined) {
    return null;
  }

  return <PolicyPreviewAuthoringForm actions={actions} authoring={authoring} commandEnabled={commandEnabled} />;
}

function PolicyPreviewAuthoringForm({
  actions,
  authoring,
  commandEnabled,
}: {
  readonly actions: PortalRenderActions;
  readonly authoring: NonNullable<ParentPolicyPreviewPanelSnapshot['authoring']>;
  readonly commandEnabled: boolean;
}): ReactElement {
  const [targetValue, setTargetValue] = useState(authoring.targetValue);
  const [requestedAction, setRequestedAction] = useState(authoring.requestedAction);

  const cancelDraft = (): void => {
    setTargetValue(authoring.targetValue);
    setRequestedAction(authoring.requestedAction);
    void actions.refreshRouteSnapshot?.();
  };

  const confirmDraft = (): void => {
    if (!authoring.confirmAction || targetValue.trim().length === 0) {
      return;
    }
    void actions.requestPolicyRequestAssistantPreviewConfirm?.({
      ...authoring.confirmAction.payload,
      [ParentUiActionPayloadField.PolicyRequestAssistantPreviewConfirmRequest]: JSON.stringify({
        targetValue,
        requestedAction,
      }),
    });
  };

  return (
    <form
      aria-label={authoring.confirmAction?.label ?? authoring.cancelAction.label}
      className={PortalDom.Classes.ProductDashboard}
      onSubmit={(event) => {
        event.preventDefault();
        confirmDraft();
      }}
    >
      <label>
        {PortalDetails.TargetValue}
        <input
          disabled={!commandEnabled}
          value={targetValue}
          onChange={(event) => setTargetValue(event.currentTarget.value)}
        />
      </label>
      <label>
        {PortalDetails.NetworkRequestedPolicyAction}
        <input
          disabled={!commandEnabled}
          value={requestedAction}
          onChange={(event) => setRequestedAction(event.currentTarget.value)}
        />
      </label>
      <div>
        {authoring.confirmAction ? (
          <button disabled={!commandEnabled || targetValue.trim().length === 0} type={PortalDom.ButtonType.Submit}>
            {authoring.confirmAction.label}
          </button>
        ) : null}
        <button disabled={!commandEnabled} type={PortalDom.ButtonType.Button} onClick={cancelDraft}>
          {authoring.cancelAction.label}
        </button>
      </div>
    </form>
  );
}

function PolicyPreviewSummaryCard({ panel }: { readonly panel: ParentPolicyPreviewPanelSnapshot }): ReactElement {
  return (
    <article className={policyPreviewCardClassName()}>
      <h2>{PortalDetails.PolicyPreview}</h2>
      <p>{panel.summary}</p>
      <PolicyPreviewDetails details={panel.summaryDetails} />
    </article>
  );
}

function PolicyPreviewEmptyCard({ panel }: { readonly panel: ParentPolicyPreviewPanelSnapshot }): ReactElement {
  return (
    <article className={policyPreviewCardClassName()}>
      <h2>{PortalDetails.Status}</h2>
      <p>{panel.emptyMessage}</p>
      <PolicyPreviewDetails
        details={[
          {
            label: PortalDetails.ProductClaim,
            value: panel.productClaim,
          },
        ]}
      />
    </article>
  );
}

function PolicyPreviewCard({ card }: { readonly card: ParentPolicyPreviewPanelCardSnapshot }): ReactElement {
  return (
    <article className={policyPreviewCardClassName()}>
      <h2>{card.title}</h2>
      <p>{card.summary}</p>
      <PolicyPreviewDetails details={card.details} />
    </article>
  );
}

function PolicyPreviewDetails({
  details,
}: {
  readonly details: readonly ParentPolicyPreviewPanelDetailSnapshot[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <PolicyPreviewDetail key={`${String(detail.label)}:${index}`} label={detail.label} value={detail.value} />
      ))}
    </dl>
  );
}

function PolicyPreviewDetail({ label, value }: { readonly label: string; readonly value: string }): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function policyPreviewCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
