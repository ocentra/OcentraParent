<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `PLATFORM_KEY_CUSTODY_MATRIX.md`
> Kind: platform custody matrix.
> Read when: When a workpack needs the platform-specific key store and manual-required states.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Platform claims must stay below the proven platform line.

<!-- /agent-capsule -->

# Data Custody Storage Plan Platform Key Custody Matrix

| Platform or surface | Key store or custody | Decrypt authority | Manual-required states | Notes |
| --- | --- | --- | --- | --- |
| Windows | DPAPI or Windows user or machine scope, depending on role | Parent desktop and child service only when explicitly provisioned | Same user, same machine, service account, other user rejected, reinstall/recovery | First proof target |
| macOS | Keychain or Secure Enclave-backed where available | Parent desktop only when explicitly provisioned | Access group missing, key unavailable, manual-required | App group sharing must be explicit |
| Linux | Secret Service, KWallet, or fallback only after decision | Manual-required until the secret store is chosen | Headless no keyring, passphrase required, unsupported | Do not silently fall back to plaintext |
| Android | Android Keystore or equivalent proof path | Parent mobile or child mobile only when proven | Hardware-backed unavailable, user-auth required, key invalidated, backup excluded | Child support remains limited until proof |
| iOS | Keychain and device security services | Parent mobile only when proven | Access group missing, biometric or passcode required, manual-required | Child support remains limited until proof |
| Web or hosted portal | No local decrypt root by default | Not the decrypt root | Manual-required | Orchestrates status only |
| Parent desktop | Parent-owned local key path | Parent decrypt authority | Key unavailable, revoked, wrong household, manual-required | Primary near-term parent custody surface |
| Child service | Child-device local key path | Child-device local evidence only | Key unavailable, revoked, wrong device, manual-required | Owns local evidence execution |
| Parent mobile | Parent-owned approval and view path | Parent-owned bundles or approved decrypt path only | Auth expired, key unavailable, manual-required | View and approval role |
| Child mobile | Platform-limited child custody path | Only if proven | Manual-required until device proof exists | No broad custody claim without proof |

## Platform rules

- Windows is the first proof target.
- Linux remains manual-required until the secret-store choice is explicit.
- Android and iOS remain limited until device proof exists.
- The hosted portal never becomes the decrypt root by default.
- A platform that cannot prove secure key custody must stay manual-required.

## Proof anchor

- `data-custody.keys.platform-custody-matrix`

