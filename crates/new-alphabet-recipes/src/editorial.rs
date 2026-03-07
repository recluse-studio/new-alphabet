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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArticleSection {
    pub id: &'static str,
    pub title: &'static str,
    pub paragraphs: &'static [&'static str],
}

impl ArticleSection {
    pub const fn new(
        id: &'static str,
        title: &'static str,
        paragraphs: &'static [&'static str],
    ) -> Self {
        Self {
            id,
            title,
            paragraphs,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArticleSidebarNav {
    pub title: &'static str,
    pub items: &'static [NavIndexItem],
}

impl ArticleSidebarNav {
    pub const fn new(title: &'static str, items: &'static [NavIndexItem]) -> Self {
        Self { title, items }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArticleMetaItem {
    pub label: &'static str,
    pub value: &'static str,
}

impl ArticleMetaItem {
    pub const fn new(label: &'static str, value: &'static str) -> Self {
        Self { label, value }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArticleAdjacentLink {
    pub label: &'static str,
    pub href: &'static str,
}

impl ArticleAdjacentLink {
    pub const fn new(label: &'static str, href: &'static str) -> Self {
        Self { label, href }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArticleAdjacentLinks {
    pub previous: Option<ArticleAdjacentLink>,
    pub next: Option<ArticleAdjacentLink>,
}

impl ArticleAdjacentLinks {
    pub const fn new(
        previous: Option<ArticleAdjacentLink>,
        next: Option<ArticleAdjacentLink>,
    ) -> Self {
        Self { previous, next }
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

#[component]
pub fn ArticleShell(
    title: &'static str,
    sections: &'static [ArticleSection],
    #[prop(optional)] dek: Option<&'static str>,
    #[prop(optional)] metadata: Option<&'static [ArticleMetaItem]>,
    #[prop(optional)] local_navigation: Option<ArticleSidebarNav>,
    #[prop(optional)] adjacent: Option<ArticleAdjacentLinks>,
) -> impl IntoView {
    let has_support = metadata.is_some() || local_navigation.is_some() || adjacent.is_some();
    let header = match dek {
        Some(dek) => view! {
            <SectionHeader
                title=title
                subtitle=dek
                annotation="Essay"
            />
        }
        .into_any(),
        None => view! {
            <SectionHeader
                title=title
                annotation="Essay"
            />
        }
        .into_any(),
    };

    view! {
        <AppShell density=DensityMode::Calm intent=FrameIntent::Editorial>
            <Band strength=SurfaceStrength::Strong>
                {header}
            </Band>
            <PageGrid intent=FrameIntent::Editorial>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <article class="na-article-shell">
                        <Stack spacing=StackSpace::Loose>
                            {sections
                                .iter()
                                .map(|section| {
                                    view! {
                                        <section id=section.id class="na-article-shell__section">
                                            <h2>{section.title}</h2>
                                            <Stack spacing=StackSpace::Default>
                                                {section
                                                    .paragraphs
                                                    .iter()
                                                    .map(|paragraph| view! { <p>{*paragraph}</p> })
                                                    .collect_view()}
                                            </Stack>
                                        </section>
                                    }
                                })
                                .collect_view()}
                        </Stack>
                    </article>
                </Region>
                {if has_support {
                    view! {
                        <Region kind=RegionClass::Support placement=RegionPlacement::Support>
                            <Stack spacing=StackSpace::Tight>
                                {local_navigation.map(|navigation| {
                                    view! {
                                        <Panel>
                                            <SectionHeader
                                                title=navigation.title
                                                subtitle="Local structure stays adjacent and optional."
                                            />
                                            <NavIndex label=navigation.title items=navigation.items />
                                        </Panel>
                                    }
                                    .into_any()
                                })}
                                {metadata.map(|metadata| {
                                    view! {
                                        <Panel>
                                            <SectionHeader
                                                title="Metadata"
                                                subtitle="Publication context remains secondary."
                                            />
                                            <dl class="na-article-shell__meta">
                                                {metadata
                                                    .iter()
                                                    .map(|item| {
                                                        view! {
                                                            <div class="na-article-shell__meta-item">
                                                                <dt>{item.label}</dt>
                                                                <dd>{item.value}</dd>
                                                            </div>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </dl>
                                        </Panel>
                                    }
                                    .into_any()
                                })}
                                {adjacent.map(|adjacent| {
                                    view! {
                                        <Panel>
                                            <SectionHeader
                                                title="Continue"
                                                subtitle="Adjacent reading remains present without crowding the article."
                                            />
                                            <Stack spacing=StackSpace::Tight>
                                                {adjacent.previous.map(|link| {
                                                    view! {
                                                        <a
                                                            href=link.href
                                                            data-focus-token=StateToken::FocusRing.id()
                                                        >
                                                            {link.label}
                                                        </a>
                                                    }
                                                })}
                                                {adjacent.next.map(|link| {
                                                    view! {
                                                        <a
                                                            href=link.href
                                                            data-focus-token=StateToken::FocusRing.id()
                                                        >
                                                            {link.label}
                                                        </a>
                                                    }
                                                })}
                                            </Stack>
                                        </Panel>
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

const ARTICLE_SECTION_ONE: &[&str] = &[
    "New Alphabet treats article structure as a reading device first and a navigation problem second.",
    "The article body should therefore stay primary at every breakpoint, with adjacent structure collapsing rather than competing.",
];

const ARTICLE_SECTION_TWO: &[&str] = &[
    "Support metadata belongs beside the reading flow, not inside it.",
    "Adjacent reading links stay visible, but they should not perform as marketing calls to action.",
];

const ARTICLE_SECTIONS: &[ArticleSection] = &[
    ArticleSection::new("premise", "Premise", ARTICLE_SECTION_ONE),
    ArticleSection::new("adjacency", "Adjacency", ARTICLE_SECTION_TWO),
];

const ARTICLE_NAV_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Premise", "#premise"),
    NavIndexItem::new("Adjacency", "#adjacency"),
];

const ARTICLE_LOCAL_NAV: ArticleSidebarNav = ArticleSidebarNav::new("Contents", ARTICLE_NAV_ITEMS);

const ARTICLE_METADATA: &[ArticleMetaItem] = &[
    ArticleMetaItem::new("Author", "Recluse Studio"),
    ArticleMetaItem::new("Published", "March 2026"),
];

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

const MINIMAL_ARTICLE_SECTION: &[ArticleSection] = &[ArticleSection::new(
    "opening",
    "Opening",
    &[
        "A narrow article shell can remain legible without side structures on small or simple surfaces.",
    ],
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

#[component]
pub fn ArticleShellExample() -> impl IntoView {
    view! {
        <ArticleShell
            title="Reading Flow as Operating Law"
            dek="A longform shell should keep the article body primary while still allowing context and adjacent movement."
            sections=ARTICLE_SECTIONS
            metadata=ARTICLE_METADATA
            local_navigation=ARTICLE_LOCAL_NAV
            adjacent=ARTICLE_ADJACENT
        />
    }
}

#[component]
pub fn ArticleShellMinimalExample() -> impl IntoView {
    view! {
        <ArticleShell
            title="Opening Note"
            sections=MINIMAL_ARTICLE_SECTION
        />
    }
}
