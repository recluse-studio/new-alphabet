use leptos::prelude::*;
use new_alphabet_foundation::{DensityMode, RegionClass};

use crate::{AppShell, FrameIntent, PageGrid, Region, RegionPlacement};

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
