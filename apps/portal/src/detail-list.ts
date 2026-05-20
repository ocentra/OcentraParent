import { PortalDom, type PortalDetailValue, type PortalDisplayText } from '@ocentra-parent/portal-domain/contracts';

export function appendDetail(list: HTMLDListElement, label: PortalDisplayText, value: PortalDetailValue): void {
  const term = document.createElement(PortalDom.Tags.DefinitionTerm);
  term.textContent = label;

  const detail = document.createElement(PortalDom.Tags.DefinitionDescription);
  detail.textContent = value;

  list.append(term, detail);
}
