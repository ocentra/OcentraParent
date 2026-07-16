use super::*;

pub(super) fn route_surface_kind_for(
    segments: &[BrowserGameUrlText],
    route_hints: &BrowserGameRouteHints,
) -> BrowserGameShapeCode {
    if route_hints.has_cloud_session_hint {
        return ROUTE_CLOUD_SESSION;
    }
    if route_hints.has_embed_hint {
        return ROUTE_EMBED;
    }
    if route_hints.has_play_hint {
        return ROUTE_PLAY;
    }
    if route_hints.has_purchase_hint {
        return ROUTE_PURCHASE;
    }
    if route_hints.has_account_hint {
        return ROUTE_ACCOUNT;
    }
    if segments.is_empty() {
        return ROUTE_HOME;
    }
    if segments.len() <= 2 {
        return ROUTE_CATALOG;
    }
    ROUTE_GAME_DETAIL
}
