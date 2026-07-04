## Platform proof status

- Windows proof expected: yes
- Windows proof status: passed locally on this host
- Android proof expected: no
- Android proof status: not relevant to this slice
- Linux proof expected: no
- Linux proof status: not relevant to this slice
- iOS proof expected: no
- iOS proof status: external-platform constraint / not relevant to this slice
- macOS proof expected: no
- macOS proof status: external-platform constraint / not relevant to this slice

## Note

- The provisioning-core slice is host-agnostic Rust logic, so the local Windows crate tests are the only platform proof needed here.
