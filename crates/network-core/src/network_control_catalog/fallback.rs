use super::network_control_catalog_text::contains_any;
use super::NetworkControlEffectStatus;

pub fn fallback_for(effect_status: NetworkControlEffectStatus, source_text: &str) -> &'static str {
    let searchable = source_text.to_lowercase();
    if contains_any(
        &searchable,
        &["exact url", "path/query", "active tab", "page title"],
    ) {
        return "Hide or disable exact URL controls unless managed browser, explicit URL filter, or proxy proof exists.";
    }
    if contains_any(
        &searchable,
        &[
            "decrypted",
            "payload",
            "page body",
            "chat content",
            "search terms",
            "form values",
            "cookies",
            "tokens",
            "credentials",
        ],
    ) {
        return "Never collect decrypted content or sensitive payload fields in the network-control catalog.";
    }
    match effect_status {
        NetworkControlEffectStatus::ManualRequired => {
            "Show manual-required until setup, privileges, and adapter proof exist; compile observe or unavailable fallback."
        }
        NetworkControlEffectStatus::Degraded => {
            "Render degraded state and keep unsupported behavior out of compiled enforcement plans."
        }
        NetworkControlEffectStatus::ProofRequired => {
            "Require evidence proof before strict effect; otherwise fall back to observe, warn, ask, or unavailable."
        }
        NetworkControlEffectStatus::FutureGap => {
            "Expose as future or planning-only and do not claim current runtime behavior."
        }
        _ => "Portal renders the control; child-agent/runtime ownership remains explicit.",
    }
}
