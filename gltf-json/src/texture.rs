use crate::validation::Checked;
use crate::{extensions, image, Extras, Index};
use gltf_derive::Validate;
// use serde::{de, ser};
use nanoserde::{DeJson, SerJson};
use std::convert::TryInto;

/// Corresponds to `GL_NEAREST`.
pub const NEAREST: u32 = 9728;

/// Corresponds to `GL_LINEAR`.
pub const LINEAR: u32 = 9729;

/// Corresponds to `GL_NEAREST_MIPMAP_NEAREST`.
pub const NEAREST_MIPMAP_NEAREST: u32 = 9984;

/// Corresponds to `GL_LINEAR_MIPMAP_NEAREST`.
pub const LINEAR_MIPMAP_NEAREST: u32 = 9985;

/// Corresponds to `GL_NEAREST_MIPMAP_LINEAR`.
pub const NEAREST_MIPMAP_LINEAR: u32 = 9986;

/// Corresponds to `GL_LINEAR_MIPMAP_LINEAR`.
pub const LINEAR_MIPMAP_LINEAR: u32 = 9987;

/// Corresponds to `GL_CLAMP_TO_EDGE`.
pub const CLAMP_TO_EDGE: u32 = 33_071;

/// Corresponds to `GL_MIRRORED_REPEAT`.
pub const MIRRORED_REPEAT: u32 = 33_648;

/// Corresponds to `GL_REPEAT`.
pub const REPEAT: u32 = 10_497;

/// All valid magnification filters.
pub const VALID_MAG_FILTERS: &[u32] = &[NEAREST, LINEAR];

/// All valid minification filters.
pub const VALID_MIN_FILTERS: &[u32] = &[
    NEAREST,
    LINEAR,
    NEAREST_MIPMAP_NEAREST,
    LINEAR_MIPMAP_NEAREST,
    NEAREST_MIPMAP_LINEAR,
    LINEAR_MIPMAP_LINEAR,
];

/// All valid wrapping modes.
pub const VALID_WRAPPING_MODES: &[u32] = &[CLAMP_TO_EDGE, MIRRORED_REPEAT, REPEAT];

/// Magnification filter.
#[derive(Clone, Copy, Debug, DeJson, Eq, PartialEq)]
pub enum MagFilter {
    /// Corresponds to `GL_NEAREST`.
    Nearest,

    /// Corresponds to `GL_LINEAR`.
    Linear,
}

impl MagFilter {
    /// OpenGL enum
    pub fn as_gl_enum(&self) -> u32 {
        match *self {
            MagFilter::Nearest => NEAREST,
            MagFilter::Linear => LINEAR,
        }
    }
}

/// Minification filter.
#[derive(Clone, Copy, Debug, DeJson, Eq, PartialEq)]
pub enum MinFilter {
    /// Corresponds to `GL_NEAREST`.
    Nearest,

    /// Corresponds to `GL_LINEAR`.
    Linear,

    /// Corresponds to `GL_NEAREST_MIPMAP_NEAREST`.
    NearestMipmapNearest,

    /// Corresponds to `GL_LINEAR_MIPMAP_NEAREST`.
    LinearMipmapNearest,

    /// Corresponds to `GL_NEAREST_MIPMAP_LINEAR`.
    NearestMipmapLinear,

    /// Corresponds to `GL_LINEAR_MIPMAP_LINEAR`.
    LinearMipmapLinear,
}

impl MinFilter {
    /// Returns the corresponding OpenGL enum value.
    pub fn as_gl_enum(&self) -> u32 {
        match *self {
            MinFilter::Nearest => NEAREST,
            MinFilter::Linear => LINEAR,
            MinFilter::NearestMipmapNearest => NEAREST_MIPMAP_NEAREST,
            MinFilter::LinearMipmapNearest => LINEAR_MIPMAP_NEAREST,
            MinFilter::NearestMipmapLinear => NEAREST_MIPMAP_LINEAR,
            MinFilter::LinearMipmapLinear => LINEAR_MIPMAP_LINEAR,
        }
    }
}

/// Texture co-ordinate wrapping mode.
#[derive(Clone, Copy, Debug, DeJson, Eq, PartialEq)]
pub enum WrappingMode {
    /// Corresponds to `GL_CLAMP_TO_EDGE`.
    ClampToEdge,

    /// Corresponds to `GL_MIRRORED_REPEAT`.
    MirroredRepeat,

    /// Corresponds to `GL_REPEAT`.
    Repeat,
}

impl WrappingMode {
    /// Returns the corresponding OpenGL enum value.
    pub fn as_gl_enum(&self) -> u32 {
        match *self {
            WrappingMode::ClampToEdge => CLAMP_TO_EDGE,
            WrappingMode::MirroredRepeat => MIRRORED_REPEAT,
            WrappingMode::Repeat => REPEAT,
        }
    }
}

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug, Default, DeJson, SerJson, Validate)]
#[nserde(default)]
pub struct Sampler {
    /// Magnification filter.
    #[nserde(rename = "magFilter")]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub mag_filter: Option<Checked<MagFilter>>,

    /// Minification filter.
    #[nserde(rename = "minFilter")]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub min_filter: Option<Checked<MinFilter>>,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", nserde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// `s` wrapping mode.
    #[nserde(default)]
    #[nserde(rename = "wrapS")]
    pub wrap_s: Checked<WrappingMode>,

    /// `t` wrapping mode.
    #[nserde(default)]
    #[nserde(rename = "wrapT")]
    pub wrap_t: Checked<WrappingMode>,

    /// Extension specific data.
    #[nserde(default)]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::texture::Sampler>,

    /// Optional application specific data.
    #[nserde(default)]
    #[cfg_attr(feature = "extras", nserde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), nserde(skip_serializing))]
    pub extras: Extras,
}

/// A texture and its sampler.
#[derive(Clone, Debug, DeJson, SerJson, Validate)]
pub struct Texture {
    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", nserde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// The index of the sampler used by this texture.
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<Index<Sampler>>,

    /// The index of the image used by this texture.
    pub source: Index<image::Image>,

    /// Extension specific data.
    #[nserde(default)]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::texture::Texture>,

    /// Optional application specific data.
    #[nserde(default)]
    #[cfg_attr(feature = "extras", nserde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), nserde(skip_serializing))]
    pub extras: Extras,
}

#[derive(Clone, Debug, DeJson, SerJson, Validate)]
/// Reference to a `Texture`.
pub struct Info {
    /// The index of the texture.
    pub index: Index<Texture>,

    /// The set index of the texture's `TEXCOORD` attribute.
    #[nserde(default)]
    #[nserde(rename = "texCoord")]
    pub tex_coord: u32,

    /// Extension specific data.
    #[nserde(default)]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::texture::Info>,

    /// Optional application specific data.
    #[nserde(default)]
    #[cfg_attr(feature = "extras", nserde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), nserde(skip_serializing))]
    pub extras: Extras,
}

impl DeJson for Checked<MagFilter> {
    fn deserialize_json(input: &str) -> Result<Self, nanoserde::DeJsonErr> {
        use Checked::*;
        use MagFilter::*;

        let value: u32 = input.try_into()?;

        Ok(match value {
            NEAREST => Valid(Nearest),
            LINEAR => Valid(Linear),
            _ => Invalid,
        })
    }
}

// impl<'de> de::Deserialize<'de> for Checked<MagFilter> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: de::Deserializer<'de>,
//     {
//         struct Visitor;
//         impl<'de> de::Visitor<'de> for Visitor {
//             type Value = Checked<MagFilter>;

//             fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
//                 write!(f, "any of: {:?}", VALID_MAG_FILTERS)
//             }

//             fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
//             where
//                 E: de::Error,
//             {
//                 use self::MagFilter::*;
//                 use crate::validation::Checked::*;
//                 Ok(match value as u32 {
//                     NEAREST => Valid(Nearest),
//                     LINEAR => Valid(Linear),
//                     _ => Invalid,
//                 })
//             }
//         }
//         deserializer.deserialize_u64(Visitor)
//     }
// }

impl DeJson for Checked<MinFilter> {
    fn deserialize_json(input: &str) -> Result<Self, nanoserde::DeJsonErr> {
        use Checked::*;
        use MinFilter::*;

        let value: u32 = input.try_into()?;

        Ok(match value {
            NEAREST => Valid(Nearest),
            LINEAR => Valid(Linear),
            NEAREST_MIPMAP_NEAREST => Valid(NearestMipmapNearest),
            LINEAR_MIPMAP_NEAREST => Valid(LinearMipmapNearest),
            NEAREST_MIPMAP_LINEAR => Valid(NearestMipmapLinear),
            LINEAR_MIPMAP_LINEAR => Valid(LinearMipmapLinear),
            _ => Invalid,
        })
    }
}

// impl<'de> de::Deserialize<'de> for Checked<MinFilter> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: de::Deserializer<'de>,
//     {
//         struct Visitor;
//         impl<'de> de::Visitor<'de> for Visitor {
//             type Value = Checked<MinFilter>;

//             fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
//                 write!(f, "any of: {:?}", VALID_MIN_FILTERS)
//             }

//             fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
//             where
//                 E: de::Error,
//             {
//                 use self::MinFilter::*;
//                 use crate::validation::Checked::*;
//                 Ok(match value as u32 {
//                     NEAREST => Valid(Nearest),
//                     LINEAR => Valid(Linear),
//                     NEAREST_MIPMAP_NEAREST => Valid(NearestMipmapNearest),
//                     LINEAR_MIPMAP_NEAREST => Valid(LinearMipmapNearest),
//                     NEAREST_MIPMAP_LINEAR => Valid(NearestMipmapLinear),
//                     LINEAR_MIPMAP_LINEAR => Valid(LinearMipmapLinear),
//                     _ => Invalid,
//                 })
//             }
//         }
//         deserializer.deserialize_u64(Visitor)
//     }
// }

// impl ser::Serialize for MinFilter {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: ser::Serializer,
//     {
//         serializer.serialize_u32(self.as_gl_enum())
//     }
// }

impl SerJson for MinFilter {
    fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
        self.as_gl_enum().ser_json(d, s);
    }
}

impl DeJson for Checked<WrappingMode> {
    fn deserialize_json(input: &str) -> Result<Self, nanoserde::DeJsonErr> {
        use Checked::*;
        use WrappingMode::*;

        let value: u32 = input.try_into()?;

        Ok(match value {
            CLAMP_TO_EDGE => Valid(ClampToEdge),
            MIRRORED_REPEAT => Valid(MirroredRepeat),
            REPEAT => Valid(Repeat),
            _ => Invalid,
        })
    }
}

// impl<'de> de::Deserialize<'de> for Checked<WrappingMode> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: de::Deserializer<'de>,
//     {
//         struct Visitor;
//         impl<'de> de::Visitor<'de> for Visitor {
//             type Value = Checked<WrappingMode>;

//             fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
//                 write!(f, "any of: {:?}", VALID_WRAPPING_MODES)
//             }

//             fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
//             where
//                 E: de::Error,
//             {
//                 use self::WrappingMode::*;
//                 use crate::validation::Checked::*;
//                 Ok(match value as u32 {
//                     CLAMP_TO_EDGE => Valid(ClampToEdge),
//                     MIRRORED_REPEAT => Valid(MirroredRepeat),
//                     REPEAT => Valid(Repeat),
//                     _ => Invalid,
//                 })
//             }
//         }
//         deserializer.deserialize_u64(Visitor)
//     }
// }

// impl ser::Serialize for MagFilter {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: ser::Serializer,
//     {
//         serializer.serialize_u32(self.as_gl_enum())
//     }
// }

impl SerJson for MagFilter {
    fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
        self.as_gl_enum().ser_json(d, s);
    }
}

impl Default for WrappingMode {
    fn default() -> Self {
        WrappingMode::Repeat
    }
}

impl SerJson for WrappingMode {
    fn ser_json(&self, d: usize, s: &mut nanoserde::SerJsonState) {
        self.as_gl_enum().ser_json(d, s);
    }
}

// impl ser::Serialize for WrappingMode {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: ser::Serializer,
//     {
//         serializer.serialize_u32(self.as_gl_enum())
//     }
// }
