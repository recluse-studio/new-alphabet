use leptos::prelude::*;
use new_alphabet_foundation::{DensityMode, RailWidthToken, RegionClass};
use new_alphabet_primitives::{
    AppShell, FrameIntent, PageGrid, Panel, Rail, RailSide, Region, RegionPlacement, Row, RowAlign,
    RowDistribution, SectionHeader, Stack, StackSpace, SurfaceStrength,
};

use crate::{
    ActionPriority, ActionState, Button, Checkbox, ChoiceOption, CommandAction, CommandBar,
    DetailField, DetailPane, EmptyState, FieldState, FilterGroup, FilterOption, FilterRail,
    InlineAlert, LinkAction, MetricBlock, NavIndex, NavIndexItem, Pagination, RadioGroup, Select,
    StatusBadge, StatusSeverity, Switch, Table, TableColumn, TableRow, TextField, Textarea,
};

const DENSITY_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("calm", "Calm"),
    ChoiceOption::new("regular", "Regular"),
    ChoiceOption::new("dense", "Dense"),
];

const REVIEW_DECISION_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("approve", "Approve"),
    ChoiceOption::new("hold", "Hold"),
    ChoiceOption::new("reject", "Reject"),
];

const DASHBOARD_METRIC_ROWS: &[TableRow] = &[
    TableRow::new(
        "review-queue",
        &[
            "Review queue",
            "18",
            "Three items exceed service rhythm and need action today.",
        ],
    ),
    TableRow::new(
        "archive-notes",
        &[
            "Archive notes",
            "42",
            "All pending archive updates fit inside the current publication window.",
        ],
    ),
];

const DASHBOARD_TABLE_COLUMNS: &[TableColumn] = &[
    TableColumn::truncate("surface", "Surface"),
    TableColumn::truncate("count", "Count"),
    TableColumn::wrap("summary", "Summary"),
];

const REVIEW_TABLE_ROWS: &[TableRow] = &[
    TableRow::new(
        "essay-142",
        &[
            "Essay 142",
            "Ready",
            "Lead is clear, but the archive citation still needs a source line.",
        ],
    ),
    TableRow::new(
        "essay-143",
        &[
            "Essay 143",
            "Hold",
            "Image rights note is incomplete and the caption should be tightened before approval.",
        ],
    ),
];

const REVIEW_TABLE_COLUMNS: &[TableColumn] = &[
    TableColumn::truncate("entry", "Entry"),
    TableColumn::truncate("state", "State"),
    TableColumn::wrap("note", "Review note"),
];

const SEARCH_NAV_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Results", "/search"),
    NavIndexItem::new("Review queue", "/review"),
    NavIndexItem::new("Settings", "/settings"),
];

const DOCS_NAV_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Foundations", "/docs/foundations"),
    NavIndexItem::new("Primitives", "/docs/primitives"),
    NavIndexItem::new("Components", "/docs/components"),
];

const FILTER_TYPE_OPTIONS: &[FilterOption] = &[
    FilterOption::selected("essay", "Essay", 12),
    FilterOption::new("note", "Note", 4),
];

const FILTER_STATE_OPTIONS: &[FilterOption] = &[
    FilterOption::selected("ready", "Ready", 8),
    FilterOption::new("hold", "Hold", 3),
];

const SEARCH_FILTER_GROUPS: &[FilterGroup] = &[
    FilterGroup::new("Type", FILTER_TYPE_OPTIONS),
    FilterGroup::new("State", FILTER_STATE_OPTIONS),
];

const REVIEW_COMMAND_PRIMARY: CommandAction =
    CommandAction::ready("Approve selection", "/review/approve");

const REVIEW_COMMAND_SECONDARY: &[CommandAction] = &[
    CommandAction::ready("Open history", "/review/history"),
    CommandAction::loading("Sync notes", "/review/sync"),
];

const REVIEW_DETAIL_FIELDS: &[DetailField] = &[
    DetailField::new("State", "Ready"),
    DetailField::new("Section", "Archive"),
    DetailField::new("Owner", "Editorial"),
];

#[component]
pub fn EditorialActionExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Calm intent=FrameIntent::Editorial>
            <PageGrid intent=FrameIntent::Editorial>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel strength=SurfaceStrength::Strong>
                        <SectionHeader
                            title="Notes"
                            subtitle="Editorial actions stay typographic and quiet."
                        />
                        <Stack spacing=StackSpace::Default>
                            <Button
                                label="Start reading"
                                priority=ActionPriority::Primary
                            />
                            <LinkAction
                                label="Join the list"
                                href="/subscribe"
                                priority=ActionPriority::Secondary
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn WorkflowActionExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Submission Review"
                            subtitle="Primary and secondary actions are separated by structure, not palette sprawl."
                        />
                        <Row align=RowAlign::Center distribution=RowDistribution::Between>
                            <Button
                                label="Approve submission"
                                priority=ActionPriority::Primary
                                state=ActionState::Loading
                            />
                            <LinkAction
                                label="Open history"
                                href="/history"
                                priority=ActionPriority::Secondary
                            />
                        </Row>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn SettingsFieldExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Profile Settings"
                            subtitle="Fields keep labels, help, and validation relationships explicit."
                        />
                        <Stack spacing=StackSpace::Default>
                            <TextField
                                label="Display name"
                                name="display-name"
                                value="Recluse Studio"
                                help="Public profile label."
                                state=FieldState::Success
                                message="Saved."
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn FormFieldExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Submission Form"
                            subtitle="Multi-line entry remains calm and structurally explicit."
                        />
                        <Stack spacing=StackSpace::Default>
                            <Textarea
                                label="Submission notes"
                                name="submission-notes"
                                value="Structured feedback."
                                help="Internal review context."
                                state=FieldState::Success
                                message="Saved."
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn SettingsChoiceExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Preferences"
                            subtitle="Named choice controls stay finite and explicit."
                        />
                        <Stack spacing=StackSpace::Default>
                            <Select
                                label="Workspace density"
                                name="workspace-density"
                                selected="regular"
                                options=DENSITY_OPTIONS
                                help="Applies to queue and detail surfaces."
                            />
                            <Switch
                                label="Private mode"
                                name="private-mode"
                                checked=true
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn FormChoiceExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Review Options"
                            subtitle="Choice controls only appear where the options are finite and named."
                        />
                        <Stack spacing=StackSpace::Default>
                            <RadioGroup
                                label="Review decision"
                                name="review-decision"
                                selected="approve"
                                options=REVIEW_DECISION_OPTIONS
                            />
                            <Checkbox
                                label="Attach follow-up"
                                name="attach-follow-up"
                                checked=true
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn EditorialStatusExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Calm intent=FrameIntent::Editorial>
            <PageGrid intent=FrameIntent::Editorial>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel strength=SurfaceStrength::Strong>
                        <SectionHeader
                            title="Archive note"
                            subtitle="Status remains textual and structural rather than decorative."
                        />
                        <Stack spacing=StackSpace::Default>
                            <StatusBadge
                                label="Published"
                                severity=StatusSeverity::Success
                            />
                            <InlineAlert
                                title="Archive note"
                                message="This issue is now part of the permanent index."
                                severity=StatusSeverity::Info
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn WorkflowStatusExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Queue feedback"
                            subtitle="Workflow status and absence states remain explicit."
                        />
                        <Stack spacing=StackSpace::Default>
                            <InlineAlert
                                title="Sync delayed"
                                message="The queue will retry in one minute."
                                severity=StatusSeverity::Warning
                            />
                            <EmptyState
                                title="No matching entries"
                                message="Adjust the filters to broaden the queue."
                                next_action="Clear filters"
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn DashboardDataExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Dense intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Stack spacing=StackSpace::Default>
                        <Panel>
                            <SectionHeader
                                title="Operations Snapshot"
                                subtitle="Metrics stay structural and typographic in dense workflow contexts."
                            />
                            <Row align=RowAlign::Baseline distribution=RowDistribution::Start>
                                <MetricBlock
                                    label="Published today"
                                    value="18"
                                    context="Across archive and review surfaces."
                                />
                                <MetricBlock
                                    label="Average queue age"
                                    value="4h"
                                    context="Measured from the first editorial touch."
                                />
                                <MetricBlock
                                    label="Blocked items"
                                    value="3"
                                    note="Rights and metadata gaps only."
                                />
                            </Row>
                        </Panel>
                        <Panel>
                            <SectionHeader
                                title="Queue summary"
                                subtitle="Dense tables define truncation and wrapping directly in the column contract."
                            />
                            <Stack spacing=StackSpace::Tight>
                                <Table
                                    label="Dashboard queue summary"
                                    columns=DASHBOARD_TABLE_COLUMNS
                                    rows=DASHBOARD_METRIC_ROWS
                                />
                                <Pagination
                                    current_page=1
                                    total_pages=4
                                    next_href="/dashboard?page=2"
                                />
                            </Stack>
                        </Panel>
                    </Stack>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn ReviewDataExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Dense intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Review queue"
                            subtitle="Workspace tables keep queue state, wrapping, and paging explicit."
                        />
                        <Stack spacing=StackSpace::Tight>
                            <Table
                                label="Review queue entries"
                                columns=REVIEW_TABLE_COLUMNS
                                rows=REVIEW_TABLE_ROWS
                            />
                            <Pagination
                                current_page=2
                                total_pages=6
                                previous_href="/review?page=1"
                                next_href="/review?page=3"
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn SearchWorkflowExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Dense intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Rail width=RailWidthToken::Default side=RailSide::Start>
                    <Stack spacing=StackSpace::Default>
                        <NavIndex label="Workspace sections" items=SEARCH_NAV_ITEMS />
                        <FilterRail label="Search filters" groups=SEARCH_FILTER_GROUPS />
                    </Stack>
                </Rail>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Search results"
                            subtitle="Navigation and filter structures remain quiet while the result field stays dense."
                        />
                        <Stack spacing=StackSpace::Tight>
                            <Table
                                label="Search result entries"
                                columns=REVIEW_TABLE_COLUMNS
                                rows=REVIEW_TABLE_ROWS
                            />
                            <Pagination
                                current_page=1
                                total_pages=3
                                next_href="/search?page=2"
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn ReviewQueueCommandExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Dense intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Review queue"
                            subtitle="Action hierarchy is explicit and separated from the dense queue surface."
                        />
                        <Stack spacing=StackSpace::Tight>
                            <CommandBar
                                label="Review commands"
                                primary=REVIEW_COMMAND_PRIMARY
                                secondary=REVIEW_COMMAND_SECONDARY
                            />
                            <Table
                                label="Review queue entries"
                                columns=REVIEW_TABLE_COLUMNS
                                rows=REVIEW_TABLE_ROWS
                            />
                        </Stack>
                    </Panel>
                </Region>
                <Region kind=RegionClass::Detail placement=RegionPlacement::Detail>
                    <DetailPane
                        title="Essay 142"
                        summary="Inspection stays adjacent, bounded, and explicit."
                        fields=REVIEW_DETAIL_FIELDS
                    />
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn DocumentationNavigationExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Calm intent=FrameIntent::Editorial>
            <PageGrid intent=FrameIntent::Editorial>
                <Region kind=RegionClass::Support placement=RegionPlacement::Support>
                    <Panel strength=SurfaceStrength::Strong>
                        <SectionHeader
                            title="Manual"
                            subtitle="Documentation navigation stays structural and typographic."
                        />
                        <NavIndex label="Documentation sections" items=DOCS_NAV_ITEMS />
                    </Panel>
                </Region>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel strength=SurfaceStrength::Strong>
                        <SectionHeader
                            title="Foundations"
                            subtitle="The docs surface shares the same grammar as workflow navigation."
                        />
                        <p>"Foundations define layout, spacing, type, density, color, border, motion, and state law."</p>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}
