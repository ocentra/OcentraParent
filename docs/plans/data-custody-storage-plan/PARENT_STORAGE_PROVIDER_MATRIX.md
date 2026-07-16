<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `PARENT_STORAGE_PROVIDER_MATRIX.md`
> Kind: provider custody matrix.
> Read when: When a workpack needs the provider-mode split and sync/delete behavior.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Provider modes here must match the bundle, sync, and UI docs.

<!-- /agent-capsule -->

# Data Custody Storage Plan Parent Storage Provider Matrix

| Mode | Scope model | Metadata leakage | Revocation behavior | Quota behavior | Delete behavior | Restore behavior | Manual-required state | Proof required |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `google-drive-appdata` | Hidden app-managed backup | File name, size, timestamps, account metadata | Provider access may expire; app must stop writing | Quota can block writes | App and provider delete are distinct | Restore via encrypted bundle only | Permission missing or revoked | `data-custody.sync.provider-capability-matrix` |
| `google-drive-picker-file` | Parent-visible selected file or folder | More visible file metadata | Parent can revoke folder access | Quota can block writes | Parent-visible delete path required | Restore via encrypted bundle only | Folder missing or permission revoked | `data-custody.sync.provider-capability-matrix` |
| `onedrive-approot` | App folder under provider account | App folder metadata only | App folder access may be revoked | Quota can block writes | App and provider delete are distinct | Restore via encrypted bundle only | App folder unavailable or revoked | `data-custody.sync.provider-capability-matrix` |
| `onedrive-parent-selected-folder` | Parent-visible folder | More visible file metadata | Parent can revoke folder access | Quota can block writes | Parent-visible delete path required | Restore via encrypted bundle only | Folder missing or permission revoked | `data-custody.sync.provider-capability-matrix` |
| `icloud-drive-app-container` | App container after Apple proof | App container metadata | Apple permission loss blocks writes | Quota can block writes | App and provider delete are distinct | Restore via encrypted bundle only | Manual-required until proven | `data-custody.sync.provider-capability-matrix` |
| `icloud-drive-parent-selected-location` | Parent-visible location | More visible file metadata | Parent can revoke folder access | Quota can block writes | Parent-visible delete path required | Restore via encrypted bundle only | Manual-required until proven | `data-custody.sync.provider-capability-matrix` |
| `dropbox-app-folder` | App folder | App folder metadata only | App folder access may be revoked | Quota can block writes | App and provider delete are distinct | Restore via encrypted bundle only | Manual-required until proven | `data-custody.sync.provider-capability-matrix` |
| `dropbox-parent-selected-folder` | Parent-visible folder | More visible file metadata | Parent can revoke folder access | Quota can block writes | Parent-visible delete path required | Restore via encrypted bundle only | Manual-required until proven | `data-custody.sync.provider-capability-matrix` |
| `nas-folder` | Local or network filesystem | Network path and file metadata | Access loss blocks sync | Local disk or network quota | File delete is separate from source delete | Restore via encrypted bundle only | Network or mount unavailable | `data-custody.sync.provider-capability-matrix` |
| `local-folder` | Local filesystem | Local file metadata only | Local access loss blocks sync | Local disk quota | File delete is separate from source delete | Restore via encrypted bundle only | Folder missing or unavailable | `data-custody.sync.provider-capability-matrix` |
| `disabled` | No provider selected | No provider metadata | No provider access | No provider quota | No provider delete | Restore limited to local or imported bundles | Disabled | `data-custody.sync.no-ocentra-default-store-negative` |

## Provider mode rules

- Hidden app-managed backup and parent-visible folder modes are different custody models.
- Provider selection must be explicit. "Cloud" is not a mode.
- Provider delete never implies local delete.
- Provider disconnect never implies provider delete.
- Provider success never means the bundle is readable without keys.
- Metadata leakage must be disclosed in UI copy.

