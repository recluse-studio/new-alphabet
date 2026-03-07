use leptos::prelude::*;
use new_alphabet_foundation::{DensityMode, RegionClass};
use new_alphabet_primitives::{
    AppShell, FrameIntent, PageGrid, Panel, Region, RegionPlacement, Row, RowAlign,
    RowDistribution, SectionHeader, Stack, StackSpace, SurfaceStrength,
};

use crate::{
    ActionPriority, ActionState, Button, Checkbox, ChoiceOption, EmptyState, FieldState,
    InlineAlert, LinkAction, RadioGroup, Select, StatusBadge, StatusSeverity, Switch, TextField,
    Textarea,
};

const DENSITY_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("calm", "Calm"),
    ChoiceOption::new("regular", "Regular"),
    ChoiceOption::new("dense", "Dense"),
];

const REVIEW_DECISION_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("approve", "Approve"),
    ChoiceOption::new("hold", "Hold"),
    ChoiceOption::new("reject", "Reject"),
];

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

#[component]
pub fn SettingsFieldExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Profile Settings"
                            subtitle="Fields keep labels, help, and validation relationships explicit."
                        />
                        <Stack spacing=StackSpace::Default>
                            <TextField
                                label="Display name"
                                name="display-name"
                                value="Recluse Studio"
                                help="Public profile label."
                                state=FieldState::Success
                                message="Saved."
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn FormFieldExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Submission Form"
                            subtitle="Multi-line entry remains calm and structurally explicit."
                        />
                        <Stack spacing=StackSpace::Default>
                            <Textarea
                                label="Submission notes"
                                name="submission-notes"
                                value="Structured feedback."
                                help="Internal review context."
                                state=FieldState::Success
                                message="Saved."
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn SettingsChoiceExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Preferences"
                            subtitle="Named choice controls stay finite and explicit."
                        />
                        <Stack spacing=StackSpace::Default>
                            <Select
                                label="Workspace density"
                                name="workspace-density"
                                selected="regular"
                                options=DENSITY_OPTIONS
                                help="Applies to queue and detail surfaces."
                            />
                            <Switch
                                label="Private mode"
                                name="private-mode"
                                checked=true
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn FormChoiceExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Review Options"
                            subtitle="Choice controls only appear where the options are finite and named."
                        />
                        <Stack spacing=StackSpace::Default>
                            <RadioGroup
                                label="Review decision"
                                name="review-decision"
                                selected="approve"
                                options=REVIEW_DECISION_OPTIONS
                            />
                            <Checkbox
                                label="Attach follow-up"
                                name="attach-follow-up"
                                checked=true
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn EditorialStatusExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Calm intent=FrameIntent::Editorial>
            <PageGrid intent=FrameIntent::Editorial>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel strength=SurfaceStrength::Strong>
                        <SectionHeader
                            title="Archive note"
                            subtitle="Status remains textual and structural rather than decorative."
                        />
                        <Stack spacing=StackSpace::Default>
                            <StatusBadge
                                label="Published"
                                severity=StatusSeverity::Success
                            />
                            <InlineAlert
                                title="Archive note"
                                message="This issue is now part of the permanent index."
                                severity=StatusSeverity::Info
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}

#[component]
pub fn WorkflowStatusExample() -> impl IntoView {
    view! {
        <AppShell density=DensityMode::Regular intent=FrameIntent::Workspace>
            <PageGrid intent=FrameIntent::Workspace>
                <Region kind=RegionClass::Main placement=RegionPlacement::Main>
                    <Panel>
                        <SectionHeader
                            title="Queue feedback"
                            subtitle="Workflow status and absence states remain explicit."
                        />
                        <Stack spacing=StackSpace::Default>
                            <InlineAlert
                                title="Sync delayed"
                                message="The queue will retry in one minute."
                                severity=StatusSeverity::Warning
                            />
                            <EmptyState
                                title="No matching entries"
                                message="Adjust the filters to broaden the queue."
                                next_action="Clear filters"
                            />
                        </Stack>
                    </Panel>
                </Region>
            </PageGrid>
        </AppShell>
    }
}
