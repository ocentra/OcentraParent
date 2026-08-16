use axum::{
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        HeaderMap, HeaderValue, StatusCode,
    },
    response::IntoResponse,
};
use ocentra_parent_agent_protocol::constants;

pub async fn serve_browser_intervention_page() -> impl IntoResponse {
    let Ok(path) = std::env::var(constants::env_var::MANAGED_BROWSER_INTERVENTION_HTML_PATH) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Ok(html) = tokio::fs::read_to_string(path).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static(constants::http::CONTENT_TYPE_TEXT_HTML_UTF8),
    );
    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_static(constants::http::CACHE_CONTROL_NO_STORE),
    );
    (headers, html).into_response()
}
