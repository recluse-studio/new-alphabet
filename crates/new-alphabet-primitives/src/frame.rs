use leptos::prelude::*;
use new_alphabet_foundation::{Breakpoint, DensityMode, RegionClass, SpanBehavior};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameIntent {
    Editorial,
    Workspace,
}

impl Default for FrameIntent {
    fn default() -> Self {
        Self::Editorial
    }
}

impl FrameIntent {
    fn id(self) -> &'static str {
        match self {
            Self::Editorial => "editorial",
            Self::Workspace => "workspace",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegionPlacement {
    Lead,
    Main,
    Support,
    RailStart,
    RailEnd,
    Detail,
    ActionBand,
}

impl RegionPlacement {
    fn id(self) -> &'static str {
        match self {
            Self::Lead => "lead",
            Self::Main => "main",
            Self::Support => "support",
            Self::RailStart => "rail_start",
            Self::RailEnd => "rail_end",
            Self::Detail => "detail",
            Self::ActionBand => "action_band",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegionTag {
    Main,
    Section,
    Aside,
    Nav,
    Header,
}

fn span_value(span: SpanBehavior) -> String {
    match span {
        SpanBehavior::Columns(columns) => columns.to_string(),
        SpanBehavior::Stacked => "stacked".to_owned(),
    }
}

#[component]
pub fn AppShell(
    #[prop(optional)] density: Option<DensityMode>,
    #[prop(optional)] intent: Option<FrameIntent>,
    children: Children,
) -> impl IntoView {
    let density = density.unwrap_or(DensityMode::Regular);
    let intent = intent.unwrap_or_default();

    view! {
        <div
            class="na-app-shell"
            data-density=density.id()
            data-intent=intent.id()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PageGrid(
    #[prop(optional)] intent: Option<FrameIntent>,
    children: Children,
) -> impl IntoView {
    let intent = intent.unwrap_or_default();

    view! {
        <div
            class="na-page-grid"
            data-intent=intent.id()
            data-columns-compact=Breakpoint::Compact.columns().to_string()
            data-columns-medium=Breakpoint::Medium.columns().to_string()
            data-columns-wide=Breakpoint::Wide.columns().to_string()
            data-allows-three-region=if new_alphabet_foundation::allows_three_region_layout(Breakpoint::Wide) {
                "true"
            } else {
                "false"
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Region(
    kind: RegionClass,
    placement: RegionPlacement,
    #[prop(optional)] tag: Option<RegionTag>,
    children: Children,
) -> impl IntoView {
    let spans = kind.spans();
    let class = format!("na-region na-region--{}", kind.id());
    let tag = tag.unwrap_or_else(|| default_tag(kind));

    match tag {
        RegionTag::Main => view! {
            <main
                class=class
                data-region=kind.id()
                data-placement=placement.id()
                data-span-compact=span_value(spans.compact)
                data-span-medium=span_value(spans.medium)
                data-span-wide=span_value(spans.wide)
            >
                {children()}
            </main>
        }
        .into_any(),
        RegionTag::Section => view! {
            <section
                class=class
                data-region=kind.id()
                data-placement=placement.id()
                data-span-compact=span_value(spans.compact)
                data-span-medium=span_value(spans.medium)
                data-span-wide=span_value(spans.wide)
            >
                {children()}
            </section>
        }
        .into_any(),
        RegionTag::Aside => view! {
            <aside
                class=class
                data-region=kind.id()
                data-placement=placement.id()
                data-span-compact=span_value(spans.compact)
                data-span-medium=span_value(spans.medium)
                data-span-wide=span_value(spans.wide)
            >
                {children()}
            </aside>
        }
        .into_any(),
        RegionTag::Nav => view! {
            <nav
                class=class
                data-region=kind.id()
                data-placement=placement.id()
                data-span-compact=span_value(spans.compact)
                data-span-medium=span_value(spans.medium)
                data-span-wide=span_value(spans.wide)
            >
                {children()}
            </nav>
        }
        .into_any(),
        RegionTag::Header => view! {
            <header
                class=class
                data-region=kind.id()
                data-placement=placement.id()
                data-span-compact=span_value(spans.compact)
                data-span-medium=span_value(spans.medium)
                data-span-wide=span_value(spans.wide)
            >
                {children()}
            </header>
        }
        .into_any(),
    }
}

fn default_tag(kind: RegionClass) -> RegionTag {
    match kind {
        RegionClass::Main => RegionTag::Main,
        RegionClass::Rail | RegionClass::Support | RegionClass::Detail => RegionTag::Aside,
        RegionClass::ActionBand | RegionClass::Full => RegionTag::Section,
    }
}
