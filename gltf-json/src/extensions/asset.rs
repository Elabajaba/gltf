use gltf_derive::Validate;
use nanoserde::{DeJson, SerJson};

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Default, DeJson, SerJson, Validate)]
pub struct Asset {}
