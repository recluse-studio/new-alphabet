use leptos::prelude::*;
use new_alphabet_components::{EmptyState, NavIndex, NavIndexItem};
use new_alphabet_foundation::{DensityMode, RegionClass, StateToken};
use new_alphabet_primitives::{
    AppShell, Band, FrameIntent, PageGrid, Panel, Region, RegionPlacement, SectionHeader, Stack,
    StackSpace, SurfaceStrength,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlogIndexEntry {
    pub title: &'static str,
    pub href: &'static str,
    pub summary: &'static str,
    pub metadata: &'static str,
}

impl BlogIndexEntry {
    pub const fn new(
        title: &'static str,
        href: &'static str,
        summary: &'static str,
        metadata: &'static str,
    ) -> Self {
        Self {
            title,
            href,
            summary,
            metadata,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlogIndexSection {
    pub title: &'static str,
    pub items: &'static [NavIndexItem],
}

impl BlogIndexSection {
    pub const fn new(title: &'static str, items: &'static [NavIndexItem]) -> Self {
        Self { title, items }
    }
}

fn support_panel(section: BlogIndexSection) -> AnyView {
    view! {
        <Panel>
            <SectionHeader
                title=section.title
                subtitle="Secondary archive structure remains adjacent and quiet."
            />
            <NavIndex label=section.title items=section.items />
        </Panel>
    }
    .into_any()
}

#[component]
pub fn BlogIndex(
    title: &'static str,
    entries: &'static [BlogIndexEntry],
    #[prop(optional)] introduction: Option<&'static str>,
    #[prop(optional)] taxonomy: Option<BlogIndexSection>,
    #[prop(optional)] archive: Option<BlogIndexSection>,
) -> impl IntoView {
    let has_support = taxonomy.is_some() || archive.is_some();
    let header = match introduction {
        Some(introduction) => view! {
            <SectionHeader
                title=title
                subtitle=introduction
                annotation="Archive"
            />
        }
        .into_any(),
        None => view! {
            <SectionHeader
                title=title
                annotation="Archive"
            />
        }
        .into_any(),
    };
    let entry_body = if entries.is_empty() {
        view! {
            <EmptyState
                title="No published entries"
                message="Add the first note to begin the archive."
                next_action="Publish the first entry"
            />
        }
        .into_any()
    } else {
        view! {
            <Stack spacing=StackSpace::Loose>
                {entries
                    .iter()
                    .map(|entry| {
                        view! {
                            <article class="na-blog-index__entry">
                                <p class="na-blog-index__meta">{entry.metadata}</p>
                                <h2>
                                    <a
                                        href=entry.href
                                        data-focus-token=StateToken::FocusRing.id()
                                    >
                                        {entry.title}
                                    </a>
                                </h2>
                                <p class="na-blog-index__summary">{entry.summary}</p>
                            </article>
                        }
                    })
                    .collect_view()}
            </Stack>
        }
        .into_any()
    };

    view! {
        <AppShell density=DensityMode::Calm intent=FrameIntent::Editorial>
            <Band strength=SurfaceStrength::Strong>
                {header}
            </Band>
            <PageGrid intent=FrameIntent::Editorial>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel strength=SurfaceStrength::Strong>
                        <SectionHeader
                            title="Entries"
                            subtitle="The archive stays primary while adjacent structure remains secondary."
                        />
                        {entry_body}
                    </Panel>
                </Region>
                {if has_support {
                    view! {
                        <Region kind=RegionClass::Support placement=RegionPlacement::Support>
                            <Stack spacing=StackSpace::Tight>
                                {taxonomy.map(support_panel)}
                                {archive.map(support_panel)}
                            </Stack>
                        </Region>
                    }
                    .into_any()
                } else {
                    view! { <></> }.into_any()
                }}
            </PageGrid>
        </AppShell>
    }
}

const STUDIO_NOTES_ENTRIES: &[BlogIndexEntry] = &[
    BlogIndexEntry::new(
        "Grid Notes for Quiet Systems",
        "/notes/grid-notes",
        "A working note on why archive structure should carry the surface before brand language does.",
        "Essay 14 · March 2026",
    ),
    BlogIndexEntry::new(
        "Release Practice and Public Memory",
        "/notes/release-practice",
        "A short note on why doctrine, progress memory, and changelog discipline belong in the product language.",
        "Note 7 · February 2026",
    ),
];

const TAXONOMY_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Essays", "/topics/essays"),
    NavIndexItem::new("Notes", "/topics/notes"),
    NavIndexItem::new("Releases", "/topics/releases"),
];

const ARCHIVE_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("2026", "/archive/2026"),
    NavIndexItem::new("2025", "/archive/2025"),
    NavIndexItem::new("2024", "/archive/2024"),
];

const TAXONOMY_SECTION: BlogIndexSection = BlogIndexSection::new("Taxonomy", TAXONOMY_ITEMS);
const ARCHIVE_SECTION: BlogIndexSection = BlogIndexSection::new("Archive", ARCHIVE_ITEMS);
const MINIMAL_ENTRIES: &[BlogIndexEntry] = &[BlogIndexEntry::new(
    "First note",
    "/notes/first-note",
    "A small opening proof that the archive can remain typographic in a single flow.",
    "Note 1 · January 2026",
)];

#[component]
pub fn BlogIndexExample() -> impl IntoView {
    view! {
        <BlogIndex
            title="Studio Notes"
            introduction="Essays, release notes, and working fragments arranged as a publication archive rather than a campaign page."
            entries=STUDIO_NOTES_ENTRIES
            taxonomy=TAXONOMY_SECTION
            archive=ARCHIVE_SECTION
        />
    }
}

#[component]
pub fn BlogIndexMinimalExample() -> impl IntoView {
    view! {
        <BlogIndex
            title="Archive"
            introduction="A narrow reading-first index without secondary navigation."
            entries=MINIMAL_ENTRIES
        />
    }
}
