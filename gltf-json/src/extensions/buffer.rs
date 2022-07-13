use gltf_derive::Validate;
use nanoserde::{DeJson, SerJson};

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, Default, DeJson, SerJson, Validate)]
pub struct Buffer {}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug, Default, DeJson, SerJson, Validate)]
pub struct View {}
