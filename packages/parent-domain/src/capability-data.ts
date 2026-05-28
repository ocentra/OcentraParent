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
