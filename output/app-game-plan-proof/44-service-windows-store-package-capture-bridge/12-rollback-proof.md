# Rollback Proof

Rollback is source removal only:

- Remove packaged-app event collection from `activity_capture_events`.
- Remove service imports and error conversion for the packaged-app source.
- Keep existing process, foreground, network, and shortcut inventory capture
  paths unchanged.

No policy state, child-device action, OS adapter setting, or blocking rule is
created by WP44.
