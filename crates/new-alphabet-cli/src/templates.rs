use new_alphabet_core::IntentKind;

pub struct RecipeTemplate {
    pub canonical_name: &'static str,
    pub module_name: &'static str,
    pub surface_name: &'static str,
    pub example_component: &'static str,
    pub intent: IntentKind,
}

pub fn recipe_template(name: &str) -> Option<RecipeTemplate> {
    match normalize(name).as_str() {
        "blogindex" => Some(RecipeTemplate {
            canonical_name: "BlogIndex",
            module_name: "blog_index",
            surface_name: "BlogIndexSurface",
            example_component: "BlogIndexExample",
            intent: IntentKind::Editorial,
        }),
        "articleshell" => Some(RecipeTemplate {
            canonical_name: "ArticleShell",
            module_name: "article_shell",
            surface_name: "ArticleShellSurface",
            example_component: "ArticleShellExample",
            intent: IntentKind::Editorial,
        }),
        "docsshell" => Some(RecipeTemplate {
            canonical_name: "DocsShell",
            module_name: "docs_shell",
            surface_name: "DocsShellSurface",
            example_component: "DocsShellExample",
            intent: IntentKind::Editorial,
        }),
        "searchresultsworkspace" => Some(RecipeTemplate {
            canonical_name: "SearchResultsWorkspace",
            module_name: "search_results_workspace",
            surface_name: "SearchResultsWorkspaceSurface",
            example_component: "SearchResultsWorkspaceExample",
            intent: IntentKind::Workspace,
        }),
        "reviewqueue" => Some(RecipeTemplate {
            canonical_name: "ReviewQueue",
            module_name: "review_queue",
            surface_name: "ReviewQueueSurface",
            example_component: "ReviewQueueExample",
            intent: IntentKind::Workspace,
        }),
        "settingsworkspace" => Some(RecipeTemplate {
            canonical_name: "SettingsWorkspace",
            module_name: "settings_workspace",
            surface_name: "SettingsWorkspaceSurface",
            example_component: "SettingsWorkspaceExample",
            intent: IntentKind::Workspace,
        }),
        "dashboardshell" => Some(RecipeTemplate {
            canonical_name: "DashboardShell",
            module_name: "dashboard_shell",
            surface_name: "DashboardShellSurface",
            example_component: "DashboardShellExample",
            intent: IntentKind::Workspace,
        }),
        _ => None,
    }
}

pub fn render_recipe_module(template: &RecipeTemplate) -> String {
    format!(
        "use leptos::prelude::*;\nuse new_alphabet_recipes::{};\n\n#[component]\npub fn {}() -> impl IntoView {{\n    view! {{ <{} /> }}\n}}\n",
        template.example_component, template.surface_name, template.example_component
    )
}

pub fn component_names() -> &'static [&'static str] {
    &[
        "Button",
        "LinkAction",
        "TextField",
        "Textarea",
        "Select",
        "Checkbox",
        "RadioGroup",
        "Switch",
        "StatusBadge",
        "InlineAlert",
        "EmptyState",
        "Table",
        "MetricBlock",
        "Pagination",
        "NavIndex",
        "CommandBar",
        "FilterRail",
        "DetailPane",
    ]
}

pub fn component_module_name(name: &str) -> Option<&'static str> {
    match normalize(name).as_str() {
        "button" => Some("button"),
        "linkaction" => Some("link_action"),
        "textfield" => Some("text_field"),
        "textarea" => Some("textarea"),
        "select" => Some("select"),
        "checkbox" => Some("checkbox"),
        "radiogroup" => Some("radio_group"),
        "switch" => Some("switch"),
        "statusbadge" => Some("status_badge"),
        "inlinealert" => Some("inline_alert"),
        "emptystate" => Some("empty_state"),
        "table" => Some("table"),
        "metricblock" => Some("metric_block"),
        "pagination" => Some("pagination"),
        "navindex" => Some("nav_index"),
        "commandbar" => Some("command_bar"),
        "filterrail" => Some("filter_rail"),
        "detailpane" => Some("detail_pane"),
        _ => None,
    }
}

pub fn component_required_states(name: &str) -> &'static [&'static str] {
    match normalize(name).as_str() {
        "button" | "linkaction" => &[
            "default",
            "loading",
            "disabled",
            "hover",
            "active",
            "focus_visible",
        ],
        "textfield" | "textarea" | "select" | "checkbox" | "radiogroup" | "switch" => {
            &["default", "error", "success", "disabled", "focus_visible"]
        }
        "statusbadge" => &["info", "success", "warning", "error"],
        "inlinealert" => &["info", "success", "warning", "error"],
        "emptystate" => &["empty"],
        "table" => &["default", "loading", "empty", "error"],
        "metricblock" => &["default"],
        "pagination" => &["default", "disabled"],
        "navindex" => &["default", "current"],
        "commandbar" => &["default", "loading", "disabled"],
        "filterrail" => &["default", "zero_result"],
        "detailpane" => &["default", "loading", "unavailable"],
        _ => &[],
    }
}

pub fn render_component_module(name: &str) -> Option<String> {
    let states = component_required_states(name)
        .iter()
        .map(|state| format!("    \"{state}\",\n"))
        .collect::<String>();

    match normalize(name).as_str() {
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
    }
}

pub fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .collect()
}

fn with_states(template: &str, states: &str) -> String {
    template.replace("__STATES__", states)
}
