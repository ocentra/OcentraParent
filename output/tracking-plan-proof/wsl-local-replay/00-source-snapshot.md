# WSL Local Replay Proof

Checked at: 2026-06-04T05:44:00.869Z
Commit: d3e137b2e034bfd8cfff06e91aefe48165354b87
Status: partial_proof_blocked_by_wsl_optional_node_dependency
Product claim ready: false

## Command Results

- build-contracts: exit 0; blocker=none; log=output/tracking-plan-proof/wsl-local-replay/build-contracts.log
- service-read-model-proof: exit 1; blocker=wsl_linux_optional_node_dependency_missing; log=output/tracking-plan-proof/wsl-local-replay/service-read-model-proof.log
- rust-core-read-model-test: exit 0; blocker=none; log=output/tracking-plan-proof/wsl-local-replay/rust-core-read-model-test.log

## Non-Claims

- WSL/local replay does not prove Android or iOS physical background behavior.
- WSL/local replay does not prove mobile permission grants, geofence delivery, killed-app behavior, reboot behavior, or OEM background reliability.
- WSL/local replay does not prove enrolled-device authority, notification provider delivery, hosted UI accessibility, or production pilot readiness.
