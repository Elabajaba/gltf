use gltf_derive::Validate;
use nanoserde::{DeJson, SerJson};

/// Image data used to create a texture.
#[derive(Clone, Debug, Default, DeJson, SerJson, Validate)]
pub struct Image {}
