use super::*;

pub(super) fn fingerprint_for(
    input: &BrowserGameRouteFingerprintInput<'_>,
) -> BrowserGameUrlFingerprint {
    BrowserGameUrlFingerprint(
        [
            FINGERPRINT_URL_SHAPE.to_string(),
            input.protocol_shape.0.to_string(),
            input.host_shape.0.to_string(),
            input.path_depth.0.to_string(),
            input.route_surface_kind.0.to_string(),
            if input.has_game_id_like_segment {
                FINGERPRINT_GAME_ID_LIKE.to_string()
            } else {
                FINGERPRINT_NO_GAME_ID.to_string()
            },
            if input.has_query_shape {
                FINGERPRINT_QUERY.to_string()
            } else {
                FINGERPRINT_NO_QUERY.to_string()
            },
            if input.has_fragment_shape {
                FINGERPRINT_FRAGMENT.to_string()
            } else {
                FINGERPRINT_NO_FRAGMENT.to_string()
            },
            if input.route_hints.has_embed_hint {
                FINGERPRINT_EMBED.to_string()
            } else {
                FINGERPRINT_NO_EMBED.to_string()
            },
            if input.route_hints.has_play_hint {
                FINGERPRINT_PLAY.to_string()
            } else {
                FINGERPRINT_NO_PLAY.to_string()
            },
            if input.route_hints.has_account_hint {
                FINGERPRINT_ACCOUNT.to_string()
            } else {
                FINGERPRINT_NO_ACCOUNT.to_string()
            },
            if input.route_hints.has_purchase_hint {
                FINGERPRINT_PURCHASE.to_string()
            } else {
                FINGERPRINT_NO_PURCHASE.to_string()
            },
            if input.route_hints.has_cloud_session_hint {
                FINGERPRINT_CLOUD_SESSION.to_string()
            } else {
                FINGERPRINT_NO_CLOUD_SESSION.to_string()
            },
        ]
        .join(FINGERPRINT_SEPARATOR),
    )
}
