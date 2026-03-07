#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Breakpoint {
    Compact,
    Medium,
    Wide,
}

pub const ALL_BREAKPOINTS: [Breakpoint; 3] =
    [Breakpoint::Compact, Breakpoint::Medium, Breakpoint::Wide];

impl Breakpoint {
    pub const fn id(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Medium => "medium",
            Self::Wide => "wide",
        }
    }

    pub const fn columns(self) -> u8 {
        match self {
            Self::Compact => 4,
            Self::Medium => 8,
            Self::Wide => 12,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SpanBehavior {
    Columns(u8),
    Stacked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ResponsiveSpan {
    pub compact: SpanBehavior,
    pub medium: SpanBehavior,
    pub wide: SpanBehavior,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RegionClass {
    Full,
    Main,
    Support,
    Rail,
    Detail,
    ActionBand,
}

pub const ALL_REGION_CLASSES: [RegionClass; 6] = [
    RegionClass::Full,
    RegionClass::Main,
    RegionClass::Support,
    RegionClass::Rail,
    RegionClass::Detail,
    RegionClass::ActionBand,
];

impl RegionClass {
    pub const fn id(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Main => "main",
            Self::Support => "support",
            Self::Rail => "rail",
            Self::Detail => "detail",
            Self::ActionBand => "action_band",
        }
    }

    pub const fn spans(self) -> ResponsiveSpan {
        match self {
            Self::Full => ResponsiveSpan {
                compact: SpanBehavior::Columns(4),
                medium: SpanBehavior::Columns(8),
                wide: SpanBehavior::Columns(12),
            },
            Self::Main => ResponsiveSpan {
                compact: SpanBehavior::Columns(4),
                medium: SpanBehavior::Columns(6),
                wide: SpanBehavior::Columns(8),
            },
            Self::Support => ResponsiveSpan {
                compact: SpanBehavior::Columns(4),
                medium: SpanBehavior::Columns(2),
                wide: SpanBehavior::Columns(4),
            },
            Self::Rail => ResponsiveSpan {
                compact: SpanBehavior::Stacked,
                medium: SpanBehavior::Columns(2),
                wide: SpanBehavior::Columns(3),
            },
            Self::Detail => ResponsiveSpan {
                compact: SpanBehavior::Stacked,
                medium: SpanBehavior::Stacked,
                wide: SpanBehavior::Columns(4),
            },
            Self::ActionBand => ResponsiveSpan {
                compact: SpanBehavior::Columns(4),
                medium: SpanBehavior::Columns(8),
                wide: SpanBehavior::Columns(12),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RailWidthToken {
    Narrow,
    Default,
    Broad,
}

pub const ALL_RAIL_WIDTH_TOKENS: [RailWidthToken; 3] = [
    RailWidthToken::Narrow,
    RailWidthToken::Default,
    RailWidthToken::Broad,
];

impl RailWidthToken {
    pub const fn id(self) -> &'static str {
        match self {
            Self::Narrow => "layout.rail.narrow.width",
            Self::Default => "layout.rail.default.width",
            Self::Broad => "layout.rail.broad.width",
        }
    }

    pub const fn columns(self, breakpoint: Breakpoint) -> Option<u8> {
        match (self, breakpoint) {
            (_, Breakpoint::Compact) => None,
            (Self::Narrow, Breakpoint::Medium) => Some(2),
            (Self::Narrow, Breakpoint::Wide) => Some(2),
            (Self::Default, Breakpoint::Medium) => Some(2),
            (Self::Default, Breakpoint::Wide) => Some(3),
            (Self::Broad, Breakpoint::Medium) => Some(3),
            (Self::Broad, Breakpoint::Wide) => Some(4),
        }
    }
}

pub const fn allows_parallel_secondary(breakpoint: Breakpoint) -> bool {
    !matches!(breakpoint, Breakpoint::Compact)
}

pub const fn allows_three_region_layout(breakpoint: Breakpoint) -> bool {
    matches!(breakpoint, Breakpoint::Wide)
}
