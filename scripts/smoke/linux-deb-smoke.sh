#!/usr/bin/env bash
set -euo pipefail

package_path="${1:?Usage: linux-deb-smoke.sh <package.deb>}"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
package_path="$(realpath "$package_path")"
package_dir="$(dirname "$package_path")"
package_file="$(basename "$package_path")"
sidecar_path="$package_path.sha256"
package_name="ocentra-parent-agent"
unit_path="/lib/systemd/system/ocentra-parent-agent.service"
binary_path="/opt/ocentra/ocentra-parent-agent/bin/ocentra-parent-agent-service"
results_dir="${OCENTRA_PARENT_LINUX_SMOKE_RESULTS_DIR:-$repo_root/test-results/linux-package-smoke}"
smoke_addr="127.0.0.1:${OCENTRA_PARENT_LINUX_SMOKE_PORT:-4577}"
health_url="http://$smoke_addr/health"
install_smoke="skipped"
agent_pid=""
extract_root=""
package_install_attempted="false"
smoke_log="$results_dir/linux-deb-smoke-$(date -u +%Y%m%dT%H%M%SZ).log"

mkdir -p "$results_dir"
exec > >(tee "$smoke_log") 2>&1

cleanup() {
  if [[ -n "$agent_pid" ]] && kill -0 "$agent_pid" >/dev/null 2>&1; then
    kill "$agent_pid" >/dev/null 2>&1 || true
    wait "$agent_pid" >/dev/null 2>&1 || true
  fi
  if [[ -n "$extract_root" ]]; then
    rm -rf "$extract_root"
  fi
  if [[ "$package_install_attempted" == "true" ]] && sudo -n true >/dev/null 2>&1; then
    sudo dpkg -r "$package_name" >/dev/null 2>&1 || true
    sudo dpkg -P "$package_name" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

fail() {
  echo "linux-deb-smoke-failed:$*" >&2
  exit 1
}

field() {
  dpkg-deb --field "$package_path" "$1"
}

require_field() {
  local name="$1"
  local expected="$2"
  local actual
  actual="$(field "$name")"
  if [[ "$actual" != "$expected" ]]; then
    fail "field $name expected [$expected] but observed [$actual]"
  fi
}

host_glibc="$(getconf GNU_LIBC_VERSION 2>/dev/null | awk '{print $2}' || true)"
if [[ -z "$host_glibc" ]] && command -v ldd >/dev/null 2>&1; then
  host_glibc="$(ldd --version | sed -n '1s/.* //p')"
fi
if [[ -z "$host_glibc" ]]; then
  fail "host glibc could not be detected"
fi

test -f "$package_path" || fail "package not found: $package_path"
test -f "$sidecar_path" || fail "sha256 sidecar not found: $sidecar_path"
(cd "$package_dir" && sha256sum --check "$(basename "$sidecar_path")")

require_field Package "$package_name"
require_field Architecture amd64
require_field Depends "libc6 (>= 2.35)"
baseline="$(field X-Ocentra-Linux-Baseline)"
required_glibc="$(field X-Ocentra-Min-GLIBC)"
build_glibc="$(field X-Ocentra-Build-GLIBC)"
if [[ "$baseline" != "ubuntu-22.04" ]]; then
  fail "Linux package baseline must be ubuntu-22.04, observed [$baseline]"
fi
if [[ "$required_glibc" != "2.35" ]]; then
  fail "Linux package minimum glibc must be 2.35, observed [$required_glibc]"
fi
if [[ "$build_glibc" != "2.35" ]]; then
  fail "Linux package build glibc must be 2.35, observed [$build_glibc]"
fi
if [[ "$(printf '%s\n%s\n' "$required_glibc" "$host_glibc" | sort -V | head -n 1)" != "$required_glibc" ]]; then
  fail "host glibc $host_glibc is older than package requirement $required_glibc"
fi

contents_path="$results_dir/${package_file}.contents.txt"
dpkg-deb --contents "$package_path" > "$contents_path"
grep -F "./lib/systemd/system/ocentra-parent-agent.service" "$contents_path" >/dev/null
grep -F "./opt/ocentra/ocentra-parent-agent/bin/ocentra-parent-agent-service" "$contents_path" >/dev/null

extract_root="$(mktemp -d)"
dpkg-deb --extract "$package_path" "$extract_root"
test -f "$extract_root$unit_path"
test -x "$extract_root$binary_path"
grep -F "ExecStart=$binary_path" "$extract_root$unit_path" >/dev/null
grep -F "Environment=OCENTRA_PARENT_AGENT_ADDR=127.0.0.1:4477" "$extract_root$unit_path" >/dev/null

health_path="$results_dir/${package_file}.health.json"
OCENTRA_PARENT_AGENT_ADDR="$smoke_addr" "$extract_root$binary_path" &
agent_pid="$!"
for _ in {1..50}; do
  if curl --fail --silent "$health_url" > "$health_path"; then
    break
  fi
  if ! kill -0 "$agent_pid" >/dev/null 2>&1; then
    fail "extracted agent exited before health check"
  fi
  sleep 0.2
done
test -s "$health_path" || fail "health endpoint did not respond: $health_url"
kill "$agent_pid" >/dev/null 2>&1 || true
wait "$agent_pid" >/dev/null 2>&1 || true
agent_pid=""

if sudo -n true >/dev/null 2>&1; then
  install_smoke="ran"
  package_install_attempted="true"
  sudo dpkg -i "$package_path"

  dpkg-query -W "$package_name" >/dev/null
  test -f "$unit_path"
  test -x "$binary_path"

  sudo dpkg -r "$package_name"

  package_state="$(dpkg-query -W -f='${db:Status-Abbrev}' "$package_name" 2>/dev/null || true)"
  if [[ "$package_state" == i* ]]; then
    fail "Package remained installed after remove: $package_name [$package_state]"
  fi
  if [[ -e "$binary_path" ]]; then
    fail "Agent executable remained after remove."
  fi
  sudo dpkg -P "$package_name" >/dev/null 2>&1 || true
  package_install_attempted="false"
else
  echo "Skipping install/remove smoke because passwordless sudo is unavailable."
fi

echo "linux-deb-smoke-ok:$package_path baseline=$baseline glibc=$required_glibc host_glibc=$host_glibc install=$install_smoke log=$smoke_log"
