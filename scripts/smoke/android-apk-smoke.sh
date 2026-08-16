#!/usr/bin/env bash
set -euo pipefail

apk_path="${1:?Usage: android-apk-smoke.sh <package.apk>}"
package_name="${2:-ca.ocentra.parent.agent}"
activity_name="${3:-$package_name/.MainActivity}"

adb install -r "$apk_path"
adb shell pm list packages "$package_name" | grep -F "package:$package_name" >/dev/null
adb shell am start -n "$activity_name"
sleep 2
adb shell pidof "$package_name" >/dev/null
adb uninstall "$package_name"

echo "android-apk-smoke-ok:$apk_path"
