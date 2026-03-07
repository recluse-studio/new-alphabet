#![forbid(unsafe_code)]

mod examples;
mod frame;

pub use examples::{EditorialAnchorExample, WorkspaceAnchorExample};
pub use frame::{AppShell, FrameIntent, PageGrid, Region, RegionPlacement, RegionTag};

#[cfg(test)]
mod tests {
    use leptos::prelude::*;

    use super::*;

    fn render(view: impl FnOnce() -> AnyView + 'static) -> String {
        view().to_html()
    }

    #[test]
    fn editorial_anchor_renders_main_region() {
        let html = render(|| view! { <EditorialAnchorExample/> }.into_any());
        assert!(html.contains("data-region=\"main\""));
        assert!(html.contains("data-columns-compact=\"4\""));
    }

    #[test]
    fn workspace_anchor_renders_detail_region() {
        let html = render(|| view! { <WorkspaceAnchorExample/> }.into_any());
        assert!(html.contains("data-region=\"detail\""));
        assert!(html.contains("data-placement=\"detail\""));
    }
}
