// Keep the vendored TSX runtime outside the portal TypeScript boundary.
import { ParentPortalSvgSurface as VendorParentPortalSvgSurface } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx';

export function ParentPortalSvgSurface(props) {
  return VendorParentPortalSvgSurface(props);
}
