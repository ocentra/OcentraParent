use std::sync::Arc;

use rustls::ClientConfig;

use super::AcceptAnyServerCertVerifier;

pub(super) fn tls_client_config() -> Option<Arc<ClientConfig>> {
    static TLS_CLIENT_CONFIG: std::sync::OnceLock<Option<Arc<ClientConfig>>> =
        std::sync::OnceLock::new();
    TLS_CLIENT_CONFIG
        .get_or_init(|| {
            let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
            let config = ClientConfig::builder()
                .dangerous()
                .with_custom_certificate_verifier(Arc::new(AcceptAnyServerCertVerifier))
                .with_no_client_auth();
            Some(Arc::new(config))
        })
        .clone()
}
