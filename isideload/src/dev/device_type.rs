use plist::{Dictionary, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeveloperDeviceType {
    Any,
    Ios,
    Tvos,
    Watchos,
}

impl DeveloperDeviceType {
    pub fn url_segment(&self) -> &'static str {
        match self {
            DeveloperDeviceType::Any => "",
            DeveloperDeviceType::Ios => "ios/",
            // tvOS is served by the iOS service path and selects its platform with
            // the DTDK_Platform/subPlatform request fields instead.
            DeveloperDeviceType::Tvos => "ios/",
            DeveloperDeviceType::Watchos => "watchos/",
        }
    }

    /// Map a lockdown `ProductType` (e.g. `AppleTV6,2`) to a developer device type.
    pub fn from_product_type(product_type: &str) -> Self {
        if product_type.starts_with("AppleTV") {
            Self::Tvos
        } else if product_type.starts_with("Watch") {
            Self::Watchos
        } else {
            Self::Ios
        }
    }

    /// Add the request fields that select this platform.
    pub fn apply_platform_fields(&self, body: &mut Dictionary) {
        let (platform, sub_platform) = match self {
            DeveloperDeviceType::Tvos => ("tvos", Some("tvOS")),
            DeveloperDeviceType::Watchos => ("watchos", None),
            _ => return,
        };

        body.insert("DTDK_Platform".into(), Value::String(platform.into()));

        if let Some(sub_platform) = sub_platform {
            body.insert("subPlatform".into(), Value::String(sub_platform.into()));
        }
    }
}

pub fn dev_url(endpoint: &str, device_type: impl Into<Option<DeveloperDeviceType>>) -> String {
    format!(
        "https://developerservices2.apple.com/services/QH65B2/{}{}.action?clientId=XABBG36SBA",
        device_type
            .into()
            .unwrap_or(DeveloperDeviceType::Ios)
            .url_segment(),
        endpoint,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_types_map_to_device_types() {
        assert_eq!(
            DeveloperDeviceType::from_product_type("AppleTV14,1"),
            DeveloperDeviceType::Tvos
        );
        assert_eq!(
            DeveloperDeviceType::from_product_type("iPhone16,1"),
            DeveloperDeviceType::Ios
        );
    }

    #[test]
    fn tvos_requests_add_platform_fields() {
        let mut body = Dictionary::new();
        DeveloperDeviceType::Tvos.apply_platform_fields(&mut body);

        assert_eq!(body.get("DTDK_Platform").unwrap().as_string(), Some("tvos"));
        assert_eq!(body.get("subPlatform").unwrap().as_string(), Some("tvOS"));
    }
}
