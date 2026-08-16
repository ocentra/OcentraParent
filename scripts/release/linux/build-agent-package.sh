#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"

bootstrap_cargo_path() {
  if command -v cargo >/dev/null 2>&1; then
    return
  fi

  if [[ -f "${HOME}/.cargo/env" ]]; then
    # shellcheck disable=SC1090
    . "${HOME}/.cargo/env"
  fi

  if command -v cargo >/dev/null 2>&1; then
    return
  fi

  if [[ -x "${HOME}/.cargo/bin/cargo" ]]; then
    export PATH="${HOME}/.cargo/bin:${PATH}"
  fi
}

bootstrap_cargo_path
legacy_child_environment=(
  OCENTRA_PARENT_VERSION
  OCENTRA_PARENT_LINUX_STAGE_PARENT
  OCENTRA_PARENT_LINUX_BASELINE_ID
  OCENTRA_PARENT_LINUX_BASELINE_VERSION
  OCENTRA_PARENT_LINUX_GLIBC_MIN
  OCENTRA_PARENT_LINUX_ALLOW_NON_BASELINE
)
for legacy_name in "${legacy_child_environment[@]}"; do
  if [[ -n "${!legacy_name:-}" ]]; then
    echo "Refusing legacy parent-scoped child package input: $legacy_name. Use the OCENTRA_CHILD_LINUX_* controls." >&2
    exit 1
  fi
done
version="${OCENTRA_CHILD_LINUX_VERSION:-$(cd "$repo_root" && node scripts/release/validate-version.mjs --print-version)}"
package_root="$repo_root/target/release-packages/linux"
stage_parent="${OCENTRA_CHILD_LINUX_STAGE_PARENT:-${TMPDIR:-/tmp}}"
stage_root=""
package_name="ocentra-child-agent-linux-amd64-v${version}.deb"
package_path="$package_root/$package_name"
latest_name="ocentra-child-agent-linux-amd64-latest.deb"
latest_path="$package_root/$latest_name"
baseline_id="${OCENTRA_CHILD_LINUX_BASELINE_ID:-ubuntu}"
baseline_version="${OCENTRA_CHILD_LINUX_BASELINE_VERSION:-22.04}"
baseline_glibc_min="${OCENTRA_CHILD_LINUX_GLIBC_MIN:-2.35}"
baseline_label="${baseline_id}-${baseline_version}"
baseline_metadata_path="$package_root/linux-baseline.json"

host_id="unknown"
host_version="unknown"
host_pretty="unknown"
if [[ -r /etc/os-release ]]; then
  # shellcheck disable=SC1091
  . /etc/os-release
  host_id="${ID:-unknown}"
  host_version="${VERSION_ID:-unknown}"
  host_pretty="${PRETTY_NAME:-unknown}"
fi

host_glibc="$(getconf GNU_LIBC_VERSION 2>/dev/null | awk '{print $2}' || true)"
if [[ -z "$host_glibc" ]] && command -v ldd >/dev/null 2>&1; then
  host_glibc="$(ldd --version | sed -n '1s/.* //p')"
fi
if [[ -z "$host_glibc" ]]; then
  host_glibc="unknown"
fi

if ! command -v cargo >/dev/null 2>&1; then
  cat >&2 <<CARGO_ERROR
Linux package builds require a Linux cargo toolchain on PATH.
The current non-login shell could not resolve cargo even after bootstrapping ${HOME}/.cargo.
Install Rust in this Linux environment or source the Linux cargo toolchain before rerunning.
CARGO_ERROR
  exit 1
fi

allow_non_baseline="${OCENTRA_CHILD_LINUX_ALLOW_NON_BASELINE:-false}"
if [[ "$allow_non_baseline" != "true" ]]; then
  if [[ "$host_id" != "$baseline_id" || "$host_version" != "$baseline_version" || "$host_glibc" != "$baseline_glibc_min" ]]; then
    cat >&2 <<BASELINE_ERROR
Linux package builds must run on ${baseline_label} with glibc ${baseline_glibc_min}.
Observed host: ${host_pretty}; ID=${host_id}; VERSION_ID=${host_version}; glibc=${host_glibc}.
Use the package-preview linux-deb job or a matching baseline builder for release proof.
Set OCENTRA_CHILD_LINUX_ALLOW_NON_BASELINE=true only for local unsupported experiments.
BASELINE_ERROR
    exit 1
  fi
fi

cleanup_stage() {
  if [[ -n "$stage_root" ]]; then
    rm -rf "$stage_root"
  fi
}
trap cleanup_stage EXIT

mkdir -p "$package_root"
mkdir -p "$stage_parent"
stage_root="$(mktemp -d "$stage_parent/ocentra-parent-linux-package.XXXXXX")"
mkdir -p "$stage_root/DEBIAN"
mkdir -p "$stage_root/opt/ocentra/ocentra-child-agent/bin"
mkdir -p "$stage_root/lib/systemd/system"
mkdir -p "$stage_root/var/lib/ocentra/ocentra-child-agent"
mkdir -p "$stage_root/var/log/ocentra/ocentra-child-agent"

(cd "$repo_root" && cargo build --release -p ocentra-child-runtime --bin ocentra-child-agent-service)

install -m 0755 "$repo_root/target/release/ocentra-child-agent-service" \
  "$stage_root/opt/ocentra/ocentra-child-agent/bin/ocentra-child-agent-service"
install -m 0644 "$repo_root/scripts/release/linux/ocentra-parent-agent.service" \
  "$stage_root/lib/systemd/system/ocentra-child-agent.service"

cat > "$stage_root/DEBIAN/control" <<CONTROL
Package: ocentra-child-agent
Version: $version
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Ocentra <support@ocentra.ca>
Depends: libc6 (>= $baseline_glibc_min)
X-Ocentra-Linux-Baseline: $baseline_label
X-Ocentra-Min-GLIBC: $baseline_glibc_min
X-Ocentra-Build-GLIBC: $host_glibc
Description: Headless local child agent for Ocentra Parent.
CONTROL

cat > "$stage_root/DEBIAN/postinst" <<'POSTINST'
#!/usr/bin/env bash
set -e
if command -v systemctl >/dev/null 2>&1; then
  systemctl daemon-reload
  systemctl enable ocentra-child-agent.service >/dev/null 2>&1 || true
  systemctl restart ocentra-child-agent.service >/dev/null 2>&1 || true
fi
POSTINST

cat > "$stage_root/DEBIAN/prerm" <<'PRERM'
#!/usr/bin/env bash
set -e
if command -v systemctl >/dev/null 2>&1; then
  systemctl stop ocentra-child-agent.service >/dev/null 2>&1 || true
  systemctl disable ocentra-child-agent.service >/dev/null 2>&1 || true
fi
PRERM

cat > "$stage_root/DEBIAN/postrm" <<'POSTRM'
#!/usr/bin/env bash
set -e
if command -v systemctl >/dev/null 2>&1; then
  systemctl daemon-reload
fi
POSTRM

chmod 0755 "$stage_root/DEBIAN/postinst" "$stage_root/DEBIAN/prerm" "$stage_root/DEBIAN/postrm"
dpkg-deb --build --root-owner-group "$stage_root" "$package_path"
cp "$package_path" "$latest_path"

cat > "$baseline_metadata_path" <<BASELINE_JSON
{
  "package": "$package_name",
  "latest": "$latest_name",
  "baseline": "$baseline_label",
  "minimumGlibc": "$baseline_glibc_min",
  "buildHost": {
    "prettyName": "$host_pretty",
    "id": "$host_id",
    "versionId": "$host_version",
    "glibc": "$host_glibc"
  },
  "nonBaselineOverride": "$allow_non_baseline"
}
BASELINE_JSON

(cd "$package_root" && sha256sum "$package_name" > "$package_name.sha256")
(cd "$package_root" && sha256sum "$latest_name" > "$latest_name.sha256")

echo "Built $package_path"
echo "Built $latest_path"
echo "Baseline metadata $baseline_metadata_path"
