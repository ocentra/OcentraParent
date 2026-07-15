#[cfg(not(windows))]
use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
#[cfg(not(windows))]
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;

#[cfg(not(windows))]
pub(super) fn current_platform() -> ParentPlatform {
    match std::env::consts::OS {
        enforcement_constants::PLATFORM_LINUX => ParentPlatform::Linux,
        enforcement_constants::PLATFORM_MACOS => ParentPlatform::Macos,
        enforcement_constants::PLATFORM_ANDROID => ParentPlatform::Android,
        enforcement_constants::PLATFORM_IOS => ParentPlatform::Ios,
        _ => ParentPlatform::Linux,
    }
}
