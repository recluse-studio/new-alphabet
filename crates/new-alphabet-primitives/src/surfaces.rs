use leptos::prelude::*;
use new_alphabet_foundation::{BorderToken, SpacingToken, StateToken};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceStrength {
    Default,
    Strong,
}

impl SurfaceStrength {
    fn border_token(self) -> BorderToken {
        match self {
            Self::Default => BorderToken::PanelDefault,
            Self::Strong => BorderToken::PanelStrong,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PanelState {
    Default,
    Loading,
    Success,
    Error,
}

impl PanelState {
    fn id(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Loading => "loading",
            Self::Success => "success",
            Self::Error => "error",
        }
    }

    fn token(self) -> Option<StateToken> {
        match self {
            Self::Default => None,
            Self::Loading => Some(StateToken::LoadingMuted),
            Self::Success => Some(StateToken::SuccessEmphasis),
            Self::Error => Some(StateToken::ErrorEmphasis),
        }
    }
}

#[component]
pub fn Panel(
    #[prop(optional)] strength: Option<SurfaceStrength>,
    #[prop(optional)] state: Option<PanelState>,
    children: Children,
) -> impl IntoView {
    let strength = strength.unwrap_or(SurfaceStrength::Default);
    let state = state.unwrap_or(PanelState::Default);
    let border = strength.border_token();
    let state_token = state.token().map(StateToken::id).unwrap_or("state.default");

    view! {
        <section
            class="na-panel"
            data-panel-state=state.id()
            data-border-token=border.id()
            data-state-token=state_token
            data-content-spacing=SpacingToken::PanelContent.id()
        >
            {children()}
        </section>
    }
}

#[component]
pub fn Band(
    #[prop(optional)] strength: Option<SurfaceStrength>,
    children: Children,
) -> impl IntoView {
    let strength = strength.unwrap_or(SurfaceStrength::Default);

    view! {
        <section
            class="na-band"
            data-border-token=strength.border_token().id()
            data-region="band"
        >
            {children()}
        </section>
    }
}

#[component]
pub fn SectionHeader(
    title: &'static str,
    #[prop(optional)] subtitle: Option<&'static str>,
    #[prop(optional)] annotation: Option<&'static str>,
) -> impl IntoView {
    view! {
        <header class="na-section-header">
            <div class="na-section-header__body">
                <h2>{title}</h2>
                {subtitle.map(|text| view! { <p class="na-section-header__subtitle">{text}</p> })}
            </div>
            {annotation.map(|text| view! { <p class="na-section-header__annotation">{text}</p> })}
        </header>
    }
}

#[component]
pub fn Divider(#[prop(optional)] strength: Option<SurfaceStrength>) -> impl IntoView {
    let strength = strength.unwrap_or(SurfaceStrength::Default);

    view! {
        <hr
            class="na-divider"
            data-border-token=strength.border_token().id()
        />
    }
}
