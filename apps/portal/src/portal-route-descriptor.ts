import {
  PortalRoute,
  PortalRouteDescriptors,
  type PortalRoute as PortalRouteValue,
  type PortalRouteDescriptor,
} from '@ocentra-parent/portal-domain/contracts';

export function routeDescriptor(route: PortalRouteValue): PortalRouteDescriptor {
  return (
    PortalRouteDescriptors.find((descriptor) => descriptor.route === route) ??
    PortalRouteDescriptors.find((descriptor) => descriptor.route === PortalRoute.Overview) ??
    PortalRouteDescriptors[0]!
  );
}
