#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

use leptos::prelude::*;
use new_alphabet_components::NavIndexItem;
use new_alphabet_recipes::{
    ArticleAdjacentLink, ArticleAdjacentLinks, ArticleMetaItem, ArticleSection, ArticleShell,
    ArticleSidebarNav, BlogIndex, BlogIndexEntry, BlogIndexSection,
};

#[derive(Clone, Copy)]
struct DemoArticle {
    slug: &'static str,
    title: &'static str,
    dek: &'static str,
    sections: &'static [ArticleSection],
    metadata: &'static [ArticleMetaItem],
    navigation: ArticleSidebarNav,
    adjacent: ArticleAdjacentLinks,
}

const BLOG_ENTRIES: &[BlogIndexEntry] = &[
    BlogIndexEntry::new(
        "Grid Law for the Archive",
        "articles/grid-law.html",
        "A short note on why the archive surface stays severe, ordered, and explicit.",
        "Essay 001 · March 2026",
    ),
    BlogIndexEntry::new(
        "Archive Discipline and Quiet Links",
        "articles/archive-discipline.html",
        "Secondary structure should stay adjacent and calm rather than turning into feed chrome.",
        "Essay 002 · March 2026",
    ),
];

const TAXONOMY_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Essays", "#essays"),
    NavIndexItem::new("Notes", "#notes"),
];

const ARCHIVE_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("2026", "#y2026"),
    NavIndexItem::new("Selected references", "#references"),
];

const TAXONOMY_SECTION: BlogIndexSection = BlogIndexSection::new("Taxonomy", TAXONOMY_ITEMS);
const ARCHIVE_SECTION: BlogIndexSection = BlogIndexSection::new("Archive", ARCHIVE_ITEMS);

const GRID_LAW_SECTIONS: &[ArticleSection] = &[
    ArticleSection::new(
        "operating-law",
        "Operating law",
        &[
            "The archive is not a pile of cards. It is a field with a named main region and bounded secondary support.",
            "When the grid is explicit, the archive reads as a system rather than a feed that happened to line up today.",
        ],
    ),
    ArticleSection::new(
        "secondary-structure",
        "Secondary structure",
        &[
            "Taxonomy, year groupings, and references remain adjacent and optional. They never displace the entry list as the primary field.",
            "That restraint is the difference between editorial coherence and generic content-product chrome.",
        ],
    ),
];

const GRID_LAW_METADATA: &[ArticleMetaItem] = &[
    ArticleMetaItem::new("Published", "March 7, 2026"),
    ArticleMetaItem::new("Surface", "ArticleShell"),
    ArticleMetaItem::new("Density", "Calm"),
];

const GRID_LAW_NAV_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Operating law", "#operating-law"),
    NavIndexItem::new("Secondary structure", "#secondary-structure"),
];

const GRID_LAW_NAV: ArticleSidebarNav = ArticleSidebarNav::new("On this page", GRID_LAW_NAV_ITEMS);
const GRID_LAW_ADJACENT: ArticleAdjacentLinks = ArticleAdjacentLinks::new(
    None,
    Some(ArticleAdjacentLink::new(
        "Next: Archive discipline and quiet links",
        "archive-discipline.html",
    )),
);

const ARCHIVE_DISCIPLINE_SECTIONS: &[ArticleSection] = &[
    ArticleSection::new(
        "quiet-links",
        "Quiet links",
        &[
            "Links in an editorial system should carry navigation, not personality. The structure does the hierarchy work first.",
            "Once the archive and article surfaces share the same grammar, adjacent reading can stay present without becoming promotional debris.",
        ],
    ),
    ArticleSection::new(
        "family-resemblance",
        "Family resemblance",
        &[
            "The archive, the article, and the docs shell can differ in content without becoming separate products.",
            "That family resemblance is the point of New Alphabet: one law, multiple calm surfaces.",
        ],
    ),
];

const ARCHIVE_DISCIPLINE_METADATA: &[ArticleMetaItem] = &[
    ArticleMetaItem::new("Published", "March 7, 2026"),
    ArticleMetaItem::new("Surface", "ArticleShell"),
    ArticleMetaItem::new("Topic", "Editorial discipline"),
];

const ARCHIVE_DISCIPLINE_NAV_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Quiet links", "#quiet-links"),
    NavIndexItem::new("Family resemblance", "#family-resemblance"),
];

const ARCHIVE_DISCIPLINE_NAV: ArticleSidebarNav =
    ArticleSidebarNav::new("On this page", ARCHIVE_DISCIPLINE_NAV_ITEMS);
const ARCHIVE_DISCIPLINE_ADJACENT: ArticleAdjacentLinks = ArticleAdjacentLinks::new(
    Some(ArticleAdjacentLink::new(
        "Previous: Grid law for the archive",
        "grid-law.html",
    )),
    None,
);

const DEMO_ARTICLES: &[DemoArticle] = &[
    DemoArticle {
        slug: "grid-law",
        title: "Grid Law for the Archive",
        dek: "Editorial surfaces stay severe when the main field is explicit and support remains secondary.",
        sections: GRID_LAW_SECTIONS,
        metadata: GRID_LAW_METADATA,
        navigation: GRID_LAW_NAV,
        adjacent: GRID_LAW_ADJACENT,
    },
    DemoArticle {
        slug: "archive-discipline",
        title: "Archive Discipline and Quiet Links",
        dek: "Adjacent reading stays present, bounded, and typographic instead of collapsing into content-product chrome.",
        sections: ARCHIVE_DISCIPLINE_SECTIONS,
        metadata: ARCHIVE_DISCIPLINE_METADATA,
        navigation: ARCHIVE_DISCIPLINE_NAV,
        adjacent: ARCHIVE_DISCIPLINE_ADJACENT,
    },
];

pub fn render_archive_document() -> String {
    let body = view! {
        <BlogIndex
            title="New Alphabet Journal"
            entries=BLOG_ENTRIES
            introduction="Editorial proof surface built from BlogIndex and ArticleShell rather than ad hoc content chrome."
            taxonomy=TAXONOMY_SECTION
            archive=ARCHIVE_SECTION
        />
    }
    .to_html()
    .replace("<!>", "");

    html_document("New Alphabet Journal", body)
}

fn render_article_document(article: DemoArticle) -> String {
    let body = view! {
        <ArticleShell
            title=article.title
            sections=article.sections
            dek=article.dek
            metadata=article.metadata
            local_navigation=article.navigation
            adjacent=article.adjacent
        />
    }
    .to_html()
    .replace("<!>", "");

    html_document(article.title, body)
}

pub fn write_site(site_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(site_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(site_dir.join("articles")).map_err(|error| error.to_string())?;

    fs::write(
        site_dir.join("index.html"),
        render_archive_document().as_bytes(),
    )
    .map_err(|error| error.to_string())?;

    for article in DEMO_ARTICLES {
        fs::write(
            site_dir
                .join("articles")
                .join(format!("{}.html", article.slug)),
            render_article_document(*article).as_bytes(),
        )
        .map_err(|error| error.to_string())?;
    }

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
    fn archive_document_contains_editorial_entries() {
        let html = render_archive_document();

        assert!(html.contains("New Alphabet Journal"));
        assert!(html.contains("Grid Law for the Archive"));
        assert!(html.contains("articles/grid-law.html"));
    }

    #[test]
    fn article_document_contains_sections_and_adjacent_links() {
        let html = render_article_document(DEMO_ARTICLES[0]);

        assert!(html.contains("Operating law"));
        assert!(html.contains("Secondary structure"));
        assert!(html.contains("archive-discipline.html"));
    }

    #[test]
    fn write_site_writes_archive_and_article_pages() {
        let site_dir = unique_test_dir("demo-blog");

        write_site(&site_dir).expect("site to write");

        assert!(site_dir.join("index.html").exists());
        assert!(site_dir.join("articles/grid-law.html").exists());
        assert!(site_dir.join("articles/archive-discipline.html").exists());
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
