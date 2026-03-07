use std::fs;
use std::path::PathBuf;

use leptos::prelude::*;
use new_alphabet_components::NavIndexItem;
use new_alphabet_recipes::{ArticleSection, DocsContextItem, DocsNavSection, DocsShell};
use new_alphabet_schema::contract_bundle;

const NAV_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Manual", "#manual"),
    NavIndexItem::new("Docs set", "#docs-set"),
    NavIndexItem::new("Recipes", "#recipes"),
    NavIndexItem::new("CLI", "#cli"),
    NavIndexItem::new("Agent workflow", "#agent-workflow"),
    NavIndexItem::new("Examples", "#examples"),
];

const NAVIGATION: DocsNavSection = DocsNavSection::new("Documentation", NAV_ITEMS);

const TOC_ITEMS: &[NavIndexItem] = &[
    NavIndexItem::current("Manual", "#manual"),
    NavIndexItem::new("Docs set", "#docs-set"),
    NavIndexItem::new("Recipes", "#recipes"),
    NavIndexItem::new("CLI", "#cli"),
    NavIndexItem::new("Agent workflow", "#agent-workflow"),
    NavIndexItem::new("Examples", "#examples"),
];

const TABLE_OF_CONTENTS: DocsNavSection = DocsNavSection::new("On this page", TOC_ITEMS);

const SECTIONS: &[ArticleSection] = &[
    ArticleSection::new(
        "manual",
        "Manual",
        &[
            "New Alphabet documentation is a reference surface first. It explains law, limits, examples, and workflow without marketing inflation.",
            "The docs set is split between source documents in /docs and this generated index surface in apps/docs/site/index.html.",
        ],
    ),
    ArticleSection::new(
        "docs-set",
        "Docs set",
        &[
            "Foundations, primitives, components, recipes, CLI, agent contract, contributing, roadmap, and release notes live in the /docs directory.",
            "Reusable session bootstrap material and sparse, moderate, and high-guidance prompt examples live in /prompts.",
            "Each document is written against the current runtime and contract rather than paraphrasing an imagined future system.",
        ],
    ),
    ArticleSection::new(
        "recipes",
        "Recipes",
        &[
            "Editorial proof is carried by BlogIndex, ArticleShell, and DocsShell.",
            "Workflow proof is carried by SearchResultsWorkspace, ReviewQueue, SettingsWorkspace, and DashboardShell.",
        ],
    ),
    ArticleSection::new(
        "cli",
        "CLI",
        &[
            "The new-alphabet CLI is manifest-first and contract-backed. Init and add write only additive project files. Explain, inspect, validate, export, and plan read from the same schema bundle.",
            "Read-heavy commands stay read-only. Export writes only when asked for an output path.",
        ],
    ),
    ArticleSection::new(
        "agent-workflow",
        "Agent workflow",
        &[
            "The agent contract exports foundations, primitives, components, recipes, composition rules, state contracts, anti-patterns, schemas, examples, and prompt intents.",
            "Session bootstrap material in /prompts tells agents which repo files and schema sections must be loaded before planning or repair.",
            "Generation chooses recipes or primitive compositions before writing UI code, and repair prefers deleting invalid structure before adding abstractions.",
        ],
    ),
    ArticleSection::new(
        "examples",
        "Examples",
        &[
            "Runnable examples live in the primitives, components, and recipes crates and are verified through cargo test --workspace.",
            "Those examples are referenced again in the schema bundle so docs, validation, and scaffolding all point at the same source material.",
        ],
    ),
];

#[component]
fn DocsIndex() -> impl IntoView {
    let bundle = contract_bundle();
    let context = Box::leak(
        vec![
            DocsContextItem::new(
                "Bundle format",
                Box::leak(bundle.bundle_format_version.into_boxed_str()),
            ),
            DocsContextItem::new(
                "Prompt intents",
                Box::leak(bundle.prompt_intents.len().to_string().into_boxed_str()),
            ),
            DocsContextItem::new(
                "Examples",
                Box::leak(bundle.examples.len().to_string().into_boxed_str()),
            ),
            DocsContextItem::new("Verification", "cargo test --workspace"),
        ]
        .into_boxed_slice(),
    );

    view! {
        <DocsShell
            title="New Alphabet Manual"
            navigation=NAVIGATION
            sections=SECTIONS
            table_of_contents=TABLE_OF_CONTENTS
            context=context
        />
    }
}

fn main() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let site_dir = root.join("site");
    fs::create_dir_all(&site_dir).map_err(|error| error.to_string())?;

    let body = view! { <DocsIndex/> }.to_html().replace("<!>", "");
    let document = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>New Alphabet Manual</title>\n</head>\n<body>\n{}\n</body>\n</html>\n",
        body
    );

    fs::write(site_dir.join("index.html"), document.as_bytes()).map_err(|error| error.to_string())
}
