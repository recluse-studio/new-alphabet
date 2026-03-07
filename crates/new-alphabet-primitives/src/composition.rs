use leptos::prelude::*;
use new_alphabet_foundation::{Breakpoint, RailWidthToken, SpacingToken};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RailSide {
    Start,
    End,
}

impl RailSide {
    fn id(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

#[component]
pub fn Rail(
    #[prop(optional)] width: Option<RailWidthToken>,
    #[prop(optional)] side: Option<RailSide>,
    children: Children,
) -> impl IntoView {
    let width = width.unwrap_or(RailWidthToken::Default);
    let side = side.unwrap_or(RailSide::Start);

    view! {
        <aside
            class=format!("na-rail na-rail--{}", side.id())
            data-rail-width=width.id()
            data-side=side.id()
            data-columns-medium=width.columns(Breakpoint::Medium).unwrap_or_default().to_string()
            data-columns-wide=width.columns(Breakpoint::Wide).unwrap_or_default().to_string()
            data-collapse="stacked"
        >
            {children()}
        </aside>
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StackSpace {
    Tight,
    Default,
    Loose,
}

impl StackSpace {
    fn token(self) -> SpacingToken {
        match self {
            Self::Tight => SpacingToken::StackTight,
            Self::Default => SpacingToken::StackDefault,
            Self::Loose => SpacingToken::StackLoose,
        }
    }
}

#[component]
pub fn Stack(#[prop(optional)] spacing: Option<StackSpace>, children: Children) -> impl IntoView {
    let spacing = spacing.unwrap_or(StackSpace::Default);
    let token = spacing.token();

    view! {
        <div
            class="na-stack"
            data-gap-token=token.id()
            data-gap-calm=token.points(new_alphabet_foundation::DensityMode::Calm).to_string()
            data-gap-regular=token.points(new_alphabet_foundation::DensityMode::Regular).to_string()
            data-gap-dense=token.points(new_alphabet_foundation::DensityMode::Dense).to_string()
        >
            {children()}
        </div>
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RowAlign {
    Start,
    Center,
    Baseline,
}

impl RowAlign {
    fn id(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::Baseline => "baseline",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RowDistribution {
    Start,
    Between,
}

impl RowDistribution {
    fn id(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Between => "between",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RowGap {
    Tight,
    Default,
}

impl RowGap {
    fn token(self) -> SpacingToken {
        match self {
            Self::Tight => SpacingToken::StackTight,
            Self::Default => SpacingToken::StackDefault,
        }
    }
}

#[component]
pub fn Row(
    #[prop(optional)] align: Option<RowAlign>,
    #[prop(optional)] distribution: Option<RowDistribution>,
    #[prop(optional)] gap: Option<RowGap>,
    children: Children,
) -> impl IntoView {
    let align = align.unwrap_or(RowAlign::Start);
    let distribution = distribution.unwrap_or(RowDistribution::Start);
    let gap = gap.unwrap_or(RowGap::Default);
    let token = gap.token();

    view! {
        <div
            class="na-row"
            data-align=align.id()
            data-distribution=distribution.id()
            data-gap-token=token.id()
        >
            {children()}
        </div>
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColumnPreset {
    Two,
    Three,
    MainSupport,
}

impl ColumnPreset {
    fn id(self) -> &'static str {
        match self {
            Self::Two => "two",
            Self::Three => "three",
            Self::MainSupport => "main_support",
        }
    }

    fn columns(self, breakpoint: Breakpoint) -> u8 {
        match (self, breakpoint) {
            (_, Breakpoint::Compact) => 1,
            (Self::Two, Breakpoint::Medium) => 2,
            (Self::Two, Breakpoint::Wide) => 2,
            (Self::Three, Breakpoint::Medium) => 2,
            (Self::Three, Breakpoint::Wide) => 3,
            (Self::MainSupport, Breakpoint::Medium) => 2,
            (Self::MainSupport, Breakpoint::Wide) => 2,
        }
    }
}

#[component]
pub fn ColumnGroup(
    #[prop(optional)] preset: Option<ColumnPreset>,
    children: Children,
) -> impl IntoView {
    let preset = preset.unwrap_or(ColumnPreset::Two);

    view! {
        <div
            class="na-column-group"
            data-preset=preset.id()
            data-columns-compact=preset.columns(Breakpoint::Compact).to_string()
            data-columns-medium=preset.columns(Breakpoint::Medium).to_string()
            data-columns-wide=preset.columns(Breakpoint::Wide).to_string()
        >
            {children()}
        </div>
    }
}
