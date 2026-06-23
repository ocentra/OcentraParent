import { PortalRouteDescriptors, type PortalRouteDescriptor } from '@ocentra-parent/portal-domain/routes';
import { PortalRoute, type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
export function routeDescriptor(route: PortalRouteValue): PortalRouteDescriptor {
  return (
    PortalRouteDescriptors.find((descriptor) => descriptor.route === route) ??
    PortalRouteDescriptors.find((descriptor) => descriptor.route === PortalRoute.Overview) ??
    PortalRouteDescriptors[0]!
  );
}
