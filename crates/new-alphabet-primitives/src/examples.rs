use crate::{
    AppShell, Band, ColumnGroup, ColumnPreset, Divider, FrameIntent, PageGrid, Panel, PanelState,
    Rail, RailSide, Region, RegionPlacement, Row, RowAlign, RowDistribution, SectionHeader, Stack,
    StackSpace, SurfaceStrength,
};
use leptos::prelude::*;
use new_alphabet_foundation::{DensityMode, RailWidthToken, RegionClass};

#[component]
pub fn EditorialAnchorExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Calm intent=FrameIntent::Editorial>
            <PageGrid intent=FrameIntent::Editorial>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <h1>"Essay Index"</h1>
                    <p>"Longform entries stay primary while the grid keeps adjacent structure quiet."</p>
                </Region>
                <Region kind=RegionClass::Support placement=RegionPlacement::Support>
                    <p>"Archive navigation and publication notes remain secondary."</p>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn EditorialStructureExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Calm intent=FrameIntent::Editorial>
            <PageGrid intent=FrameIntent::Editorial>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Stack spacing=StackSpace::Loose>
                        <h1>"Documentation Surface"</h1>
                        <ColumnGroup preset=ColumnPreset::MainSupport>
                            <div>"Reading flow stays dominant."</div>
                            <div>"Support material stays adjacent and secondary."</div>
                        </ColumnGroup>
                    </Stack>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn WorkspaceAnchorExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <h1>"Review Queue"</h1>
                    <p>"The center region carries the active queue and decision surface."</p>
                </Region>
                <Region kind=RegionClass::Detail placement=RegionPlacement::Detail>
                    <p>"The detail region stays adjacent on wide surfaces and collapses when needed."</p>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn WorkflowStructureExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Rail width=RailWidthToken::Default side=RailSide::Start>
                    <p>"Filters and navigation stay in a bounded side structure."</p>
                </Rail>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Stack spacing=StackSpace::Default>
                        <Row align=RowAlign::Center distribution=RowDistribution::Between>
                            <h1>"Results"</h1>
                            <span>"24 open items"</span>
                        </Row>
                        <p>"Row and stack rhythm stay token-driven inside the primary workspace region."</p>
                    </Stack>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn EditorialSurfaceExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Calm intent=FrameIntent::Editorial>
            <Band strength=SurfaceStrength::Strong>
                <SectionHeader
                    title="Issue Notes"
                    subtitle="A bounded opening surface clarifies the editorial frame."
                    annotation="Spring 2026"
                />
            </Band>
            <PageGrid intent=FrameIntent::Editorial>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel strength=SurfaceStrength::Strong state=PanelState::Default>
                        <SectionHeader
                            title="Lead Essay"
                            subtitle="Hierarchy comes from proportion, border, and spacing."
                        />
                        <Divider/>
                        <p>"Panel and header structure stay quiet while the reading flow remains primary."</p>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn SettingsSurfaceExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel state=PanelState::Loading>
                        <SectionHeader
                            title="Account Settings"
                            subtitle="The panel surface can express loading without inventing a new visual language."
                            annotation="Sync pending"
                        />
                        <Divider strength=SurfaceStrength::Strong/>
                        <p>"Settings panels stay bounded, explicit, and structurally legible."</p>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}
