# Android foreground permission proof

- Checked at: 2026-06-07T00:12:44.112Z
- Commit: 0f4340c8e744ca2425bd0d3c784ddd3a48f54d2e
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


## Tracking claim boundary

- Foreground location permission requested: true
- Foreground location permission granted: true
- Background location permission requested: false
- Foreground service observed: true
- Product location/geofence claim ready: false
