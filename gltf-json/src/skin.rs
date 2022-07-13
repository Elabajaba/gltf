use crate::{accessor, extensions, scene, Extras, Index};
use gltf_derive::Validate;
use nanoserde::{DeJson, SerJson};

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, DeJson, SerJson, Validate)]
pub struct Skin {
    /// Extension specific data.
    #[nserde(default)]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::skin::Skin>,

    /// Optional application specific data.
    #[nserde(default)]
    #[cfg_attr(feature = "extras", nserde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), nserde(skip_serializing))]
    pub extras: Extras,

    /// The index of the accessor containing the 4x4 inverse-bind matrices.
    ///
    /// When `None`,each matrix is assumed to be the 4x4 identity matrix
    /// which implies that the inverse-bind matrices were pre-applied.
    #[nserde(rename = "inverseBindMatrices")]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub inverse_bind_matrices: Option<Index<accessor::Accessor>>,

    /// Indices of skeleton nodes used as joints in this skin.
    ///
    /// The array length must be the same as the `count` property of the
    /// `inverse_bind_matrices` `Accessor` (when defined).
    #[nserde(skip_serializing_if = "Vec::is_empty")]
    pub joints: Vec<Index<scene::Node>>,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", nserde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// The index of the node used as a skeleton root.
    ///
    /// When `None`, joints transforms resolve to scene root.
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub skeleton: Option<Index<scene::Node>>,
}
