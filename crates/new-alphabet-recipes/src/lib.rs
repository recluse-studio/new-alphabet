#![forbid(unsafe_code)]

mod editorial;

pub use editorial::{
    ArticleAdjacentLink, ArticleAdjacentLinks, ArticleMetaItem, ArticleSection, ArticleShell,
    ArticleShellExample, ArticleShellMinimalExample, ArticleSidebarNav, BlogIndex, BlogIndexEntry,
    BlogIndexExample, BlogIndexMinimalExample, BlogIndexSection, DocsContextItem, DocsNavSection,
    DocsShell, DocsShellExample, DocsShellMinimalExample,
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
}
