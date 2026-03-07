use leptos::prelude::*;
use new_alphabet_foundation::{DensityMode, RegionClass};
use new_alphabet_primitives::{
    AppShell, FrameIntent, PageGrid, Panel, Region, RegionPlacement, Row, RowAlign,
    RowDistribution, SectionHeader, Stack, StackSpace, SurfaceStrength,
};

use crate::{ActionPriority, ActionState, Button, LinkAction};

#[component]
pub fn EditorialActionExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Calm intent=FrameIntent::Editorial>
            <PageGrid intent=FrameIntent::Editorial>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel strength=SurfaceStrength::Strong>
                        <SectionHeader
                            title="Notes"
                            subtitle="Editorial actions stay typographic and quiet."
                        />
                        <Stack spacing=StackSpace::Default>
                            <Button
                                label="Start reading"
                                priority=ActionPriority::Primary
                            />
                            <LinkAction
                                label="Join the list"
                                href="/subscribe"
                                priority=ActionPriority::Secondary
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn WorkflowActionExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Submission Review"
                            subtitle="Primary and secondary actions are separated by structure, not palette sprawl."
                        />
                        <Row align=RowAlign::Center distribution=RowDistribution::Between>
                            <Button
                                label="Approve submission"
                                priority=ActionPriority::Primary
                                state=ActionState::Loading
                            />
                            <LinkAction
                                label="Open history"
                                href="/history"
                                priority=ActionPriority::Secondary
                            />
                        </Row>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}
