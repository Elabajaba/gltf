use std::convert::TryInto;

use crate::validation::Checked;
use crate::{extensions, Extras, Index};
use gltf_derive::Validate;
use nanoserde::{DeJson, DeJsonErr, SerJson};

/// Corresponds to `GL_ARRAY_BUFFER`.
pub const ARRAY_BUFFER: u32 = 34_962;

/// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
pub const ELEMENT_ARRAY_BUFFER: u32 = 34_963;

/// The minimum byte stride.
pub const MIN_BYTE_STRIDE: u32 = 4;

/// The maximum byte stride.
pub const MAX_BYTE_STRIDE: u32 = 252;

/// All valid GPU buffer targets.
pub const VALID_TARGETS: &[u32] = &[ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER];

/// Specifies the target a GPU buffer should be bound to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Target {
    /// Corresponds to `GL_ARRAY_BUFFER`.
    ArrayBuffer = 1,

    /// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
    ElementArrayBuffer,
}

impl SerJson for Target {
    fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
        match *self {
            Target::ArrayBuffer => ARRAY_BUFFER.ser_json(d, s),
            Target::ElementArrayBuffer => ELEMENT_ARRAY_BUFFER.ser_json(d, s),
        }
    }
}

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, DeJson, SerJson, Validate)]
pub struct Buffer {
    /// The length of the buffer in bytes.
    #[nserde(default)]
    #[nserde(rename = "byteLength")]
    pub byte_length: u32,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", nserde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// The uri of the buffer.  Relative paths are relative to the .gltf file.
    /// Instead of referencing an external file, the uri can also be a data-uri.
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// Extension specific data.
    #[nserde(default)]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::buffer::Buffer>,

    /// Optional application specific data.
    #[nserde(default)]
    #[cfg_attr(feature = "extras", nserde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), nserde(skip_serializing))]
    pub extras: Extras,
}

/// A view into a buffer generally representing a subset of the buffer.
///
/// <https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#reference-bufferview>
///
#[derive(Clone, Debug, DeJson, SerJson, Validate)]
pub struct View {
    /// The parent `Buffer`.
    pub buffer: Index<Buffer>,

    /// The length of the `BufferView` in bytes.
    #[nserde(rename = "byteLength")]
    pub byte_length: u32,

    /// Offset into the parent buffer in bytes.
    #[nserde(default)]
    #[nserde(skip_serializing_if = "Option::is_none")]
    #[nserde(rename = "byteOffset")]
    pub byte_offset: Option<u32>,

    /// The stride in bytes between vertex attributes or other interleavable data.
    ///
    /// When zero, data is assumed to be tightly packed.
    #[nserde(rename = "byteStride")]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub byte_stride: Option<u32>,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", nserde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// Optional target the buffer should be bound to.
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Checked<Target>>,

    /// Extension specific data.
    #[nserde(default)]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::buffer::View>,

    /// Optional application specific data.
    #[nserde(default)]
    #[cfg_attr(feature = "extras", nserde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), nserde(skip_serializing))]
    pub extras: Extras,
}

impl DeJson for Checked<Target> {
    fn deserialize_json(input: &str) -> Result<Self, DeJsonErr> {
        use Checked::*;

        // TODO: Is this right? What should I do for an Invalid thing?

        let val: u32 = input.try_into()?;
        Ok(match val {
            ARRAY_BUFFER => Valid(Target::ArrayBuffer),
            ELEMENT_ARRAY_BUFFER => Valid(Target::ElementArrayBuffer),
            _ => Invalid,
        })
    }
    // fn de_json(state: &mut DeJsonState, input: &mut Chars<'_>) -> Result<Self, DeJsonErr> {
    //     use Checked::*;
    //     let val: u32 = state.as_f64().try_into()?;
    //     let temp = match val {
    //         ARRAY_BUFFER => Valid(Target::ArrayBuffer),
    //         ELEMENT_ARRAY_BUFFER => Valid(Target::ElementArrayBuffer),
    //         _ => Invalid,
    //     };

    //     Ok(temp)
    // }
}

// impl<'de> de::Deserialize<'de> for Checked<Target> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: de::Deserializer<'de>,
//     {
//         struct Visitor;
//         impl<'de> de::Visitor<'de> for Visitor {
//             type Value = Checked<Target>;

//             fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
//                 write!(f, "any of: {:?}", VALID_TARGETS)
//             }

//             fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
//             where
//                 E: de::Error,
//             {
//                 use self::Target::*;
//                 use crate::validation::Checked::*;
//                 Ok(match value as u32 {
//                     ARRAY_BUFFER => Valid(ArrayBuffer),
//                     ELEMENT_ARRAY_BUFFER => Valid(ElementArrayBuffer),
//                     _ => Invalid,
//                 })
//             }
//         }
//         deserializer.deserialize_u64(Visitor)
//     }
// }
