#![forbid(unsafe_code)]

mod actions;
mod choices;
mod examples;
mod fields;

pub use actions::{ActionPriority, ActionState, Button, ButtonType, LinkAction};
pub use choices::{Checkbox, ChoiceOption, RadioGroup, Select, Switch};
pub use examples::{
    EditorialActionExample, FormChoiceExample, FormFieldExample, SettingsChoiceExample,
    SettingsFieldExample, WorkflowActionExample,
};
pub use fields::{FieldState, TextField, Textarea};

#[cfg(test)]
const PLAN_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("solo", "Solo"),
    ChoiceOption::new("studio", "Studio"),
];

#[cfg(test)]
const DECISION_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption::new("approve", "Approve"),
    ChoiceOption::new("hold", "Hold"),
];

#[cfg(test)]
mod tests {
    use leptos::prelude::*;

    use super::*;

    fn render(view: impl FnOnce() -> AnyView + 'static) -> String {
        view().to_html()
    }

    #[test]
    fn button_renders_loading_state() {
        let html = render(|| {
            view! {
                <Button
                    label="Save changes"
                    priority=ActionPriority::Primary
                    state=ActionState::Loading
                />
            }
            .into_any()
        });

        assert!(html.contains("data-priority=\"primary\""));
        assert!(html.contains("data-state=\"loading\""));
        assert!(html.contains("aria-busy=\"true\""));
    }

    #[test]
    fn link_action_renders_secondary_semantics() {
        let html = render(|| {
            view! {
                <LinkAction
                    label="Read more"
                    href="/notes"
                    priority=ActionPriority::Secondary
                />
            }
            .into_any()
        });

        assert!(html.contains("href=\"/notes\""));
        assert!(html.contains("data-priority=\"secondary\""));
    }

    #[test]
    fn editorial_action_example_renders_editorial_surface() {
        let html = render(|| view! { <EditorialActionExample/> }.into_any());
        assert!(html.contains("Start reading"));
        assert!(html.contains("Join the list"));
    }

    #[test]
    fn workflow_action_example_renders_primary_and_secondary_actions() {
        let html = render(|| view! { <WorkflowActionExample/> }.into_any());
        assert!(html.contains("Loading…"));
        assert!(html.contains("Open history"));
    }

    #[test]
    fn text_field_renders_error_relationships() {
        let html = render(|| {
            view! {
                <TextField
                    label="Email"
                    name="email"
                    value="writer@example.com"
                    state=FieldState::Error
                    help="Used for publication updates."
                    message="Enter a valid address."
                />
            }
            .into_any()
        });

        assert!(html.contains("aria-invalid=\"true\""));
        assert!(html.contains("data-state=\"error\""));
        assert!(html.contains("Used for publication updates."));
        assert!(html.contains("Enter a valid address."));
    }

    #[test]
    fn textarea_renders_success_relationships() {
        let html = render(|| {
            view! {
                <Textarea
                    label="Notes"
                    name="notes"
                    value="Structured feedback."
                    state=FieldState::Success
                    help="Internal review context."
                    message="Saved."
                />
            }
            .into_any()
        });

        assert!(html.contains("data-state=\"success\""));
        assert!(html.contains("Saved."));
    }

    #[test]
    fn settings_field_example_renders_settings_context() {
        let html = render(|| view! { <SettingsFieldExample/> }.into_any());
        assert!(html.contains("Display name"));
        assert!(html.contains("Public profile label."));
    }

    #[test]
    fn form_field_example_renders_form_context() {
        let html = render(|| view! { <FormFieldExample/> }.into_any());
        assert!(html.contains("Submission notes"));
        assert!(html.contains("Saved."));
    }

    #[test]
    fn select_renders_error_relationships() {
        let html = render(|| {
            view! {
                <Select
                    label="Plan"
                    name="plan"
                    selected="studio"
                    options=PLAN_OPTIONS
                    state=FieldState::Error
                    message="Choose a supported plan."
                />
            }
            .into_any()
        });

        assert!(html.contains("aria-invalid=\"true\""));
        assert!(html.contains("Choose a supported plan."));
        assert!(html.contains("value=\"studio\""));
        assert!(html.contains("selected"));
    }

    #[test]
    fn checkbox_and_switch_render_accessible_labels() {
        let html = render(|| {
            view! {
                <div>
                    <Checkbox
                        label="Send updates"
                        name="updates"
                        checked=true
                    />
                    <Switch
                        label="Private mode"
                        name="private-mode"
                        checked=false
                    />
                </div>
            }
            .into_any()
        });

        assert!(html.contains("Send updates"));
        assert!(html.contains("role=\"switch\""));
        assert!(html.contains("Private mode"));
    }

    #[test]
    fn radio_group_renders_named_options() {
        let html = render(|| {
            view! {
                <RadioGroup
                    label="Review decision"
                    name="decision"
                    selected="approve"
                    options=DECISION_OPTIONS
                />
            }
            .into_any()
        });

        assert!(html.contains("Review decision"));
        assert!(html.contains("value=\"approve\" checked"));
    }

    #[test]
    fn settings_choice_example_renders_settings_context() {
        let html = render(|| view! { <SettingsChoiceExample/> }.into_any());
        assert!(html.contains("Workspace density"));
        assert!(html.contains("Private mode"));
    }

    #[test]
    fn form_choice_example_renders_form_sequence() {
        let html = render(|| view! { <FormChoiceExample/> }.into_any());
        assert!(html.contains("Review decision"));
        assert!(html.contains("Attach follow-up"));
    }
}
