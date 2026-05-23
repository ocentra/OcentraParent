import { PortalDom } from '@ocentra-parent/portal-domain/contracts';

export function renderDashboard(container: HTMLElement, render: (dashboard: HTMLElement) => void): void {
  const dashboard = document.createElement(PortalDom.Tags.Division);
  dashboard.className = PortalDom.Classes.ProductDashboard;
  render(dashboard);
  container.append(dashboard);
}
