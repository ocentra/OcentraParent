# Android background permission proof

- Checked at: 2026-06-08T16:14:44.445Z
- Commit: ca748d0c1f68def1d56f4b328bd1872951a81698
- Device: Android SDK built for x86_64 / Android 13 API 33
- Package: ca.ocentra.parent.agent
- Resolved activity: priority=0 preferredOrder=0 match=0x108000 specificIndex=-1 isDefault=false
ca.ocentra.parent.agent/.MainActivity

## Requested permissions

- android.permission.FOREGROUND_SERVICE: granted=true
- android.permission.FOREGROUND_SERVICE_DATA_SYNC: granted=false
- android.permission.FOREGROUND_SERVICE_MEDIA_PROJECTION: granted=false
- android.permission.POST_NOTIFICATIONS: granted=false
- android.permission.ACCESS_COARSE_LOCATION: granted=true
- android.permission.ACCESS_FINE_LOCATION: granted=true
- android.permission.ACCESS_BACKGROUND_LOCATION: granted=true


## Tracking claim boundary

- Foreground location permission requested: true
- Foreground location permission granted: true
- Background location permission requested: true
- Background location permission granted: true
- Foreground service observed: true
- Product location/geofence claim ready: false
