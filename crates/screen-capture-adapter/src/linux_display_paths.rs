use std::{
    ffi::OsStr,
    path::{Component, Path, PathBuf},
};

pub(super) fn x11_socket_path(display: &OsStr) -> Option<PathBuf> {
    let value = display.to_str()?.trim();
    let value = value.strip_prefix("unix/").unwrap_or(value);
    let (host, display) = value.rsplit_once(':')?;
    if !host.is_empty() && host != "unix" {
        return None;
    }
    let display_number = display.split('.').next()?;
    if display_number.is_empty()
        || !display_number
            .chars()
            .all(|character| character.is_ascii_digit())
    {
        return None;
    }
    Some(PathBuf::from(format!("/tmp/.X11-unix/X{display_number}")))
}

pub(super) fn wayland_socket_path(
    display: Option<&OsStr>,
    runtime_dir: Option<&OsStr>,
) -> Option<PathBuf> {
    let display = display?.to_str()?.trim();
    if display.is_empty() {
        return None;
    }
    let display = Path::new(display);
    if display.is_absolute() {
        return None;
    }
    let mut components = display.components();
    if !matches!(
        (components.next(), components.next()),
        (Some(Component::Normal(_)), None)
    ) {
        return None;
    }
    let runtime_dir = Path::new(runtime_dir?.to_str()?);
    runtime_dir.is_absolute().then(|| runtime_dir.join(display))
}
