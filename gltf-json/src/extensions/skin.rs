use gltf_derive::Validate;
use nanoserde::{DeJson, SerJson};

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, Default, DeJson, SerJson, Validate)]
pub struct Skin {}
