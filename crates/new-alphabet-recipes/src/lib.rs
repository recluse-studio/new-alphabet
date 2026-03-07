#![forbid(unsafe_code)]

mod editorial;
mod workflow;

pub use editorial::{
    ArticleAdjacentLink, ArticleAdjacentLinks, ArticleMetaItem, ArticleSection, ArticleShell,
    ArticleShellExample, ArticleShellMinimalExample, ArticleSidebarNav, BlogIndex, BlogIndexEntry,
    BlogIndexExample, BlogIndexMinimalExample, BlogIndexSection, DocsContextItem, DocsNavSection,
    DocsShell, DocsShellExample, DocsShellMinimalExample,
};
pub use workflow::{
    DashboardMetric, DashboardShell, DashboardShellExample, ReviewQueue, ReviewQueueEmptyExample,
    ReviewQueueExample, ReviewQueueLoadingExample, ReviewQueueUnavailableDetailExample,
    SearchResultsWorkspace, SearchResultsWorkspaceExample, SearchResultsWorkspaceLoadingExample,
    SearchResultsWorkspaceZeroResultsExample, SettingsControl, SettingsPanel, SettingsWorkspace,
    SettingsWorkspaceExample, WorkspaceCommands, WorkspaceContextItem, WorkspaceDetail,
    WorkspaceNavSection, WorkspacePagination, WorkspaceStatus,
};

#[cfg(test)]
const ENTRY_ONE: &[BlogIndexEntry] = &[BlogIndexEntry::new(
    "Grid Notes for Quiet Systems",
    "/notes/grid-notes",
    "A working note on why archive structure should carry the surface before brand language does.",
    "Essay 14 · March 2026",
)];

#[cfg(test)]
const TAXONOMY_ITEMS: &[new_alphabet_components::NavIndexItem] = &[
    new_alphabet_components::NavIndexItem::current("Essays", "/topics/essays"),
    new_alphabet_components::NavIndexItem::new("Notes", "/topics/notes"),
];

#[cfg(test)]
const ARCHIVE_ITEMS: &[new_alphabet_components::NavIndexItem] = &[
    new_alphabet_components::NavIndexItem::current("2026", "/archive/2026"),
    new_alphabet_components::NavIndexItem::new("2025", "/archive/2025"),
];

#[cfg(test)]
const TAXONOMY_SECTION: BlogIndexSection = BlogIndexSection::new("Taxonomy", TAXONOMY_ITEMS);

#[cfg(test)]
const ARCHIVE_SECTION: BlogIndexSection = BlogIndexSection::new("Archive", ARCHIVE_ITEMS);

#[cfg(test)]
const ARTICLE_PARAGRAPHS_ONE: &[&str] = &[
    "New Alphabet treats article structure as a reading device first and a navigation problem second.",
];

#[cfg(test)]
const ARTICLE_PARAGRAPHS_TWO: &[&str] =
    &["Support metadata belongs beside the reading flow, not inside it."];

#[cfg(test)]
const ARTICLE_SECTIONS: &[ArticleSection] = &[
    ArticleSection::new("premise", "Premise", ARTICLE_PARAGRAPHS_ONE),
    ArticleSection::new("adjacency", "Adjacency", ARTICLE_PARAGRAPHS_TWO),
];

#[cfg(test)]
const ARTICLE_NAV_ITEMS: &[new_alphabet_components::NavIndexItem] = &[
    new_alphabet_components::NavIndexItem::current("Premise", "#premise"),
    new_alphabet_components::NavIndexItem::new("Adjacency", "#adjacency"),
];

#[cfg(test)]
const ARTICLE_LOCAL_NAV: ArticleSidebarNav = ArticleSidebarNav::new("Contents", ARTICLE_NAV_ITEMS);

#[cfg(test)]
const ARTICLE_METADATA: &[ArticleMetaItem] = &[
    ArticleMetaItem::new("Author", "Recluse Studio"),
    ArticleMetaItem::new("Published", "March 2026"),
];

#[cfg(test)]
const ARTICLE_ADJACENT: ArticleAdjacentLinks = ArticleAdjacentLinks::new(
    Some(ArticleAdjacentLink::new(
        "Previous: Grid Notes",
        "/notes/grid-notes",
    )),
    Some(ArticleAdjacentLink::new(
        "Next: Release Practice",
        "/notes/release-practice",
    )),
);

#[cfg(test)]
const MINIMAL_ARTICLE_PARAGRAPHS: &[&str] =
    &["A narrow article shell can remain legible without side structures."];

#[cfg(test)]
const MINIMAL_ARTICLE_SECTIONS: &[ArticleSection] = &[ArticleSection::new(
    "opening",
    "Opening",
    MINIMAL_ARTICLE_PARAGRAPHS,
)];

#[cfg(test)]
const DOCS_NAV_ITEMS: &[new_alphabet_components::NavIndexItem] = &[
    new_alphabet_components::NavIndexItem::current("Foundations", "/docs/foundations"),
    new_alphabet_components::NavIndexItem::new("Primitives", "/docs/primitives"),
];

#[cfg(test)]
const DOCS_NAVIGATION: DocsNavSection = DocsNavSection::new("Documentation", DOCS_NAV_ITEMS);

#[cfg(test)]
const DOCS_TOC_ITEMS: &[new_alphabet_components::NavIndexItem] = &[
    new_alphabet_components::NavIndexItem::current("Layer model", "#layer-model"),
    new_alphabet_components::NavIndexItem::new("State law", "#state-law"),
];

#[cfg(test)]
const DOCS_TOC: DocsNavSection = DocsNavSection::new("On this page", DOCS_TOC_ITEMS);

#[cfg(test)]
const DOCS_CONTEXT: &[DocsContextItem] = &[
    DocsContextItem::new("Package", "new-alphabet-recipes"),
    DocsContextItem::new("Surface", "DocsShell"),
];

#[cfg(test)]
const DOCS_SECTIONS: &[ArticleSection] = &[
    ArticleSection::new(
        "layer-model",
        "Layer model",
        &["DocsShell keeps navigation and reading flow in the same grammar."],
    ),
    ArticleSection::new(
        "state-law",
        "State law",
        &["Supporting context belongs in an adjacent detail region."],
    ),
];

#[cfg(test)]
const MINIMAL_DOCS_SECTIONS: &[ArticleSection] = &[ArticleSection::new(
    "foundations",
    "Foundations",
    &["A narrow docs surface can run as rail plus reading column."],
)];

#[cfg(test)]
const SEARCH_FILTER_TYPE_OPTIONS: &[new_alphabet_components::FilterOption] = &[
    new_alphabet_components::FilterOption::selected("essay", "Essay", 12),
    new_alphabet_components::FilterOption::new("note", "Note", 4),
];

#[cfg(test)]
const SEARCH_FILTER_STATE_OPTIONS: &[new_alphabet_components::FilterOption] = &[
    new_alphabet_components::FilterOption::selected("ready", "Ready", 8),
    new_alphabet_components::FilterOption::new("hold", "Hold", 3),
];

#[cfg(test)]
const SEARCH_FILTERS: &[new_alphabet_components::FilterGroup] = &[
    new_alphabet_components::FilterGroup::new("Type", SEARCH_FILTER_TYPE_OPTIONS),
    new_alphabet_components::FilterGroup::new("State", SEARCH_FILTER_STATE_OPTIONS),
];

#[cfg(test)]
const SEARCH_COLUMNS: &[new_alphabet_components::TableColumn] = &[
    new_alphabet_components::TableColumn::truncate("entry", "Entry"),
    new_alphabet_components::TableColumn::truncate("state", "State"),
    new_alphabet_components::TableColumn::wrap("summary", "Summary"),
];

#[cfg(test)]
const SEARCH_ROWS: &[new_alphabet_components::TableRow] = &[
    new_alphabet_components::TableRow::new(
        "essay-142",
        &[
            "Essay 142",
            "Ready",
            "Lead is clear and the archive note now links to the correct citation.",
        ],
    ),
    new_alphabet_components::TableRow::new(
        "essay-143",
        &[
            "Essay 143",
            "Hold",
            "Image rights note is incomplete and the caption needs tightening before release.",
        ],
    ),
];

#[cfg(test)]
const SEARCH_DETAIL_FIELDS: &[new_alphabet_components::DetailField] = &[
    new_alphabet_components::DetailField::new("State", "Ready"),
    new_alphabet_components::DetailField::new("Owner", "Editorial"),
    new_alphabet_components::DetailField::new("Section", "Archive"),
];

#[cfg(test)]
const SEARCH_COMMAND_SECONDARY: &[new_alphabet_components::CommandAction] =
    &[new_alphabet_components::CommandAction::ready(
        "Export results",
        "/search/export",
    )];

#[cfg(test)]
const REVIEW_NAV_ITEMS: &[new_alphabet_components::NavIndexItem] = &[
    new_alphabet_components::NavIndexItem::current("Assigned", "/review/assigned"),
    new_alphabet_components::NavIndexItem::new("All", "/review/all"),
];

#[cfg(test)]
const REVIEW_NAVIGATION: WorkspaceNavSection = WorkspaceNavSection::new("Queues", REVIEW_NAV_ITEMS);

#[cfg(test)]
const REVIEW_COLUMNS: &[new_alphabet_components::TableColumn] = &[
    new_alphabet_components::TableColumn::truncate("entry", "Entry"),
    new_alphabet_components::TableColumn::truncate("state", "State"),
    new_alphabet_components::TableColumn::wrap("note", "Review note"),
];

#[cfg(test)]
const REVIEW_ROWS: &[new_alphabet_components::TableRow] = &[
    new_alphabet_components::TableRow::new(
        "essay-142",
        &[
            "Essay 142",
            "Ready",
            "Archive citation is resolved and the lead paragraph reads cleanly.",
        ],
    ),
    new_alphabet_components::TableRow::new(
        "essay-143",
        &[
            "Essay 143",
            "Hold",
            "Rights note is incomplete and the caption still needs tightening.",
        ],
    ),
];

#[cfg(test)]
const REVIEW_ACTION_SECONDARY: &[new_alphabet_components::CommandAction] = &[
    new_alphabet_components::CommandAction::ready("Open history", "/review/history"),
    new_alphabet_components::CommandAction::ready("Request revision", "/review/request-revision"),
];

#[cfg(test)]
const REVIEW_DETAIL_FIELDS: &[new_alphabet_components::DetailField] = &[
    new_alphabet_components::DetailField::new("State", "Ready"),
    new_alphabet_components::DetailField::new("Owner", "Editorial"),
    new_alphabet_components::DetailField::new("Section", "Archive"),
];

#[cfg(test)]
const SETTINGS_NAV_ITEMS: &[new_alphabet_components::NavIndexItem] = &[
    new_alphabet_components::NavIndexItem::current("Profile", "/settings/profile"),
    new_alphabet_components::NavIndexItem::new("Workflow", "/settings/workflow"),
];

#[cfg(test)]
const SETTINGS_NAVIGATION: WorkspaceNavSection =
    WorkspaceNavSection::new("Settings sections", SETTINGS_NAV_ITEMS);

#[cfg(test)]
const SETTINGS_OPTIONS: &[new_alphabet_components::ChoiceOption] = &[
    new_alphabet_components::ChoiceOption::new("calm", "Calm"),
    new_alphabet_components::ChoiceOption::new("regular", "Regular"),
];

#[cfg(test)]
const SETTINGS_PROFILE_CONTROLS: &[SettingsControl] = &[
    SettingsControl::text(
        "Display name",
        "display-name",
        "Recluse Studio",
        new_alphabet_components::FieldState::Success,
        Some("Public profile label."),
        Some("Saved."),
    ),
    SettingsControl::select(
        "Workspace density",
        "workspace-density",
        "regular",
        SETTINGS_OPTIONS,
        new_alphabet_components::FieldState::Default,
        Some("Applies to queue and detail surfaces."),
        None,
    ),
];

#[cfg(test)]
const SETTINGS_PANELS: &[SettingsPanel] = &[SettingsPanel::new(
    "Profile settings",
    "Editable public-facing labels stay explicit and bounded.",
    SETTINGS_PROFILE_CONTROLS,
)];

#[cfg(test)]
const WORKSPACE_CONTEXT: &[WorkspaceContextItem] = &[
    WorkspaceContextItem::new("Sync", "Last synced 18 seconds ago"),
    WorkspaceContextItem::new("Scope", "Workspace-only"),
];

#[cfg(test)]
const DASHBOARD_METRICS: &[DashboardMetric] = &[
    DashboardMetric::new(
        "Published today",
        "18",
        Some("Across archive and review surfaces."),
        None,
    ),
    DashboardMetric::new(
        "Blocked items",
        "3",
        None,
        Some("Rights and metadata gaps only."),
    ),
];

#[cfg(test)]
const DASHBOARD_COLUMNS: &[new_alphabet_components::TableColumn] = &[
    new_alphabet_components::TableColumn::truncate("surface", "Surface"),
    new_alphabet_components::TableColumn::truncate("count", "Count"),
    new_alphabet_components::TableColumn::wrap("summary", "Summary"),
];

#[cfg(test)]
const DASHBOARD_ROWS: &[new_alphabet_components::TableRow] =
    &[new_alphabet_components::TableRow::new(
        "review-queue",
        &[
            "Review queue",
            "18",
            "Three items exceed service rhythm and need action today.",
        ],
    )];

#[cfg(test)]
mod tests {
    use leptos::prelude::*;

    use super::*;

    fn render(view: impl FnOnce() -> AnyView + 'static) -> String {
        view().to_html()
    }

    #[test]
    fn blog_index_renders_entries_intro_and_support_regions() {
        let html = render(|| {
            view! {
                <BlogIndex
                    title="Studio Notes"
                    introduction="Essays, release notes, and working fragments arranged as a publication archive."
                    entries=ENTRY_ONE
                    taxonomy=TAXONOMY_SECTION
                    archive=ARCHIVE_SECTION
                />
            }
            .into_any()
        });

        assert!(html.contains("Studio Notes"));
        assert!(html.contains("Grid Notes for Quiet Systems"));
        assert!(html.contains("Taxonomy"));
        assert!(html.contains("Archive"));
        assert!(html.contains("data-region=\"support\""));
    }

    #[test]
    fn blog_index_omits_support_region_when_optional_sections_are_absent() {
        let html = render(|| {
            view! {
                <BlogIndex
                    title="Archive"
                    entries=ENTRY_ONE
                />
            }
            .into_any()
        });

        assert!(html.contains("data-region=\"main\""));
        assert!(!html.contains("data-region=\"support\""));
    }

    #[test]
    fn blog_index_renders_empty_state_for_blank_archive() {
        let html = render(|| {
            view! {
                <BlogIndex
                    title="Archive"
                    entries=&[]
                />
            }
            .into_any()
        });

        assert!(html.contains("No published entries"));
        assert!(html.contains("Publish the first entry"));
    }

    #[test]
    fn blog_index_example_renders_editorial_archive_context() {
        let html = render(|| view! { <BlogIndexExample/> }.into_any());

        assert!(html.contains("Studio Notes"));
        assert!(html.contains("data-intent=\"editorial\""));
        assert!(html.contains("Archive"));
    }

    #[test]
    fn blog_index_minimal_example_renders_single_flow_archive() {
        let html = render(|| view! { <BlogIndexMinimalExample/> }.into_any());

        assert!(html.contains("First note"));
        assert!(!html.contains("data-region=\"support\""));
        assert!(html.contains("data-span-compact=\"4\""));
    }

    #[test]
    fn article_shell_renders_header_body_and_support_regions() {
        let html = render(|| {
            view! {
                <ArticleShell
                    title="Reading Flow as Operating Law"
                    dek="A longform shell should keep the article body primary."
                    sections=ARTICLE_SECTIONS
                    metadata=ARTICLE_METADATA
                    local_navigation=ARTICLE_LOCAL_NAV
                    adjacent=ARTICLE_ADJACENT
                />
            }
            .into_any()
        });

        assert!(html.contains("Reading Flow as Operating Law"));
        assert!(html.contains("Premise"));
        assert!(html.contains("Contents"));
        assert!(html.contains("Metadata"));
        assert!(html.contains("Continue"));
        assert!(html.contains("data-region=\"support\""));
    }

    #[test]
    fn article_shell_omits_support_region_when_optional_sections_are_absent() {
        let html = render(|| {
            view! {
                <ArticleShell
                    title="Opening Note"
                    sections=MINIMAL_ARTICLE_SECTIONS
                />
            }
            .into_any()
        });

        assert!(html.contains("Opening Note"));
        assert!(!html.contains("data-region=\"support\""));
    }

    #[test]
    fn article_shell_example_renders_reading_first_context() {
        let html = render(|| view! { <ArticleShellExample/> }.into_any());

        assert!(html.contains("Reading Flow as Operating Law"));
        assert!(html.contains("data-intent=\"editorial\""));
        assert!(html.contains("Previous: Grid Notes"));
    }

    #[test]
    fn article_shell_minimal_example_renders_single_flow_article() {
        let html = render(|| view! { <ArticleShellMinimalExample/> }.into_any());

        assert!(html.contains("Opening Note"));
        assert!(!html.contains("data-region=\"support\""));
        assert!(html.contains("data-span-compact=\"4\""));
    }

    #[test]
    fn docs_shell_renders_navigation_content_and_detail_regions() {
        let html = render(|| {
            view! {
                <DocsShell
                    title="New Alphabet Manual"
                    navigation=DOCS_NAVIGATION
                    sections=DOCS_SECTIONS
                    table_of_contents=DOCS_TOC
                    context=DOCS_CONTEXT
                />
            }
            .into_any()
        });

        assert!(html.contains("Documentation"));
        assert!(html.contains("Layer model"));
        assert!(html.contains("On this page"));
        assert!(html.contains("Context"));
        assert!(html.contains("data-region=\"detail\""));
    }

    #[test]
    fn docs_shell_omits_detail_region_when_optional_sections_are_absent() {
        let html = render(|| {
            view! {
                <DocsShell
                    title="Foundations"
                    navigation=DOCS_NAVIGATION
                    sections=MINIMAL_DOCS_SECTIONS
                />
            }
            .into_any()
        });

        assert!(html.contains("Foundations"));
        assert!(!html.contains("data-region=\"detail\""));
        assert!(html.contains("data-side=\"start\""));
    }

    #[test]
    fn docs_shell_example_renders_family_resemblance() {
        let html = render(|| view! { <DocsShellExample/> }.into_any());

        assert!(html.contains("New Alphabet Manual"));
        assert!(html.contains("data-intent=\"editorial\""));
        assert!(html.contains("DocsShell"));
    }

    #[test]
    fn docs_shell_minimal_example_renders_two_region_docs_surface() {
        let html = render(|| view! { <DocsShellMinimalExample/> }.into_any());

        assert!(html.contains("Foundations"));
        assert!(!html.contains("data-region=\"detail\""));
        assert!(html.contains("data-side=\"start\""));
    }

    #[test]
    fn search_results_workspace_renders_required_and_optional_regions() {
        let html = render(|| {
            view! {
                <SearchResultsWorkspace
                    title="Search Results Workspace"
                    query="archive note"
                    filters=SEARCH_FILTERS
                    results_columns=SEARCH_COLUMNS
                    results_rows=SEARCH_ROWS
                    status=WorkspaceStatus::new(
                        "Results updated",
                        "The search index refreshed 18 seconds ago.",
                        new_alphabet_components::StatusSeverity::Info,
                    )
                    commands=WorkspaceCommands::new(
                        "Results commands",
                        new_alphabet_components::CommandAction::ready("Open selected entry", "/search/open"),
                        SEARCH_COMMAND_SECONDARY
                    )
                    detail=WorkspaceDetail::new(
                        "Essay 142",
                        Some("Context remains adjacent so the result field stays dense and calm."),
                        SEARCH_DETAIL_FIELDS,
                        new_alphabet_components::DetailPaneState::Default,
                        None,
                    )
                    pagination=WorkspacePagination::new(1, 3, None, Some("/search?page=2"))
                />
            }
            .into_any()
        });

        assert!(html.contains("data-region=\"action_band\""));
        assert!(html.contains("data-region=\"detail\""));
        assert!(html.contains("Search query"));
        assert!(html.contains("Search results"));
    }

    #[test]
    fn search_results_workspace_loading_example_renders_loading_states() {
        let html = render(|| view! { <SearchResultsWorkspaceLoadingExample/> }.into_any());

        assert!(html.contains("Loading rows."));
        assert!(html.contains("Loading detail."));
    }

    #[test]
    fn search_results_workspace_zero_results_example_renders_zero_results_and_unavailable_detail() {
        let html = render(|| view! { <SearchResultsWorkspaceZeroResultsExample/> }.into_any());

        assert!(html.contains("No results match the current filters."));
        assert!(html.contains("No rows available."));
        assert!(html.contains("No matching result is available to inspect."));
    }

    #[test]
    fn review_queue_renders_required_structure_and_optional_rail() {
        let html = render(|| {
            view! {
                <ReviewQueue
                    title="Review Queue"
                    queue_columns=REVIEW_COLUMNS
                    queue_rows=REVIEW_ROWS
                    actions=WorkspaceCommands::new(
                        "Queue commands",
                        new_alphabet_components::CommandAction::ready("Approve selected", "/review/approve"),
                        REVIEW_ACTION_SECONDARY,
                    )
                    status=WorkspaceStatus::new(
                        "Selection updated",
                        "The queue is synced and ready for the next review pass.",
                        new_alphabet_components::StatusSeverity::Success,
                    )
                    navigation=REVIEW_NAVIGATION
                    filters=SEARCH_FILTERS
                    detail=WorkspaceDetail::new(
                        "Essay 142",
                        Some("Selection stays adjacent so review decisions do not interrupt the queue rhythm."),
                        REVIEW_DETAIL_FIELDS,
                        new_alphabet_components::DetailPaneState::Default,
                        None,
                    )
                />
            }
            .into_any()
        });

        assert!(html.contains("data-region=\"action_band\""));
        assert!(html.contains("data-region=\"detail\""));
        assert!(html.contains("Queues"));
        assert!(html.contains("Review queue"));
    }

    #[test]
    fn review_queue_loading_example_renders_loading_state() {
        let html = render(|| view! { <ReviewQueueLoadingExample/> }.into_any());

        assert!(html.contains("Loading rows."));
        assert!(html.contains("Loading detail."));
    }

    #[test]
    fn review_queue_empty_example_renders_empty_state() {
        let html = render(|| view! { <ReviewQueueEmptyExample/> }.into_any());

        assert!(html.contains("No rows available."));
        assert!(html.contains("No matching result is available to inspect."));
    }

    #[test]
    fn review_queue_unavailable_detail_example_renders_unavailable_detail() {
        let html = render(|| view! { <ReviewQueueUnavailableDetailExample/> }.into_any());

        assert!(html.contains(
            "The selected entry cannot be inspected because the archive is still syncing."
        ));
    }

    #[test]
    fn settings_workspace_renders_navigation_panels_and_context() {
        let html = render(|| {
            view! {
                <SettingsWorkspace
                    title="Settings Workspace"
                    navigation=SETTINGS_NAVIGATION
                    panels=SETTINGS_PANELS
                    status=WorkspaceStatus::new(
                        "Settings saved",
                        "Profile changes are now active.",
                        new_alphabet_components::StatusSeverity::Success,
                    )
                    context=WORKSPACE_CONTEXT
                />
            }
            .into_any()
        });

        assert!(html.contains("Settings sections"));
        assert!(html.contains("Profile settings"));
        assert!(html.contains("Last synced 18 seconds ago"));
    }

    #[test]
    fn dashboard_shell_renders_metrics_table_and_context() {
        let html = render(|| {
            view! {
                <DashboardShell
                    title="Operations Dashboard"
                    metrics=DASHBOARD_METRICS
                    summary_columns=DASHBOARD_COLUMNS
                    summary_rows=DASHBOARD_ROWS
                    status=WorkspaceStatus::new(
                        "Overview refreshed",
                        "Metric and queue summaries were updated 18 seconds ago.",
                        new_alphabet_components::StatusSeverity::Info,
                    )
                    context=WORKSPACE_CONTEXT
                />
            }
            .into_any()
        });

        assert!(html.contains("Operations Dashboard"));
        assert!(html.contains("Published today"));
        assert!(html.contains("Dashboard overview"));
        assert!(html.contains("Context help"));
    }

    #[test]
    fn settings_workspace_example_renders_family_consistency() {
        let html = render(|| view! { <SettingsWorkspaceExample/> }.into_any());

        assert!(html.contains("Settings Workspace"));
        assert!(html.contains("data-intent=\"workspace\""));
        assert!(html.contains("Workflow settings"));
    }

    #[test]
    fn dashboard_shell_example_renders_family_consistency() {
        let html = render(|| view! { <DashboardShellExample/> }.into_any());

        assert!(html.contains("Operations Dashboard"));
        assert!(html.contains("data-intent=\"workspace\""));
        assert!(html.contains("Overview refreshed"));
    }
}
