use gltf_derive::Validate;
use nanoserde::{DeJson, SerJson};
use std::fmt;

#[cfg(feature = "extras")]
pub use serde_json::value::RawValue;

/// Data type of the `extras` attribute on all glTF objects.
#[cfg(feature = "extras")]
pub type Extras = Option<::std::boxed::Box<RawValue>>;

/// Data type of the `extras` attribute on all glTF objects.
#[cfg(not(feature = "extras"))]
pub type Extras = Void;

/// Type representing no user-defined data.
#[derive(Clone, Default, DeJson, SerJson, Validate)]
pub struct Void {
    #[nserde(default)]
    #[nserde(skip_serializing)]
    _allow_unknown_fields: (),
}

impl fmt::Debug for Void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}

impl fmt::Display for Void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}
