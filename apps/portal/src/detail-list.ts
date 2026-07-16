import { decodeParentPortalDetailValue, type ParentPortalDetailValue } from '../generated/parent-ui-bridge';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalFormatting } from '@ocentra-parent/portal-domain/formatting';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
export function appendDetail(list: HTMLDListElement, label: PortalDisplayText, value: ParentPortalDetailValue): void {
  const term = document.createElement(PortalDom.Tags.DefinitionTerm);
  term.textContent = label;

  const detail = document.createElement(PortalDom.Tags.DefinitionDescription);
  detail.textContent = value;

  list.append(term, detail);
}

export function portalDetailFromValue(value: unknown): ParentPortalDetailValue {
  if (value === undefined || value === null) {
    return notReportedDetail();
  }
  return decodeParentPortalDetailValue(String(value));
}

export function portalDetailFromSequence(values: readonly unknown[]): ParentPortalDetailValue {
  const normalizedValues = values.map((value) => String(value)).filter((value) => value.length > 0);
  if (normalizedValues.length === 0) {
    return notReportedDetail();
  }
  return portalDetailFromValue(normalizedValues.join(PortalFormatting.EventDetailSeparator));
}

export function notReportedDetail(): ParentPortalDetailValue {
  return decodeParentPortalDetailValue(resolvePortalDevText(PortalDevTextToken.NotReported));
}
