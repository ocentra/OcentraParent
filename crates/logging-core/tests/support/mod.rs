macro_rules! temp_dir {
    () => {{
        let nanos = ::std::time::SystemTime::now()
            .duration_since(::std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        ::std::env::temp_dir().join(format!("ocentra-parent-logging-core-{nanos}"))
    }};
}
