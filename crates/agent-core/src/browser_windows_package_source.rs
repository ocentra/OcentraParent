use std::{
    fs,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_APPX_ATTRIBUTE_DISPLAY_NAME, APP_GAME_APPX_ATTRIBUTE_ID, APP_GAME_APPX_ATTRIBUTE_NAME,
    APP_GAME_APPX_ELEMENT_APPLICATION, APP_GAME_APPX_ELEMENT_DISPLAY_NAME,
    APP_GAME_APPX_ELEMENT_IDENTITY, APP_GAME_APPX_ELEMENT_VISUAL_ELEMENTS,
    APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME,
};
use ocentra_parent_agent_protocol::constants;

use crate::BrowserWindowsPackageIdentity;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserWindowsLivePackageEntry {
    pub package_name: String,
    pub display_name: Option<String>,
    pub app_user_model_id: Option<String>,
}

pub fn live_windows_browser_package_entries_with_limit(
    limit: usize,
) -> Vec<BrowserWindowsPackageIdentity> {
    live_windows_browser_package_entries(limit)
        .into_iter()
        .map(package_identity_from_entry)
        .collect()
}

pub fn live_windows_browser_package_entries_from_roots(
    roots: &[PathBuf],
    limit: usize,
) -> Vec<BrowserWindowsPackageIdentity> {
    manifest_paths_from_roots(roots, limit)
        .iter()
        .filter_map(|path| package_entry_from_manifest_path(path))
        .map(package_identity_from_entry)
        .collect()
}

#[cfg(windows)]
fn live_windows_browser_package_entries(limit: usize) -> Vec<BrowserWindowsLivePackageEntry> {
    let roots = live_windows_package_roots();
    manifest_paths_from_roots(&roots, limit)
        .iter()
        .filter_map(|path| package_entry_from_manifest_path(path))
        .collect()
}

#[cfg(not(windows))]
fn live_windows_browser_package_entries(_limit: usize) -> Vec<BrowserWindowsLivePackageEntry> {
    Vec::new()
}

#[cfg(windows)]
fn live_windows_package_roots() -> Vec<PathBuf> {
    std::env::var_os(constants::env_var::PROGRAM_FILES)
        .map(windows_apps_root)
        .into_iter()
        .collect()
}

#[cfg(windows)]
fn windows_apps_root(root: std::ffi::OsString) -> PathBuf {
    let mut path = PathBuf::from(root);
    path.push(APP_GAME_WINDOWS_PATH_WINDOWS_APPS);
    path
}

fn package_identity_from_entry(
    entry: BrowserWindowsLivePackageEntry,
) -> BrowserWindowsPackageIdentity {
    BrowserWindowsPackageIdentity {
        package_name: entry.package_name,
        display_name: entry.display_name,
        app_user_model_id: entry.app_user_model_id,
    }
}

fn manifest_paths_from_roots(roots: &[PathBuf], limit: usize) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for root in roots {
        collect_manifest_paths(root, limit, &mut paths);
        if paths.len() >= limit {
            break;
        }
    }
    paths
}

fn collect_manifest_paths(root: &Path, limit: usize, paths: &mut Vec<PathBuf>) {
    if paths.len() >= limit {
        return;
    }
    if is_manifest_path(root) {
        paths.push(root.to_path_buf());
        return;
    }
    let direct_manifest = root.join(APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME);
    if direct_manifest.is_file() {
        paths.push(direct_manifest);
        return;
    }
    collect_child_manifest_paths(root, limit, paths);
}

fn collect_child_manifest_paths(root: &Path, limit: usize, paths: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            let manifest = path.join(APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME);
            if manifest.is_file() {
                paths.push(manifest);
            }
        } else if is_manifest_path(&path) {
            paths.push(path);
        }
        if paths.len() >= limit {
            break;
        }
    }
}

fn is_manifest_path(path: &Path) -> bool {
    path.file_name().is_some_and(|file_name| {
        file_name
            .to_string_lossy()
            .eq_ignore_ascii_case(APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME)
    })
}

fn package_entry_from_manifest_path(path: &Path) -> Option<BrowserWindowsLivePackageEntry> {
    let manifest = fs::read_to_string(path).ok()?;
    package_entry_from_manifest_xml(&manifest)
}

fn package_entry_from_manifest_xml(manifest: &str) -> Option<BrowserWindowsLivePackageEntry> {
    let document = roxmltree::Document::parse(manifest).ok()?;
    let identity = document
        .descendants()
        .find(|node| node.has_tag_name(APP_GAME_APPX_ELEMENT_IDENTITY))?;
    let package_name = optional_attribute(identity, APP_GAME_APPX_ATTRIBUTE_NAME)?;
    let application = document
        .descendants()
        .find(|node| node.has_tag_name(APP_GAME_APPX_ELEMENT_APPLICATION));
    let app_user_model_id = application
        .and_then(|node| optional_attribute(node, APP_GAME_APPX_ATTRIBUTE_ID))
        .map(|id| app_user_model_id(&package_name, &id));
    Some(BrowserWindowsLivePackageEntry {
        package_name,
        display_name: display_label(&document),
        app_user_model_id,
    })
}

fn display_label(document: &roxmltree::Document<'_>) -> Option<String> {
    visual_elements_display_name(document).or_else(|| properties_display_name(document))
}

fn visual_elements_display_name(document: &roxmltree::Document<'_>) -> Option<String> {
    document
        .descendants()
        .find(|node| node.has_tag_name(APP_GAME_APPX_ELEMENT_VISUAL_ELEMENTS))
        .and_then(|node| optional_attribute(node, APP_GAME_APPX_ATTRIBUTE_DISPLAY_NAME))
}

fn properties_display_name(document: &roxmltree::Document<'_>) -> Option<String> {
    document
        .descendants()
        .find(|node| node.has_tag_name(APP_GAME_APPX_ELEMENT_DISPLAY_NAME))
        .and_then(|node| node.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn optional_attribute(node: roxmltree::Node<'_, '_>, attribute: &str) -> Option<String> {
    node.attribute(attribute)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn app_user_model_id(package_name: &str, application_id: &str) -> String {
    let mut id = String::from(package_name);
    id.push(constants::delimiter::BANG);
    id.push_str(application_id);
    id
}
#[cfg(target_os = "windows")]
use ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_PATH_WINDOWS_APPS;
