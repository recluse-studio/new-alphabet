use new_alphabet_core::IntentKind;
use new_alphabet_schema::contract_bundle;

pub struct RecipeTemplate {
    pub canonical_name: String,
    pub module_name: String,
    pub surface_name: String,
    pub example_component: String,
    pub intent: IntentKind,
    pub required_regions: Vec<String>,
    pub optional_regions: Vec<String>,
    pub primitives: Vec<String>,
    pub components: Vec<String>,
    pub example_ids: Vec<String>,
    pub documentation_paths: Vec<String>,
}

pub struct ComponentTemplate {
    pub canonical_name: String,
    pub module_name: String,
    pub required_states: Vec<String>,
    pub built_from: Vec<String>,
    pub foundation_bindings: Vec<String>,
    pub example_ids: Vec<String>,
    pub documentation_paths: Vec<String>,
}

pub fn recipe_template(name: &str) -> Option<RecipeTemplate> {
    let recipe = contract_bundle()
        .recipes
        .into_iter()
        .find(|recipe| normalize(&recipe.id) == normalize(name))?;

    Some(RecipeTemplate {
        module_name: to_snake_case(&recipe.id),
        surface_name: format!("{}Surface", recipe.id),
        example_component: format!("{}Example", recipe.id),
        documentation_paths: recipe_documentation_paths(recipe.intent.clone()),
        canonical_name: recipe.id,
        intent: recipe.intent,
        required_regions: recipe.required_regions,
        optional_regions: recipe.optional_regions,
        primitives: recipe.primitives,
        components: recipe.components,
        example_ids: recipe.example_ids,
    })
}

pub fn render_recipe_module(template: &RecipeTemplate) -> String {
    format!(
        "use leptos::prelude::*;\nuse new_alphabet_recipes::{};\n\npub const RECIPE_ID: &str = \"{}\";\npub const REQUIRED_REGIONS: &[&str] = &[\n{}];\npub const OPTIONAL_REGIONS: &[&str] = &[\n{}];\npub const REQUIRED_PRIMITIVES: &[&str] = &[\n{}];\npub const REQUIRED_COMPONENTS: &[&str] = &[\n{}];\npub const REFERENCE_EXAMPLES: &[&str] = &[\n{}];\npub const DOCUMENTATION_PATHS: &[&str] = &[\n{}];\n\n#[component]\npub fn {}() -> impl IntoView {{\n    view! {{ <{} /> }}\n}}\n",
        template.example_component,
        template.canonical_name,
        render_string_array(&template.required_regions),
        render_string_array(&template.optional_regions),
        render_string_array(&template.primitives),
        render_string_array(&template.components),
        render_string_array(&template.example_ids),
        render_string_array(&template.documentation_paths),
        template.surface_name,
        template.example_component
    )
}

pub fn component_names() -> Vec<String> {
    contract_bundle()
        .components
        .into_iter()
        .map(|component| component.id)
        .collect()
}

pub fn component_module_name(name: &str) -> Option<String> {
    let component = contract_bundle()
        .components
        .into_iter()
        .find(|component| normalize(&component.id) == normalize(name))?;

    Some(to_snake_case(&component.id))
}

fn component_template(name: &str) -> Option<ComponentTemplate> {
    let component = contract_bundle()
        .components
        .into_iter()
        .find(|component| normalize(&component.id) == normalize(name))?;

    Some(ComponentTemplate {
        module_name: to_snake_case(&component.id),
        foundation_bindings: component_foundation_bindings(&component.id),
        documentation_paths: component_documentation_paths(&component.id),
        canonical_name: component.id,
        required_states: component.required_states,
        built_from: component.built_from,
        example_ids: component.example_ids,
    })
}

pub fn render_component_module(name: &str) -> Option<String> {
    let template = component_template(name)?;
    let states = render_string_array(&template.required_states);

    let rendered = match template.module_name.as_str() {
        "button" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{ActionPriority, ActionState, Button};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn ButtonScaffold() -> impl IntoView {
    view! {
        <Button
            label="Primary action"
            priority=ActionPriority::Primary
            state=ActionState::Ready
        />
    }
}
"#,
            &states,
        )),
        "linkaction" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{ActionPriority, ActionState, LinkAction};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn LinkActionScaffold() -> impl IntoView {
    view! {
        <LinkAction
            label="Read archive note"
            href="/notes/grid-law"
            priority=ActionPriority::Secondary
            state=ActionState::Ready
        />
    }
}
"#,
            &states,
        )),
        "textfield" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{FieldState, TextField};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn TextFieldScaffold() -> impl IntoView {
    view! {
        <TextField
            label="Display name"
            name="display-name"
            value="Recluse Studio"
            state=FieldState::Default
            help="Public profile label."
        />
    }
}
"#,
            &states,
        )),
        "textarea" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{FieldState, Textarea};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn TextareaScaffold() -> impl IntoView {
    view! {
        <Textarea
            label="Review note"
            name="review-note"
            value="Context stays explicit and structural."
            state=FieldState::Default
            help="Internal review summary."
        />
    }
}
"#,
            &states,
        )),
        "select" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{ChoiceOption, FieldState, Select};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

const OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("calm", "Calm"),
    ChoiceOption::new("regular", "Regular"),
    ChoiceOption::new("dense", "Dense"),
];

#[component]
pub fn SelectScaffold() -> impl IntoView {
    view! {
        <Select
            label="Workspace density"
            name="workspace-density"
            selected="regular"
            options=OPTIONS
            state=FieldState::Default
            help="Choose a named operating mode."
        />
    }
}
"#,
            &states,
        )),
        "checkbox" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{Checkbox, FieldState};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn CheckboxScaffold() -> impl IntoView {
    view! {
        <Checkbox
            label="Attach follow-up automatically"
            name="attach-follow-up"
            checked=false
            state=FieldState::Default
            message="Adds a note after each decision."
        />
    }
}
"#,
            &states,
        )),
        "radiogroup" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{ChoiceOption, FieldState, RadioGroup};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

const OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("approve", "Approve"),
    ChoiceOption::new("hold", "Hold"),
];

#[component]
pub fn RadioGroupScaffold() -> impl IntoView {
    view! {
        <RadioGroup
            label="Review decision"
            name="review-decision"
            selected="approve"
            options=OPTIONS
            state=FieldState::Default
            message="Choose a named review outcome."
        />
    }
}
"#,
            &states,
        )),
        "switch" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{FieldState, Switch};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn SwitchScaffold() -> impl IntoView {
    view! {
        <Switch
            label="Private mode"
            name="private-mode"
            checked=true
            state=FieldState::Default
            message="Can be changed later in settings."
        />
    }
}
"#,
            &states,
        )),
        "statusbadge" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{StatusBadge, StatusSeverity};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn StatusBadgeScaffold() -> impl IntoView {
    view! { <StatusBadge label="Ready" severity=StatusSeverity::Success /> }
}
"#,
            &states,
        )),
        "inlinealert" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{InlineAlert, StatusSeverity};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn InlineAlertScaffold() -> impl IntoView {
    view! {
        <InlineAlert
            title="Queue synced"
            message="The review field is ready for the next pass."
            severity=StatusSeverity::Info
        />
    }
}
"#,
            &states,
        )),
        "emptystate" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::EmptyState;

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn EmptyStateScaffold() -> impl IntoView {
    view! {
        <EmptyState
            title="No published entries"
            message="The archive is empty."
            next_action="Publish the first entry."
        />
    }
}
"#,
            &states,
        )),
        "table" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{Table, TableColumn, TableRow, TableState};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

const COLUMNS: &[TableColumn] = &[
    TableColumn::truncate("entry", "Entry"),
    TableColumn::truncate("state", "State"),
    TableColumn::wrap("summary", "Summary"),
];

const ROWS: &[TableRow] = &[
    TableRow::new(
        "essay-142",
        &[
            "Essay 142",
            "Ready",
            "Lead is clear and the archive note now links to the correct citation.",
        ],
    ),
];

#[component]
pub fn TableScaffold() -> impl IntoView {
    view! {
        <Table
            label="Review queue"
            columns=COLUMNS
            rows=ROWS
            state=TableState::Default
        />
    }
}
"#,
            &states,
        )),
        "metricblock" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::MetricBlock;

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn MetricBlockScaffold() -> impl IntoView {
    view! {
        <MetricBlock
            label="Published today"
            value="18"
            context="Across archive and review surfaces."
            note="Blocked items remain separate."
        />
    }
}
"#,
            &states,
        )),
        "pagination" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::Pagination;

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

#[component]
pub fn PaginationScaffold() -> impl IntoView {
    view! {
        <Pagination
            current_page=1
            total_pages=3
            next_href="/review?page=2"
        />
    }
}
"#,
            &states,
        )),
        "navindex" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{NavIndex, NavIndexItem};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

const ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Assigned", "/review/assigned"),
    NavIndexItem::new("All", "/review/all"),
];

#[component]
pub fn NavIndexScaffold() -> impl IntoView {
    view! { <NavIndex label="Queues" items=ITEMS /> }
}
"#,
            &states,
        )),
        "commandbar" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{CommandAction, CommandBar};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

const SECONDARY: &[CommandAction] = &[
    CommandAction::ready("Open history", "/review/history"),
    CommandAction::disabled("Export queue", "/review/export"),
];

#[component]
pub fn CommandBarScaffold() -> impl IntoView {
    view! {
        <CommandBar
            label="Queue commands"
            primary=CommandAction::ready("Approve selected", "/review/approve")
            secondary=SECONDARY
        />
    }
}
"#,
            &states,
        )),
        "filterrail" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{FilterGroup, FilterOption, FilterRail, FilterRailState};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

const TYPE_OPTIONS: &[FilterOption] = &[
    FilterOption::selected("essay", "Essay", 12),
    FilterOption::new("note", "Note", 4),
];

const GROUPS: &[FilterGroup] = &[
    FilterGroup::new("Type", TYPE_OPTIONS),
];

#[component]
pub fn FilterRailScaffold() -> impl IntoView {
    view! {
        <FilterRail
            label="Filters"
            groups=GROUPS
            state=FilterRailState::Default
        />
    }
}
"#,
            &states,
        )),
        "detailpane" => Some(with_states(
            r#"use leptos::prelude::*;
use new_alphabet_components::{DetailField, DetailPane, DetailPaneState};

pub const REQUIRED_STATES: &[&str] = &[
__STATES__];

const FIELDS: &[DetailField] = &[
    DetailField::new("State", "Ready"),
    DetailField::new("Owner", "Editorial"),
];

#[component]
pub fn DetailPaneScaffold() -> impl IntoView {
    view! {
        <DetailPane
            title="Essay 142"
            fields=FIELDS
            summary="Context remains adjacent so the queue stays dense."
            state=DetailPaneState::Default
        />
    }
}
"#,
            &states,
        )),
        _ => None,
    }?;

    Some(inject_component_contract_metadata(rendered, &template))
}

pub fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .collect()
}

fn render_string_array(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("    \"{value}\",\n"))
        .collect()
}

fn to_snake_case(value: &str) -> String {
    let mut rendered = String::new();

    for (index, character) in value.chars().enumerate() {
        if character.is_ascii_uppercase() {
            if index > 0 {
                rendered.push('_');
            }
            rendered.push(character.to_ascii_lowercase());
        } else {
            rendered.push(character);
        }
    }

    rendered
}

fn recipe_documentation_paths(intent: IntentKind) -> Vec<String> {
    let mut paths = vec![
        "docs/cli.md".to_owned(),
        "docs/foundations.md".to_owned(),
        "docs/primitives.md".to_owned(),
        "docs/recipes.md".to_owned(),
    ];

    if matches!(intent, IntentKind::Workspace) {
        paths.push("docs/components.md".to_owned());
    }

    paths
}

fn component_documentation_paths(id: &str) -> Vec<String> {
    let mut paths = vec![
        "docs/cli.md".to_owned(),
        "docs/foundations.md".to_owned(),
        "docs/primitives.md".to_owned(),
        "docs/components.md".to_owned(),
    ];

    if matches!(
        id,
        "Table" | "NavIndex" | "CommandBar" | "FilterRail" | "DetailPane"
    ) {
        paths.push("docs/recipes.md".to_owned());
    }

    paths
}

fn component_foundation_bindings(id: &str) -> Vec<String> {
    match id {
        "Button" | "LinkAction" => vec![
            "type.action.label".to_owned(),
            "spacing.control.inset".to_owned(),
            "state.interactive".to_owned(),
        ],
        "TextField" | "Textarea" | "Select" | "Checkbox" | "RadioGroup" | "Switch" => vec![
            "type.label.control".to_owned(),
            "spacing.control.inset".to_owned(),
            "state.field".to_owned(),
        ],
        "StatusBadge" | "InlineAlert" | "EmptyState" => vec![
            "type.label.meta".to_owned(),
            "color.status.semantic".to_owned(),
            "state.feedback".to_owned(),
        ],
        "Table" | "MetricBlock" | "Pagination" | "NavIndex" | "CommandBar" | "FilterRail"
        | "DetailPane" => vec![
            "type.data.compact".to_owned(),
            "spacing.stack.default".to_owned(),
            "state.workspace".to_owned(),
        ],
        _ => Vec::new(),
    }
}

fn inject_component_contract_metadata(body: String, template: &ComponentTemplate) -> String {
    let metadata = format!(
        "pub const COMPONENT_ID: &str = \"{}\";\npub const BUILT_FROM_PRIMITIVES: &[&str] = &[\n{}];\npub const FOUNDATION_BINDINGS: &[&str] = &[\n{}];\npub const REFERENCE_EXAMPLES: &[&str] = &[\n{}];\npub const DOCUMENTATION_PATHS: &[&str] = &[\n{}];",
        template.canonical_name,
        render_string_array(&template.built_from),
        render_string_array(&template.foundation_bindings),
        render_string_array(&template.example_ids),
        render_string_array(&template.documentation_paths),
    );

    body.replacen(
        "\n\n#[component]",
        &format!("\n\n{metadata}\n\n#[component]"),
        1,
    )
}

fn with_states(template: &str, states: &str) -> String {
    template.replace("__STATES__", states)
}
