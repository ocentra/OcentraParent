use ocentra_parent_agent_maintenance::args::{parse_args_from, CommandLine};
use ocentra_parent_agent_maintenance::constants::DEFAULT_MANIFEST_URL;

#[test]
fn parse_run_once_dry_run_command() {
    let args = vec![
        "run-once".to_owned(),
        "--manifest-url".to_owned(),
        "https://github.com/ocentra/OcentraParent/releases/latest/download/latest-windows.json"
            .to_owned(),
        "--dry-run".to_owned(),
        "--current-version".to_owned(),
        "0.1.0".to_owned(),
    ];
    let parsed_result = parse_args_from(&args);
    assert!(
        parsed_result.is_ok(),
        "args parse failed: {parsed_result:?}"
    );
    let Ok(parsed) = parsed_result else {
        return;
    };

    if let CommandLine::RunOnce {
        manifest_url,
        dry_run,
        current_version,
    } = parsed
    {
        assert!(dry_run);
        assert_eq!(manifest_url, DEFAULT_MANIFEST_URL);
        assert_eq!(current_version, "0.1.0");
    } else {
        assert!(matches!(parsed, CommandLine::RunOnce { .. }));
    }
}
