use gltf_derive::Validate;
use nanoserde::{DeJson, SerJson};

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Default, DeJson, SerJson, Validate)]
pub struct Mesh {}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Default, DeJson, SerJson, Validate)]
pub struct Primitive {
    #[cfg(feature = "KHR_materials_variants")]
    #[nserde(default)]
    #[nserde(rename = "KHR_materials_variants")]
    #[nserde(skip_serializing_if = "Option::is_none")]
    pub khr_materials_variants: Option<KhrMaterialsVariants>,
}

#[cfg(feature = "KHR_materials_variants")]
#[derive(Clone, Debug, Default, DeJson, SerJson, Validate)]
pub struct KhrMaterialsVariants {
    pub mappings: Vec<Mapping>,
}

#[cfg(feature = "KHR_materials_variants")]
#[derive(Clone, Debug, Default, DeJson, SerJson, Validate)]
pub struct Mapping {
    pub material: u32,
    pub variants: Vec<u32>,
}
