use super::super::http::text::{extract_xml_text, parse_udn};
use super::super::http::{parse_device_type, parse_http_status_code, split_http_headers};
use super::super::{
    SsdpDeviceDescription, SsdpDiscoveryError, SSDP_MAX_DESCRIPTION_BYTES,
    SSDP_MAX_DESCRIPTION_TEXT_BYTES,
};

pub(super) fn parse_device_description_response(
    response: &[u8],
    description_url: &str,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    let (status_line, _headers, body) = split_http_headers(response)?;
    if parse_http_status_code(status_line)? != 200 {
        return Err(SsdpDiscoveryError::InvalidDescription);
    }
    let xml = std::str::from_utf8(body).map_err(|_error| SsdpDiscoveryError::InvalidDescription)?;
    parse_device_description_xml(xml, description_url)
}

pub(super) fn parse_device_description_xml(
    xml: &str,
    description_url: &str,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    if xml.len() > SSDP_MAX_DESCRIPTION_BYTES {
        return Err(SsdpDiscoveryError::ResponseTooLarge);
    }
    if xml.contains("<!DOCTYPE") || xml.contains("<!ENTITY") {
        return Err(SsdpDiscoveryError::InvalidDescription);
    }
    let friendly_name =
        bounded_text(xml, "friendlyName")?.ok_or(SsdpDiscoveryError::InvalidDescription)?;
    let device_type = bounded_text(xml, "deviceType")?
        .and_then(|value| parse_device_type(&value).or(Some(value)));
    let manufacturer = bounded_text(xml, "manufacturer")?;
    let model_name = bounded_text(xml, "modelName")?;
    let udn = bounded_text(xml, "UDN")?.and_then(|value| parse_udn(&value));
    Ok(SsdpDeviceDescription {
        friendly_name,
        manufacturer,
        model_name,
        device_type,
        udn,
        description_url: description_url.to_string(),
    })
}

fn bounded_text(xml: &str, tag: &str) -> Result<Option<String>, SsdpDiscoveryError> {
    match extract_xml_text(xml, tag) {
        Some(value) if value.len() > SSDP_MAX_DESCRIPTION_TEXT_BYTES => {
            Err(SsdpDiscoveryError::InvalidDescription)
        }
        value => Ok(value),
    }
}
