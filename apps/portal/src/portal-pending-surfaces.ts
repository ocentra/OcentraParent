import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { decodeParentPortalDetailValue } from '../generated/parent-ui-bridge';
import { appendDetail } from './detail-list';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';

export function renderPendingPanel(container: HTMLElement, titleText: PortalDisplayText): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = titleText;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(
    metadata,
    PortalDetails.Status,
    decodeParentPortalDetailValue(resolvePortalDevText(PortalDevTextToken.PendingTypedIntent))
  );
  appendDetail(
    metadata,
    PortalDetails.Capability,
    decodeParentPortalDetailValue(resolvePortalDevText(PortalDevTextToken.PendingServiceReadModel))
  );

  const note = document.createElement(PortalDom.Tags.Paragraph);
  note.className = PortalDom.Classes.ProductPanelNote;
  note.textContent = resolvePortalDevText(PortalDevTextToken.ProductSurfacePending);

  panel.append(title, metadata, note);
  container.append(panel);
}

export function productBadge(text: PortalDisplayText): HTMLElement {
  const badge = document.createElement(PortalDom.Tags.Span);
  badge.className = PortalDom.Classes.ProductBadge;
  badge.textContent = text;
  return badge;
}
