#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
version="${OCENTRA_PARENT_VERSION:-$(cd "$repo_root" && node scripts/release/validate-version.mjs --print-version)}"
package_root="$repo_root/target/release-packages/macos"
payload_root="$package_root/payload"
scripts_root="$package_root/scripts"
package_name="ocentra-child-agent-macos-v${version}.pkg"
package_path="$package_root/$package_name"
latest_name="ocentra-child-agent-macos-latest.pkg"
latest_path="$package_root/$latest_name"

rm -rf "$payload_root" "$scripts_root"
mkdir -p "$payload_root/Library/Ocentra/Ocentra Child Agent/bin"
mkdir -p "$payload_root/Library/LaunchDaemons"
mkdir -p "$scripts_root"
mkdir -p "$package_root"

(cd "$repo_root" && cargo build --release -p ocentra-child-runtime --bin ocentra-child-agent-service)

install -m 0755 "$repo_root/target/release/ocentra-child-agent-service" \
  "$payload_root/Library/Ocentra/Ocentra Child Agent/bin/ocentra-child-agent-service"
install -m 0644 "$repo_root/scripts/release/macos/ca.ocentra.parent.agent.plist" \
  "$payload_root/Library/LaunchDaemons/ca.ocentra.child.agent.plist"

cat > "$scripts_root/postinstall" <<'POSTINSTALL'
#!/usr/bin/env bash
set -e
launchctl bootout system /Library/LaunchDaemons/ca.ocentra.child.agent.plist >/dev/null 2>&1 || true
launchctl bootstrap system /Library/LaunchDaemons/ca.ocentra.child.agent.plist >/dev/null 2>&1 || true
launchctl enable system/ca.ocentra.child.agent >/dev/null 2>&1 || true
POSTINSTALL

cat > "$scripts_root/preinstall" <<'PREINSTALL'
#!/usr/bin/env bash
set -e
launchctl bootout system /Library/LaunchDaemons/ca.ocentra.child.agent.plist >/dev/null 2>&1 || true
PREINSTALL

chmod 0755 "$scripts_root/postinstall" "$scripts_root/preinstall"
pkgbuild \
  --root "$payload_root" \
  --scripts "$scripts_root" \
  --identifier "ca.ocentra.child.agent" \
  --version "$version" \
  --install-location "/" \
  "$package_path"

cp "$package_path" "$latest_path"
shasum -a 256 "$package_path" > "$package_path.sha256"
shasum -a 256 "$latest_path" > "$latest_path.sha256"

echo "Built $package_path"
echo "Built $latest_path"
