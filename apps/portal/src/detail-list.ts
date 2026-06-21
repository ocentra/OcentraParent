import {
  PortalDom,
  PortalText,
  PortalTextToken,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import { PortalFormatting } from '@ocentra-parent/portal-domain/formatting';
import { decodePortalDetailValue, type PortalDetailValue } from '@ocentra-parent/portal-domain/detail-values';

export function appendDetail(list: HTMLDListElement, label: PortalDisplayText, value: PortalDetailValue): void {
  const term = document.createElement(PortalDom.Tags.DefinitionTerm);
  term.textContent = label;

  const detail = document.createElement(PortalDom.Tags.DefinitionDescription);
  detail.textContent = value;

  list.append(term, detail);
}

export function portalDetailFromValue(value: unknown): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReportedDetail();
  }
  return decodePortalDetailValue(String(value));
}

export function portalDetailFromSequence(values: readonly unknown[]): PortalDetailValue {
  const normalizedValues = values.map((value) => String(value)).filter((value) => value.length > 0);
  if (normalizedValues.length === 0) {
    return notReportedDetail();
  }
  return portalDetailFromValue(normalizedValues.join(PortalFormatting.EventDetailSeparator));
}

export function notReportedDetail(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}
