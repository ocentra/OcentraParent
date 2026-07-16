macro_rules! weak_name_evidence_from_source {
    ($source:expr, $value:expr, $observed_at:expr, $network_interface:expr) => {{
        [
            (
                "reverse-dns",
                reverse_dns_name_evidence
                    as fn(&str, &str, Option<&str>) -> Option<LanNeighborNameEvidence>,
            ),
            (
                "netbios",
                netbios_name_evidence
                    as fn(&str, &str, Option<&str>) -> Option<LanNeighborNameEvidence>,
            ),
            (
                "llmnr",
                llmnr_name_evidence
                    as fn(&str, &str, Option<&str>) -> Option<LanNeighborNameEvidence>,
            ),
        ]
        .into_iter()
        .find(|(candidate, _)| *candidate == $source)
        .and_then(|(_, parser)| parser($value, $observed_at, $network_interface))
    }};
}
