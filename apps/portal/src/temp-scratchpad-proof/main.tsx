import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { PortalDom, PortalText, PortalTextToken } from '@ocentra-parent/portal-domain/contracts';
import { WeeklySchedulerScratchPage } from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/WeeklySchedulerScratchPage';
import './base.css';

document.getElementById(PortalDom.Ids.AppLoading)?.remove();

const rootElement = document.querySelector<HTMLDivElement>(PortalDom.RootSelector);
if (rootElement === null) {
  throw new Error(PortalText.Resolve(PortalTextToken.RootMissing));
}

createRoot(rootElement).render(
  <StrictMode>
    <WeeklySchedulerScratchPage />
  </StrictMode>
);
