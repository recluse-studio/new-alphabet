#![forbid(unsafe_code)]

mod actions;
mod examples;

pub use actions::{ActionPriority, ActionState, Button, ButtonType, LinkAction};
pub use examples::{EditorialActionExample, WorkflowActionExample};

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
}
