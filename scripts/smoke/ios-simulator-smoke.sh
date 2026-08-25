#!/usr/bin/env bash
set -euo pipefail

app_path="${1:?Usage: ios-simulator-smoke.sh <OcentraChildAgent.app>}"
bundle_id="${2:-ca.ocentra.child.agent}"
device_udid="$(xcrun simctl list devices available | awk -F '[()]' '/iPhone/ { print $2; exit }')"

if [[ -z "$device_udid" ]]; then
  echo "No available iPhone simulator device found." >&2
  exit 1
fi

xcrun simctl boot "$device_udid" >/dev/null 2>&1 || true
xcrun simctl bootstatus "$device_udid" -b
xcrun simctl install "$device_udid" "$app_path"
xcrun simctl launch "$device_udid" "$bundle_id" >/dev/null
xcrun simctl terminate "$device_udid" "$bundle_id" >/dev/null 2>&1 || true
xcrun simctl uninstall "$device_udid" "$bundle_id" >/dev/null 2>&1 || true
xcrun simctl shutdown "$device_udid" >/dev/null 2>&1 || true

echo "ios-simulator-smoke-ok:$app_path"
