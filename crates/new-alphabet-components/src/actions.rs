use leptos::prelude::*;
use new_alphabet_foundation::StateToken;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionPriority {
    Primary,
    Secondary,
}

impl ActionPriority {
    fn id(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionState {
    Ready,
    Loading,
    Disabled,
}

impl Default for ActionState {
    fn default() -> Self {
        Self::Ready
    }
}

impl ActionState {
    fn id(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Loading => "loading",
            Self::Disabled => "disabled",
        }
    }

    fn is_inactive(self) -> bool {
        matches!(self, Self::Loading | Self::Disabled)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonType {
    Button,
    Submit,
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Button
    }
}

impl ButtonType {
    fn as_attr(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
        }
    }
}

#[component]
pub fn Button(
    label: &'static str,
    #[prop(optional)] priority: Option<ActionPriority>,
    #[prop(optional)] state: Option<ActionState>,
    #[prop(optional)] button_type: Option<ButtonType>,
) -> impl IntoView {
    let priority = priority.unwrap_or(ActionPriority::Secondary);
    let state = state.unwrap_or_default();
    let button_type = button_type.unwrap_or_default();
    let rendered_label = match state {
        ActionState::Loading => "Loading…",
        _ => label,
    };

    view! {
        <button
            class=format!("na-button na-button--{}", priority.id())
            type=button_type.as_attr()
            disabled=state.is_inactive()
            aria-busy=if matches!(state, ActionState::Loading) { "true" } else { "false" }
            data-priority=priority.id()
            data-state=state.id()
            data-hover="explicit"
            data-active="explicit"
            data-focus-token=StateToken::FocusRing.id()
            data-loading-token=StateToken::LoadingMuted.id()
        >
            {rendered_label}
        </button>
    }
}

#[component]
pub fn LinkAction(
    label: &'static str,
    href: &'static str,
    #[prop(optional)] priority: Option<ActionPriority>,
    #[prop(optional)] state: Option<ActionState>,
) -> impl IntoView {
    let priority = priority.unwrap_or(ActionPriority::Secondary);
    let state = state.unwrap_or_default();

    view! {
        <a
            class=format!("na-link-action na-link-action--{}", priority.id())
            href=if matches!(state, ActionState::Disabled) { "#" } else { href }
            aria-busy=if matches!(state, ActionState::Loading) { "true" } else { "false" }
            aria-disabled=if matches!(state, ActionState::Disabled) { "true" } else { "false" }
            tabindex=if matches!(state, ActionState::Disabled) { "-1" } else { "0" }
            data-priority=priority.id()
            data-state=state.id()
            data-hover="explicit"
            data-active="explicit"
            data-focus-token=StateToken::FocusRing.id()
        >
            {label}
        </a>
    }
}
