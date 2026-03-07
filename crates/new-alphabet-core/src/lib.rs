#![forbid(unsafe_code)]

mod contract;
mod project;
mod validation;

pub use contract::{
    AntiPattern, ComponentContract, CompositionRule, CompositionRuleKind, ContractBundle,
    ContractOptionGroup, DoctrineSummary, FoundationFamily, FoundationToken, IntentKind, LayerKind,
    NamedValue, PrimitiveContract, PromptIntent, PromptLevel, RecipeContract, ReferenceExample,
    SchemaDocument, Severity, StateContract, ValidationCategory, ValidationRule,
};
pub use project::{
    AccessibilitySnapshot, ComponentInstance, ProjectManifest, SurfaceManifest, SurfaceRegion,
};
pub use validation::{ValidationMessage, ValidationReport};
