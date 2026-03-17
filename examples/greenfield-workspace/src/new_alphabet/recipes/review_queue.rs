use leptos::prelude::*;
use new_alphabet_recipes::ReviewQueueExample;

pub const RECIPE_ID: &str = "ReviewQueue";
pub const REQUIRED_REGIONS: &[&str] = &[
    "action_band",
    "main",
    "detail",
];
pub const OPTIONAL_REGIONS: &[&str] = &[
    "rail",
];
pub const REQUIRED_PRIMITIVES: &[&str] = &[
    "AppShell",
    "PageGrid",
    "Rail",
    "Region",
    "Panel",
    "Stack",
    "Band",
];
pub const REQUIRED_COMPONENTS: &[&str] = &[
    "CommandBar",
    "InlineAlert",
    "Table",
    "FilterRail",
    "DetailPane",
    "NavIndex",
];
pub const REFERENCE_EXAMPLES: &[&str] = &[
    "ReviewQueueExample",
    "ReviewQueueLoadingExample",
    "ReviewQueueEmptyExample",
    "ReviewQueueUnavailableDetailExample",
];
pub const DOCUMENTATION_PATHS: &[&str] = &[
    "docs/cli.md",
    "docs/foundations.md",
    "docs/primitives.md",
    "docs/recipes.md",
    "docs/components.md",
];

#[component]
pub fn ReviewQueueSurface() -> impl IntoView {
    view! { <ReviewQueueExample /> }
}
