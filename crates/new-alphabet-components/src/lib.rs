#![forbid(unsafe_code)]

mod actions;
mod choices;
mod data_display;
mod examples;
mod fields;
mod status;
mod workflow;

pub use actions::{ActionPriority, ActionState, Button, ButtonType, LinkAction};
pub use choices::{Checkbox, ChoiceOption, RadioGroup, Select, Switch};
pub use data_display::{
    MetricBlock, Pagination, PaginationDirection, Table, TableCellMode, TableColumn, TableRow,
    TableState,
};
pub use examples::{
    DashboardDataExample, DocumentationNavigationExample, EditorialActionExample,
    EditorialStatusExample, FormChoiceExample, FormFieldExample, ReviewDataExample,
    ReviewQueueCommandExample, SearchWorkflowExample, SettingsChoiceExample, SettingsFieldExample,
    WorkflowActionExample, WorkflowStatusExample,
};
pub use fields::{FieldState, TextField, Textarea};
pub use status::{EmptyState, InlineAlert, StatusBadge, StatusSeverity};
pub use workflow::{
    CommandAction, CommandBar, DetailField, DetailPane, DetailPaneState, FilterGroup, FilterOption,
    FilterRail, FilterRailState, NavIndex, NavIndexItem, NavIndexItemState,
};

#[cfg(test)]
const PLAN_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("solo", "Solo"),
    ChoiceOption::new("studio", "Studio"),
];

#[cfg(test)]
const DECISION_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("approve", "Approve"),
    ChoiceOption::new("hold", "Hold"),
];

#[cfg(test)]
const QUEUE_TABLE_COLUMNS: &[TableColumn] = &[
    TableColumn::truncate("entry", "Entry"),
    TableColumn::truncate("state", "State"),
    TableColumn::wrap("note", "Note"),
];

#[cfg(test)]
const QUEUE_ROW_ONE: &[&str] = &[
    "Essay 142",
    "Ready",
    "Lead is clear, but the archive citation still needs a source line.",
];

#[cfg(test)]
const QUEUE_ROW_TWO: &[&str] = &[
    "Essay 143",
    "Hold",
    "Image rights note is incomplete and the caption should be tightened before approval.",
];

#[cfg(test)]
const QUEUE_TABLE_ROWS: &[TableRow] = &[
    TableRow::new("essay-142", QUEUE_ROW_ONE),
    TableRow::new("essay-143", QUEUE_ROW_TWO),
];

#[cfg(test)]
const WORKSPACE_NAV_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Results", "/search"),
    NavIndexItem::new("Review queue", "/review"),
];

#[cfg(test)]
const PRIMARY_REVIEW_ACTION: CommandAction = CommandAction::ready("Approve selection", "/approve");

#[cfg(test)]
const SECONDARY_REVIEW_ACTIONS: &[CommandAction] = &[
    CommandAction::ready("Open history", "/history"),
    CommandAction::disabled("Export queue", "/export"),
];

#[cfg(test)]
const FILTER_TYPE_OPTIONS: &[FilterOption] = &[
    FilterOption::selected("essay", "Essay", 12),
    FilterOption::new("note", "Note", 4),
];

#[cfg(test)]
const FILTER_STATE_OPTIONS: &[FilterOption] = &[
    FilterOption::selected("ready", "Ready", 8),
    FilterOption::new("hold", "Hold", 3),
];

#[cfg(test)]
const FILTER_GROUPS: &[FilterGroup] = &[
    FilterGroup::new("Type", FILTER_TYPE_OPTIONS),
    FilterGroup::new("State", FILTER_STATE_OPTIONS),
];

#[cfg(test)]
const REVIEW_DETAIL_FIELDS: &[DetailField] = &[
    DetailField::new("State", "Ready"),
    DetailField::new("Section", "Archive"),
    DetailField::new("Owner", "Editorial"),
];

#[cfg(test)]
mod tests {
    use leptos::prelude::*;

    use super::*;

    fn render(view: impl FnOnce() -> AnyView + 'static) -> String {
        view().to_html()
    }

    #[test]
    fn button_renders_loading_state() {
        let html = render(|| {
            view! {
                <Button
                    label="Save changes"
                    priority=ActionPriority::Primary
                    state=ActionState::Loading
                />
            }
            .into_any()
        });

        assert!(html.contains("data-priority=\"primary\""));
        assert!(html.contains("data-state=\"loading\""));
        assert!(html.contains("aria-busy=\"true\""));
    }

    #[test]
    fn link_action_renders_secondary_semantics() {
        let html = render(|| {
            view! {
                <LinkAction
                    label="Read more"
                    href="/notes"
                    priority=ActionPriority::Secondary
                />
            }
            .into_any()
        });

        assert!(html.contains("href=\"/notes\""));
        assert!(html.contains("data-priority=\"secondary\""));
    }

    #[test]
    fn editorial_action_example_renders_editorial_surface() {
        let html = render(|| view! { <EditorialActionExample/> }.into_any());
        assert!(html.contains("Start reading"));
        assert!(html.contains("Join the list"));
    }

    #[test]
    fn workflow_action_example_renders_primary_and_secondary_actions() {
        let html = render(|| view! { <WorkflowActionExample/> }.into_any());
        assert!(html.contains("Loading…"));
        assert!(html.contains("Open history"));
    }

    #[test]
    fn text_field_renders_error_relationships() {
        let html = render(|| {
            view! {
                <TextField
                    label="Email"
                    name="email"
                    value="writer@example.com"
                    state=FieldState::Error
                    help="Used for publication updates."
                    message="Enter a valid address."
                />
            }
            .into_any()
        });

        assert!(html.contains("aria-invalid=\"true\""));
        assert!(html.contains("data-state=\"error\""));
        assert!(html.contains("Used for publication updates."));
        assert!(html.contains("Enter a valid address."));
    }

    #[test]
    fn textarea_renders_success_relationships() {
        let html = render(|| {
            view! {
                <Textarea
                    label="Notes"
                    name="notes"
                    value="Structured feedback."
                    state=FieldState::Success
                    help="Internal review context."
                    message="Saved."
                />
            }
            .into_any()
        });

        assert!(html.contains("data-state=\"success\""));
        assert!(html.contains("Saved."));
    }

    #[test]
    fn settings_field_example_renders_settings_context() {
        let html = render(|| view! { <SettingsFieldExample/> }.into_any());
        assert!(html.contains("Display name"));
        assert!(html.contains("Public profile label."));
    }

    #[test]
    fn form_field_example_renders_form_context() {
        let html = render(|| view! { <FormFieldExample/> }.into_any());
        assert!(html.contains("Submission notes"));
        assert!(html.contains("Saved."));
    }

    #[test]
    fn select_renders_error_relationships() {
        let html = render(|| {
            view! {
                <Select
                    label="Plan"
                    name="plan"
                    selected="studio"
                    options=PLAN_OPTIONS
                    state=FieldState::Error
                    message="Choose a supported plan."
                />
            }
            .into_any()
        });

        assert!(html.contains("aria-invalid=\"true\""));
        assert!(html.contains("Choose a supported plan."));
        assert!(html.contains("value=\"studio\""));
        assert!(html.contains("selected"));
    }

    #[test]
    fn checkbox_and_switch_render_accessible_labels() {
        let html = render(|| {
            view! {
                <div>
                    <Checkbox
                        label="Send updates"
                        name="updates"
                        checked=true
                    />
                    <Switch
                        label="Private mode"
                        name="private-mode"
                        checked=false
                    />
                </div>
            }
            .into_any()
        });

        assert!(html.contains("Send updates"));
        assert!(html.contains("role=\"switch\""));
        assert!(html.contains("Private mode"));
    }

    #[test]
    fn radio_group_renders_named_options() {
        let html = render(|| {
            view! {
                <RadioGroup
                    label="Review decision"
                    name="decision"
                    selected="approve"
                    options=DECISION_OPTIONS
                />
            }
            .into_any()
        });

        assert!(html.contains("Review decision"));
        assert!(html.contains("value=\"approve\" checked"));
    }

    #[test]
    fn settings_choice_example_renders_settings_context() {
        let html = render(|| view! { <SettingsChoiceExample/> }.into_any());
        assert!(html.contains("Workspace density"));
        assert!(html.contains("Private mode"));
    }

    #[test]
    fn form_choice_example_renders_form_sequence() {
        let html = render(|| view! { <FormChoiceExample/> }.into_any());
        assert!(html.contains("Review decision"));
        assert!(html.contains("Attach follow-up"));
    }

    #[test]
    fn status_badge_renders_semantic_label() {
        let html = render(|| {
            view! {
                <StatusBadge
                    label="Published"
                    severity=StatusSeverity::Success
                />
            }
            .into_any()
        });

        assert!(html.contains("Published"));
        assert!(html.contains("data-severity=\"success\""));
    }

    #[test]
    fn inline_alert_renders_structural_message() {
        let html = render(|| {
            view! {
                <InlineAlert
                    title="Sync delayed"
                    message="The queue will retry in one minute."
                    severity=StatusSeverity::Warning
                />
            }
            .into_any()
        });

        assert!(html.contains("Sync delayed"));
        assert!(html.contains("The queue will retry in one minute."));
        assert!(html.contains("role=\"status\""));
    }

    #[test]
    fn empty_state_explains_absence_and_next_action() {
        let html = render(|| {
            view! {
                <EmptyState
                    title="No matching entries"
                    message="Adjust the filters to broaden the archive."
                    next_action="Clear filters"
                />
            }
            .into_any()
        });

        assert!(html.contains("No matching entries"));
        assert!(html.contains("Clear filters"));
    }

    #[test]
    fn editorial_status_example_renders_editorial_feedback() {
        let html = render(|| view! { <EditorialStatusExample/> }.into_any());
        assert!(html.contains("Published"));
        assert!(html.contains("Archive note"));
    }

    #[test]
    fn workflow_status_example_renders_workflow_feedback() {
        let html = render(|| view! { <WorkflowStatusExample/> }.into_any());
        assert!(html.contains("Sync delayed"));
        assert!(html.contains("No matching entries"));
    }

    #[test]
    fn table_renders_dense_columns_and_rows() {
        let html = render(|| {
            view! {
                <Table
                    label="Review queue entries"
                    columns=QUEUE_TABLE_COLUMNS
                    rows=QUEUE_TABLE_ROWS
                />
            }
            .into_any()
        });

        assert!(html.contains("data-density=\"dense\""));
        assert!(html.contains("data-cell-mode=\"truncate\""));
        assert!(html.contains("data-cell-mode=\"wrap\""));
        assert!(html.contains("Essay 142"));
    }

    #[test]
    fn table_renders_loading_state() {
        let html = render(|| {
            view! {
                <Table
                    label="Review queue entries"
                    columns=QUEUE_TABLE_COLUMNS
                    rows=QUEUE_TABLE_ROWS
                    state=TableState::Loading
                />
            }
            .into_any()
        });

        assert!(html.contains("data-state=\"loading\""));
        assert!(html.contains("Loading rows."));
        assert!(html.contains("state.loading.muted"));
    }

    #[test]
    fn table_renders_empty_and_error_states() {
        let empty_html = render(|| {
            view! {
                <Table
                    label="Review queue entries"
                    columns=QUEUE_TABLE_COLUMNS
                    rows=&[]
                />
            }
            .into_any()
        });
        let error_html = render(|| {
            view! {
                <Table
                    label="Review queue entries"
                    columns=QUEUE_TABLE_COLUMNS
                    rows=QUEUE_TABLE_ROWS
                    state=TableState::Error
                    error_message="The review queue could not be loaded."
                />
            }
            .into_any()
        });

        assert!(empty_html.contains("data-state=\"empty\""));
        assert!(empty_html.contains("No rows available."));
        assert!(error_html.contains("data-state=\"error\""));
        assert!(error_html.contains("The review queue could not be loaded."));
    }

    #[test]
    fn metric_block_renders_label_value_and_note() {
        let html = render(|| {
            view! {
                <MetricBlock
                    label="Blocked items"
                    value="3"
                    note="Rights and metadata gaps only."
                />
            }
            .into_any()
        });

        assert!(html.contains("Blocked items"));
        assert!(html.contains(">3<"));
        assert!(html.contains("Rights and metadata gaps only."));
    }

    #[test]
    fn pagination_renders_semantic_navigation() {
        let html = render(|| {
            view! {
                <Pagination
                    current_page=2
                    total_pages=6
                    previous_href="/review?page=1"
                    next_href="/review?page=3"
                />
            }
            .into_any()
        });

        assert!(html.contains("aria-label=\"Pagination\""));
        assert!(html.contains("Page 2 of 6"));
        assert!(html.contains("rel=\"prev\""));
        assert!(html.contains("rel=\"next\""));
    }

    #[test]
    fn dashboard_data_example_renders_metrics_and_table() {
        let html = render(|| view! { <DashboardDataExample/> }.into_any());

        assert!(html.contains("Operations Snapshot"));
        assert!(html.contains("Published today"));
        assert!(html.contains("Dashboard queue summary"));
    }

    #[test]
    fn review_data_example_renders_workspace_queue() {
        let html = render(|| view! { <ReviewDataExample/> }.into_any());

        assert!(html.contains("Review queue"));
        assert!(html.contains("Review queue entries"));
        assert!(html.contains("Page 2 of 6"));
    }

    #[test]
    fn nav_index_renders_current_item() {
        let html = render(|| {
            view! {
                <NavIndex label="Workspace sections" items=WORKSPACE_NAV_ITEMS />
            }
            .into_any()
        });

        assert!(html.contains("aria-label=\"Workspace sections\""));
        assert!(html.contains("aria-current=\"page\""));
        assert!(html.contains("Review queue"));
    }

    #[test]
    fn command_bar_renders_primary_and_secondary_hierarchy() {
        let html = render(|| {
            view! {
                <CommandBar
                    label="Review commands"
                    primary=PRIMARY_REVIEW_ACTION
                    secondary=SECONDARY_REVIEW_ACTIONS
                />
            }
            .into_any()
        });

        assert!(html.contains("data-kind=\"command-bar\""));
        assert!(html.contains("data-priority=\"primary\""));
        assert!(html.contains("data-priority=\"secondary\""));
        assert!(html.contains("Approve selection"));
    }

    #[test]
    fn filter_rail_renders_groups_and_zero_result_state() {
        let html = render(|| {
            view! {
                <FilterRail
                    label="Search filters"
                    groups=FILTER_GROUPS
                    state=FilterRailState::ZeroResults
                />
            }
            .into_any()
        });

        assert!(html.contains("data-state=\"zero_results\""));
        assert!(html.contains("No results match the current filters."));
        assert!(html.contains("Essay"));
        assert!(html.contains(">12<"));
    }

    #[test]
    fn detail_pane_renders_default_fields() {
        let html = render(|| {
            view! {
                <DetailPane
                    title="Essay 142"
                    summary="Inspection stays adjacent, bounded, and explicit."
                    fields=REVIEW_DETAIL_FIELDS
                />
            }
            .into_any()
        });

        assert!(html.contains("data-state=\"default\""));
        assert!(html.contains("Inspection stays adjacent, bounded, and explicit."));
        assert!(html.contains("Archive"));
    }

    #[test]
    fn detail_pane_renders_loading_empty_and_unavailable_states() {
        let loading_html = render(|| {
            view! {
                <DetailPane
                    title="Essay 142"
                    fields=REVIEW_DETAIL_FIELDS
                    state=DetailPaneState::Loading
                />
            }
            .into_any()
        });
        let empty_html = render(|| {
            view! {
                <DetailPane
                    title="Essay 142"
                    fields=&[]
                />
            }
            .into_any()
        });
        let unavailable_html = render(|| {
            view! {
                <DetailPane
                    title="Essay 142"
                    fields=REVIEW_DETAIL_FIELDS
                    state=DetailPaneState::Unavailable
                    unavailable_message="This entry is unavailable because the archive is still syncing."
                />
            }
            .into_any()
        });

        assert!(loading_html.contains("data-state=\"loading\""));
        assert!(loading_html.contains("Loading detail."));
        assert!(empty_html.contains("data-state=\"empty\""));
        assert!(empty_html.contains("Select an entry to inspect."));
        assert!(unavailable_html.contains("data-state=\"unavailable\""));
        assert!(
            unavailable_html
                .contains("This entry is unavailable because the archive is still syncing.")
        );
    }

    #[test]
    fn search_workflow_example_renders_navigation_and_filters() {
        let html = render(|| view! { <SearchWorkflowExample/> }.into_any());

        assert!(html.contains("Workspace sections"));
        assert!(html.contains("Search filters"));
        assert!(html.contains("Search result entries"));
    }

    #[test]
    fn review_queue_command_example_renders_command_bar_and_detail() {
        let html = render(|| view! { <ReviewQueueCommandExample/> }.into_any());

        assert!(html.contains("Review commands"));
        assert!(html.contains("Approve selection"));
        assert!(html.contains("Essay 142"));
    }

    #[test]
    fn documentation_navigation_example_renders_editorial_navigation() {
        let html = render(|| view! { <DocumentationNavigationExample/> }.into_any());

        assert!(html.contains("Documentation sections"));
        assert!(html.contains("Foundations"));
        assert!(html.contains("Manual"));
    }
}
