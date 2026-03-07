use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LayerKind {
    Foundation,
    Primitive,
    Component,
    Recipe,
    Project,
    Contract,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IntentKind {
    Editorial,
    Workspace,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PromptLevel {
    Sparse,
    Moderate,
    High,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationCategory {
    Composition,
    Spacing,
    StateCoverage,
    Accessibility,
    Naming,
    AntiPatternUsage,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Note,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CompositionRuleKind {
    Allowed,
    Disallowed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NamedValue {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoctrineSummary {
    pub thesis: String,
    pub posture: Vec<String>,
    pub required_session_context: Vec<String>,
    pub hard_rules: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FoundationToken {
    pub id: String,
    pub description: String,
    pub values: Vec<NamedValue>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FoundationFamily {
    pub id: String,
    pub title: String,
    pub tokens: Vec<FoundationToken>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractOptionGroup {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrimitiveContract {
    pub id: String,
    pub purpose: String,
    pub required_props: Vec<String>,
    pub optional_props: Vec<String>,
    pub finite_options: Vec<ContractOptionGroup>,
    pub composition_notes: Vec<String>,
    pub example_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentContract {
    pub id: String,
    pub purpose: String,
    pub built_from: Vec<String>,
    pub required_states: Vec<String>,
    pub accessibility: Vec<String>,
    pub finite_options: Vec<ContractOptionGroup>,
    pub example_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeContract {
    pub id: String,
    pub intent: IntentKind,
    pub purpose: String,
    pub required_regions: Vec<String>,
    pub optional_regions: Vec<String>,
    pub adaptation_limits: Vec<String>,
    pub primitives: Vec<String>,
    pub components: Vec<String>,
    pub example_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompositionRule {
    pub id: String,
    pub kind: CompositionRuleKind,
    pub composition: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateContract {
    pub id: String,
    pub applies_to: Vec<String>,
    pub required_states: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AntiPattern {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub repair: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationRule {
    pub id: String,
    pub category: ValidationCategory,
    pub default_severity: Severity,
    pub summary: String,
    pub repair: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SchemaDocument {
    pub id: String,
    pub title: String,
    pub layer: LayerKind,
    pub document: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReferenceExample {
    pub id: String,
    pub layer: LayerKind,
    pub target: String,
    pub source_path: String,
    pub summary: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptIntent {
    pub id: String,
    pub level: PromptLevel,
    pub prompt: String,
    pub recommended_recipe: String,
    pub plan_outline: Vec<String>,
    pub validation_focus: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ContractBundle {
    pub bundle_format_version: String,
    pub doctrine: DoctrineSummary,
    pub foundations: Vec<FoundationFamily>,
    pub primitives: Vec<PrimitiveContract>,
    pub components: Vec<ComponentContract>,
    pub recipes: Vec<RecipeContract>,
    pub composition_rules: Vec<CompositionRule>,
    pub state_contracts: Vec<StateContract>,
    pub anti_patterns: Vec<AntiPattern>,
    pub validation_rules: Vec<ValidationRule>,
    pub schemas: Vec<SchemaDocument>,
    pub examples: Vec<ReferenceExample>,
    pub prompt_intents: Vec<PromptIntent>,
    pub generation_sequence: Vec<String>,
    pub repair_sequence: Vec<String>,
}
