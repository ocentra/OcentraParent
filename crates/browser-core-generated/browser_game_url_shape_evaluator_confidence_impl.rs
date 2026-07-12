use super::*;

pub(super) fn confidence_for(
    route_surface_kind: BrowserGameShapeCode,
    host_shape: BrowserGameShapeCode,
    path_depth: BrowserGameShapeCode,
) -> BrowserGameShapeCode {
    if host_shape == HOST_UNKNOWN
        || path_depth == PATH_UNKNOWN
        || route_surface_kind == ROUTE_UNKNOWN
    {
        return CONFIDENCE_UNKNOWN;
    }
    if route_surface_kind == ROUTE_HOME || route_surface_kind == ROUTE_CATALOG {
        return CONFIDENCE_MEDIUM;
    }
    CONFIDENCE_HIGH
}
