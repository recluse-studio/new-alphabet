use leptos::prelude::*;
use new_alphabet_foundation::StateToken;

use crate::FieldState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChoiceOption {
    pub value: &'static str,
    pub label: &'static str,
}

impl ChoiceOption {
    pub const fn new(value: &'static str, label: &'static str) -> Self {
        Self { value, label }
    }
}

fn described_by(base_id: &str, help: Option<&str>, message: Option<&str>) -> String {
    let help_id = help.map(|_| format!("{base_id}-help"));
    let message_id = message.map(|_| format!("{base_id}-message"));

    match (help_id, message_id) {
        (Some(help_id), Some(message_id)) => format!("{help_id} {message_id}"),
        (Some(help_id), None) => help_id,
        (None, Some(message_id)) => message_id,
        (None, None) => String::new(),
    }
}

#[component]
pub fn Select(
    label: &'static str,
    name: &'static str,
    selected: &'static str,
    options: &'static [ChoiceOption],
    #[prop(optional)] state: Option<FieldState>,
    #[prop(optional)] help: Option<&'static str>,
    #[prop(optional)] message: Option<&'static str>,
) -> impl IntoView {
    let state = state.unwrap_or_default();
    let input_id = format!("{name}-select");
    let aria_described_by = described_by(&input_id, help, message);
    let aria_described_by = if aria_described_by.is_empty() {
        None
    } else {
        Some(aria_described_by)
    };

    view! {
        <div
            class="na-select"
            data-state=state.id()
            data-focus-token=StateToken::FocusRing.id()
        >
            <label for=input_id.clone()>{label}</label>
            <select
                id=input_id.clone()
                name=name
                disabled=state.is_disabled()
                aria-invalid=state.aria_invalid()
                aria-describedby=aria_described_by
            >
                {options.iter().map(|option| {
                    let is_selected = option.value == selected;
                    view! {
                        <option value=option.value selected=is_selected>{option.label}</option>
                    }
                }).collect_view()}
            </select>
            {help.map(|text| view! { <p id=format!("{input_id}-help") class="na-field-help">{text}</p> })}
            {message.map(|text| view! { <p id=format!("{input_id}-message") class="na-field-message">{text}</p> })}
        </div>
    }
}

#[component]
pub fn Checkbox(
    label: &'static str,
    name: &'static str,
    checked: bool,
    #[prop(optional)] state: Option<FieldState>,
    #[prop(optional)] message: Option<&'static str>,
) -> impl IntoView {
    let state = state.unwrap_or_default();
    let input_id = format!("{name}-checkbox");
    let message_id = message.map(|_| format!("{input_id}-message"));

    view! {
        <div
            class="na-checkbox"
            data-state=state.id()
            data-focus-token=StateToken::FocusRing.id()
        >
            <label for=input_id.clone()>
                <input
                    id=input_id.clone()
                    name=name
                    type="checkbox"
                    checked=checked
                    disabled=state.is_disabled()
                    aria-describedby=message_id.clone()
                />
                <span>{label}</span>
            </label>
            {message.zip(message_id.clone()).map(|(text, message_id)| view! { <p id=message_id class="na-field-message">{text}</p> })}
        </div>
    }
}

#[component]
pub fn RadioGroup(
    label: &'static str,
    name: &'static str,
    selected: &'static str,
    options: &'static [ChoiceOption],
    #[prop(optional)] state: Option<FieldState>,
    #[prop(optional)] message: Option<&'static str>,
) -> impl IntoView {
    let state = state.unwrap_or_default();
    let message_id = message.map(|_| format!("{name}-message"));

    view! {
        <fieldset
            class="na-radio-group"
            data-state=state.id()
            data-focus-token=StateToken::FocusRing.id()
            disabled=state.is_disabled()
            aria-describedby=message_id.clone()
        >
            <legend>{label}</legend>
            {options.iter().map(|option| {
                let is_selected = option.value == selected;
                let option_id = format!("{name}-{}", option.value);
                view! {
                    <label for=option_id.clone()>
                        <input
                            id=option_id.clone()
                            name=name
                            type="radio"
                            value=option.value
                            checked=is_selected
                            disabled=state.is_disabled()
                        />
                        <span>{option.label}</span>
                    </label>
                }
            }).collect_view()}
            {message.zip(message_id.clone()).map(|(text, message_id)| view! { <p id=message_id class="na-field-message">{text}</p> })}
        </fieldset>
    }
}

#[component]
pub fn Switch(
    label: &'static str,
    name: &'static str,
    checked: bool,
    #[prop(optional)] state: Option<FieldState>,
    #[prop(optional)] message: Option<&'static str>,
) -> impl IntoView {
    let state = state.unwrap_or_default();
    let input_id = format!("{name}-switch");
    let message_id = message.map(|_| format!("{input_id}-message"));

    view! {
        <div
            class="na-switch"
            data-state=state.id()
            data-focus-token=StateToken::FocusRing.id()
        >
            <label for=input_id.clone()>
                <input
                    id=input_id.clone()
                    name=name
                    type="checkbox"
                    role="switch"
                    checked=checked
                    disabled=state.is_disabled()
                    aria-describedby=message_id.clone()
                />
                <span>{label}</span>
            </label>
            {message.zip(message_id.clone()).map(|(text, message_id)| view! { <p id=message_id class="na-field-message">{text}</p> })}
        </div>
    }
}
