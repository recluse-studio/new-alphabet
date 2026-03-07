#![forbid(unsafe_code)]

mod composition;
mod examples;
mod frame;

pub use composition::{
    ColumnGroup, ColumnPreset, Rail, RailSide, Row, RowAlign, RowDistribution, RowGap, Stack,
    StackSpace,
};
pub use examples::{
    EditorialAnchorExample, EditorialStructureExample, WorkflowStructureExample,
    WorkspaceAnchorExample,
};
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

    #[test]
    fn workflow_structure_renders_rail_contract() {
        let html = render(|| view! { <WorkflowStructureExample/> }.into_any());
        assert!(html.contains("data-rail-width=\"layout.rail.default.width\""));
        assert!(html.contains("data-columns-wide=\"3\""));
    }

    #[test]
    fn editorial_structure_renders_token_driven_composition() {
        let html = render(|| view! { <EditorialStructureExample/> }.into_any());
        assert!(html.contains("data-gap-token=\"spacing.stack.loose\""));
        assert!(html.contains("data-columns-medium=\"2\""));
    }
}
