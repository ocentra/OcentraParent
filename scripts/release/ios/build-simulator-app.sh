#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
if [[ -n "${OCENTRA_PARENT_VERSION:-}" ]]; then
  echo 'Refusing legacy parent-scoped iOS child package version input. Use OCENTRA_CHILD_IOS_VERSION.' >&2
  exit 1
fi
version="${OCENTRA_CHILD_IOS_VERSION:-$(cd "$repo_root" && node scripts/release/validate-version.mjs --print-version)}"
package_root="$repo_root/target/release-packages/ios"
derived_data="$repo_root/target/ios-derived-data"
app_path="$derived_data/Build/Products/Debug-iphonesimulator/OcentraChildAgent.app"
zip_name="ocentra-child-agent-ios-simulator-v${version}.zip"
zip_path="$package_root/$zip_name"
latest_path="$package_root/ocentra-child-agent-ios-simulator-latest.zip"

rm -rf "$derived_data"
mkdir -p "$package_root"

xcodebuild \
  -project "$repo_root/platforms/ios/OcentraChildAgent.xcodeproj" \
  -scheme OcentraChildAgent \
  -configuration Debug \
  -sdk iphonesimulator \
  -derivedDataPath "$derived_data" \
  CODE_SIGNING_ALLOWED=NO \
  build

ditto -c -k --sequesterRsrc --keepParent "$app_path" "$zip_path"
cp "$zip_path" "$latest_path"
shasum -a 256 "$zip_path" > "$zip_path.sha256"
shasum -a 256 "$latest_path" > "$latest_path.sha256"

echo "Built $zip_path"
echo "Built $latest_path"
