# WSL Local Replay Proof

Checked at: 2026-06-04T18:16:10.590Z
Commit: ff171f0a87c504a6ad2345f79bd2a4ba2f3b6f1d
Status: proved
Product claim ready: false

## Command Results

- rolldown-linux-binding-probe: exit 0; blocker=none; log=output/tracking-plan-proof/wsl-local-replay/rolldown-linux-binding-probe.log
- build-contracts: exit 0; blocker=none; log=output/tracking-plan-proof/wsl-local-replay/build-contracts.log
- service-read-model-proof: exit 0; blocker=none; log=output/tracking-plan-proof/wsl-local-replay/service-read-model-proof.log
- rust-core-read-model-test: exit 0; blocker=none; log=output/tracking-plan-proof/wsl-local-replay/rust-core-read-model-test.log

## Non-Claims

- WSL/local replay does not prove Android or iOS physical background behavior.
- WSL/local replay does not prove mobile permission grants, geofence delivery, killed-app behavior, reboot behavior, or OEM background reliability.
- WSL/local replay does not prove enrolled-device authority, notification provider delivery, hosted UI accessibility, or production pilot readiness.
