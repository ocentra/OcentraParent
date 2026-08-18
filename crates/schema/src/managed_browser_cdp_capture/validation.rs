use super::*;

impl ManagedBrowserCdpCaptureRequest {
    pub fn validate(&self) -> Result<(), ManagedBrowserCdpCaptureRequestError> {
        if self.schema_version != MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION {
            return Err(ManagedBrowserCdpCaptureRequestError::UnsupportedSchemaVersion);
        }
        if self.target_id.trim().is_empty() {
            return Err(ManagedBrowserCdpCaptureRequestError::EmptyTargetId);
        }
        if self.target_id.len() > 256 {
            return Err(ManagedBrowserCdpCaptureRequestError::TargetIdTooLong);
        }

        match self.mode {
            ManagedBrowserCdpCaptureMode::Page => validate_page(self),
            ManagedBrowserCdpCaptureMode::Viewport => validate_viewport(self),
            ManagedBrowserCdpCaptureMode::Crop => validate_crop_mode(self),
        }
    }
}

fn validate_page(
    request: &ManagedBrowserCdpCaptureRequest,
) -> Result<(), ManagedBrowserCdpCaptureRequestError> {
    if request.crop.is_some()
        || request.viewport_width.is_some()
        || request.viewport_height.is_some()
    {
        return Err(ManagedBrowserCdpCaptureRequestError::CropNotAllowed);
    }
    Ok(())
}

fn validate_viewport(
    request: &ManagedBrowserCdpCaptureRequest,
) -> Result<(), ManagedBrowserCdpCaptureRequestError> {
    validate_dimensions(request)?;
    if request.crop.is_some() {
        return Err(ManagedBrowserCdpCaptureRequestError::CropNotAllowed);
    }
    Ok(())
}

fn validate_crop_mode(
    request: &ManagedBrowserCdpCaptureRequest,
) -> Result<(), ManagedBrowserCdpCaptureRequestError> {
    validate_dimensions(request)?;
    let Some(crop) = request.crop.as_ref() else {
        return Err(ManagedBrowserCdpCaptureRequestError::CropRequired);
    };
    let (Some(viewport_width), Some(viewport_height)) =
        (request.viewport_width, request.viewport_height)
    else {
        return Err(ManagedBrowserCdpCaptureRequestError::DimensionsRequired);
    };
    validate_crop(crop, viewport_width, viewport_height)
}

fn validate_dimensions(
    request: &ManagedBrowserCdpCaptureRequest,
) -> Result<(), ManagedBrowserCdpCaptureRequestError> {
    let (Some(width), Some(height)) = (request.viewport_width, request.viewport_height) else {
        return Err(ManagedBrowserCdpCaptureRequestError::DimensionsRequired);
    };
    if width == 0
        || height == 0
        || width > MANAGED_BROWSER_CDP_MAX_DIMENSION
        || height > MANAGED_BROWSER_CDP_MAX_DIMENSION
        || u64::from(width) * u64::from(height) > MANAGED_BROWSER_CDP_MAX_PIXELS
    {
        return Err(ManagedBrowserCdpCaptureRequestError::DimensionsOutOfBounds);
    }
    Ok(())
}

fn validate_crop(
    crop: &ManagedBrowserCdpCrop,
    viewport_width: u32,
    viewport_height: u32,
) -> Result<(), ManagedBrowserCdpCaptureRequestError> {
    if crop.width == 0
        || crop.height == 0
        || crop.width > MANAGED_BROWSER_CDP_MAX_DIMENSION
        || crop.height > MANAGED_BROWSER_CDP_MAX_DIMENSION
        || u64::from(crop.width) * u64::from(crop.height) > MANAGED_BROWSER_CDP_MAX_PIXELS
        || crop
            .x
            .checked_add(crop.width)
            .map_or(true, |right| right > viewport_width)
        || crop
            .y
            .checked_add(crop.height)
            .map_or(true, |bottom| bottom > viewport_height)
    {
        return Err(ManagedBrowserCdpCaptureRequestError::CropOutOfBounds);
    }
    Ok(())
}
