use serde::{Deserialize, Serialize};

use crate::IntentKind;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SurfaceRegion {
    pub kind: String,
    pub placement: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentInstance {
    pub id: String,
    pub states: Vec<String>,
    pub accessible_name: bool,
    pub keyboard_support: bool,
    pub text_equivalent: bool,
    pub custom_variant_props: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccessibilitySnapshot {
    pub semantic_html: bool,
    pub focus_visible: bool,
    pub color_independent_meaning: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SurfaceManifest {
    pub id: String,
    pub name: String,
    pub intent: IntentKind,
    pub recipe: String,
    pub regions: Vec<SurfaceRegion>,
    pub primitives: Vec<String>,
    pub components: Vec<ComponentInstance>,
    pub spacing_tokens: Vec<String>,
    pub custom_spacing_values: Vec<u16>,
    pub accessibility: AccessibilitySnapshot,
    pub style_escape_hatch: bool,
    pub invented_layout: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectManifest {
    pub schema_version: String,
    pub project_name: String,
    pub surfaces: Vec<SurfaceManifest>,
}
