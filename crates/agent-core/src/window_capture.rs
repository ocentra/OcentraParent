use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForegroundWindowObservation {
    pub status: ActivityCaptureCapabilityStatus,
    pub pid: Option<u32>,
    pub app_name: Option<String>,
    pub process_path: Option<String>,
    pub title: Option<String>,
    pub window_id: Option<String>,
}

impl ForegroundWindowObservation {
    pub fn active(
        pid: u32,
        app_name: String,
        process_path: String,
        title: String,
        window_id: String,
    ) -> Self {
        Self {
            status: ActivityCaptureCapabilityStatus::Available,
            pid: Some(pid),
            app_name: Some(app_name),
            process_path: Some(process_path),
            title: Some(title),
            window_id: Some(window_id),
        }
    }

    pub fn degraded(status: ActivityCaptureCapabilityStatus) -> Self {
        Self {
            status,
            pid: None,
            app_name: None,
            process_path: None,
            title: None,
            window_id: None,
        }
    }
}

pub fn collect_foreground_window_observation() -> ForegroundWindowObservation {
    platform_foreground_window_observation()
}

#[cfg(windows)]
fn platform_foreground_window_observation() -> ForegroundWindowObservation {
    match active_win_pos_rs::get_active_window() {
        Ok(window) => {
            if window.window_id.is_empty() && window.process_id == 0 {
                return ForegroundWindowObservation::degraded(
                    ActivityCaptureCapabilityStatus::NoActiveWindow,
                );
            }
            ForegroundWindowObservation::active(
                window.process_id as u32,
                window.app_name,
                window.process_path.to_string_lossy().into_owned(),
                window.title,
                window.window_id,
            )
        }
        Err(()) => {
            ForegroundWindowObservation::degraded(ActivityCaptureCapabilityStatus::AdapterError)
        }
    }
}

#[cfg(not(windows))]
fn platform_foreground_window_observation() -> ForegroundWindowObservation {
    ForegroundWindowObservation::degraded(ActivityCaptureCapabilityStatus::Unavailable)
}
