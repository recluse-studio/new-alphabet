#![forbid(unsafe_code)]

mod editorial;

pub use editorial::{
    BlogIndex, BlogIndexEntry, BlogIndexExample, BlogIndexMinimalExample, BlogIndexSection,
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
}
