use super::NEIGHBOR_HOSTNAME_CACHE;

pub fn clear_cached_neighbor_identities() {
    if let Some(cache) = NEIGHBOR_HOSTNAME_CACHE.get() {
        let _ = cache.lock().map(|mut entries| entries.clear());
    }
}
