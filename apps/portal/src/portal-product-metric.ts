import { PortalDom, type PortalDetailValue, type PortalDisplayText } from '@ocentra-parent/portal-domain/contracts';

export function productMetric(
  labelText: PortalDisplayText,
  valueText: PortalDetailValue,
  metaText: PortalDisplayText
): HTMLElement {
  const metric = document.createElement(PortalDom.Tags.Division);
  metric.className = PortalDom.Classes.ProductMetric;

  const label = document.createElement(PortalDom.Tags.Span);
  label.className = PortalDom.Classes.ProductMetricLabel;
  label.textContent = labelText;

  const value = document.createElement(PortalDom.Tags.Strong);
  value.className = PortalDom.Classes.ProductMetricValue;
  value.textContent = valueText;

  const meta = document.createElement(PortalDom.Tags.Span);
  meta.className = PortalDom.Classes.ProductMetricMeta;
  meta.textContent = metaText;

  metric.append(label, value, meta);
  return metric;
}
