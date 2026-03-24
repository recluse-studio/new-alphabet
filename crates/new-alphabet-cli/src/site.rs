use std::fs;
use std::path::{Path, PathBuf};

use leptos::prelude::*;
use new_alphabet_components::{NavIndexItem, NavIndexItemState};
use new_alphabet_core::{IntentKind, ProjectManifest, SurfaceManifest};
use new_alphabet_foundation::render_stylesheet;
use new_alphabet_recipes::{
    ArticleAdjacentLink, ArticleAdjacentLinks, ArticleMetaItem, ArticleSection, ArticleShell,
    ArticleSidebarNav, BlogIndex, BlogIndexEntry, BlogIndexSection, DocsShellExample,
    ReviewQueueExample, SettingsWorkspaceExample,
};
use new_alphabet_schema::bundle_format_version;
use serde::{Deserialize, Serialize};

pub const SITE_SOURCE_FILE: &str = "new-alphabet-site.json";
const SITE_DIR: &str = "site";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SiteSource {
    pub schema_version: String,
    pub project_name: String,
    pub prompt_id: String,
    pub prompt: String,
    pub site: GeneratedSite,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GeneratedSite {
    Blog(BlogSiteSource),
    Example(ExampleSiteSource),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExampleSiteSource {
    pub recipe: String,
    pub document_title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlogSiteSource {
    pub title: String,
    pub introduction: String,
    pub taxonomy: Vec<LinkItemSource>,
    pub archive: Vec<LinkItemSource>,
    pub articles: Vec<BlogArticleSource>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkItemSource {
    pub label: String,
    pub href: String,
    pub current: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlogArticleSource {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub metadata: String,
    pub dek: String,
    pub sections: Vec<ArticleSectionSource>,
    pub metadata_items: Vec<LabeledValueSource>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArticleSectionSource {
    pub id: String,
    pub title: String,
    pub paragraphs: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LabeledValueSource {
    pub label: String,
    pub value: String,
}

pub fn seed_site_source(
    project_name: &str,
    prompt_id: &str,
    prompt: &str,
) -> Result<SiteSource, String> {
    let site =
        match prompt_id {
            "prompt.blog" => GeneratedSite::Blog(seed_blog_site(project_name)),
            "prompt.docs" => GeneratedSite::Example(ExampleSiteSource {
                recipe: "DocsShell".to_owned(),
                document_title: format!("{project_name} Manual"),
            }),
            "prompt.review_workspace" | "prompt.review_workspace_dense" => {
                GeneratedSite::Example(ExampleSiteSource {
                    recipe: "ReviewQueue".to_owned(),
                    document_title: format!("{project_name} Review Workspace"),
                })
            }
            "prompt.settings" => GeneratedSite::Example(ExampleSiteSource {
                recipe: "SettingsWorkspace".to_owned(),
                document_title: format!("{project_name} Settings Workspace"),
            }),
            _ => return Err(
                "generate currently supports blog, docs, review workspace, and settings prompts."
                    .to_owned(),
            ),
        };

    Ok(SiteSource {
        schema_version: bundle_format_version().to_owned(),
        project_name: project_name.to_owned(),
        prompt_id: prompt_id.to_owned(),
        prompt: prompt.to_owned(),
        site,
    })
}

pub fn load_site_source(root: &Path) -> Result<SiteSource, String> {
    let content = fs::read_to_string(root.join(SITE_SOURCE_FILE)).map_err(io_error)?;
    serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse {SITE_SOURCE_FILE}: {error}"))
}

pub fn manifest_for_site_source(
    source: &SiteSource,
    surfaces: Vec<SurfaceManifest>,
) -> ProjectManifest {
    ProjectManifest {
        schema_version: bundle_format_version().to_owned(),
        project_name: source.project_name.clone(),
        surfaces,
    }
}

pub fn recipe_inventory(source: &SiteSource) -> Vec<(String, IntentKind)> {
    match &source.site {
        GeneratedSite::Blog(_) => vec![
            ("BlogIndex".to_owned(), IntentKind::Editorial),
            ("ArticleShell".to_owned(), IntentKind::Editorial),
        ],
        GeneratedSite::Example(example) => vec![(
            example.recipe.clone(),
            match example.recipe.as_str() {
                "DocsShell" => IntentKind::Editorial,
                _ => IntentKind::Workspace,
            },
        )],
    }
}

pub fn write_site(root: &Path) -> Result<Vec<PathBuf>, String> {
    let source = load_site_source(root)?;
    let site_dir = root.join(SITE_DIR);
    let mut wrote_files = Vec::new();

    fs::create_dir_all(site_dir.join("assets")).map_err(io_error)?;
    write_if_changed(
        &site_dir.join("assets").join("new-alphabet.css"),
        &render_stylesheet(),
        &mut wrote_files,
    )?;

    match &source.site {
        GeneratedSite::Blog(blog) => write_blog_site(blog, &site_dir, &mut wrote_files)?,
        GeneratedSite::Example(example) => {
            write_example_site(example, &site_dir, &mut wrote_files)?
        }
    }

    Ok(wrote_files)
}

fn write_example_site(
    example: &ExampleSiteSource,
    site_dir: &Path,
    wrote_files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    fs::create_dir_all(site_dir).map_err(io_error)?;
    let body = match example.recipe.as_str() {
        "DocsShell" => view! { <DocsShellExample /> }.to_html(),
        "ReviewQueue" => view! { <ReviewQueueExample /> }.to_html(),
        "SettingsWorkspace" => view! { <SettingsWorkspaceExample /> }.to_html(),
        _ => {
            return Err("unsupported generated example recipe".to_owned());
        }
    }
    .replace("<!>", "");

    write_if_changed(
        &site_dir.join("index.html"),
        &html_document(&example.document_title, &body, "assets/new-alphabet.css"),
        wrote_files,
    )
}

fn write_blog_site(
    blog: &BlogSiteSource,
    site_dir: &Path,
    wrote_files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    fs::create_dir_all(site_dir).map_err(io_error)?;
    fs::create_dir_all(site_dir.join("articles")).map_err(io_error)?;

    write_if_changed(
        &site_dir.join("index.html"),
        &render_blog_archive_document(blog),
        wrote_files,
    )?;

    for (index, article) in blog.articles.iter().enumerate() {
        write_if_changed(
            &site_dir
                .join("articles")
                .join(format!("{}.html", article.slug)),
            &render_blog_article_document(blog, index, article),
            wrote_files,
        )?;
    }

    Ok(())
}

fn render_blog_archive_document(source: &BlogSiteSource) -> String {
    let entries = leak_slice(
        source
            .articles
            .iter()
            .map(|article| BlogIndexEntry {
                title: leak_str(&article.title),
                href: leak_str(format!("articles/{}.html", article.slug)),
                summary: leak_str(&article.summary),
                metadata: leak_str(&article.metadata),
            })
            .collect(),
    );
    let taxonomy = optional_blog_index_section("Taxonomy", &source.taxonomy);
    let archive = optional_blog_index_section("Archive", &source.archive);
    let body = match (taxonomy, archive) {
        (Some(taxonomy), Some(archive)) => view! {
            <BlogIndex
                title=leak_str(&source.title)
                entries=entries
                introduction=leak_str(&source.introduction)
                taxonomy=taxonomy
                archive=archive
            />
        }
        .to_html(),
        (Some(taxonomy), None) => view! {
            <BlogIndex
                title=leak_str(&source.title)
                entries=entries
                introduction=leak_str(&source.introduction)
                taxonomy=taxonomy
            />
        }
        .to_html(),
        (None, Some(archive)) => view! {
            <BlogIndex
                title=leak_str(&source.title)
                entries=entries
                introduction=leak_str(&source.introduction)
                archive=archive
            />
        }
        .to_html(),
        (None, None) => view! {
            <BlogIndex
                title=leak_str(&source.title)
                entries=entries
                introduction=leak_str(&source.introduction)
            />
        }
        .to_html(),
    }
    .replace("<!>", "");

    html_document(&source.title, &body, "assets/new-alphabet.css")
}

fn render_blog_article_document(
    source: &BlogSiteSource,
    index: usize,
    article: &BlogArticleSource,
) -> String {
    let sections = leak_slice(
        article
            .sections
            .iter()
            .map(|section| ArticleSection {
                id: leak_str(&section.id),
                title: leak_str(&section.title),
                paragraphs: leak_slice(
                    section
                        .paragraphs
                        .iter()
                        .map(|paragraph| leak_str(paragraph))
                        .collect(),
                ),
            })
            .collect(),
    );

    let metadata = leak_slice(
        article
            .metadata_items
            .iter()
            .map(|item| ArticleMetaItem {
                label: leak_str(&item.label),
                value: leak_str(&item.value),
            })
            .collect(),
    );

    let navigation = ArticleSidebarNav {
        title: "On this page",
        items: leak_slice(
            article
                .sections
                .iter()
                .enumerate()
                .map(|(section_index, section)| NavIndexItem {
                    label: leak_str(&section.title),
                    href: leak_str(format!("#{}", section.id)),
                    state: if section_index == 0 {
                        NavIndexItemState::Current
                    } else {
                        NavIndexItemState::Default
                    },
                })
                .collect(),
        ),
    };

    let adjacent = ArticleAdjacentLinks {
        previous: source
            .articles
            .get(index.saturating_sub(1))
            .filter(|_| index > 0)
            .map(|previous| ArticleAdjacentLink {
                label: leak_str(format!("Previous: {}", previous.title)),
                href: leak_str(format!("{}.html", previous.slug)),
            }),
        next: source
            .articles
            .get(index + 1)
            .map(|next| ArticleAdjacentLink {
                label: leak_str(format!("Next: {}", next.title)),
                href: leak_str(format!("{}.html", next.slug)),
            }),
    };

    let body = view! {
        <ArticleShell
            title=leak_str(&article.title)
            sections=sections
            dek=leak_str(&article.dek)
            metadata=metadata
            local_navigation=navigation
            adjacent=adjacent
        />
    }
    .to_html()
    .replace("<!>", "");

    html_document(&article.title, &body, "../assets/new-alphabet.css")
}

fn optional_blog_index_section(title: &str, items: &[LinkItemSource]) -> Option<BlogIndexSection> {
    if items.is_empty() {
        return None;
    }

    Some(BlogIndexSection {
        title: leak_str(title),
        items: leak_slice(
            items
                .iter()
                .map(|item| NavIndexItem {
                    label: leak_str(&item.label),
                    href: leak_str(&item.href),
                    state: if item.current {
                        NavIndexItemState::Current
                    } else {
                        NavIndexItemState::Default
                    },
                })
                .collect(),
        ),
    })
}

fn seed_blog_site(project_name: &str) -> BlogSiteSource {
    BlogSiteSource {
        title: format!("{project_name} Journal"),
        introduction: "A severe editorial front page built from New Alphabet with archive-first structure and adjacent support.".to_owned(),
        taxonomy: vec![
            LinkItemSource {
                label: "Essays".to_owned(),
                href: "#essays".to_owned(),
                current: true,
            },
            LinkItemSource {
                label: "Notes".to_owned(),
                href: "#notes".to_owned(),
                current: false,
            },
        ],
        archive: vec![
            LinkItemSource {
                label: "2026".to_owned(),
                href: "#y2026".to_owned(),
                current: true,
            },
            LinkItemSource {
                label: "Selected references".to_owned(),
                href: "#references".to_owned(),
                current: false,
            },
        ],
        articles: vec![
            BlogArticleSource {
                slug: "grid-law".to_owned(),
                title: "Grid Law for the Archive".to_owned(),
                summary: "A short note on why the archive surface stays severe, ordered, and explicit.".to_owned(),
                metadata: "Essay 001 · March 2026".to_owned(),
                dek: "Editorial surfaces stay severe when the main field is explicit and support remains secondary.".to_owned(),
                sections: vec![
                    ArticleSectionSource {
                        id: "operating-law".to_owned(),
                        title: "Operating law".to_owned(),
                        paragraphs: vec![
                            "The archive is not a pile of cards. It is a field with a named main region and bounded secondary support.".to_owned(),
                            "When the grid is explicit, the archive reads as a system rather than a feed that happened to line up today.".to_owned(),
                        ],
                    },
                    ArticleSectionSource {
                        id: "secondary-structure".to_owned(),
                        title: "Secondary structure".to_owned(),
                        paragraphs: vec![
                            "Taxonomy, year groupings, and references remain adjacent and optional. They never displace the entry list as the primary field.".to_owned(),
                            "That restraint is the difference between editorial coherence and generic content-product chrome.".to_owned(),
                        ],
                    },
                ],
                metadata_items: vec![
                    LabeledValueSource {
                        label: "Published".to_owned(),
                        value: "March 7, 2026".to_owned(),
                    },
                    LabeledValueSource {
                        label: "Surface".to_owned(),
                        value: "ArticleShell".to_owned(),
                    },
                    LabeledValueSource {
                        label: "Density".to_owned(),
                        value: "Calm".to_owned(),
                    },
                ],
            },
            BlogArticleSource {
                slug: "archive-discipline".to_owned(),
                title: "Archive Discipline and Quiet Links".to_owned(),
                summary: "Secondary structure should stay adjacent and calm rather than turning into feed chrome.".to_owned(),
                metadata: "Essay 002 · March 2026".to_owned(),
                dek: "Adjacent reading stays present, bounded, and typographic instead of collapsing into content-product chrome.".to_owned(),
                sections: vec![
                    ArticleSectionSource {
                        id: "quiet-links".to_owned(),
                        title: "Quiet links".to_owned(),
                        paragraphs: vec![
                            "Links in an editorial system should carry navigation, not personality. The structure does the hierarchy work first.".to_owned(),
                            "Once the archive and article surfaces share the same grammar, adjacent reading can stay present without becoming promotional debris.".to_owned(),
                        ],
                    },
                    ArticleSectionSource {
                        id: "family-resemblance".to_owned(),
                        title: "Family resemblance".to_owned(),
                        paragraphs: vec![
                            "The archive, the article, and the docs shell can differ in content without becoming separate products.".to_owned(),
                            "That family resemblance is the point of New Alphabet: one law, multiple calm surfaces.".to_owned(),
                        ],
                    },
                ],
                metadata_items: vec![
                    LabeledValueSource {
                        label: "Published".to_owned(),
                        value: "March 7, 2026".to_owned(),
                    },
                    LabeledValueSource {
                        label: "Surface".to_owned(),
                        value: "ArticleShell".to_owned(),
                    },
                    LabeledValueSource {
                        label: "Topic".to_owned(),
                        value: "Editorial discipline".to_owned(),
                    },
                ],
            },
        ],
    }
}

fn html_document(title: &str, body: &str, stylesheet_path: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{title}</title>\n<link rel=\"stylesheet\" href=\"{stylesheet_path}\">\n</head>\n<body>\n{body}\n</body>\n</html>\n"
    )
}

fn leak_str(value: impl AsRef<str>) -> &'static str {
    Box::leak(value.as_ref().to_owned().into_boxed_str())
}

fn leak_slice<T>(values: Vec<T>) -> &'static [T] {
    Box::leak(values.into_boxed_slice())
}

fn write_if_changed(
    path: &Path,
    content: &str,
    wrote_files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let existing = fs::read_to_string(path).ok();
    if existing.as_deref() == Some(content) {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }

    fs::write(path, content.as_bytes()).map_err(io_error)?;
    wrote_files.push(path.to_path_buf());
    Ok(())
}

fn io_error(error: std::io::Error) -> String {
    error.to_string()
}
