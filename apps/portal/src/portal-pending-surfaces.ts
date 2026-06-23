import { decodePortalDetailValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { appendDetail } from './detail-list';

export function renderPendingPanel(container: HTMLElement, titleText: PortalDisplayText): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = titleText;

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(
    metadata,
    PortalDetails.Status,
    decodePortalDetailValue(resolvePortalDevText(PortalDevTextToken.PendingTypedIntent))
  );
  appendDetail(
    metadata,
    PortalDetails.Capability,
    decodePortalDetailValue(resolvePortalDevText(PortalDevTextToken.PendingServiceReadModel))
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
