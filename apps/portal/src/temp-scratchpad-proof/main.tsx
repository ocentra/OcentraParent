import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { WeeklySchedulerScratchPage } from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/WeeklySchedulerScratchPage';
import './base.module.css';

document.getElementById(PortalDom.Ids.AppLoading)?.remove();

const rootElement = document.querySelector<HTMLDivElement>(PortalDom.RootSelector);
if (rootElement === null) {
  throw new Error(resolvePortalDevText(PortalDevTextToken.RootMissing));
}

createRoot(rootElement).render(
  <StrictMode>
    <WeeklySchedulerScratchPage />
  </StrictMode>
);
