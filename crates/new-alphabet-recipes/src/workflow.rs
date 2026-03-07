use leptos::prelude::*;
use new_alphabet_components::{
    CommandAction, CommandBar, DetailField, DetailPane, DetailPaneState, FieldState, FilterGroup,
    FilterRail, FilterRailState, InlineAlert, NavIndex, NavIndexItem, StatusSeverity, Table,
    TableColumn, TableRow, TableState, TextField,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkspaceNavSection {
    pub title: &'static str,
    pub items: &'static [NavIndexItem],
}

impl WorkspaceNavSection {
    pub const fn new(title: &'static str, items: &'static [NavIndexItem]) -> Self {
        Self { title, items }
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

#[component]
pub fn ReviewQueue(
    title: &'static str,
    queue_columns: &'static [TableColumn],
    queue_rows: &'static [TableRow],
    actions: WorkspaceCommands,
    detail: WorkspaceDetail,
    #[prop(optional)] queue_state: Option<TableState>,
    #[prop(optional)] status: Option<WorkspaceStatus>,
    #[prop(optional)] navigation: Option<WorkspaceNavSection>,
    #[prop(optional)] filters: Option<&'static [FilterGroup]>,
    #[prop(optional)] filter_state: Option<FilterRailState>,
) -> impl IntoView {
    let queue_state = queue_state.unwrap_or_default();
    let filter_state = filter_state.unwrap_or_default();
    let show_rail = navigation.is_some() || filters.is_some();

    view! {
        <AppShell density=DensityMode::Dense intent=FrameIntent::Workspace>
            <Band strength=SurfaceStrength::Strong>
                <SectionHeader
                    title=title
                    subtitle="Queue, decision, and inspection remain ordered by named regions rather than local dashboard invention."
                    annotation="Review"
                />
            </Band>
            <PageGrid intent=FrameIntent::Workspace>
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
                        <CommandBar
                            label=actions.label
                            primary=actions.primary
                            secondary=actions.secondary
                        />
                    </Stack>
                </Region>
                {if show_rail {
                    view! {
                        <Rail side=RailSide::Start>
                            <Stack spacing=StackSpace::Default>
                                {navigation.map(|navigation| {
                                    view! {
                                        <Panel strength=SurfaceStrength::Strong>
                                            <SectionHeader
                                                title=navigation.title
                                                subtitle="Queue navigation remains bounded in the rail."
                                            />
                                            <NavIndex label=navigation.title items=navigation.items />
                                        </Panel>
                                    }
                                    .into_any()
                                })}
                                {filters.map(|filters| {
                                    view! {
                                        <Panel>
                                            <SectionHeader
                                                title="Filters"
                                                subtitle="Queue filters stay secondary to the main decision field."
                                            />
                                            <FilterRail
                                                label="Review filters"
                                                groups=filters
                                                state=filter_state
                                            />
                                        </Panel>
                                    }
                                    .into_any()
                                })}
                            </Stack>
                        </Rail>
                    }
                    .into_any()
                } else {
                    view! { <></> }.into_any()
                }}
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Queue"
                            subtitle="The queue field stays dense, legible, and explicit about empty and loading states."
                        />
                        <Table
                            label="Review queue"
                            columns=queue_columns
                            rows=queue_rows
                            state=queue_state
                        />
                    </Panel>
                </Region>
                <Region kind=RegionClass::Detail placement=RegionPlacement::Detail>
                    {render_detail(detail)}
                </Region>
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

const REVIEW_NAV_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Assigned", "/review/assigned"),
    NavIndexItem::new("All", "/review/all"),
];

const REVIEW_NAVIGATION: WorkspaceNavSection = WorkspaceNavSection::new("Queues", REVIEW_NAV_ITEMS);

const REVIEW_COLUMNS: &[TableColumn] = &[
    TableColumn::truncate("entry", "Entry"),
    TableColumn::truncate("state", "State"),
    TableColumn::wrap("note", "Review note"),
];

const REVIEW_ROWS: &[TableRow] = &[
    TableRow::new(
        "essay-142",
        &[
            "Essay 142",
            "Ready",
            "Archive citation is resolved and the lead paragraph reads cleanly.",
        ],
    ),
    TableRow::new(
        "essay-143",
        &[
            "Essay 143",
            "Hold",
            "Rights note is incomplete and the caption still needs tightening.",
        ],
    ),
];

const REVIEW_ACTIONS: WorkspaceCommands = WorkspaceCommands::new(
    "Queue commands",
    CommandAction::ready("Approve selected", "/review/approve"),
    &[
        CommandAction::ready("Open history", "/review/history"),
        CommandAction::ready("Request revision", "/review/request-revision"),
    ],
);

const REVIEW_STATUS: WorkspaceStatus = WorkspaceStatus::new(
    "Selection updated",
    "The queue is synced and ready for the next review pass.",
    StatusSeverity::Success,
);

const REVIEW_DETAIL_FIELDS: &[DetailField] = &[
    DetailField::new("State", "Ready"),
    DetailField::new("Owner", "Editorial"),
    DetailField::new("Section", "Archive"),
];

const REVIEW_DETAIL: WorkspaceDetail = WorkspaceDetail::new(
    "Essay 142",
    Some("Selection stays adjacent so review decisions do not interrupt the queue rhythm."),
    REVIEW_DETAIL_FIELDS,
    DetailPaneState::Default,
    None,
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
pub fn ReviewQueueExample() -> impl IntoView {
    view! {
        <ReviewQueue
            title="Review Queue"
            queue_columns=REVIEW_COLUMNS
            queue_rows=REVIEW_ROWS
            actions=REVIEW_ACTIONS
            status=REVIEW_STATUS
            navigation=REVIEW_NAVIGATION
            filters=SEARCH_FILTERS
            detail=REVIEW_DETAIL
        />
    }
}

#[component]
pub fn ReviewQueueLoadingExample() -> impl IntoView {
    view! {
        <ReviewQueue
            title="Review Queue"
            queue_columns=REVIEW_COLUMNS
            queue_rows=REVIEW_ROWS
            queue_state=TableState::Loading
            actions=REVIEW_ACTIONS
            detail=WorkspaceDetail::new(
                "Loading detail",
                Some("Detail loading remains explicit while the queue stays visible."),
                REVIEW_DETAIL_FIELDS
                ,
                DetailPaneState::Loading,
                None,
            )
        />
    }
}

#[component]
pub fn ReviewQueueEmptyExample() -> impl IntoView {
    view! {
        <ReviewQueue
            title="Review Queue"
            queue_columns=REVIEW_COLUMNS
            queue_rows=&[]
            queue_state=TableState::Empty
            actions=REVIEW_ACTIONS
            detail=UNAVAILABLE_DETAIL
        />
    }
}

#[component]
pub fn ReviewQueueUnavailableDetailExample() -> impl IntoView {
    view! {
        <ReviewQueue
            title="Review Queue"
            queue_columns=REVIEW_COLUMNS
            queue_rows=REVIEW_ROWS
            actions=REVIEW_ACTIONS
            detail=WorkspaceDetail::new(
                "Selection unavailable",
                Some("The queue remains usable even when the selected detail is unavailable."),
                &[],
                DetailPaneState::Unavailable,
                Some("The selected entry cannot be inspected because the archive is still syncing."),
            )
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
