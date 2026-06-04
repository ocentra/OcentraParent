# Android background permission proof

- Checked at: 2026-06-04T14:11:20.301Z
- Commit: 7f59604f57f114fa4b31547db3a6f7716647d5a6
- Device: sdk_gphone64_x86_64 / Android 15 API 35
- Package: ca.ocentra.parent.agent
- Resolved activity: priority=0 preferredOrder=0 match=0x108000 specificIndex=-1 isDefault=false
ca.ocentra.parent.agent/.MainActivity

## Requested permissions

- android.permission.POST_NOTIFICATIONS: granted=false
- android.permission.FOREGROUND_SERVICE: granted=true
- android.permission.FOREGROUND_SERVICE_DATA_SYNC: granted=true
- android.permission.FOREGROUND_SERVICE_MEDIA_PROJECTION: granted=true


## Tracking claim boundary

- Foreground location permission requested: false
- Background location permission requested: false
- Foreground service observed: true
- Product location/geofence claim ready: false
