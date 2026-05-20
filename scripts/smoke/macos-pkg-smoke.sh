#!/usr/bin/env bash
set -euo pipefail

package_path="${1:?Usage: macos-pkg-smoke.sh <package.pkg>}"
expanded_root="$(mktemp -d)"
payload_file_list="$expanded_root/payload-files.txt"

cleanup() {
  rm -rf "$expanded_root"
}
trap cleanup EXIT

pkgutil --expand-full "$package_path" "$expanded_root/pkg"
pkgutil --payload-files "$package_path" > "$payload_file_list"

grep -F "Library/Ocentra/Ocentra Parent Agent/bin/ocentra-parent-agent-service" "$payload_file_list" >/dev/null
grep -F "Library/LaunchDaemons/ca.ocentra.parent.agent.plist" "$payload_file_list" >/dev/null
find "$expanded_root/pkg" -name postinstall -type f -perm -111 | grep . >/dev/null
find "$expanded_root/pkg" -name preinstall -type f -perm -111 | grep . >/dev/null

echo "macos-pkg-smoke-ok:$package_path"
