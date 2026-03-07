use leptos::prelude::*;
use new_alphabet_components::{
    CommandAction, CommandBar, DetailField, DetailPane, DetailPaneState, FieldState, FilterGroup,
    FilterRail, FilterRailState, InlineAlert, StatusSeverity, Table, TableColumn, TableRow,
    TableState, TextField,
};
use new_alphabet_foundation::{DensityMode, RegionClass};
use new_alphabet_primitives::{
    AppShell, Band, FrameIntent, PageGrid, Panel, Rail, RailSide, Region, RegionPlacement,
    SectionHeader, Stack, StackSpace, SurfaceStrength,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkspaceCommands {
    pub label: &'static str,
    pub primary: CommandAction,
    pub secondary: &'static [CommandAction],
}

impl WorkspaceCommands {
    pub const fn new(
        label: &'static str,
        primary: CommandAction,
        secondary: &'static [CommandAction],
    ) -> Self {
        Self {
            label,
            primary,
            secondary,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkspaceStatus {
    pub title: &'static str,
    pub message: &'static str,
    pub severity: StatusSeverity,
}

impl WorkspaceStatus {
    pub const fn new(title: &'static str, message: &'static str, severity: StatusSeverity) -> Self {
        Self {
            title,
            message,
            severity,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkspaceDetail {
    pub title: &'static str,
    pub summary: Option<&'static str>,
    pub fields: &'static [DetailField],
    pub state: DetailPaneState,
    pub unavailable_message: Option<&'static str>,
}

impl WorkspaceDetail {
    pub const fn new(
        title: &'static str,
        summary: Option<&'static str>,
        fields: &'static [DetailField],
        state: DetailPaneState,
        unavailable_message: Option<&'static str>,
    ) -> Self {
        Self {
            title,
            summary,
            fields,
            state,
            unavailable_message,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkspacePagination {
    pub current_page: usize,
    pub total_pages: usize,
    pub previous_href: Option<&'static str>,
    pub next_href: Option<&'static str>,
}

impl WorkspacePagination {
    pub const fn new(
        current_page: usize,
        total_pages: usize,
        previous_href: Option<&'static str>,
        next_href: Option<&'static str>,
    ) -> Self {
        Self {
            current_page,
            total_pages,
            previous_href,
            next_href,
        }
    }
}

fn render_pagination(pagination: WorkspacePagination) -> AnyView {
    match (pagination.previous_href, pagination.next_href) {
        (Some(previous_href), Some(next_href)) => view! {
            <new_alphabet_components::Pagination
                current_page=pagination.current_page
                total_pages=pagination.total_pages
                previous_href=previous_href
                next_href=next_href
            />
        }
        .into_any(),
        (Some(previous_href), None) => view! {
            <new_alphabet_components::Pagination
                current_page=pagination.current_page
                total_pages=pagination.total_pages
                previous_href=previous_href
            />
        }
        .into_any(),
        (None, Some(next_href)) => view! {
            <new_alphabet_components::Pagination
                current_page=pagination.current_page
                total_pages=pagination.total_pages
                next_href=next_href
            />
        }
        .into_any(),
        (None, None) => view! {
            <new_alphabet_components::Pagination
                current_page=pagination.current_page
                total_pages=pagination.total_pages
            />
        }
        .into_any(),
    }
}

fn render_detail(detail: WorkspaceDetail) -> AnyView {
    match (detail.summary, detail.unavailable_message) {
        (Some(summary), Some(unavailable_message)) => view! {
            <DetailPane
                title=detail.title
                summary=summary
                fields=detail.fields
                state=detail.state
                unavailable_message=unavailable_message
            />
        }
        .into_any(),
        (Some(summary), None) => view! {
            <DetailPane
                title=detail.title
                summary=summary
                fields=detail.fields
                state=detail.state
            />
        }
        .into_any(),
        (None, Some(unavailable_message)) => view! {
            <DetailPane
                title=detail.title
                fields=detail.fields
                state=detail.state
                unavailable_message=unavailable_message
            />
        }
        .into_any(),
        (None, None) => view! {
            <DetailPane
                title=detail.title
                fields=detail.fields
                state=detail.state
            />
        }
        .into_any(),
    }
}

#[component]
pub fn SearchResultsWorkspace(
    title: &'static str,
    query: &'static str,
    filters: &'static [FilterGroup],
    results_columns: &'static [TableColumn],
    results_rows: &'static [TableRow],
    #[prop(optional)] filter_state: Option<FilterRailState>,
    #[prop(optional)] results_state: Option<TableState>,
    #[prop(optional)] status: Option<WorkspaceStatus>,
    #[prop(optional)] commands: Option<WorkspaceCommands>,
    #[prop(optional)] detail: Option<WorkspaceDetail>,
    #[prop(optional)] pagination: Option<WorkspacePagination>,
) -> impl IntoView {
    let filter_state = filter_state.unwrap_or_default();
    let results_state = results_state.unwrap_or_default();
    let show_action_band = status.is_some() || commands.is_some();

    view! {
        <AppShell density=DensityMode::Dense intent=FrameIntent::Workspace>
            <Band strength=SurfaceStrength::Strong>
                <SectionHeader
                    title=title
                    subtitle="Search, filtering, results, and contextual inspection remain ordered by structure rather than dashboard ornament."
                    annotation="Workspace"
                />
            </Band>
            <PageGrid intent=FrameIntent::Workspace>
                {if show_action_band {
                    view! {
                        <Region kind=RegionClass::ActionBand placement=RegionPlacement::ActionBand>
                            <Stack spacing=StackSpace::Tight>
                                {status.map(|status| {
                                    view! {
                                        <InlineAlert
                                            title=status.title
                                            message=status.message
                                            severity=status.severity
                                        />
                                    }
                                    .into_any()
                                })}
                                {commands.map(|commands| {
                                    view! {
                                        <CommandBar
                                            label=commands.label
                                            primary=commands.primary
                                            secondary=commands.secondary
                                        />
                                    }
                                    .into_any()
                                })}
                            </Stack>
                        </Region>
                    }
                    .into_any()
                } else {
                    view! { <></> }.into_any()
                }}
                <Rail side=RailSide::Start>
                    <Stack spacing=StackSpace::Default>
                        <Panel strength=SurfaceStrength::Strong>
                            <SectionHeader
                                title="Search"
                                subtitle="Query and filter structure remain bounded in the rail."
                            />
                            <Stack spacing=StackSpace::Tight>
                                <TextField
                                    label="Search query"
                                    name="search-query"
                                    value=query
                                    state=FieldState::Default
                                />
                                <FilterRail
                                    label="Filters"
                                    groups=filters
                                    state=filter_state
                                />
                            </Stack>
                        </Panel>
                    </Stack>
                </Rail>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Results"
                            subtitle="Dense result sets stay explicit about loading, empty, and pagination behavior."
                        />
                        <Stack spacing=StackSpace::Tight>
                            <Table
                                label="Search results"
                                columns=results_columns
                                rows=results_rows
                                state=results_state
                            />
                            {pagination.map(render_pagination)}
                        </Stack>
                    </Panel>
                </Region>
                {detail.map(|detail| {
                    view! {
                        <Region kind=RegionClass::Detail placement=RegionPlacement::Detail>
                            {render_detail(detail)}
                        </Region>
                    }
                    .into_any()
                })}
            </PageGrid>
        </AppShell>
    }
}

const FILTER_TYPE_OPTIONS: &[new_alphabet_components::FilterOption] = &[
    new_alphabet_components::FilterOption::selected("essay", "Essay", 12),
    new_alphabet_components::FilterOption::new("note", "Note", 4),
];

const FILTER_STATE_OPTIONS: &[new_alphabet_components::FilterOption] = &[
    new_alphabet_components::FilterOption::selected("ready", "Ready", 8),
    new_alphabet_components::FilterOption::new("hold", "Hold", 3),
];

const SEARCH_FILTERS: &[FilterGroup] = &[
    FilterGroup::new("Type", FILTER_TYPE_OPTIONS),
    FilterGroup::new("State", FILTER_STATE_OPTIONS),
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
            "Image rights note is incomplete and the caption needs tightening before release.",
        ],
    ),
];

const SEARCH_COMMANDS: WorkspaceCommands = WorkspaceCommands::new(
    "Results commands",
    CommandAction::ready("Open selected entry", "/search/open"),
    &[
        CommandAction::ready("Export results", "/search/export"),
        CommandAction::ready("Save filter set", "/search/save"),
    ],
);

const SEARCH_STATUS: WorkspaceStatus = WorkspaceStatus::new(
    "Results updated",
    "The search index refreshed 18 seconds ago.",
    StatusSeverity::Info,
);

const SEARCH_DETAIL_FIELDS: &[DetailField] = &[
    DetailField::new("State", "Ready"),
    DetailField::new("Owner", "Editorial"),
    DetailField::new("Section", "Archive"),
];

const SEARCH_DETAIL: WorkspaceDetail = WorkspaceDetail::new(
    "Essay 142",
    Some("Context remains adjacent so the result field stays dense and calm."),
    SEARCH_DETAIL_FIELDS,
    DetailPaneState::Default,
    None,
);

const SEARCH_PAGINATION: WorkspacePagination =
    WorkspacePagination::new(1, 3, None, Some("/search?page=2"));

const UNAVAILABLE_DETAIL: WorkspaceDetail = WorkspaceDetail::new(
    "No result selected",
    Some("The detail pane remains present but explicit when nothing can be inspected."),
    &[],
    DetailPaneState::Unavailable,
    Some("No matching result is available to inspect."),
);

#[component]
pub fn SearchResultsWorkspaceExample() -> impl IntoView {
    view! {
        <SearchResultsWorkspace
            title="Search Results Workspace"
            query="archive note"
            filters=SEARCH_FILTERS
            results_columns=SEARCH_COLUMNS
            results_rows=SEARCH_ROWS
            status=SEARCH_STATUS
            commands=SEARCH_COMMANDS
            detail=SEARCH_DETAIL
            pagination=SEARCH_PAGINATION
        />
    }
}

#[component]
pub fn SearchResultsWorkspaceLoadingExample() -> impl IntoView {
    view! {
        <SearchResultsWorkspace
            title="Search Results Workspace"
            query="archive note"
            filters=SEARCH_FILTERS
            results_columns=SEARCH_COLUMNS
            results_rows=SEARCH_ROWS
            results_state=TableState::Loading
            detail=WorkspaceDetail::new(
                "Loading detail",
                Some("Detail loading stays explicit and cancellable."),
                SEARCH_DETAIL_FIELDS,
                DetailPaneState::Loading,
                None,
            )
        />
    }
}

#[component]
pub fn SearchResultsWorkspaceZeroResultsExample() -> impl IntoView {
    view! {
        <SearchResultsWorkspace
            title="Search Results Workspace"
            query="missing reference"
            filters=SEARCH_FILTERS
            filter_state=FilterRailState::ZeroResults
            results_columns=SEARCH_COLUMNS
            results_rows=&[]
            results_state=TableState::Empty
            detail=UNAVAILABLE_DETAIL
        />
    }
}
