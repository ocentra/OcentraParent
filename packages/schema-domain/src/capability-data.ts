/* generated from crates/schema/src/parent_control_capability_data_ts.rs */

export const ParentControlPlatformCapabilityInputs = [
  {
    platform: 'windows',
    capabilities: [
      {
        capability: 'headless-agent-service',
        status: 'supported',
        note: 'Windows service package is the first supported agent target.',
      },
      {
        capability: 'local-websocket-control',
        status: 'supported',
        note: 'Local portal can connect to the Windows agent over localhost.',
      },
      {
        capability: 'lan-websocket-control',
        status: 'implemented',
        note: 'LAN transport exists for development and must require pairing before child activity control.',
      },
      {
        capability: 'owned-process-terminate',
        status: 'implemented',
        note: 'Windows proof can terminate an owned process through the Rust service when the process id and expected executable match, and rejects missing process id or mismatched executable requests before broad app blocking is claimed.',
      },
      {
        capability: 'app-time-limit',
        status: 'implemented',
        note: 'App time-limit lifecycle has service proof for create, restart recovery, parent cancel, expiry, audit, and storage.',
      },
      {
        capability: 'app-blocking',
        status: 'manual-required',
        note: 'Broad application blocking is still manual-required; the runtime proof keeps app-target block-process requests unavailable until an OS-approved adapter proves app identity and block behavior on a real host.',
      },
      {
        capability: 'network-domain-blocking',
        status: 'manual-required',
        note: 'Domain or network blocking requires a real OS/network adapter proof and is not silently implemented through localhost proof.',
      },
      {
        capability: 'managed-browser-control',
        status: 'implemented',
        note: 'Managed browser intervention is proved only inside an Ocentra-owned profile bridge; exact URL control is not claimed outside that boundary.',
      },
      {
        capability: 'unmanaged-browser-detection',
        status: 'implemented',
        note: 'Windows can detect known unmanaged browser processes and terminate a matching process by pid/name; exact unmanaged URL evidence is not claimed.',
      },
      {
        capability: 'signed-auto-update',
        status: 'supported',
        note: 'Windows update manifest signing and MSI upgrade scaffold are wired.',
      },
    ],
  },
  {
    platform: 'linux',
    capabilities: [
      {
        capability: 'headless-agent-service',
        status: 'preview-scaffold',
        note: 'Linux deb and systemd package preview builds in CI.',
      },
    ],
  },
  {
    platform: 'macos',
    capabilities: [
      {
        capability: 'headless-agent-service',
        status: 'preview-scaffold',
        note: 'macOS pkg and launchd package preview builds in CI.',
      },
    ],
  },
  {
    platform: 'android',
    capabilities: [
      {
        capability: 'parent-mobile-observer',
        status: 'scaffold',
        note: 'Parent Android observer/backend state is represented, but mobile UX proof is not complete.',
      },
      {
        capability: 'parent-mobile-controller',
        status: 'manual-required',
        note: 'Parent Android controller takeover requires real mobile package and device proof before support is claimed.',
      },
      {
        capability: 'foreground-mobile-service',
        status: 'manual-required',
        note: 'Android debug APK foreground service preview builds in CI.',
      },
      {
        capability: 'notifications',
        status: 'manual-required',
        note: 'Android notification permission and delivery behavior requires emulator or physical-device proof.',
      },
      {
        capability: 'local-storage',
        status: 'scaffold',
        note: 'Android local storage compatibility must be proved on emulator or device before child-agent support is claimed.',
      },
      {
        capability: 'typed-protocol-bridge',
        status: 'scaffold',
        note: 'Shared protocol shapes exist; Android bridge behavior still needs real package proof.',
      },
      {
        capability: 'usage-stats',
        status: 'manual-required',
        note: 'UsageStats requires a real permission grant and device evidence.',
      },
      {
        capability: 'accessibility-service',
        status: 'manual-required',
        note: 'Accessibility enforcement or observation requires explicit user-granted service proof.',
      },
      {
        capability: 'vpn-dns-filtering',
        status: 'manual-required',
        note: 'VPN or DNS filtering requires an OS-approved adapter and real device proof.',
      },
      {
        capability: 'device-owner-policy',
        status: 'manual-required',
        note: 'Device-owner policy is not claimed until enrollment and policy tests exist.',
      },
      {
        capability: 'managed-profile',
        status: 'manual-required',
        note: 'Managed-profile behavior requires enrollment proof before child-agent support is claimed.',
      },
      {
        capability: 'package-lifecycle',
        status: 'manual-required',
        note: 'Android install, update, backgrounding, reboot, and uninstall lifecycle behavior requires emulator or physical-device artifacts.',
      },
      {
        capability: 'store-distribution',
        status: 'planned',
        note: 'Google Play signing and release tracks are not wired yet.',
      },
    ],
  },
  {
    platform: 'ios',
    capabilities: [
      {
        capability: 'parent-mobile-observer',
        status: 'scaffold',
        note: 'Parent iOS observer/backend state is represented, but mobile UX proof is not complete.',
      },
      {
        capability: 'parent-mobile-controller',
        status: 'manual-required',
        note: 'Parent iOS controller takeover requires real mobile package, signing, and device proof.',
      },
      {
        capability: 'foreground-mobile-service',
        status: 'unavailable',
        note: 'iOS simulator app preview builds in CI.',
      },
      {
        capability: 'family-controls-entitlement',
        status: 'manual-required',
        note: 'Apple Family Controls entitlement is not claimed until entitlement and device tests exist.',
      },
      {
        capability: 'device-activity',
        status: 'manual-required',
        note: 'DeviceActivity proof requires Apple entitlement and real device or TestFlight evidence.',
      },
      {
        capability: 'screen-time-api',
        status: 'manual-required',
        note: 'Screen Time control requires approved Apple APIs and entitlement proof.',
      },
      {
        capability: 'network-extension',
        status: 'manual-required',
        note: 'Network Extension behavior requires signing, entitlement, and device proof.',
      },
      {
        capability: 'notifications',
        status: 'manual-required',
        note: 'Notification behavior requires permission and device proof.',
      },
      {
        capability: 'background-execution',
        status: 'manual-required',
        note: 'Background execution cannot be claimed from simulator scaffold alone.',
      },
      {
        capability: 'signing-entitlements',
        status: 'manual-required',
        note: 'iOS signing and entitlement behavior requires Apple credentials, approved entitlements, and device or TestFlight proof.',
      },
      {
        capability: 'testflight-distribution',
        status: 'manual-required',
        note: 'TestFlight availability requires signing and App Store Connect evidence.',
      },
      {
        capability: 'store-distribution',
        status: 'planned',
        note: 'Apple signing, notarization, and App Store workflows are not wired yet.',
      },
    ],
  },
] as const;
