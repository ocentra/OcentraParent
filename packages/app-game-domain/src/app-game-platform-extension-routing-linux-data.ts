import { linuxRoute } from './app-game-platform-extension-routing-data-support';

export const LinuxPlatformExtensionRows = [
  linuxRoute(
    'LINUX-01',
    'Desktop entry inventory adapter',
    'inventory',
    'observe-only',
    'not-required',
    'Desktop-entry fixture proof'
  ),
  linuxRoute(
    'LINUX-02',
    'dpkg/rpm/pacman package inventory adapter',
    'inventory',
    'observe-only',
    'not-required',
    'Distro matrix proof'
  ),
  linuxRoute(
    'LINUX-03',
    'Flatpak inventory adapter',
    'inventory',
    'observe-only',
    'not-required',
    'Flatpak app id proof'
  ),
  linuxRoute('LINUX-04', 'Snap inventory adapter', 'inventory', 'observe-only', 'not-required', 'Snap app id proof'),
  linuxRoute(
    'LINUX-05',
    'AppImage bounded scan adapter',
    'inventory',
    'user-approved-helper',
    'permission-required',
    'Bounded scan/no-secret proof',
    ['@requires-admin-root']
  ),
  linuxRoute(
    'LINUX-06',
    'procfs runtime adapter',
    'runtime',
    'observe-only',
    'not-required',
    'Real host/process proof'
  ),
  linuxRoute(
    'LINUX-07',
    'cgroup/systemd identity adapter',
    'identity',
    'root-or-admin-service',
    'admin-or-root-required',
    'systemd/cgroup proof',
    ['@requires-admin-root']
  ),
  linuxRoute(
    'LINUX-08',
    'X11 foreground adapter',
    'foreground',
    'user-approved-helper',
    'permission-required',
    'X11 foreground proof',
    ['@requires-x11']
  ),
  linuxRoute(
    'LINUX-09',
    'Wayland compositor capability matrix',
    'foreground',
    'manual-required',
    'manual-required',
    'Compositor-specific proof',
    ['@requires-wayland']
  ),
  linuxRoute(
    'LINUX-10',
    'Linux terminate adapter',
    'terminate-process',
    'root-or-admin-service',
    'admin-or-root-required',
    'Target recheck and rollback proof',
    ['@requires-admin-root']
  ),
  linuxRoute(
    'LINUX-11',
    'cgroup/systemd scope enforcement proof',
    'block-launch',
    'root-or-admin-service',
    'admin-or-root-required',
    'Service-manager proof',
    ['@requires-admin-root']
  ),
  linuxRoute(
    'LINUX-12',
    'AppArmor/SELinux manual proof',
    'block-launch',
    'root-or-admin-service',
    'admin-or-root-required',
    'Policy profile proof',
    ['@requires-admin-root']
  ),
  linuxRoute(
    'LINUX-13',
    'Package-manager restriction proof',
    'allowlist',
    'root-or-admin-service',
    'admin-or-root-required',
    'Distro package proof',
    ['@requires-admin-root']
  ),
  linuxRoute(
    'LINUX-14',
    'Flatpak/Snap restriction proof',
    'managed-configuration',
    'root-or-admin-service',
    'admin-or-root-required',
    'Sandbox permission proof',
    ['@requires-admin-root']
  ),
] as const;
