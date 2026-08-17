# Repository Custody Status

This is the current Git/worktree/stash custody record for the consolidated
Ocentra Parent program. It records recoverability and promotion state; it is
not plan completion, test proof, or release approval.

## Current truth - 2026-08-17

| Surface | Verified state |
| --- | --- |
| Consolidated source | The current integration lane is `codex/eventing-wp09-production` at pushed checkpoint `117e18603b73e3ba279cdadec9e7a686289b10a4`. Its E-drive worktree also contains an uncommitted Device Trust WP01/WP03, Eventing WP11, Account WP08, and graph-routing batch; that batch is recoverable only from this machine until its review checkpoint is pushed. The older coordination lane `codex/app-game-plan-code-audit` remains at pushed ancestor `b730a3f6a63515666e2a8f3c87812f1584dc1a49`. |
| Protected bases | `main` is `eb4e66a791`; `develop` is `4ece515282`; `production` is `683a07c`. `develop` is 3 commits ahead of `main`; the pushed integration checkpoint is 204 commits ahead of `main` and 201 ahead of `develop`, with neither protected base containing a unique commit outside its ancestry. |
| Pull requests | Zero open pull requests. No current CI or merge claim exists for the integration checkpoint; PR CI starts only after a scoped promotion PR is opened. |
| Local work branches | `codex/app-game-plan-code-audit` and `codex/eventing-wp09-production` only. |
| Registered worktrees | `E:/OcentraParent` at `b730a3f6a` and `E:/OcentraWorktrees/lanes/eventing-wp09-production` at pushed checkpoint `117e18603`. No registered OcentraParent worktree exists on C:. |
| C drive | No OcentraParent worktree remains under `C:/Users/sujan/.codex/worktrees`; only unrelated Enforcer and Ocentra Games entries remain there. |
| Remote work branches | `origin/codex/app-game-plan-code-audit` is `b730a3f6a`; `origin/codex/eventing-wp09-production` is `117e18603`. Both local branches match their remote checkpoints. |
| Remote safety archives | Protected `codex/archive/all-remote-tips-20260815` plus protected complete archive `codex/archive/all-local-remote-tips-20260816` at `ac9f65bb4`. |
| Stashes | One local stash remains: the intentional Eventing WP09 draft. The 129 rejected historical entries were dropped locally only after all 130 commits were made reachable from the complete 2026-08-16 remote archive. |
| Executable graph | Valid at 705 nodes / 744 edges, 23 plans / 681 workpacks, with states 366 planned, 21 blocked, 2 ready, 2 active, 289 validation, and 1 done. It maps 2,961 implementation files and 1,145 test files across 680 reviewed workpack maps; mapping is not semantic completion. |

## Safety archive

`origin/codex/archive/all-local-remote-tips-20260816` is an isolated custody
ref, not an integration branch. Its synthetic commit chain retains 146 unique
objects: all branch tips selected for cleanup, the prior archive, the
local-only tracking tip, the consolidated/Eventing tips, and all 130 stash
commits including their tracked, index, and untracked parents. Reachability was
verified 146/146 after push. The branch is protected against deletion and force
push. Never merge this archive into `develop`, `main`, or `production`.

The older `codex/archive/all-remote-tips-20260815` archive does not contain the
newer branch tips or stashes. GitHub rejected its deletion because it is
protected, so it remains as an additional historical safety point.

## Branch and worktree reconciliation

Thirteen superseded remote work branches and fourteen superseded local branches
were removed only after patch-equivalence/source review and after the complete
archive was pushed and verified. Their production fixes were already present
in the consolidated head; stale branch baselines were not merged. The sole
local-only tracking tip was an ancestor of the consolidated head.

No short-lived audit checkout or stale physical OcentraParent copy remains.
The Eventing lane is intentionally retained as the single integration lane.
WP09 is now in validation; Device Trust WP01 and Eventing WP11 are READY.
Account Identity WP08 and Cloudflare WP01 are in validation with independently
reviewed implementation-only evidence. Cloudflare WP06 is authorized only for
its next source packet; normal readiness remains dependency-blocked. The lane's
last remote-safe checkpoint is `117e18603`; its current dirty Device Trust,
Eventing, Account, and graph-routing batch must be committed and pushed before
any promotion. The older WP09 draft stash remains at
`3f785b7f5832c303ac759fa15b4e1625e58296e5` and is also remote-archived.

## Stash semantic decision

- Stash 0 is the one intentional live draft: Eventing WP09 durable network
  ingestion/journal recovery. It remains uncommitted and is not a completion
  claim.
- Stashes 1-129 were reviewed against current production owners. Candidate
  code was superseded, unshipped, coordination-only, static/proof read-model
  work, or explicitly non-claim/synthetic behavior. Even the broad browser,
  app-game, tracking, LAN, Screen AI, custody, and delivery snapshots contain
  no smallest real production patch absent from current Rust owners.
- No rejected stash should be transplanted. Their SHAs remain in the protected
  archive for historical recovery and audit.

Local cleanup dropped 129 rejected entries without failure. The surviving local
stash is `3f785b7f5832c303ac759fa15b4e1625e58296e5`.

## Promotion boundary

The requested feature -> `develop` -> `main` promotion system already exists;
no workflow or branch-policy change is currently required. `develop` and
`main` are protected with strict up-to-date checks, pull-request
review flow, conversation resolution, no force push, and no deletion. Both
require `Format, Lint, Types, Rust Check`, `Full Validation Gate`, and
`Package Preview Gate`. Product work should reach `develop` through a normal
green PR; `develop` then promotes to `main` through a separate normal green PR,
followed by a normal `main` -> `develop` back-sync PR. Main-bound proof must
come from a PR whose base is `main`; develop CI is not treated as a substitute.
`production` remains a separate release boundary owned by `release.yml`.

The consolidated head is not being promoted yet. The agreed order remains:
finish real production code, write/repair expected tests, run focused
validation and Enforcer, retain proof and run normal pre-commit, then open the
plan/integration PR and pay for full CI once.

## Complete pre-cleanup stash manifest

The SHA is authoritative after local stash indexes change. Every SHA below is
reachable from `origin/codex/archive/all-local-remote-tips-20260816`.

| Original local index | Stash commit | Original subject |
| --- | --- | --- |
| stash@{0} | `3f785b7f5832c303ac759fa15b4e1625e58296e5` | On codex/eventing-wp09-production: draft WP09 durable network ingestion journal recovery 2026-08-16 |
| stash@{1} | `fcddf7ddab95221ace5b7100d8efc447461c902f` | autostash |
| stash@{2} | `d0b7566bf3cc1d6e7a5146988f38b7f07a80665a` | On codex/authenticated-delivery-consumer-hardening: codex-authenticated-consumer-hardening-before-main-rebase |
| stash@{3} | `9d5a3345e33b370c9096927ad2e4579af89f654b` | On codex/custody-authority-event-flow: codex-wp07-authority-event-flow-staged |
| stash@{4} | `0cd7abf91460fad21fc27c778e209ab755a9f585` | On codex/lan-runtime-local-helper-coverage: archive before post-566 cleanup: lan runtime helper split |
| stash@{5} | `a2d5beea315a232fef22c630dc0f78ff250fe57c` | autostash |
| stash@{6} | `eebe3bf754fae385c8e47b8faf496b3eae7078d4` | On codex/tracking-plan-full-continuation-a: codex pre-rebase wip |
| stash@{7} | `65295605bd11740f2d6295794f53af3b3b5fdd04` | On codex/screen-ai-full-scope-b: temporary hub-state stash before pr545 main update |
| stash@{8} | `bb3bae87b48704b8112a8c434c872c29425c35a2` | On codex/hub-event-ledger-architecture: paused hub event ledger architecture migration |
| stash@{9} | `7d97e4672b9cfcb64eabcc8f3018136e9923f2f3` | On codex/network-live-capture-execution-proof: E-D raw artifact store parked due codex-a lock |
| stash@{10} | `d40e7cc14469de9590d980dcf27fb31fbaa5573e` | On codex/e-c-production-support-provider-secret-rotation-revocation-status-proof: pre-rebase provider-secret proof output drift |
| stash@{11} | `0984e718a25cc2e93d754bbdb8517fdb668b64c2` | On codex/screen-ai-full-scope-b: degraded-screen-service-event-bridge-wip-blocked-by-ed-lib-export |
| stash@{12} | `94fe7981cb40bece6621eb198a0cac6aca4eb4a5` | On codex/screen-ai-full-scope-b: degraded-screen-service-event-bridge-before-main-sync |
| stash@{13} | `6d2a917380271561baeb69d8a2ee7fe875317aca` | On codex/network-row10g-remote-outbox-handoff-on-row10f: row10g-proof-after-pr518-rebase |
| stash@{14} | `a9c700daabc38cdda6cff4e2359a3507b12d9ede` | On codex/network-row10f-remote-delivery-status-bridge-on-row10e: row10f remote delivery status bridge WIP before PR502 restack |
| stash@{15} | `3559b1f4c4b514ebaa11264ad3d01baf116f2866` | On codex/e-c-production-support-status-backend-payload-custody-proof: payload custody deterministic proof before PR502 rebase |
| stash@{16} | `443e0a8eb69ce10b195b4aba362c412b8195bbe4` | On codex/network-row10f-remote-delivery-status-bridge-on-row10e: row10f remote delivery status bridge WIP before PR501 restack |
| stash@{17} | `b80989bca24a9022d13487a86914a3b5a1ad7901` | On codex/e-c-production-support-status-backend-delete-executor-proof: E-C delete executor superseded by payload custody assignment |
| stash@{18} | `dbd07b7a455dea70bd0a506dbbc7099c1b462e03` | On codex/network-row10f-remote-delivery-status-bridge-on-row10e: row10f remote delivery status bridge WIP before row10e restack |
| stash@{19} | `36b0aad53b2fdb8310bbdd2dd526afc3edb7d626` | On codex/network-row10e-durable-envelope-on-row10d: row10e durable envelope wip before row10d proof stability fix |
| stash@{20} | `c51b9ac894c676dd19d8811d63cac6bd2925f2a2` | On codex/screen-ai-full-scope-b: codex-b-screen-settings-persistence-wip |
| stash@{21} | `7a3102ae271d50c8a26e8527e8b02d902a96e536` | On codex/network-parser-fixture-proof-artifacts: wip network parser fixture proof artifacts deferred for doc overlap |
| stash@{22} | `3a6d15297ed72d0ed7ad4fe4427fe851dc6f1efd` | On codex/e-b-app-install-product-claim-store-upgrade-readiness-proof: E-B old PR477 proof metadata drift |
| stash@{23} | `0c85811f0e6f20b53c6f1d1f621454af85fe42bd` | On codex/browser-child-intervention-endpoint-flow: codex-d GAME-03 live portal pattern proof before main sync |
| stash@{24} | `06287b3db180065c3b265f72291509c65b927554` | On codex/network-remote-delivery-outbox-handoff-proof: row10h proof metadata after main rebase |
| stash@{25} | `f33da35a5eb7a1b4ca5862920248abbc8a3844a8` | On codex/browser-child-intervention-endpoint-flow: codex-d social-04 proof before main sync |
| stash@{26} | `80d1c01c6ebcb99f4df60d77a27a6d90cdcc7049` | On codex/browser-child-intervention-endpoint-flow: codex-d social-02 proof before a79e sync |
| stash@{27} | `f84128095d9c93ff92ec1d7eab3950eb1d5a197e` | On codex/app-game-timer-service-read-api-response-consumer-handoff-wp107: wp107-before-pr471-rebase-75cb334e |
| stash@{28} | `eb2f89d08ac25d2e175cf9dff8d6951f9b6eb929` | On codex/pr445-tracking-family-dashboard-hosted-ui-proof: stale-pr445-lint-fix-preserved-after-merge |
| stash@{29} | `238954c4bf277c40ae3115aa47ee779065dd7858` | On codex/app-game-timer-service-read-api-response-consumer-handoff-wp107: wp107-before-stacking-pr471-fix |
| stash@{30} | `c5e60cd03841e995cc25ee810e4d2eb18fa52583` | On codex/app-game-timer-service-read-api-response-consumer-handoff-wp107: wp107-before-pr471-ci-fix |
| stash@{31} | `5aec2c16c3c598b08c3378e05f15a88fd277ec74` | On codex/e-b-app-install-product-claim-store-handoff-proof: E-B store handoff WIP before PR470 conflict fix |
| stash@{32} | `94bb406a4a51ecc9f2ac5898493e784e9c710b28` | On codex/tracking-family-dashboard-hosted-ui-proof: codex-a hosted family dashboard UI proof before PR442 format fix |
| stash@{33} | `54958872aff4e50c1e8fb00d1617854d2bfb280c` | On codex/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff: wp93-read-api-response-handoff-wip |
| stash@{34} | `d6a94c08c3d4e2af083b19f50f42f15e0f02427b` | On codex/e-c-production-support-publication-execution-status-proof: e-c publication execution status wip before PR437 main sync |
| stash@{35} | `0215e7269c2daad5cb64a659f9311ef5bfcac3af` | On codex/e-c-production-support-publication-execution-status-proof: e-c publication execution status wip before PR436 main sync |
| stash@{36} | `c6dd7569d90bdd4f46d8bfd9913c2c22a2ce2ef2` | On codex/e-c-production-support-publication-execution-status-proof: e-c publication execution status wip before PR435 main sync |
| stash@{37} | `2c085dcd0f6d5c988dd590925f0e89d5e8ccfa16` | On codex/e-c-production-support-publication-execution-status-proof: e-c publication execution status wip before PR431 main sync |
| stash@{38} | `6e05cd79b0106c12348a41db78f3c92ed350d208` | On codex/local-ai-recent-memory-window-proof: codex-b local ai recent memory window proof before PR431 sync |
| stash@{39} | `022605987ab07cbca972dab90dad3c71ae9c1c43` | On codex/e-c-production-support-publication-execution-status-proof: e-c publication execution status wip before PR433 sync |
| stash@{40} | `dba36a3d58f8c8b11f1ed207737e911c368d642a` | On codex/local-ai-recent-memory-window-proof: codex-b local ai recent memory window proof before main sync |
| stash@{41} | `8d73949c27d3d3da4aca826a87a47712e91d28ec` | On codex/e-c-production-support-publication-execution-status-proof: e-c publication execution status wip before origin-main-1e96f9608 sync |
| stash@{42} | `6a9c37b27a7b3fb10b379c63a7eb36ece06596cf` | On codex/e-c-production-support-publication-execution-status-proof: e-c publication execution status wip before PR434 sync |
| stash@{43} | `6fe39a918af30547f517a098965a947b70c53c6b` | On codex/browser-child-intervention-endpoint-flow: preserve accidental production-support stash pop in codex-d |
| stash@{44} | `4a561cdb44083e583dd89ca8d2a1de20ec578383` | On codex/e-b-app-install-provider-store-execution-preflight-proof: e-b-provider-store-execution-preflight-wip |
| stash@{45} | `ce31d086d7014335a7b2a0480b25415b77fc5807` | On codex/e-c-production-support-publication-execution-status-proof: e-c publication execution status wip before PR431 sync |
| stash@{46} | `9f139cb55c33331ef689f23e15faa0f14eae92ea` | On codex/tracking-family-dashboard-rendered-rollup-ui-proof: codex-a tracking rendered rollup ui proof wip before PR426 sync |
| stash@{47} | `79fd1cce0d461f318ace6a8c74394929064e1174` | On codex/app-game-source-gated-policy-preview-timer-status: wp79-proof-before-pr423-sync |
| stash@{48} | `d56fa6b3983f198972322b45fc7e5393016f0e37` | On codex/e-b-app-install-platform-limitation-action-proof: e-b platform limitation proof refresh before pr420 sync |
| stash@{49} | `b704883eaca415bf251381525436f6325d6dfbab` | On codex/app-game-source-gated-policy-preview-timer-runtime-readiness: wp81-proof-before-pr422-sync |
| stash@{50} | `febc87e98b7a2353b616f1ed820d7401622dc0c0` | On codex/app-game-source-gated-policy-preview-timer-status: wp79 timer status before pr409 sync |
| stash@{51} | `85090a085d9942913dabf64a100facef727aa010` | On codex/app-game-source-gated-policy-preview-export-readiness: wp77 proof refresh before pr406 sync |
| stash@{52} | `419149782b0c42daa1da19b4cad82be85e2ac09c` | On codex/app-game-source-gated-policy-preview-export-readiness: wp77 proof refresh before pr405 sync |
| stash@{53} | `53c7be84cac74a7245188830be743146eb672ef3` | On codex/app-game-source-gated-policy-preview-read-model: wp76-before-wp75-branch-proof-metadata-fix |
| stash@{54} | `704bd8c80b5279bd5e18b18b7b3f94c9fbef9496` | On codex/app-game-source-gated-policy-preview-read-model: wp76-before-main-1620947e |
| stash@{55} | `97391822107d2183f68fc08f80a5453a7003182a` | On codex/app-game-source-gated-policy-preview-read-model: wp75-proof-refresh-before-pr400-rebase |
| stash@{56} | `3bfae9c573c2e3323437a4ba2c29379aef8c251d` | On codex/e-c-production-support-account-sla-status-proof: e-c account sla status proof wip after pr400 |
| stash@{57} | `e610d1480e498844957dead9417ddce6afdfcc3a` | On codex/app-game-source-gated-policy-preview-read-model: wp76-source-gated-policy-preview-read-model-before-wp75-rebase |
| stash@{58} | `4c711ea5c44210d8cd3f6cc8ec971924f23880d1` | On codex/screen-vlm-worker-contract-proof: preserve accidental network stash pop before VLM resume |
| stash@{59} | `e7a4b174c4871b4906b4185890e60ea71627d3ef` | On codex/app-game-source-freshness-quality-gate: codex-c-wp72-proof-refresh-before-wp71-build-order |
| stash@{60} | `9c86efc254d5ef60ea3cf7d5a430e51fd774eb07` | On codex/app-game-policy-preview-service-read-model: codex-c-wp71-proof-refresh-before-pr387-rebase |
| stash@{61} | `b3d9b486a9c270a7f5e14fc79d133358c268bb49` | On codex/browser-windows-registry-start-menu-proof: codex-d wp04 registry proof refresh after PR388 |
| stash@{62} | `0663ee383ad2401fc4fa16302977c721bb9fcb5c` | On codex/browser-ai-proof-gate-ui-delivery-proof: codex-d-ai25-proof-before-6059f536-rebase |
| stash@{63} | `0d4430848b12a2f822dae7cf0d5872386ec1a147` | On codex/app-game-policy-preview-service-read-model: WP71 policy preview service read model in progress |
| stash@{64} | `088d721905c47d0dfe47b1034c1e315896945935` | On codex/browser-ai-proof-gate-ui-delivery-proof: codex-d-ai25-proof-before-ba88c8d8-rebase |
| stash@{65} | `56e34b789cf33a05dd8b140156439e8f53ecdf9f` | On codex/browser-ai-proof-gate-ui-delivery-proof: codex-d-ai25-proof-before-0afa30e2-rebase |
| stash@{66} | `87d6de84d961ac65fe8bcdd535a62df4b8c61a72` | On codex/browser-ai-proof-gate-ui-delivery-proof: codex-d-ai25-proof-before-f4e1cd37-rebase |
| stash@{67} | `61b3b57c8c4e171ea02e33d9d32db9687bfc6729` | On codex/browser-ai-proof-gate-ui-delivery-proof: codex-d-ai25-proof-before-6e3a175d-rebase |
| stash@{68} | `a06d4d1b32b9a9f8f516e27bdc8d6db2917ab79d` | On codex/tracking-missing-device-mode-proof: wp29 generated proof artifacts before main rebase |
| stash@{69} | `301a19cc0e707825fbb70ed2e384794ddc7177e3` | On codex/screen-ai-adapter-readiness-proof: codex-b-adapter-export-fix-before-main-rebase |
| stash@{70} | `0d2d17d356a53f24a580fe79010c5133c119ecd5` | On codex/browser-ai-proof-gate-ui-delivery-proof: codex-d-ai25-proof-before-pr369-rebase |
| stash@{71} | `433b6c7903fe87d048b2a3f6d47e36ee5f9eba89` | On codex/browser-ai-proof-gate-ui-delivery-proof: preserve accidental tracking temporary live stash pop before ai25 recovery retry |
| stash@{72} | `cf62b462b129612421c287990c27f2cfdc376b0d` | On codex/tracking-temporary-live-mode-proof: preserve-unrelated-screen-ai-pop-from-codex-a-pr368-sync |
| stash@{73} | `1bbaab2bca7ba7a05916b8df0d7ecfdc10848a1c` | On codex/e-c-support-case-resolution-status-proof: e-c-support-case-resolution-status-proof-before-pr368-main |
| stash@{74} | `d884abf8d48dc6367b19b0139ccf41f56488d068` | On codex/browser-ai-proof-gate-ui-delivery-proof: preserve accidental tracking temporary live stash pop before ai25 recovery |
| stash@{75} | `c37d2ab59954a870f5ccf4e6b5cd4fe219d4609c` | On codex/browser-ai-proof-gate-ui-delivery-proof: codex-d-ai25-proof-before-pr368-rebase |
| stash@{76} | `fba6bef580678f3f5c276f2040d4a852382789c6` | On codex/e-b-v7-app-install-parent-action-delivery-readiness-proof: E-B parent-action delivery readiness proof resolved before latest main rebase |
| stash@{77} | `5bb49e3d7c7fdaf5e0c5f4056fef4ce16f9f7e12` | On codex/screen-ai-adapter-readiness-proof: adapter readiness proof wip before pr367 merge rebase |
| stash@{78} | `f6753ff7e581c8cc3db9e88059f5330b4a5b755c` | On codex/e-b-v7-app-install-parent-action-delivery-readiness-proof: E-B parent-action delivery readiness proof in progress before main rebase |
| stash@{79} | `78f72e011b444ddaa99036c77176a9fca6042ecd` | On codex/e-c-support-case-resolution-status-proof: e-c-support-case-resolution-status-proof-staged |
| stash@{80} | `ffd26f7fc586673f954f07d5467ffe25aacdc167` | On codex/screen-ai-adapter-readiness-proof: adapter readiness proof wip before pr365 rebase |
| stash@{81} | `d4c195dde7ac7b2f7501fcbfdcb368f6bd230444` | On codex/screen-winrt-ocr-worker-proof: screen-winrt-ocr-worker-wip-before-main-8111abc7 |
| stash@{82} | `c3a27d7f87e7c65869beb9f1e204fcfce09e3f19` | On codex/screen-winrt-ocr-worker-proof: screen-winrt-ocr-worker-wip-before-wp26-pr338-rebase |
| stash@{83} | `4bafc7b4d10be50f66814fb21741376ec66fd977` | On codex/screen-child-disclosure-ux-proof: wp26-staged-before-main-0f6288d14 |
| stash@{84} | `6c2b4a74a5b978cd7db0e8e49de9a2cca239fe9f` | On codex/screen-child-disclosure-ux-proof: wp26-staged-before-main-68d0ae43 |
| stash@{85} | `b816884a97235540e2b0150ed149a413f3c0eda0` | On codex/screen-child-disclosure-ux-proof: wp26-staged-before-pr338-531 |
| stash@{86} | `5db68714da903fd432d6ceccaad0bca9a5853ada` | On codex/screen-child-disclosure-ux-proof: wp26-staged-before-pr338-528 |
| stash@{87} | `c63dd2953cefab1933b0d38ef1164e3b4acf3d1f` | On codex/screen-child-disclosure-ux-proof: wp26-staged-before-pr338-526 |
| stash@{88} | `f9501ef397937d3dce5cd2ffc7028b8576b88102` | On codex/screen-child-disclosure-ux-proof: wp26-child-disclosure-before-pr338-doc-fix-524 |
| stash@{89} | `170f466003d228ecc17f25b3a8f4545764d84470` | On codex/e-c-public-privacy-legal-support-docs-proof: E-C proof refresh before main sync |
| stash@{90} | `9c7ac69114b26d4f807d0a3d618ad29d03667f9d` | On codex/screen-child-disclosure-ux-proof: wp26-child-disclosure-wip-before-pr338-fix |
| stash@{91} | `cef66a418820ec369c6298159b2d0a9bce5688e3` | On codex/app-game-source-freshness-source-panel: wp63-source-panel-rebased-wip |
| stash@{92} | `5da625b8837d0169b27c5854d75ff189c785c921` | On codex/app-game-source-freshness-source-panel: wp63-source-panel-wip |
| stash@{93} | `352c090fb2bc26fdbf9f03e7d73a8b42329b8e53` | On codex/browser-vimeo-generic-live-metadata-proof: codex-d-ai22-sync-da8659 |
| stash@{94} | `7bf723c8a007a5de8948455791c6cf1b5cd694ed` | On codex/browser-vimeo-generic-live-metadata-proof: codex-d-ai22-sync-pr312 |
| stash@{95} | `852c9412564d7e9c640879d8331568c721702b34` | On codex/app-game-notification-scheduler-bridge: preserve accidental screen scheduler stash pop in codex-c |
| stash@{96} | `c49cb7f5d1fa76617fe2cbd9ff740ede64d47965` | On codex/screen-local-ai-resource-scheduler-proof: preserve unrelated browser vimeo pop from codex-b rebase recovery |
| stash@{97} | `9f8e7166a323dbd8ded3857c35965b506675ab4a` | On codex/screen-settings-writable-intent-proof: preserve final screen settings screenshots before PR307 rebase |
| stash@{98} | `323ded5d5a2a17d33eb6fb95c2357f055fd301e9` | On codex/screen-settings-writable-intent-proof: preserve screen settings writable proof before main refresh |
| stash@{99} | `ebc9d4d63f1abe45d4843d344408a2566d5b2fb0` | On codex/browser-vimeo-generic-live-metadata-proof: preserve-unrelated-screen-settings-pop |
| stash@{100} | `3bfd8201f274a52c204372a1bb4459d80e596b00` | On codex/screen-plan-proof-reconciliation: writable screen settings proof continuation wip |
| stash@{101} | `22c14214496051809a840dd097663306e168ce46` | On codex/tracking-child-check-in-ui-proof: tracking child check-in WIP before main sync after PR305 |
| stash@{102} | `5849e596047df53e38bd05466ad7d846ef19b647` | On codex/tracking-hosted-ui-accessibility-proof-v2: tracking child check-in WIP before continuation branch |
| stash@{103} | `602e2b3b34e417143407b607edb5a377e4b65405` | On codex/screen-ai-service-retention-sweeper-proof: screen-ai-retention-sweeper-wip-before-pr282-rebase-20260604T085622Z |
| stash@{104} | `de38d31c7f491a18195dcf38f0371ae23e205044` | On codex/screen-ai-block-action-dispatch-proof: wip-screen-ai-retention-custody-proof-after-pr281-hold |
| stash@{105} | `3e74ed70ee468bf81623d140c2edfac3828c048f` | On codex/screen-ai-block-action-dispatch-proof: screen-ai-full-live-operator-proof-wip-20260604T1050Z |
| stash@{106} | `62f4eb6e1c147ebf5fb0728f636f2d8a2cf7471a` | On codex/screen-ai-block-action-dispatch-proof: preserve-screen-ai-proof-wip-before-pr274-refresh-20260604T0830Z |
| stash@{107} | `d73713d5b7e5f3bc54e6a8ac4eec3e99f5bfde78` | On codex/app-plan-evidence-control-continuation: codex-c app-game stack before main refresh no-bom |
| stash@{108} | `767daf1ff36f3159c964e522a442edf3b86fc3d0` | On codex/app-plan-evidence-control-continuation: codex-c app-game stack before main refresh |
| stash@{109} | `40b0b90ab6850210a495d4ec63f8a59fea44a008` | On codex/browser-plan-full-scope-continuation: codex-d browser AI UX read-model WIP blocked by C protocol overlap |
| stash@{110} | `69be26d38b0dd3b43f572b0ee97527c1e6cbf949` | On codex/screen-ai-pipeline-continuation: wip-screen-ai-before-8e1de42-sync-20260603T2155Z |
| stash@{111} | `82d8289a4c3d2a8e6b00c510060c55bb9ef1c2dd` | On main: primary-plan-docs-before-pr261-sync |
| stash@{112} | `a69d52ee1f4ca904768a6919cd62db0584e061f1` | On codex/screen-ai-pipeline-proof: preserve-screen-ai-proof-before-pr256-rebase-20260603T1456Z |
| stash@{113} | `bb741cc3661797cf632f2afd8397e366658840ff` | On codex/browser-plan-implementation: codex-d browser-plan wip before final rebase |
| stash@{114} | `b6b48d5e09abe2db5dea218b3501bd60bcf665a9` | On codex/browser-plan-implementation: codex-d browser-plan WIP before origin-main-be763ed |
| stash@{115} | `ffb38de8eb5ebd892aa8b0f7676b3bcd5aad82fb` | On codex/browser-plan-implementation: codex-d browser-plan WIP before origin-main-95801c09 |
| stash@{116} | `a0bf41002793e4ef987f44eb7ff3cc388e12cf2b` | On codex/browser-plan-implementation: codex-d browser-plan work before origin-main rebase 2026-06-03T08-50Z |
| stash@{117} | `7f45d578949cd5ffa35a5adc54c98d03a2bf6ba2` | On codex/screen-ai-pipeline-proof: pre-main-latest-screen-ai-proof-20260603T0848Z |
| stash@{118} | `021c4ebd8fb2af6b74db83ded9179fce4762ab6f` | On codex/screen-ai-pipeline-proof: pre-main-304b4c7-screen-ai-proof-20260603T0840Z |
| stash@{119} | `d50f57798edf3f7beb738d119edfca4f331f6edf` | On codex/screen-ai-pipeline-proof: pre-main-2bb4a2b-screen-ai-proof-20260603T0838Z |
| stash@{120} | `639e9932d209e5b4060d3c780645eec310cc68d5` | On codex/browser-plan-implementation: codex-d browser-plan WIP before PR239 PR240 rebase |
| stash@{121} | `917889d8bfb9323d2385f1a060a0e2c198883ba0` | On codex/app-game-windows-process-runtime: wp08-process-runtime-wip-clean |
| stash@{122} | `11fd05a8fb934b8fe30270d65f811723608143db` | On codex/app-game-windows-process-runtime: wp08-process-runtime-wip |
| stash@{123} | `5806612e82dddf59db37850f110421834e1060a8` | On codex/v0-9-lan-source-matrix-plan-completion: preserve-cross-lane-lan-proof-checklist-contracts-20260603 |
| stash@{124} | `1fbcd6ce0822d2da0af75e3a253da3b6f112f95c` | On codex/v0-9-lan-source-matrix-plan-completion: preserve-lan-source-matrix-before-main-advanced-20260602T1815Z |
| stash@{125} | `02c867c922cf892027d911234af367ecd873668c` | On codex/v0-9-lan-source-matrix-plan-completion: preserve-lan-source-matrix-plan-completion-before-pr233-20260602T1725Z |
| stash@{126} | `538e8c7c23e6cf546d4f6966fd39c639d803ef90` | On codex/v0-9-lan-signed-discovery-relay-spine: preserve-lan-source-matrix-follow-up-20260602T160446Z |
| stash@{127} | `9df5655cc2da21185a9a04aaf8caadaa24bf3fa5` | On codex/v0-9-lan-signed-discovery-relay-spine: primary-handoff-enforcement-integrity-from-codex-b-20260602T114802Z |
| stash@{128} | `e226aa6b296d5426e610583e1210895e3bb5ad54` | On codex/v0-8-enforcement-integrity-runtime-audit: handoff-lan-spine-from-codex-a-20260602T1148Z |
| stash@{129} | `7516cc5ca6804a8affa6b159d0410e567e809b95` | On codex/local-model-artifact-cache-contracts: codex-b gpu proof before main rebase |
