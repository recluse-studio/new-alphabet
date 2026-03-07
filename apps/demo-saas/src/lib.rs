#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

use leptos::prelude::*;
use new_alphabet_components::{
    ChoiceOption, CommandAction, DetailField, DetailPaneState, FieldState, FilterGroup,
    FilterOption, FilterRailState, NavIndexItem, StatusSeverity, TableColumn, TableRow, TableState,
};
use new_alphabet_recipes::{
    DashboardMetric, DashboardShell, ReviewQueue, SearchResultsWorkspace, SettingsControl,
    SettingsPanel, SettingsWorkspace, WorkspaceCommands, WorkspaceContextItem, WorkspaceDetail,
    WorkspaceNavSection, WorkspacePagination, WorkspaceStatus,
};

const SEARCH_TYPE_OPTIONS: &[FilterOption] = &[
    FilterOption::selected("essay", "Essay", 12),
    FilterOption::new("note", "Note", 4),
];

const SEARCH_STATE_OPTIONS: &[FilterOption] = &[
    FilterOption::selected("ready", "Ready", 8),
    FilterOption::new("hold", "Hold", 3),
];

const SEARCH_FILTERS: &[FilterGroup] = &[
    FilterGroup::new("Type", SEARCH_TYPE_OPTIONS),
    FilterGroup::new("State", SEARCH_STATE_OPTIONS),
];

const SEARCH_COLUMNS: &[TableColumn] = &[
    TableColumn::truncate("entry", "Entry"),
    TableColumn::truncate("state", "State"),
    TableColumn::wrap("summary", "Summary"),
];

const SEARCH_ROWS: &[TableRow] = &[
    TableRow::new(
        "essay-142",
        &[
            "Essay 142",
            "Ready",
            "Lead is clear and the archive note now links to the correct citation.",
        ],
    ),
    TableRow::new(
        "essay-143",
        &[
            "Essay 143",
            "Hold",
            "Rights note is incomplete and the caption needs tightening before release.",
        ],
    ),
];

const SEARCH_SECONDARY: &[CommandAction] = &[
    CommandAction::ready("Open review queue", "/review/"),
    CommandAction::disabled("Export slice", "/search/export"),
];

const SEARCH_COMMANDS: WorkspaceCommands = WorkspaceCommands::new(
    "Search commands",
    CommandAction::ready("Open selected result", "/review/essay-142"),
    SEARCH_SECONDARY,
);

const SEARCH_DETAIL_FIELDS: &[DetailField] = &[
    DetailField::new("State", "Ready"),
    DetailField::new("Owner", "Editorial"),
];

const SEARCH_DETAIL: WorkspaceDetail = WorkspaceDetail::new(
    "Essay 142",
    Some("Search keeps the result field primary while adjacent detail remains explicit."),
    SEARCH_DETAIL_FIELDS,
    DetailPaneState::Default,
    None,
);

const SEARCH_STATUS: WorkspaceStatus = WorkspaceStatus::new(
    "Results refreshed",
    "Filters and table state now match the latest sync.",
    StatusSeverity::Info,
);

const SEARCH_PAGINATION: WorkspacePagination =
    WorkspacePagination::new(1, 3, None, Some("/search/page/2"));

const REVIEW_NAV_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Assigned", "/review/"),
    NavIndexItem::new("All", "/review/all"),
    NavIndexItem::new("Held", "/review/held"),
];

const REVIEW_NAV: WorkspaceNavSection = WorkspaceNavSection::new("Queues", REVIEW_NAV_ITEMS);

const REVIEW_FILTER_OPTIONS: &[FilterOption] = &[
    FilterOption::selected("ready", "Ready", 8),
    FilterOption::new("hold", "Hold", 3),
];

const REVIEW_FILTERS: &[FilterGroup] = &[FilterGroup::new("State", REVIEW_FILTER_OPTIONS)];

const REVIEW_COLUMNS: &[TableColumn] = &[
    TableColumn::truncate("entry", "Entry"),
    TableColumn::truncate("priority", "Priority"),
    TableColumn::wrap("summary", "Summary"),
];

const REVIEW_ROWS: &[TableRow] = &[
    TableRow::new(
        "essay-142",
        &[
            "Essay 142",
            "Immediate",
            "Lead is clean and the reference trail is complete.",
        ],
    ),
    TableRow::new(
        "essay-143",
        &[
            "Essay 143",
            "Follow-up",
            "Caption and rights note still need explicit approval.",
        ],
    ),
];

const REVIEW_SECONDARY: &[CommandAction] = &[
    CommandAction::ready("Open search", "/search/"),
    CommandAction::disabled("Bulk export", "/review/export"),
];

const REVIEW_ACTIONS: WorkspaceCommands = WorkspaceCommands::new(
    "Queue commands",
    CommandAction::ready("Approve selected", "/review/approve"),
    REVIEW_SECONDARY,
);

const REVIEW_DETAIL_FIELDS: &[DetailField] = &[
    DetailField::new("State", "Ready"),
    DetailField::new("Owner", "Editorial"),
    DetailField::new("History", "2 previous passes"),
];

const REVIEW_DETAIL: WorkspaceDetail = WorkspaceDetail::new(
    "Essay 142",
    Some("Decision context stays adjacent so the queue remains dense and legible."),
    REVIEW_DETAIL_FIELDS,
    DetailPaneState::Default,
    None,
);

const REVIEW_STATUS: WorkspaceStatus = WorkspaceStatus::new(
    "Queue synced",
    "The next decision set is ready.",
    StatusSeverity::Success,
);

const SETTINGS_NAV_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Profile", "#profile"),
    NavIndexItem::new("Workflow", "#workflow"),
    NavIndexItem::new("Publishing", "#publishing"),
];

const SETTINGS_NAV: WorkspaceNavSection = WorkspaceNavSection::new("Sections", SETTINGS_NAV_ITEMS);

const DENSITY_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("calm", "Calm"),
    ChoiceOption::new("regular", "Regular"),
    ChoiceOption::new("dense", "Dense"),
];

const PROFILE_CONTROLS: &[SettingsControl] = &[
    SettingsControl::text(
        "Display name",
        "display-name",
        "Recluse Studio",
        FieldState::Default,
        Some("Public profile label."),
        None,
    ),
    SettingsControl::select(
        "Workspace density",
        "workspace-density",
        "dense",
        DENSITY_OPTIONS,
        FieldState::Default,
        Some("Choose a named operating mode."),
        None,
    ),
];

const WORKFLOW_CONTROLS: &[SettingsControl] = &[
    SettingsControl::switch(
        "Private mode",
        "private-mode",
        true,
        FieldState::Default,
        Some("Can be changed later in settings."),
    ),
    SettingsControl::checkbox(
        "Attach follow-up automatically",
        "attach-follow-up",
        false,
        FieldState::Default,
        Some("Adds a note after each decision."),
    ),
];

const SETTINGS_PANELS: &[SettingsPanel] = &[
    SettingsPanel::new(
        "Profile",
        "Public identity and reading posture remain explicit and narrow.",
        PROFILE_CONTROLS,
    ),
    SettingsPanel::new(
        "Workflow",
        "Operational toggles stay semantic and finite.",
        WORKFLOW_CONTROLS,
    ),
];

const SETTINGS_STATUS: WorkspaceStatus = WorkspaceStatus::new(
    "Settings saved",
    "The workspace profile matches the current operating mode.",
    StatusSeverity::Success,
);

const SETTINGS_CONTEXT: &[WorkspaceContextItem] = &[
    WorkspaceContextItem::new("Review queue", "/review/"),
    WorkspaceContextItem::new("Search", "/search/"),
];

const DASHBOARD_METRICS: &[DashboardMetric] = &[
    DashboardMetric::new(
        "Published today",
        "18",
        Some("Across archive and review surfaces."),
        Some("Held items remain separate."),
    ),
    DashboardMetric::new(
        "Ready for review",
        "12",
        Some("Current queue depth."),
        Some("Two require rights follow-up."),
    ),
];

const DASHBOARD_COLUMNS: &[TableColumn] = &[
    TableColumn::truncate("surface", "Surface"),
    TableColumn::truncate("state", "State"),
    TableColumn::wrap("note", "Note"),
];

const DASHBOARD_ROWS: &[TableRow] = &[
    TableRow::new(
        "search",
        &[
            "Search",
            "Active",
            "Filters and results remain in sync with the current query.",
        ],
    ),
    TableRow::new(
        "review",
        &[
            "Review",
            "Ready",
            "Decision queue is explicit about state and adjacent detail.",
        ],
    ),
    TableRow::new(
        "settings",
        &[
            "Settings",
            "Saved",
            "Workspace profile remains narrow and semantic.",
        ],
    ),
];

const DASHBOARD_STATUS: WorkspaceStatus = WorkspaceStatus::new(
    "Morning pass ready",
    "Search, review, and settings surfaces are aligned with the current workflow state.",
    StatusSeverity::Info,
);

const DASHBOARD_CONTEXT: &[WorkspaceContextItem] = &[
    WorkspaceContextItem::new("Search", "/search/"),
    WorkspaceContextItem::new("Review", "/review/"),
    WorkspaceContextItem::new("Settings", "/settings/"),
];

pub fn render_dashboard_document() -> String {
    let body = view! {
        <DashboardShell
            title="Editorial Operations"
            metrics=DASHBOARD_METRICS
            summary_columns=DASHBOARD_COLUMNS
            summary_rows=DASHBOARD_ROWS
            status=DASHBOARD_STATUS
            context=DASHBOARD_CONTEXT
            summary_state=TableState::Default
        />
    }
    .to_html()
    .replace("<!>", "");

    html_document("Editorial Operations", body)
}

pub fn render_search_document() -> String {
    let body = view! {
        <SearchResultsWorkspace
            title="Search workspace"
            query="essay"
            filters=SEARCH_FILTERS
            results_columns=SEARCH_COLUMNS
            results_rows=SEARCH_ROWS
            filter_state=FilterRailState::Default
            results_state=TableState::Default
            status=SEARCH_STATUS
            commands=SEARCH_COMMANDS
            detail=SEARCH_DETAIL
            pagination=SEARCH_PAGINATION
        />
    }
    .to_html()
    .replace("<!>", "");

    html_document("Search workspace", body)
}

pub fn render_review_document() -> String {
    let body = view! {
        <ReviewQueue
            title="Review queue"
            queue_columns=REVIEW_COLUMNS
            queue_rows=REVIEW_ROWS
            actions=REVIEW_ACTIONS
            detail=REVIEW_DETAIL
            queue_state=TableState::Default
            status=REVIEW_STATUS
            navigation=REVIEW_NAV
            filters=REVIEW_FILTERS
            filter_state=FilterRailState::Default
        />
    }
    .to_html()
    .replace("<!>", "");

    html_document("Review queue", body)
}

pub fn render_settings_document() -> String {
    let body = view! {
        <SettingsWorkspace
            title="Settings workspace"
            navigation=SETTINGS_NAV
            panels=SETTINGS_PANELS
            status=SETTINGS_STATUS
            context=SETTINGS_CONTEXT
        />
    }
    .to_html()
    .replace("<!>", "");

    html_document("Settings workspace", body)
}

pub fn write_site(site_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(site_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(site_dir.join("search")).map_err(|error| error.to_string())?;
    fs::create_dir_all(site_dir.join("review")).map_err(|error| error.to_string())?;
    fs::create_dir_all(site_dir.join("settings")).map_err(|error| error.to_string())?;

    fs::write(
        site_dir.join("index.html"),
        render_dashboard_document().as_bytes(),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        site_dir.join("search").join("index.html"),
        render_search_document().as_bytes(),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        site_dir.join("review").join("index.html"),
        render_review_document().as_bytes(),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        site_dir.join("settings").join("index.html"),
        render_settings_document().as_bytes(),
    )
    .map_err(|error| error.to_string())?;

    Ok(())
}

fn html_document(title: &str, body: String) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{title}</title>\n</head>\n<body>\n{body}\n</body>\n</html>\n"
    )
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn dashboard_document_contains_operational_summary() {
        let html = render_dashboard_document();

        assert!(html.contains("Editorial Operations"));
        assert!(html.contains("Published today"));
        assert!(html.contains("Search"));
    }

    #[test]
    fn review_document_contains_queue_and_detail() {
        let html = render_review_document();

        assert!(html.contains("Review queue"));
        assert!(html.contains("Approve selected"));
        assert!(html.contains("Decision context stays adjacent"));
    }

    #[test]
    fn write_site_writes_all_surface_pages() {
        let site_dir = unique_test_dir("demo-saas");

        write_site(&site_dir).expect("site to write");

        assert!(site_dir.join("index.html").exists());
        assert!(site_dir.join("search/index.html").exists());
        assert!(site_dir.join("review/index.html").exists());
        assert!(site_dir.join("settings/index.html").exists());
    }

    fn unique_test_dir(label: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let dir = env::temp_dir().join(format!("new-alphabet-{label}-{stamp}"));
        fs::create_dir_all(&dir).expect("dir");
        dir
    }
}
