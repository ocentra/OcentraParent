use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::constants;

pub fn windows_browser_inventory_candidate_paths(roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for root in roots {
        push_managed_chromium_paths(&mut paths, root);
        push_manual_chromium_paths(&mut paths, root);
        push_unsupported_browser_paths(&mut paths, root);
    }
    paths
}

fn push_managed_chromium_paths(paths: &mut Vec<PathBuf>, root: &Path) {
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_MICROSOFT,
            constants::browser::PATH_SEGMENT_EDGE,
        ],
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_MICROSOFT,
            constants::browser::PATH_SEGMENT_EDGE_BETA,
        ],
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_MICROSOFT,
            constants::browser::PATH_SEGMENT_EDGE_DEV,
        ],
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_MICROSOFT,
            constants::browser::PATH_SEGMENT_EDGE_SXS,
        ],
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_GOOGLE,
            constants::browser::PATH_SEGMENT_CHROME,
        ],
        constants::browser::EXECUTABLE_CHROME_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_GOOGLE,
            constants::browser::PATH_SEGMENT_CHROME_FOR_TESTING,
        ],
        constants::browser::EXECUTABLE_CHROME_WINDOWS,
    );
}

fn push_manual_chromium_paths(paths: &mut Vec<PathBuf>, root: &Path) {
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_BRAVE_SOFTWARE,
            constants::browser::PATH_SEGMENT_BRAVE_BROWSER,
        ],
        constants::browser::EXECUTABLE_BRAVE_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_VIVALDI],
        constants::browser::EXECUTABLE_VIVALDI_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_OPERA_SOFTWARE,
            constants::browser::PATH_SEGMENT_OPERA_STABLE,
        ],
        constants::browser::EXECUTABLE_OPERA_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_OPERA_SOFTWARE,
            constants::browser::PATH_SEGMENT_OPERA_GX_STABLE,
        ],
        constants::browser::EXECUTABLE_OPERA_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_CHROMIUM],
        constants::browser::EXECUTABLE_CHROME_WINDOWS,
    );
}

fn push_unsupported_browser_paths(paths: &mut Vec<PathBuf>, root: &Path) {
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_MOZILLA_FIREFOX],
        constants::browser::EXECUTABLE_FIREFOX_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_FIREFOX_DEVELOPER_EDITION],
        constants::browser::EXECUTABLE_FIREFOX_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_FIREFOX_NIGHTLY],
        constants::browser::EXECUTABLE_FIREFOX_WINDOWS,
    );
    paths.push(
        root.join(constants::browser::PATH_SEGMENT_TOR_BROWSER)
            .join(constants::browser::PATH_SEGMENT_BROWSER)
            .join(constants::browser::EXECUTABLE_FIREFOX_WINDOWS),
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_DUCKDUCKGO],
        constants::browser::EXECUTABLE_DUCKDUCKGO_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_ARC],
        constants::browser::EXECUTABLE_ARC_WINDOWS,
    );
}

fn push_application_path(
    paths: &mut Vec<PathBuf>,
    root: &Path,
    product_segments: &[&str],
    executable: &str,
) {
    let mut path = root.to_path_buf();
    for segment in product_segments {
        path = path.join(segment);
    }
    paths.push(
        path.join(constants::browser::PATH_SEGMENT_APPLICATION)
            .join(executable),
    );
}
