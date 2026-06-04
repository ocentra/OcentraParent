# Manual Platform Proof

Platform scope: Windows service capture path only.

WP35 uses the existing live `sysinfo` process snapshot source and service
activity-capture path. It proves recurring bounded capture on the local service
path, not AppLocker/App Control, MDM, Endpoint Security, Device Owner/Profile
Owner, FamilyControls/ManagedSettings, cgroup/systemd, or broad blocking.

Manual-required gaps:

- Foreground source proof.
- Richer process start/exit subscriptions.
- Portal source freshness polish.
- Policy consumption.
- Adapter execution and rollback proof.
