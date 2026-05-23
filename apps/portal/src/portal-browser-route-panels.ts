import {
  PortalDetails,
  PortalDom,
  PortalReadableValues,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import { eventStatus, notReported } from './event-detail-values';
import type { PortalLiveActivityState } from './live-activity-state';

type ProductStatusCardVariant =
  | typeof PortalDom.Classes.ProductStatusCardManaged
  | typeof PortalDom.Classes.ProductStatusCardEvidence
  | typeof PortalDom.Classes.ProductStatusCardProtection;

interface ProductStatusCardDetail {
  readonly label: PortalDisplayText;
  readonly value: PortalDetailValue;
}

export function renderBrowserStatusSummary(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const status = eventStatus(liveActivity.browserManagedEvent);
  container.append(
    productStatusCard(
      PortalText.Resolve(PortalTextToken.BrowserManagedStatus),
      bodyText(PortalTextToken.NoBrowserManagedStatus),
      PortalDom.Classes.ProductStatusCardManaged,
      readableBadge(status),
      [
        { label: PortalDetails.Status, value: status },
        { label: PortalDetails.ManagedState, value: detail(liveActivity.browserManagedStatus?.managedState) },
        { label: PortalDetails.Capability, value: detail(liveActivity.browserManagedStatus?.capabilityStatus) },
        { label: PortalDetails.BrowserFamily, value: detail(liveActivity.browserManagedStatus?.browserFamily) },
      ]
    )
  );
}

export function renderBrowserEvidenceSummary(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const status = eventStatus(liveActivity.browserEvidenceEvent);
  const readModel = liveActivity.browserEvidenceReadModel;
  const latestRow = readModel?.rows[0] ?? null;

  container.append(
    productStatusCard(
      PortalText.Resolve(PortalTextToken.BrowserEvidence),
      bodyText(PortalTextToken.NoBrowserEvidence),
      PortalDom.Classes.ProductStatusCardEvidence,
      readableBadge(status),
      [
        { label: PortalDetails.Status, value: status },
        { label: PortalDetails.RowsReturned, value: detail(readModel?.returned) },
        { label: PortalDetails.Domain, value: detail(latestRow?.domain) },
        { label: PortalDetails.Custody, value: detail(readModel?.custodyLabel) },
      ]
    )
  );
}

export function renderBrowserProtectionSummary(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const status = eventStatus(liveActivity.browserInterventionEvent);
  const readModel = liveActivity.browserInterventionReadModel;
  const latestRow = readModel?.rows[0] ?? null;

  container.append(
    productStatusCard(
      PortalText.Resolve(PortalTextToken.BrowserIntervention),
      bodyText(PortalTextToken.NoBrowserIntervention),
      PortalDom.Classes.ProductStatusCardProtection,
      readableBadge(status),
      [
        { label: PortalDetails.Status, value: status },
        { label: PortalDetails.RowsReturned, value: detail(readModel?.returned) },
        {
          label: PortalDetails.ManagedSessionIntervention,
          value: detail(readModel?.managedSessionInterventionCapability),
        },
        {
          label: PortalDetails.InterventionAction,
          value: detail(latestRow?.interventionAction),
        },
      ]
    )
  );
}

function productStatusCard(
  titleText: PortalDisplayText,
  body: PortalDisplayText,
  variantClass: ProductStatusCardVariant,
  badgeText: PortalDetailValue,
  details: readonly ProductStatusCardDetail[]
): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard, variantClass].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const media = document.createElement(PortalDom.Tags.Division);
  media.className = PortalDom.Classes.ProductStatusCardMedia;
  media.setAttribute(PortalDom.Attributes.AriaHidden, PortalDom.Attributes.True);

  const badge = document.createElement(PortalDom.Tags.Span);
  badge.className = PortalDom.Classes.ProductStatusCardBadge;
  badge.textContent = badgeText;
  media.append(badge);

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = titleText;

  const bodyCopy = document.createElement(PortalDom.Tags.Paragraph);
  bodyCopy.className = PortalDom.Classes.ProductStatusCardBody;
  bodyCopy.textContent = body;

  panel.append(media, title, bodyCopy, metadata(details));
  return panel;
}

function metadata(details: readonly ProductStatusCardDetail[]): HTMLElement {
  const grid = document.createElement(PortalDom.Tags.Division);
  grid.className = PortalDom.Classes.ProductStatusCardMeta;
  for (const item of details) {
    grid.append(metadataItem(item));
  }
  return grid;
}

function metadataItem(item: ProductStatusCardDetail): HTMLElement {
  const wrapper = document.createElement(PortalDom.Tags.Division);
  wrapper.className = PortalDom.Classes.ProductStatusCardMetaItem;

  const label = document.createElement(PortalDom.Tags.Span);
  label.className = PortalDom.Classes.ProductStatusCardMetaLabel;
  label.textContent = item.label;

  const value = document.createElement(PortalDom.Tags.Strong);
  value.className = PortalDom.Classes.ProductStatusCardMetaValue;
  value.textContent = item.value;

  wrapper.append(label, value);
  return wrapper;
}

function bodyText(token: (typeof PortalTextToken)[keyof typeof PortalTextToken]): PortalDisplayText {
  return PortalText.Resolve(token);
}

function readableBadge(value: PortalDetailValue): PortalDetailValue {
  const readableValue = PortalReadableValues[String(value)];
  if (readableValue !== undefined) {
    return decodePortalDetailValue(readableValue);
  }
  return value;
}

function detail(value: unknown): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  const readableValue = PortalReadableValues[String(value)];
  if (readableValue !== undefined) {
    return decodePortalDetailValue(readableValue);
  }
  return decodePortalDetailValue(String(value));
}
