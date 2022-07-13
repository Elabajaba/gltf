use crate::{extensions, Extras};
use gltf_derive::Validate;
use nanoserde::{DeJson, SerJson};

/// Metadata about the glTF asset.
#[derive(Clone, Debug, DeJson, SerJson, Validate)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator.
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,

    /// Extension specific data.
    #[nserde(default)]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::asset::Asset>,

    /// Optional application specific data.
    #[nserde(default)]
    #[cfg_attr(feature = "extras", nserde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), nserde(skip_serializing))]
    pub extras: Extras,

    /// Tool that generated this glTF model.
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,

    /// The minimum glTF version that this asset targets.
    #[nserde(rename = "minVersion")]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,

    /// The glTF version of this asset.
    pub version: String,
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            copyright: None,
            extensions: Default::default(),
            extras: Default::default(),
            generator: None,
            min_version: None,
            version: "2.0".to_string(),
        }
    }
}
