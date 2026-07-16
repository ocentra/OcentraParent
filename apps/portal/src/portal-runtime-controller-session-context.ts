import { ParentRoute, type ParentRouteContext, type ParentRouteId } from '../generated/parent-ui-bridge';
import {
  readStoredManageTargetSelection,
  selectedChildDeviceIdFromManageTargetSelection,
} from '@ocentra-parent/portal-domain/manage-target-selection';

export function createCurrentRouteContext(): ParentRouteContext {
  const selectedChildDeviceId = selectedChildDeviceIdFromManageTargetSelection(readStoredManageTargetSelection());
  return selectedChildDeviceId ? { selectedChildDeviceId } : {};
}

export function shouldPrimeDeveloperRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.Commands || route === ParentRoute.Events || route === ParentRoute.Logs;
}
