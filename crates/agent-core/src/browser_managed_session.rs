use std::{
    fs,
    io::ErrorKind,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    path::{Path, PathBuf},
    process::Command,
};

use ocentra_parent_agent_protocol::{
    constants, BrowserChannel, BrowserCustodyLabel, BrowserFamily,
    BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry,
    BROWSER_EVIDENCE_SCHEMA_VERSION,
};

use crate::browser_managed_discovery::managed_browser_executable_identity;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedLaunchConfig {
    pub executable_path: PathBuf,
    pub profile_dir: PathBuf,
    pub bridge_port: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedBridgePortReservation {
    pub endpoint: SocketAddr,
    pub bridge_port: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedProfileStoreConfig {
    pub profile_root_dir: PathBuf,
    pub profile_id: String,
    pub profile_scope_id: String,
    pub device_id: String,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub policy_revision: String,
    pub now: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedProfileStoreRecord {
    pub profile_dir: PathBuf,
    pub metadata_path: PathBuf,
    pub entry: BrowserManagedProfileStoreEntry,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedLaunchPlan {
    pub executable_path: PathBuf,
    pub args: Vec<String>,
    pub bridge_port: u16,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub profile_path_ref: String,
    pub bridge_endpoint_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedLaunch {
    pub process_id: u32,
    pub bridge_port: u16,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub profile_path_ref: String,
    pub bridge_endpoint_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserManagedLaunchError {
    DefaultProfileRejected,
    UnownedProfileRejected,
    BridgePortUnavailable,
    UnsupportedBrowser,
    Io,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserManagedProfileStoreError {
    DefaultProfileRejected,
    UnownedProfileRejected,
    MetadataCorrupt,
    Io,
}

impl BrowserManagedProfileStoreError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::DefaultProfileRejected | Self::UnownedProfileRejected => {
                constants::value::MANAGED_BROWSER_INVALID_PROFILE
            }
            Self::MetadataCorrupt => constants::value::MANAGED_BROWSER_PROFILE_METADATA_CORRUPT,
            Self::Io => constants::value::MANAGED_BROWSER_PROFILE_STORE_IO_ERROR,
        }
    }
}

impl BrowserManagedLaunchError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::DefaultProfileRejected | Self::UnownedProfileRejected => {
                constants::value::MANAGED_BROWSER_INVALID_PROFILE
            }
            Self::BridgePortUnavailable => {
                constants::value::MANAGED_BROWSER_BRIDGE_PORT_UNAVAILABLE
            }
            Self::UnsupportedBrowser => constants::value::MANAGED_BROWSER_UNSUPPORTED_EXECUTABLE,
            Self::Io => constants::value::MANAGED_BROWSER_LAUNCH_ERROR,
        }
    }
}

pub fn load_managed_browser_profile_store(
    config: BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(&config)?;
    let stored_entry = read_profile_store_entry(&paths.metadata_path)?;
    let created_at = stored_entry
        .as_ref()
        .map(|entry| entry.created_at.clone())
        .unwrap_or_else(|| config.now.clone());

    if !paths.profile_dir.is_dir() {
        return Ok(profile_store_record(
            &config,
            paths,
            created_at,
            BrowserManagedProfileLifecycleState::Missing,
            Some(config.now.clone()),
            None,
            None,
            Some(constants::browser::PROFILE_STORE_REASON_PROFILE_DIR_MISSING.to_string()),
        ));
    }

    if stored_entry.is_none() {
        return Ok(profile_store_record(
            &config,
            paths,
            created_at,
            BrowserManagedProfileLifecycleState::RepairRequired,
            None,
            None,
            None,
            Some(constants::browser::PROFILE_STORE_REASON_METADATA_MISSING.to_string()),
        ));
    }

    Ok(profile_store_record(
        &config,
        paths,
        created_at,
        BrowserManagedProfileLifecycleState::Ready,
        None,
        None,
        None,
        None,
    ))
}

pub fn create_or_repair_managed_browser_profile_store(
    config: BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(&config)?;
    let profile_existed = paths.profile_dir.is_dir();
    fs::create_dir_all(&config.profile_root_dir)
        .map_err(|_| BrowserManagedProfileStoreError::Io)?;
    let stored_entry = read_profile_store_entry(&paths.metadata_path).unwrap_or(None);
    fs::create_dir_all(&paths.profile_dir).map_err(|_| BrowserManagedProfileStoreError::Io)?;
    let created_at = stored_entry
        .as_ref()
        .map(|entry| entry.created_at.clone())
        .unwrap_or_else(|| config.now.clone());
    let repaired_at = if (profile_existed && stored_entry.is_none())
        || (!profile_existed && stored_entry.is_some())
    {
        Some(config.now.clone())
    } else {
        stored_entry
            .as_ref()
            .and_then(|entry| match entry.lifecycle_state {
                BrowserManagedProfileLifecycleState::Ready => None,
                _ => Some(config.now.clone()),
            })
    };
    let repair_reason = repaired_at
        .as_ref()
        .map(|_| constants::browser::PROFILE_STORE_REASON_REPAIRED.to_string());
    let record = profile_store_record(
        &config,
        paths,
        created_at,
        BrowserManagedProfileLifecycleState::Ready,
        None,
        repaired_at,
        None,
        repair_reason,
    );
    write_profile_store_entry(&record.metadata_path, &record.entry)?;
    Ok(record)
}

pub fn delete_managed_browser_profile_store(
    config: BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(&config)?;
    fs::create_dir_all(&config.profile_root_dir)
        .map_err(|_| BrowserManagedProfileStoreError::Io)?;
    let stored_entry = read_profile_store_entry(&paths.metadata_path).unwrap_or(None);
    if paths.profile_dir.exists() {
        fs::remove_dir_all(&paths.profile_dir).map_err(|_| BrowserManagedProfileStoreError::Io)?;
    }
    let created_at = stored_entry
        .as_ref()
        .map(|entry| entry.created_at.clone())
        .unwrap_or_else(|| config.now.clone());
    let record = profile_store_record(
        &config,
        paths,
        created_at,
        BrowserManagedProfileLifecycleState::Deleted,
        None,
        None,
        Some(config.now.clone()),
        Some(constants::browser::PROFILE_STORE_REASON_DELETED.to_string()),
    );
    write_profile_store_entry(&record.metadata_path, &record.entry)?;
    Ok(record)
}

pub fn reserve_managed_browser_bridge_port(
) -> Result<BrowserManagedBridgePortReservation, BrowserManagedLaunchError> {
    let endpoint = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        constants::browser::DEVTOOLS_PORT_UNRESERVED,
    );
    let listener = TcpListener::bind(endpoint)
        .map_err(|_| BrowserManagedLaunchError::BridgePortUnavailable)?;
    let reserved_endpoint = listener
        .local_addr()
        .map_err(|_| BrowserManagedLaunchError::BridgePortUnavailable)?;
    if !reserved_endpoint.ip().is_loopback()
        || reserved_endpoint.port() == constants::browser::DEVTOOLS_PORT_UNRESERVED
    {
        return Err(BrowserManagedLaunchError::BridgePortUnavailable);
    }
    drop(listener);

    Ok(BrowserManagedBridgePortReservation {
        endpoint: reserved_endpoint,
        bridge_port: reserved_endpoint.port(),
    })
}

pub fn managed_browser_launch_plan(
    config: BrowserManagedLaunchConfig,
) -> Result<BrowserManagedLaunchPlan, BrowserManagedLaunchError> {
    let identity = managed_browser_executable_identity(&config.executable_path);
    if !identity.supports_managed_cdp {
        return Err(BrowserManagedLaunchError::UnsupportedBrowser);
    }
    if default_profile_path_rejected(&config.profile_dir) {
        return Err(BrowserManagedLaunchError::DefaultProfileRejected);
    }
    if !managed_profile_path_owned(&config.profile_dir) {
        return Err(BrowserManagedLaunchError::UnownedProfileRejected);
    }
    if config.bridge_port == constants::browser::DEVTOOLS_PORT_UNRESERVED {
        return Err(BrowserManagedLaunchError::BridgePortUnavailable);
    }

    let profile = config.profile_dir.to_string_lossy();
    let mut debugging_address =
        String::from(constants::browser::CHROMIUM_ARG_REMOTE_DEBUGGING_ADDRESS_PREFIX);
    debugging_address.push_str(constants::browser::CHROMIUM_REMOTE_DEBUGGING_LOOPBACK);
    let mut debugging = String::from(constants::browser::CHROMIUM_ARG_REMOTE_DEBUGGING_PORT_PREFIX);
    debugging.push_str(&config.bridge_port.to_string());
    let mut user_data = String::from(constants::browser::CHROMIUM_ARG_USER_DATA_DIR_PREFIX);
    user_data.push_str(&profile);
    let mut profile_directory =
        String::from(constants::browser::CHROMIUM_ARG_PROFILE_DIRECTORY_PREFIX);
    profile_directory.push_str(constants::browser::PROFILE_DIRECTORY_MANAGED_CHILD);

    Ok(BrowserManagedLaunchPlan {
        executable_path: config.executable_path,
        args: vec![
            debugging_address,
            debugging,
            user_data,
            profile_directory,
            constants::browser::CHROMIUM_ARG_NO_FIRST_RUN.to_string(),
            constants::browser::CHROMIUM_ARG_NO_DEFAULT_BROWSER_CHECK.to_string(),
            constants::browser::CHROMIUM_DEFAULT_URL.to_string(),
        ],
        bridge_port: config.bridge_port,
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        profile_path_ref: constants::browser::PROFILE_PATH_REF_MANAGED.to_string(),
        bridge_endpoint_ref: constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS.to_string(),
    })
}

pub fn launch_managed_browser(
    config: BrowserManagedLaunchConfig,
) -> Result<BrowserManagedLaunch, BrowserManagedLaunchError> {
    let plan = managed_browser_launch_plan(config)?;
    let child = Command::new(&plan.executable_path)
        .args(&plan.args)
        .spawn()
        .map_err(|_| BrowserManagedLaunchError::Io)?;

    Ok(BrowserManagedLaunch {
        process_id: child.id(),
        bridge_port: plan.bridge_port,
        browser_family: plan.browser_family,
        browser_channel: plan.browser_channel,
        profile_path_ref: plan.profile_path_ref,
        bridge_endpoint_ref: plan.bridge_endpoint_ref,
    })
}

fn default_profile_path_rejected(path: &Path) -> bool {
    normalized_component_names(path).iter().any(|name| {
        name == constants::browser::PATH_SEGMENT_DEFAULT_NORMALIZED
            || name == constants::browser::PATH_SEGMENT_USER_DATA_NORMALIZED
    })
}

fn managed_profile_path_owned(path: &Path) -> bool {
    path.file_name()
        .map(|name| {
            name.to_string_lossy()
                .to_ascii_lowercase()
                .starts_with(constants::browser::PROFILE_ID_PREFIX_MANAGED)
        })
        .unwrap_or(false)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BrowserManagedProfileStorePaths {
    profile_dir: PathBuf,
    metadata_path: PathBuf,
}

fn managed_profile_store_paths(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStorePaths, BrowserManagedProfileStoreError> {
    let profile_dir = config.profile_root_dir.join(&config.profile_id);
    if default_profile_path_rejected(&config.profile_root_dir)
        || default_profile_path_rejected(&profile_dir)
    {
        return Err(BrowserManagedProfileStoreError::DefaultProfileRejected);
    }
    if profile_id_contains_path_separator(&config.profile_id)
        || !config
            .profile_id
            .starts_with(constants::browser::PROFILE_ID_PREFIX_MANAGED)
        || !managed_profile_path_owned(&profile_dir)
    {
        return Err(BrowserManagedProfileStoreError::UnownedProfileRejected);
    }

    let mut metadata_file_name = config.profile_id.clone();
    metadata_file_name.push_str(constants::browser::PROFILE_STORE_METADATA_SUFFIX);
    Ok(BrowserManagedProfileStorePaths {
        profile_dir,
        metadata_path: config.profile_root_dir.join(metadata_file_name),
    })
}

fn profile_store_record(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    created_at: String,
    lifecycle_state: BrowserManagedProfileLifecycleState,
    missing_since: Option<String>,
    repaired_at: Option<String>,
    deleted_at: Option<String>,
    repair_reason: Option<String>,
) -> BrowserManagedProfileStoreRecord {
    BrowserManagedProfileStoreRecord {
        profile_dir: paths.profile_dir,
        metadata_path: paths.metadata_path,
        entry: BrowserManagedProfileStoreEntry {
            schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
            profile_id: config.profile_id.clone(),
            profile_path_ref: constants::browser::PROFILE_PATH_REF_MANAGED.to_string(),
            profile_root_ref: constants::browser::PROFILE_ROOT_REF_MANAGED.to_string(),
            profile_scope_id: config.profile_scope_id.clone(),
            device_id: config.device_id.clone(),
            browser_family: config.browser_family.clone(),
            browser_channel: config.browser_channel.clone(),
            lifecycle_state,
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            policy_revision: config.policy_revision.clone(),
            created_at,
            updated_at: config.now.clone(),
            missing_since,
            repaired_at,
            deleted_at,
            repair_reason,
        },
    }
}

fn read_profile_store_entry(
    metadata_path: &Path,
) -> Result<Option<BrowserManagedProfileStoreEntry>, BrowserManagedProfileStoreError> {
    match fs::read_to_string(metadata_path) {
        Ok(contents) => serde_json::from_str(&contents)
            .map(Some)
            .map_err(|_| BrowserManagedProfileStoreError::MetadataCorrupt),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(_) => Err(BrowserManagedProfileStoreError::Io),
    }
}

fn write_profile_store_entry(
    metadata_path: &Path,
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    let contents =
        serde_json::to_string_pretty(entry).map_err(|_| BrowserManagedProfileStoreError::Io)?;
    fs::write(metadata_path, contents).map_err(|_| BrowserManagedProfileStoreError::Io)
}

fn profile_id_contains_path_separator(profile_id: &str) -> bool {
    profile_id.contains(constants::browser::PATH_SEPARATOR_FORWARD)
        || profile_id.contains(constants::browser::PATH_SEPARATOR_BACKSLASH)
        || profile_id.contains(constants::browser::PATH_SEPARATOR_COLON)
}

fn normalized_component_names(path: &Path) -> Vec<String> {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().to_ascii_lowercase())
        .collect()
}
