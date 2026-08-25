//! Service configuration observation accessors.

use super::super::{SecurityDescriptorObservation, ServiceObservation};

impl ServiceObservation {
    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub fn service_type(&self) -> u32 {
        self.service_type
    }

    pub fn binary_path(&self) -> Option<&str> {
        self.binary_path.as_deref()
    }

    pub fn start_name(&self) -> &str {
        self.start_name.as_deref().unwrap_or_default()
    }

    pub fn start_type(&self) -> u32 {
        self.start_type
    }

    pub fn error_control(&self) -> u32 {
        self.error_control
    }

    pub fn load_order_group(&self) -> Option<&str> {
        self.load_order_group.as_deref()
    }

    pub fn tag_id(&self) -> u32 {
        self.tag_id
    }

    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    pub fn start_name_value(&self) -> Option<&str> {
        self.start_name.as_deref()
    }

    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    pub fn service_sid_type(&self) -> u32 {
        self.service_sid_type
    }

    pub fn required_privileges(&self) -> &[String] {
        &self.required_privileges
    }

    pub fn delayed_auto_start(&self) -> bool {
        self.delayed_auto_start
    }

    pub fn launch_protected(&self) -> u32 {
        self.launch_protected
    }

    pub fn security(&self) -> &SecurityDescriptorObservation {
        &self.security
    }
}
