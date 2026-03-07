use leptos::prelude::*;
use new_alphabet_foundation::StateToken;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldState {
    Default,
    Error,
    Success,
    Disabled,
}

impl Default for FieldState {
    fn default() -> Self {
        Self::Default
    }
}

impl FieldState {
    pub(crate) fn id(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Error => "error",
            Self::Success => "success",
            Self::Disabled => "disabled",
        }
    }

    fn described_by(self, base_id: &str, help: Option<&str>, message: Option<&str>) -> String {
        let help_id = help.map(|_| format!("{base_id}-help"));
        let message_id = message.map(|_| format!("{base_id}-message"));

        match (help_id, message_id) {
            (Some(help_id), Some(message_id)) => format!("{help_id} {message_id}"),
            (Some(help_id), None) => help_id,
            (None, Some(message_id)) => message_id,
            (None, None) => String::new(),
        }
    }

    pub(crate) fn aria_invalid(self) -> &'static str {
        if matches!(self, Self::Error) {
            "true"
        } else {
            "false"
        }
    }

    pub(crate) fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }
}

#[component]
pub fn TextField(
    label: &'static str,
    name: &'static str,
    value: &'static str,
    #[prop(optional)] state: Option<FieldState>,
    #[prop(optional)] help: Option<&'static str>,
    #[prop(optional)] message: Option<&'static str>,
) -> impl IntoView {
    let state = state.unwrap_or_default();
    let input_id = format!("{name}-field");
    let described_by = state.described_by(&input_id, help, message);

    view! {
        <div
            class="na-text-field"
            data-state=state.id()
            data-focus-token=StateToken::FocusRing.id()
        >
            <label for=input_id.clone()>{label}</label>
            <input
                id=input_id.clone()
                name=name
                type="text"
                value=value
                disabled=state.is_disabled()
                aria-invalid=state.aria_invalid()
                aria-describedby=described_by
            />
            {help.map(|text| view! { <p id=format!("{input_id}-help") class="na-field-help">{text}</p> })}
            {message.map(|text| view! { <p id=format!("{input_id}-message") class="na-field-message">{text}</p> })}
        </div>
    }
}

#[component]
pub fn Textarea(
    label: &'static str,
    name: &'static str,
    value: &'static str,
    #[prop(optional)] state: Option<FieldState>,
    #[prop(optional)] help: Option<&'static str>,
    #[prop(optional)] message: Option<&'static str>,
) -> impl IntoView {
    let state = state.unwrap_or_default();
    let input_id = format!("{name}-field");
    let described_by = state.described_by(&input_id, help, message);

    view! {
        <div
            class="na-textarea"
            data-state=state.id()
            data-focus-token=StateToken::FocusRing.id()
        >
            <label for=input_id.clone()>{label}</label>
            <textarea
                id=input_id.clone()
                name=name
                disabled=state.is_disabled()
                aria-invalid=state.aria_invalid()
                aria-describedby=described_by
            >
                {value}
            </textarea>
            {help.map(|text| view! { <p id=format!("{input_id}-help") class="na-field-help">{text}</p> })}
            {message.map(|text| view! { <p id=format!("{input_id}-message") class="na-field-message">{text}</p> })}
        </div>
    }
}
