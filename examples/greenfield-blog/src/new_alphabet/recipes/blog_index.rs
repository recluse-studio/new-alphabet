use leptos::prelude::*;
use new_alphabet_recipes::BlogIndexExample;

pub const RECIPE_ID: &str = "BlogIndex";
pub const REQUIRED_REGIONS: &[&str] = &[
    "main",
];
pub const OPTIONAL_REGIONS: &[&str] = &[
    "support",
];
pub const REQUIRED_PRIMITIVES: &[&str] = &[
    "AppShell",
    "PageGrid",
    "Region",
    "Panel",
    "Stack",
    "SectionHeader",
];
pub const REQUIRED_COMPONENTS: &[&str] = &[
    "EmptyState",
    "NavIndex",
];
pub const REFERENCE_EXAMPLES: &[&str] = &[
    "BlogIndexExample",
    "BlogIndexMinimalExample",
];
pub const DOCUMENTATION_PATHS: &[&str] = &[
    "docs/cli.md",
    "docs/foundations.md",
    "docs/primitives.md",
    "docs/recipes.md",
];

#[component]
pub fn BlogIndexSurface() -> impl IntoView {
    view! { <BlogIndexExample /> }
}
