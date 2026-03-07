#![forbid(unsafe_code)]

mod composition;
mod examples;
mod frame;
mod surfaces;

pub use composition::{
    ColumnGroup, ColumnPreset, Rail, RailSide, Row, RowAlign, RowDistribution, RowGap, Stack,
    StackSpace,
};
pub use examples::{
    EditorialAnchorExample, EditorialStructureExample, EditorialSurfaceExample,
    SettingsSurfaceExample, WorkflowStructureExample, WorkspaceAnchorExample,
};
pub use frame::{AppShell, FrameIntent, PageGrid, Region, RegionPlacement, RegionTag};
pub use surfaces::{Band, Divider, Panel, PanelState, SectionHeader, SurfaceStrength};

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

    #[test]
    fn editorial_surface_renders_panel_hierarchy() {
        let html = render(|| view! { <EditorialSurfaceExample/> }.into_any());
        assert!(html.contains("data-panel-state=\"default\""));
        assert!(html.contains("data-border-token=\"border.panel.strong\""));
    }

    #[test]
    fn settings_surface_renders_panel_state_and_divider() {
        let html = render(|| view! { <SettingsSurfaceExample/> }.into_any());
        assert!(html.contains("data-panel-state=\"loading\""));
        assert!(html.contains("data-state-token=\"state.loading.muted\""));
        assert!(html.contains("class=\"na-divider\""));
    }
}
