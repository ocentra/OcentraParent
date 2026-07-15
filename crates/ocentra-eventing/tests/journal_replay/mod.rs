#[path = "fixtures.rs"]
mod fixtures;

mod suite {
    #[path = "../bus_policy.rs"]
    mod bus_policy;
    #[path = "../file.rs"]
    mod file;
    #[path = "../replay.rs"]
    mod replay;
    #[path = "../support.rs"]
    mod support;
}
