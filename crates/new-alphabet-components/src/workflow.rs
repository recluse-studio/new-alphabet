use leptos::prelude::*;
use new_alphabet_foundation::StateToken;

use crate::actions::{ActionPriority, ActionState, LinkAction};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NavIndexItemState {
    Default,
    Current,
}

impl NavIndexItemState {
    fn id(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Current => "current",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NavIndexItem {
    pub label: &'static str,
    pub href: &'static str,
    pub state: NavIndexItemState,
}

impl NavIndexItem {
    pub const fn new(label: &'static str, href: &'static str) -> Self {
        Self {
            label,
            href,
            state: NavIndexItemState::Default,
        }
    }

    pub const fn current(label: &'static str, href: &'static str) -> Self {
        Self {
            label,
            href,
            state: NavIndexItemState::Current,
        }
    }
}

#[component]
pub fn NavIndex(label: &'static str, items: &'static [NavIndexItem]) -> impl IntoView {
    view! {
        <nav class="na-nav-index" aria-label=label data-kind="nav-index">
            <ul>
                {items
                    .iter()
                    .map(|item| {
                        view! {
                            <li data-state=item.state.id()>
                                <a
                                    href=item.href
                                    aria-current=if matches!(item.state, NavIndexItemState::Current) {
                                        "page"
                                    } else {
                                        "false"
                                    }
                                    data-focus-token=StateToken::FocusRing.id()
                                >
                                    {item.label}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandAction {
    pub label: &'static str,
    pub href: &'static str,
    pub state: ActionState,
}

impl CommandAction {
    pub const fn ready(label: &'static str, href: &'static str) -> Self {
        Self {
            label,
            href,
            state: ActionState::Ready,
        }
    }

    pub const fn loading(label: &'static str, href: &'static str) -> Self {
        Self {
            label,
            href,
            state: ActionState::Loading,
        }
    }

    pub const fn disabled(label: &'static str, href: &'static str) -> Self {
        Self {
            label,
            href,
            state: ActionState::Disabled,
        }
    }
}

#[component]
pub fn CommandBar(
    label: &'static str,
    primary: CommandAction,
    secondary: &'static [CommandAction],
) -> impl IntoView {
    view! {
        <section
            class="na-command-bar"
            aria-label=label
            data-kind="command-bar"
            data-secondary-count=secondary.len().to_string()
        >
            <div class="na-command-bar__primary">
                <LinkAction
                    label=primary.label
                    href=primary.href
                    priority=ActionPriority::Primary
                    state=primary.state
                />
            </div>
            <div class="na-command-bar__secondary">
                {secondary
                    .iter()
                    .map(|action| {
                        view! {
                            <LinkAction
                                label=action.label
                                href=action.href
                                priority=ActionPriority::Secondary
                                state=action.state
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FilterRailState {
    Default,
    ZeroResults,
}

impl Default for FilterRailState {
    fn default() -> Self {
        Self::Default
    }
}

impl FilterRailState {
    fn id(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::ZeroResults => "zero_results",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FilterOption {
    pub value: &'static str,
    pub label: &'static str,
    pub count: usize,
    pub selected: bool,
}

impl FilterOption {
    pub const fn new(value: &'static str, label: &'static str, count: usize) -> Self {
        Self {
            value,
            label,
            count,
            selected: false,
        }
    }

    pub const fn selected(value: &'static str, label: &'static str, count: usize) -> Self {
        Self {
            value,
            label,
            count,
            selected: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FilterGroup {
    pub label: &'static str,
    pub options: &'static [FilterOption],
}

impl FilterGroup {
    pub const fn new(label: &'static str, options: &'static [FilterOption]) -> Self {
        Self { label, options }
    }
}

fn filter_id(group: &str, value: &str) -> String {
    format!(
        "filter-{}-{}",
        group.to_lowercase().replace(' ', "-"),
        value.replace(' ', "-")
    )
}

#[component]
pub fn FilterRail(
    label: &'static str,
    groups: &'static [FilterGroup],
    #[prop(optional)] state: Option<FilterRailState>,
    #[prop(optional)] zero_result_message: Option<&'static str>,
) -> impl IntoView {
    let state = state.unwrap_or_default();
    let zero_result_message =
        zero_result_message.unwrap_or("No results match the current filters.");

    view! {
        <section
            class="na-filter-rail"
            aria-label=label
            data-kind="filter-rail"
            data-state=state.id()
            data-group-count=groups.len().to_string()
        >
            <h3>{label}</h3>
            {if matches!(state, FilterRailState::ZeroResults) {
                view! {
                    <p class="na-filter-rail__status" role="status" data-kind="zero-results">
                        {zero_result_message}
                    </p>
                }
                    .into_any()
            } else {
                view! { <></> }.into_any()
            }}
            {groups
                .iter()
                .map(|group| {
                    view! {
                        <fieldset class="na-filter-rail__group">
                            <legend>{group.label}</legend>
                            {group
                                .options
                                .iter()
                                .map(|option| {
                                    let input_id = filter_id(group.label, option.value);
                                    view! {
                                        <label for=input_id.clone()>
                                            <input
                                                id=input_id.clone()
                                                type="checkbox"
                                                name=group.label
                                                value=option.value
                                                checked=option.selected
                                                data-focus-token=StateToken::FocusRing.id()
                                            />
                                            <span class="na-filter-rail__label">{option.label}</span>
                                            <span class="na-filter-rail__count">{option.count}</span>
                                        </label>
                                    }
                                })
                                .collect_view()}
                        </fieldset>
                    }
                })
                .collect_view()}
        </section>
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DetailPaneState {
    Default,
    Loading,
    Empty,
    Unavailable,
}

impl Default for DetailPaneState {
    fn default() -> Self {
        Self::Default
    }
}

impl DetailPaneState {
    fn id(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Loading => "loading",
            Self::Empty => "empty",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DetailField {
    pub label: &'static str,
    pub value: &'static str,
}

impl DetailField {
    pub const fn new(label: &'static str, value: &'static str) -> Self {
        Self { label, value }
    }
}

fn effective_detail_state(state: DetailPaneState, fields: &[DetailField]) -> DetailPaneState {
    if matches!(state, DetailPaneState::Default) && fields.is_empty() {
        DetailPaneState::Empty
    } else {
        state
    }
}

#[component]
pub fn DetailPane(
    title: &'static str,
    fields: &'static [DetailField],
    #[prop(optional)] summary: Option<&'static str>,
    #[prop(optional)] state: Option<DetailPaneState>,
    #[prop(optional)] loading_message: Option<&'static str>,
    #[prop(optional)] empty_message: Option<&'static str>,
    #[prop(optional)] unavailable_message: Option<&'static str>,
) -> impl IntoView {
    let state = effective_detail_state(state.unwrap_or_default(), fields);
    let loading_message = loading_message.unwrap_or("Loading detail.");
    let empty_message = empty_message.unwrap_or("Select an entry to inspect.");
    let unavailable_message = unavailable_message.unwrap_or("This detail is unavailable.");

    let body = match state {
        DetailPaneState::Default => view! {
            <dl class="na-detail-pane__fields">
                {fields
                    .iter()
                    .map(|field| {
                        view! {
                            <div class="na-detail-pane__field">
                                <dt>{field.label}</dt>
                                <dd>{field.value}</dd>
                            </div>
                        }
                    })
                    .collect_view()}
            </dl>
        }
        .into_any(),
        DetailPaneState::Loading => view! {
            <p
                class="na-detail-pane__status"
                role="status"
                data-kind="loading"
                data-state-token=StateToken::LoadingMuted.id()
            >
                {loading_message}
            </p>
        }
        .into_any(),
        DetailPaneState::Empty => view! {
            <p class="na-detail-pane__status" role="status" data-kind="empty">
                {empty_message}
            </p>
        }
        .into_any(),
        DetailPaneState::Unavailable => view! {
            <p class="na-detail-pane__status" role="alert" data-kind="unavailable">
                {unavailable_message}
            </p>
        }
        .into_any(),
    };

    view! {
        <section class="na-detail-pane" aria-label=title data-kind="detail-pane" data-state=state.id()>
            <header class="na-detail-pane__header">
                <h3>{title}</h3>
                {summary.map(|text| view! { <p class="na-detail-pane__summary">{text}</p> })}
            </header>
            {body}
        </section>
    }
}
