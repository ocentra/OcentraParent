use super::*;

pub(super) fn reason_codes_for(
    route_surface_kind: BrowserGameShapeCode,
    route_hints: &BrowserGameRouteHints,
) -> Vec<BrowserGameShapeCode> {
    if route_hints.has_cloud_session_hint || route_surface_kind == ROUTE_CLOUD_SESSION {
        return vec![REASON_CLOUD_SESSION_HINT];
    }
    if route_hints.has_embed_hint || route_surface_kind == ROUTE_EMBED {
        return vec![REASON_EMBED_ROUTE_HINT];
    }
    if route_hints.has_play_hint || route_surface_kind == ROUTE_PLAY {
        return vec![REASON_GAME_ROUTE_HINT];
    }
    if route_hints.has_purchase_hint || route_surface_kind == ROUTE_PURCHASE {
        return vec![REASON_PURCHASE_ROUTE_HINT];
    }
    if route_hints.has_account_hint || route_surface_kind == ROUTE_ACCOUNT {
        return vec![REASON_ACCOUNT_ROUTE_HINT];
    }
    vec![REASON_CATALOG_ROUTE_HINT]
}
