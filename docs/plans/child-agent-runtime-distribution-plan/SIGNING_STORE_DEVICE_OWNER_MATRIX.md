# Signing, Store, and Device-Owner Matrix

| Artifact              | Signing state                    | Store / distribution state               | Device-owner / custody state                     | Claim allowed                   |
| --------------------- | -------------------------------- | ---------------------------------------- | ------------------------------------------------ | ------------------------------- |
| Child Windows package | code signing / installer signing | package / preview / manual-required      | service-manager custody                          | child Windows distribution only |
| Child macOS package   | signing / notarization           | package / preview / manual-required      | launchd custody                                  | child macOS distribution only   |
| Child Linux package   | package signing / repo signing   | package / preview / manual-required      | service-manager custody                          | child Linux distribution only   |
| Child Android package | app signing / bundle signing     | Play / sideload / manual-required        | device-owner / managed-profile / manual-required | child Android distribution only |
| Child iOS package     | app signing / provisioning       | TestFlight / App Store / manual-required | supervised / managed / manual-required           | child iOS distribution only     |

## Rules

- Signing proof must name the artifact.
- Store proof must name the channel.
- Device-owner or custody proof must be explicit.
- Parent client claims are out of scope.
