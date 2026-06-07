# Android background permission proof

- Checked at: 2026-06-07T01:52:44.667Z
- Commit: 698601c6d88f6b0968f628182c9ac124cc02bdb0
- Device: sdk_gphone64_x86_64 / Android 15 API 35
- Package: ca.ocentra.parent.agent
- Resolved activity: priority=0 preferredOrder=0 match=0x108000 specificIndex=-1 isDefault=false
ca.ocentra.parent.agent/.MainActivity

## Requested permissions

- android.permission.POST_NOTIFICATIONS: granted=false
- android.permission.ACCESS_FINE_LOCATION: granted=true
- android.permission.FOREGROUND_SERVICE: granted=true
- android.permission.ACCESS_COARSE_LOCATION: granted=true
- android.permission.FOREGROUND_SERVICE_DATA_SYNC: granted=true
- android.permission.FOREGROUND_SERVICE_MEDIA_PROJECTION: granted=true
- android.permission.ACCESS_BACKGROUND_LOCATION: granted=true


## Tracking claim boundary

- Foreground location permission requested: true
- Foreground location permission granted: true
- Background location permission requested: true
- Background location permission granted: true
- Foreground service observed: true
- Product location/geofence claim ready: false
